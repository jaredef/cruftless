# OSR-EXT 1 — Design: A/B/C scope options for the op-set coverage closure

*Enumerates three structural options for closing Finding VII.2 (whole-body bail discipline gates inner-loop JIT eligibility on FULL enclosing-scope alphabet coverage). Per-option scope + LOC + reclaim + falsifier + risks. Keeper selects at round close.*

## 1. The problem (one-paragraph recap)

cruft's JIT entry mechanism today is whole-function (or whole-module post-TL-EXT 3) bail. If ANY op in the enclosing scope falls outside the alphabet, the JIT cannot compile the body at all. For json_parse_transform's top-level body — which uses MakeClosure / Op::Call / Op::CallMethod / LoadGlobal / StoreGlobal for the `const payload = makePayload(N)` / `const text = JSON.stringify(payload)` / outer for-loop setup / etc. — the inner charCodeAt loop's JIT eligibility is gated on every other op in the body also being JIT-eligible. Closing the entire body's alphabet is large scope. OSR (or loop-extraction) closes the gap structurally by reducing the enclosing scope from "whole module body" to "just the inner loop."

## 2. Constraints (recap from seed §I.2)

```
C1-C8 per seed. Critical for design choice:
  C2  ECMA loop semantics (break/continue/return/throw at any iter; +
      side effects + completion-value)
  C6  Compose with VD String encoding + TL module-body wrapper + TB
      function-body metadata cache
  C7  GC root preservation across OSR entry/re-entry
  C8  Bail discipline at well-defined pc; no silent corruption
```

## 3. Option A — Runtime bytecode transform (RBT)

**Mechanism**: in the interp dispatch loop, when entering a backward-jump (loop back-edge), increment a per-pc counter. When the counter exceeds a threshold (e.g., 1000), pause and attempt:

1. Walk forward from the loop entry pc to identify the loop body bytecode range (loop entry → matching back-edge).
2. Build a synthetic FunctionProto wrapping the loop-body bytecode + the current frame's local layout + the current frame's local values as initial state.
3. JIT-compile the synthetic proto via the existing `compile_function` path.
4. If compile succeeds, call the JIT'd loop body once. The JIT body's locals are populated from the frame; the JIT body runs the loop until its natural exit (back-edge target == loop-exit pc) or until a deopt fires. On return, the frame's locals are updated from the JIT body's final state.
5. On any compile failure or deopt, fall through to interp at the original pc.

**Composition**: reuses the existing JIT pipeline (translator, deopt, TB metadata where applicable). Reuses VD String encoding for any String locals the loop body references. Reuses TL's synthetic-FunctionProto pattern.

**Subtleties**:
- Loop bytecode boundary detection: the bytecode currently doesn't carry an explicit loop marker. The runtime detects the back-edge (Jump with negative displacement) at iter N+1; identifying the loop entry pc requires either tracking the back-edge target as the entry, or scanning the bytecode to find the matching forward Jump that exits the loop (the break target).
- Local-state synchronization: the loop body reads/writes the frame's locals; the JIT body's locals are register-allocated by Cranelift. At JIT body entry, locals must be copied from frame → JIT registers; at exit, copied back.
- Side effects within JIT body: any extern call (e.g., a method dispatch that bails to interp) must preserve the frame's state correctly. Existing JIT deopt machinery handles this for function-body JIT; OSR adds the constraint that the deopt resumes at a mid-loop pc, not the loop entry.

**LOC estimate**: ~250-350 (bytecode boundary detection + synthetic FunctionProto builder + local-state copy in/out + back-edge counter + integration with deopt to resume at correct pc).

**Per-fixture reclaim**: high if the loop body is JIT-eligible per (b-narrow) alphabet + VD String encoding. For json_parse_transform's checksum loop: closes the residual 1480 ms → ~50-200 ms (8-30× reclaim on the dominator-loop). CRB projection: 2188 → 700-1000 ms; **Pred-osr.1 ≥40% target reached**.

