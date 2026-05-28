//! ECMAScript lexer per specs/ecma262-lexical.spec.md.
//!
//! The lexer operates on a `&str` source and emits a stream of `Token`s.
//! Caller controls goal-symbol selection (InputElementDiv vs
//! InputElementRegExp vs InputElementTemplateTail) via `LexerGoal` —
//! ECMA-262 §12.1 specifies that the choice is context-dependent and
//! resolves a `/` token as DivPunctuator vs RegularExpressionLiteral.
//!
//! Module-only in v1: legacy octal integer/escape sequences and
//! HTML-comment Annex B extensions are rejected outright. Sloppy
//! script-goal extension is a successor refinement.

use crate::token::{NumberKind, Punct, Span, TemplatePart, Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexerGoal {
    /// After binary-operator-like positions: `/` is DivPunctuator.
    Div,
    /// After punctuators that could begin a new expression: `/` opens a regex.
    RegExp,
    /// Inside a template literal after `${...}` returns: `}` closes the
    /// substitution and the next characters are TemplateMiddle/Tail.
    TemplateTail,
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
    pub message: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexErrorKind {
    UnterminatedString,
    UnterminatedTemplate,
    UnterminatedRegex,
    UnterminatedComment,
    InvalidEscape,
    InvalidNumeric,
    InvalidIdentifier,
    LegacyOctalInModule,
    UnexpectedChar,
}

pub struct Lexer<'src> {
    src: &'src [u8],
    /// Byte offset into `src`.
    pos: usize,
    /// True when the last consumed run of whitespace contained a LineTerminator
    /// (or a MultiLineComment containing one). Resets at each non-whitespace token.
    saw_line_terminator: bool,
    /// True until the first non-trivia token. Hashbang only allowed at start.
    at_start: bool,
    /// SLEC-EXT 1: lexer-side strict-mode flag for legacy-escape rejection.
    /// Per §12.9.4 + Annex B B.1.2, strict-mode forbids `\1`-`\7` and
    /// `\0` followed by a digit in string literals. The parser pushes
    /// its strict_mode state into the lexer via `set_strict` when the
    /// state flips (after a "use strict" directive or on module entry).
    pub(crate) strict_mode: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
            saw_line_terminator: false,
            at_start: true,
            strict_mode: false,
        }
    }

    /// SLEC-EXT 1: set the lexer's strict-mode flag. The Parser owns the
    /// canonical strict_mode state; this updates the lexer's view so the
    /// legacy-escape rejection rules apply.
    pub fn set_strict(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// Reposition the lexer. Used by the parser for goal-symbol re-lexing
    /// (template-tail re-fetch when leaving a substitution).
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
        self.saw_line_terminator = false;
        self.at_start = false;
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    // LTC-EXT 1: return the byte-length of a LineTerminator at the cursor,
    // or None. Covers LF (1), CR (1), CRLF (2), U+2028/U+2029 (3).
    fn peek_lt_bytes(&self) -> Option<usize> {
        let c = self.peek_byte()?;
        if c == b'\n' {
            return Some(1);
        }
        if c == b'\r' {
            return Some(if self.peek_byte_at(1) == Some(b'\n') {
                2
            } else {
                1
            });
        }
        if c == 0xE2 && self.peek_byte_at(1) == Some(0x80) {
            return match self.peek_byte_at(2) {
                Some(0xA8) | Some(0xA9) => Some(3),
                _ => None,
            };
        }
        None
    }

    // LTC-EXT 1: post-numeric "identifier-start follows" check that excludes
    // LineTerminator and Unicode whitespace at high-byte positions. Otherwise
    // bytes like 0xE2 (first byte of LS/PS, etc.) would falsely register as
    // identifier-start under the ASCII-fast-path predicate.
    fn peek_is_ident_start_strict(&self) -> bool {
        let Some(c) = self.peek_byte() else {
            return false;
        };
        if c < 0x80 {
            return is_identifier_start_byte(c);
        }
        if self.peek_lt_bytes().is_some() {
            return false;
        }
        if let Some(cp) = self.peek_codepoint() {
            if is_unicode_whitespace(cp) {
                return false;
            }
        }
        true
    }

    /// Lex one token under the given goal. Consumers re-pick the goal at
    /// each call site based on parser context.
    pub fn next_token(&mut self, goal: LexerGoal) -> Result<Token, LexError> {
        // Hashbang at source start (before any trivia).
        if self.at_start && self.peek_str(2) == Some("#!") {
            let start = self.pos;
            self.pos += 2;
            while let Some(c) = self.peek_byte() {
                if is_line_terminator_byte(c) {
                    break;
                }
                self.advance_one_char();
            }
            self.at_start = false;
            let lexeme = std::str::from_utf8(&self.src[start..self.pos])
                .unwrap()
                .to_string();
            return Ok(Token {
                kind: TokenKind::Hashbang(lexeme),
                span: Span::new(start, self.pos),
                preceded_by_line_terminator: false,
            });
        }
        self.at_start = false;

        self.saw_line_terminator = false;
        self.skip_trivia()?;
        let preceded_by_lt = self.saw_line_terminator;

        let start = self.pos;
        let Some(c) = self.peek_byte() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: Span::new(start, start),
                preceded_by_line_terminator: preceded_by_lt,
            });
        };

        // RightBracePunctuator under TemplateTail goal — close substitution.
        if goal == LexerGoal::TemplateTail && c == b'}' {
            self.pos += 1;
            return self.continue_template(start, preceded_by_lt);
        }

        // Identifier / reserved word / private identifier
        if c == b'#' {
            self.pos += 1;
            let name = self.read_identifier_name().ok_or_else(|| {
                self.err(
                    LexErrorKind::InvalidIdentifier,
                    start,
                    "expected identifier after #",
                )
            })?;
            return Ok(Token {
                kind: TokenKind::PrivateIdent(name),
                span: Span::new(start, self.pos),
                preceded_by_line_terminator: preceded_by_lt,
            });
        }
        if is_identifier_start_byte(c) || c == b'\\' {
            let name = self.read_identifier_name().ok_or_else(|| {
                self.err(LexErrorKind::InvalidIdentifier, start, "invalid identifier")
            })?;
            return Ok(Token {
                kind: TokenKind::Ident(name),
                span: Span::new(start, self.pos),
                preceded_by_line_terminator: preceded_by_lt,
            });
        }

        // Numeric literal
        if c.is_ascii_digit()
            || (c == b'.' && self.peek_byte_at(1).map_or(false, |b| b.is_ascii_digit()))
        {
            return self.read_numeric_literal(start, preceded_by_lt);
        }

        // String literal
        if c == b'"' || c == b'\'' {
            return self.read_string_literal(start, preceded_by_lt, c);
        }

        // Template literal (no-substitution or head)
        if c == b'`' {
            self.pos += 1;
            return self.read_template_segment(start, preceded_by_lt, true);
        }

        // Regex literal (only under RegExp goal)
        if c == b'/' && goal == LexerGoal::RegExp {
            return self.read_regex_literal(start, preceded_by_lt);
        }

        // Punctuator
        self.read_punctuator(start, preceded_by_lt)
    }

    // ────────────── Trivia (whitespace, line terminators, comments) ──────────────

    fn skip_trivia(&mut self) -> Result<(), LexError> {
        loop {
            let Some(c) = self.peek_byte() else {
                return Ok(());
            };
            if is_whitespace_byte(c) {
                self.advance_one_char();
                continue;
            }
            if is_line_terminator_byte(c) {
                self.saw_line_terminator = true;
                // CR LF as a single line terminator.
                if c == b'\r' && self.peek_byte_at(1) == Some(b'\n') {
                    self.pos += 2;
                } else {
                    self.advance_one_char();
                }
                continue;
            }
            // HLCL-EXT 1: Annex B B.1.3 SingleLineHTMLOpenComment `<!--`
            // and SingleLineHTMLCloseComment `-->`. Both behave like `//`
            // — consume the rest of the line. Per spec the `-->` form
            // requires LT-preceded position (or start-of-source); cruft's
            // saw_line_terminator tracks that. Always-on; matches cruft's
            // other Annex-B acceptances (legacy octal, etc.).
            if c == b'<'
                && self.peek_byte_at(1) == Some(b'!')
                && self.peek_byte_at(2) == Some(b'-')
                && self.peek_byte_at(3) == Some(b'-')
            {
                self.pos += 4;
                while let Some(b) = self.peek_byte() {
                    if is_line_terminator_byte(b) || self.peek_lt_bytes().is_some() {
                        break;
                    }
                    self.advance_one_char();
                }
                continue;
            }
            if c == b'-'
                && self.peek_byte_at(1) == Some(b'-')
                && self.peek_byte_at(2) == Some(b'>')
                && (self.at_start || self.saw_line_terminator)
            {
                self.pos += 3;
                while let Some(b) = self.peek_byte() {
                    if is_line_terminator_byte(b) || self.peek_lt_bytes().is_some() {
                        break;
                    }
                    self.advance_one_char();
                }
                continue;
            }
            if c == b'/' {
                match self.peek_byte_at(1) {
                    Some(b'/') => {
                        self.pos += 2;
                        while let Some(c) = self.peek_byte() {
                            if is_line_terminator_byte(c) {
                                break;
                            }
                            // LTC-EXT 1: U+2028/U+2029 also terminate the line comment.
                            if self.peek_lt_bytes().is_some() {
                                break;
                            }
                            self.advance_one_char();
                            let _ = c;
                        }
                        continue;
                    }
                    Some(b'*') => {
                        let start = self.pos;
                        self.pos += 2;
                        let mut closed = false;
                        while self.pos < self.src.len() {
                            let b = self.src[self.pos];
                            if is_line_terminator_byte(b) {
                                self.saw_line_terminator = true;
                            }
                            if b == b'*' && self.peek_byte_at(1) == Some(b'/') {
                                self.pos += 2;
                                closed = true;
                                break;
                            }
                            self.advance_one_char();
                        }
                        if !closed {
                            return Err(self.err(
                                LexErrorKind::UnterminatedComment,
                                start,
                                "unterminated /* */ comment",
                            ));
                        }
                        continue;
                    }
                    _ => return Ok(()),
                }
            }
            // ZWNBSP (BOM) U+FEFF is whitespace per spec.
            if c == 0xEF && self.peek_byte_at(1) == Some(0xBB) && self.peek_byte_at(2) == Some(0xBF)
            {
                self.pos += 3;
                continue;
            }
            // Non-ASCII whitespace (NBSP, etc.) handled by multi-byte check.
            if c >= 0x80 {
                if let Some(cp) = self.peek_codepoint() {
                    if is_unicode_whitespace(cp) {
                        let len = utf8_len(c);
                        self.pos += len;
                        continue;
                    }
                    if cp == 0x2028 || cp == 0x2029 {
                        self.saw_line_terminator = true;
                        self.pos += 3;
                        continue;
                    }
                }
            }
            return Ok(());
        }
    }

    // ────────────── IdentifierName ──────────────

    fn read_identifier_name(&mut self) -> Option<String> {
        let mut out = String::new();
        // Start
        let cp = self.consume_identifier_codepoint(true)?;
        push_char(&mut out, cp);
        // Continuation
        while let Some(cp) = self.consume_identifier_codepoint(false) {
            push_char(&mut out, cp);
        }
        Some(out)
    }

    /// Consume one identifier code-point. For the start position, only
    /// IdentifierStart code points + `\u`-escapes are accepted; for
    /// continuation, IdentifierPart code points + ZWNJ/ZWJ + escapes.
    fn consume_identifier_codepoint(&mut self, is_start: bool) -> Option<u32> {
        if self.peek_byte() == Some(b'\\') {
            // UnicodeEscapeSequence
            let save = self.pos;
            self.pos += 1;
            if self.peek_byte() != Some(b'u') {
                self.pos = save;
                return None;
            }
            self.pos += 1;
            let cp = self.read_unicode_escape_inner()?;
            if is_start {
                if !is_id_start(cp) {
                    self.pos = save;
                    return None;
                }
            } else if !is_id_continue(cp) {
                self.pos = save;
                return None;
            }
            return Some(cp);
        }
        let cp = self.peek_codepoint()?;
        if is_start {
            if !is_id_start(cp) {
                return None;
            }
        } else if !is_id_continue(cp) {
            return None;
        }
        let len = utf8_len(self.src[self.pos]);
        self.pos += len;
        Some(cp)
    }

    fn read_unicode_escape_inner(&mut self) -> Option<u32> {
        // After `\u` already consumed.
        if self.peek_byte() == Some(b'{') {
            self.pos += 1;
            let mut val: u32 = 0;
            let mut count = 0;
            while let Some(c) = self.peek_byte() {
                if c == b'}' {
                    break;
                }
                let d = hex_digit_value(c)?;
                val = val.checked_mul(16)?.checked_add(d as u32)?;
                if val > 0x10FFFF {
                    return None;
                }
                self.pos += 1;
                count += 1;
                if count > 6 {
                    return None;
                }
            }
            if self.peek_byte() != Some(b'}') {
                return None;
            }
            // SLEC-EXT 1: §12.9.4 — \u{...} requires at least one hex digit.
            // Pre-fix accepted empty braces \u{} as codepoint 0; spec mandates
            // SyntaxError. The CodePoint :: HexDigits production requires
            // ≥1 HexDigit.
            if count == 0 {
                return None;
            }
            self.pos += 1;
            Some(val)
        } else {
            // Exactly 4 hex digits.
            let mut val: u32 = 0;
            for _ in 0..4 {
                let c = self.peek_byte()?;
                let d = hex_digit_value(c)?;
                val = val * 16 + d as u32;
                self.pos += 1;
            }
            Some(val)
        }
    }

    // ────────────── Numeric literals ──────────────

    fn read_numeric_literal(
        &mut self,
        start: usize,
        preceded_by_lt: bool,
    ) -> Result<Token, LexError> {
        let first = self.src[self.pos];

        // Radix-prefixed forms: 0x / 0b / 0o
        if first == b'0' {
            if let Some(next) = self.peek_byte_at(1) {
                match next {
                    b'x' | b'X' => {
                        return self.read_radix_int(
                            start,
                            preceded_by_lt,
                            NumberKind::Hex,
                            16,
                            |b| b.is_ascii_hexdigit(),
                        )
                    }
                    b'b' | b'B' => {
                        return self.read_radix_int(
                            start,
                            preceded_by_lt,
                            NumberKind::Binary,
                            2,
                            |b| b == b'0' || b == b'1',
                        )
                    }
                    b'o' | b'O' => {
                        return self.read_radix_int(
                            start,
                            preceded_by_lt,
                            NumberKind::Octal,
                            8,
                            |b| (b'0'..=b'7').contains(&b),
                        )
                    }
                    _ => {}
                }
            }
        }

        // Decimal (potentially with leading `0` then more digits — legacy octal
        // or non-octal-decimal-integer; both rejected in module mode).
        let mut has_digits_before_dot = false;
        while let Some(c) = self.peek_byte() {
            if c.is_ascii_digit() || c == b'_' {
                if c == b'_'
                    && (!has_digits_before_dot
                        || !self.peek_byte_at(1).map_or(false, |b| b.is_ascii_digit()))
                {
                    return Err(self.err(
                        LexErrorKind::InvalidNumeric,
                        start,
                        "invalid numeric separator",
                    ));
                }
                // LEP-EXT 2 / NLC follow-on: §12.8.1 — separators are not
                // allowed inside legacy-octal-like leading-zero forms
                // (`0_0`, `0_8`, `00_1`, `08_0`, etc.). Detect via:
                // first byte is `0`, and we encounter `_` at any position.
                // Since legacy-octal forms have NO separators in the spec
                // grammar, any `_` after a leading `0` is invalid.
                if c == b'_' && first == b'0' {
                    return Err(self.err(
                        LexErrorKind::InvalidNumeric,
                        start,
                        "numeric separator not allowed in legacy-octal-like leading-zero form",
                    ));
                }
                if c.is_ascii_digit() {
                    has_digits_before_dot = true;
                }
                self.pos += 1;
            } else {
                break;
            }
        }
        // Reject leading-zero forms (legacy octal, non-octal-decimal-integer)
        // NLC-EXT 1 (revised; replaces the prior 'module-mode' check that
        // was firing backwards). §12.8 + Annex B B.1.1: legacy octal
        // (`0` + octal digits) and non-octal-decimal (`08`/`09`) integer
        // literals are forbidden in STRICT mode (script + module). In
        // sloppy script mode they are accepted via Annex B.
        //
        // Pre-fix: this check fired unconditionally as 'forbidden in
        // module code,' which (a) wrongly rejected sloppy-script-mode
        // legacy octals like `0001` and (b) failed to reject strict-mode
        // legacy octals at all. The LegacyOctalInModule LexErrorKind name
        // is retained for git-blame continuity; semantically it now fires
        // on strict-mode legacy forms.
        if first == b'0' && self.pos > start + 1 && self.strict_mode {
            let second = self.src[start + 1];
            if second.is_ascii_digit() {
                return Err(self.err(
                    LexErrorKind::LegacyOctalInModule,
                    start,
                    "legacy octal/non-octal-decimal integer literal in strict mode",
                ));
            }
        }
        // Fractional part
        let mut has_dot = false;
        if self.peek_byte() == Some(b'.') {
            has_dot = true;
            self.pos += 1;
            while let Some(c) = self.peek_byte() {
                if c.is_ascii_digit() || c == b'_' {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        // Exponent
        let mut has_exp = false;
        if matches!(self.peek_byte(), Some(b'e') | Some(b'E')) {
            has_exp = true;
            self.pos += 1;
            if matches!(self.peek_byte(), Some(b'+') | Some(b'-')) {
                self.pos += 1;
            }
            let exp_start = self.pos;
            while let Some(c) = self.peek_byte() {
                if c.is_ascii_digit() || c == b'_' {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            if self.pos == exp_start {
                return Err(self.err(
                    LexErrorKind::InvalidNumeric,
                    start,
                    "exponent has no digits",
                ));
            }
        }
        // BigInt suffix — only allowed on integer (no dot, no exp)
        if self.peek_byte() == Some(b'n') {
            if has_dot || has_exp {
                return Err(self.err(
                    LexErrorKind::InvalidNumeric,
                    start,
                    "BigInt suffix on non-integer",
                ));
            }
            let digits = std::str::from_utf8(&self.src[start..self.pos])
                .unwrap()
                .replace('_', "");
            self.pos += 1;
            return Ok(Token {
                kind: TokenKind::BigInt(digits, NumberKind::Decimal),
                span: Span::new(start, self.pos),
                preceded_by_line_terminator: preceded_by_lt,
            });
        }
        // Disallow ident-start immediately after numeric (e.g., 1abc).
        // LTC-EXT 1: high-byte LT (U+2028/U+2029) and Unicode whitespace
        // must not trigger the reject — they are token separators per §11.3.
        if self.peek_is_ident_start_strict() {
            return Err(self.err(
                LexErrorKind::InvalidNumeric,
                start,
                "identifier directly after numeric literal",
            ));
        }
        let lexeme = std::str::from_utf8(&self.src[start..self.pos])
            .unwrap()
            .replace('_', "");
        let value: f64 = lexeme.parse().map_err(|_| {
            self.err(
                LexErrorKind::InvalidNumeric,
                start,
                "malformed numeric literal",
            )
        })?;
        Ok(Token {
            kind: TokenKind::Number(value, NumberKind::Decimal),
            span: Span::new(start, self.pos),
            preceded_by_line_terminator: preceded_by_lt,
        })
    }

    fn read_radix_int<F: Fn(u8) -> bool>(
        &mut self,
        start: usize,
        preceded_by_lt: bool,
        kind: NumberKind,
        radix: u32,
        is_digit: F,
    ) -> Result<Token, LexError> {
        self.pos += 2; // consume "0x" / "0b" / "0o"
        let digits_start = self.pos;
        let mut last_was_underscore = false;
        let mut has_digits = false;
        while let Some(c) = self.peek_byte() {
            if c == b'_' {
                if last_was_underscore || !has_digits {
                    return Err(self.err(
                        LexErrorKind::InvalidNumeric,
                        start,
                        "invalid numeric separator",
                    ));
                }
                last_was_underscore = true;
                self.pos += 1;
            } else if is_digit(c) {
                has_digits = true;
                last_was_underscore = false;
                self.pos += 1;
            } else {
                break;
            }
        }
        if !has_digits || last_was_underscore {
            return Err(self.err(
                LexErrorKind::InvalidNumeric,
                start,
                "invalid radix-prefixed literal",
            ));
        }
        // LEP-EXT 2 / NLC follow-on: §12.8.3 — after the radix-prefixed
        // digit run ends, a following ASCII digit (e.g. `0b14` where `4`
        // is invalid for binary, or `0o89` where `8/9` invalid for octal)
        // OR a following identifier-start char (`0b1abc`) must be
        // rejected as InvalidNumeric. Carve-out: `n` is the BigIntLiteral
        // suffix (`0xffn`, `0b1n`) — handled by the next branch below,
        // not rejected here.
        if let Some(b) = self.peek_byte() {
            let high_lt_or_ws = b >= 0x80
                && (self.peek_lt_bytes().is_some()
                    || self.peek_codepoint().map_or(false, is_unicode_whitespace));
            if b != b'n' && !high_lt_or_ws && (b.is_ascii_digit() || is_identifier_start_byte(b)) {
                return Err(self.err(
                    LexErrorKind::InvalidNumeric,
                    start,
                    "invalid character after radix-prefixed literal",
                ));
            }
        }
        let digits = std::str::from_utf8(&self.src[digits_start..self.pos])
            .unwrap()
            .replace('_', "");
        // BigInt suffix
        if self.peek_byte() == Some(b'n') {
            self.pos += 1;
            return Ok(Token {
                kind: TokenKind::BigInt(digits, kind),
                span: Span::new(start, self.pos),
                preceded_by_line_terminator: preceded_by_lt,
            });
        }
        let value = u128::from_str_radix(&digits, radix).map_err(|_| {
            self.err(
                LexErrorKind::InvalidNumeric,
                start,
                "out-of-range radix-prefixed literal",
            )
        })?;
        Ok(Token {
            kind: TokenKind::Number(value as f64, kind),
            span: Span::new(start, self.pos),
            preceded_by_line_terminator: preceded_by_lt,
        })
    }

    // ────────────── String literals ──────────────

    fn read_string_literal(
        &mut self,
        start: usize,
        preceded_by_lt: bool,
        quote: u8,
    ) -> Result<Token, LexError> {
        self.pos += 1; // consume opening quote
        let mut out = String::new();
        loop {
            let Some(c) = self.peek_byte() else {
                return Err(self.err(
                    LexErrorKind::UnterminatedString,
                    start,
                    "unterminated string",
                ));
            };
            if c == quote {
                self.pos += 1;
                break;
            }
            // ECMA-262 §12.9.4.1 (post-2019 JSON-superset amendment): only
            // LF and CR are forbidden literally inside StringLiteral. U+2028
            // and U+2029 are now permitted as literal characters and pass
            // through unchanged.
            if is_line_terminator_byte(c) {
                return Err(self.err(
                    LexErrorKind::UnterminatedString,
                    start,
                    "line terminator in string",
                ));
            }
            if c == b'\\' {
                self.pos += 1;
                self.read_string_escape(start, &mut out)?;
                continue;
            }
            // Pass through one code point.
            let cp = self.peek_codepoint().ok_or_else(|| {
                self.err(LexErrorKind::UnterminatedString, start, "malformed UTF-8")
            })?;
            push_char(&mut out, cp);
            let len = utf8_len(c);
            self.pos += len;
        }
        Ok(Token {
            kind: TokenKind::String(out),
            span: Span::new(start, self.pos),
            preceded_by_line_terminator: preceded_by_lt,
        })
    }

    fn read_string_escape(&mut self, start: usize, out: &mut String) -> Result<(), LexError> {
        let Some(c) = self.peek_byte() else {
            return Err(self.err(LexErrorKind::InvalidEscape, start, "lone backslash"));
        };
        // LTC-EXT 1: \<LS> and \<PS> are LineContinuation per §12.9.4 —
        // consume the 3-byte sequence and contribute nothing to the string.
        if c == 0xE2 && self.peek_byte_at(1) == Some(0x80) {
            if let Some(b3) = self.peek_byte_at(2) {
                if b3 == 0xA8 || b3 == 0xA9 {
                    self.pos += 3;
                    return Ok(());
                }
            }
        }
        self.pos += 1;
        match c {
            b'n' => out.push('\n'),
            b'r' => out.push('\r'),
            b't' => out.push('\t'),
            b'b' => out.push('\u{0008}'),
            b'f' => out.push('\u{000C}'),
            b'v' => out.push('\u{000B}'),
            b'0' => {
                // Ω.5.P53.E8 + SLEC-EXT 1: legacy octal escape Annex B B.1.2.
                // `\0` alone (not followed by a digit) is the NULL escape
                // and is always allowed. `\0` followed by any digit is a
                // legacy octal escape; spec forbids it in strict mode.
                if self.peek_byte().map_or(false, |b| b.is_ascii_digit()) {
                    if self.strict_mode {
                        return Err(self.err(
                            LexErrorKind::InvalidEscape,
                            start,
                            "legacy octal escape sequence in strict mode",
                        ));
                    }
                    let mut v: u32 = 0;
                    let mut n = 0;
                    while n < 3 {
                        match self.peek_byte() {
                            Some(b) if (b'0'..=b'7').contains(&b) => {
                                v = v * 8 + (b - b'0') as u32;
                                self.pos += 1;
                                n += 1;
                            }
                            _ => break,
                        }
                    }
                    push_char(out, v);
                } else {
                    out.push('\0');
                }
            }
            b'\'' | b'"' | b'\\' => out.push(c as char),
            b'x' => {
                let hi = self
                    .peek_byte()
                    .and_then(|b| hex_digit_value(b))
                    .ok_or_else(|| {
                        self.err(LexErrorKind::InvalidEscape, start, "bad \\x escape")
                    })?;
                self.pos += 1;
                let lo = self
                    .peek_byte()
                    .and_then(|b| hex_digit_value(b))
                    .ok_or_else(|| {
                        self.err(LexErrorKind::InvalidEscape, start, "bad \\x escape")
                    })?;
                self.pos += 1;
                let cp = (hi * 16 + lo) as u32;
                push_char(out, cp);
            }
            b'u' => {
                let cp = self.read_unicode_escape_inner().ok_or_else(|| {
                    self.err(LexErrorKind::InvalidEscape, start, "bad \\u escape")
                })?;
                if (0xD800..=0xDBFF).contains(&cp)
                    && self.peek_byte() == Some(b'\\')
                    && self.peek_byte_at(1) == Some(b'u')
                {
                    let save = self.pos;
                    self.pos += 2;
                    if let Some(low) = self.read_unicode_escape_inner() {
                        if (0xDC00..=0xDFFF).contains(&low) {
                            let scalar = 0x10000 + ((cp - 0xD800) << 10) + (low - 0xDC00);
                            push_char(out, scalar);
                            return Ok(());
                        }
                    }
                    self.pos = save;
                }
                push_char(out, cp);
            }
            b'\n' => { /* line continuation — contributes nothing */ }
            b'\r' => {
                if self.peek_byte() == Some(b'\n') {
                    self.pos += 1;
                }
            }
            // Ω.5.P53.E8 + SLEC-EXT 1: legacy octal escape (Annex B B.1.2).
            // §12.9.4 + B.1.2: strict mode forbids `\1`-`\7` legacy octal.
            // Per the LegacyOctalEscapeSequence grammar, three-digit form is
            // restricted to leading 0-3 (ZeroToThree OctalDigit OctalDigit).
            // Leading 4-7 (FourToSeven OctalDigit) caps at two digits — a
            // following octal digit produces value-of-two-digit-octal then
            // the next char is treated as a literal. Pre-fix accepted
            // three-digit form for 4-7 leadings (e.g., \400 → 256); spec
            // says \400 is \40 (32) then literal "0".
            b'1'..=b'7' => {
                if self.strict_mode {
                    return Err(self.err(
                        LexErrorKind::InvalidEscape,
                        start,
                        "legacy octal escape sequence in strict mode",
                    ));
                }
                let mut v: u32 = (c - b'0') as u32;
                let leading_four_to_seven = matches!(c, b'4'..=b'7');
                let max_extra_digits = if leading_four_to_seven { 1 } else { 2 };
                let mut n = 0;
                while n < max_extra_digits {
                    match self.peek_byte() {
                        Some(b) if (b'0'..=b'7').contains(&b) => {
                            v = v * 8 + (b - b'0') as u32;
                            self.pos += 1;
                            n += 1;
                        }
                        _ => break,
                    }
                }
                push_char(out, v);
            }
            // §12.9.4 + B.1.2: strict mode forbids \8 and \9 (NonOctalDecimalEscapeSequence).
            b'8' | b'9' => {
                if self.strict_mode {
                    return Err(self.err(
                        LexErrorKind::InvalidEscape,
                        start,
                        "legacy non-octal decimal escape sequence in strict mode",
                    ));
                }
                out.push(c as char);
            }
            _ => out.push(c as char),
        }
        Ok(())
    }

    // ────────────── Template literals ──────────────

    fn read_template_segment(
        &mut self,
        start: usize,
        preceded_by_lt: bool,
        is_open: bool,
    ) -> Result<Token, LexError> {
        let mut cooked = String::new();
        let mut raw = String::new();
        let mut cooked_ok = true;
        loop {
            let Some(c) = self.peek_byte() else {
                return Err(self.err(
                    LexErrorKind::UnterminatedTemplate,
                    start,
                    "unterminated template",
                ));
            };
            if c == b'`' {
                self.pos += 1;
                return Ok(Token {
                    kind: TokenKind::Template {
                        cooked: if cooked_ok { Some(cooked) } else { None },
                        raw,
                        part: if is_open {
                            TemplatePart::NoSubstitution
                        } else {
                            TemplatePart::Tail
                        },
                    },
                    span: Span::new(start, self.pos),
                    preceded_by_line_terminator: preceded_by_lt,
                });
            }
            if c == b'$' && self.peek_byte_at(1) == Some(b'{') {
                self.pos += 2;
                return Ok(Token {
                    kind: TokenKind::Template {
                        cooked: if cooked_ok { Some(cooked) } else { None },
                        raw,
                        part: if is_open {
                            TemplatePart::Head
                        } else {
                            TemplatePart::Middle
                        },
                    },
                    span: Span::new(start, self.pos),
                    preceded_by_line_terminator: preceded_by_lt,
                });
            }
            if c == b'\\' {
                // Raw form preserves the backslash + next char(s) verbatim.
                let escape_start = self.pos;
                let template_decimal_escape_invalid_cooked = match self.peek_byte_at(1) {
                    Some(b'0') => self
                        .peek_byte_at(2)
                        .map_or(false, |b| b.is_ascii_digit()),
                    Some(b'1'..=b'9') => true,
                    _ => false,
                };
                self.pos += 1;
                if self.consume_forbidden_template_numeric_escape() {
                    cooked_ok = false;
                    raw.push_str(std::str::from_utf8(&self.src[escape_start..self.pos]).unwrap());
                    continue;
                }
                let mut buf = String::new();
                match self.read_string_escape(start, &mut buf) {
                    Ok(()) => cooked.push_str(&buf),
                    Err(_) => cooked_ok = false,
                }
                if template_decimal_escape_invalid_cooked {
                    cooked_ok = false;
                }
                raw.push_str(std::str::from_utf8(&self.src[escape_start..self.pos]).unwrap());
                continue;
            }
            if c == b'\r' {
                // Spec: \r and \r\n normalize to \n in both cooked and raw.
                cooked.push('\n');
                raw.push('\n');
                self.pos += 1;
                if self.peek_byte() == Some(b'\n') {
                    self.pos += 1;
                }
                continue;
            }
            let cp = self.peek_codepoint().ok_or_else(|| {
                self.err(LexErrorKind::UnterminatedTemplate, start, "malformed UTF-8")
            })?;
            push_char(&mut cooked, cp);
            push_char(&mut raw, cp);
            let len = utf8_len(c);
            self.pos += len;
        }
    }

    fn continue_template(&mut self, start: usize, preceded_by_lt: bool) -> Result<Token, LexError> {
        self.read_template_segment(start, preceded_by_lt, false)
    }

    fn consume_forbidden_template_numeric_escape(&mut self) -> bool {
        let Some(c) = self.peek_byte() else {
            return false;
        };
        match c {
            b'0' if self.peek_byte_at(1).map_or(false, |b| b.is_ascii_digit()) => {
                self.pos += 1;
                let mut n = 0;
                while n < 2 {
                    match self.peek_byte() {
                        Some(b) if (b'0'..=b'7').contains(&b) => {
                            self.pos += 1;
                            n += 1;
                        }
                        _ => break,
                    }
                }
                true
            }
            b'1'..=b'7' => {
                self.pos += 1;
                let leading_four_to_seven = matches!(c, b'4'..=b'7');
                let max_extra_digits = if leading_four_to_seven { 1 } else { 2 };
                let mut n = 0;
                while n < max_extra_digits {
                    match self.peek_byte() {
                        Some(b) if (b'0'..=b'7').contains(&b) => {
                            self.pos += 1;
                            n += 1;
                        }
                        _ => break,
                    }
                }
                true
            }
            b'8' | b'9' => {
                self.pos += 1;
                true
            }
            _ => false,
        }
    }

    // ────────────── Regex literals ──────────────

    fn read_regex_literal(
        &mut self,
        start: usize,
        preceded_by_lt: bool,
    ) -> Result<Token, LexError> {
        self.pos += 1; // consume `/`
        let body_start = self.pos;
        let mut in_class = false;
        loop {
            let Some(c) = self.peek_byte() else {
                return Err(self.err(LexErrorKind::UnterminatedRegex, start, "unterminated regex"));
            };
            if is_line_terminator_byte(c) || self.peek_lt_bytes().is_some() {
                return Err(self.err(
                    LexErrorKind::UnterminatedRegex,
                    start,
                    "line terminator in regex",
                ));
            }
            if c == b'\\' {
                self.pos += 1;
                if self
                    .peek_byte()
                    .map_or(true, |b| is_line_terminator_byte(b))
                    || self.peek_lt_bytes().is_some()
                {
                    return Err(self.err(
                        LexErrorKind::UnterminatedRegex,
                        start,
                        "bad escape in regex",
                    ));
                }
                self.advance_one_char();
                continue;
            }
            if c == b'[' {
                in_class = true;
                self.pos += 1;
                continue;
            }
            if c == b']' {
                in_class = false;
                self.pos += 1;
                continue;
            }
            if c == b'/' && !in_class {
                let body = std::str::from_utf8(&self.src[body_start..self.pos])
                    .unwrap()
                    .to_string();
                self.pos += 1;
                // Flags
                let flags_start = self.pos;
                while let Some(c) = self.peek_byte() {
                    if is_identifier_part_byte(c) {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                let flags = std::str::from_utf8(&self.src[flags_start..self.pos])
                    .unwrap()
                    .to_string();
                return Ok(Token {
                    kind: TokenKind::Regex { body, flags },
                    span: Span::new(start, self.pos),
                    preceded_by_line_terminator: preceded_by_lt,
                });
            }
            self.advance_one_char();
        }
    }

    // ────────────── Punctuators ──────────────

    fn read_punctuator(&mut self, start: usize, preceded_by_lt: bool) -> Result<Token, LexError> {
        // Helper: try to match the longest punctuator from the table.
        macro_rules! emit {
            ($p:expr, $len:expr) => {{
                self.pos += $len;
                Ok(Token {
                    kind: TokenKind::Punct($p),
                    span: Span::new(start, self.pos),
                    preceded_by_line_terminator: preceded_by_lt,
                })
            }};
        }

        let s0 = self.src[self.pos];
        let s1 = self.peek_byte_at(1);
        let s2 = self.peek_byte_at(2);
        let s3 = self.peek_byte_at(3);

        // 4-character punctuators
        if s0 == b'>' && s1 == Some(b'>') && s2 == Some(b'>') && s3 == Some(b'=') {
            return emit!(Punct::UShrAssign, 4);
        }
        // 3-character
        if s0 == b'.' && s1 == Some(b'.') && s2 == Some(b'.') {
            return emit!(Punct::Spread, 3);
        }
        if s0 == b'=' && s1 == Some(b'=') && s2 == Some(b'=') {
            return emit!(Punct::StrictEq, 3);
        }
        if s0 == b'!' && s1 == Some(b'=') && s2 == Some(b'=') {
            return emit!(Punct::StrictNe, 3);
        }
        if s0 == b'*' && s1 == Some(b'*') && s2 == Some(b'=') {
            return emit!(Punct::StarStarAssign, 3);
        }
        if s0 == b'<' && s1 == Some(b'<') && s2 == Some(b'=') {
            return emit!(Punct::ShlAssign, 3);
        }
        if s0 == b'>' && s1 == Some(b'>') && s2 == Some(b'=') {
            return emit!(Punct::ShrAssign, 3);
        }
        if s0 == b'>' && s1 == Some(b'>') && s2 == Some(b'>') {
            return emit!(Punct::UShr, 3);
        }
        if s0 == b'&' && s1 == Some(b'&') && s2 == Some(b'=') {
            return emit!(Punct::LogicalAndAssign, 3);
        }
        if s0 == b'|' && s1 == Some(b'|') && s2 == Some(b'=') {
            return emit!(Punct::LogicalOrAssign, 3);
        }
        if s0 == b'?' && s1 == Some(b'?') && s2 == Some(b'=') {
            return emit!(Punct::NullishAssign, 3);
        }
        // 2-character
        let two = (s0, s1);
        match two {
            (b'=', Some(b'>')) => return emit!(Punct::Arrow, 2),
            (b'?', Some(b'.')) => {
                // Only an optional-chain punctuator if not followed by a digit
                // (per spec — to disambiguate from numeric continuation).
                if s2.map_or(true, |b| !b.is_ascii_digit()) {
                    return emit!(Punct::OptionalChain, 2);
                }
            }
            (b'=', Some(b'=')) => return emit!(Punct::Eq, 2),
            (b'!', Some(b'=')) => return emit!(Punct::Ne, 2),
            (b'<', Some(b'=')) => return emit!(Punct::Le, 2),
            (b'>', Some(b'=')) => return emit!(Punct::Ge, 2),
            (b'+', Some(b'+')) => return emit!(Punct::Inc, 2),
            (b'-', Some(b'-')) => return emit!(Punct::Dec, 2),
            (b'*', Some(b'*')) => return emit!(Punct::StarStar, 2),
            (b'<', Some(b'<')) => return emit!(Punct::Shl, 2),
            (b'>', Some(b'>')) => return emit!(Punct::Shr, 2),
            (b'&', Some(b'&')) => return emit!(Punct::LogicalAnd, 2),
            (b'|', Some(b'|')) => return emit!(Punct::LogicalOr, 2),
            (b'?', Some(b'?')) => return emit!(Punct::NullishCoalesce, 2),
            (b'+', Some(b'=')) => return emit!(Punct::PlusAssign, 2),
            (b'-', Some(b'=')) => return emit!(Punct::MinusAssign, 2),
            (b'*', Some(b'=')) => return emit!(Punct::StarAssign, 2),
            (b'%', Some(b'=')) => return emit!(Punct::PercentAssign, 2),
            (b'/', Some(b'=')) => return emit!(Punct::SlashAssign, 2),
            (b'&', Some(b'=')) => return emit!(Punct::BitAndAssign, 2),
            (b'|', Some(b'=')) => return emit!(Punct::BitOrAssign, 2),
            (b'^', Some(b'=')) => return emit!(Punct::BitXorAssign, 2),
            _ => {}
        }
        // 1-character
        let p = match s0 {
            b'{' => Punct::LBrace,
            b'}' => Punct::RBrace,
            b'(' => Punct::LParen,
            b')' => Punct::RParen,
            b'[' => Punct::LBracket,
            b']' => Punct::RBracket,
            b';' => Punct::Semicolon,
            b',' => Punct::Comma,
            b':' => Punct::Colon,
            b'.' => Punct::Dot,
            b'<' => Punct::Lt,
            b'>' => Punct::Gt,
            b'+' => Punct::Plus,
            b'-' => Punct::Minus,
            b'*' => Punct::Star,
            b'%' => Punct::Percent,
            b'/' => Punct::Slash,
            b'&' => Punct::BitAnd,
            b'|' => Punct::BitOr,
            b'^' => Punct::BitXor,
            b'~' => Punct::BitNot,
            b'!' => Punct::LogicalNot,
            b'?' => Punct::Question,
            b'=' => Punct::Assign,
            _ => return Err(self.err(LexErrorKind::UnexpectedChar, start, "unexpected character")),
        };
        emit!(p, 1)
    }

    // ────────────── Utilities ──────────────

    fn err(&self, kind: LexErrorKind, start: usize, message: &'static str) -> LexError {
        LexError {
            kind,
            span: Span::new(start, self.pos.max(start + 1)),
            message,
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }
    fn peek_byte_at(&self, off: usize) -> Option<u8> {
        self.src.get(self.pos + off).copied()
    }
    fn peek_str(&self, len: usize) -> Option<&str> {
        let end = self.pos.checked_add(len)?;
        std::str::from_utf8(self.src.get(self.pos..end)?).ok()
    }
    fn peek_codepoint(&self) -> Option<u32> {
        let b0 = *self.src.get(self.pos)?;
        if b0 < 0x80 {
            return Some(b0 as u32);
        }
        let s = std::str::from_utf8(&self.src[self.pos..])
            .or_else(|e| std::str::from_utf8(&self.src[self.pos..self.pos + e.valid_up_to()]))
            .ok()?;
        s.chars().next().map(|c| c as u32)
    }
    fn advance_one_char(&mut self) {
        if let Some(b) = self.peek_byte() {
            self.pos += utf8_len(b);
        }
    }
}

// ────────────── Module-level helpers ──────────────

fn utf8_len(b0: u8) -> usize {
    if b0 < 0x80 {
        1
    } else if b0 < 0xC0 {
        1
    }
    // continuation byte — invalid leading, advance one
    else if b0 < 0xE0 {
        2
    } else if b0 < 0xF0 {
        3
    } else {
        4
    }
}

fn push_char(out: &mut String, cp: u32) {
    if let Some(c) = char::from_u32(cp) {
        out.push(c);
    }
    // Lone surrogate halves are stored as replacement char per WTF-16
    // policy; v1 of the parser tolerates them silently in identifier
    // names (caller may reject later).
    else {
        out.push('\u{FFFD}');
    }
}

fn hex_digit_value(b: u8) -> Option<u32> {
    match b {
        b'0'..=b'9' => Some((b - b'0') as u32),
        b'a'..=b'f' => Some((b - b'a' + 10) as u32),
        b'A'..=b'F' => Some((b - b'A' + 10) as u32),
        _ => None,
    }
}

fn is_whitespace_byte(b: u8) -> bool {
    matches!(b, 0x09 | 0x0B | 0x0C | 0x20)
}

fn is_unicode_whitespace(cp: u32) -> bool {
    matches!(
        cp,
        0x00A0 | 0x1680 | 0x2000..=0x200A | 0x202F | 0x205F | 0x3000 | 0xFEFF
    )
}

fn is_line_terminator_byte(b: u8) -> bool {
    matches!(b, 0x0A | 0x0D)
}

// LTC-EXT 1: ECMA-262 §11.3 LineTerminator includes U+2028 LS and U+2029 PS
// in addition to LF/CR. Their UTF-8 encoding is 0xE2 0x80 0xA8 / 0xA9. At any
// lex site that checks for line termination (single-line comment terminator,
// regex/string body LT-rejection), call `peek_is_line_terminator_bytes` to
// get the byte-length to consume; `None` means no LT here.

fn is_identifier_start_byte(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b == b'$' || b >= 0x80
}

fn is_identifier_part_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'$' || b >= 0x80
}

// Identifier predicates — v1 uses an ASCII-fast-path plus a permissive
// fallback for non-ASCII (treat as ID_Start/ID_Continue). A successor
// round wires in a precomputed Unicode table for stricter conformance.
fn is_id_start(cp: u32) -> bool {
    if cp < 0x80 {
        let b = cp as u8;
        b.is_ascii_alphabetic() || b == b'_' || b == b'$'
    } else {
        // Permissive: accept any non-ASCII code point as ID_Start. v2
        // will gate against the Unicode ID_Start property table.
        // LTC-EXT 1: exclude LineTerminator code points (U+2028 LS,
        // U+2029 PS) and Unicode whitespace — they are §11.3/§11.2
        // separators, not identifier characters. PNL-EXT 1: ZWNJ/ZWJ
        // are IdentifierPart but not IdentifierStart.
        cp >= 0xA0
            && cp != 0x200C
            && cp != 0x200D
            && cp != 0xFEFF
            && cp != 0x2028
            && cp != 0x2029
            && !is_unicode_whitespace(cp)
    }
}

fn is_id_continue(cp: u32) -> bool {
    if cp < 0x80 {
        let b = cp as u8;
        b.is_ascii_alphanumeric() || b == b'_' || b == b'$'
    } else {
        // Permissive: accept any non-ASCII code point as ID_Continue,
        // including ZWNJ (U+200C) and ZWJ (U+200D).
        // LTC-EXT 1: same separator exclusion as is_id_start.
        cp >= 0xA0 && cp != 0xFEFF && cp != 0x2028 && cp != 0x2029 && !is_unicode_whitespace(cp)
    }
}
