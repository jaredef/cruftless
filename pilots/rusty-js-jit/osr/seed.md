# rusty-js-jit/osr — Resume Vector / Seed

**Locale tag**: `L.rusty-js-jit.osr` (nested under LeJIT per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (OSR-EXT 0)**. Spawned per keeper directive 2026-05-23 22:37-local as the (ii) OSR / loop-extraction pivot from the session chapter close (VD locale trajectory). Architectural-tier pilot.

**Workstream**: extend the JIT entry mechanism so a hot inner loop within an otherwise-non-JIT-eligible enclosing scope can JIT independently of the surrounding body's alphabet coverage. Closes the **op-set coverage tier** in Doc 740 §II.2's relevant-tier set R per Finding VII.2 (engagement findings.md Addendum V).

Combined with the prior session's deliverables — VD pilot (value-domain coverage closed, Addendum VI) + TL pilot (entry-mechanism tier closed, but module-body-whole-or-nothing per C8) — OSR is the third of three architectural-tier closures required for full multi-tier pipeline-connection on json_parse_transform per Doc 740 §II.2 (P4).

**Author**: 2026-05-23 session close → next session.
**Parent**: LeJIT (`pilots/rusty-js-jit/`).
**Siblings**:
- `top-level/` (TL pilot; (b-narrow) chapter closed at TL-EXT 3; entry-mechanism for module body)
- `value-domain/` (VD pilot; first cut closed at (P2.a); String NaN-boxing encoding)
- `f64-calling-convention/` (Φ; default-on; Number+Object encoding baseline)

**Composes with**:
- [Findings doc Addendum V](../findings.md) — Finding VII.2 op-set-coverage check (the apparatus that named this pilot's scope)
- [Doc 740](../../../../corpus-master/corpus/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) — multi-tier reading; this pilot closes the 3rd of 4 tiers in R for json_parse_transform
- [Doc 731 §XIV.d](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — alphabet purity; OSR extracts a sub-region with a tighter alphabet, preserving the purity invariant
- [VD pilot](../value-domain/seed.md) — String encoding consumed by hot intrinsics in OSR-extracted loops
- [TL pilot](../top-level/seed.md) — TL's (b-narrow) blocker (Finding TL.1) is what OSR closes structurally
- [JSF-EXT 8 component A/B probe](../../rusty-js-json-fast/fixtures/component-ab-probe.mjs) — standing instrument for re-measurement at OSR-EXT N close

## I. Telos

**Empirical answer to**: can a hot inner loop within a non-JIT-eligible enclosing module/function body be extracted into a JIT-eligible sub-region whose JIT entry fires independently of the surrounding body's alphabet coverage?

The bench-anchored target: post-implementation, json_parse_transform's checksum loop (top-level for-loop with `for (i; i<out.length; i++) cs += out.charCodeAt(i)`) is JIT-compiled and invoked from interp once per outer iteration (or once per hot-detection threshold). Combined with VD's String encoding + TL's module-body wrapper + alphabet extensions for the loop body's specific ops (GetProp+length-IC, CallMethod+charCodeAt-IC; deferred to a follow-on alphabet pilot or folded into OSR's scope per design decision), Pred-jsf.1 (≥40% CRB reclaim on json_parse_transform) becomes empirically reachable.

### I.1 First-cut scope (deferred — pending OSR-EXT 1 design doc)

Scope decisions deferred to OSR-EXT 1 (design doc round). Candidate first-cut shapes:

- **A) Runtime bytecode transform**: at interp loop entry, detect "loop body is JIT-eligible alphabet + integer-accumulator-shape"; rewrite bytecode in-place to insert a "JIT this sub-region" marker; JIT-compile the sub-region the first time control reaches it; subsequent iterations call the JIT'd loop body.
- **B) Static loop detection at compile time**: the bytecode compiler emits a "JIT-eligible loop region" marker alongside the normal bytecode at every loop construct; the JIT entry mechanism reads the marker at module/function entry and JIT-compiles the marked sub-regions.
- **C) On-stack replacement (OSR) per V8 / SpiderMonkey shape**: at loop back-edge, count iterations; when threshold exceeded, JIT-compile the loop, recover register state, jump to JIT'd code at the loop header pc. Most invasive; matches production engines.

Per Pin-Art scope discipline: design doc enumerates A/B/C with per-option scope estimates + falsifier anchoring before keeper selects.

### I.2 Constraints (Pin-Art enumeration per Φ §I.2 discipline)

```
C1. Existing Σ/Τ/Ψ/Φ default-on paths produce byte-identical bench
    numbers post-OSR (composition gate).
C2. ECMA semantics preserved: loop iteration count + side effects +
    completion-value semantics + try/catch + break/continue +
    deopt/bail-on-unexpected-state-back-to-interp.
C3. Canonical fuzz acc=-932188103 byte-identical throughout.
C4. diff-prod 42/42 throughout.
C5. JIT lib tests pass throughout (existing 9 ignored remain ignored).
C6. Composition with VD String encoding + TL module-body wrapper +
    existing function-body TB metadata cache (TB-EXT 7 Box-wrap).
C7. GC root preservation: locals across OSR entry / re-entry must remain
    visible to the GC; the OSR-extracted loop body must not hide
    a GC root from a scavenger pass.
C8. Bail discipline: any unexpected state (deopt, exception thrown
    from inside JIT body, non-Number value where Number expected, etc.)
    falls through to interp at a well-defined pc; no silent corruption.

Architecture induced by C1-C8: option (A) or (B) per OSR-EXT 1 design;
(C) deferred unless first cut requires it. The first cut prioritizes
"OSR-extracted loop body runs the existing JIT pipeline (translator,
deopt) with a synthetic FunctionProto-equivalent" — composable with
VD + TL substrate.
```

### I.3 Falsifiers (provisional; refined at OSR-EXT 1)

**Pred-osr.1**: post-implementation, json_parse_transform CRB fixture drops by ≥40% wall-clock vs JSF-EXT 0 baseline (target 2481 ms → ≤1500 ms; closes Pred-jsf.1 + Pred-tl.1). Falsifier: <40% reclaim → another tier is missing or the OSR mechanism's overhead dominates.

**Pred-osr.2**: canonical fuzz remains byte-identical (acc=-932188103). Falsifier: divergence → (P2.c) illegal-speed bug at OSR boundary.

**Pred-osr.3**: diff-prod 42/42 holds. Falsifier: any regression.

**Pred-osr.4**: composition with existing TB/Φ/Σ/VD/TL defaults holds; bench_call_overhead + bench_ic + A/B probe stay within ±5% of post-VD baselines. Falsifier: regression → OSR mechanism broke existing JIT paths.

**Pred-osr.5**: scope discipline — first cut targets ONLY the json_parse_transform charCodeAt-loop shape; other hot loops (Array.filter callbacks, JSON.parse internal loops, etc.) are NOT JIT'd in the first cut. Falsifier: any other loop accidentally JIT'd.

## II. Apparatus

- **Source-read** at OSR-EXT 1: how do existing engines (V8, SpiderMonkey) handle OSR? Brief; for architectural reference, not source.
- **Bytecode compiler** (`pilots/rusty-js-bytecode/derived/src/compiler.rs`): if option (B), emit loop-region markers.
- **Interp dispatcher** (`pilots/rusty-js-runtime/derived/src/interp.rs`): if option (A), detect loop entry + invoke JIT; if option (C), implement back-edge counter + state recovery.
- **JIT crate** (`pilots/rusty-js-jit/derived/src/translator.rs`): new entry point for sub-region compilation (takes bytecode slice + locals layout + entry pc); existing translator reused.
- **Alphabet extensions**: GetProp+length-IC + CallMethod+charCodeAt-IC consuming VD String encoding. Either folded into OSR pilot or spawned as separate sub-pilot per design decision.
- **Correctness instruments** (rule 5 + rule 10 + rule 11 + rule 12): canonical fuzz + diff-prod + JIT lib tests + A/B probe + adversarial-state unit tests at each round.

## III. Methodology

1. **OSR-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **OSR-EXT 1** — design doc: enumerate A/B/C options; per-option scope + LOC + reclaim estimate + falsifier anchoring + risk enumeration; keeper selects. Output: `docs/design.md`.
3. **OSR-EXT 2+** — implementation per the selected option. Round count depends on selection (A: ~4 rounds; B: ~6 rounds; C: ~10+ rounds).
4. **OSR-EXT N+1** — composition probe + CRB final disposition + Pred-osr.1 gate.
5. **OSR-EXT N+2** (conditional) — Findings doc Addendum VII if a new pattern recognition emerges.

## IV. Carve-outs and bounded scope

- json_parse_transform CRB fixture as load-bearing measurement; other CRB fixtures observed but not gated.
- Aarch64 only (engagement reference).
- First-cut: ONE loop shape (charCodeAt-loop pattern); generalization deferred to follow-on rounds.
- Alphabet extensions (GetProp+length-IC; CallMethod+charCodeAt-IC) folded in or spawned as separate sub-pilot per OSR-EXT 1 design.
- Per Findings rule 5 + standing rule 10: three-probe-levels at each round.
- Per standing rule 11: A/B probe re-measurement at OSR-EXT N close + at any default-on flip.
- Per standing rule 12: if OSR introduces a bit-pattern-tagging scheme, adversarial special-value tests.

## V. Standing artefacts

- `pilots/rusty-js-jit/osr/seed.md`, `trajectory.md`
- `pilots/rusty-js-jit/osr/docs/design.md` (OSR-EXT 1)
- `pilots/rusty-js-jit/osr/fixtures/` for pilot-specific fixtures
- Implementation lands in `pilots/rusty-js-runtime/derived/src/interp.rs` + `pilots/rusty-js-jit/derived/src/translator.rs` + possibly `pilots/rusty-js-bytecode/derived/src/compiler.rs`

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read VD locale's session-close entry (the architectural-pivot summary at VD-EXT 3) for full session context. Read Doc 740 §II.2 multi-tier reading + Findings VII.2 + VII.3 + VIII.1 + standing rules 11 + 12 (Addenda V + VI) for apparatus context. Read TL findings.md TL.1 for the structural blocker that OSR closes. Φ seed §I.2 for the constraint-enumeration discipline reused here.
