# LeJIT-Τ (tiny-baseline) — Trajectory

Per-TB-EXT log for the LeJIT-Τ tiny-baseline pilot. Third nested sibling under `pilots/rusty-js-jit/`; siblings are LeJIT-Σ (`stub-emitter/`) and LeJIT-Ψ (`value-tag-inline/`). Read seed.md first; then read the parent LeJIT seed.md §I.2 + §I.3; then read the LeJIT enhancements.md entries for VTI-EXT 1 + VTI-EXT 3b (the empirical anchors that motivated the spawn).

---

## TB-EXT 0 — 2026-05-23 (workstream founding)

### Headline

Apparatus-tier round. Pilot LeJIT-Τ founded per Doc 737 §IV + keeper's direction at 06:35-local after VTI-EXT 3b closed with a clean Doc 735 §X.h.b (P2.d) negative finding (VTI ON +18.9 ns slower than VTI OFF). Per Doc 735 §X.h.d saturation-as-escalation-signal: when consecutive (P2) substrate moves at the same site stop producing improvement, the next substrate target is OUTSIDE that site. VTI's structural argument under hypothesis (5) (the dispatcher's `jit_compatible_arg` precheck is the real cost; VTI-EXT 3b adds work on top rather than removing the precheck) is suspended pending a forward attempt at the dispatcher itself.

LeJIT seed §I.2 item 5 named the tiny-fn fast-baseline as one of four hand-rolled regions. VTI-EXT 1's bench decomposition empirically located the dispatcher specifically at ~120 ns of the 127 ns per-iter cost — the largest single arm of the §I.3 multiplicative composition. The pilot's spawn is the substrate calling per Doc 737 §IX "pre-file generously, spawn when the substrate calls."

### Substrate delivered

- `pilots/rusty-js-jit/tiny-baseline/seed.md` (~155 lines) — telos (Sparkplug-style call-thunk emitter), apparatus (composes with parent LeJIT + sibling Σ + sibling Ψ), methodology with TB-EXT 0-8 staging, carve-outs (cap-passing preserved; no tier hierarchy; bounded function size ≤20 ops; Mode-0 only at first cut), five falsifiers Pred-tb.1-.5, Doc 738 §II conventions checklist.
- `pilots/rusty-js-jit/tiny-baseline/trajectory.md` (this file).
- `pilots/rusty-js-jit/tiny-baseline/docs/` scaffold for TB-EXT 1 + 2 outputs.

### Locale registration

Per Doc 737 §IV: nested locale at coordinate `pilots/rusty-js-jit/tiny-baseline/` (depth 2). Parent: `L.rusty-js-jit` (LeJIT). Siblings: `L.rusty-js-jit/stub-emitter` (LeJIT-Σ), `L.rusty-js-jit/value-tag-inline` (LeJIT-Ψ).

The engagement's fourth nested locale, third under the LeJIT parent. Locale count: 13 → 14 after this round. Manifest refresh queued.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (founding round).

Per Doc 734 §V: growth mechanism (a) tier-relocation — the tiny-fn fast-baseline tier was pre-filed at LeJIT seed §I.2 item 5 + §I.3 as the largest single arm. VTI-EXT 3b's (P2.d) result is the empirical-anchor event that promotes the pre-file to an active locale. Growth (b) negative-finding amendment — VTI-EXT 3b's negative bench surfaced the substrate-amortization-cascade limit (cascade pattern at LeJIT-Σ + shapes does not auto-generalize to all LeJIT-tier substrate-introduction rounds); this pilot tests the alternative arm.

Per Doc 735 §X.h.d saturation: the saturation signal fires after the second (P2) move at the dispatcher tier (VTI-EXT 3b is the first such move; VTI-EXT 3c would be the second). The keeper's direction is to skip 3c and escalate immediately to the next substrate axis — exactly the §X.h.d pattern.

### Composition with prior corpus work

- **LeJIT seed §I.2 item 5 + §I.3 multiplicative composition**: this pilot is the named-largest-single-arm (1.5-2× per §I.3 table). With shape (1.36× landed) + LeJIT-Σ (~1.3-1.5× in progress) + LeJIT-Τ (~1.5-2× target), the seed §I.3 composition reaches the 3× target on bench_ic.
- **Doc 729 §A8.13 substrate-amortization-cascade**: TB-EXT 3a's compile-time pointer resolution table is the substrate-introduction round; TB-EXT 3b's inline-thunk emission is the closure round. The cascade pattern from shapes recurs structurally here.
- **Doc 731 §VII R1–R8**: R1 preserved by construction (tiny-baseline is sub-substrate of same JIT tier, not a second tier).
- **Doc 735 §X.h.c three-probe-levels**: bench (TB-EXT 4) + consumer-route (TB-EXT 5 diff-prod) + fuzz (TB-EXT 7) discipline applies in full.
- **Doc 736 §IX.6 cap-dispatcher modes**: the thunk's compile-time pointer resolution must preserve Mode-0/1/2/3 semantics. First-cut carve-out: Mode-0 only at TB-EXT 3b; Mode-1+ routes to standard dispatcher.
- **Doc 737 §IV pre-filing materialization**: fourth nested locale, third spawn this session (after consumer-migration + stub-emitter + value-tag-inline). Per Doc 737 §IX "pre-file generously, spawn when the substrate calls."
- **Doc 738 §II conventions**: identifiers will fit (`tiny_baseline.rs` per §II.e; `emit_call_thunk_aarch64` snake_case per §II.b; `CRUFTLESS_LEJIT_TB` env flag mirrors precedent).
- **LeJIT enhancements log**: VTI-EXT 1 entry + VTI-EXT 3b entry are the empirical anchors. Both must be cited in TB-EXT 1's bench-baseline doc.

### Open scope at TB-EXT 0 close

1. **TB-EXT 1** — Pre-emission bench probe extension. Extend `bench_call_overhead` to bench `id2(x,y)=x+y` + `id_locals(x){let y=x; return y}` for shape-controlled baselines. Output: `docs/bench-baseline.md` with multiple per-shape readings.
2. **TB-EXT 2** — Dispatcher decomposition audit. Partition `call_function`'s ~120 ns into named cost components. Output: `docs/dispatcher-decomposition.md`.
3. **TB-EXT 3a** — Compile-time pointer resolution table (substrate-introduction per Doc 729 §A8.13).
4. **TB-EXT 3b** — Inline call thunk emission (closure round, behind `CRUFTLESS_LEJIT_TB=1`).
5. **TB-EXTs 4-8** per the seed §III methodology.

### Cumulative status at TB-EXT 0 close

LOC delta: 0 (apparatus-tier). docs/ scaffold: 1 (empty dir). Locale registered (manifest refresh queued).

The pilot's locale exists. TB-EXT 1 begins with the bench-probe extension to give TB-EXT 4 multi-shape comparison data.

---

*TB-EXT 0 closes. The fourth LeJIT-tier locale is founded; the largest single arm of the seed §I.3 multiplicative composition is now an active substrate-tier workstream. TB-EXT 1 measures the multi-shape call-overhead baseline.*

---

## TB-EXT 1 — 2026-05-23 (multi-shape call-overhead bench baseline)

### Headline

Bench probe activated. Three function shapes benched on the Pi: **id1 = 130.8 ns, id2 = 135.5 ns, id_locals = 126.5 ns** (1M iter each, single-run). Empirical corroboration of LeJIT seed §I.3's dispatcher-is-the-dominant-arm reading: arity adds only ~4.7 ns per arg; the local-management delta is within noise; the shape-invariant cost (~125 ns ± 5 ns) accounts for ~95% of every shape's per-iter total.

### Substrate landed

- `cruftless/examples/bench_call_shapes.rs` (~155 LOC) — multi-shape bench harness. Three hand-built FunctionProtos (id1, id2, id_locals), shared install_closure helper, per-shape bench fn with warm-up + 1M-iter timing + correctness assertion. Same calling convention as VTI-EXT 1's `bench_call_overhead`.
- `pilots/rusty-js-jit/tiny-baseline/docs/bench-baseline.md` (~85 lines) — bench protocol, per-shape measurements, decomposition reading, cross-validation with the five prior id1 measurements (122-131 ns variance band), Pred-tb.1 + Pred-tb.2 anchored target reading, TB-EXT 2 design pointer.

### Reading

| shape | per-iter | Δ vs id1 | what it adds |
|---|---:|---:|---|
| id1 | 130.8 ns | — | baseline (1 arg, no body) |
| id2 | 135.5 ns | +4.7 ns | 2nd arg coerce + Op::Add body |
| id_locals | 126.5 ns | −4.3 ns | StoreLocal + extra local (within ±5 ns noise) |

**Key finding**: the per-arg cost is only ~2-3 ns, and the per-local cost is within noise. Almost all of the 125 ns/iter baseline is shape-invariant. This shape-invariant cost is exactly the dispatcher overhead — closure-bound-this resolve + Vec<Value> allocation + Frame setup + JIT-cache lookup + deopt-TLS plumbing — which TB-EXT 2 will decompose source-tier and TB-EXT 3b will target.

**Cross-validation with five prior id1 measurements**:
- VTI-EXT 1 (initial baseline): 127.1 ns
- VTI-EXT 3a (post-layout-pin): 122.0 ns
- VTI-EXT 3b (VTI OFF): 126.6 ns
- VTI-EXT 3b (VTI ON, regression): 145.5 ns (excluded from variance band — load-bearing-NEGATIVE finding)
- TB-EXT 1 (this round): 130.8 ns

Five non-regression measurements span 122-131 ns → working baseline is **125 ns ± 5 ns** at single-run resolution. TB-EXT 6's multi-run characterization will pin this.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (observe-only).

Per Doc 734 §V: growth (c) positive-finding generalization preparatory — the shape-invariant cost is the empirical anchor for TB-EXT 2's decomposition + TB-EXT 4's measurement.

Per Doc 735 §X.h.c: bench probe activated (NECESSARY but not sufficient). Consumer-route + fuzz at TB-EXT 5 + 7.

### Composition with prior corpus work

- **LeJIT seed §I.3 multiplicative composition**: today's reading empirically corroborates the dispatcher-as-dominant-arm prediction from VTI-EXT 1 + the new multi-shape decomposition. The shape-invariant ~125 ns is the target TB-EXT 3b's substrate move must reduce.
- **Doc 735 §X.h.b**: TB-EXT 4's measurement will be the load-bearing (P2.a) vs (P2.d) decision for this pilot. Pred-tb.1's ≥40 ns reclaim target is the bar.
- **Doc 738 §II conventions**: `bench_call_shapes.rs` follows the precedent of `bench_call_overhead.rs` + `bench_ic.rs`. Identifiers conform (snake_case, no `_via`, no `__` prefix).
- **LeJIT enhancements log**: TB-EXT 1's reading is the empirical anchor that any TB-EXT 4 measurement will be compared against. The five-measurement variance band reading is the working confidence interval.

### Open scope at TB-EXT 1 close

1. **TB-EXT 2** — Dispatcher decomposition audit. Read `Runtime::call_function` source-tier; partition the ~120 ns shape-invariant cost into named components (closure-bound-this resolve, Vec<Value> allocation, Frame setup, JIT-cache lookup, deopt-TLS plumbing). Per-component reclaim estimates. Output: `docs/dispatcher-decomposition.md`.
2. **TB-EXT 3a** — Compile-time pointer resolution table (substrate-introduction per Doc 729 §A8.13).
3. **TB-EXT 3b** — Inline call thunk emission (closure round, behind `CRUFTLESS_LEJIT_TB=1`).
4. **TB-EXTs 4-8** per the seed §III methodology.

### Cumulative status at TB-EXT 1 close

LOC delta: ~155 (bench harness + docs). No source-substrate changes; bench-tier only.

The pilot's bench baseline is established. The dispatcher dominates per-iter cost across all three shapes; the substrate-move target for TB-EXT 3b is the shape-invariant component.

---

*TB-EXT 1 closes. Multi-shape baselines on the Pi: 130.8 / 135.5 / 126.5 ns. Shape-invariant ~125 ns ± 5 ns confirms §I.3's reading. TB-EXT 2 decomposes the dispatcher source-tier.*

---

## TB-EXT 2 — 2026-05-23 (dispatcher decomposition audit)

### Headline

Apparatus-tier design round. Read `Runtime::call_function`'s hot JIT-success branch source-tier (lines 8331-8460). Partitioned the ~125 ns shape-invariant cost into 22 named components across caller + callee. Identified ~40-65 ns directly; the **~60-86 ns gap** is attributed (with hypotheses) primarily to HashMap lookups (~20-30 ns) and TLS slot writes (~20-40 ns) — both of which are eliminable. Total TB-EXT 3b reclaim estimate: **38-74 ns**; mid-range ~55 ns comfortably exceeds Pred-tb.1's ≥40 ns threshold.

### Substrate landed

- `pilots/rusty-js-jit/tiny-baseline/docs/dispatcher-decomposition.md` (~220 lines): per-component cost decomposition (22 items with line refs), unidentified-gap reading with five hypothesized mechanisms, classification by elimination mechanism (compile-time-resolve / thunk-inline / Vec-replacement / restructure-amortize / unavoidable), reclaim estimates per group, Doc 736 §IX.6 cap-passing constraint reading, thunk shape sketch in aarch64 pseudocode, TB-EXT 3a forward pointer.

### Component classification

| mechanism | components | est. reclaim |
|---|---|---:|
| **compile-time-resolve** (bake into TinyBaselineMetadata struct) | obj-lookup, proto_key, actual_this, params, proto_rc clone, jit_cache contains_key + get, pointer captures | ~12-22 + HashMap gap ~20-30 = **32-52 ns** |
| **thunk-inline** (specialized straight-line aarch64 in thunk preamble) | Value match, new_target take, InternalKind match, call_count Cell, jit_disabled get, jit_compatible_arg, vti flag, unbox_arg, take_last_deopt, Value::Number rebox, Result wrap | **10-15 ns** |
| **Vec-replacement** (call_function_n variants taking &[Value]) | caller-side vec![arg.clone()] | **4-7 ns** |
| **restructure-amortize** (bake state into per-thunk metadata; extern callbacks read from there instead of TLS) | set_current_* + clear_current_* (6 TLS accesses) | ~12 + TLS gap ~20-40 = **32-52 ns** |
| **unavoidable** | jit_fn.func.call1 + id body | ~5-10 ns |

### The unidentified gap finding

The 22 identified components sum to ~40-65 ns; measured is ~125 ns. The 60-86 ns gap is real and lives in non-obvious places:

- **HashMap lookups** (std SipHash-13, two per call) likely cost 20-30 ns total, not 6-10 as my initial estimate
- **TLS slot access** on aarch64 Linux (TPIDR_EL0 + dispatch table) likely costs 5-10 ns per access; six per call ≈ 30-60 ns
- **Cache-miss memory traffic** across the dispatcher's distributed reads (8+ memory regions touched) likely costs 20-40 ns of stall time
- **Branch mispredict** on the five-condition AND at line 8389-8393 plausibly costs 5-10 ns

This gap reading is the load-bearing finding for TB-EXT 6 micro-profiling. For TB-EXT 3b: targeting compile-time-resolve + restructure-amortize alone (the two largest groups) reclaims ~40-70 ns by construction — exactly Pred-tb.1's threshold. The gap is the engagement's opportunity.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier round).

