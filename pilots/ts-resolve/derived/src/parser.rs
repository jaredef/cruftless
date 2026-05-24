//! TS parser — TSR-EXT 2 scaffolding.
//!
//! At TSR-EXT 2 (lexer round) the parser is a thin pass-through that
//! delegates to `rusty_js_parser::parse_module`. This handles every
//! `.ts` source that uses ZERO TypeScript-specific syntax — i.e., a
//! `.ts` file that is also valid JavaScript runs end-to-end via cruft
//! today.
//!
//! TSR-EXT 3 replaces this pass-through with a real parser that handles
//! the Tier-A + Tier-B TS surface per `docs/design.md` §2.

use rusty_js_ast::Module;
use crate::ts_ast::TypeWitness;

#[derive(Debug)]
pub struct TsParseError {
    pub message: String,
}

impl From<rusty_js_parser::ParseError> for TsParseError {
    fn from(e: rusty_js_parser::ParseError) -> Self {
        TsParseError { message: format!("{:?}", e) }
    }
}

impl std::fmt::Display for TsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TsParseError {}

pub struct TsParser<'src> {
    src: &'src str,
}

impl<'src> TsParser<'src> {
    pub fn new(src: &'src str) -> Result<Self, TsParseError> {
        Ok(TsParser { src })
    }

    /// Parse the module + collect any TS annotations as witnesses. At
    /// TSR-EXT 2 the witness vector is always empty (no annotations
    /// recognized yet). TSR-EXT 3+ populates it.
    pub fn parse_module(&mut self) -> Result<(Module, Vec<TypeWitness>), TsParseError> {
        let module = rusty_js_parser::parse_module(self.src)?;
        Ok((module, Vec::new()))
    }
}
