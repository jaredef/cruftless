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

use rusty_js_parser::{Lexer, LexerGoal, Token, TokenKind, Punct, Span};
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
}

impl<'src> Scanner<'src> {
    fn new(src: &'src str) -> Self {
        Scanner {
            src,
            toks: Vec::new(),
            strips: Vec::new(),
            witnesses: Vec::new(),
            brace_stack: Vec::new(),
        }
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
                || n == "typeof"
                || n == "delete"
                || n == "void"
                || n == "new" => BraceCtx::ObjectLit,
            _ => BraceCtx::Block,
        }
    }

    fn lex_all(&mut self) -> Result<(), StripError> {
        let mut lx = Lexer::new(self.src);
        loop {
            let t = lx.next_token(LexerGoal::Div)
                .map_err(|e| StripError {
                    message: format!("lex: {:?}", e),
                    pos: lx.pos(),
                })?;
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
        // Inside an object literal, `:` is always a key separator, not
        // an annotation. Bail unconditionally.
        if matches!(self.brace_stack.last(), Some(BraceCtx::ObjectLit)) {
            return false;
        }
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
        if !(prev_is_close_paren || prev_is_ident || prev_is_close_brack) {
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
        while i < self.toks.len() {
            let stop = matches!(self.toks[i].kind,
                TokenKind::Eof
                | TokenKind::Punct(Punct::Semicolon)
            );
            if stop { break; }
            let at_top = depth_angle == 0 && depth_paren == 0 && depth_brace == 0 && depth_brack == 0;
            if at_top {
                // Stoppers at top level: `,`, `=`, `)`, `}`, `]`, `{` (body start)
                match &self.toks[i].kind {
                    TokenKind::Punct(Punct::Comma)
                    | TokenKind::Punct(Punct::Assign)
                    | TokenKind::Punct(Punct::RParen)
                    | TokenKind::Punct(Punct::RBrace)
                    | TokenKind::Punct(Punct::RBracket) => break,
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
            i += 1;
        }
        i
    }
}
