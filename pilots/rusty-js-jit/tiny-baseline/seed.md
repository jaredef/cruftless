# LeJIT-Τ (tiny-baseline) — Resume Vector / Seed

*(Internal name LeJIT-Τ per the LeJIT seed §I.2 item 5 pre-file. Third nested sibling under `pilots/rusty-js-jit/`, alongside LeJIT-Σ at `stub-emitter/` and LeJIT-Ψ at `value-tag-inline/`. The "Τ" (tau) reads as "tiny-baseline" — the Sparkplug-style fast-baseline tier that bypasses the Rust `call_function` dispatcher's per-invocation overhead. On-disk source code will land at `pilots/rusty-js-jit/derived/src/tiny_baseline.rs` within the parent LeJIT crate; the seed + trajectory at this coordinate hold the Pin-Art record.)*

**Locale tag**: `L.rusty-js-jit/tiny-baseline` (nested per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (TB-EXT 0)**. No code yet. Spawned per LeJIT seed §I.2 item 5 + Doc 735 §X.h.d saturation-as-escalation-signal triggered by VTI-EXT 3b's (P2.d) negative empirical finding, with keeper direction at 2026-05-23 06:35-local.

**Workstream**: a hand-rolled tiny-function fast-baseline that replaces the Rust `call_function` dispatcher for the hot small-function call path. Per LeJIT seed §I.2 item 5: *"Tiny-function compile latency. Cranelift's fixed compile-time overhead (regalloc + scheduling + isel) dominates for functions of ~20 instructions. A hand-rolled Sparkplug-style stack-machine-to-register baseline compiles in microseconds."* Today's empirical reading (VTI-EXT 1 + 3b) sharpens this: the dispatcher overhead is also the dominant single arm at the call-overhead tier — `call_function` accounts for ~95% of the 127 ns/iter bench. This pilot targets the dispatcher itself.

**Author**: 2026-05-23 session.
**Parent**: `pilots/rusty-js-jit/` (LeJIT).
**Siblings**:
- `pilots/rusty-js-jit/stub-emitter/` (LeJIT-Σ)
- `pilots/rusty-js-jit/value-tag-inline/` (LeJIT-Ψ)

**Composes with**:
- [LeJIT seed §I.2 item 5](../seed.md) — names tiny-fn fast-baseline as one of four hand-rolled regions.
- [LeJIT seed §I.3](../seed.md) — substrate-amortization cascade; this pilot is the named-as-largest single arm of the multiplicative composition (1.5-2× per the §I.3 table).
- [LeJIT enhancements log](../enhancements.md) — VTI-EXT 1's 127 ns call-overhead bench + VTI-EXT 3b's (P2.d) finding are the empirical anchors for spawning this pilot now.
- [Doc 729 §A8.13](../../../../corpus-master/corpus/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — substrate-amortization-cascade staging.
- [Doc 731 §VII R1–R8](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — single-tier baseline constraint (R1 preserved by construction: tiny-baseline is a sub-substrate, not a second tier).
- [Doc 735 §X.h](../../../../corpus-master/corpus/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — three-probe-levels + (P2) four-sub-case categorization; saturation-as-escalation-signal §X.h.d motivated this spawn after VTI-EXT 3b's (P2.d) reading.
- [Doc 736 §IX](../../../../corpus-master/corpus/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — capability-passing mode-aware dispatcher; any dispatcher refactor at this pilot must preserve the four-mode semantics under all enforcement levels.
- [Doc 737 §IV](../../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md) — locale-as-coordinate; third nested sibling materializing from a pre-file.
- [Doc 738 §II](../../../../corpus-master/corpus/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md) — source-tier convention space; `pilots/rusty-js-jit/derived/src/tiny_baseline.rs` pillar-path per §II.e.

## I. Telos

The dispatcher refactor's load-bearing claim is empirically demonstrated by VTI-EXT 1's bench decomposition:

| component | per-iter cost | targeted by |
|---|---:|---|
| IC GetPropOnObject dispatch | 72 ns | LeJIT-Σ (stub-emitter) |
| Shape fast-path savings | 72 ns absorbed | shapes substrate (already landed) |
| Dispatcher + arg-coerce + JIT id + rebox | ~127 ns | **LeJIT-Τ (this pilot)** + LeJIT-Ψ |
| JIT preamble arg-coerce specifically | ~5-15 ns | LeJIT-Ψ (this is the LeJIT-Ψ-only portion) |
| **Dispatcher specifically** | **~112-122 ns** (95% of 127) | **LeJIT-Τ** |

The dispatcher specifically (the Rust `call_function` closure-bound-this resolve + Vec<Value> allocation + Frame setup + JIT-cache lookup + deopt-TLS plumbing) is ~120 ns of overhead per call. LeJIT-Τ targets this directly.

### I.1 First-cut telos

A **Sparkplug-style fast-baseline call path** for JIT-compiled functions of bounded size (≤20 ops) where:

1. **The dispatcher's per-call work is amortized at JIT-compile time, not per-call.** The closure-bound-this cell pointer, the FunctionProto pointer, the JIT-cache slot pointer — all are resolved once at compile-time and inlined into the JIT'd function's call thunk. Per-call cost drops to the inline thunk's trampoline + the JIT body itself.

2. **The Vec<Value> argument allocation is eliminated for ≤2-arg paths.** The dispatcher currently allocates a Vec to pass args; the fast-baseline call thunk takes args via registers or stack slots directly, bypassing the Vec.

3. **The deopt-TLS plumbing is set up once per JIT-cache-hit, not per-call.** The CURRENT_RUNTIME / CURRENT_PROTO / CURRENT_DEOPT_SITES TLS slots are set+cleared per call today; for tiny-baseline functions, the thunk reads them from a per-function compile-time-resolved table.

4. **Correctness invariants preserved**: deopt still works (the thunk routes to the existing `take_last_deopt` mechanism); cap-dispatcher mode semantics still hold (per Doc 736 §IX.6, the four modes must be observable through this path); single-tier R1 (per Doc 731 §VII) preserved because tiny-baseline is a sub-substrate of the same JIT tier.

The first-cut closure criterion: a hand-built `function id(x) { return x; }` benched via `bench_call_overhead` drops from ~127 ns/iter (current) to ~50 ns/iter or better (composition target ~2.5× reclaim on the dispatcher arm), with all 38 JIT tests + 35 runtime tests GREEN and diff-prod 42/42 holding.

### I.2 Falsifiers

**Pred-tb.1**: cumulatively across TB-EXT 3b + 3c + (3d if needed), tiny-baseline reduces per-call overhead by ≥40 ns on `bench_call_overhead` (≥30% of the ~127 ns dispatcher cost). Falsifier: substrate-complete implementation across all three rounds fails to flip the bench by ≥40 ns. If true, the dispatcher's overhead is not in the named per-call work but in some other component the design did not identify (the unidentified gap from TB-EXT 2 §3 is structurally inaccessible from the per-call substrate axis).

*Per-round sub-targets* (added 2026-05-23 after TB-EXT 3b scope analysis — see trajectory.md): TB-EXT 3b first-cut bar is ≥20 ns reclaim as framework validation under approach (A) closure-side metadata caching; TB-EXT 3c bar is the additional 20+ ns reclaim under approach (B) deopt-restructure-to-arg-passing; TB-EXT 3d if needed is approach (C) native call thunk emission. VTI-EXT 3b's (P2.d) experience constrained the staging: the Rust optimizer's response to restructured calling conventions is the surprising mechanism that flipped VTI from +5-10 ns prediction to +18.9 ns regression. The 3b → 3c → 3d staging validates the framework empirically before committing higher-LOC restructuring.

**Pred-tb.2**: composition with shape (1.36×) + LeJIT-Σ (~1.3-1.5×) reaches the seed §I.3 multiplicative target (~3×) on `bench_ic` under (shape + LEJIT_STUB + LEJIT_TB). Falsifier: composition stays below 2.5×. If true, the seed §I.3 composition reading needs further refinement; possibly a fourth arm (e.g., GC interaction reduction) is required.

**Pred-tb.3**: tiny-baseline preserves all four cap-dispatcher modes (Doc 736 §IX) semantically. Falsifier: a mode-3 (sealed) capability test that passes under the standard dispatcher fails under tiny-baseline. If true, the thunk's compile-time pointer resolution is unsound under capability-passing.

**Pred-tb.4**: tiny-baseline's compile latency is below the standard Cranelift JIT for the same function. Per LeJIT seed §I.2 item 5's original claim, Sparkplug-style stack-machine-to-register baselines compile in microseconds. Falsifier: tiny-baseline compile takes ≥10× the standard JIT compile time.

**Pred-tb.5**: no (P2.c) illegal-speed bug per Doc 735 §X.h.b. The thunk's compile-time pointer resolution must produce identical observable results to the per-call resolution. Falsifier: a fuzz probe (TB-EXT 7+) that finds an input where tiny-baseline returns a result different from the standard dispatcher path.

## II. Apparatus

The tiny-baseline is **a hand-rolled call-thunk emitter at the JIT-function-entry boundary**, alongside Cranelift's function-body lowering. It composes with:

- **Cranelift** owns the function body's lowering (unchanged from the existing JIT path).
- **The standard dispatcher** remains the fallback for any function the tiny-baseline opts out of (size > threshold, mode > Mode-0, deopt active).
- **LeJIT-Σ + LeJIT-Ψ** compose multiplicatively per seed §I.3; the tiny-baseline's thunk must preserve the IC stub fast path (LeJIT-Σ) + the optional VTI calling convention (LeJIT-Ψ) when their env flags are set.

Per Doc 738 §II.e pillar-path: `pilots/rusty-js-jit/derived/src/tiny_baseline.rs`. Per §II.b: emitter function names follow post-§A8.32 receiver-discriminated form (no `_via` suffix since these are JIT-emitter functions). Examples: `emit_inline_call_thunk`, `resolve_compile_time_pointers`, `eligible_for_tiny_baseline`.

## III. Methodology

Per the LeJIT-tier methodology refined across StubE-EXT 0-5b + VTI-EXT 0-3b:

1. **TB-EXT 1 — Pre-emission bench probe**: extend `bench_call_overhead` to exercise multiple function shapes (1-arg, 2-arg, with-locals, with-branches). Establish the per-shape baseline so post-implementation comparison is shape-controlled.

2. **TB-EXT 2 — Apparatus design**: dispatcher decomposition audit. Read `call_function`'s ~120 ns and partition into named cost components (closure-bound-this resolve, Vec allocation, Frame setup, JIT-cache lookup, deopt-TLS plumbing). Output: `docs/dispatcher-decomposition.md`. The target list for the tiny-baseline thunk is the named-cost partition.

3. **TB-EXT 3a — Substrate-introduction**: per Doc 729 §A8.13. Compile-time pointer resolution table (per-JIT-function metadata holding closure_v Rc, FunctionProto Rc, JIT-cache slot pointer). No thunk emission yet; the table is the apparatus.

4. **TB-EXT 3b — Closure round (approach A: closure-side metadata caching)**: add `Cell<Option<*const TinyBaselineMetadata>>` to `ClosureInternals`; populate on first JIT-hit. Dispatcher under `CRUFTLESS_LEJIT_TB=1` reads the cell; if Some + eligible, fast-paths around the `jit_cache.get` HashMap lookup + the multi-condition AND + jit_compatible_arg per-arg match. Keeps TLS sets/clears + InternalKind match + Vec-arg cost (deferred to 3c). LOC estimate: ~80-120. Reclaim target: ≥20 ns (framework validation; HashMap absorption + match-arm simplification). Behind the env flag; default OFF.

   *(Staging refinement added 2026-05-23 — see trajectory TB-EXT 3b scope-analysis entry.)* Per VTI-EXT 3b's (P2.d) lesson, the Rust optimizer's behavior under restructured calling conventions is unpredictable; the 3b → 3c → 3d staging validates the framework empirically at each step before committing higher-LOC restructuring. If 3b shows <10 ns reclaim or regresses, the tiny-baseline pilot is (P2.d) at first cut and the gap is structurally inaccessible from the per-call substrate axis (consider AHash for the JIT cache as alternative, or escalate to a different substrate per Doc 735 §X.h.d).

5. **TB-EXT 3c — Closure round (approach B: restructured deopt to arg-passing)**: gated on TB-EXT 3b showing ≥20 ns reclaim. Remove the `set_current_*` / `clear_current_*` TLS pattern; thread metadata pointer through extern callbacks. Eliminates the ~20-40 ns TLS gap component identified in TB-EXT 2 §3 (d). LOC estimate: ~250-400 across deopt.rs + 3 extern fn signatures + thunk callers. Additional reclaim target: ≥20 ns (cumulative ≥40 ns vs TB-EXT 1 baseline — meets Pred-tb.1).

6. **TB-EXT 3d — Closure round (approach C: native call thunk)**: gated on TB-EXT 3c result + keeper authorization. Emit aarch64 directly that bypasses the Rust dispatcher entirely (Sparkplug-style per the seed's original framing). LOC estimate: ~400-600; first hand-rolled native emission in the engagement. Additional reclaim target: ≥20 ns (cumulative ≥60 ns vs TB-EXT 1 baseline). Only pursued if the cumulative reclaim from 3b + 3c falls short of Pred-tb.1's full target.

7. **TB-EXT 4 — Bench measurement**: re-bench `bench_call_overhead` under (a) no flags, (b) TB=1 only, (c) TB=1 + STUB=1, (d) TB=1 + STUB=1 + VTI=1. Run after each of 3b/3c/3d closes. Three-probe-levels per Doc 735 §X.h.c starts here.

8. **TB-EXT 5 — Consumer-route probe**: diff-prod under TB=1; expected NEUTRAL since diff-prod fixtures don't exercise JIT hot paths. Surface any unexpected regressions.

9. **TB-EXT 6 — Variance characterization**: multi-run bench (≥20 runs per configuration) to bound the variance band on TB-EXT 4's reading + retroactively on VTI-EXT 3a/3b.

10. **TB-EXT 7 — Fuzz probe**: random call patterns exercising deopt + capability-mode boundaries. Goal: 2000-fixture run, 0 divergent results per Doc 735 §X.h.c.

11. **TB-EXT 8 — Default-on flip**: if TB-EXT 4 shows (P2.a) strict-win, TB-EXT 7 shows 0/2000 divergent, and the LeJIT seed §I.3 composition target is reached — flip default to ON. Otherwise document the (P2) sub-case categorization and leave behind the flag.

## IV. Carve-outs and bounded scope

- **Cap-passing semantics preserved by construction**. Per Doc 736 §IX.6: the thunk reads the dispatcher's mode flag at thunk-entry; under Mode-1+ the thunk routes to the standard dispatcher (does not bypass capability checks).
- **No tier hierarchy**. Per Doc 731 §VII R1: tiny-baseline is a sub-substrate of the same JIT tier; deopt path is the interpreter (unchanged); no second compiler tier introduced.
- **Bounded function size**. Only functions of ≤20 ops are eligible. Larger functions stay on the standard dispatcher.
- **Mode-0 only at first cut**. Modes 1/2/3 (Doc 736 §IX) route to the standard dispatcher.

## V. Resume protocol

Read in order: this seed, then trajectory.md, then the parent LeJIT seed.md §I.2 + §I.3, then the LeJIT enhancements.md log entries for VTI-EXT 1 + VTI-EXT 3b. The latter two name the empirical anchors that motivated the spawn.

First substrate move: TB-EXT 1's bench probe extension — extend `bench_call_overhead` to also bench `id2(x, y) { return x + y; }` and `id_locals(x) { let y = x; return y; }` for shape-controlled baselines.

## VI. Doc 738 §II conventions checklist

- Crate path: `pilots/rusty-js-jit/derived/src/tiny_baseline.rs` per §II.e.
- Module name: `tiny_baseline` (snake_case, no underscores leading).
- Function naming: post-§A8.32 form, no `_via` (JIT-emitter, not Runtime-dispatching). Examples: `emit_call_thunk_aarch64`, `resolve_jit_cache_slot`, `eligible_for_tiny_baseline`.
- Env flag: `CRUFTLESS_LEJIT_TB=1` (mirrors `CRUFTLESS_LEJIT_STUB` + `CRUFTLESS_LEJIT_VTI` precedent).
- No `__` prefix (no engine-internal sentinels at this tier; the thunk emits plain JIT code).
- No `set_own_*` interaction (no JS-observable property installation).
- No `register_*` interaction (no JS-globally visible binding).
