//! TS type-stripper — walks the source via `rusty_js_parser::Lexer` and
//! produces an erased source string + a vector of `TypeWitness` records.
//!
//! TSR-EXT 3 design (per `docs/design.md` §1): rather than reimplementing
//! the full ECMAScript expression+statement grammar, TSR strips TS-only
//! syntax at the source-text tier and feeds the stripped text into the
//! existing `rusty-js-parser`. This is Pin-Art-consistent (the
//! stripping rules are derived from TS spec excerpts) and dramatically
//! lower-LOC than a parser fork.
//!
//! Strip ranges are byte spans; each is replaced with ASCII spaces in
//! the output to preserve source positions (one-to-one byte mapping
//! keeps error spans + future source-maps correct).
//!
//! TSR-EXT 3 covers the unambiguous high-frequency TS surface:
//!   - `: T` annotation on let/const/var/param/field/return
//!   - `?` optional postfix (in `x?: T` shape)
//!   - `!` non-null postfix (after expr-position token)
//!   - `as T` cast expression
//!   - `interface X { ... }` declaration
//!   - `type X = ...;` alias
//!   - `declare ...` declaration (entire statement stripped)
//!
//! Deferred to TSR-EXT 4: generics on declarations/calls (angle-bracket
//! disambiguation), enums (require runtime lowering, not pure stripping),
//! `public/private/protected/readonly` ctor-param shorthand (requires
//! body rewrite), namespaces.

use rusty_js_parser::{Lexer, LexerGoal, Token, TokenKind, Punct, Span, TemplatePart};

/// TRCAPS-EXT 1 (regex-literal goal fix): select `LexerGoal::RegExp`
/// when the previous token's right-hand context permits a regex
/// literal (i.e., we're at an expression-start position), else `Div`.
/// Mirrors the JS parser's standard goal-selection rule per ECMA-262.
/// TRGC-EXT 2 follow-on: keywords that must NOT trigger the method-
/// overload-no-body strip rule, because they introduce control-flow
/// constructs that legitimately have `(...)` then `{...}` body without
/// an annotation between them. Filtering by name avoids requiring
/// expensive lookback to verify class-body vs control-flow context.
fn is_overload_blocked_name(name: &str) -> bool {
    // `do` is safe to NOT block — `do { ... } while ()` has `{` as
    // immediate-next-token, which fails the next_punct_immediate(LParen)
    // check naturally. `do(...)` as a method name is valid TS.
    matches!(name,
        "if" | "for" | "while" | "switch" | "catch" | "with"
        | "return" | "yield" | "await" | "throw" | "new" | "typeof"
        | "delete" | "void" | "in" | "of" | "instanceof"
        | "let" | "const" | "var" | "function" | "class"
        | "import" | "export" | "default" | "from" | "as"
        | "true" | "false" | "null" | "undefined" | "this"
        | "super" | "async" | "static"
    )
}

fn expr_or_div_goal(prev: Option<&TokenKind>) -> LexerGoal {
    let prev = match prev {
        Some(p) => p,
        None => return LexerGoal::RegExp,  // start of file = expression position
    };
    match prev {
        // Tokens that END an expression: a `/` after them is division.
        TokenKind::Ident(name) if !matches!(name.as_str(),
            "return" | "typeof" | "delete" | "void" | "await" | "yield"
            | "throw" | "new" | "in" | "of" | "instanceof" | "case"
        ) => LexerGoal::Div,
        TokenKind::Number(_, _)
        | TokenKind::BigInt(_, _)
        | TokenKind::String(_)
        | TokenKind::Template { .. }
        | TokenKind::Punct(Punct::RParen)
        | TokenKind::Punct(Punct::RBracket)
        | TokenKind::Punct(Punct::RBrace)
        | TokenKind::Punct(Punct::Inc)
        | TokenKind::Punct(Punct::Dec) => LexerGoal::Div,
        // Everything else is an expression-position context: a `/` is
        // a regex literal opener.
        _ => LexerGoal::RegExp,
    }
}
use crate::ts_ast::{TypeWitness, TypeWitnessKind, TsTypeRef};

#[derive(Debug)]
pub struct StripError {
    pub message: String,
    pub pos: usize,
}

impl std::fmt::Display for StripError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ts-strip @{}: {}", self.pos, self.message)
    }
}

/// Strip TS-only syntax from `src`; return the erased source + captured
/// type witnesses. Output bytes align with input bytes (stripped ranges
/// are space-filled), so downstream parse errors retain meaningful
/// positions.
pub fn strip_ts(src: &str) -> Result<(String, Vec<TypeWitness>), StripError> {
    let mut scanner = Scanner::new(src);
    scanner.run()?;

    let mut out = src.as_bytes().to_vec();
    for (start, end) in &scanner.strips {
        // Replace [start..end) with spaces, preserving newlines so the
        // ASI engine + line-based error reporting keep working.
        for i in *start..*end {
            if out[i] != b'\n' && out[i] != b'\r' {
                out[i] = b' ';
            }
        }
    }
    let stripped = String::from_utf8(out)
        .map_err(|e| StripError { message: format!("utf-8 corruption: {}", e), pos: 0 })?;
    Ok((stripped, scanner.witnesses))
}

// ─── scanner internals ─────────────────────────────────────────────────

