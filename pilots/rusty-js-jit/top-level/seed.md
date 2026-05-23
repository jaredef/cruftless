# rusty-js-jit/top-level — Resume Vector / Seed

**Locale tag**: `L.rusty-js-jit.top-level` (nested under LeJIT per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (TL-EXT 0)**. Spawned per keeper directive 2026-05-23 20:52-local as the (b-narrow) first cut from Doc 740's multi-tier reading. Addresses the json_parse_transform CRB residual after the JSF + CharCode chain landed (cumulative -12%; cruft/node 17.93×; residual checksum loop 1480 ms at 0.592 μs per charCodeAt call). The dispatch-tier IC closed the per-call cost floor; the remaining residual is interp loop dispatch + arithmetic + comparison overhead per iter. Top-level JIT closes that residual.

**Workstream**: extend LeJIT to JIT top-level module bytecode + extend JIT alphabet for the ops that appear in json_parse_transform's inner for-loop. Per Doc 740 §II.3 multi-tier reading: this pilot closes both the entry-mechanism tier AND the alphabet-coverage tier in dependency order.

**Author**: 2026-05-23 session.
**Parent**: LeJIT (`pilots/rusty-js-jit/`).
**Composes with**:
- [Doc 740](../../../../corpus-master/corpus/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) — multi-tier cascade-revival apparatus; this pilot is the (b-narrow) instantiation
- [Doc 739](../../../../corpus-master/corpus/739-constraint-closure-as-cascade-revival-when-lifting-an-upstream-structural-constraint-auto-resolves-stalled-sibling-pilots.md) — single-tier cascade-revival; the alphabet extensions are downstream of the entry-mechanism closure
- [Doc 731 §XIV.d](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — alphabet purity; this pilot extends the alphabet narrowly per Pred-tl.4
- [Φ seed §I.2](../f64-calling-convention/seed.md) — constraint enumeration discipline; reused here for entry-mechanism + alphabet design
- [CharCode-EXT 2](../../rusty-js-runtime/derived/src/interp.rs) — interp-tier hot-intrinsic IC pattern; the JIT-tier CallMethod lowering reuses the same IC-shape
- [JSF-EXT 8 component A/B probe](../../rusty-js-json-fast/fixtures/component-ab-probe.mjs) — standing instrument for re-measurement at each round
- [Findings doc Addendum IV](../findings.md) — Finding II.3 multi-tier cascade-revival; standing rule 11 component-A/B-before-spawn (already satisfied via JSF-EXT 8)

## I. Telos

**Empirical answer to**: can LeJIT close the remaining ~1480 ms residual on the json_parse_transform charCodeAt loop by extending entry mechanism (top-level JIT eligibility) + JIT alphabet (Op::PushConst, Op::GetProp via String-length IC, Op::CallMethod via charCodeAt IC)?

Per JSF-EXT 10 measurement: residual checksum loop 1480 ms at 0.592 μs per charCodeAt call (the IC bypassed call_function; the remainder is interp loop dispatch per iter — fetch op, dispatch, op body, GC root maintenance per push/pop). JIT'ing the loop body would replace interp dispatch with direct machine code; the per-iter overhead drops to ~10-50 ns (typical Cranelift output for tight loops).

Expected ceiling: 1480 ms residual → ~50-200 ms via top-level JIT (8-30× reclaim on the dominator-loop). Translated to CRB: ~2188 ms → ~700-1000 ms (40-60% reclaim from JSF-EXT 0 baseline 2481 ms; meets Pred-jsf.1 target ≤1500 ms via this multi-tier chain).

### I.1 First-cut scope (b-narrow)

- **Top-level module-body JIT entry**: wrap module bytecode as a JIT-eligible pseudo-function; JIT-compile at first module-body call; cache the compiled artifact.
- **JIT alphabet extension** (narrow): three new ops only.
  - **Op::PushConst** — recognize int/string/null/undefined constants; emit as immediate.
  - **Op::GetProp** via property-key IC — narrow to the String.length-getter shape first (receiver==Value::String + key=="length"); inline ASCII length-as-f64.
  - **Op::CallMethod** via cached-intrinsic IC — narrow to the charCodeAt shape first (n==1, receiver==Value::String, method==cached charCodeAt id); inline the ASCII byte-fetch directly. Bail to extern for the slow path.
- **Eligibility check**: existing JIT eligibility criteria still apply (bytecode length threshold, op-set coverage); top-level entry adds a wrapper but doesn't lower the bar.
- **No OSR**: top-level body is JIT'd in entirety or not at all. If the body contains unsupported ops outside the inner loop, the whole module bails to interp.

Out of scope (deferred to follow-on):
- Other JIT alphabet ops (LoadGlobal, StoreGlobal, MakeClosure, etc.)
- Other intrinsic IC shapes (charAt, push, indexOf, etc.)
- OSR / loop extraction for non-top-level hot loops
- f64 fast-path for the IC-bailed slow paths (already at Φ-tier maturity)

### I.2 Constraints (per Pin-Art apparatus + Φ §I.2 enumeration discipline)

```
C1. Correctness preserved: canonical fuzz acc=-932188103 byte-identical; diff-prod 42/42 throughout.
C2. ECMA-262 §10.x semantics preserved (module-body execution order; var/let scoping; global writes).
C3. No bench-fixture restructuring; json_parse_transform stays at top-level shape per CRB measurement comparability.
C4. Hot-intrinsic IC discipline preserved at the JIT tier: verify against cached intrinsic ObjectId before fast-path; bail to extern (existing call_function) on mismatch.
C5. Top-level entry must not break existing function-body JIT (TB-EXT 7 metadata cache integrity; jit_cache HashMap stability per standing rule 9).
C6. Module-body JIT must respect GC roots (top-level locals are GC-root candidates; the JIT body must preserve them across calls into extern helpers).
C7. PushConst lowering must not break Φ-EXT 3 f64-default calling convention (constants flow as f64 in the f64 alphabet; integer constants get bitcast).
C8. Bail discipline: any unsupported op in the module body causes the whole module to fall through to interp (no partial compilation); the existing per-op alphabet check applies.

The architecture induced by C1-C8: ModuleProto-like wrapper around module bytecode; existing compile_function called with the wrapped proto; alphabet extensions are op-by-op additive (each in its own round); IC shapes mirror CharCode-EXT 2's interp-tier pattern.
```

### I.3 Falsifiers

**Pred-tl.1**: post-implementation, json_parse_transform CRB fixture drops by ≥40% wall-clock vs JSF-EXT 0 baseline (target 2481 ms → ≤1500 ms; closes the Pred-jsf.1 target that JSF + CharCode chain alone did not meet). Falsifier: <40% cumulative reclaim → either the alphabet extensions didn't fire, or the residual cost lives elsewhere (Array.map dispatcher? interp module-tier overhead? JIT body emission inefficiency?).

**Pred-tl.2**: canonical fuzz (CMig-EXT 17 fuzz-canonical.mjs) remains byte-identical post-implementation (acc=-932188103 vs node). Falsifier: divergence → (P2.c) illegal-speed bug at the JIT body or IC fast-path.

**Pred-tl.3**: diff-prod 42/42 holds. Falsifier: any regression.

**Pred-tl.4**: JIT alphabet extensions stay within the (b-narrow) scope: PushConst + GetProp-String-length-IC + CallMethod-charCodeAt-IC. No accidental scope creep into other alphabet ops or IC shapes during the first cut. Falsifier: any other op added to ParsedOp in this pilot's rounds.

**Pred-tl.5**: composition with existing TB + Φ + Σ default-on holds. bench_call_overhead + bench_ic per-iter latencies stay within ±5% of post-Φ baselines. Falsifier: regression → top-level wrapping broke function-body JIT eligibility.

## II. Apparatus

- **Translator** (`pilots/rusty-js-jit/derived/src/translator.rs`): extend ParsedOp enum + lowering for the three new ops.
- **Tiny-baseline metadata** (`pilots/rusty-js-jit/derived/src/tiny_baseline.rs`): may need a ModuleMetadata variant or unification with TinyBaselineMetadata.
- **Bytecode** (`pilots/rusty-js-bytecode/derived/src/compiler.rs`): inspect CompiledModule to wrap as FunctionProto-equivalent for JIT consumption.
- **Interpreter** (`pilots/rusty-js-runtime/derived/src/interp.rs`): module-tier entry path (Runtime::run_module ~6503); add JIT eligibility check at module entry.
- **IC verification**: reuse Runtime::intrinsic_string_charcodeat_id (CharCode-EXT 2); add string_length_getter_id for the GetProp IC if needed.
- **Bench instrument**: `pilots/rusty-js-json-fast/fixtures/component-ab-probe.mjs` (re-run each round) + CRB json_parse_transform (final disposition).
- **Correctness instruments** (rule 5 + rule 10): canonical fuzz + diff-prod at each round.

## III. Methodology

1. **TL-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **TL-EXT 1** — design doc: enumerate the 4-5 substrate moves per the (b-narrow) scope; per-move LOC + reclaim estimate + falsifier anchoring. Output: `docs/design.md`.
3. **TL-EXT 2** — Move 1: Op::PushConst in JIT alphabet. Substrate-introduction (enables literal-arg patterns in JIT bodies).
4. **TL-EXT 3** — Move 2: module-body JIT entry wrapper. Substrate-introduction at the entry-mechanism tier.
5. **TL-EXT 4** — Move 3: Op::CallMethod with charCodeAt IC inlined in JIT. Cascade-revival pilot #1 (consumer of Move 2 entry + Move 1 PushConst).
6. **TL-EXT 5** — Move 4: Op::GetProp with String-length IC inlined in JIT. Cascade-revival pilot #2.
7. **TL-EXT 6** — composition probe: A/B re-measurement + CRB final disposition + Pred-tl.1 gate.
8. **TL-EXT 7** (conditional) — Findings doc Addendum V if the multi-tier reading produces a new pattern recognition.

## IV. Carve-outs and bounded scope

- json_parse_transform CRB fixture only as the load-bearing measurement; other CRB fixtures observed but not gated.
- Aarch64 only (engagement reference).
- Top-level entry as module-body wrap; no OSR; no loop extraction.
- Alphabet additions strictly the three named ops; other ops deferred.
- IC shapes strictly charCodeAt + String-length; other intrinsic ICs deferred.
- Per Findings rule 5 + standing rule 10: each round runs canonical fuzz + diff-prod + A/B probe + (at TL-EXT 6) CRB.

## V. Standing artefacts

- `pilots/rusty-js-jit/top-level/seed.md`, `trajectory.md`
- `pilots/rusty-js-jit/top-level/docs/design.md` (TL-EXT 1)
- `pilots/rusty-js-jit/top-level/fixtures/` for any pilot-specific test fixtures
- Implementation lands in `pilots/rusty-js-jit/derived/src/translator.rs`, `tiny_baseline.rs`, and `pilots/rusty-js-runtime/derived/src/interp.rs`

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read Doc 740 for the multi-tier cascade-revival apparatus that this pilot instantiates. Read the JSF locale's JSF-EXT 8-10 trajectory entries for the empirical context (component A/B probe; CharCode-EXT 1+2 substrate + dispatch closures).