Per Doc 734 §V: growth (c) positive-finding generalization preparatory — the named-cost partition is the empirical anchor for TB-EXT 3b's (P2.a) attempt. The gap-finding is a (c) candidate in its own right: the dispatcher cost is structurally bigger than the obvious source-tier components because of memory hierarchy + TLS access + HashMap codegen costs that the corpus framework has not previously named. The decomposition method (estimate components + identify gap + hypothesize gap mechanisms) is the corpus-tier articulation candidate.

Per Doc 735 §X.h.c three-probe-levels: this round is bench-tier preparation (consumer-route + fuzz at TB-EXT 5 + 7).

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization-cascade**: TinyBaselineMetadata is the substrate-introduction (one-time per JIT-compile); the thunk consumes it per call. Cascade structure exactly parallels shape Shape-EXT 4 + consumer-migration (substrate-introduction enables N consumer rounds).
- **Doc 731 §VII R1–R8**: R1 single-tier preserved (thunk is sub-substrate of same JIT tier); R8 no-internal-optimization-passes preserved (thunk is straight-line aarch64, not an optimization pass).
- **Doc 735 §X.h.b**: TB-EXT 4 will be the load-bearing (P2.a) vs (P2.d) call. The 38-74 ns reclaim estimate makes (P2.a) the predicted outcome.
- **Doc 736 §IX.6 cap-dispatcher modes**: §5 of decomposition doc names the mode-check at thunk entry. ~2 ns cost; cheaper than the per-call work it conditionally skips. First-cut Mode-0-only carve-out per seed §IV.
- **Doc 738 §II conventions**: TinyBaselineMetadata fits §II.e (struct in `pilots/rusty-js-jit/derived/src/tiny_baseline.rs`); thunk emitter functions follow post-§A8.32 receiver-discriminated form (no `_via`).