/// One token + its span. We collect the whole token stream upfront
/// rather than streaming; .ts files are typically <50K LOC and the
/// scanner needs lookahead.
struct ScanTok {
    kind: TokenKind,
    span: Span,
    preceded_by_line_terminator: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BraceCtx {
    /// `{` that opens a block / class body / function body / module
    /// body — its contents are statements; `:` at top level is an
    /// annotation (class field) or label.
    Block,
    /// `{` that opens an object literal — its `:` are object-key
    /// separators, NOT annotations. Annotation-strip MUST bail here.
    ObjectLit,
    /// `{` that opens an object TYPE literal — annotation-strip is
    /// already inside a stripped region; doesn't matter.
    ObjectType,
}

struct Scanner<'src> {
    src: &'src str,
    toks: Vec<ScanTok>,
    /// Strip ranges (byte spans), accumulated as we walk.
    strips: Vec<(usize, usize)>,
    witnesses: Vec<TypeWitness>,
    /// Brace context stack — mirrors token-stream nesting. The top
    /// answers "what `{` am I currently inside" for is_annotation_colon
    /// + skip_type disambiguation.
    brace_stack: Vec<BraceCtx>,
    /// Paren depth — mirrors `(`/`)` nesting. Used to gate the
    /// object-literal-context bail in is_annotation_colon (a `:`
    /// inside an obj-lit-enclosed `()` is a method-param annotation,
    /// not a key:value separator).
    paren_depth: i32,
    /// Open ternary stack — each `?` operator (NOT `?:` optional or
    /// `?.` chain) pushes the current paren_depth. The matching `:`
    /// pops. Used by is_annotation_colon to reject a `:` whose top-
    /// of-stack matches current paren_depth (it's a ternary's else
    /// branch, not an annotation).
    ternary_stack: Vec<i32>,
}

impl<'src> Scanner<'src> {
    fn new(src: &'src str) -> Self {
        Scanner {
            src,
            toks: Vec::new(),
            strips: Vec::new(),
            witnesses: Vec::new(),
            brace_stack: Vec::new(),
            paren_depth: 0,
            ternary_stack: Vec::new(),
        }
    }

    /// TRCAPS-EXT 1: rough heuristic — are we currently inside a
    /// class body? Scan backwards through brace nesting looking for
    /// `class NAME` immediately before an LBrace at depth 1. Used to
    /// gate TS-only class-member-modifier stripping.
    fn in_class_body(&self) -> bool {
        // Walk the toks array up to the current scan position is
        // expensive at every step; instead consult brace_stack's depth.
        // We approximate: if brace_stack contains Block and the most
        // recent Block was preceded by `class NAME[ extends ...]
        // [implements ...]`, we're inside a class body. Tracking that
        // requires a per-Block kind tag — too much state for this
        // round. Simpler bound: if brace_stack.len() > 0 AND we can
        // find `class` Ident at the right depth via a small backward
        // scan from the current position, accept.
        //
        // At step(i) call sites we don't currently pass `i` here;
        // make this a no-op (returns true) for now and rely on the
        // is_ts_class_modifier name set + "followed by Ident" filter.
        // The cost of over-stripping `public` outside a class is low:
        // `public` is a reserved word in strict mode anyway, so the
        // stripped result would never have been a valid value-position
        // identifier. Same for `private/protected/abstract/override/
        // readonly`.
        true
    }

    /// Classify an `{` at token index `i` as Block vs ObjectLit by
    /// inspecting the immediately-preceding token. Heuristic suitable
    /// for the high-frequency real-world cases; edge cases (e.g. arrow
    /// body returning an object via `=> ({ ... })`) are handled by the
    /// `(` wrapper, not at the brace.
    fn classify_brace(&self, i: usize) -> BraceCtx {
        if i == 0 { return BraceCtx::Block; }
        match &self.toks[i - 1].kind {
            // Expression contexts → object literal.
            TokenKind::Punct(Punct::Assign)
            | TokenKind::Punct(Punct::LParen)
            | TokenKind::Punct(Punct::LBracket)
            | TokenKind::Punct(Punct::Comma)
            | TokenKind::Punct(Punct::Colon)
            | TokenKind::Punct(Punct::Question)
            | TokenKind::Punct(Punct::Arrow)
            | TokenKind::Punct(Punct::LogicalAnd)
            | TokenKind::Punct(Punct::LogicalOr)
            | TokenKind::Punct(Punct::NullishCoalesce)
            | TokenKind::Punct(Punct::Spread) => BraceCtx::ObjectLit,
            TokenKind::Ident(n) if n == "return"
                || n == "yield"
                || n == "throw"
                || n == "in"
                || n == "of"
                || n == "delete"
                || n == "new" => BraceCtx::ObjectLit,
            // NOTE: `void` and `typeof` excluded — these names appear
            // very commonly as TS return-type or type-query annotations
            // (e.g. `function f(): void {`). Including them caused the
            // following function body `{` to mis-classify as ObjectLit,
            // which broke annotation detection inside the body.
            // The cost of exclusion is an obscure unary-on-object case
            // like `void { x: 1 }` being treated as a block — extremely
            // rare in real code.
            _ => BraceCtx::Block,
        }
    }

