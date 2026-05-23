# TL-EXT 1 — Top-level JIT + 3-op alphabet extension design

*Enumerates per-move substrate-move plan derived from Doc 740 multi-tier reading + TL-EXT 0 seed §I-§III. Five rounds dependency-ordered (TL-EXT 2-6); per-round mechanism + LOC + reclaim + falsifier.*

## 1. Design constraints (C1-C8 from seed §I.2)

Re-stated for in-doc reference:

```
C1. Correctness preserved (canonical fuzz acc=-932188103; diff-prod 42/42).
C2. ECMA module semantics preserved (top-level execution order; scoping).
C3. No bench-fixture restructuring (json_parse_transform stays at top-level).
C4. Hot-intrinsic IC discipline at JIT tier (verify cached intrinsic id; bail to extern).
C5. Existing function-body JIT not broken (TB metadata cache integrity; standing rule 9).
C6. GC roots preserved across module-body JIT (top-level locals).
C7. PushConst lowering respects Φ-EXT 3 f64-default (constants flow as f64).
C8. Bail discipline: any unsupported op in module body → whole module falls to interp.

Architecture induced: ModuleProto wrapper; alphabet additions op-by-op-additive;
IC shapes mirror CharCode-EXT 2's interp pattern; existing compile_function reused.
```

## 2. Per-round substrate-move plan

### Move 1 — TL-EXT 2: Op::PushConst (Number-only) in JIT alphabet

**Mechanism**: extend translator.rs ParsedOp enum with `PushConst(u16)`. In the parse pass (bytecode → ParsedOp), decode the u16 index into the constants pool and read the Constant. If `Constant::Number(n)`, lower to `builder.ins().f64const(n)` push onto operand stack. If String/BigInt/Regex/Function, mark the function as non-JIT-eligible (existing bail discipline).

**Composition**: required infrastructure for any literal pattern (`i = 0` in for-loop init; literal args to method calls; `i < 5000` style bounded loops in TB-eligible function bodies). Unlocks both top-level for-loops AND existing function-body JIT for patterns that use numeric literals.

**LOC estimate**: ~30 (ParsedOp variant + parse pass clause + translate pass clause + eligibility check).

**Falsifier (Pred-tl.4 scope discipline)**: only Number constants supported; String/BigInt/Regex/Function bail. The bail must be detected at parse-time (not translate-time) so the function is correctly classified as non-eligible upstream of the JIT entry attempt.

**Three-probe gates (per Findings rule 5 + standing rule 10)**:
- canonical fuzz GREEN
- diff-prod 42/42 GREEN
- bench: bench_call_overhead + bench_ic stay within ±5% of post-Φ baselines (Pred-tl.5)

### Move 2 — TL-EXT 3: Module-body JIT entry wrapper

**Mechanism**: create a synthetic FunctionProto-equivalent wrapping CompiledModule's bytecode + constants + locals. The wrapper has 0 args, the module's locals as its locals, no upvalues. In Runtime::run_module (interp.rs ~6503), before entering the interp dispatch loop, attempt `compile_function` on the wrapper. If compile succeeds, call the JIT'd body once (it returns Undefined per ReturnUndef discipline); else fall through to the existing interp dispatch path.

**Composition**: this is the entry-mechanism upstream constraint-closure per Doc 740 §III.4. Move 3 + Move 4's cascade-revival pilots cannot fire without it.

**LOC estimate**: ~80 (wrapper struct + Runtime::run_module branch + JIT eligibility check at module-tier + bail discipline preserving the interp fallback).

**Subtlety**: module bytecode uses Op::LoadGlobal / Op::StoreGlobal (not LoadLocal) for module-level let/const after the compile_module pass per CompiledModule's locals scheme. These ops are NOT in the JIT alphabet; per C8, modules containing them bail. Most realistic modules will bail at this layer; **first-cut goal is to land the entry mechanism + measure on a synthetic module-body fixture that uses only LoadLocal**. The json_parse_transform fixture is the load-bearing measurement; whether it bails or fires is a per-fixture empirical question that the round closes.

**Falsifier**: existing function-body JIT (Σ/Τ/Ψ/Φ) tests still pass at the same per-iter latencies. The wrapper must not regress function-body entry.

### Move 3 — TL-EXT 4: Op::GetProp with String-length IC inlined in JIT

**Mechanism**: extend ParsedOp with `GetProp(u16)` (the u16 is the constant pool index for the key). At translate time, read the constant; if `Constant::String("length")`, emit a JIT body that:

