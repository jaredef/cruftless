//! rusty-js-runtime — bytecode interpreter + Value representation +
//! abstract operations + minimal host-hook API. Per
//! specs/rusty-js-runtime-design.md.
//!
//! v1 round 3.d.b scope: Value enum (Undefined/Null/Boolean/Number/String/
//! Object) + Object representation + Frame + dispatch loop + first 20
//! opcodes (stack ops, arithmetic, comparison, local-slot variables).
//! Control flow + function frames + intrinsics in subsequent sub-rounds.

pub mod abstract_ops;
pub mod bigint;
pub mod caps;
pub mod cjs_lexer;
pub mod interp;
pub mod interp_ic_table;
pub mod intrinsics;
pub mod iterator;
pub mod job_queue;
pub mod module;
pub mod napi;
pub mod promise;
pub mod prototype;
pub mod regex_hand;
pub mod regexp;
pub mod value;

pub use job_queue::{Job, JobKind, JobQueue};
pub use module::{detect_module_kind, HostHook, ModuleKind, ModuleStatus};

pub use interp::{Runtime, RuntimeError};
pub use value::{InternalKind, Object, ObjectRef, PropertyDescriptor, Value};

/// Convenience: parse + compile + run a module source string, with v1
/// intrinsics pre-installed.
pub fn run_module(src: &str) -> Result<Value, RuntimeError> {
    let module = rusty_js_bytecode::compile_module(src)
        .map_err(|e| RuntimeError::CompileError(format!("{}", e.message)))?;
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    rt.run_module(&module)
}
pub mod generated;