    fn lex_all(&mut self) -> Result<(), StripError> {
        // TRSLS-EXT 1 (2026-05-24, ts-resolve-string-literal-safety
        // locale, Finding TCC.3 fix): goal-symbol selection per token.
        //
        // Template literals require `LexerGoal::TemplateTail` when the
        // lexer is positioned at the `}` that closes a substitution
        // (e.g. inside `` `${x}post` ``). Without the goal switch, the
        // lexer treats the `}` as a punctuator and the rest of the
        // template lexes as fresh tokens — the closing backtick becomes
        // a stray punctuator and an UnterminatedString lex error fires
        // somewhere downstream.
        //
        // Track template-substitution depth via a small stack: each
        // Template{Head, Middle} token opens a substitution we're
        // "outside of after this Template token returns"; each balanced
        // `}` paired with the opening `${` (which the lexer has already
        // consumed as part of the preceding Template token) closes one.
        //
        // Brace-depth tracking distinguishes substitution-closing `}`
        // from ordinary block-closing `}`: when we enter a substitution
        // (after Template::Head/Middle), record the brace depth at
        // entry; the `}` at THAT depth closes the substitution.
        let mut lx = Lexer::new(self.src);
        let mut tmpl_brace_depths: Vec<i32> = Vec::new();
        let mut brace_depth: i32 = 0;
        let mut prev_kind: Option<TokenKind> = None;
        loop {
            // Goal selection:
            // - If we just emitted a Template{Head|Middle}, the lexer
            //   is positioned at the start of the substitution; goal
            //   is Div (standard expression goal).
            // - If we're at a `}` AND it would close a substitution
            //   (brace_depth matches the recorded substitution-entry
            //   depth), goal is TemplateTail.
            let goal = if let Some(&entry_depth) = tmpl_brace_depths.last() {
                if brace_depth == entry_depth {
                    LexerGoal::TemplateTail
                } else {
                    expr_or_div_goal(prev_kind.as_ref())
                }
            } else {
                expr_or_div_goal(prev_kind.as_ref())
            };
            let t = lx.next_token(goal)
                .map_err(|e| StripError {
                    message: format!("lex: {:?}", e),
                    pos: lx.pos(),
                })?;
            // Maintain brace_depth + template-substitution stack.
            match &t.kind {
                TokenKind::Punct(Punct::LBrace) => brace_depth += 1,
                TokenKind::Punct(Punct::RBrace) => brace_depth -= 1,
                TokenKind::Template { part, .. } => {
                    match part {
                        TemplatePart::Head => {
                            // `pre${` — substitution opens; expression
                            // contents begin at current brace_depth.
                            tmpl_brace_depths.push(brace_depth);
                        }
                        TemplatePart::Middle => {
                            // `}mid${` — the lexer consumed both the
                            // closing `}` of the previous substitution
                            // AND the opening `${` of the next one.
                            // Stack stays the same (pop+push = no-op).
                        }
                        TemplatePart::Tail => {
                            // `}tail`` — closing substitution; pop.
                            tmpl_brace_depths.pop();
                        }
                        TemplatePart::NoSubstitution => {
                            // `simple` — no substitution; no state change.
                        }
                    }
                }
                _ => {}
            }
            let _ = prev_kind; // reserved for future heuristics
            prev_kind = Some(t.kind.clone());
            let done = matches!(t.kind, TokenKind::Eof);
            self.toks.push(ScanTok {
                kind: t.kind,
                span: t.span,
                preceded_by_line_terminator: t.preceded_by_line_terminator,
            });
            if done { return Ok(()); }
        }
    }

    fn run(&mut self) -> Result<(), StripError> {
        self.lex_all()?;
        let n = self.toks.len();
        let mut i = 0;
        while i < n {
            i = self.step(i)?;
        }
        // Merge overlapping/adjacent strips, then sort.
        self.strips.sort_by_key(|r| r.0);
        let mut merged: Vec<(usize, usize)> = Vec::with_capacity(self.strips.len());
        for r in self.strips.drain(..) {
            if let Some(last) = merged.last_mut() {
                if r.0 <= last.1 {
                    last.1 = last.1.max(r.1);
                    continue;
                }
            }
            merged.push(r);
        }
        self.strips = merged;
        Ok(())
    }

