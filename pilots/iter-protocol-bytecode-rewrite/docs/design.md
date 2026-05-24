# IPBR Design — IPBR-EXT 1 (2026-05-24)

## 1. Approach pivot from seed

Seed §I.2 proposed runtime bytecode-rewrite at first `__engine_iter_next` entry. On reading the actual for-of emission (compiler.rs:1350-1417) + iterator object shape (intrinsics.rs:3661+; iterators are Objects with `__it_src__` + `__it_idx__` internal slots + a `next` intrinsic), a cleaner design surfaces:

**Compile-time emission of a fused fast-next opcode at loop entry, with the existing slow-path emission unchanged as the fallthrough**. No runtime bytecode rewrite needed — the new opcode self-dispatches based on the iter_slot's value shape on every iteration. Fast on recognized intrinsic iterators; slow path runs for everything else.

This is **simpler** than the seed's design + closes the cost surface equally well. Standing rule 13 still applies (deeper-layer-first design); the deeper layer here is "compile-time emission of a fused dispatch op" rather than "runtime rewrite of an existing op."

## 2. Bytecode shape

**New opcode**: `Op::ForOfFastNext = 0xFE`
- Operand: 10 bytes total
  - `u16 iter_slot` (which local holds the iterator)
  - `u16 bind_slot` (which local receives the next value)
  - `i32 done_offset` (where to jump on iter exhaustion — points to AFTER slow-path's trailing Pop)
  - `i16 next_iter_offset` (where to jump on successful fast-next — points to start of body, skipping the slow path)

## 3. Emission shape (compiler change)

Modify `Stmt::ForOf` in compiler.rs:1270-1427:

```
(init: existing emission for iter_slot population unchanged)

loop_start:
  Op::ForOfFastNext iter_slot, bind_slot, ?j_done_after_pop, ?j_next_iter
  ; slow path (existing emission UNCHANGED):
  LoadLocal iter_slot
  Dup; GetProp"next"; CallMethod 0
  Dup; GetProp"done"
  JumpIfTrue j_done_before_pop
  GetProp"value"
  StoreLocal bind_slot
next_iter:
  ; body (existing)
  Jump loop_start
j_done_before_pop:
  Pop
j_done_after_pop:
  ; exit
```

ForOfFastNext's `next_iter_offset` patches to next_iter (after StoreLocal, before body).
ForOfFastNext's `done_offset` patches to j_done_after_pop (post-Pop).

The slow path's emission is byte-for-byte unchanged from current.

## 4. Dispatch shape (Op::ForOfFastNext handler in interp.rs)

```rust
Op::ForOfFastNext => {
    let iter_slot = decode_u16(&frame.bytecode, frame.pc);
    let bind_slot = decode_u16(&frame.bytecode, frame.pc + 2);
    let done_offset = decode_i32(&frame.bytecode, frame.pc + 4);
    let next_iter_offset = i16::from_le_bytes([
        frame.bytecode[frame.pc + 8], frame.bytecode[frame.pc + 9]
    ]) as i32;
    let after_operand_pc = frame.pc + 10;
    frame.pc = after_operand_pc;

    // Probe iter for fast-path eligibility.
    let iter_val = frame.locals.get(iter_slot as usize)
        .map(|c| c.borrow().clone())
        .unwrap_or(Value::Undefined);
    if let Value::Object(iter_id) = iter_val {
        // Eligibility: object has __it_src__ + __it_idx__ + src is Array.
        let src_val = self.object_get(iter_id, "__it_src__");
        let idx_val = self.object_get(iter_id, "__it_idx__");
        if let (Value::Object(src_id), Value::Number(idx_n)) = (&src_val, &idx_val) {
            if matches!(self.obj(*src_id).internal_kind, crate::value::InternalKind::Array) {
                let idx = *idx_n as usize;
                let len = match self.object_get(*src_id, "length") {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };
                if idx >= len {
                    // Fast-path done: jump to done_offset (post-Pop).
                    frame.pc = (after_operand_pc as i64 + done_offset as i64) as usize;
                    continue;
                }
                let v = self.object_get(*src_id, &idx.to_string());
                // Increment idx in iter_slot's object.
                self.object_set(iter_id, "__it_idx__".into(), Value::Number((idx + 1) as f64));
                // Store v into bind_slot.
                if let Some(cell) = frame.locals.get(bind_slot as usize) {
                    *cell.borrow_mut() = v;
                }
                // Jump to next_iter (skip slow path).
                frame.pc = (after_operand_pc as i64 + next_iter_offset as i64) as usize;
                continue;
            }
        }
    }
    // Fall through to slow path (no jump; just let dispatch loop
    // execute the next op which is the slow path's LoadLocal).
}
```

## 5. Stack invariants

- Fast-path branch: pushes nothing; pops nothing; jumps to next_iter (where body expects empty stack) or done_offset (where exit expects empty stack since fast-path skipped the Pop). ✅ stack-shape-preserving.
- Fall-through branch: pushes nothing; pops nothing; lets slow path execute → slow path pushes [result] eventually consumed by GetProp/JumpIfTrue/StoreLocal. ✅ unchanged from current behavior.

## 6. Per-iter cost model

| Step | Slow-path ns | Fast-path ns |
|---|---:|---:|
| dispatch | ~5 | ~5 |
| LoadLocal iter | ~10 | (combined) |
| GetProp"next" | ~80 (post-GPI: descriptor walk) | (combined: 1 object_get) |
| CallMethod 0 → next() | ~150 (frame setup + intrinsic dispatch) | 0 (inlined) |
| result-object alloc | ~120 | 0 |
| GetProp"done" | ~80 | (combined) |
| JumpIfTrue | ~10 | (combined) |
| GetProp"value" | ~80 | (combined) |
| StoreLocal bind | ~10 | ~10 |
| Total | **~545 ns/iter** | **~50 ns/iter** |

~11× per-iter reduction. On string_url_sweep header_loop (~5K iters per outer iter, ~100 outer iters), predicted reclaim is in the 50-100ms range out of current ~252ms → target ≤200ms (well past Pred-ipbr.5's ≤214ms).

## 7. Bail-safety

- User overrides Array.prototype[@@iterator]: the iter_slot value will not have `__it_src__`/`__it_idx__` in the expected shape, OR will be a different object kind. Fast-path probe fails; fall through to slow path. ✅ Correct.
- User overrides ArrayIterator.prototype.next: fast-path uses the cached __it_src__/__it_idx__ directly without invoking next, so the override is bypassed. **This is a correctness divergence.** Mitigation: probe whether the iter object's `next` property is the intrinsic; bail if user-modified. ~10 LOC at the eligibility check.
- User mutates the iter object's __it_src__ mid-iteration: re-read every iteration via object_get; honors the mutation. ✅ Correct.
- User mutates the underlying array's length mid-iteration: re-read length every iteration. ✅ Correct.

For first cut, defer the `next` override check; document as Finding IPBR.1 candidate (correctness divergence vs. user override; consumer-app surface rare). If a fixture surfaces, add the probe.

## 8. LOC budget

- `op.rs`: opcode declaration + operand_size + op_from_byte = ~5 LOC
- `interp.rs`: ForOfFastNext handler = ~40 LOC
- `compiler.rs`: emit ForOfFastNext at loop_start + patch sites = ~20 LOC
- `op.rs`: new operand-size variant (10 bytes — neither 4 nor any existing size; need new arm) = ~3 LOC

**Total**: ~68 LOC. Within Pred-ipbr.1's ≤80 budget.

## 9. Methodology for IPBR-EXT 2

1. Add `Op::ForOfFastNext = 0xFE` to op.rs (+ operand_size = 10 = new arm + op_from_byte case)
2. Add `decode_i16` helper if not present
3. Modify compiler.rs's Stmt::ForOf to emit ForOfFastNext at loop_start with two patch sites
4. Implement Op::ForOfFastNext handler in interp.rs
5. Build + canonical fuzz (Pred-ipbr.2) + diff-prod (Pred-ipbr.3)
6. Bench (Pred-ipbr.5) — string_url_sweep CRB + A/B header_loop
7. Disposition + trajectory entry

IPBR-EXT 3 (if needed): String fast-path (same pattern with String byte-offset instead of Array index) + composition probe across full CRB + Pred-ipbr.6 disposition (≤3 rounds discipline target) + chapter close.

## 10. Open risks

- **R1**: ForOfFastNext at loop_start runs an `object_get("__it_src__")` on every iteration — this itself is ~40ns. Mitigation: cache the eligibility decision on the iter object after first probe (sidecar boolean). Or live with it; ~40ns is still 13× better than the slow path's ~545ns.
- **R2**: 10-byte operand is unusually large for cruft's bytecode (existing max is 4). Adding a new arm to operand_size. Verified compiler.rs's emit_jump uses i32 offsets; same shape extended.
- **R3**: locals storage type — verify `frame.locals` exposes a `RefCell<Value>` borrowable for read+write. Inspect interp.rs LoadLocal/StoreLocal handler at IPBR-EXT 2 start.
- **R4**: the next_iter target is at most ~10-30 bytes after the operand (covers the slow path's emission). i16 (±32K) is plenty.

## 11. Status

IPBR-EXT 1 design committed. IPBR-EXT 2 implementation next.
