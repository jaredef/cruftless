# GPI Design — GPI-EXT 1 (2026-05-24)

## 1. Target site

`Op::GetProp` handler at `pilots/rusty-js-runtime/derived/src/interp.rs:7433+` (~135 LOC) is the per-resolve cost surface. Per IHI-EXT 10/11 cost-analysis: ~200-500ns/resolve on String-receiver method-resolves in hot loops, dominated by:
1. `self.string_prototype` deref (cold-path; one mem access)
2. `object_get(proto, &key)` descriptor walk (lookup in IndexMap-backed own properties; small for `%String.prototype%` but per-call)
3. Allocation/Rc bump for the returned Value

The downstream consumer for the produced method Value is **always** `Op::CallMethod` followed (after IHI-EXT 11) by `Op::CallMethodIcCached(idx)` at hot sites. After IHI rewrites the CallMethod site, the GetProp's Value is **consumed but content-unused** on the IC fast-path hit branch (interp.rs:8431-8433: method is popped, then if entry.fast returns Some, the method itself is never inspected).

## 2. Bytecode shape

New opcode: `Op::GetPropSkipForMethod = 0xFD`.
- Operand size: **2 bytes** (u16 const idx) — same as `Op::GetProp`.
- Same width permits in-place op-byte rewrite without operand shift.

Encoded by: rewriting `Op::GetProp`'s opcode byte at a site_pc where:
- The receiver was a String at last dispatch (i.e., the site's actual receiver path matches IHI's String dispatch precondition), AND
- The next opcode at `frame.pc + 0` (post-operand-consume) is `Op::CallMethodIcCached`, AND
- The cached IHI entry's `receiver_kind == IhiReceiverKind::String`.

## 3. Dispatch shape

```rust
Op::GetPropSkipForMethod => {
    // GPI-EXT 2: sentinel-push. The IC at the following
    // Op::CallMethodIcCached doesn't inspect the popped method
    // on its fast-path hit (interp.rs:8431-8433).
    let _idx = decode_u16(&frame.bytecode, frame.pc);
    frame.pc += 2;
    // Receiver stays on stack; we did NOT pop it. Push sentinel
    // as the "method" value. The next op pops it without use.
    frame.push(Value::Undefined);
}
```

Wait — that's wrong. `Op::GetProp` POPS the receiver, pushes the method. The pattern at a method-call is `Dup; GetProp("key"); ...args; CallMethod`. The Dup duplicates the receiver, so after GetProp the stack has `[..., receiver, method]`. CallMethodIcCached pops args, then method, then receiver — receiver is at the bottom of those, requiring GetProp to consume one stack slot and produce one (the method).

So GetPropSkipForMethod must:
- Pop the receiver (discard it; the Dup-preceding has another copy below)
- Wait — there is only ONE receiver below; the Dup put two on stack: `[..., recv, recv]`. GetProp pops one + pushes method: `[..., recv, method]`. CallMethodIcCached pops args, method, receiver: `[..., result]`.

So GetPropSkipForMethod must STILL pop the receiver and push a sentinel:

```rust
Op::GetPropSkipForMethod => {
    let _idx = decode_u16(&frame.bytecode, frame.pc);
    frame.pc += 2;
    let _receiver_unused = frame.pop()?;
    frame.push(Value::Undefined);
}
```

But that pops the wrong copy! After Dup the stack is `[..., recv, recv]`. GetProp pops the TOP recv (consumed for the property lookup), pushes the method. So the bottom recv remains, which is the one CallMethodIcCached will pop as `receiver`.

GetPropSkipForMethod doing `pop + push Undefined` preserves that exact shape: pops the top recv (discards) + pushes Undefined. Stack: `[..., recv, Undefined]`. Perfect.

## 4. Rewrite trigger

Decision: rewrite at **Op::CallMethod's IC-fast-path-hit branch** (interp.rs:8367+), as a follow-on to the existing IHI rewrite. When IHI rewrites the CallMethod byte to CallMethodIcCached(idx), and the entry's receiver_kind is String, ALSO walk back to the preceding GetProp site and rewrite its op byte.

