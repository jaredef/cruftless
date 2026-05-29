//! Statement-grammar parser (Tier-Ω.3.b round 3b subset).
//!
//! Replaces the prior opaque-byte-span skip for top-level statements with
//! typed Stmt AST. v1 covers VariableStatement, ExpressionStatement, Block,
//! EmptyStatement, FunctionDeclaration (body-opaque), ClassDeclaration
//! (body-opaque). Control-flow forms (If/For/While/Switch/Try/Return/Throw/
//! Break/Continue/Labelled/With/Debugger) fall back to Stmt::Opaque until
//! a follow-on sub-round.

use crate::parser::{ParseError, Parser};
use crate::token::{Punct, TokenKind};
use rusty_js_ast::{
    ArrayElement, ArrayPattern, BindingElement, BindingIdentifier, BindingPattern, CatchClause,
    Expr, ForBinding, ForInit, ObjectKey, ObjectPattern, ObjectPatternProperty, ObjectProperty,
    PropertyKey, Span, Stmt, SwitchCase, VariableDeclarator, VariableKind, VariableStatement,
};

/// Convert a parsed Expr produced by the cover grammar (e.g. an array or
/// object literal in for-of/for-in LHS or destructuring-assignment LHS) into
/// the equivalent BindingPattern. Returns None when the expression isn't a
/// valid assignment target (which the caller treats as a syntax error or
/// falls back to opaque-name handling).
fn expr_to_binding_pattern(e: Expr) -> Option<BindingPattern> {
    match e {
        Expr::Identifier { name, span } => {
            Some(BindingPattern::Identifier(BindingIdentifier { name, span }))
        }
        Expr::Array {
            elements,
            trailing_comma_after_spread,
            span,
        } => {
            // ARTC-EXT 1: §13.3.3 — AssignmentRestElement is the last
            // element of an AssignmentElementList and is not followed by
            // a comma. The parser preserved this flag from source text.
            if trailing_comma_after_spread {
                return None;
            }
            let mut out: Vec<Option<BindingElement>> = Vec::with_capacity(elements.len());
            let mut rest: Option<Box<BindingPattern>> = None;
            let n = elements.len();
            for (i, el) in elements.into_iter().enumerate() {
                match el {
                    ArrayElement::Elision { .. } => out.push(None),
                    ArrayElement::Expr(inner) => {
                        let (target_expr, default) = match inner {
                            Expr::Assign {
                                operator: rusty_js_ast::AssignOp::Assign,
                                target,
                                value,
                                ..
                            } => (*target, Some(*value)),
                            other => (other, None),
                        };
                        let span = target_expr.span();
                        let target = expr_to_binding_pattern(target_expr)?;
                        out.push(Some(BindingElement {
                            target,
                            default,
                            span,
                        }));
                    }
                    ArrayElement::Spread { expr, .. } => {
                        // Spec: rest element must be last.
                        if i + 1 != n {
                            return None;
                        }
                        rest = Some(Box::new(expr_to_binding_pattern(expr)?));
                    }
                }
            }
            Some(BindingPattern::Array(ArrayPattern {
                elements: out,
                rest,
                span,
            }))
        }
        Expr::Object { properties, span } => {
            let mut props: Vec<ObjectPatternProperty> = Vec::with_capacity(properties.len());
            let mut rest: Option<Box<BindingIdentifier>> = None;
            let n = properties.len();
            for (i, p) in properties.into_iter().enumerate() {
                match p {
                    ObjectProperty::Property {
                        key,
                        value,
                        shorthand,
                        kind: _,
                        span: pspan,
                    } => {
                        let pk = match key {
                            ObjectKey::Identifier { name, span } => {
                                PropertyKey::Identifier(BindingIdentifier { name, span })
                            }
                            ObjectKey::String { value, .. } => {
                                PropertyKey::String(std::rc::Rc::new(value))
                            }
                            ObjectKey::Number { value, .. } => PropertyKey::Number(value),
                            ObjectKey::Computed { expr, .. } => PropertyKey::Computed(expr),
                        };
                        let (target_expr, default) = match value {
                            Expr::Assign {
                                operator: rusty_js_ast::AssignOp::Assign,
                                target,
                                value,
                                ..
                            } => (*target, Some(*value)),
                            other => (other, None),
                        };
                        let target = expr_to_binding_pattern(target_expr)?;
                        props.push(ObjectPatternProperty {
                            key: pk,
                            value: BindingElement {
                                target,
                                default,
                                span: pspan,
                            },
                            shorthand,
                            span: pspan,
                        });
                    }
                    ObjectProperty::Spread { expr, .. } => {
                        if i + 1 != n {
                            return None;
                        }
                        if let Expr::Identifier { name, span } = expr {
                            rest = Some(Box::new(BindingIdentifier { name, span }));
                        } else {
                            return None;
                        }
                    }
                }
            }
            Some(BindingPattern::Object(ObjectPattern {
                properties: props,
                rest,
                span,
            }))
        }
        _ => None,
    }
}

fn starts_with_word(bytes: &[u8], word: &[u8]) -> bool {
    if !bytes.starts_with(word) {
        return false;
    }
    match bytes.get(word.len()) {
        Some(b) => !b.is_ascii_alphanumeric() && *b != b'_' && *b != b'$',
        None => true,
    }
}

fn is_valid_for_assignment_target(e: &Expr) -> bool {
    match e {
        Expr::Identifier { .. } | Expr::Member { .. } => true,
        Expr::Parenthesized { expr, .. } => is_valid_for_assignment_target(expr),
        _ => false,
    }
}

fn is_valid_assignment_pattern_expr(e: &Expr) -> bool {
    match e {
        Expr::Identifier { .. } | Expr::Member { .. } => true,
        Expr::Parenthesized { expr, .. } => is_valid_assignment_pattern_expr(expr),
        Expr::Array {
            elements,
            trailing_comma_after_spread,
            ..
        } => {
            if *trailing_comma_after_spread {
                return false;
            }
            let n = elements.len();
            elements.iter().enumerate().all(|(i, el)| match el {
                ArrayElement::Elision { .. } => true,
                ArrayElement::Expr(expr) => match expr {
                    Expr::Assign {
                        operator: rusty_js_ast::AssignOp::Assign,
                        target,
                        ..
                    } => is_valid_assignment_pattern_expr(target),
                    other => is_valid_assignment_pattern_expr(other),
                },
                ArrayElement::Spread { expr, .. } => {
                    i + 1 == n && is_valid_assignment_pattern_expr(expr)
                }
            })
        }
        Expr::Object { properties, .. } => {
            let n = properties.len();
            properties.iter().enumerate().all(|(i, prop)| match prop {
                ObjectProperty::Property { value, .. } => match value {
                    Expr::Assign {
                        operator: rusty_js_ast::AssignOp::Assign,
                        target,
                        ..
                    } => is_valid_assignment_pattern_expr(target),
                    other => is_valid_assignment_pattern_expr(other),
                },
                ObjectProperty::Spread { expr, .. } => {
                    i + 1 == n && matches!(expr, Expr::Identifier { .. })
                }
            })
        }
        _ => false,
    }
}

impl<'src> Parser<'src> {
    fn parse_assignment_expression_no_in(&mut self) -> Result<Expr, ParseError> {
        let saved_in_disallowed = self.in_disallowed;
        self.in_disallowed = true;
        let result = self.parse_assignment_expression();
        self.in_disallowed = saved_in_disallowed;
        result
    }

    fn parse_expression_no_in(&mut self) -> Result<Expr, ParseError> {
        let saved_in_disallowed = self.in_disallowed;
        self.in_disallowed = true;
        let result = self.parse_expression();
        self.in_disallowed = saved_in_disallowed;
        result
    }

