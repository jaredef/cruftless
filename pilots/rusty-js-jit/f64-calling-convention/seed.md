# LeJIT-Φ (f64-calling-convention) — Resume Vector / Seed

*(Internal name LeJIT-Φ — fourth nested sibling under `pilots/rusty-js-jit/`, alongside LeJIT-Σ (stub-emitter), LeJIT-Τ (tiny-baseline), and LeJIT-Ψ (value-tag-inline). The "Φ" (phi) reads as "f64." On-disk source code will land at `pilots/rusty-js-jit/derived/src/translator.rs` (modified) + dispatch sites in `pilots/rusty-js-runtime/derived/src/interp.rs`; the seed + trajectory at this coordinate hold the Pin-Art record.)*

**Locale tag**: `L.rusty-js-jit/f64-calling-convention` (nested per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (Φ-EXT 0)**. No code yet. Spawned per keeper directive 2026-05-23 16:02-local after pre-implementation analysis surfaced that VTI's structural (P2.d) traces to the JIT's i64-only calling convention. The architectural move induced by enumerating C1-C10 (substrate constraints; see §I.2) is: f64 default + bytecode-tier-driven typed-i64 promoted fast path.

**Workstream**: switch the JIT calling convention from i64-everywhere to f64-default with optional typed-i64 specialization. Removes the dispatcher precheck's integer-validity check that forced VTI into (P2.d). Composes with existing default-on flips (shape, STUB, TB) without regressing the engagement-tier baseline.

**Author**: 2026-05-23 session.
**Parent**: `pilots/rusty-js-jit/` (LeJIT).
**Siblings**:
- `pilots/rusty-js-jit/stub-emitter/` (LeJIT-Σ) — closed at engagement default
- `pilots/rusty-js-jit/tiny-baseline/` (LeJIT-Τ) — closed at engagement default
- `pilots/rusty-js-jit/value-tag-inline/` (LeJIT-Ψ) — closed at (P2.d); revival path is THIS pilot's downstream consequence

**Composes with**:
- [LeJIT seed §I.1](../seed.md) — the first cut's "typed-i64 alphabet first; f64 deferred" carve-out. Φ closes that deferral.
- [LeJIT seed §I.2](../seed.md) — the hybrid stance's four hand-rolled regions. Φ is the architectural complement: lifts the value-domain interface between the Cranelift and hand-rolled tiers.
- [LeJIT seed §I.3 + CRB-EXT 8 amendment](../seed.md) — composition reading. Φ's per-workload effect is bounded by C10 (preserve engagement-tier baseline on covered workloads).
- [LeJIT seed §I.4](../seed.md) — first-cut composition empirically met. Φ is the natural §I.5 evolution after first-cut chapter closes.
- [LeJIT findings.md V.3 + V.6](../findings.md) — VTI (P2.d) finding + LeJIT-first-cut-met finding. Φ resolves V.3 by removing the architectural constraint VTI was working around.
- [LeJIT enhancements.md TB-EXT 7 entry](../enhancements.md) — the calling-convention-restructure-CAN-pay-when-precheck-removed insight; Φ generalizes it.
- [Doc 731 §VII R1, R5, R6, R8](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — preserve single-tier, finite deopt, straight-line lowering.
- [Doc 731 §XIII alphabet promotion](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — bytecode-tier typing as the inversion vs profile-driven specialization. The typed-i64 fast path (a follow-on pilot, NOT in Φ's scope) lives at the bytecode tier.
- [Doc 735 §X.h.b + .c](../../../../corpus-master/corpus/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — (P2.a)/(P2.d) categorization + three-probe-levels gate.
- [Doc 736 §IX.6](../../../../corpus-master/corpus/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — cap-passing modes preserved (Φ doesn't touch cap surface; JIT body still primitive).
- [Doc 737 §IV](../../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md) — fourth nested LeJIT-tier locale.
- [Doc 738 §II](../../../../corpus-master/corpus/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md) — source-tier convention space.

## I. Telos

**Empirical answer to**: with cruft's typed-i64 JIT carve-out hitting its limit at VTI's (P2.d), what architectural move resolves the structural constraint while preserving the engagement-tier baseline?

Per the constraint analysis (§I.2 below), the induced architecture is **f64 default + bytecode-tier-driven typed-i64 promoted fast path**. This pilot lands Move 1 (f64 default); the typed-i64 promoted fast path is a separate downstream pilot at the bytecode tier (not Φ's scope).

### I.1 First-cut telos

A single-tier JIT whose calling convention is f64 throughout. JIT body does fadd/fsub/fmul; receives f64 args (no i64 truncation); returns f64 (no rebox). The dispatcher's precheck collapses from "integer-Number-or-Object" to "Number-or-Object" — implementable as inline tag-check at JIT-prologue, finally giving VTI a structurally winnable revival.

**The first cut closure criterion**:
1. JIT translator emits fadd/fsub/fmul for Op::Add/Sub/Mul (replacing iadd/isub/imul)
2. Dispatcher passes f64 args (replacing i64 truncation)
3. Dispatcher's `jit_compatible_arg` collapses to tag-only (was: tag + integer-validity)
4. Composition with shape + STUB + TB default-on preserved
5. diff-prod 42/42 holds
6. Fuzz fixture (5-pattern multi-shape) holds
7. Bench probes: bench_call_overhead within ±15% of post-flip baseline (71.2 ns); bench_ic within ±10% (81 ns). f64 arith may slow per-op marginally on Pi.

### I.2 The induced architecture — constraint enumeration

Ten substrate-tier constraints (C1-C10) name the invariants any new JIT architecture must respect. The constraints are themselves the substrate-improvement framework for Φ:

```
C1. JS Number semantics are f64. Any deviation requires proven-correct
    typed specialization.
C2. Bytecode alphabet is the JIT's input contract. The bytecode tier owns
    the typing discipline.
C3. Single-tier per Doc 731 §VII R1.
C4. Deopt is finite-enumerable per Doc 731 §VII R5.
C5. Composes with shape + STUB + TB default-on.
C6. No internal optimization passes per Doc 731 §VII R8. Cranelift OptLevel::None.
C7. Cap-passing modes preserved (Doc 736 §IX.6).
C8. Cross-arch via Cranelift OR hand-rolled per architecture.
C9. Bench probes catch (P2.c) per Findings rule 5 + standing rule 9.
C10. Engagement-tier baseline (71/81 ns post-flips) preserved on covered workloads.
```

**The induced architecture** (near-necessity from C1+C2+C3+C5+C10): f64 default + bytecode-tier-driven typed-i64 promoted fast path. Φ lands Move 1 (f64 default); the typed-i64 fast path is Move 2 (separate pilot).

### I.3 Falsifiers

**Pred-φ.1** (bench_call_overhead preservation): post-Φ, bench_call_overhead `none` stays within +15% of current 71.2 ns baseline (target ≤82 ns). Falsifier: substantial regression (>15%); revisit the f64 calling-convention design or back the move out.

**Pred-φ.2** (bench_ic preservation): post-Φ, bench_ic `none` stays within +10% of current 81 ns baseline (target ≤89 ns). Same falsifier semantics.

**Pred-φ.3** (VTI revival): post-Φ, VTI=1 can be re-attempted without (P2.d) regression. The dispatcher's precheck reduces to tag-check only; inline tag-check at JIT prologue replaces it. Pred-vti.1's ≥5 ns reclaim becomes structurally reachable. Falsifier: VTI still slower than baseline post-Φ.

**Pred-φ.4** (correctness on non-integer Numbers): JIT now handles Math.PI / 0.5 / NaN / Infinity correctly (no silent truncation). diff-prod fixtures involving fractional Numbers pass under JIT. Falsifier: any fixture passing under interp + JIT-disabled but failing under default (which is JIT-on).

**Pred-φ.5** (no new (P2.c)): fuzz fixture covering integer + fractional + non-Number args produces byte-identical output across default / JIT-disabled / node. Falsifier: divergent output under any config.

**Pred-φ.6** (composition holds): TB+STUB on bench_ic post-Φ stays within ±10% of current 81 ns. The new f64 path composes constructively with TB's dispatcher bypass + STUB's IC fast path. Falsifier: composition regresses substantially.

## II. Apparatus

Φ is **an architectural substrate move at the JIT calling-convention tier**. It composes with:

- **Cranelift** owns the generic codegen; Φ changes the IR types passed (i64 → f64).
- **The bytecode alphabet** remains unchanged at the input (current Op set); Op::Add semantics shift from i64-add to f64-add (consistent with JS Number semantics).
- **Shape substrate (default-on)**: unaffected; JIT body still reads shape via inline fast-path through TB.
- **STUB IC fast-path (default-on)**: unaffected; the IC fast-path returns i64-as-from-Number-truncation today; post-Φ returns f64 directly. STUB's runtime_ic_fast_get extern signature changes from `-> i64` to `-> f64` (or stays i64 with the f64 reinterpreted via transmute).
- **TB closure-side metadata cache (default-on)**: unaffected by Φ in mechanism; the cached CompiledFn's call signature changes from `(i64) -> i64` to `(f64) -> f64`. TB's fast-path code at the dispatcher passes f64.

Per Doc 738 §II.e pillar-path: same JIT crate (`pilots/rusty-js-jit/derived/src/`); no new module needed — modifies existing translator.rs + interp.rs dispatch sites. Per §II.b: identifiers continue post-§A8.32 form.

## III. Methodology

Per the LeJIT-tier staged-validation discipline (Findings doc II.2 + standing rule 4; informed by TB-EXT 3b scope-analysis precedent):

1. **Φ-EXT 0** — Workstream founding (this seed + trajectory + scaffold).

2. **Φ-EXT 1** — Pre-implementation design doc. Enumerate the per-op IR-change deltas (which iadd → fadd; which load i64 → load f64; etc.); enumerate the per-extern signature changes (jit_getprop_with_ic, runtime_ic_fast_get, JitFn types); enumerate the dispatch-site changes (call_function under both standard + TB fast paths). Output: `docs/f64-design.md`.

3. **Φ-EXT 2** — Substrate-introduction (per Doc 729 §A8.13): JitFn signature change + dispatcher arg-passing change. No JIT-body IR change yet; the JIT body still does iadd but receives args as `*f as i64` via the new path. Substrate is in place; behavior unchanged.

4. **Φ-EXT 3** — Closure round: switch JIT translator to emit fadd/fsub/fmul for Op::Add/Sub/Mul. JIT body now does f64 arithmetic. Result reboxed as Value::Number(f64).

5. **Φ-EXT 4** — Composition re-bench. Composition matrix across {none, TB, STUB, TB+STUB} × {bench_call_overhead, bench_ic}. Pred-φ.1 + Pred-φ.2 + Pred-φ.6 disposition.

6. **Φ-EXT 5** — Consumer-route probe. diff-prod 42/42; cross-runtime-bench under default + JIT-disabled; correctness on fractional-Number fixtures (Pred-φ.4).

7. **Φ-EXT 6** — Fuzz probe per Doc 735 §X.h.c. Fixture covers integer + fractional + non-Number args; verifies (P2.c) absence per Pred-φ.5.

8. **Φ-EXT 7** — VTI re-attempt (Pred-φ.3). With f64 calling convention in place, VTI's inline tag-check is cheap (no integer-validity check needed). Measure VTI's revival reclaim; categorize per Doc 735 §X.h.b.

9. **Φ-EXT 8** — Default-on confirmation (Φ is default-on by construction — it's an architectural change, not a flag-gated experiment). If Φ-EXT 4-6 all hold, the substrate stands. If any falsify, back out + redesign.

## IV. Carve-outs and bounded scope

- **Typed-i64 fast path is NOT in Φ's scope.** That's Move 2 — a separate pilot at the bytecode tier (Doc 731 §XIII alphabet promotion). Φ removes the i64-only constraint; the i64 fast path returns when bytecode tier-1.5 IR lands.
- **Cap-passing semantics preserved by construction.** Φ doesn't touch cap surface (JIT body has no cap access).
- **Single-tier R1 preserved.** Φ is a calling-convention change to the existing single JIT tier; no second tier.
- **OptLevel::None preserved.** Cranelift configuration unchanged.
- **No new deopt sites at first cut.** Existing arith-overflow deopt sites become moot (f64 arith doesn't overflow in the i64 sense); they remain in code but unreachable. Cleanup deferred to forward optimization.
- **VTI revival is gated on Φ-EXT 7, not Φ first cut.** Φ-EXT 7 is a follow-on round; Φ ships first as a foundation.

## V. Standing artefacts

- `pilots/rusty-js-jit/f64-calling-convention/seed.md` (this file)
- `pilots/rusty-js-jit/f64-calling-convention/trajectory.md`
- `pilots/rusty-js-jit/f64-calling-convention/docs/f64-design.md` (Φ-EXT 1 output)
- `pilots/rusty-js-jit/f64-calling-convention/fixtures/` (Φ-EXT 6 fuzz fixture)
- Substrate modifications at `pilots/rusty-js-jit/derived/src/translator.rs` + `pilots/rusty-js-runtime/derived/src/interp.rs` + helper externs

## VI. Resume protocol

Read this seed, then trajectory.md tail. Then read LeJIT seed §I.4 (first-cut composition met) + findings.md (especially V.3 + V.6 + II.2 + standing rule 9). The induced-architecture analysis in §I.2 is the seed's central design claim; subsequent substrate moves should refer back to constraint C1-C10 as the framework.

## VII. Doc 738 §II conventions checklist

- Locale path: `pilots/rusty-js-jit/f64-calling-convention/` (§II.e nested under LeJIT).
- Filenames: snake_case for new fixtures; existing translator.rs / interp.rs unchanged in name.
- Function naming: post-§A8.32 receiver-discriminated form. New helpers: `unbox_arg_f64`, `runtime_ic_fast_get_f64` (if signature change requires renaming for clarity).
- No new env flag (Φ is architectural, default-on by construction per §III item 8).
- No `__` prefix for new identifiers (no engine-internal sentinels at this tier).
- Existing env flags (CRUFTLESS_LEJIT_STUB, CRUFTLESS_LEJIT_TB, CRUFTLESS_LEJIT_VTI) preserved.