Locating the preceding GetProp: requires tracking the GetProp's site_pc at the Frame. Add `pending_method_getprop_pc: Option<usize>` to Frame (mirrors `pending_method_name`); populate at GetProp dispatch; consume at CallMethod IC-hit rewrite.

## 5. Bail-safety

`Op::CallMethodIcCached`'s bail path (interp.rs:8442) calls `self.call_function(method, receiver, args)`. If method is the GPI sentinel `Value::Undefined`, this fails with "callee is not callable".

**Mitigation**: detect sentinel + re-resolve via the IHI entry's key. Modified bail:

```rust
} else {
    // GPI-aware bail: if method is Undefined sentinel (i.e., the
    // preceding GetProp was rewritten to GetPropSkipForMethod),
    // re-resolve the method via the IHI entry's key. Bail is the
    // slow path; the extra lookup is acceptable.
    let resolved_method = if matches!(method, Value::Undefined) {
        if let Some(proto) = self.string_prototype {
            self.object_get(proto, entry.key)
        } else { method }
    } else { method };
    let result = self.call_function(resolved_method, receiver, args)?;
    frame.push(result);
}
```

This preserves correctness for the rare bail path while keeping the hot path zero-cost.

## 6. LOC budget

- Op enum entry (op.rs): 3 LOC
- operand_size case: 1 LOC
- op_from_byte case: 1 LOC
- Frame field: 1 LOC + 3 init sites
- Op::GetProp site_pc capture: 2 LOC
- Op::GetPropSkipForMethod handler: ~6 LOC
- Op::CallMethod IC-hit rewrite (GetProp-side): ~10 LOC
- Op::CallMethodIcCached bail mitigation: ~6 LOC

**Total**: ~33 LOC. Within Pred-gpi.1's ≤50 budget.

## 7. Composition with cruft's existing default-on paths

C1 holds: a GetProp site is only rewritten if it has hit the CallMethodIcCached path at least once (which itself requires the IHI fast-path-hit). Non-method-call GetProp sites are never rewritten. Non-String receivers never trigger the IHI String fast path so never trigger the GPI rewrite.

C5 (composition with IHI): the design IS the composition. Two rewrites at the same source-line method-call; both consult the same IHI_TABLE idx; both eliminate distinct per-call cost components.

## 8. Methodology for GPI-EXT 2

1. Add `Op::GetPropSkipForMethod = 0xFD` to op.rs (+ operand_size + op_from_byte)
2. Add `pending_method_getprop_pc` to Frame (4 init sites)
3. Capture site_pc in Op::GetProp dispatch
4. Implement Op::GetPropSkipForMethod handler
5. Extend Op::CallMethod's IC-hit rewrite to also rewrite the GetProp site
6. Implement bail-mitigation in Op::CallMethodIcCached
7. Build + canonical fuzz (Pred-gpi.2) + diff-prod (Pred-gpi.3)
8. Bench (Pred-gpi.5): A/B header_loop + CRB string_url_sweep
9. Disposition + trajectory entry

## 9. Open risks

- **R1**: GetProp's existing handler captures `frame.last_property_lookup` and `frame.pending_method_name`; the SkipForMethod handler skips both. This may degrade diagnostic enrichment (Tier-Ω.5.uuu, Tier-Ω.5.yyyyy) for errors thrown post-rewrite. Mitigation: pre-rewrite, the IHI fast-path-hit means dispatch succeeded; bail path's call_function might emit errors that lack the lookup hint. Acceptable for first cut; document as Finding GPI.1 candidate.
- **R2**: Site-pc capture stale across functions: pending_method_getprop_pc must clear on function boundary. Mirror pending_method_name's clear sites (interp.rs:8200, 6724, 9214, 9873).
- **R3**: If the user's bytecode generator emits Dup;GetProp;...;CallMethod with intervening writes to the stack, the post-GetProp position of method might differ. cruft's compiler is the only producer; this pattern is invariant under emission. Verify via op.rs/compile_*.rs review at GPI-EXT 2 time.

## 10. Status

GPI-EXT 1 design committed. GPI-EXT 2 implementation next.