    /// SDIBP-EXT 1: parse a Statement in a position where Declaration is
    /// forbidden (the body of for / for-in / for-of / if / else / while /
    /// do-while / with / labelled). Per ECMA-262 §13.1 Statement grammar,
    /// HoistableDeclaration, ClassDeclaration, and LexicalDeclaration are
    /// NOT Statements. Cruft's parse_statement accepts all of them; this
    /// substatement-checked entry rejects the obvious Declaration tokens
    /// before delegating.
    pub fn parse_substatement(&mut self) -> Result<Stmt, ParseError> {
        // Forbid lexical declarations: let / const.
        // (let-followed-by-ident-or-bracket distinguishes from member-access
        // on identifier `let`; for this position we conservatively reject
        // bare `let` followed by anything that could begin a binding.)
        if self.is_ident("const") {
            return Err(
                self.err_here("LexicalDeclaration `const` is not allowed as Statement body".into())
            );
        }
        if self.is_ident("let") {
            // `let [` or `let {` or `let <ident>` is LexicalDeclaration.
            let pos = self.lookahead_span().end;
            let bytes = self.source().as_bytes();
            let mut p = pos;
            while p < bytes.len() && (bytes[p].is_ascii_whitespace()) {
                p += 1;
            }
            if p < bytes.len() {
                let b = bytes[p];
                if b == b'[' || b == b'{' || b.is_ascii_alphabetic() || b == b'_' || b == b'$' {
                    return Err(self.err_here(
                        "LexicalDeclaration `let` is not allowed as Statement body".into(),
                    ));
                }
            }
        }
        // ClassDeclaration.
        if self.is_ident("class") {
            return Err(self.err_here("ClassDeclaration is not allowed as Statement body".into()));
        }
        // FunctionDeclaration / GeneratorDeclaration / AsyncFunctionDeclaration /
        // AsyncGeneratorDeclaration. Annex B B.3.4 permits a plain
        // FunctionDeclaration as the Statement body of `if` / `else` in
        // sloppy mode (web-compat). HDSB-EXT 1: when the caller signals
        // an Annex-B-eligible context AND we are not in strict mode AND
        // the function token is NOT `function*` (generator), accept it
        // and let parse_statement handle the FunctionDeclaration.
        // for/while/do-while/with/labelled bodies are NOT in the carve-out.
        if self.is_ident("function") {
            if self.allow_annex_b_function_in_substatement && !self.strict_mode {
                // Carve out generator: `function *` is NOT permitted by B.3.4.
                let pos = self.lookahead_span().end;
                let bytes = self.source().as_bytes();
                let mut p = pos;
                while p < bytes.len() && bytes[p].is_ascii_whitespace() {
                    p += 1;
                }
                if p >= bytes.len() || bytes[p] != b'*' {
                    return self.parse_statement();
                }
            }
            return Err(
                self.err_here("HoistableDeclaration is not allowed as Statement body".into())
            );
        }
        if self.is_ident("async") {
            let pos = self.lookahead_span().end;
            let bytes = self.source().as_bytes();
            let mut p = pos;
            while p < bytes.len() && bytes[p].is_ascii_whitespace() {
                p += 1;
            }
            if bytes[p..].starts_with(b"function") {
                return Err(self
                    .err_here("AsyncFunctionDeclaration is not allowed as Statement body".into()));
            }
        }
        self.parse_statement()
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;

        // VariableStatement
        if self.is_ident("var") || self.is_ident("let") || self.is_ident("const") {
            let v = self.parse_variable_statement()?;
            return Ok(Stmt::Variable(v));
        }
        // FunctionDeclaration (sync + async)
        if self.is_ident("function") {
            return self.parse_function_decl_stmt(false);
        }
        if self.is_ident("async") {
            // Peek-2 disambiguation: `async function` vs `async <expr>`.
            let pos = self.lookahead_span().end;
            let bytes = self.source().as_bytes();
            let mut p = pos;
            while p < bytes.len() && bytes[p].is_ascii_whitespace() {
                p += 1;
            }
            if bytes[p..].starts_with(b"function") {
                self.bump()?; // consume `async`
                return self.parse_function_decl_stmt(true);
            }
        }
        // ClassDeclaration
        if self.is_ident("class") {
            return self.parse_class_decl_stmt();
        }
        // Block
        if matches!(self.current_kind(), TokenKind::Punct(Punct::LBrace)) {
            return self.parse_block_statement();
        }
        // EmptyStatement
        if matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon)) {
            let span = self.lookahead_span();
            self.bump()?;
            return Ok(Stmt::Empty { span });
        }
        // Control-flow forms — typed in round 3c.
        if self.is_ident("if") {
            return self.parse_if_statement();
        }
        if self.is_ident("for") {
            return self.parse_for_statement();
        }
        if self.is_ident("while") {
            return self.parse_while_statement();
        }
        if self.is_ident("do") {
            return self.parse_do_while_statement();
        }
        if self.is_ident("switch") {
            return self.parse_switch_statement();
        }
        if self.is_ident("try") {
            return self.parse_try_statement();
        }
        if self.is_ident("return") {
            return self.parse_return_statement();
        }
        if self.is_ident("throw") {
            return self.parse_throw_statement();
        }
        if self.is_ident("break") {
            return self.parse_break_statement();
        }
        if self.is_ident("continue") {
            return self.parse_continue_statement();
        }
        if self.is_ident("debugger") {
            let span = self.lookahead_span();
            self.bump()?;
            self.consume_semicolon_pub()?;
            return Ok(Stmt::Debugger { span });
        }
        // `with` remains forbidden by strict/module semantics at higher
        // layers, but sloppy script/function code needs a typed node so the
        // bytecode tier can install the object environment.
        // Tier-Ω.5.gggggg: yield is now a real expression — let it fall
        // through to ExpressionStatement parsing where parse_unary picks
        // it up as Expr::Unary{Yield, ...}.
        if self.is_ident("with") {
            return self.parse_with_statement();
        }
        // LabelledStatement (Identifier ':' Statement) — typed.
        if let TokenKind::Ident(_) = self.current_kind() {
            let peek_pos = self.lookahead_span().end;
            let bytes = self.source().as_bytes();
            let mut p = peek_pos;
            while p < bytes.len() && bytes[p].is_ascii_whitespace() {
                p += 1;
            }
            if bytes.get(p) == Some(&b':') {
                let name = if let TokenKind::Ident(n) = self.current_kind().clone() {
                    n
                } else {
                    unreachable!()
                };
                let label_span = self.lookahead_span();
                if (self.in_generator || self.strict_mode) && name == "yield" {
                    return Err(ParseError {
                        span: label_span,
                        message: "`yield` is not a valid label in this context".into(),
                    });
                }
                if self.in_async && name == "await" {
                    return Err(ParseError {
                        span: label_span,
                        message: "`await` is not a valid label in async function code".into(),
                    });
                }
                self.bump()?; // consume label
                self.expect_punct(Punct::Colon)?;
                let body = self.parse_substatement()?;
                let end = body.span().start.max(self.last_span_end());
                return Ok(Stmt::Labelled {
                    label: BindingIdentifier {
                        name,
                        span: label_span,
                    },
                    body: Box::new(body),
                    span: Span::new(start, end),
                });
            }
        }
        // ExpressionStatement
        let expr = self.parse_expression()?;
        self.consume_semicolon_pub()?;
        let end = self.last_span_end();
        Ok(Stmt::Expression {
            expr,
            span: Span::new(start, end),
        })
    }

    pub(crate) fn parse_variable_statement(&mut self) -> Result<VariableStatement, ParseError> {
        let start = self.lookahead_span().start;
        let kind = match self.current_kind() {
            TokenKind::Ident(s) if s == "var" => VariableKind::Var,
            TokenKind::Ident(s) if s == "let" => VariableKind::Let,
            TokenKind::Ident(s) if s == "const" => VariableKind::Const,
            _ => return Err(self.err_here("expected var/let/const".into())),
        };
        self.bump()?;
        let mut declarators = Vec::new();
        loop {
            let d_start = self.lookahead_span().start;
            let target = self.parse_binding_target()?;
            // LABNL-EXT 1: §13.3.1.1 — lexical decl cannot bind "let".
            Self::check_no_let_bound_name(kind, &target)?;
            let init = if matches!(self.current_kind(), TokenKind::Punct(Punct::Assign)) {
                self.bump()?;
                Some(self.parse_assignment_expression()?)
            } else {
                None
            };
            if init.is_none()
                && !matches!(
                    self.current_kind(),
                    TokenKind::Punct(Punct::Comma)
                        | TokenKind::Punct(Punct::Semicolon)
                        | TokenKind::Punct(Punct::RParen)
                        | TokenKind::Punct(Punct::RBrace)
                        | TokenKind::Eof
                )
                && !self.lookahead_preceded_by_lt()
            {
                return Err(ParseError {
                    span: self.lookahead_span(),
                    message: "expected initializer, comma, or semicolon after declaration".into(),
                });
            }
            let d_end = self.last_span_end();
            declarators.push(VariableDeclarator {
                target,
                init,
                span: Span::new(d_start, d_end),
            });
            if matches!(self.current_kind(), TokenKind::Punct(Punct::Comma)) {
                self.bump()?;
            } else {
                break;
            }
        }
        self.consume_semicolon_pub()?;
        let end = self.last_span_end();
        Ok(VariableStatement {
            kind,
            declarators,
            span: Span::new(start, end),
        })
    }

    pub(crate) fn parse_function_decl_stmt(&mut self, is_async: bool) -> Result<Stmt, ParseError> {
        let start = if is_async {
            // `async` already consumed; recover start from before it.
            // The bump-tracker doesn't preserve prior span; use lookahead.
            self.lookahead_span().start
        } else {
            self.lookahead_span().start
        };
        self.expect_keyword("function")?;
        let is_generator = if matches!(self.current_kind(), TokenKind::Punct(Punct::Star)) {
            self.bump()?;
            true
        } else {
            false
        };
        let name = if let TokenKind::Ident(n) = self.current_kind().clone() {
            let span = self.lookahead_span();
            // IDT-EXT 1: §11.6.2.1 ReservedWord exclusion at function-decl
            // name (inline-constructed BindingIdentifier; bypasses
            // parse_binding_identifier).
            if crate::parser::is_unconditional_reserved_word(&n) {
                return Err(ParseError {
                    span,
                    message: format!(
                        "`{}` is a reserved word and cannot be used as a function name",
                        n
                    ),
                });
            }
            if self.strict_mode && (n == "eval" || n == "arguments") {
                return Err(ParseError {
                    span,
                    message: format!("Function name '{}' is not allowed in strict mode", n),
                });
            }
            if (is_generator || self.strict_mode) && n == "yield" {
                return Err(ParseError {
                    span,
                    message: "`yield` is not a valid function name in this context".into(),
                });
            }
            if is_async && n == "await" {
                return Err(ParseError {
                    span,
                    message: "`await` is not a valid function name in async function code".into(),
                });
            }
            self.bump()?;
            Some(BindingIdentifier { name: n, span })
        } else {
            None
        };
        let params = self.parse_function_parameters_ga(is_generator, is_async)?;
        let body = self.parse_function_body_gs(
            Some(is_generator),
            Some(is_async),
            Self::is_simple_param_list(&params),
        )?;
        let end = self.last_span_end();
        Ok(Stmt::FunctionDecl {
            name,
            is_async,
            is_generator,
            params,
            body,
            span: Span::new(start, end),
        })
    }

    pub(crate) fn parse_function_parameters(
        &mut self,
    ) -> Result<Vec<rusty_js_ast::Parameter>, ParseError> {
        self.parse_function_parameters_g(false)
    }

    /// YIFP-EXT 2 follow-on: parse a function's formal parameters with an
    /// explicit `is_generator` override. When parsing a generator function's
    /// OWN params, in_generator must be true so the yield-branch fires on
    /// `function* g(x = yield)` per §15.5.1 (FormalParameters Contains
    /// YieldExpression is a SyntaxError). For non-generator callers the
    /// flag defaults to inheriting the enclosing in_generator value.
    pub(crate) fn parse_function_parameters_g(
        &mut self,
        is_generator: bool,
    ) -> Result<Vec<rusty_js_ast::Parameter>, ParseError> {
        self.parse_function_parameters_ga(is_generator, false)
    }

    pub(crate) fn parse_function_parameters_ga(
        &mut self,
        is_generator: bool,
        is_async: bool,
    ) -> Result<Vec<rusty_js_ast::Parameter>, ParseError> {
        self.expect_punct(Punct::LParen)?;
        let prior_in_params = self.in_function_params;
        let prior_in_generator = self.in_generator;
        let prior_in_async = self.in_async;
        self.in_function_params = true;
        if is_generator {
            self.in_generator = true;
        }
        if is_async {
            self.in_async = true;
        }
        let result = self.parse_function_parameters_inner();
        self.in_function_params = prior_in_params;
        self.in_generator = prior_in_generator;
        self.in_async = prior_in_async;
        result
    }

    fn parse_function_parameters_inner(
        &mut self,
    ) -> Result<Vec<rusty_js_ast::Parameter>, ParseError> {
        let mut out = Vec::new();
        while !matches!(self.current_kind(), TokenKind::Punct(Punct::RParen)) {
            let p_start = self.lookahead_span().start;
            let rest = if matches!(self.current_kind(), TokenKind::Punct(Punct::Spread)) {
                self.bump()?;
                true
            } else {
                false
            };
            let target = self.parse_binding_target()?;
            let default = if matches!(self.current_kind(), TokenKind::Punct(Punct::Assign)) {
                self.bump()?;
                Some(self.parse_assignment_expression()?)
            } else {
                None
            };
            // RPDF-EXT 1: §15.1.1 / §14.1 — a BindingRestElement cannot
            // have an Initializer. `(...x = []) => {}` is a SyntaxError.
            if rest && default.is_some() {
                return Err(ParseError {
                    span: Span::new(p_start, self.last_span_end()),
                    message: "Rest parameter may not have a default initializer".into(),
                });
            }
            let p_end = self.last_span_end();
            out.push(rusty_js_ast::Parameter {
                target,
                default,
                rest,
                span: Span::new(p_start, p_end),
            });
            if matches!(self.current_kind(), TokenKind::Punct(Punct::Comma)) {
                // RPTC-locale (rest-param-trailing-comma): ECMA-262 §15.1.1
                // — a rest parameter may not be followed by a trailing comma
                // (and per §15.1.1, a rest must also be the last parameter,
                // so even a non-trailing comma after rest is a syntax error).
                if rest {
                    return Err(ParseError {
                        span: self.lookahead_span(),
                        message: "Rest parameter may not be followed by a trailing comma".into(),
                    });
                }
                self.bump()?;
            } else {
                break;
            }
        }
        self.expect_punct(Punct::RParen)?;
        Ok(out)
    }

    pub(crate) fn parse_function_body(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.parse_function_body_g(None)
    }

    /// SMPT-EXT 3: parse_function_body with generator-context override.
    /// `is_generator = Some(g)` introduces a generator boundary (saves
    /// prior `in_generator`, sets to `g`, restores on body exit). `None`
    /// preserves enclosing (arrow body, static block — neither introduces
    /// a generator boundary per ECMA-262 §15.3 + §15.7).
    pub(crate) fn parse_function_body_g(
        &mut self,
        is_generator: Option<bool>,
    ) -> Result<Vec<Stmt>, ParseError> {
        self.parse_function_body_gs(is_generator, None, true)
    }

    /// LABNL-EXT 1: §13.3.1.1 — BoundNames of a LexicalDeclaration must
    /// not contain "let". Universal (not strict-only). Walks the binding
    /// pattern's leaf identifiers; throws on the first match. Var
    /// declarations are exempt per spec.
    pub(crate) fn check_no_let_bound_name(
        kind: rusty_js_ast::VariableKind,
        target: &rusty_js_ast::BindingPattern,
    ) -> Result<(), ParseError> {
        if matches!(kind, rusty_js_ast::VariableKind::Var) {
            return Ok(());
        }
        for id in target.collect_names() {
            if id.name == "let" {
                return Err(ParseError {
                    span: id.span,
                    message: "Lexical declaration may not bind the name 'let'".into(),
                });
            }
        }
        Ok(())
    }

    /// NSPS-EXT 1: IsSimpleParameterList per ECMA-262 §15.2.1.4. A param is
    /// simple iff it is a plain BindingPattern::Identifier with no default
    /// initializer and is not rest. List is simple iff every param is.
    pub(crate) fn is_simple_param_list(params: &[rusty_js_ast::Parameter]) -> bool {
        params.iter().all(|p| {
            matches!(p.target, rusty_js_ast::BindingPattern::Identifier(_))
                && p.default.is_none()
                && !p.rest
        })
    }

    /// NSPS-EXT 1: parse_function_body with both generator-context AND
    /// is-simple-parameter-list overrides. is_simple=false enforces the
    /// ECMA-262 §15.2.1 / §15.3.1 early error: when ContainsUseStrict(body)
    /// is true AND IsSimpleParameterList(params) is false, throw SyntaxError.
    pub(crate) fn parse_function_body_gs(
        &mut self,
        is_generator: Option<bool>,
        is_async: Option<bool>,
        is_simple: bool,
    ) -> Result<Vec<Stmt>, ParseError> {
        let body_start = self.lookahead_span();
        self.expect_punct(Punct::LBrace)?;
        // SMPT-EXT 1: track function-body depth for yield-context disambiguation.
        self.function_body_depth += 1;
        // SMPT-EXT 2 + NSPS-EXT 1: §16.1.1 DirectivePrologues + §15.2.1/§15.3.1
        // non-simple-params early error.
        let prior_strict = self.strict_mode;
        if self.peek_use_strict_directive() {
            if !is_simple {
                return Err(ParseError {
                    span: body_start,
                    message:
                        "Illegal 'use strict' directive in function with non-simple parameter list"
                            .into(),
                });
            }
            self.strict_mode = true;
            self.set_lexer_strict(true); // SLEC-EXT 1: push to lexer for legacy-escape rejection
        }
        // SMPT-EXT 3: generator-context propagation.
        let prior_gen = self.in_generator;
        if let Some(g) = is_generator {
            self.in_generator = g;
        }
        let prior_async = self.in_async;
        if let Some(a) = is_async {
            self.in_async = a;
        }
        let mut out = Vec::new();
        while !matches!(self.current_kind(), TokenKind::Punct(Punct::RBrace))
            && !self.at_eof_internal()
        {
            out.push(self.parse_statement()?);
        }
        self.expect_punct(Punct::RBrace)?;
        self.function_body_depth = self.function_body_depth.saturating_sub(1);
        self.strict_mode = prior_strict;
        self.set_lexer_strict(prior_strict); // SLEC-EXT 1: restore lexer's view on body exit
        self.in_generator = prior_gen;
        self.in_async = prior_async;
        Ok(out)
    }

    pub(crate) fn parse_class_decl_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("class")?;
        let name = if let TokenKind::Ident(n) = self.current_kind().clone() {
            if n != "extends" {
                let span = self.lookahead_span();
                // IDT-EXT 1: §11.6.2.1 ReservedWord exclusion at class-decl
                // name (inline-constructed BindingIdentifier; bypasses
                // parse_binding_identifier).
                if crate::parser::is_unconditional_reserved_word(&n) {
                    return Err(ParseError {
                        span,
                        message: format!(
                            "`{}` is a reserved word and cannot be used as a class name",
                            n
                        ),
                    });
                }
                self.bump()?;
                Some(BindingIdentifier { name: n, span })
            } else {
                None
            }
        } else {
            None
        };
        let super_class = if self.is_ident("extends") {
            self.bump()?;
            Some(self.parse_left_hand_side_expression()?)
        } else {
            None
        };
        let members = self.parse_class_body()?;
        let end = self.last_span_end();
        Ok(Stmt::ClassDecl {
            name,
            super_class,
            members,
            span: Span::new(start, end),
        })
    }

    /// `{ ClassElement* }` — parses class member definitions per spec
    /// section 15.7. Method shorthand, getter / setter, static, generator
    /// (`*`), async, private (`#name`), computed (`[expr]`), field with
    /// optional initializer, ES2022 static-block.
    pub(crate) fn parse_class_body(
        &mut self,
    ) -> Result<Vec<rusty_js_ast::ClassMember>, ParseError> {
        use rusty_js_ast::{ClassMember, ClassMemberName, MethodKind};
        self.expect_punct(Punct::LBrace)?;
        let mut out = Vec::new();
        while !matches!(self.current_kind(), TokenKind::Punct(Punct::RBrace))
            && !self.at_eof_internal()
        {
            // Allow stray semicolons.
            if matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon)) {
                self.bump()?;
                continue;
            }
            let m_start = self.lookahead_span().start;
            let is_static = if self.is_ident("static") {
                // Disambiguate `static { ... }` (static-block), `static (`
                // / `static =` / `static ;` (method or field named `static`),
                // and `static method/field` (the modifier).
                let pos = self.lookahead_span().end;
                let bytes = self.source().as_bytes();
                let mut p = pos;
                while p < bytes.len() && bytes[p].is_ascii_whitespace() {
                    p += 1;
                }
                let next = bytes.get(p).copied();
                if next == Some(b'{') {
                    self.bump()?; // `static`
                                  // SMPT-EXT 4: ClassStaticBlockStatementList is strict
                                  // code and is parsed with [~Yield], even when the class
                                  // appears inside a generator function body.
                    let prior_strict = self.strict_mode;
                    let prior_gen = self.in_generator;
                    self.strict_mode = true;
                    self.set_lexer_strict(true);
                    self.in_generator = false;
                    let body_result = self.parse_function_body_gs(Some(false), None, true);
                    self.strict_mode = prior_strict;
                    self.set_lexer_strict(prior_strict);
                    self.in_generator = prior_gen;
                    let body = body_result?;
                    let end = self.last_span_end();
                    out.push(ClassMember::StaticBlock {
                        body,
                        span: Span::new(m_start, end),
                    });
                    continue;
                }
                // If `static` is immediately followed by `(`, `=`, `;`, or
                // `}`, it's a method-name / field-name, not the modifier.
                // fast-glob's compiled output uses `static(patterns,opts)`
                // as a real method.
                if matches!(next, Some(b'(') | Some(b'=') | Some(b';') | Some(b'}')) {
                    false
                } else {
                    self.bump()?;
                    true
                }
            } else {
                false
            };

            // Detect getter / setter / async / generator modifiers.
            let mut kind = MethodKind::Method;
            let mut is_async = false;
            let mut is_generator = false;

            if self.is_ident("get") {
                // `get` could be a method name OR the getter modifier.
                if !self.next_is_method_open_or_field_terminator() {
                    self.bump()?;
                    kind = MethodKind::Getter;
                }
            } else if self.is_ident("set") {
                if !self.next_is_method_open_or_field_terminator() {
                    self.bump()?;
                    kind = MethodKind::Setter;
                }
            } else if self.is_ident("async") {
                if !self.next_is_method_open_or_field_terminator() {
                    self.bump()?;
                    is_async = true;
                }
            }
            if matches!(self.current_kind(), TokenKind::Punct(Punct::Star)) {
                is_generator = true;
                self.bump()?;
            }

            // PropertyName / PrivateIdentifier.
            let name = self.parse_class_member_name()?;

            // Field or method?
            if matches!(self.current_kind(), TokenKind::Punct(Punct::LParen)) {
                let params = self.parse_function_parameters_ga(is_generator, is_async)?;
                let body = self.parse_function_body_gs(
                    Some(is_generator),
                    Some(is_async),
                    Self::is_simple_param_list(&params),
                )?;
                let end = self.last_span_end();
                // Constructor detection (only when not static and name is `constructor`).
                let method_kind = if !is_static && kind == MethodKind::Method {
                    match &name {
                        ClassMemberName::Identifier { name: n, .. } if n == "constructor" => {
                            MethodKind::Constructor
                        }
                        _ => MethodKind::Method,
                    }
                } else {
                    kind
                };
                out.push(ClassMember::Method {
                    name,
                    kind: method_kind,
                    is_static,
                    is_async,
                    is_generator,
                    params,
                    body,
                    span: Span::new(m_start, end),
                });
                continue;
            }
            // Field definition (with optional `= init` and `;`).
            let init = if matches!(self.current_kind(), TokenKind::Punct(Punct::Assign)) {
                self.bump()?;
                Some(self.parse_assignment_expression()?)
            } else {
                None
            };
            self.consume_class_field_terminator()?;
            let end = self.last_span_end();
            out.push(ClassMember::Field {
                name,
                is_static,
                init,
                span: Span::new(m_start, end),
            });
        }
        self.expect_punct(Punct::RBrace)?;
        self.validate_class_static_semantics(&out)?;
        Ok(out)
    }

    fn validate_class_static_semantics(
        &self,
        members: &[rusty_js_ast::ClassMember],
    ) -> Result<(), ParseError> {
        use rusty_js_ast::{ClassMember, ClassMemberName};
        let mut private_names = std::collections::HashSet::new();
        for m in members {
            match m {
                ClassMember::Method { name, .. } | ClassMember::Field { name, .. } => {
                    if let ClassMemberName::Private { name, .. } = name {
                        private_names.insert(name.as_str());
                    }
                }
                ClassMember::StaticBlock { .. } => {}
            }
        }
        for m in members {
            match m {
                ClassMember::Field {
                    name, init, span, ..
                } => {
                    if let Some(init) = init {
                        if self.expr_contains_arguments(init) {
                            return Err(ParseError {
                                message: "class field initializer cannot contain arguments".into(),
                                span: *span,
                            });
                        }
                    }
                    if let ClassMemberName::Computed { expr, span } = name {
                        if let Some(missing) = self.first_unbound_private_name(expr, &private_names)
                        {
                            return Err(ParseError {
                                message: format!("PrivateName #{} is not declared", missing),
                                span: *span,
                            });
                        }
                    }
                }
                ClassMember::Method { name, .. } => {
                    if let ClassMemberName::Computed { expr, span } = name {
                        if let Some(missing) = self.first_unbound_private_name(expr, &private_names)
                        {
                            return Err(ParseError {
                                message: format!("PrivateName #{} is not declared", missing),
                                span: *span,
                            });
                        }
                    }
                }
                ClassMember::StaticBlock { .. } => {}
            }
        }
        Ok(())
    }

    fn expr_contains_arguments(&self, expr: &rusty_js_ast::Expr) -> bool {
        use rusty_js_ast::{Argument, ArrowBody, Expr, MemberProperty, ObjectKey, ObjectProperty};
        match expr {
            Expr::Identifier { name, .. } => name == "arguments",
            Expr::Array { elements, .. } => elements.iter().any(|e| match e {
                rusty_js_ast::ArrayElement::Expr(e)
                | rusty_js_ast::ArrayElement::Spread { expr: e, .. } => {
                    self.expr_contains_arguments(e)
                }
                rusty_js_ast::ArrayElement::Elision { .. } => false,
            }),
            Expr::Object { properties, .. } => properties.iter().any(|p| match p {
                ObjectProperty::Property { key, value, .. } => {
                    matches!(key, ObjectKey::Computed { expr, .. } if self.expr_contains_arguments(expr))
                        || self.expr_contains_arguments(value)
                }
                ObjectProperty::Spread { expr, .. } => self.expr_contains_arguments(expr),
            }),
            Expr::Parenthesized { expr, .. }
            | Expr::Update { argument: expr, .. }
            | Expr::Unary { argument: expr, .. } => self.expr_contains_arguments(expr),
            Expr::Member {
                object, property, ..
            } => {
                self.expr_contains_arguments(object)
                    || matches!(
                        property.as_ref(),
                        MemberProperty::Computed { expr, .. } if self.expr_contains_arguments(expr)
                    )
            }
            Expr::Call {
                callee, arguments, ..
            }
            | Expr::New {
                callee, arguments, ..
            } => {
                self.expr_contains_arguments(callee)
                    || arguments.iter().any(|a| match a {
                        Argument::Expr(e) | Argument::Spread { expr: e, .. } => {
                            self.expr_contains_arguments(e)
                        }
                    })
            }
            Expr::Binary { left, right, .. } | Expr::Assign { target: left, value: right, .. } => {
                self.expr_contains_arguments(left) || self.expr_contains_arguments(right)
            }
            Expr::Conditional {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.expr_contains_arguments(test)
                    || self.expr_contains_arguments(consequent)
                    || self.expr_contains_arguments(alternate)
            }
            Expr::Sequence { expressions, .. } => {
                expressions.iter().any(|e| self.expr_contains_arguments(e))
            }
            Expr::Arrow { body, .. } => match body {
                ArrowBody::Expression(e) => self.expr_contains_arguments(e),
                ArrowBody::Block(stmts) => self.stmts_contain_arguments(stmts),
            },
            Expr::TemplateLiteral { expressions, .. } => {
                expressions.iter().any(|e| self.expr_contains_arguments(e))
            }
            Expr::Function { .. }
            | Expr::Class { .. }
            | Expr::NullLiteral { .. }
            | Expr::BoolLiteral { .. }
            | Expr::NumberLiteral { .. }
            | Expr::BigIntLiteral { .. }
            | Expr::StringLiteral { .. }
            | Expr::This { .. }
            | Expr::Super { .. }
            | Expr::MetaProperty { .. }
            | Expr::TemplateObject { .. }
            | Expr::RegExp { .. }
            | Expr::Opaque { .. } => false,
        }
    }

    fn stmts_contain_arguments(&self, stmts: &[rusty_js_ast::Stmt]) -> bool {
        use rusty_js_ast::Stmt;
        stmts.iter().any(|s| match s {
            Stmt::Expression { expr, .. } => self.expr_contains_arguments(expr),
            Stmt::Block { body, .. } => self.stmts_contain_arguments(body),
            Stmt::If {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.expr_contains_arguments(test)
                    || self.stmt_contains_arguments(consequent)
                    || alternate
                        .as_deref()
                        .is_some_and(|s| self.stmt_contains_arguments(s))
            }
            Stmt::Return { argument, .. } => argument
                .as_ref()
                .is_some_and(|e| self.expr_contains_arguments(e)),
            Stmt::Throw { argument, .. } => self.expr_contains_arguments(argument),
            _ => false,
        })
    }

    fn stmt_contains_arguments(&self, stmt: &rusty_js_ast::Stmt) -> bool {
        self.stmts_contain_arguments(std::slice::from_ref(stmt))
    }

    fn first_unbound_private_name<'a>(
        &self,
        expr: &'a rusty_js_ast::Expr,
        private_names: &std::collections::HashSet<&str>,
    ) -> Option<&'a str> {
        use rusty_js_ast::{Argument, Expr, MemberProperty};
        match expr {
            Expr::Member {
                object, property, ..
            } => self
                .first_unbound_private_name(object, private_names)
                .or_else(|| {
                    if let MemberProperty::Private { name, .. } = property.as_ref() {
                        if !private_names.contains(name.as_str()) {
                            return Some(name.as_str());
                        }
                    }
                    None
                }),
            Expr::Call {
                callee, arguments, ..
            }
            | Expr::New {
                callee, arguments, ..
            } => self
                .first_unbound_private_name(callee, private_names)
                .or_else(|| {
                    arguments.iter().find_map(|a| match a {
                        Argument::Expr(e) | Argument::Spread { expr: e, .. } => {
                            self.first_unbound_private_name(e, private_names)
                        }
                    })
                }),
            Expr::Parenthesized { expr, .. }
            | Expr::Update { argument: expr, .. }
            | Expr::Unary { argument: expr, .. } => {
                self.first_unbound_private_name(expr, private_names)
            }
            Expr::Binary { left, right, .. }
            | Expr::Assign {
                target: left,
                value: right,
                ..
            } => self
                .first_unbound_private_name(left, private_names)
                .or_else(|| self.first_unbound_private_name(right, private_names)),
            Expr::Conditional {
                test,
                consequent,
                alternate,
                ..
            } => self
                .first_unbound_private_name(test, private_names)
                .or_else(|| self.first_unbound_private_name(consequent, private_names))
                .or_else(|| self.first_unbound_private_name(alternate, private_names)),
            Expr::Sequence { expressions, .. } | Expr::TemplateLiteral { expressions, .. } => {
                expressions
                    .iter()
                    .find_map(|e| self.first_unbound_private_name(e, private_names))
            }
            _ => None,
        }
    }

    fn consume_class_field_terminator(&mut self) -> Result<(), ParseError> {
        if matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon)) {
            self.bump()?;
            return Ok(());
        }
        if matches!(self.current_kind(), TokenKind::Punct(Punct::RBrace))
            || self.lookahead_preceded_by_lt()
        {
            return Ok(());
        }
        Err(self.err_here("expected class field terminator".into()))
    }

    fn parse_class_member_name(&mut self) -> Result<rusty_js_ast::ClassMemberName, ParseError> {
        use rusty_js_ast::ClassMemberName;
        let span = self.lookahead_span();
        match self.current_kind().clone() {
            TokenKind::Ident(name) => {
                self.bump()?;
                Ok(ClassMemberName::Identifier { name, span })
            }
            TokenKind::PrivateIdent(name) => {
                if name == "constructor" {
                    return Err(ParseError {
                        message: "PrivateName cannot be #constructor".into(),
                        span,
                    });
                }
                self.bump()?;
                Ok(ClassMemberName::Private { name, span })
            }
            TokenKind::String(value) => {
                self.bump()?;
                Ok(ClassMemberName::String { value, span })
            }
            TokenKind::Number(value, _) => {
                self.bump()?;
                Ok(ClassMemberName::Number { value, span })
            }
            TokenKind::Punct(Punct::LBracket) => {
                self.bump()?;
                let expr = self.parse_assignment_expression()?;
                self.expect_punct(Punct::RBracket)?;
                Ok(ClassMemberName::Computed {
                    expr,
                    span: Span::new(span.start, self.last_span_end()),
                })
            }
            _ => Err(self.err_here("expected class member name".into())),
        }
    }

    /// Peek: does the byte immediately after this token look like the start
    /// of a method (`(`) or a field-terminator (`=`, `;`, line-break)? If so,
    /// the current `get` / `set` / `async` is actually a *name*, not a
    /// modifier.
    fn next_is_method_open_or_field_terminator(&self) -> bool {
        let pos = self.lookahead_span().end;
        let bytes = self.source().as_bytes();
        let mut p = pos;
        while p < bytes.len() && (bytes[p] == b' ' || bytes[p] == b'\t') {
            p += 1;
        }
        matches!(
            bytes.get(p),
            Some(&b'(') | Some(&b'=') | Some(&b';') | Some(&b'\n') | Some(&b'\r') | Some(&b'}')
        )
    }

    fn parse_block_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_punct(Punct::LBrace)?;
        let mut body = Vec::new();
        while !matches!(self.current_kind(), TokenKind::Punct(Punct::RBrace))
            && !self.at_eof_internal()
        {
            body.push(self.parse_statement()?);
        }
        self.expect_punct(Punct::RBrace)?;
        // BBND-EXT 1: §13.2.1 Block early errors —
        //  - LexicallyDeclaredNames must not contain duplicates
        //  - LexicallyDeclaredNames ∩ VarDeclaredNames must be empty
        // LDN = let/const/class/async-function/generator/async-generator,
        //       plus function in strict mode (Annex B B.3.2 carves out
        //       function-in-block as VDN in non-strict).
        // VDN = var, plus function in non-strict.
        self.check_block_bound_names(&body)?;
        let end = self.last_span_end();
        Ok(Stmt::Block {
            body,
            span: Span::new(start, end),
        })
    }

    /// Skip to top-level `;` / ASI / closing brace, returning the span of
    /// what was consumed. Used for the v1 opaque-control-flow fallback.
    fn skip_to_top_terminator(&mut self) -> Result<Span, ParseError> {
        let start = self.lookahead_span().start;
        let mut depth_paren = 0i32;
        let mut depth_brace = 0i32;
        let mut depth_bracket = 0i32;
        while !self.at_eof_internal() {
            let kind = self.current_kind().clone();
            match kind {
                TokenKind::Punct(Punct::LParen) => depth_paren += 1,
                TokenKind::Punct(Punct::RParen) => depth_paren -= 1,
                TokenKind::Punct(Punct::LBrace) => depth_brace += 1,
                TokenKind::Punct(Punct::RBrace) => {
                    if depth_brace == 0 {
                        break;
                    }
                    depth_brace -= 1;
                    // WBMS-EXT 1: if this `}` closes a brace opened inside
                    // this skip (depth_brace drops to 0) AND the statement
                    // was a brace-bodied form like `with(p){...}`, the
                    // statement is complete here — bump the `}` and return
                    // so the outer statement-list loop does not see it as a
                    // stray RBrace. Without this, the ASI fallback below
                    // breaks BEFORE bumping the `}` (because the `}` itself
                    // is LT-preceded once `with(p){\n ... \n}` body has a
                    // line terminator before the close), leaving the `}` as
                    // lookahead and tripping the expression parser.
                    if depth_brace == 0 && depth_paren == 0 && depth_bracket == 0 {
                        let end = self.lookahead_span().end;
                        self.bump()?;
                        return Ok(Span::new(start, end));
                    }
                }
                TokenKind::Punct(Punct::LBracket) => depth_bracket += 1,
                TokenKind::Punct(Punct::RBracket) => depth_bracket -= 1,
                TokenKind::Punct(Punct::Semicolon) => {
                    if depth_paren == 0 && depth_brace == 0 && depth_bracket == 0 {
                        let end = self.lookahead_span().end;
                        self.bump()?;
                        return Ok(Span::new(start, end));
                    }
                }
                _ => {}
            }
            // ASI: line-terminator-preceded top-level token closes the stmt.
            if depth_paren == 0
                && depth_brace == 0
                && depth_bracket == 0
                && self.lookahead_preceded_by_lt()
                && self.lookahead_span().start != start
            {
                break;
            }
            self.bump()?;
        }
        Ok(Span::new(start, self.last_span_end()))
    }

    // ─────────────────── Typed control-flow (round 3c) ───────────────────

    fn parse_if_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("if")?;
        self.expect_punct(Punct::LParen)?;
        let test = self.parse_expression()?;
        self.expect_punct(Punct::RParen)?;
        // HDSB-EXT 1: Annex B B.3.4 permits FunctionDeclaration as the
        // Statement body of `if` / `else` under sloppy mode. Enable the
        // carve-out only across these two parses; restore around.
        let prev_allow = self.allow_annex_b_function_in_substatement;
        self.allow_annex_b_function_in_substatement = true;
        let consequent = self.parse_substatement()?;
        let alternate = if self.is_ident("else") {
            self.bump()?;
            Some(Box::new(self.parse_substatement()?))
        } else {
            None
        };
        self.allow_annex_b_function_in_substatement = prev_allow;
        let end = self.last_span_end();
        Ok(Stmt::If {
            test,
            consequent: Box::new(consequent),
            alternate,
            span: Span::new(start, end),
        })
    }

    fn parse_for_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("for")?;
        // `for await (...)` (ES2018 — for-await-of)
        let await_form = if self.is_ident("await") {
            self.bump()?;
            true
        } else {
            false
        };
        self.expect_punct(Punct::LParen)?;

        // Head form discrimination: VariableDeclaration vs Expression vs empty.
        let head_is_var = self.is_ident("var")
            || self.is_ident("const")
            || (self.is_ident("let") && !self.for_head_let_is_lhs_identifier());
        let head_is_empty = matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon));

        // Try parsing the head; then peek for `in`/`of` to disambiguate.
        // For `for (let x of arr)` and `for (let x in obj)`, the head is one
        // BindingIdentifier with no `=` and the next token is `in`/`of`.
        if head_is_var {
            // Capture kind + first binding identifier, then peek.
            let kind = match self.current_kind() {
                TokenKind::Ident(s) if s == "var" => VariableKind::Var,
                TokenKind::Ident(s) if s == "let" => VariableKind::Let,
                TokenKind::Ident(s) if s == "const" => VariableKind::Const,
                _ => unreachable!(),
            };
            let kw_span = self.lookahead_span();
            self.bump()?;
            // Destructure head: `for (const [a, b] of …)` or `for (const {a} of …)`.
            // Parse one BindingPattern then look for `in`/`of`. Tier-Ω.5.g.3.
            if matches!(
                self.current_kind(),
                TokenKind::Punct(Punct::LBracket) | TokenKind::Punct(Punct::LBrace)
            ) {
                let pat_start = self.lookahead_span().start;
                let target = self.parse_binding_target()?;
                // LABNL-EXT 1: §13.3.1.1 lexical-decl-bound-name check.
                Self::check_no_let_bound_name(kind, &target)?;
                let pat_end = self.last_span_end();
                if self.is_ident("in") || self.is_contextual_keyword("of") {
                    let is_of = self.is_contextual_keyword("of");
                    self.bump()?;
                    let right = if is_of {
                        self.parse_assignment_expression()?
                    } else {
                        self.parse_expression()?
                    };
                    self.expect_punct(Punct::RParen)?;
                    let body = self.parse_substatement()?;
                    let end = self.last_span_end();
                    let left = ForBinding::Decl {
                        kind,
                        target,
                        span: Span::new(pat_start, pat_end),
                    };
                    return if is_of {
                        Ok(Stmt::ForOf {
                            left,
                            right,
                            body: Box::new(body),
                            await_: await_form,
                            span: Span::new(start, end),
                        })
                    } else {
                        Ok(Stmt::ForIn {
                            left,
                            right,
                            body: Box::new(body),
                            span: Span::new(start, end),
                        })
                    };
                }
                // C-style for with destructure declarator initializer.
                let init = if matches!(self.current_kind(), TokenKind::Punct(Punct::Assign)) {
                    self.bump()?;
                    Some(self.parse_assignment_expression_no_in()?)
                } else {
                    None
                };
                let mut declarators = vec![VariableDeclarator {
                    target,
                    init,
                    span: Span::new(pat_start, self.last_span_end()),
                }];
                while matches!(self.current_kind(), TokenKind::Punct(Punct::Comma)) {
                    self.bump()?;
                    let d_start = self.lookahead_span().start;
                    let dt = self.parse_binding_target()?;
                    Self::check_no_let_bound_name(kind, &dt)?;
                    let di = if matches!(self.current_kind(), TokenKind::Punct(Punct::Assign)) {
                        self.bump()?;
                        Some(self.parse_assignment_expression_no_in()?)
                    } else {
                        None
                    };
                    declarators.push(VariableDeclarator {
                        target: dt,
                        init: di,
                        span: Span::new(d_start, self.last_span_end()),
                    });
                }
                self.expect_punct(Punct::Semicolon)?;
                let test = if !matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon)) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.expect_punct(Punct::Semicolon)?;
                let update = if !matches!(self.current_kind(), TokenKind::Punct(Punct::RParen)) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.expect_punct(Punct::RParen)?;
                let body = self.parse_substatement()?;
                let end = self.last_span_end();
                let init_st = ForInit::Variable(VariableStatement {
                    kind,
                    declarators,
                    span: Span::new(kw_span.start, kw_span.end),
                });
                return Ok(Stmt::For {
                    init: Some(init_st),
                    test,
                    update,
                    body: Box::new(body),
                    span: Span::new(start, end),
                });
            }
            if let TokenKind::Ident(n) = self.current_kind().clone() {
                let id_span = self.lookahead_span();
                // IDT-EXT 1: §11.6.2.1 ReservedWord exclusion at the
                // for-head plain-id inline-construction path (bypasses
                // parse_binding_target and parse_binding_identifier).
                if crate::parser::is_unconditional_reserved_word(&n) {
                    return Err(ParseError {
                        span: id_span,
                        message: format!(
                            "`{}` is a reserved word and cannot be used as a binding identifier",
                            n
                        ),
                    });
                }
                // SBEA-EXT 1: §13.2 strict-mode binding-id check at the
                // for-(var|let|const) plain-identifier head — this path
                // bypasses parse_binding_target and parse_binding_identifier
                // by constructing the BindingIdentifier inline.
                if self.strict_mode && (n == "eval" || n == "arguments") {
                    return Err(ParseError {
                        span: id_span,
                        message: format!(
                            "Binding identifier '{}' is not allowed in strict mode",
                            n
                        ),
                    });
                }
                // LABNL-EXT 1: §13.3.1.1 — lexical for-binding cannot be "let".
                if !matches!(kind, VariableKind::Var) && n == "let" {
                    return Err(ParseError {
                        span: id_span,
                        message: "Lexical declaration may not bind the name 'let'".into(),
                    });
                }
                self.bump()?;
                // for-in / for-of head
                if self.is_ident("in") || self.is_contextual_keyword("of") {
                    let is_of = self.is_contextual_keyword("of");
                    self.bump()?;
                    let right = if is_of {
                        self.parse_assignment_expression()?
                    } else {
                        self.parse_expression()?
                    };
                    self.expect_punct(Punct::RParen)?;
                    let body = self.parse_substatement()?;
                    let end = self.last_span_end();
                    let left = ForBinding::Decl {
                        kind,
                        target: BindingPattern::Identifier(BindingIdentifier {
                            name: n,
                            span: id_span,
                        }),
                        span: Span::new(kw_span.start, id_span.end),
                    };
                    return if is_of {
                        Ok(Stmt::ForOf {
                            left,
                            right,
                            body: Box::new(body),
                            await_: await_form,
                            span: Span::new(start, end),
                        })
                    } else {
                        Ok(Stmt::ForIn {
                            left,
                            right,
                            body: Box::new(body),
                            span: Span::new(start, end),
                        })
                    };
                }
                // C-style with single var decl + optional initializer +
                // possibly more declarators. Recover via parse_variable_statement-like loop.
                let target = BindingPattern::Identifier(BindingIdentifier {
                    name: n.clone(),
                    span: id_span,
                });
                let init = if matches!(self.current_kind(), TokenKind::Punct(Punct::Assign)) {
                    self.bump()?;
                    // FII-EXT 1: parse Initializer under [-In] so that
                    // `var a = expr in obj` doesn't consume `in obj` as a
                    // RelationalExpression. The `in` must remain visible
                    // to the FII carve-out below.
                    let prev_in_disallowed = self.in_disallowed;
                    self.in_disallowed = true;
                    let e = self.parse_assignment_expression();
                    self.in_disallowed = prev_in_disallowed;
                    Some(e?)
                } else {
                    None
                };
                // FII-EXT 1: Annex B B.3.5 — for ( var BindingIdentifier
                // Initializer in Expression ) Statement. Sloppy mode only,
                // var only, plain BindingIdentifier only (not pattern),
                // single declarator only, for-in only (not for-of).
                if init.is_some()
                    && matches!(kind, VariableKind::Var)
                    && !self.strict_mode
                    && self.is_ident("in")
                {
                    self.bump()?; // consume `in`
                    let right = self.parse_expression()?;
                    self.expect_punct(Punct::RParen)?;
                    let body = self.parse_substatement()?;
                    let end = self.last_span_end();
                    // FII-EXT 1 emission: lower as a Block containing
                    //   var a = init;         // hoist + initialize
                    //   for (a in right) body // bare-name for-in
                    let var_stmt = VariableStatement {
                        kind,
                        declarators: vec![VariableDeclarator {
                            target: BindingPattern::Identifier(BindingIdentifier {
                                name: n.clone(),
                                span: id_span,
                            }),
                            init,
                            span: Span::new(id_span.start, id_span.end),
                        }],
                        span: kw_span,
                    };
                    let left = ForBinding::Pattern(BindingPattern::Identifier(BindingIdentifier {
                        name: n,
                        span: id_span,
                    }));
                    let for_in = Stmt::ForIn {
                        left,
                        right,
                        body: Box::new(body),
                        span: Span::new(start, end),
                    };
                    return Ok(Stmt::Block {
                        body: vec![Stmt::Variable(var_stmt), for_in],
                        span: Span::new(start, end),
                    });
                }
                let mut declarators = vec![VariableDeclarator {
                    target,
                    init,
                    span: Span::new(id_span.start, self.last_span_end()),
                }];
                while matches!(self.current_kind(), TokenKind::Punct(Punct::Comma)) {
                    self.bump()?;
                    let d_start = self.lookahead_span().start;
                    // Ω.5.P59.E2: accept BindingPattern (identifier OR
                    // destructure) for subsequent declarators in a
                    // multi-binding C-style for-init. flatted (and
                    // therefore stylelint downstream) uses
                    //   for (let ke = keys(o), {length} = ke, y = 0; ...)
                    // which is spec-permitted but pre-P59.E2 cruftless's
                    // parser broke on the {length} = ke declarator.
                    let target = if matches!(
                        self.current_kind(),
                        TokenKind::Punct(Punct::LBracket) | TokenKind::Punct(Punct::LBrace)
                    ) {
                        self.parse_binding_target()?
                    } else if let TokenKind::Ident(nn) = self.current_kind().clone() {
                        let nn_span = self.lookahead_span();
                        self.bump()?;
                        BindingPattern::Identifier(BindingIdentifier {
                            name: nn,
                            span: nn_span,
                        })
                    } else {
                        break;
                    };
                    let init = if matches!(self.current_kind(), TokenKind::Punct(Punct::Assign)) {
                        self.bump()?;
                        Some(self.parse_assignment_expression_no_in()?)
                    } else {
                        None
                    };
                    declarators.push(VariableDeclarator {
                        target,
                        init,
                        span: Span::new(d_start, self.last_span_end()),
                    });
                }
                self.expect_punct(Punct::Semicolon)?;
                let test = if !matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon)) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.expect_punct(Punct::Semicolon)?;
                let update = if !matches!(self.current_kind(), TokenKind::Punct(Punct::RParen)) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.expect_punct(Punct::RParen)?;
                let body = self.parse_substatement()?;
                let end = self.last_span_end();
                let init = ForInit::Variable(VariableStatement {
                    kind,
                    declarators,
                    span: Span::new(kw_span.start, kw_span.end),
                });
                return Ok(Stmt::For {
                    init: Some(init),
                    test,
                    update,
                    body: Box::new(body),
                    span: Span::new(start, end),
                });
            }
            // Fallback: pattern in head — opaque
        }

        // Expression-headed for / for-in / for-of
        if head_is_empty {
            self.bump()?;
        }
        // PPIF-EXT 2 (DELETED): the bare-identifier `for (id in/of …)`
        // fast-path lived here. It existed because `parse_expression` under
        // the implicit [+In] default consumed `id in obj` as a
        // RelationalExpression, tripping `expected `;``. The fast-path
        // bumped the ident, peeked for `in`/`of`, and rewound on miss.
        //
        // PPIF-EXT 1 named the [+In]/[-In] grammar parameter as parser
        // state (`Parser::in_disallowed`) and threaded `[-In]` around the
        // expression-head LHS parse at the path below. With that in place
        // the fast-path is structurally redundant: the expression-head
        // path now handles every shape the fast-path handled, plus shapes
        // the fast-path could not (MemberExpression LHS, ParenthesizedExpr
        // LHS, etc.) — see Finding PPIF.2.
        //
        // The async-of grammar-lookahead check (FAOF-EXT 1) and the
        // `this`/`super` SimpleAssignmentTarget check (FHLA-EXT 1) moved
        // to the expression-head path below where they apply against the
        // parsed expression's shape rather than the fast-path's peeked
        // ident name. `rewind_lexer_to` deleted along with this block
        // (its only caller was the fast-path's bail).
        let mut init_expr: Option<Expr> = None;
        if !head_is_empty && !matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon)) {
            // PPIF-EXT 1: enter for-head LHS under `[-In]`. §13.7.5
            // ForStatement uses Expression[~In, ...] in for-in/of LHS
            // position; setting in_disallowed = true here makes the
            // precedence climber refuse to consume `in` as a
            // RelationalExpression operator, so the LHS parse returns
            // before `in` is reached.
            let e = self.parse_expression_no_in()?;
            // Check for `in`/`of` after a LeftHandSideExpression head.
            if self.is_ident("in") || self.is_contextual_keyword("of") {
                let is_of = self.is_contextual_keyword("of");
                // FAOF-EXT 1 (relocated PPIF-EXT 2): §14.7.5 grammar
                // lookahead — the token sequence `async of` is forbidden
                // as a for-of head (disambiguates from `for await … of …`).
                // Pre-PPIF-EXT 2 this lived in the bare-ident fast-path;
                // now it operates on the parsed expression's identifier
                // shape.
                if is_of && !await_form {
                    if let Expr::Identifier { name, span } = &e {
                        let raw = &self.source()[span.start..span.end];
                        if name == "async" && raw == "async" {
                            return Err(ParseError {
                                span: *span,
                                message: "`async` cannot be the for-of LHS (grammar lookahead restriction)".into(),
                            });
                        }
                    }
                }
                self.bump()?;
                let right = if is_of {
                    self.parse_assignment_expression()?
                } else {
                    self.parse_expression()?
                };
                self.expect_punct(Punct::RParen)?;
                let body = self.parse_substatement()?;
                let end = self.last_span_end();
                let left = {
                    let span_fallback = e.span();
                    // FHAPV-EXT 1: §14.7.5.1 — when the for-in/for-of head
                    // is an Array/Object literal, it must be a valid
                    // AssignmentPattern. expr_to_binding_pattern returns
                    // None for invalid shapes (rest-not-last, rest-with-
                    // init, nested invalid LHS, object-rest-not-last).
                    // Pre-fix silently fell back to an empty BindingIdentifier;
                    // spec mandates SyntaxError at parse.
                    let is_pattern_literal = matches!(&e, Expr::Array { .. } | Expr::Object { .. });
                    // FHLA-EXT 1: §13.15.1 IsValidSimpleAssignmentTarget —
                    // `this` (and `super`) are not valid assignment targets;
                    // reject explicitly at for-head LHS rather than falling
                    // through to the opaque-name fallback.
                    // Unwrap any parenthesized layers for the assignment-
                    // target check, e.g. `for ((this) of …)`.
                    let mut probe = &e;
                    while let Expr::Parenthesized { expr, .. } = probe {
                        probe = expr;
                    }
                    if matches!(probe, Expr::This { .. } | Expr::Super { .. }) {
                        return Err(ParseError {
                            span: e.span(),
                            message: "Invalid left-hand side in for-in/for-of head".into(),
                        });
                    }
                    if is_valid_for_assignment_target(&e) {
                        ForBinding::AssignmentTarget(e.clone())
                    } else {
                        match expr_to_binding_pattern(e.clone()) {
                            Some(pat) => {
                                // SBAP-EXT 1: §13.15.1 + §13.2 — leaf binding-ids
                                // in the AssignmentPattern must obey strict-mode
                                // (eval/arguments) and generator (yield) rules.
                                self.check_pattern_binding_ids(&pat, span_fallback)?;
                                ForBinding::Pattern(pat)
                            }
                            None if is_pattern_literal && is_valid_assignment_pattern_expr(&e) => {
                                ForBinding::AssignmentTarget(e)
                            }
                            None if is_pattern_literal => {
                                return Err(ParseError {
                                    span: span_fallback,
                                    message:
                                        "Invalid destructuring assignment target in for-in/for-of head"
                                            .into(),
                                });
                            }
                            None => {
                                return Err(ParseError {
                                    span: span_fallback,
                                    message: "Invalid left-hand side in for-in/for-of head".into(),
                                });
                            }
                        }
                    }
                };
                return if is_of {
                    Ok(Stmt::ForOf {
                        left,
                        right,
                        body: Box::new(body),
                        await_: await_form,
                        span: Span::new(start, end),
                    })
                } else {
                    Ok(Stmt::ForIn {
                        left,
                        right,
                        body: Box::new(body),
                        span: Span::new(start, end),
                    })
                };
            }
            init_expr = Some(e);
        }
        if !head_is_empty {
            self.expect_punct(Punct::Semicolon)?;
        }
        let test = if !matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon)) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect_punct(Punct::Semicolon)?;
        let update = if !matches!(self.current_kind(), TokenKind::Punct(Punct::RParen)) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect_punct(Punct::RParen)?;
        let body = self.parse_substatement()?;
        let end = self.last_span_end();
        let init = init_expr.map(ForInit::Expression);
        Ok(Stmt::For {
            init,
            test,
            update,
            body: Box::new(body),
            span: Span::new(start, end),
        })
    }

    fn for_head_let_is_lhs_identifier(&self) -> bool {
        if !self.is_ident("let") {
            return false;
        }
        let bytes = self.source().as_bytes();
        let mut i = self.lookahead_span().end;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let rest = &bytes[i..];
        starts_with_word(rest, b"in")
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("while")?;
        self.expect_punct(Punct::LParen)?;
        let test = self.parse_expression()?;
        self.expect_punct(Punct::RParen)?;
        let body = self.parse_substatement()?;
        let end = self.last_span_end();
        Ok(Stmt::While {
            test,
            body: Box::new(body),
            span: Span::new(start, end),
        })
    }

    fn parse_with_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("with")?;
        self.expect_punct(Punct::LParen)?;
        let object = self.parse_expression()?;
        self.expect_punct(Punct::RParen)?;
        let body = self.parse_substatement()?;
        let end = self.last_span_end();
        Ok(Stmt::With {
            object,
            body: Box::new(body),
            span: Span::new(start, end),
        })
    }

    fn parse_do_while_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("do")?;
        let body = self.parse_substatement()?;
        self.expect_keyword("while")?;
        self.expect_punct(Punct::LParen)?;
        let test = self.parse_expression()?;
        self.expect_punct(Punct::RParen)?;
        self.consume_semicolon_pub()?;
        let end = self.last_span_end();
        Ok(Stmt::DoWhile {
            body: Box::new(body),
            test,
            span: Span::new(start, end),
        })
    }

    fn parse_switch_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("switch")?;
        self.expect_punct(Punct::LParen)?;
        let discriminant = self.parse_expression()?;
        self.expect_punct(Punct::RParen)?;
        self.expect_punct(Punct::LBrace)?;
        let mut cases = Vec::new();
        while !matches!(self.current_kind(), TokenKind::Punct(Punct::RBrace))
            && !self.at_eof_internal()
        {
            let case_start = self.lookahead_span().start;
            let test = if self.is_ident("case") {
                self.bump()?;
                let t = self.parse_expression()?;
                self.expect_punct(Punct::Colon)?;
                Some(t)
            } else if self.is_ident("default") {
                self.bump()?;
                self.expect_punct(Punct::Colon)?;
                None
            } else {
                return Err(self.err_here("expected `case` or `default` in switch body".into()));
            };
            let mut consequent = Vec::new();
            while !self.is_ident("case")
                && !self.is_ident("default")
                && !matches!(self.current_kind(), TokenKind::Punct(Punct::RBrace))
                && !self.at_eof_internal()
            {
                consequent.push(self.parse_statement()?);
            }
            let case_end = self.last_span_end();
            cases.push(SwitchCase {
                test,
                consequent,
                span: Span::new(case_start, case_end),
            });
        }
        self.expect_punct(Punct::RBrace)?;
        let end = self.last_span_end();
        Ok(Stmt::Switch {
            discriminant,
            cases,
            span: Span::new(start, end),
        })
    }

    fn parse_try_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("try")?;
        let block = self.parse_block_statement_public()?;
        let handler = if self.is_ident("catch") {
            let h_start = self.lookahead_span().start;
            self.bump()?;
            let param = if matches!(self.current_kind(), TokenKind::Punct(Punct::LParen)) {
                self.bump()?;
                let p = Some(self.parse_binding_target()?);
                self.expect_punct(Punct::RParen)?;
                p
            } else {
                None
            }; // ES2019 optional catch binding
            let body = self.parse_block_statement_public()?;
            let h_end = self.last_span_end();
            Some(CatchClause {
                param,
                body: Box::new(body),
                span: Span::new(h_start, h_end),
            })
        } else {
            None
        };
        let finalizer = if self.is_ident("finally") {
            self.bump()?;
            Some(Box::new(self.parse_block_statement_public()?))
        } else {
            None
        };
        let end = self.last_span_end();
        Ok(Stmt::Try {
            block: Box::new(block),
            handler,
            finalizer,
            span: Span::new(start, end),
        })
    }

    fn parse_return_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("return")?;
        // Per spec: return ASI applies if newline before next token.
        let argument = if matches!(self.current_kind(), TokenKind::Punct(Punct::Semicolon))
            || matches!(self.current_kind(), TokenKind::Punct(Punct::RBrace))
            || matches!(self.current_kind(), TokenKind::Eof)
            || self.lookahead_preceded_by_lt()
        {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.consume_semicolon_pub()?;
        let end = self.last_span_end();
        Ok(Stmt::Return {
            argument,
            span: Span::new(start, end),
        })
    }

    fn parse_throw_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("throw")?;
        if self.lookahead_preceded_by_lt() {
            return Err(self
                .err_here("no line terminator permitted between `throw` and its argument".into()));
        }
        let argument = self.parse_expression()?;
        self.consume_semicolon_pub()?;
        let end = self.last_span_end();
        Ok(Stmt::Throw {
            argument,
            span: Span::new(start, end),
        })
    }

    fn parse_break_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("break")?;
        let label = self.parse_optional_label()?;
        self.consume_semicolon_pub()?;
        let end = self.last_span_end();
        Ok(Stmt::Break {
            label,
            span: Span::new(start, end),
        })
    }

    fn parse_continue_statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.lookahead_span().start;
        self.expect_keyword("continue")?;
        let label = self.parse_optional_label()?;
        self.consume_semicolon_pub()?;
        let end = self.last_span_end();
        Ok(Stmt::Continue {
            label,
            span: Span::new(start, end),
        })
    }

    fn parse_optional_label(&mut self) -> Result<Option<BindingIdentifier>, ParseError> {
        // No-LT-before rule per spec — label only if same-line identifier.
        if self.lookahead_preceded_by_lt() {
            return Ok(None);
        }
        if let TokenKind::Ident(n) = self.current_kind().clone() {
            // Excludes keywords that always terminate the statement.
            if !matches!(n.as_str(), "else") {
                let span = self.lookahead_span();
                self.bump()?;
                return Ok(Some(BindingIdentifier { name: n, span }));
            }
        }
        Ok(None)
    }

    fn parse_block_statement_public(&mut self) -> Result<Stmt, ParseError> {
        self.parse_block_statement()
    }

    /// BBND-EXT 1+2: §13.2.1 Block static-semantics early errors.
    /// LexicallyDeclaredNames duplicate detection + LDN ∩ VDN check,
    /// per-decl identity tracked so Annex B B.3.2 carve-outs apply
    /// correctly. VDN harvested from nested blocks/loops/etc to model
    /// var-hoisting.
    pub(crate) fn check_block_bound_names(&self, body: &[Stmt]) -> Result<(), ParseError> {
        // Each entry: (name, span, decl_id, is_lex, is_var, is_plain_func_nonstrict)
        // A plain FunctionDecl in non-strict contributes BOTH is_lex and
        // is_var with the same decl_id (Annex B B.3.2/B.3.3 dual-role).
        let mut entries: Vec<(String, Span, u32, bool, bool, bool)> = Vec::new();
        let mut next_id: u32 = 0;
        self.collect_block_entries(body, false, &mut entries, &mut next_id);

        use std::collections::HashMap;
        let mut by_name: HashMap<&str, Vec<&(String, Span, u32, bool, bool, bool)>> =
            HashMap::new();
        for e in &entries {
            by_name.entry(&e.0).or_default().push(e);
        }
        for (_, es) in by_name {
            // Dup-LDN: distinct decl_ids that contribute to LDN, where
            // not all such contributions are plain-function-non-strict.
            let mut lex_ids: Vec<(u32, bool)> = Vec::new();
            for e in &es {
                if e.3 {
                    if !lex_ids.iter().any(|(id, _)| *id == e.2) {
                        lex_ids.push((e.2, e.5));
                    }
                }
            }
            if lex_ids.len() >= 2 {
                let all_plain_func = lex_ids.iter().all(|(_, pfn)| *pfn);
                if !all_plain_func {
                    let bad = es.iter().find(|e| e.3).unwrap();
                    return Err(ParseError {
                        span: bad.1,
                        message: format!(
                            "Identifier `{}` has already been declared in this block",
                            bad.0
                        ),
                    });
                }
            }
            // LDN∩VDN: exists a lex entry and a var entry from DIFFERENT
            // decl_ids, where NOT both sides are plain-function-non-strict
            // (Annex B B.3.2/B.3.3 lets multiple plain functions in
            // non-strict block coexist as both LDN and VDN without error).
            let lex_pairs: Vec<(u32, bool)> =
                es.iter().filter(|e| e.3).map(|e| (e.2, e.5)).collect();
            let var_pairs: Vec<(u32, bool)> =
                es.iter().filter(|e| e.4).map(|e| (e.2, e.5)).collect();
            let cross = lex_pairs.iter().any(|(li, lpf)| {
                var_pairs
                    .iter()
                    .any(|(vi, vpf)| li != vi && !(*lpf && *vpf))
            });
            if cross {
                let bad = es.iter().find(|e| e.3).unwrap();
                return Err(ParseError {
                    span: bad.1,
                    message: format!(
                        "Identifier `{}` cannot be redeclared (lexical/var conflict)",
                        bad.0
                    ),
                });
            }
        }
        Ok(())
    }

    /// Walk a StatementList and emit (name, span, decl_id, is_lex,
    /// is_var, is_plain_func_nonstrict) entries. `nested` controls var-
    /// hoisting recursion: at depth=0 we emit all entries; at depth>0
    /// we emit only VDN entries (vars hoist into the enclosing block;
    /// lex declarations are block-scoped and do not).
    fn collect_block_entries(
        &self,
        body: &[Stmt],
        nested: bool,
        out: &mut Vec<(String, Span, u32, bool, bool, bool)>,
        next_id: &mut u32,
    ) {
        use rusty_js_ast::{Stmt as S, VariableKind};
        for s in body {
            match s {
                S::Variable(vs) => {
                    let id = *next_id;
                    *next_id += 1;
                    let (is_lex, is_var) = match vs.kind {
                        VariableKind::Let | VariableKind::Const => (!nested, false),
                        VariableKind::Var => (false, true),
                    };
                    if !is_lex && !is_var {
                        continue;
                    }
                    for d in &vs.declarators {
                        for nm in d.target.collect_names() {
                            out.push((nm.name.clone(), nm.span, id, is_lex, is_var, false));
                        }
                    }
                }
                S::FunctionDecl {
                    name: Some(n),
                    is_async,
                    is_generator,
                    ..
                } => {
                    let id = *next_id;
                    *next_id += 1;
                    let is_plain = !is_async && !is_generator;
                    let plain_func_nonstrict = is_plain && !self.strict_mode;
                    // At nested depth, only the var-side contribution
                    // hoists (and only plain functions in non-strict
                    // contribute to var). Async/generator/async-gen and
                    // strict-mode plain functions are pure-LDN and stay
                    // in their own block.
                    if nested {
                        if plain_func_nonstrict {
                            out.push((n.name.clone(), n.span, id, false, true, true));
                        }
                    } else {
                        let is_lex = true; // FunctionDecl is LDN in all modes
                        let is_var = plain_func_nonstrict;
                        out.push((
                            n.name.clone(),
                            n.span,
                            id,
                            is_lex,
                            is_var,
                            plain_func_nonstrict,
                        ));
                    }
                }
                S::ClassDecl { name: Some(n), .. } => {
                    if !nested {
                        let id = *next_id;
                        *next_id += 1;
                        out.push((n.name.clone(), n.span, id, true, false, false));
                    }
                }
                // Recurse into containers that can hold var declarations
                // which hoist outward.
                S::Block { body: inner, .. } => {
                    self.collect_block_entries(inner, true, out, next_id);
                }
                S::If {
                    consequent,
                    alternate,
                    ..
                } => {
                    self.collect_stmt_entries(consequent, true, out, next_id);
                    if let Some(a) = alternate {
                        self.collect_stmt_entries(a, true, out, next_id);
                    }
                }
                S::For { body: b, .. }
                | S::ForIn { body: b, .. }
                | S::ForOf { body: b, .. }
                | S::While { body: b, .. }
                | S::DoWhile { body: b, .. } => {
                    self.collect_stmt_entries(b, true, out, next_id);
                }
                S::Switch { cases, .. } => {
                    for c in cases {
                        self.collect_block_entries(&c.consequent, true, out, next_id);
                    }
                }
                S::Try {
                    block,
                    handler,
                    finalizer,
                    ..
                } => {
                    self.collect_stmt_entries(block, true, out, next_id);
                    if let Some(h) = handler {
                        self.collect_stmt_entries(&h.body, true, out, next_id);
                    }
                    if let Some(f) = finalizer {
                        self.collect_stmt_entries(f, true, out, next_id);
                    }
                }
                S::Labelled { body: b, .. } => {
                    self.collect_stmt_entries(b, true, out, next_id);
                }
                _ => {}
            }
        }
    }

    fn collect_stmt_entries(
        &self,
        s: &Stmt,
        nested: bool,
        out: &mut Vec<(String, Span, u32, bool, bool, bool)>,
        next_id: &mut u32,
    ) {
        let slice = std::slice::from_ref(s);
        self.collect_block_entries(slice, nested, out, next_id);
    }

    /// SBAP-EXT 1: walk a BindingPattern's leaf binding-identifiers and
    /// reject names disallowed by the current parser context per §13.2 +
    /// §13.15.1: `eval`/`arguments` in strict, `yield` in generator/strict.
    pub(crate) fn check_pattern_binding_ids(
        &self,
        pat: &BindingPattern,
        span: Span,
    ) -> Result<(), ParseError> {
        match pat {
            BindingPattern::Identifier(id) => {
                let n = &id.name;
                if self.strict_mode && (n == "eval" || n == "arguments") {
                    return Err(ParseError {
                        span: id.span,
                        message: format!("`{}` is not a valid binding in strict mode", n),
                    });
                }
                if (self.in_generator || self.strict_mode) && n == "yield" {
                    return Err(ParseError {
                        span: id.span,
                        message: "`yield` is not a valid binding in this context".into(),
                    });
                }
                Ok(())
            }
            BindingPattern::Array(ap) => {
                for el in &ap.elements {
                    if let Some(be) = el {
                        self.check_pattern_binding_ids(&be.target, span)?;
                    }
                }
                if let Some(r) = &ap.rest {
                    self.check_pattern_binding_ids(r, span)?;
                }
                Ok(())
            }
            BindingPattern::Object(op) => {
                for prop in &op.properties {
                    self.check_pattern_binding_ids(&prop.value.target, span)?;
                }
                if let Some(r) = &op.rest {
                    let n = &r.name;
                    if self.strict_mode && (n == "eval" || n == "arguments") {
                        return Err(ParseError {
                            span: r.span,
                            message: format!("`{}` is not a valid binding in strict mode", n),
                        });
                    }
                    if (self.in_generator || self.strict_mode) && n == "yield" {
                        return Err(ParseError {
                            span: r.span,
                            message: "`yield` is not a valid binding in this context".into(),
                        });
                    }
                }
                Ok(())
            }
        }
    }
}