    /// Process one token; return next index. Most tokens just advance
    /// by 1; the TS-specific shapes consume larger ranges.
    fn step(&mut self, i: usize) -> Result<usize, StripError> {
        let t = &self.toks[i];
        match &t.kind {
            TokenKind::Ident(name) => {
                // TSR-EXT 4: decl-head generics. `function NAME<T,...>(`
                // or `class NAME<T,...>` — strip the `<...>` between
                // the name and the opening punctuator. Unambiguous
                // because the contexts are syntactically distinct from
                // the `<` operator.
                if (name == "function" || name == "class") && self.next_is_ident(i + 1) {
                    let after_name = i + 2;
                    if after_name < self.toks.len()
                        && matches!(self.toks[after_name].kind, TokenKind::Punct(Punct::Lt))
                    {
                        if let Some(close) = self.match_angle(after_name) {
                            let start = self.toks[after_name].span.start;
                            let end = self.toks[close].span.end;
                            self.strips.push((start, end));
                            // Don't return here; let normal stepping
                            // continue past this token. The decl head
                            // still has its own subsequent annotations.
                        }
                    }
                }
                // TRCAPS-EXT 1 (2026-05-24, ts-resolve-class-and-param-
                // shapes locale): `extends NAME<T,...>` — strip the
                // generic arguments on an extends clause. The Ident
                // `extends` is a reserved word at value position and a
                // class-clause keyword here; safe to detect by name +
                // next-is-ident + after-name-is-Lt.
                if name == "extends" && self.next_is_ident(i + 1) {
                    let after_name = i + 2;
                    if after_name < self.toks.len()
                        && matches!(self.toks[after_name].kind, TokenKind::Punct(Punct::Lt))
                    {
                        if let Some(close) = self.match_angle(after_name) {
                            let start = self.toks[after_name].span.start;
                            let end = self.toks[close].span.end;
                            self.strips.push((start, end));
                        }
                    }
                }
                // TRCAPS-EXT 1: `implements TYPE[<T>][, TYPE[<T>]]* {`
                // — strip the entire implements clause through the
                // class-body `{`. Only valid at class-decl context
                // (after a class head or its extends clause); detect by
                // scanning forward to the next top-level `{`.
                if name == "implements" {
                    // The implements list ends at the next top-level
                    // LBrace (class body). Find it without descending.
                    let mut j = i + 1;
                    let mut depth = 0i32;
                    while j < self.toks.len() {
                        match &self.toks[j].kind {
                            TokenKind::Punct(Punct::LBrace) if depth == 0 => break,
                            TokenKind::Punct(Punct::LParen)
                            | TokenKind::Punct(Punct::LBracket) => depth += 1,
                            TokenKind::Punct(Punct::RParen)
                            | TokenKind::Punct(Punct::RBracket) if depth > 0 => depth -= 1,
                            TokenKind::Punct(Punct::Lt) => depth += 1,
                            TokenKind::Punct(Punct::Gt) if depth > 0 => depth -= 1,
                            TokenKind::Eof | TokenKind::Punct(Punct::Semicolon) => break,
                            _ => {}
                        }
                        j += 1;
                    }
                    if j < self.toks.len()
                        && matches!(self.toks[j].kind, TokenKind::Punct(Punct::LBrace))
                        && j > i
                    {
                        let start = t.span.start;
                        let end = self.toks[j - 1].span.end;
                        self.strips.push((start, end));
                    }
                }
                // TRCAPS-EXT 1: TS-only class-member modifiers. At top
                // level of a class body, strip the lone keyword before
                // a member name. ECMAScript `static` is NOT stripped
                // (it's a real JS modifier).
                let is_ts_class_modifier = matches!(name.as_str(),
                    "public" | "private" | "protected" | "readonly"
                    | "abstract" | "override"
                );
                if is_ts_class_modifier && self.in_class_body() {
                    // Only strip when followed by another Ident (the
                    // member name) or by another modifier — i.e., the
                    // keyword is genuinely modifying something, not
                    // serving as an identifier (e.g. `let public = 1`
                    // is technically valid JS though rare).
                    if i + 1 < self.toks.len()
                        && matches!(self.toks[i + 1].kind, TokenKind::Ident(_))
                    {
                        self.strips.push((t.span.start, t.span.end));
                    }
                }
                // TRGC-EXT 2 follow-on: TS method overload declarations
                // have a signature but NO body, ending in `;`. JS
                // doesn't allow this — strip the entire signature.
                // Pattern: at class-body top level (brace_stack.last
                // is Block AND we're at member-start position):
                //   Ident NAME `(` ... `)` [`:` TYPE] `;`
                //   (no `{` between `)` and `;`).
                // Gated by:
                //   - is_overload_blocked_name(name) negative
                //   - brace_stack.last() == Block (in a class body)
                //   - preceded_by_line_terminator OR prev is `{` or `;`
                //     (statement-start position in the class body)
                // Class-body member-start OR module-level statement-
                // start (function overload at top level: `function
                // NAME(...): T;`). brace_stack.last == None covers
                // the latter; the prev-token check is the same.
                let in_block_or_module = matches!(self.brace_stack.last(),
                    Some(BraceCtx::Block) | None);
                let stmt_start_prev = i == 0
                    || t.preceded_by_line_terminator
                    || matches!(self.toks[i - 1].kind,
                        TokenKind::Punct(Punct::LBrace)
                        | TokenKind::Punct(Punct::Semicolon))
                    || matches!(&self.toks[i - 1].kind,
                        TokenKind::Ident(prev_name) if prev_name == "function");
                let at_class_member_start = in_block_or_module && stmt_start_prev;
                if at_class_member_start && !is_overload_blocked_name(name) {
                    // Allow generic-args `<T,...>` between name and `(`
                    // for `function NAME<T>(...): R;` overloads.
                    let lparen_search_pos = if i + 1 < self.toks.len()
                        && matches!(self.toks[i + 1].kind, TokenKind::Punct(Punct::Lt))
                    {
                        if let Some(close) = self.match_angle(i + 1) {
                            close + 1
                        } else { i + 1 }
                    } else { i + 1 };
                    if let Some(lparen) = self.next_punct_immediate(lparen_search_pos, Punct::LParen) {
                        if let Some(rparen) = self.match_parens(lparen) {
                            // Tighten: the IMMEDIATE next token after
                            // `)` must be `:` (annotation), `;` (no-
                            // body), or `{` (body). Anything else
                            // (`.`, `[`, `(`, `,`, etc.) makes this a
                            // call/expression, not a method-decl.
                            let after_rparen = rparen + 1;
                            let after_kind = self.toks.get(after_rparen).map(|t| &t.kind);
                            let is_method_decl_shape = matches!(after_kind,
                                Some(TokenKind::Punct(Punct::Colon))
                                | Some(TokenKind::Punct(Punct::LBrace))
                                | Some(TokenKind::Punct(Punct::Semicolon))
                            );
                            if !is_method_decl_shape {
                                // Fall through to other rules.
                            } else {
                                // Scan from after rparen for `{` (body
                                // — not an overload) or `;` (overload
                                // signature) at the SAME depth, with no
                                // intervening `{` for the body.
                                let mut k = after_rparen;
                                let mut depth = 0i32;
                                let mut found_overload = false;
                                while k < self.toks.len() {
                                    match &self.toks[k].kind {
                                        TokenKind::Punct(Punct::LBrace) if depth == 0 => break,
                                        TokenKind::Punct(Punct::LParen)
                                        | TokenKind::Punct(Punct::LBracket)
                                        | TokenKind::Punct(Punct::Lt) => depth += 1,
                                        TokenKind::Punct(Punct::RParen)
                                        | TokenKind::Punct(Punct::RBracket)
                                        | TokenKind::Punct(Punct::Gt) if depth > 0 => depth -= 1,
                                        TokenKind::Punct(Punct::Semicolon) if depth == 0 => {
                                            found_overload = true;
                                            break;
                                        }
                                        TokenKind::Eof => break,
                                        _ => {}
                                    }
                                    k += 1;
                                }
                                if found_overload {
                                    // For module-level `function NAME(...)
                                    // : T;` overloads, extend strip
                                    // range backward to include the
                                    // `function` keyword — otherwise a
                                    // bare `function` keyword remains
                                    // and the file fails to parse.
                                    let start = if i > 0 && matches!(&self.toks[i - 1].kind,
                                        TokenKind::Ident(n) if n == "function")
                                    {
                                        self.toks[i - 1].span.start
                                    } else { t.span.start };
                                    let end = self.toks[k].span.end;
                                    self.strips.push((start, end));
                                    return Ok(k + 1);
                                }
                            }
                        }
                    }
                }
                // `interface NAME { ... }` — strip the entire decl.
                if name == "interface" && self.next_is_ident(i + 1) {
                    if let Some(brace_open) = self.find_punct(i + 2, Punct::LBrace) {
                        if let Some(brace_close) = self.match_braces(brace_open) {
                            let start = t.span.start;
                            let end = self.toks[brace_close].span.end;
                            self.strips.push((start, end));
                            return Ok(brace_close + 1);
                        }
                    }
                }
                // `type NAME = ...;` — strip alias. Must NOT eat a
                // `type` identifier used as a value (e.g. `type = 1`).
                // Heuristic: at statement start (preceded by line term
                // or first token) AND next token is an Ident.
                if name == "type" && self.is_stmt_start(i) && self.next_is_ident(i + 1) {
                    if let Some(end_idx) = self.find_stmt_end(i + 2) {
                        let start = t.span.start;
                        let end = self.toks[end_idx].span.end;
                        self.strips.push((start, end));
                        return Ok(end_idx + 1);
                    }
                }
                // `declare ...` — strip entire statement.
                if name == "declare" && self.is_stmt_start(i) {
                    if let Some(end_idx) = self.find_stmt_end(i + 1) {
                        let start = t.span.start;
                        let end = self.toks[end_idx].span.end;
                        self.strips.push((start, end));
                        return Ok(end_idx + 1);
                    }
                }
                // `as T` — strip from `as` through end-of-type. Only
                // valid when preceding token is expr-yielding.
                if name == "as" && i > 0 && self.is_expr_terminator(i - 1) {
                    let type_start = t.span.start;
                    let after = self.skip_type(i + 1);
                    let type_end = if after > i + 1 {
                        self.toks[after - 1].span.end
                    } else { t.span.end };
                    self.strips.push((type_start, type_end));
                    return Ok(after);
                }
                Ok(i + 1)
            }
            TokenKind::Punct(Punct::Colon) => {
                // TRGC-EXT 1 ternary tracking: if the top of the
                // ternary stack matches current paren_depth, this `:`
                // closes a ternary's else-branch — pop and skip
                // annotation handling entirely.
                if let Some(&top) = self.ternary_stack.last() {
                    if top == self.paren_depth {
                        self.ternary_stack.pop();
                        return Ok(i + 1);
                    }
                }
                // Annotation site: a `:` that follows an Ident in a
                // declaration/param/field/return position. Bail on
                // ternaries / labels / object-literal-keys / case-labels.
                if self.is_annotation_colon(i) {
                    let start = t.span.start;
                    let after = self.skip_type(i + 1);
                    let end = if after > i + 1 {
                        self.toks[after - 1].span.end
                    } else { t.span.end };
                    self.strips.push((start, end));
                    // Emit a witness for the type. The name is the
                    // preceding Ident if available.
                    if i > 0 {
                        if let TokenKind::Ident(nm) = &self.toks[i - 1].kind {
                            let ty_text = &self.src[t.span.end..end];
                            self.witnesses.push(TypeWitness {
                                kind: TypeWitnessKind::LocalBinding {
                                    name: nm.clone(),
                                    ty: TsTypeRef::Named { name: ty_text.trim().to_string(), type_args: vec![] },
                                },
                                span: Span::new(start, end),
                            });
                        }
                    }
                    return Ok(after);
                }
                Ok(i + 1)
            }
            TokenKind::Punct(Punct::Question) => {
                // `x?: T` — strip the `?` if immediately followed by
                // `:` AND preceded by Ident (param/property optional).
                if i + 1 < self.toks.len()
                    && matches!(self.toks[i + 1].kind, TokenKind::Punct(Punct::Colon))
                    && i > 0
                    && matches!(self.toks[i - 1].kind, TokenKind::Ident(_))
                {
                    self.strips.push((t.span.start, t.span.end));
                    return Ok(i + 1);
                }
                // TRGC-EXT 1 ternary tracking: a `?` that is NOT a
                // `?:` (optional-prop) AND NOT `?.` (optional-chain)
                // AND NOT preceded by another `?` (nullish coalesce
                // already lexes as ?? Punct, so not seen here) is a
                // ternary opener at expression position. Push current
                // paren_depth so the matching `:` can pop.
                let next_is_colon = i + 1 < self.toks.len()
                    && matches!(self.toks[i + 1].kind, TokenKind::Punct(Punct::Colon));
                let next_is_dot = i + 1 < self.toks.len()
                    && matches!(self.toks[i + 1].kind, TokenKind::Punct(Punct::Dot));
                if !next_is_colon && !next_is_dot && i > 0 && self.is_expr_terminator(i - 1) {
                    self.ternary_stack.push(self.paren_depth);
                }
                Ok(i + 1)
            }
            TokenKind::Punct(Punct::LogicalNot) => {
                // `expr!` non-null postfix. The `!` follows an expr-
                // yielding token AND is NOT followed by `=` (which
                // would make it `!==`/`!=` — already disambiguated by
                // the lexer into StrictNe/Ne tokens, so a bare
                // LogicalNot here CAN'T be the equality form).
                // Postfix iff: preceded by expr terminator AND followed
                // by an operator/punctuator that's a postfix context
                // (`.`, `?.`, `,`, `;`, `)`, `]`, `}`, EOF, binop).
                if i > 0 && self.is_expr_terminator(i - 1) && self.next_is_postfix_context(i + 1) {
                    self.strips.push((t.span.start, t.span.end));
                    return Ok(i + 1);
                }
                Ok(i + 1)
            }
            TokenKind::Punct(Punct::LBrace) => {
                let ctx = self.classify_brace(i);
                self.brace_stack.push(ctx);
                Ok(i + 1)
            }
            TokenKind::Punct(Punct::RBrace) => {
                self.brace_stack.pop();
                Ok(i + 1)
            }
            TokenKind::Punct(Punct::LParen) => {
                self.paren_depth += 1;
                Ok(i + 1)
            }
            TokenKind::Punct(Punct::RParen) => {
                if self.paren_depth > 0 { self.paren_depth -= 1; }
                Ok(i + 1)
            }
            TokenKind::Punct(Punct::Lt) => {
                // TRGC-EXT 1 (2026-05-24, ts-resolve-generics-calls
                // locale): generic arrow `<T>(...) =>` AND generic
                // call/instantiation/method-decl `NAME<T>(...)`.
                // Distinguish from `<` operator via match_angle + `(`
                // look-ahead filter — operators never produce a
                // balanced `<...>(` shape.
                if let Some(close) = self.match_angle(i) {
                    let after = close + 1;
                    if after < self.toks.len()
                        && matches!(self.toks[after].kind, TokenKind::Punct(Punct::LParen))
                    {
                        // Decide between generic-arrow and generic-call
                        // based on prev-token context:
                        // - Expr-start (no prev OR prev in the expr-start
                        //   set) → generic-arrow; strip `<...>` only
                        //   (the `(...)` is the arrow's param list).
                        // - Ident / `)` / `]` prev → generic-call;
                        //   strip `<...>` only (the `(...)` is the
                        //   call's arg list).
                        let prev_is_expr_terminator = i > 0 && self.is_expr_terminator(i - 1);
                        let _ = prev_is_expr_terminator;  // both cases strip the same range
                        let start = self.toks[i].span.start;
                        let end = self.toks[close].span.end;
                        self.strips.push((start, end));
                    }
                }
                Ok(i + 1)
            }
            _ => Ok(i + 1),
        }
    }