1. Takes the receiver f64 from stack
2. Bitcasts to the Value-tag union; check tag == VALUE_TAG_STRING
3. If tag matches: bitcast payload bits → `*const String` (per the existing String encoding convention; need to inspect Value's repr in value.rs); inline the ASCII fast-path from CharCode-EXT 1 (`if s.is_ascii() { s.len() as f64 } else { call_extern_chars_count }`); push result
4. If tag != STRING: bail to extern (existing GetProp helper)

For keys other than "length", bail to extern at parse-time (mark non-eligible). For the first cut, ONLY the "length" key triggers the inline path.

**Composition**: cascade-revival pilot #1 per Doc 740 §III.4. Closes the .length-read overhead in the inner for-loop bound check.

**LOC estimate**: ~80 (ParsedOp variant + parse pass + translate IR emission + extern declaration + IC verification helper).

**Falsifier**: canonical fuzz must remain byte-identical (any non-ASCII string-length read must produce the same value as the interp path). i64::MIN-style edge cases don't apply to length (always non-negative).

### Move 4 — TL-EXT 5: Op::CallMethod with charCodeAt IC inlined in JIT

**Mechanism**: extend ParsedOp with `CallMethod(u8)` (the u8 is n_args). At translate time, look back: if the preceding ParsedOp is `GetProp(key_idx)` where the constant is `Constant::String("charCodeAt")` AND n_args==1, emit a JIT body that:

1. Takes [receiver, method, arg] from stack
2. Bitcasts receiver to Value-tag; check tag == VALUE_TAG_STRING; bail if not
3. Bitcasts method to ObjectId; check method_id == cached intrinsic_string_charcodeat_id; bail if not (intrinsic IC discipline)
4. Bitcasts arg to f64; check is_finite && >= 0; bail if not (slow path handles NaN/negative)
5. Inline ASCII byte-fetch: if string.is_ascii() && i < len → emit `bytes[i] as f64`; else NaN
6. Push result

For other shapes of CallMethod (different method, different arity), bail to extern at parse-time.

**Subtlety**: the receiver appears BOTH at the GetProp site (where it's consumed to lookup the method) AND at the CallMethod site (where it's needed as `this`). The bytecode uses an explicit Dup to keep the receiver under the method (per the compiler pattern at compiler.rs:1352-1356). The JIT lowering must preserve this stack discipline — Dup is already JIT-supported.

**Pattern-match scope**: only `Dup → GetProp("charCodeAt") → LoadLocal → CallMethod(1)` patterns at first cut. Other interleavings (e.g., method stored to local then called later) bail at the CallMethod ParsedOp recognition.

**LOC estimate**: ~100 (ParsedOp variant + parse-pass pattern-match for the preceding GetProp + translate IR emission + 3-guard verification chain + extern call for bail path).

**Falsifier**: per-call result byte-identical to CharCode-EXT 2's interp IC. Canonical fuzz catches divergence.

### Move 5 — TL-EXT 6: Composition probe + CRB final disposition

**Mechanism**: measurement-only round. Re-run JSF component A/B probe (Pred-jsf.* delta on checksum loop); re-run CRB json_parse_transform (Pred-tl.1 ≥40% reclaim target); re-run bench_call_overhead + bench_ic (Pred-tl.5 composition with TB/Φ/Σ).

**Outputs**: `docs/measurements.md` with per-round delta table; trajectory entry with the final disposition; if Pred-tl.1 met, declare the pilot at (P2.a) and queue for findings doc addendum V; if missed, run secondary A/B probe to identify the new dominator (per standing rule 11).

## 3. Composition reading

Multiplicative expected reclaim per round:

| round | move | reclaim signal | expected |
|---|---|---|---:|
| TL-EXT 2 | M1 PushConst | unlocks literal-arg JIT patterns; standalone reclaim near-zero (substrate-introduction signature per Finding II.2-bis) | ~0% on CRB |
| TL-EXT 3 | M2 module-body wrap | entry-mechanism opens; standalone reclaim near-zero on json_parse_transform if inner loop has unsupported ops | ~0% on CRB |
| TL-EXT 4 | M3 GetProp+length-IC | inner loop bound check JIT-able; partial reclaim on outer iter overhead | ~5-15% on CRB |
| TL-EXT 5 | M4 CallMethod+charCodeAt-IC | inner loop body JIT-able; full loop closure | **~40-60% on CRB** (the pipeline-connection point) |
| TL-EXT 6 | composition | measurement | (gate) |

Per Doc 740 §II.2 (P4): cumulative reclaim materializes at the final-tier-closure round (TL-EXT 5). M1-M3 are each substrate-introduction at their respective tier; per Finding II.2-bis, near-zero standalone reclaim is the signature, not pilot failure.

## 4. Pred-tl.1 gating analysis

Current state (post-JSF chain + CharCode-EXT 1+2):
- CRB json_parse_transform: 2188 ms
- checksum loop component: 1480 ms (cruft) vs ~0 ms (node)
- per-charCodeAt-call: 0.592 μs (mostly interp loop dispatch + arithmetic ops per iter; the IC closed the call_function-side overhead)

Target: CRB ≤1500 ms (-40% from JSF-EXT 0 baseline 2481 ms; -32% from current 2188 ms).

Required reclaim: ~688 ms from current state. If the JIT body reduces per-iter cost from ~0.6 μs to ~50 ns (typical Cranelift tight-loop output), the checksum loop drops from 1480 ms to ~125 ms — releases ~1355 ms. **Pred-jsf.1 + Pred-tl.1 both met** at the projected ceiling.

Risk: if the JIT body cannot match Cranelift's tight-loop output (e.g., per-iter GC-root maintenance overhead; bailout chains for unexpected non-ASCII; extern-call thunk overhead for the slow paths), the actual reclaim is bounded above by the per-iter overhead floor. Empirical at TL-EXT 5.

## 5. Risks

**R1 — String encoding bit-layout discovery**: GetProp + CallMethod IC bodies need to bitcast f64 → String pointer. The Value enum's repr (value.rs) needs source-read at TL-EXT 4 implementation time to determine the exact bit-pattern + tag check. Mitigation: read value.rs early in TL-EXT 4; document the encoding in the round's trajectory entry.

**R2 — Module-body GC roots**: the JIT body manipulates locals that are GC roots. If a GC pass fires mid-JIT-body (via an extern call back into Runtime), the JIT body's in-register Values must be visible to the GC. Mitigation: use the existing function-body JIT discipline (which already handles this for TB-eligible bodies); if extra plumbing is needed, surface at TL-EXT 3.

**R3 — Compile-time bail predictability**: the alphabet-check at parse-time must correctly reject any unsupported op so the JIT entry attempt doesn't waste cycles compiling a body that will bail at translate. Mitigation: per-op enumeration is a fixed list (translator.rs:1003-1026); add the 3 new ops at the same site; bail any op not in the union.

**R4 — Composition with TB metadata**: the module-body wrapper isn't a real closure; it has no call_count cell (modules are called once). The TB metadata cache shape doesn't apply. Mitigation: module-tier entry uses a separate "compile once, cache once, never recompile" path; doesn't touch the per-closure metadata.

**R5 — Top-level scope vs function-body scope**: top-level may use Op::LoadGlobal / Op::StoreGlobal for module-level let/const (not LoadLocal). If json_parse_transform's `let parsed = ...` compiles to LoadGlobal/StoreGlobal at module scope, the JIT body bails per C8. Mitigation: empirical — TL-EXT 3 lands the wrapper + reports whether json_parse_transform fires or bails. If it bails for global-scope reasons, **TL-EXT 4+ would need to extend the alphabet for LoadGlobal/StoreGlobal as well**, expanding scope.

**R6 — Per-tier reclaim shortfall**: Doc 740 multi-tier reading predicts cumulative reclaim only after all relevant tiers close. If R6 from the JSF chain's interp-overhead-floor finding applies (the JIT body itself carries irreducible per-iter overhead that bounds reclaim), the pipeline-connection point may be partial rather than full. Mitigation: at TL-EXT 5 / TL-EXT 6, if reclaim falls short of projection, run secondary A/B probe to identify what tier remains.

## 6. Forward to TL-EXT 2

TL-EXT 2 lands Move 1 (PushConst-Number). Substrate-introduction at the alphabet tier; standalone reclaim expected near-zero per Finding II.2-bis; enables the cascade-revival rounds downstream.

---

*TL-EXT 1 closes. 5 rounds enumerated; dependency-ordered; per-round mechanism + LOC + reclaim + falsifier specified. Pred-tl.1 gate projection: ≥40% CRB reclaim achievable at TL-EXT 5 if JIT body matches Cranelift tight-loop output. 6 named risks with mitigations. TL-EXT 2 begins implementation with Move 1 PushConst.*
