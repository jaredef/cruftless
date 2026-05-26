//! rusty-js-parser — ECMAScript module-goal parser.
//!
//! Spec corpus: specs/ecma262-lexical.spec.md + specs/ecma262-module.spec.md.
//! Module-goal only in v1. Statement and expression bodies are captured as
//! opaque byte-spans until the expression-grammar sub-round.

pub mod expr;
pub mod lexer;
pub mod parser;
pub mod stmt;
pub mod token;

pub use lexer::{LexError, LexErrorKind, Lexer, LexerGoal};
pub use parser::{ParseError, Parser};
pub use token::{NumberKind, Punct, Span, TemplatePart, Token, TokenKind};

/// Convenience: parse a complete module from a source string. Returns the
/// AST or the first parse error.
pub fn parse_module(src: &str) -> Result<rusty_js_ast::Module, ParseError> {
    let mut p = Parser::new(src)?;
    p.parse_module()
}