    fn next_is_ident(&self, i: usize) -> bool {
        i < self.toks.len() && matches!(self.toks[i].kind, TokenKind::Ident(_))
    }

    fn is_stmt_start(&self, i: usize) -> bool {
        if i == 0 { return true; }
        let prev = &self.toks[i - 1];
        if prev.preceded_by_line_terminator || self.toks[i].preceded_by_line_terminator { return true; }
        matches!(prev.kind,
            TokenKind::Punct(Punct::Semicolon)
            | TokenKind::Punct(Punct::LBrace)
            | TokenKind::Punct(Punct::RBrace)
        )
    }

    /// Return Some(j) if the token at `at` is exactly `p`; else None.
    /// Used for the overload-detection pattern where we need the next
    /// token to be `(` immediately (no intervening tokens allowed —
    /// otherwise we'd be matching e.g. `name <generic>(`).
    fn next_punct_immediate(&self, at: usize, p: Punct) -> Option<usize> {
        if at < self.toks.len() {
            if let TokenKind::Punct(pp) = &self.toks[at].kind {
                if *pp == p { return Some(at); }
            }
        }
        None
    }

    /// Given LParen index, return matching RParen index. Mirrors
    /// match_braces; balances brace/bracket/paren nesting.
    fn match_parens(&self, lparen: usize) -> Option<usize> {
        let mut depth = 0i32;
        for j in lparen..self.toks.len() {
            match &self.toks[j].kind {
                TokenKind::Punct(Punct::LParen) => depth += 1,
                TokenKind::Punct(Punct::RParen) => {
                    depth -= 1;
                    if depth == 0 { return Some(j); }
                }
                TokenKind::Eof => return None,
                _ => {}
            }
        }
        None
    }

