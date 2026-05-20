//! Bytecode → Cranelift IR translator. First-cut: supports a narrow
//! op set sufficient to JIT-compile pure-i64-arithmetic functions
//! where all values are treated as raw 64-bit integers and the function
//! signature is `extern "C" fn(i64, i64) -> i64` (two i64 args, one i64
//! return).
//!
//! Per Doc 731 §VII R3 (verifier-before-emission), `compile_function`
//! returns `None` for any function whose bytecode contains an op not
//! yet in the supported set. The interpreter remains the fallback.
//!
//! Per the JIT-EXT 1 op-p4-classification, Class A ops (Cranelift-
//! direct) are the first-cut targets. This translator currently supports
//! a tiny subset of Class A: LoadArg, Add (as iadd on raw i64), Return.
//! Future JIT-EXT rounds extend coverage incrementally.

use cranelift_codegen::ir::types::I64;
use cranelift_codegen::ir::{AbiParam, InstBuilder, Value as ClValue};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use rusty_js_bytecode::compiler::FunctionProto;
use rusty_js_bytecode::op::{op_from_byte, Op};

/// A JIT-compiled function with the i64-arithmetic signature. Used for
/// the first-cut subset of FunctionProtos that pass the narrow verifier.
pub type JitI64Add = extern "C" fn(i64, i64) -> i64;

/// Compiled artifact returned by `compile_function`. Holds the leaked
/// JITModule to keep the function pointer valid for the program's
/// lifetime. v1: the module is intentionally leaked because cruftless
/// has no JIT-eviction mechanism yet. Future GC tier work will
/// reclaim JIT'd code via Cranelift's free_memory hook.
pub struct JitFn {
    pub func_ptr: JitI64Add,
    _module: &'static mut JITModule,
}

impl std::fmt::Debug for JitFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JitFn(ptr=0x{:x})", self.func_ptr as usize)
    }
}

/// Try to JIT-compile `proto` to a callable `extern "C" fn(i64, i64) -> i64`.
/// Returns Err with a short reason if the bytecode contains any op not
/// in the supported set. The caller (runtime) keeps the bytecode
/// interpreter as the fallback when this returns Err.
pub fn compile_function(proto: &FunctionProto) -> Result<JitFn, String> {
    if proto.params != 2 {
        return Err(format!("first-cut JIT requires exactly 2 params; got {}", proto.params));
    }

    // Walk the bytecode and collect the op sequence; reject early on
    // any unsupported op so we don't waste Cranelift effort.
    let ops = collect_ops(&proto.bytecode)?;

    // Build the Cranelift JIT module + function.
    let mut flag_builder = settings::builder();
    flag_builder.set("use_colocated_libcalls", "false")
        .map_err(|e| format!("flag use_colocated_libcalls: {e:?}"))?;
    flag_builder.set("is_pic", "false")
        .map_err(|e| format!("flag is_pic: {e:?}"))?;
    let isa_builder = cranelift_native::builder()
        .map_err(|e| format!("isa builder: {e}"))?;
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .map_err(|e| format!("isa finish: {e:?}"))?;
    let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    let mut module = JITModule::new(builder);

    let mut ctx = module.make_context();
    let mut fb_ctx = FunctionBuilderContext::new();

    ctx.func.signature.params.push(AbiParam::new(I64));
    ctx.func.signature.params.push(AbiParam::new(I64));
    ctx.func.signature.returns.push(AbiParam::new(I64));

    {
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fb_ctx);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        let arg0 = builder.block_params(entry)[0];
        let arg1 = builder.block_params(entry)[1];

        // Virtual operand stack: a Vec<ClValue> tracked at compile
        // time. Cranelift values are SSA, so we don't need to materialize
        // a runtime stack — we just thread the operand chain symbolically.
        let mut stack: Vec<ClValue> = Vec::new();
        let mut returned = false;

        for op_entry in &ops {
            match op_entry {
                OpEntry::LoadArg(slot) => {
                    let v = match *slot {
                        0 => arg0,
                        1 => arg1,
                        _ => return Err(format!("LoadArg slot {} out of range (only 0,1 supported in first cut)", slot)),
                    };
                    stack.push(v);
                }
                OpEntry::Add => {
                    let r = stack.pop().ok_or("Add: stack underflow")?;
                    let l = stack.pop().ok_or("Add: stack underflow")?;
                    let sum = builder.ins().iadd(l, r);
                    stack.push(sum);
                }
                OpEntry::Sub => {
                    let r = stack.pop().ok_or("Sub: stack underflow")?;
                    let l = stack.pop().ok_or("Sub: stack underflow")?;
                    let diff = builder.ins().isub(l, r);
                    stack.push(diff);
                }
                OpEntry::Mul => {
                    let r = stack.pop().ok_or("Mul: stack underflow")?;
                    let l = stack.pop().ok_or("Mul: stack underflow")?;
                    let prod = builder.ins().imul(l, r);
                    stack.push(prod);
                }
                OpEntry::Return => {
                    let v = stack.pop().ok_or("Return: stack underflow")?;
                    builder.ins().return_(&[v]);
                    returned = true;
                    break;
                }
                OpEntry::ReturnUndef => {
                    // Treat as return 0 in this i64-only first cut.
                    let z = builder.ins().iconst(I64, 0);
                    builder.ins().return_(&[z]);
                    returned = true;
                    break;
                }
            }
        }

        if !returned {
            // Synthesize a return at end if the function falls through
            // (matches the interpreter's implicit-ReturnUndef behavior).
            let z = builder.ins().iconst(I64, 0);
            builder.ins().return_(&[z]);
        }

        builder.finalize();
    }

    let id = module
        .declare_function(&format!("jit_fn_{}", proto.display_name), Linkage::Export, &ctx.func.signature)
        .map_err(|e| format!("declare_function: {e}"))?;
    module.define_function(id, &mut ctx)
        .map_err(|e| format!("define_function: {e}"))?;
    module.clear_context(&mut ctx);
    module.finalize_definitions()
        .map_err(|e| format!("finalize_definitions: {e}"))?;

    let code_ptr = module.get_finalized_function(id);
    // Safety: signature matches; module leaked to keep ptr valid.
    let func_ptr: JitI64Add = unsafe { std::mem::transmute(code_ptr) };
    let leaked: &'static mut JITModule = Box::leak(Box::new(module));
    Ok(JitFn { func_ptr, _module: leaked })
}