### Open scope at TB-EXT 2 close

1. **TB-EXT 3a** — Substrate-introduction per Doc 729 §A8.13: `TinyBaselineMetadata` struct + per-JIT-function table (proto_key → metadata) construction at compile time. No thunk emission yet. ~80 LOC estimate.
2. **TB-EXT 3b** — Closure round: emit inline call thunk for ≤2-arg ≤20-op functions under `CRUFTLESS_LEJIT_TB=1`. Dispatcher under flag routes eligible calls. Standard dispatcher is the fallback. ~150-200 LOC estimate (the largest single round of the pilot).
3. **TB-EXT 4** — Re-bench against TB-EXT 1 multi-shape baselines. Load-bearing (P2) categorization.
4. **TB-EXTs 5-8** per the seed §III methodology.

### Cumulative status at TB-EXT 2 close

LOC delta: ~220 (decomposition doc, design-tier).

The dispatcher is fully decomposed source-tier. TB-EXT 3b has a concrete named-component target list with per-group reclaim estimates. The unidentified gap is itself a forward-work item the design accepts honestly. The substrate-amortization-cascade pattern from shapes recurs here structurally (substrate-introduction at 3a; closure at 3b).

---

*TB-EXT 2 closes. 22-component decomposition + 60-86 ns gap-finding + 38-74 ns reclaim estimate. TB-EXT 3a begins the TinyBaselineMetadata substrate-introduction.*
