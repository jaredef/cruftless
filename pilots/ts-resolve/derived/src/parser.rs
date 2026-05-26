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

use crate::ts_ast::TypeWitness;
use rusty_js_ast::Module;

#[derive(Debug)]
pub struct TsParseError {
    pub message: String,
}

impl From<rusty_js_parser::ParseError> for TsParseError {
    fn from(e: rusty_js_parser::ParseError) -> Self {
        TsParseError {
            message: format!("{:?}", e),
        }
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

    /// Parse the module + collect any TS annotations as witnesses.
    ///
    /// TSR-EXT 3 (2026-05-24): run the type-stripper to produce an
    /// erased source string + a witness vector; then feed the stripped
    /// source to rusty-js-parser. Pin-Art-consistent — stripping rules
    /// are derived from TS spec excerpts at the source-text tier (see
    /// `strip.rs`'s rule list).
    pub fn parse_module(&mut self) -> Result<(Module, Vec<TypeWitness>), TsParseError> {
        let (stripped, witnesses) = crate::strip::strip_ts(self.src).map_err(|e| TsParseError {
            message: format!("strip: {}", e),
        })?;
        let module = rusty_js_parser::parse_module(&stripped)?;
        Ok((module, witnesses))
    }
}