    /// Find the index of a specific punctuator starting from `from`,
    /// without descending into braces. Returns None if not found before
    /// EOF.
    fn find_punct(&self, from: usize, p: Punct) -> Option<usize> {
        for j in from..self.toks.len() {
            if let TokenKind::Punct(pp) = &self.toks[j].kind {
                if *pp == p { return Some(j); }
            }
        }
        None
    }

    /// Match an opening `<` (at index `lt`) with its closing `>`.
    /// Handles nesting + `>>` (Shr) as two closers. Bails on any
    /// statement-terminator before finding a match (defensive: the
    /// caller's `<` may not actually be a generic-args opener).
    fn match_angle(&self, lt: usize) -> Option<usize> {
        let mut depth = 0i32;
        for j in lt..self.toks.len() {
            match &self.toks[j].kind {
                TokenKind::Punct(Punct::Lt) => depth += 1,
                TokenKind::Punct(Punct::Gt) => {
                    depth -= 1;
                    if depth == 0 { return Some(j); }
                }
                TokenKind::Punct(Punct::Shr) => {
                    depth -= 2;
                    if depth <= 0 { return Some(j); }
                }
                TokenKind::Eof
                | TokenKind::Punct(Punct::Semicolon)
                | TokenKind::Punct(Punct::LBrace) => return None,
                _ => {}
            }
        }
        None
    }

    /// Given LBrace index, return matching RBrace index. Handles nesting.
    fn match_braces(&self, lbrace: usize) -> Option<usize> {
        let mut depth = 0i32;
        for j in lbrace..self.toks.len() {
            match &self.toks[j].kind {
                TokenKind::Punct(Punct::LBrace) => depth += 1,
                TokenKind::Punct(Punct::RBrace) => {
                    depth -= 1;
                    if depth == 0 { return Some(j); }
                }
                _ => {}
            }
        }
        None
    }

    /// Find end of a statement starting at `from`: scan to the next
    /// top-level `;`, `}`, or ASI boundary (line terminator before a
    /// token that can't continue the expression).
    fn find_stmt_end(&self, from: usize) -> Option<usize> {
        let mut depth_paren = 0i32;
        let mut depth_brace = 0i32;
        let mut depth_brack = 0i32;
        let mut j = from;
        while j < self.toks.len() {
            let k = &self.toks[j].kind;
            match k {
                TokenKind::Punct(Punct::LParen) => depth_paren += 1,
                TokenKind::Punct(Punct::RParen) => depth_paren -= 1,
                TokenKind::Punct(Punct::LBrace) => depth_brace += 1,
                TokenKind::Punct(Punct::RBrace) => {
                    if depth_brace == 0 { return Some(j.saturating_sub(1).max(from)); }
                    depth_brace -= 1;
                }
                TokenKind::Punct(Punct::LBracket) => depth_brack += 1,
                TokenKind::Punct(Punct::RBracket) => depth_brack -= 1,
                TokenKind::Punct(Punct::Semicolon)
                    if depth_paren == 0 && depth_brace == 0 && depth_brack == 0 =>
                {
                    return Some(j);
                }
                TokenKind::Eof => return Some(j.saturating_sub(1).max(from)),
                _ => {}
            }
            j += 1;
        }
        Some(self.toks.len().saturating_sub(1))
    }

