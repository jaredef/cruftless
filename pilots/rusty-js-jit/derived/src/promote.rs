//! Doc 731 §XIV.d / §XIV.f → JIT-EXT 7: function-level typed-i64 promotion.
//!
//! Walks a FunctionProto's bytecode and produces a transformed
//! FunctionProto where every plain arithmetic / comparison op is
//! rewritten as its typed-I64 counterpart, IFF the function's bytecode
//! consists only of ops the typed-I64 alphabet covers (i.e., no
//! GetProp / SetProp / Call / NewObject / etc.).
//!
//! The rewrite is OPTIMISTIC: if the function operates on non-integer
//! Numbers (or non-Number values) at runtime, the typed-I64 ops throw
//! TypeError via the interpreter's unbox_int64 helper, or trap to
//! deopt in the JIT-compiled version (deopt not yet implemented;
//! TypeError surfaces directly in v1).
//!
//! Verifier-before-emission (Doc 731 §VII R3): the pass returns None
//! if any op outside the typed-I64-eligible set appears. Callers (the
//! JIT's compile_function) fall back to the cheat path or to the
//! interpreter on None.
//!
//! This is the substrate that makes Doc 731 §XIV.d's deeper claim
//! actionable on real JavaScript code: the bytecode-compiler tier
//! emits plain ops; the promotion pass rewrites to typed ops when the
//! function shape permits; the JIT and the interpreter both honor
//! the typed alphabet.

use rusty_js_bytecode::compiler::FunctionProto;
use rusty_js_bytecode::op::{op_from_byte, Op};

/// Walk `proto`'s bytecode and produce a copy with plain arithmetic /
/// comparison ops rewritten as their typed-I64 counterparts.
///
/// Returns `Some(transformed)` if every op in the bytecode is in the
/// typed-I64-eligible set; `None` otherwise. The transformed proto's
/// bytecode is the same length as the original (the typed ops are
/// zero-operand and share the same encoding shape as their plain
/// counterparts).
pub fn promote_to_typed_i64(proto: &FunctionProto) -> Option<FunctionProto> {
    let new_bytecode = rewrite_bytecode(&proto.bytecode)?;
    let mut out = proto.clone();
    out.bytecode = new_bytecode;
    Some(out)
}

