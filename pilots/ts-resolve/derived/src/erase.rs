//! Erasure pass — drops TS-only AST nodes; returns a `rusty_js_ast::Module`
//! consumable verbatim by `rusty-js-ir`.
//!
//! TSR-EXT 2 scaffolding: at this round the TS parser produces no
//! TS-only nodes (every node is already a plain `rusty_js_ast` node from
//! the JS parser pass-through). Erasure is identity. TSR-EXT 3+ adds
//! real erasure for annotations, interfaces, type aliases, etc.
//! TSR-EXT 4 adds the enum-lowering pass (the one TS construct with
//! runtime presence beyond JS).

use rusty_js_ast::Module;

/// Identity-pass at TSR-EXT 2; replaced by real erasure logic at
/// TSR-EXT 3+.
pub fn erase_module(m: Module) -> Module {
    m
}