#[derive(Debug, Clone, Copy)]
enum OpEntry {
    LoadArg(u16),
    Add,
    Sub,
    Mul,
    Return,
    ReturnUndef,
}

fn collect_ops(bytecode: &[u8]) -> Result<Vec<OpEntry>, String> {
    let mut pc = 0;
    let mut out = Vec::new();
    while pc < bytecode.len() {
        let opcode = op_from_byte(bytecode[pc])
            .ok_or_else(|| format!("unknown opcode byte 0x{:02x} at pc={}", bytecode[pc], pc))?;
        pc += 1;
        let entry = match opcode {
            Op::LoadArg => {
                let slot = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                pc += 2;
                OpEntry::LoadArg(slot)
            }
            Op::Add => OpEntry::Add,
            Op::Sub => OpEntry::Sub,
            Op::Mul => OpEntry::Mul,
            Op::Return => OpEntry::Return,
            Op::ReturnUndef => OpEntry::ReturnUndef,
            other => return Err(format!(
                "first-cut JIT does not yet support op {:?} at pc={}",
                other, pc - 1
            )),
        };
        out.push(entry);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_js_bytecode::compiler::{FunctionProto, LocalDescriptor, UpvalueDescriptor};
    use rusty_js_bytecode::constants::ConstantsPool;
    use rusty_js_bytecode::op::{encode_op, encode_u16};

    fn empty_proto(bytecode: Vec<u8>, params: u16) -> FunctionProto {
        FunctionProto {
            bytecode,
            constants: ConstantsPool::new(),
            params,
            display_name: "test".to_string(),
            function_length: params,
            locals: Vec::<LocalDescriptor>::new(),
            upvalues: Vec::<UpvalueDescriptor>::new(),
            rest_param_slot: None,
            arguments_slot: None,
            self_name_slot: None,
            is_generator: false,
            is_async: false,
            source_url: String::new(),
            line_starts: Vec::new(),
            source_map: Vec::new(),
            construct_tags: Vec::new(),
            strict: false,
        }
    }

    #[test]
    fn jit_add_two_args() {
        // function add(a, b) { return a + b; }
        // bytecode: LoadArg 0; LoadArg 1; Add; Return
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Add);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let jit = compile_function(&proto).expect("compile failed");
        assert_eq!((jit.func_ptr)(2, 3), 5);
        assert_eq!((jit.func_ptr)(-10, 100), 90);
        assert_eq!((jit.func_ptr)(i64::MAX - 1, 1), i64::MAX);
    }

    #[test]
    fn jit_combined_arith() {
        // function f(a, b) { return (a + b) * (a - b); }
        // bytecode: LoadArg 0; LoadArg 1; Add; LoadArg 0; LoadArg 1; Sub; Mul; Return
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Add);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Sub);
        encode_op(&mut bc, Op::Mul);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let jit = compile_function(&proto).expect("compile failed");
        // (5+3) * (5-3) = 8 * 2 = 16
        assert_eq!((jit.func_ptr)(5, 3), 16);
        // (10+4) * (10-4) = 14 * 6 = 84
        assert_eq!((jit.func_ptr)(10, 4), 84);
    }

    #[test]
    fn jit_rejects_unsupported_op() {
        // bytecode: PushNull; Return — PushNull not in first-cut op set
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::PushNull);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let result = compile_function(&proto);
        assert!(result.is_err(), "should reject unsupported op");
        let err = result.unwrap_err();
        assert!(err.contains("does not yet support"), "err: {}", err);
    }
}