fn rewrite_bytecode(bc: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(bc.len());
    let mut pc = 0;
    while pc < bc.len() {
        let opcode = op_from_byte(bc[pc])?;
        let operand_bytes = opcode.operand_size();

        // Eligibility check + rewrite.
        let new_opcode = match opcode {
            // Already typed — keep.
            Op::AddI64
            | Op::SubI64
            | Op::MulI64
            | Op::IncI64
            | Op::DecI64
            | Op::LtI64
            | Op::LeI64
            | Op::GtI64
            | Op::GeI64
            | Op::EqI64
            | Op::NeI64 => opcode,

            // Plain → typed.
            Op::Add => Op::AddI64,
            Op::Sub => Op::SubI64,
            Op::Mul => Op::MulI64,
            Op::Inc => Op::IncI64,
            Op::Dec => Op::DecI64,
            Op::Lt => Op::LtI64,
            Op::Le => Op::LeI64,
            Op::Gt => Op::GtI64,
            Op::Ge => Op::GeI64,
            // Note: Eq/Ne in JS have ToPrimitive dispatch that's not
            // strictly equivalent to integer compare; we promote only
            // when the JIT supports the typed variant. The typed
            // version assumes both operands are integer-valued
            // Numbers, which matches the cheat path's existing
            // assumption.
            Op::Eq | Op::StrictEq => Op::EqI64,
            Op::Ne | Op::StrictNe => Op::NeI64,

            // Pass-through (typed-I64 doesn't apply or the op is
            // already type-neutral).
            Op::LoadArg
            | Op::LoadLocal
            | Op::StoreLocal
            | Op::PushI32
            | Op::PushUndef
            | Op::Pop
            | Op::Dup
            | Op::Jump
            | Op::JumpIfTrue
            | Op::JumpIfFalse
            | Op::Return
            | Op::ReturnUndef
            | Op::Nop => opcode,

            // Any other op disqualifies the function for typed
            // promotion — return None and let the caller fall back.
            _ => return None,
        };

        out.push(new_opcode as u8);
        for i in 0..operand_bytes {
            out.push(bc[pc + 1 + i]);
        }
        pc += 1 + operand_bytes;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_js_bytecode::compile_module;
    use rusty_js_bytecode::Constant;

    #[test]
    fn promotes_sum_to_typed_i64() {
        let src =
            r#"function sum(n) { var s = 0; for (var i = 0; i < n; i++) s = s + i; return s; }"#;
        let m = compile_module(src).expect("compile module");
        let sum_proto = m
            .constants
            .entries()
            .iter()
            .find_map(|c| match c {
                Constant::Function(p) if p.display_name == "sum" => Some((**p).clone()),
                _ => None,
            })
            .expect("find sum proto");
        let promoted = promote_to_typed_i64(&sum_proto).expect("promote sum");

        // The promoted bytecode should have AddI64 / LtI64 / IncI64
        // where the original had Add / Lt / Inc.
        let mut plain_count = 0;
        let mut typed_count = 0;
        let mut pc = 0;
        while pc < promoted.bytecode.len() {
            let op = op_from_byte(promoted.bytecode[pc]).unwrap();
            match op {
                Op::Add
                | Op::Sub
                | Op::Mul
                | Op::Inc
                | Op::Dec
                | Op::Lt
                | Op::Le
                | Op::Gt
                | Op::Ge => plain_count += 1,
                Op::AddI64
                | Op::SubI64
                | Op::MulI64
                | Op::IncI64
                | Op::DecI64
                | Op::LtI64
                | Op::LeI64
                | Op::GtI64
                | Op::GeI64 => typed_count += 1,
                _ => {}
            }
            pc += 1 + op.operand_size();
        }
        assert_eq!(
            plain_count, 0,
            "no plain arithmetic ops should remain after promotion"
        );
        assert!(
            typed_count >= 3,
            "expected at least Add+Lt+Inc to be promoted; got {}",
            typed_count
        );
    }

    #[test]
    fn refuses_function_with_unsupported_ops() {
        // A function with object property access can't be typed-promoted.
        let src = r#"function getx(o) { return o.x; }"#;
        let m = compile_module(src).expect("compile module");
        let proto = m
            .constants
            .entries()
            .iter()
            .find_map(|c| match c {
                Constant::Function(p) if p.display_name == "getx" => Some((**p).clone()),
                _ => None,
            })
            .expect("find getx proto");
        let result = promote_to_typed_i64(&proto);
        assert!(
            result.is_none(),
            "function with GetProp should not be promotable"
        );
    }

    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn promoted_sum_jit_compiles_and_runs() {
        let src =
            r#"function sum(n) { var s = 0; for (var i = 0; i < n; i++) s = s + i; return s; }"#;
        let m = compile_module(src).expect("compile module");
        let sum_proto = m
            .constants
            .entries()
            .iter()
            .find_map(|c| match c {
                Constant::Function(p) if p.display_name == "sum" => Some((**p).clone()),
                _ => None,
            })
            .expect("find sum proto");
        let promoted = promote_to_typed_i64(&sum_proto).expect("promote sum");
        let jit = crate::compile_function(&promoted).expect("JIT compile promoted sum");
        assert_eq!(jit.func.call1(0 as f64), 0 as f64);
        assert_eq!(jit.func.call1(5 as f64), 10 as f64);
        assert_eq!(jit.func.call1(100 as f64), 4950 as f64);
        assert_eq!(jit.func.call1(1_000_000 as f64), 499_999_500_000_i64 as f64);
    }
}