    /// True iff the token at index `i` ends an expression (so the next
    /// token can be a postfix operator like `!` or a cast keyword `as`).
    fn is_expr_terminator(&self, i: usize) -> bool {
        match &self.toks[i].kind {
            TokenKind::Ident(_)
            | TokenKind::Number(_, _)
            | TokenKind::BigInt(_, _)
            | TokenKind::String(_)
            | TokenKind::Template { .. }
            | TokenKind::Punct(Punct::RParen)
            | TokenKind::Punct(Punct::RBracket)
            | TokenKind::Punct(Punct::RBrace) => true,
            _ => false,
        }
    }

    fn next_is_postfix_context(&self, i: usize) -> bool {
        if i >= self.toks.len() { return true; }
        match &self.toks[i].kind {
            TokenKind::Eof
            | TokenKind::Punct(Punct::Semicolon)
            | TokenKind::Punct(Punct::Comma)
            | TokenKind::Punct(Punct::RParen)
            | TokenKind::Punct(Punct::RBracket)
            | TokenKind::Punct(Punct::RBrace)
            | TokenKind::Punct(Punct::Dot)
            | TokenKind::Punct(Punct::OptionalChain)
            | TokenKind::Punct(Punct::Plus) | TokenKind::Punct(Punct::Minus)
            | TokenKind::Punct(Punct::Star) | TokenKind::Punct(Punct::Slash)
            | TokenKind::Punct(Punct::Percent) | TokenKind::Punct(Punct::StarStar)
            | TokenKind::Punct(Punct::Lt) | TokenKind::Punct(Punct::Gt)
            | TokenKind::Punct(Punct::Le) | TokenKind::Punct(Punct::Ge)
            | TokenKind::Punct(Punct::Eq) | TokenKind::Punct(Punct::Ne)
            | TokenKind::Punct(Punct::StrictEq) | TokenKind::Punct(Punct::StrictNe)
            | TokenKind::Punct(Punct::LogicalAnd) | TokenKind::Punct(Punct::LogicalOr)
            | TokenKind::Punct(Punct::NullishCoalesce)
            | TokenKind::Punct(Punct::Question)
            | TokenKind::Punct(Punct::Colon)
            => true,
            _ => false,
        }
    }

    /// True iff the `:` at index `i` is an annotation (vs ternary /
    /// object-key / label / case).
    fn is_annotation_colon(&self, i: usize) -> bool {
        if i == 0 { return false; }
        // Inside an object literal, an Ident-anchored `:` is a
        // key:value separator, not an annotation. BUT a `)`-anchored
        // `:` inside an object literal is a method-shorthand return-
        // type annotation (`{ method(): T { } }`), which IS an
        // annotation. Defer the bail decision until after anchor
        // resolution.
        let in_obj_lit = matches!(self.brace_stack.last(), Some(BraceCtx::ObjectLit));
        // Annotation contexts: after Ident or after `)` (return type),
        // BUT NOT inside a ternary (preceded by an expr starting with
        // `?` higher up — harder to detect) and NOT inside an object
        // literal value (the Ident is a key in {key: value} shape).
        // Treat `?` as transparent for the purpose of locating the
        // anchor token — `x?: T` should annotate against `x`.
        let mut anchor = i - 1;
        if matches!(self.toks[anchor].kind, TokenKind::Punct(Punct::Question)) && anchor > 0 {
            anchor -= 1;
        }
        let prev = &self.toks[anchor];
        let prev_is_close_paren = matches!(prev.kind, TokenKind::Punct(Punct::RParen));
        let prev_is_ident = matches!(prev.kind, TokenKind::Ident(_));
        let prev_is_close_brack = matches!(prev.kind, TokenKind::Punct(Punct::RBracket));
        let prev_is_close_brace = matches!(prev.kind, TokenKind::Punct(Punct::RBrace));
        if !(prev_is_close_paren || prev_is_ident || prev_is_close_brack || prev_is_close_brace) {
            return false;
        }
        // Apply the object-literal bail only for Ident-anchored `:`s
        // (the key:value case) AND only when we're at the obj-lit's
        // own level (paren_depth == 0 at this brace depth). A `:`
        // inside an `(...)` enclosed by an obj-lit is a function-param
        // annotation, not a key:value separator.
        if in_obj_lit && prev_is_ident && self.paren_depth == 0 {
            return false;
        }
        // TRCAPS-EXT 1: destructured-pattern parameter annotation
        // `function f({a, b}: T)` or `function f([x, y]: T)`. When the
        // anchor is RBrace/RBracket, only accept as annotation if
        // skip_type lands on a param-list terminator (`,` or `)` or
        // `=`). Else bail — could be e.g. a block statement followed
        // by a labelled statement (rare but possible).
        if prev_is_close_brace || prev_is_close_brack {
            let after = self.skip_type(i + 1);
            if after < self.toks.len() {
                return matches!(self.toks[after].kind,
                    TokenKind::Punct(Punct::Comma)
                    | TokenKind::Punct(Punct::RParen)
                    | TokenKind::Punct(Punct::Assign)
                    | TokenKind::Eof
                );
            }
            return false;
        }
        // TRCAPS-EXT 1: ternary `(...condition...) ? then : else`
        // disambig. When anchor is `)`, the `:` could be either a
        // return-type annotation (`function f(): T {`) or a ternary's
        // else-branch (`return cond(...) ? a : b`). Distinguish by
        // skip_type's landing:
        //   - `{` or `=>` → function/arrow body opener → annotation
        //   - `;` or `,` at param-list/decl boundary → annotation
        //   - `}` or anything else → ternary or unknown; bail
        if prev_is_close_paren {
            let after = self.skip_type(i + 1);
            if after < self.toks.len() {
                return matches!(self.toks[after].kind,
                    TokenKind::Punct(Punct::LBrace)
                    | TokenKind::Punct(Punct::Arrow)
                    | TokenKind::Punct(Punct::Semicolon)
                    | TokenKind::Punct(Punct::Comma)
                    | TokenKind::Punct(Punct::Assign)
                    | TokenKind::Eof
                );
            }
            return false;
        }
        // Look back further to filter object-literal-key contexts.
        // Heuristic: in an object literal, the token before the key Ident
        // is `{` or `,`. In a let/param/field/return context, it's
        // `let`/`const`/`var`/`(`/`,` (param-list)/`:` is fine in
        // class field but those are also annotations we want to strip.
        //
        // The conservative rule we use: if the Ident is immediately
        // preceded by `,` or `{` AND there is NO type-position context
        // (function signature, let-binding) above, treat as object key
        // and bail. The full detection needs a paren-stack; here we
        // bail only on the obvious `{ key: value }` shape:
        if prev_is_ident && anchor >= 1 {
            let two_back = &self.toks[anchor - 1].kind;
            if matches!(two_back,
                TokenKind::Punct(Punct::LBrace)
                | TokenKind::Punct(Punct::Comma)
            ) {
                // Could be object-literal-key OR destructuring pattern
                // OR param list. Disambiguate by scanning the type:
                // an object-literal value can be ANY expression; a
                // type annotation can only be a type. If skip_type
                // lands cleanly on `,`/`)`/`;`/`}`/`=`, accept as
                // annotation. Otherwise bail.
                let after = self.skip_type(i + 1);
                if after < self.toks.len() {
                    return matches!(self.toks[after].kind,
                        TokenKind::Punct(Punct::Comma)
                        | TokenKind::Punct(Punct::RParen)
                        | TokenKind::Punct(Punct::RBrace)
                        | TokenKind::Punct(Punct::Semicolon)
                        | TokenKind::Punct(Punct::Assign)
                        | TokenKind::Eof
                    );
                }
                return true;
            }
        }
        true
    }