**Risks**:
- R1 (boundary detection): backward Jump dispatched at end of iter; the loop entry is the target of that Jump. ECMA `break` ops also produce forward Jumps; need to disambiguate. Mitigation: pattern-match on Op::Jump opcode + sign of displacement; the loop entry is uniquely identified by being the back-edge target.
- R2 (local-state sync overhead): copy-in + copy-out per JIT body call. For loops with many iterations, the cost amortizes; for short loops, the overhead may dominate. Mitigation: threshold-gate the JIT entry (skip if iteration count < 100).
- R3 (deopt mid-loop): if the JIT body deopts at mid-loop pc, the existing deopt-recover machinery (JIT-EXT 21 resume_from_deopt_state) needs to handle resuming at the deopt-pc, not the loop entry. May require deopt-recovery enhancements.
- R4 (mutation during JIT body): if the loop body's extern call mutates the frame's locals (e.g., via a GC pass that touches Cells), the JIT body's in-register copies diverge. Mitigation: bail to interp on any extern call that returns; conservative first cut.

**Falsifier**: Pred-osr.1 ≥40% CRB reclaim. If <40%, the JIT body's per-iter overhead floor dominates.

## 4. Option B — Compile-time loop marker

**Mechanism**: the bytecode compiler emits a `Op::LoopRegionMark { kind: Enter, region_id }` at loop entry + `Op::LoopRegionMark { kind: Exit, region_id }` at loop exit. The runtime detects the mark at compile time (when building CompiledModule); for each region, builds a sub-FunctionProto (bytecode-slice + local layout); at module load + JIT entry attempt, tries to compile each region; caches the compiled regions.

Runtime dispatch: when the interp reaches a LoopRegionMark(Enter) and the region is JIT-compiled, jump into the JIT'd region; on return, resume interp at the Exit pc.

**Composition**: similar to A, but the marker-driven approach moves loop detection from runtime to compile time. Each loop region is a discrete JIT compilation unit; precompilation at module load amortizes the compile cost.

**Subtleties**:
- New bytecode ops (Op::LoopRegionMark) require updating the bytecode compiler + serialization + every op-table dispatch in the interp. Cross-cutting change.
- The bytecode compiler emits markers around every loop construct (for/while/do-while/for-in/for-of); the JIT may compile a subset that fits the alphabet.
- Marker dispatch in interp adds a small overhead per loop entry/exit even when the region isn't JIT-compiled.

**LOC estimate**: ~500-700 (compiler markers at 5+ loop constructs + new Op variants + serialization + interp dispatch + sub-FunctionProto builder + per-region compilation cache + entry/exit dispatch logic).

**Per-fixture reclaim**: same as A in the ideal case (closes the residual 1480 ms). Compile cost is paid at module load (one-time amortized).

**Risks**:
- R1 (cross-cutting bytecode change): every existing fixture's bytecode shape changes (new Op present). Diff-prod risk: any fixture whose semantics depend on bytecode layout (e.g., the deopt mid-pc recovery tables, source maps) needs to handle the new ops. Mitigation: opt-in flag at compile time; off by default until validated.
- R2 (loop construct enumeration): for/while/do-while/for-in/for-of/labeled break — each requires careful marker placement to NOT cross try/catch boundaries or other ECMA semantic boundaries.
- R3 (compile-time over-eager marker emission): markers around loops that NEVER get hot waste bytecode space; markers around loops that bail at JIT-compile-time waste compile cost. Mitigation: heuristic at compile time (e.g., only mark loops whose body LOC < 50).

**Falsifier**: Pred-osr.1 ≥40% CRB reclaim + diff-prod 42/42 (the marker emission must not break any existing fixture).

## 5. Option C — V8/SpiderMonkey-style OSR with back-edge counter + state recovery

**Mechanism**: the production-engine standard. The interp adds a per-back-edge counter; on threshold trip, suspend the interp loop, take a snapshot of all live locals + operand stack + pc, JIT-compile the FUNCTION (not the loop body — the whole enclosing function) with an OSR entry point at the back-edge pc, jump into the JIT'd code at the OSR entry, restore state.

**Composition**: requires the whole enclosing function to be JIT-eligible (back to Finding VII.2 op-set coverage at the function-body scope, NOT the loop scope). This option **does not actually close Finding VII.2** because it still bails on the surrounding function's alphabet. The state-recovery mechanism is the value-add (jumping into JIT mid-function), not the scope reduction.

**Why this is wrong for our case**: V8/SpiderMonkey use OSR because they already have full-function JIT support; OSR is the entry-mechanism optimization on top. cruft's blocker is the whole-function alphabet, not the entry mechanism for already-JIT-eligible functions. **Option C does not close our gap.**

