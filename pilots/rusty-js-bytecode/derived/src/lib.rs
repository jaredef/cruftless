//! rusty-js-bytecode — stack-based bytecode + single-pass compiler from
//! rusty_js_ast AST. Per specs/rusty-js-bytecode-design.md.
//!
//! v1 scope (round 3.c.b): literals + arithmetic + comparison + simple
//! variable access via global slot. Control flow + scope resolution +
//! function bodies + try/catch follow in 3.c.c and 3.c.d.

pub mod compiler;
pub mod constants;
pub mod disasm;
pub mod op;

pub use compiler::{
    CompileError, CompiledModule, Compiler, ExportBinding, ImportBinding, ImportBindingKind,
    LocalDescriptor, UpvalueDescriptor, UpvalueSource,
};
pub use constants::{Constant, ConstantsPool};
pub use disasm::disassemble;
pub use op::{encode_op, Op};

/// Convenience: parse + compile a module source string.
pub fn compile_module(src: &str) -> Result<CompiledModule, CompileError> {
    compile_module_with_url(src, "")
}

/// Ω.5.P51.E1: same as compile_module but threads a source URL so runtime
/// errors can emit `@url:line:col` for closure frames whose defining module
/// has long since returned. evaluate_module / evaluate_cjs_module call this
/// directly to wire the resource locator.
pub fn compile_module_with_url(src: &str, url: &str) -> Result<CompiledModule, CompileError> {
    let ast = rusty_js_parser::parse_module(src).map_err(|e| CompileError {
        span: e.span,
        message: format!("parse: {} @byte{}", e.message, e.span.start),
    })?;
    let mut c = Compiler::new();
    c.set_source_line_starts(compute_line_starts(src));
    c.set_source_url(url.to_string());
    c.compile_module(&ast)
}

/// ES-EXT 1 (eval-scope-binding-chain): Script-mode compilation entry point.
/// Per ECMA-262 §19.2.1.3 PerformEval indirect-eval branch, the eval source
/// runs as a Script (not a Module). Scripts attach top-level var declarations
/// to the realm's variable environment (which IS the global object for
/// Scripts), whereas Modules keep them as module-local bindings.
///
/// ES-EXT 1 (foundation rung): this entry point currently delegates to
/// compile_module_with_url so the substrate-program structure is in place
/// without semantic change yet. ES-EXT 2 will flip top-level var emissions
/// from StoreLocal to StoreGlobal at compile time.
pub fn compile_script_with_url(src: &str, url: &str) -> Result<CompiledModule, CompileError> {
    // ES-EXT 2: set script_mode on the Compiler so top-level `var`
    // declarations route to StoreGlobal at compile time, attaching to the
    // realm's global object per §19.2.1.3 PerformEval (indirect-eval) and
    // §16.1 Scripts.
    let ast = rusty_js_parser::parse_module(src).map_err(|e| CompileError {
        span: e.span,
        message: format!("parse: {} @byte{}", e.message, e.span.start),
    })?;
    let mut c = Compiler::new();
    c.set_source_line_starts(compute_line_starts(src));
    c.set_source_url(url.to_string());
    c.set_script_mode(true);
    c.compile_module(&ast)
}

/// Byte offsets of the start of each line in `src`. Index 0 is offset 0.
/// Line i starts at line_starts[i] (inclusive); line i ends at
/// line_starts[i+1] (exclusive, accounting for the newline byte itself).
pub fn compute_line_starts(src: &str) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::with_capacity(src.len() / 32 + 1);
    v.push(0);
    for (i, b) in src.bytes().enumerate() {
        if b == b'\n' {
            v.push((i + 1) as u32);
        }
    }
    v
}

/// Convert a byte offset to (line, column), both 1-indexed for editor
/// conventions. Returns (1, 1) on empty input.
pub fn byte_offset_to_line_col(line_starts: &[u32], offset: u32) -> (u32, u32) {
    if line_starts.is_empty() {
        return (1, 1);
    }
    let idx = line_starts.partition_point(|&start| start <= offset);
    let line = idx as u32; // 1-indexed because partition_point returns count <= offset
    let col = offset + 1 - line_starts[idx - 1];
    (line, col)
}
