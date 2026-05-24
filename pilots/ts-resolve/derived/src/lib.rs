//! ts-resolve — TypeScript source-language resolver feeding rusty-js-ir.
//!
//! Pin-Art locale: see `pilots/ts-resolve/seed.md`.
//!
//! TSR wraps and extends `rusty-js-parser` rather than duplicating it.
//! The lexer is byte-for-byte identical to JS at the token tier (TS
//! contextual keywords like `type`, `interface`, `keyof`, `as`, `is`
//! emerge as Ident tokens; disambiguation is the parser's concern). The
//! parser intercepts at type-position to consume + discard (erasure) or
//! capture (sidecar at TSR-EXT 5). The output AST is `rusty_js_ast::*`
//! verbatim — no TS-specific residue reaches rusty-js-ir (C3 per
//! seed.md §I.2).

pub mod lexer;
pub mod ts_ast;
pub mod parser;
pub mod erase;
pub mod strip;

pub use parser::{TsParser, TsParseError};
pub use ts_ast::{TsTypeRef, TsLiteralVal, TsAnnotation};

/// Convenience: parse a TS module + erase types, returning a plain
/// `rusty_js_ast::Module` consumable by rusty-js-ir.
pub fn parse_and_erase(src: &str) -> Result<rusty_js_ast::Module, TsParseError> {
    let mut p = TsParser::new(src)?;
    let (module, _witnesses) = p.parse_module()?;
    Ok(erase::erase_module(module))
}

/// Sidecar-aware parse: returns the erased module + the collected
/// `TypeWitness` records for downstream IC/JIT consumers. TSR-EXT 5
/// fully wires this through; for now witnesses are an empty stash.
pub fn parse_with_witnesses(src: &str) -> Result<(rusty_js_ast::Module, Vec<ts_ast::TypeWitness>), TsParseError> {
    let mut p = TsParser::new(src)?;
    let (module, witnesses) = p.parse_module()?;
    Ok((erase::erase_module(module), witnesses))
}