    /// Skip a TS type expression starting at `i`; return index of token
    /// AFTER the type. Balances <> [] {} () and consumes type-operator
    /// tokens (`|`, `&`, `=>`, `[]`, `.`, `<>` for generics).
    fn skip_type(&self, mut i: usize) -> usize {
        let mut depth_angle = 0i32;
        let mut depth_paren = 0i32;
        let mut depth_brace = 0i32;
        let mut depth_brack = 0i32;
        let start = i;
        // TRGC-EXT 1 follow-on: track whether the last top-level token
        // closed a balanced `(...)`. If true AND the next top-level
        // token is `=>`, we're inside a function-type `(args) => ret`
        // — consume the `=>`. Otherwise top-level `=>` is the value-
        // position arrow that ends the annotation (e.g. `: T => body`).
        let mut prev_was_rparen_at_top = false;
        while i < self.toks.len() {
            let stop = matches!(self.toks[i].kind,
                TokenKind::Eof
                | TokenKind::Punct(Punct::Semicolon)
            );
            if stop { break; }
            let at_top = depth_angle == 0 && depth_paren == 0 && depth_brace == 0 && depth_brack == 0;
            if at_top {
                // TRGC-EXT 2 follow-on: ASI-aware top-level break.
                // Class-field annotations end at line-terminator before
                // the next member: `readonly x: T\n  y: U`. Without
                // this break, skip_type consumes past `T` into `y`'s
                // declaration.
                if i > start && self.toks[i].preceded_by_line_terminator {
                    break;
                }
                // Stoppers at top level: `,`, `=`, `)`, `}`, `]`, `{` (body start),
                // `=>` (only when not after balanced `(...)`).
                match &self.toks[i].kind {
                    TokenKind::Punct(Punct::Comma)
                    | TokenKind::Punct(Punct::Assign)
                    | TokenKind::Punct(Punct::RParen)
                    | TokenKind::Punct(Punct::RBrace)
                    | TokenKind::Punct(Punct::RBracket) => break,
                    TokenKind::Punct(Punct::Arrow) => {
                        if !prev_was_rparen_at_top { break; }
                        // Else fall through; consume as fn-type arrow.
                    }
                    TokenKind::Punct(Punct::LBrace) => {
                        // `{ x: T }` at the START of the type position
                        // is an object type literal — descend. After
                        // we've consumed type tokens, a top-level `{`
                        // is the function/initializer body — stop.
                        if i == start {
                            depth_brace += 1;
                        } else {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            match &self.toks[i].kind {
                TokenKind::Punct(Punct::Lt) => depth_angle += 1,
                TokenKind::Punct(Punct::Gt) => depth_angle = (depth_angle - 1).max(0),
                TokenKind::Punct(Punct::Shr) => depth_angle = (depth_angle - 2).max(0),
                TokenKind::Punct(Punct::LParen) => depth_paren += 1,
                TokenKind::Punct(Punct::RParen) if depth_paren > 0 => depth_paren -= 1,
                TokenKind::Punct(Punct::LBrace) if !at_top => depth_brace += 1,
                TokenKind::Punct(Punct::RBrace) if depth_brace > 0 => depth_brace -= 1,
                TokenKind::Punct(Punct::LBracket) => depth_brack += 1,
                TokenKind::Punct(Punct::RBracket) if depth_brack > 0 => depth_brack -= 1,
                _ => {}
            }
            // Set prev_was_rparen_at_top AFTER bracket-depth updates,
            // so a `)` that just popped to zero is recognized as a
            // top-level RParen on the next iteration.
            prev_was_rparen_at_top = matches!(self.toks[i].kind, TokenKind::Punct(Punct::RParen))
                && depth_angle == 0 && depth_paren == 0 && depth_brace == 0 && depth_brack == 0;
            i += 1;
        }
        i
    }
}