**LOC estimate**: ~800+ (back-edge counter machinery + state snapshot at suspend + OSR-entry IR emission in the JIT + state restore at JIT entry + deopt handling at OSR boundary). Largest by 2-3×.

**Per-fixture reclaim**: 0% on json_parse_transform (the surrounding function still bails).

**Risks**:
- R1: doesn't close Finding VII.2.
- R2: largest implementation cost.
- R3: requires deeper Cranelift integration (OSR entry blocks; sparse register state recovery).

**Recommendation**: rule out (C) for the (ii) pivot. (C) is a future optimization if/when cruft's whole-function JIT scope grows to cover the relevant CRB fixtures' enclosing functions.

## 6. Comparative table

| dimension | A (RBT) | B (compile-marker) | C (V8-style OSR) |
|---|---|---|---|
| closes Finding VII.2 | yes (loop scope) | yes (loop scope) | no (function scope) |
| LOC estimate | 250-350 | 500-700 | 800+ |
| cross-cutting change | low (interp + JIT integration only) | high (bytecode op set + compiler + serialization) | medium (interp + JIT IR) |
| compile cost | per first hot detection | at module load | per first hot detection |
| extra runtime overhead (cold loops) | low (back-edge counter only) | low (marker dispatch) | low (back-edge counter only) |
| risk to existing default-on paths (Σ/Τ/Ψ/Φ/VD) | low (additive) | medium (new ops) | medium (counter + IR changes) |
| reclaim on json_parse_transform | high (closes residual) | high (closes residual) | 0% |
| matches engagement Pin-Art discipline | strong (smallest change closing the gap) | medium (cross-cutting) | weak (doesn't close gap) |

## 7. Recommendation

**Option A — Runtime bytecode transform (RBT).** Rationale:

1. **Closes Finding VII.2** by reducing enclosing scope to the loop body, matching the structural insight that the gap is whole-body-bail, not entry-mechanism.
2. **Smallest LOC estimate** among options that close the gap.
3. **Localized to the interp dispatcher** (no cross-cutting bytecode-op-set change; no compiler change at first cut).
4. **Reuses existing JIT pipeline + VD encoding + TL synthetic-FunctionProto pattern**. Cumulative cost across the architectural session is leveraged.
5. **Composition risk is low**: additive to existing default-on paths; the back-edge counter and JIT entry attempt are opt-in by hot-detection threshold.

Option B is viable but adds bytecode-op-set churn that compounds across every fixture. Defer to a follow-on round if A's runtime detection proves insufficient (e.g., if loop-boundary detection fails on common bytecode patterns).

Option C should be ruled out for this pivot.

## 8. First-cut scope for Option A (if selected)

OSR-EXT 2-N rounds under Option A:

1. **OSR-EXT 2** — Back-edge counter + threshold detection. Substrate-introduction; no JIT entry yet.
2. **OSR-EXT 3** — Loop bytecode boundary detection (forward-scan from back-edge target to identify the loop body's pc range). Substrate-introduction.
3. **OSR-EXT 4** — Synthetic FunctionProto builder for the loop region + JIT compile attempt + bail-clean-on-failure.
4. **OSR-EXT 5** — Local-state copy-in + JIT body invoke + copy-out. Cascade-revival pilot.
5. **OSR-EXT 6** — Alphabet extension for the loop body (GetProp+length-IC; CallMethod+charCodeAt-IC — the TL Moves 3+4 revival folded in or spawned as sibling). Cascade-revival pilot #2.
6. **OSR-EXT 7** — Composition probe + CRB final disposition + Pred-osr.1 gate.

Each round runs three-probe-levels (canonical fuzz + diff-prod + JIT lib tests) per rule 10 + rule 11 + rule 12 (no new bit-pattern schemes expected; rule 12 holds vacuously).

## 9. Forward to OSR-EXT 2

Pending keeper selection of A/B/C. If A: OSR-EXT 2 lands the back-edge counter + threshold detection. ~50 LOC; substrate-introduction signature expected (flat A/B probe; the counter alone doesn't trigger JIT).

---

*OSR-EXT 1 closes. Three options enumerated; A (RBT) recommended; per-option scope + reclaim + risks + composition + falsifier specified. Pivot decision pending keeper signal.*
