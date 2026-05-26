//! TS lexer — thin wrapper over `rusty_js_parser::Lexer`.
//!
//! TSR-EXT 2 finding: TypeScript adds NO new token kinds vs ECMAScript at
//! the lexer tier. All TS contextual keywords (`type`, `interface`,
//! `keyof`, `as`, `is`, `readonly`, `unique`, `infer`, `out`, `in`,
//! `satisfies`, etc.) are valid identifier names at value position and
//! are reserved only at type position — disambiguation belongs to the
//! parser, not the lexer. The TS-only punctuation TSR cares about (`!`
//! non-null postfix, `?` optional postfix, `<>` generic angle brackets,
//! `=>` arrow in function types) all exist in ECMAScript already.
//!
//! Per Doc 731's alphabet-purity claim: keeping the lexer's alphabet
//! identical between JS and TS means TSR-EXT 5's annotation sidecar
//! channel does not need a parallel token stream — annotations live in
//! the parser's tree, not the token sequence.

pub use rusty_js_parser::{LexError, LexErrorKind, Lexer, LexerGoal};
pub use rusty_js_parser::{NumberKind, Punct, Span, TemplatePart, Token, TokenKind};

/// TS-specific contextual keywords. None of these are reserved at value
/// position; the parser consults this set when deciding whether a given
/// Ident token introduces a type-position construct.
pub const TS_CONTEXTUAL_KEYWORDS: &[&str] = &[
    "type",
    "interface",
    "keyof",
    "as",
    "is",
    "readonly",
    "unique",
    "infer",
    "satisfies",
    "namespace",
    "module",
    "declare",
    "abstract",
    "override",
    "public",
    "private",
    "protected",
    "implements",
    "out",
    "asserts",
    "global",
];

/// Returns true if `name` is a TS contextual keyword. The parser uses
/// this at statement-start to detect e.g. `interface Foo {...}` vs
/// `interface = 1` (where `interface` is an ordinary identifier).
#[inline]
pub fn is_ts_contextual_keyword(name: &str) -> bool {
    TS_CONTEXTUAL_KEYWORDS.iter().any(|k| *k == name)
}
