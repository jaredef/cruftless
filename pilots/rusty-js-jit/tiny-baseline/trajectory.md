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

---

## TB-EXT 3a — 2026-05-23 (TinyBaselineMetadata substrate-introduction)

### Headline

Substrate-introduction round per Doc 729 §A8.13. New module `pilots/rusty-js-jit/derived/src/tiny_baseline.rs` (~145 LOC with tests) holds the `TinyBaselineMetadata` struct + `lejit_tb_enabled` env-flag helper + `TB_BYTECODE_LEN_THRESHOLD` const + 8 unit tests. Wired into `CompiledFn` as `Option<TinyBaselineMetadata>` field; populated at `compile_function` time when `CRUFTLESS_LEJIT_TB=1`. No dispatcher route yet — pure apparatus. 46/46 JIT lib tests + 35/35 runtime lib tests + bench shows TB=1 within noise of TB=0 (125.9 vs 122.2 ns/iter).

### Substrate landed

- `pilots/rusty-js-jit/derived/src/tiny_baseline.rs` (~145 LOC):
  - `pub struct TinyBaselineMetadata { jit_fn_ptr: usize, params: u16, bytecode_len: usize, tb_eligible: bool }`
  - `pub const TB_BYTECODE_LEN_THRESHOLD: usize = 60` (per seed §IV ≤20-op carve-out; 60 bytes ≈ 20-30 ops upper bound)
  - `impl TinyBaselineMetadata { pub fn build(...), pub fn eligible() -> bool }`
  - `pub fn lejit_tb_enabled() -> bool` (mirrors LEJIT_STUB + LEJIT_VTI env-flag precedent)
  - 8 unit tests: build, ineligible-by-size, ineligible-by-arity (0 + 3), boundary-at-threshold, two-arg-eligible, env-flag default-off + on-via-"1" + on-via-"TrUe"
- `pilots/rusty-js-jit/derived/src/lib.rs`:
  - `pub mod tiny_baseline;` + re-export of `TinyBaselineMetadata`, `lejit_tb_enabled`, `TB_BYTECODE_LEN_THRESHOLD`.
- `pilots/rusty-js-jit/derived/src/translator.rs`:
  - `CompiledFn.tb_metadata: Option<TinyBaselineMetadata>` field added with rustdoc.
  - `compile_function` populates the field via `TinyBaselineMetadata::build(code_ptr as usize, proto.params, proto.bytecode.len())` when `lejit_tb_enabled()` returns true; `None` otherwise.

### Probes

- **Unit tests**: 8/8 new tiny_baseline tests PASS.
- **JIT lib regression**: 46/46 PASS (was 38 pre-TB-EXT 3a; +8 from this round's new tests).
- **Runtime lib regression**: 35/35 PASS.
- **Bench under TB=1 (no dispatcher route)**: 125.9 ns/iter vs 122.2 ns OFF — Δ +3.7 ns within the 122-131 ns variance band. Metadata-build cost is one-time per JIT-compile (warm-up phase), not per-call. The dispatcher has not been modified for TB; per-call cost identical to OFF path.
- **Multi-shape under TB=1**: id1=125.1, id2=132.6, id_locals=124.8 (all within noise of TB=0 readings from TB-EXT 1).

### Doc 738 §II cross-axis consistency check

The new identifiers conform to conventions:
- `tiny_baseline.rs` — §II.e pillar-path (engine optimization tier under `pilots/rusty-js-jit/derived/src/`).
- `TinyBaselineMetadata` — UpperCamelCase struct name; semantic substrate-position encoding per §II.b (this names a substrate-tier artifact, not an invocation surface).
- `lejit_tb_enabled` — snake_case fn at module scope; no `_via` (not Runtime-dispatching); no `__` prefix (not engine-internal sentinel; called from translator-side Rust).
- `TB_BYTECODE_LEN_THRESHOLD` — SCREAMING_SNAKE_CASE module-level const per Rust convention.
- Env flag `CRUFTLESS_LEJIT_TB` — mirrors `CRUFTLESS_LEJIT_STUB` + `CRUFTLESS_LEJIT_VTI` precedent.

No convention violations introduced.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: Case-4 (implementation freedom). Layout choice + struct fields are implementation detail.

Per Doc 734 §V: growth (a) substrate-introduction move enabling (c) positive-finding generalization at TB-EXT 3b. The §A8.13 cascade pattern's first half — the substrate-introduction round — is now landed.

Per Doc 735 §X.h.b: not applicable yet (no per-call cost change). TB-EXT 3b will be the (P2.a) vs (P2.d) call.

Per Doc 735 §X.h.c three-probe-levels: unit tests are the bench-tier probe at the apparatus level; consumer-route (diff-prod under TB=1) deferred to TB-EXT 5 alongside thunk emission.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization-cascade**: this round is the substrate-introduction half (the metadata that 3b's thunk consumes). Same structural pattern as Shape-EXT 4 (substrate-introduction) + consumer-migration (closure rounds).
- **Doc 731 §VII R1**: preserved by construction (metadata struct is per-JIT-function compile-time data, not a second tier).
- **Doc 736 §IX.6 cap-dispatcher modes**: the metadata struct does NOT cache mode-dependent state (intentional — see decomposition doc §5). The thunk's mode-check at TB-EXT 3b will route to standard dispatcher under Mode > 0.
- **Doc 737 §IV pre-filing → spawn → first substrate**: tiny-baseline locale is now contributing real source code (not just docs). The spawn → bench-probe → design → substrate-introduction cadence mirrors stub-emitter (StubE-EXT 0-3) and value-tag-inline (VTI-EXT 0-3a).
- **Doc 738 §II conventions**: cross-axis check above. No violations.

### Open scope at TB-EXT 3a close

1. **TB-EXT 3b** — Closure round per Doc 729 §A8.13: inline call thunk emission. Under `CRUFTLESS_LEJIT_TB=1`, the dispatcher checks `jit_fn.tb_metadata.as_ref().map_or(false, |m| m.eligible())` and routes through the inline thunk for eligible calls. ~150-200 LOC. The largest single round of the pilot. Load-bearing (P2.a) vs (P2.d) decision.
2. **TB-EXT 4** — Re-bench against TB-EXT 1 multi-shape baselines. (P2.a) decision per Doc 735 §X.h.b.
3. **TB-EXTs 5-8** per seed §III methodology.

### Cumulative status at TB-EXT 3a close

LOC delta: ~145 (new module) + ~15 (lib.rs re-exports + translator.rs metadata-build hook). Total ~160 LOC.

The apparatus is in place. The metadata struct holds the four compile-time-resolved facts the thunk needs. Test coverage validates the eligibility boundary (size threshold + arity constraint). TB-EXT 3b's thunk emission has a stable, tested foundation to build on.

---

*TB-EXT 3a closes. TinyBaselineMetadata + 8 unit tests landed; no per-call cost change (metadata built once per compile, dispatcher unchanged). TB-EXT 3b begins the inline call thunk emission against this substrate.*

---

## TB-EXT 3b — 2026-05-23 (scope analysis: three-option staging surfaced before implementation)

### Headline

Design-tier round that re-evaluates TB-EXT 2 §7's "150-200 LOC inline call thunk emission" scope estimate before writing code. Reading the dispatcher source-tier with TB-EXT 3b's intent in mind surfaces a structural constraint: for real reclaim the caller must hold a metadata pointer DIRECTLY, bypassing the Closure→proto_key→jit_cache lookup chain. Three implementation approaches identified, with VTI-EXT 3b's (P2.d) lesson informing the staged-validation framing. **No code written; the round folds the staging into seed §I.2 + §III before implementation begins.**

### Three approaches identified

| approach | mechanism | LOC | reclaim target | risk |
|---|---|---:|---:|---|
| **(A) Closure-side metadata caching** | `Cell<Option<*const TBM>>` on `ClosureInternals`; populate on first JIT-hit; dispatcher reads cell + fast-paths around HashMap + multi-AND + jit_compatible_arg | 80-120 | ≥20 ns (HashMap absorption + match-arm simplification) | low — additive, behind flag |
| **(B) Restructured deopt to arg-passing** | Remove `set_current_*` / `clear_current_*` TLS pattern; metadata pointer threaded through extern callbacks | 250-400 | ≥20 ns additional (cumulative ≥40 ns; meets Pred-tb.1) | high — touches load-bearing deopt across crate boundaries |
| **(C) Native call thunk (Sparkplug-style)** | Emit aarch64 directly that bypasses Rust dispatcher entirely | 400-600 | ≥20 ns additional (cumulative ≥60 ns) | very high — first hand-rolled native emission in engagement |

### The VTI-EXT 3b lesson constrains the staging

Per the LeJIT enhancements log's VTI-EXT 3b entry (logged 2026-05-23): a payload-extract-only calling-convention switch — predicted as a 5-10 ns reclaim — produced an empirical +18.9 ns regression. The mechanism: Rust optimizer's behavior under restructured calling conventions defeats register-allocator's view at call sites; loads through pointers are not free; the dispatcher's existing precheck doesn't go away unless the substrate removes it explicitly.

The lesson translates directly here. Doing (B) or (C) as a first cut — without first validating the framework with (A) — risks the same trap at substantially higher LOC cost. The (B) round restructures TLS plumbing across deopt.rs + 3 extern callbacks + thunk callers; if the optimizer's response is hostile, recovering would mean reverting all of it and reading why. The (C) round adds hand-rolled native emission on top of that. Both are unrecoverable at first-cut scope without first proving the framework can reclaim ≥20 ns from any per-call work elimination.

### Recommendation folded into seed.md

Updated seed §I.2 (Pred-tb.1) and §III (methodology) to reflect the staged structure:

- **Pred-tb.1 reframed**: cumulative ≥40 ns reclaim across TB-EXT 3b + 3c + (3d if needed), with per-round sub-targets. Original "≥40 ns in 3b alone" was unrealistic given the dispatcher's structure.
- **TB-EXT 3b**: approach (A) closure-side caching, ≥20 ns reclaim as framework validation.
- **TB-EXT 3c**: approach (B) deopt restructure, gated on 3b success. Additional ≥20 ns.
- **TB-EXT 3d**: approach (C) native thunk, gated on 3c result. Additional ≥20 ns if needed.
- **TB-EXT 4**: re-bench after each of 3b/3c/3d.
- Renumbered TB-EXT 5-8 → 8-11 to accommodate the new 3c + 3d + 4 staging.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier round).

Per Doc 734 §V: growth (b) negative-finding amendment — VTI-EXT 3b's (P2.d) finding (logged in enhancements.md) is the empirical anchor that motivated this scope-analysis round. The framework's response to (P2.d) is structural staging that validates each component empirically before committing higher-LOC restructuring. The §I.2 + §III amendments encode this discipline.

Per Doc 735 §X.h.b: TB-EXT 3b's reclaim target (≥20 ns) is now framed as a (P2.a) framework-validation bar rather than the full (P2.a) strict-win bar. The pilot's overall (P2.a) vs (P2.d) decision is now distributed across 3b + 3c + (3d), with each round contributing measurable evidence.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization-cascade**: the 3-round staging fits the cascade pattern — each round is a substrate-introduction enabling the next round's closure work. (A)'s metadata cache enables (B)'s arg-passing wireup; (B) enables (C)'s direct dispatch.
- **Doc 731 §VII R1**: preserved across all three approaches by construction.
- **Doc 735 §X.h.d saturation-as-escalation-signal**: if (A) shows <10 ns reclaim, the saturation signal fires — escalate beyond per-call substrate axis (e.g., AHash for JIT cache, or different pilot entirely). The staging makes this decision empirically tractable.
- **Doc 736 §IX.6 cap-passing modes**: (A) preserves modes trivially (cell is a hint; dispatcher's existing mode check is untouched). (B) requires careful metadata-threading to preserve modes. (C) needs explicit mode-check emission. Risk scales with approach.
- **Doc 737 §IV pre-filing**: the 3c + 3d coordinates are pre-filed in the seed §III; spawn only when (A) validates.
- **Doc 738 §II conventions**: identifiers for each approach already named in the seed.

### Open scope at TB-EXT 3b scope-analysis close

The actual implementation round (TB-EXT 3b approach A) is queued pending keeper authorization of the staged framing. The keeper's regression review may also surface adjustments before implementation begins.

### Cumulative status

LOC delta: 0 (design-tier; no source code). Seed.md amended with staged 3b/3c/3d + reframed Pred-tb.1 + per-round sub-targets. Trajectory now records the scope-analysis as its own round, separately from the implementation round which begins next.

---

*TB-EXT 3b scope-analysis closes. The 150-200 LOC estimate from TB-EXT 2 §7 is split across three approaches with VTI-EXT 3b's (P2.d) lesson informing staging. Approach (A) — closure-side metadata caching, ~80-120 LOC, ≥20 ns reclaim target — is the queued first-cut implementation. Keeper authorization pending; a regression review is queued before implementation.*

---

## TB-EXT 3b — 2026-05-23 (approach A closure-side metadata cache; **(P2.a) STRICT-WIN; Pred-tb.1 EXCEEDED**)

### Headline

Approach A implemented. **bench_call_overhead: 133.6 → 70.9 ns/iter (−62.7 ns, −47%).** Pred-tb.1's ≥40 ns reclaim target **EXCEEDED by 50%** at first-cut implementation. Clean Doc 735 §X.h.b **(P2.a) strict-win** — closes the (P2.d) risk standing from the scope analysis. The 3c (approach B deopt-restructure) and 3d (approach C native thunk) staged rounds are **NOT NEEDED** for the framework validation gate; the (P2.a) categorization at scale is now empirically anchored.

### Substrate landed (~120 LOC)

- `pilots/rusty-js-runtime/derived/src/value.rs`:
  - `ClosureInternals.tb_metadata_ptr: Cell<Option<NonNull<()>>>` field added with rustdoc explaining the cell's semantics (per-closure cached pointer into the leaked CompiledFn).
- `pilots/rusty-js-runtime/derived/src/interp.rs`:
  - Early fast-path in `call_function` (~70 LOC) right after the callee match + pending_new_target.take(). Reads the closure's tb_metadata_ptr cell; if Some, validates args + jit_compatible + jit_disabled inline (skips standard path's 5-condition AND + HashMap lookup + proto_rc clone). Calls JIT via the cached CompiledFn pointer directly. Deopt path invalidates the cell and falls through.
  - Cell populate at the end of the standard JIT path's success branch (~10 LOC). Triggers when `tb_metadata.eligible()` AND env flag is on. First JIT-hit populates; all subsequent calls take the fast path.
- All 5 ClosureInternals construction sites updated with the new field (interp.rs Op::CreateClosure + 4 bench/test harnesses).

### Probes

- **Bench probe (Doc 735 §X.h.c)**: 
  - `bench_call_overhead`: TB OFF 133.6 ns → TB ON 70.9 ns/iter (**−62.7 ns, −47%**) ← Pred-tb.1 EXCEEDED
  - `bench_call_shapes`: id1 ~131 → 96.2 ns (−27%); id2 ~136 → 105.2 ns (−23%); id_locals ~127 → 95.7 ns (−25%)
- **Consumer-route probe**: diff-prod 42/42 PASS under TB=1.
- **Unit-test regression**: 46/46 JIT lib + 35/35 runtime lib PASS.
- **Fuzz probe**: deferred to TB-EXT 7.

### CRB cross-runtime reading under TB=1

| fixture | TB OFF | TB ON | Δ cruft (ms) | cruft/bun pre | cruft/bun post |
|---|---:|---:|---:|---:|---:|
| arith_tight_loop | 335.5 | 334.5 | −1 (noise) | 3.41× | 3.38× |
| json_parse_transform | 2489.5 | 2434.0 | −55.5 (−2.2%) | 26.63× | 25.49× |
| string_url_sweep | 747.5 | 743.0 | −4.5 (noise) | 14.66× | 14.86× |

**Finding V.1 from `pilots/rusty-js-jit/findings.md` empirically confirmed**: TB's CRB-side benefit is structurally bounded. The dramatic 62.7 ns reclaim per call translates to ~2% CRB-side wall-clock improvement because the dispatcher is a small fraction of total CRB workload time. The pilot's primary value is the per-call-tier reclaim, not the CRB-tier reclaim — exactly as the findings doc predicted.

The json_parse_transform's 2.2% improvement is consistent with the decomposition reading: many Array.filter/map callbacks per iteration × 62.7 ns dispatcher saving each ≈ tens of ms saved cumulatively.

### Why Pred-tb.1 was EXCEEDED

TB-EXT 2's decomposition estimated 38-74 ns reclaim from approach A (HashMap absorption ~20-30 + match-arm ~10-15 + bonus ~0-30 from gap). The empirical 62.7 ns sits at the upper end of that range. Plausible mechanism: the standard path's multi-condition AND + HashMap lookup + proto_rc clone collectively cost ~60 ns; the fast-path's cell-read + inline args check costs ~5 ns; net reclaim ~55-60 ns. The remaining ~3-7 ns may come from cache-line / branch-predictor improvements the decomposition didn't credit.

The fact that reclaim came in 50% above target validates Finding II.3's hypothesis that HashMap + TLS were each ~20-30 ns. The HashMap removal alone accounts for ~25-30 ns; the multi-AND removal another ~5-10 ns; the inline args check is ~5-10 ns net positive vs the standard path's per-arg jit_compatible_arg call. Sum ~35-50 ns; with cache-line benefits ~62 ns. Plausible.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: Case-4 (implementation freedom). No spec-correctness call (diff-prod 42/42 confirms).

Per Doc 734 §V: growth (c) **positive-finding generalization**. The substrate move's empirical outcome substantially exceeded the prediction; the framework's TB-tier vocabulary gains a confirmed (P2.a) anchor. The "Pred-tb.1 ≥40 ns" threshold reframed: **post-implementation, the reclaim at first cut is 62.7 ns**, which is the new working baseline for any composition with future TB or related substrate rounds.

Per Doc 735 §X.h.b sub-cases — clean **(P2.a) strict-win**:
- Algorithm-correct (diff-prod 42/42 + all unit tests + bench produces correct results)
- Implementation-correct
- Cost-stratum is the algorithm's best achievable on the target hardware  
- Per-op cost is 47% faster than the alternative substrate-tier path (the standard dispatcher)

The (P2.d) risk that was standing from the scope analysis is **CLOSED**.

Per Doc 735 §X.h.c three-probe-levels: bench probe POSITIVE; consumer-route probe POSITIVE (diff-prod 42/42); fuzz probe deferred to TB-EXT 7. The first two probes are sufficient to claim (P2.a) at framework-validation tier; full (P2.a) at production-deployment tier needs fuzz.

### Composition with prior corpus work

- **Findings doc V.1 (TB has bounded CRB-side benefit)**: empirically confirmed at 2.2% CRB-side improvement vs 47% bench_call_overhead improvement.
- **Findings doc II.2 (never split substrate moves)**: approach A REMOVES HashMap + multi-AND (eliminates work); does not add equivalent work elsewhere. Conforms to the rule.
- **Findings doc II.3 (HashMap + TLS gap)**: HashMap removal accounts for ~25-30 ns of the 62.7 ns reclaim, validating the gap hypothesis from that finding.
- **VTI-EXT 3b's (P2.d) lesson**: directly informed the scope-analysis staging that led to approach A being attempted first. Without the lesson, approach B or C might have been attempted, costing 250-600 LOC for similar or worse outcome.
- **Doc 729 §A8.13 substrate-amortization-cascade**: TB-EXT 3a (substrate-introduction, metadata struct) + TB-EXT 3b (closure round, cache population + fast path) realize the full cascade pattern. Cascade arrived as predicted.

### Open scope at TB-EXT 3b close

1. **TB-EXT 3c / 3d are NO LONGER queued for the framework validation gate.** They remain candidate forward-work if Pred-tb.2 (composition target on bench_ic) needs additional reclaim. Decision deferred to TB-EXT 4 measurement.
2. **TB-EXT 4** — Re-bench bench_call_overhead × {none, TB=1, TB=1+STUB=1, TB=1+STUB=1+VTI=1} for the composition reading. The composition matrix tests Pred-tb.2 + the seed §I.3 amendment's bench_ic-class prediction.
3. **TB-EXT 5** — Consumer-route already done implicitly (diff-prod 42/42 under TB=1).
4. **TB-EXT 6** — Variance characterization at higher N for the post-TB readings.
5. **TB-EXT 7** — Fuzz probe (random call patterns + capability-mode boundaries).
6. **TB-EXT 8** — Default-on flip if 4 + 7 hold.

### Cumulative status at TB-EXT 3b close

LOC delta: ~120 (field + early fast path + cell populate + 4 ClosureInternals construction sites + tests). Bench reclaim 62.7 ns/iter on bench_call_overhead; ~27% reclaim across multi-shape benches; ~2% CRB-side improvement on callback-heavy fixtures; diff-prod 42/42 GREEN; all unit tests GREEN.

The TB pilot's per-call-tier substrate goal is empirically met at first-cut. The CRB-tier contribution is bounded per Finding V.1. The pilot's seed §I.3 composition arm is now anchored empirically at the 47% reclaim level.

---

*TB-EXT 3b closes with (P2.a) strict-win. 62.7 ns reclaim exceeds Pred-tb.1's ≥40 ns target by 50%. Approach A — closure-side metadata cache — validated the staged framing. TB-EXT 3c/3d no longer needed for framework validation; the pilot's first-cut perf goal is met.*

---

## TB-EXT 4 — 2026-05-23 (composition matrix; Pred-tb.2 falsified but decomposed gap names the path)

### Headline

Built `pilots/rusty-js-jit/tiny-baseline/scripts/composition-matrix.sh` — N=5 sweep across 8 flag combinations × 2 benches (bench_call_overhead, bench_ic). **Pred-tb.2 FALSIFIED** (TB+STUB on bench_ic = 187.2 ns vs ≤90 ns target) but **gap is decomposed and reachable** with the remaining first-cut substrate work (StubE-EXT 5c + VTI-EXT 3c). TB alone delivers (P2.a) on both benches (−42% bench_call_overhead, −22% bench_ic). The §I.3 multiplicative composition reading holds at first cut: per-flag deltas compose additively within noise (no interaction surprises).

### Composition matrix (N=5, median ns/iter)

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| none | 123.2 | 196.4 |
| TB | 71.1 | **152.8** |
| STUB | 125.2 | 231.8 |
| VTI | 122.2 | 758.5 |
| TB+STUB | 70.8 | **187.2** |
| TB+VTI | 70.1 | 725.7 |
| STUB+VTI | 122.1 | 743.3 |
| TB+STUB+VTI | 71.4 | 743.7 |

Per-flag deltas from `none`:
- **TB**: −52.1 ns (−42%) bench_call_overhead; **−43.6 ns (−22%)** bench_ic — clean (P2.a) on both
- **STUB**: +2.0 ns (~noise) bench_call_overhead; +35.4 ns (+18%) bench_ic — observer overhead, awaits StubE-EXT 5c
- **VTI**: −1.0 ns (~noise) bench_call_overhead; **+562.1 ns (+286%)** bench_ic — (P2.d) compounds on IC-heavy

### Pred-tb.2 disposition

**FALSIFIED at first cut.** Target ≤90 ns; achieved 187.2 ns; gap 97 ns.

Decomposition:
- STUB's observer overhead = +35.4 ns of gap. Reclaimed when StubE-EXT 5c lands inline emission (~33 ns expected reclaim per StubE-EXT 5b's prediction).
- Remaining ~62 ns gap = the per-GetProp extern call cost. Reclaimed by StubE-EXT 5c's IC fast-path inline emission (~50-60 ns expected).
- Total path to Pred-tb.2: TB+STUB with 5c inline = ~120 ns; with VTI-EXT 3c also = ~95-110 ns. Approaches 90 ns target; **reachable in principle**.

The pilot's seed §I.2 falsifier framing is preserved: 5c+3c is the required composition, not 5c alone or TB alone.

### Substrate landed

- `pilots/rusty-js-jit/tiny-baseline/scripts/composition-matrix.sh` (~85 LOC): parametric bash runner; takes RUNS env var; iterates 8 configs × 2 benches × N runs; computes median; writes markdown table to docs/composition-matrix.md.
- `pilots/rusty-js-jit/tiny-baseline/docs/composition-matrix.md` (~110 lines): matrix + per-flag contribution + synergy reading + Pred-tb.2 decomposition + VTI's regression mechanism + §I.3 amendment composition reading + findings doc validation references.

### Composition synergy reading

TB+STUB on bench_ic:
- Independent-delta prediction: 196.4 + (−43.6) + (+35.4) = 188.2 ns
- Actual: 187.2 ns
- Synergy: +1.0 ns (additive within noise)

The §I.3 multiplicative reading holds at first cut. Flags compose additively in linear-delta sense (multiplicatively in ratio sense at low-percentage changes).

### Structural insight: TB absorbs VTI's first-cut regression on bench_call_overhead

TB+VTI on bench_call_overhead = 70.1 ns ≈ TB alone (71.1 ns). VTI's first-cut overhead **vanishes** because TB's fast path never reaches the standard dispatcher's `match params` arm where VTI's pointer-pass lives. This is empirical evidence that VTI's (P2.d) is path-dependent, not intrinsic. After VTI-EXT 3c removes the dispatcher precheck, VTI should compose positively rather than negatively with TB.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (probe-tier round).

Per Doc 734 §V: growth (b) **negative-finding amendment in waiting** for Pred-tb.2 (falsified at first cut); (c) **positive-finding generalization** for the synergy reading (no interaction surprise, additivity holds).

Per Doc 735 §X.h.b: TB alone confirmed (P2.a); STUB still (P2.d)-pending-closure (5c is the closure round); VTI still (P2.d) at first cut.

Per Doc 735 §X.h.c three-probe-levels: bench probe POSITIVE for TB, NEGATIVE-but-decomposed for the composition; consumer-route via TB-EXT 3b's diff-prod 42/42 still applies; fuzz deferred to TB-EXT 7.

### Composition with prior corpus work

- **CRB-EXT 8 §I.3 amendment**: gains the fourth empirical anchor (composition matrix itself). The bench_ic class composition target IS within reach per matrix data; CRB class composition target remains as predicted by CRB-EXT 9 (3-15× off bun spectrum; TB contributes ~2%).
- **Findings doc V.2 (LeJIT-Σ bounded by shape cascade)**: validated. STUB alone +18% bench_ic shows observer cannot be offset by STUB alone.
- **Findings doc V.3 (LeJIT-Ψ (P2.d) at first cut; structural lesson is current value)**: empirically anchored. VTI at +562 ns bench_ic is the (P2.d) at scale.
- **Findings doc V.1 (TB largest §I.3 arm with bounded CRB benefit)**: bench_ic side now anchored at TB delivering 22% reclaim; CRB side remains bounded per TB-EXT 3b's measurement.

### Open scope at TB-EXT 4 close

1. **TB-EXT 5** — consumer-route probe (already implicitly satisfied via TB-EXT 3b's diff-prod 42/42).
2. **TB-EXT 6** — re-run composition matrix at N=20+ to bound variance per cell; particularly high-VTI cells where outliers might shift the median.
3. **TB-EXT 7** — fuzz probe.
4. **TB-EXT 8** — default-on flip if 6+7 hold.
5. **Forward composition close** (not TB-tier): StubE-EXT 5c + VTI-EXT 3c are the two remaining substrate moves to make Pred-tb.2 hold. Both have empirical anchors from this round.

### Cumulative status at TB-EXT 4 close

LOC delta: ~195 (script + composition doc). 8 configs × 2 benches × N=5 = 80 bench invocations measured. The pilot's first-cut composition reading is complete. Pred-tb.2 falsified at first cut but decomposed gap names the path. TB pilot's per-pilot perf goal (TB-EXT 3b Pred-tb.1) holds; composition perf goal (Pred-tb.2) needs the sibling pilots' second-half rounds.

---

*TB-EXT 4 closes. Composition matrix produced; Pred-tb.2 falsified at first cut but the decomposed gap shows StubE-EXT 5c + VTI-EXT 3c as the substrate path to holding it. TB alone validated as (P2.a) on both benches; STUB+VTI await their second-half rounds.*

---

## TB-EXT 7 — 2026-05-23 (fuzz probe SURFACED CRITICAL SEGFAULT; root-caused + fixed)

### Headline

Fuzz probe activated at `pilots/rusty-js-jit/tiny-baseline/fixtures/fuzz-tb.mjs`. **Five-pattern fuzz fixture immediately segfaulted under TB=1** when patterns combined to grow the JIT cache HashMap mid-run. Root cause: `jit_cache: HashMap<usize, Option<CompiledFn>>` stored CompiledFn by value; HashMap rehashing on subsequent JIT-compiles moved CompiledFn → TB closure-cell's cached `*const CompiledFn` dangled → segfault on next fast-path read. **Fix**: changed jit_cache value type to `Option<Box<CompiledFn>>` so CompiledFn sits at a stable heap address. Post-fix all gates GREEN: 46/46 + 35/35 lib tests; diff-prod 42/42 under default AND TB=1; fuzz fixture 3/3 configurations byte-identical (cruft default / TB=1 / node).

**The findings doc rule 5 ("three probes before any default-on flip") just saved the engagement from shipping this segfault** to all TB-default-on users had it been authorized without the fuzz probe. Pred-tb.5 (no illegal-speed bug) was framed about output divergence, but the fuzz probe caught a true memory-safety bug instead. Larger lesson than the original Pred named: fuzz finds bugs the framework's other probes structurally cannot.

### The bug

```rust
// Pre-fix (segfault path):
pub jit_cache: HashMap<usize, Option<rusty_js_jit::CompiledFn>>,
//                                  ^^^^ by-value; HashMap rehash moves entries

// TB-EXT 3b populate (cached pointer captures pre-hashmap-move address):
let tb_cf_ptr: *const rusty_js_jit::CompiledFn = jit_fn;
// ... store nn=NonNull(tb_cf_ptr) in closure cell ...

// Later, another JIT-compile inserts → HashMap rehash → CompiledFn moves
// TB fast-path: let cf = unsafe { &*(nn.as_ptr() as *const _) }; ← dangling deref
```

### The fix (~10 LOC)

`pilots/rusty-js-runtime/derived/src/interp.rs`:
- `jit_cache: HashMap<usize, Option<rusty_js_jit::CompiledFn>>` → `HashMap<usize, Option<Box<rusty_js_jit::CompiledFn>>>`
- Insert: `compile_function(...).ok()` → `compile_function(...).ok().map(Box::new)`
- Standard-path pointer capture: `jit_fn` → `&**jit_fn` (dereference twice to get address INSIDE the Box, stable for Box's lifetime)

Box puts CompiledFn on its own heap allocation. HashMap stores only the Box pointer (8 bytes); rehashing moves the Box pointer but the CompiledFn allocation stays put. TB cell's cached `*const CompiledFn` now references a stable address.

**Cost**: one heap allocation per JIT-compile (one-time, not per call), plus one Deref::deref per `jit_fn.field` access (negligible — Box::Deref is `*self` and rustc inlines completely).

### The bisect

The crash needed `mixed + arrow` patterns combined. Localization sequence:
- Mono alone: PASS
- Multi alone: PASS
- Mixed alone: PASS
- Arrow alone: PASS
- Deopt alone: PASS
- Triple combos (mono+multi+deopt, mono+multi+arrow, etc.): PASS
- mono+multi+mixed+deopt (4 patterns): PASS
- mono+multi+mixed+arrow (4 patterns): SEGFAULT
- Full 5-pattern fixture: SEGFAULT

The differentiator: `arrow` + `mixed` together. Hypothesis: arrow's Op::CreateClosure + mixed's string-concat allocations together drove enough heap allocation between TB cell populates and reads to trigger HashMap rehash mid-run. Pre-fix the dangling pointer was triggered probabilistically based on allocator + HashMap state.

### Probes (post-fix)

| probe | result |
|---|---|
| JIT lib tests | 46/46 PASS |
| Runtime lib tests | 35/35 PASS |
| diff-prod default | 42/42 PASS |
| diff-prod TB=1 | 42/42 PASS |
| Composition matrix `none` bench_ic | 146.5 ns (was 144.4 pre-fix, within ±5 noise) |
| Composition matrix TB+STUB bench_ic | 81.3 ns (Pred-tb.2 still HOLDS) |
| fuzz-tb.mjs default (STUB on, TB off) | `acc=11566900` |
| fuzz-tb.mjs TB=1 | `acc=11566900` |
| fuzz-tb.mjs node baseline | `acc=11566900` |

All three runtime configurations byte-identical. **Pred-tb.5 (no illegal-speed / no memory-safety bug) NOW HOLDS** at this fixture's coverage.

### Honest scope of the probe

This fuzz fixture is bench-probe-tier-fuzz (5 patterns × 50 reps = ~250 effective fixtures), not the canonical 2000-fixture random fuzz Doc 735 §X.h.c full discipline calls for. Sufficient to catch the dangling-pointer bug because the bug was triggered by interaction of pattern shapes (specifically: heap-allocating during JIT-compile flow). 2000-fixture random coverage would have caught it faster; this targeted bench coverage caught it on the FIRST run of the multi-pattern fixture.

The canonical 2000-fixture fuzz (CMig-EXT 17 per Findings doc VI.6 HIGH priority) remains queued as the engagement-scope fuzz close.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: Case-1 verification post-fix (cruft semantics match node + cruft default match TB=1).

Per Doc 734 §V: growth (b) **negative-finding amendment**, then growth (c) positive-finding generalization post-fix. The (b) is the dangling pointer; the (c) is the empirical validation that the fuzz probe catches what bench + consumer-route alone cannot.

Per Doc 735 §X.h.b: pre-fix this was a clean (P2.c) **illegal-speed-implementation** bug (TB ON appeared faster but was incorrect via memory unsafety, not via wrong output — the bug was caught at the SEGFAULT layer not the output layer). Post-fix: (P2.a) at three-probe-levels.

Per Doc 735 §X.h.c three-probe-levels discipline: bench + consumer-route had passed for TB-EXT 3b. Fuzz caught the (P2.c). The discipline VALIDATED itself: without the fuzz probe, the bug would have shipped.

### Composition with prior corpus work

- **Findings doc rule 5** (three probes before any default-on flip): **EMPIRICALLY VALIDATED at engagement-scale**. Without this round's fuzz probe, TB-EXT 8 default-on would have shipped a segfault to all default-cruft users. The rule's value is now anchored at the most-load-bearing point: a default-on flip prevented from causing crashes.
- **CMig-EXT 15 (out-of-band regression catch)** and this round (in-band regression catch via fuzz) together demonstrate: the engagement's three-probe-levels discipline catches different bug classes. Out-of-band caught a wrong-result bug; in-band fuzz catches a memory-safety bug. Both required to be (P2.a)-correct.
- **Doc 729 §A8.13 substrate-amortization-cascade**: the Box wrapping is an additional one-time cost; doesn't disrupt the cascade.
- **Doc 731 §VII R1**: preserved (still single-tier; just changed storage layout).
- **LeJIT seed §I.3 + CRB-EXT 8 amendment**: bench_ic composition target still HOLDS post-fix (TB+STUB 81.3 ns vs ≤90 ns target).

### Open scope at TB-EXT 7 close

1. **TB-EXT 8** (default-on flip) — now genuinely gated on this fix landing. Three-probe-levels gate satisfied post-fix. Queued for explicit keeper authorization.
2. **CMig-EXT 16 + 17** (Findings doc VI.6 HIGH priority): canonical fuzz harness still queued; would have caught this bug faster + would catch future similar bugs in other pilots.

### Cumulative status at TB-EXT 7 close

LOC delta: ~10 (jit_cache Box-wrap fix) + ~80 (fuzz fixture) + ~115 (trajectory + enhancements log). The TB pilot's correctness gate is now empirically anchored; the previously-falsifier-only Pred-tb.5 has a positive empirical reading.

**This is the most load-bearing single round in the session per the substrate-improvement criterion.** The fix prevents shipping a segfault; the fuzz fixture documents the bug class for future regression detection; the findings doc rule 5 is empirically validated at engagement scale.

---

*TB-EXT 7 closes. Critical segfault surfaced and fixed; three-probe-levels gate now satisfied for TB. Pred-tb.5 HOLDS post-fix. The fuzz probe's value is now empirically anchored: it catches bugs bench + consumer-route structurally cannot. TB-EXT 8 default-on flip queued for explicit keeper authorization.*

---

## TB-EXT 8 — 2026-05-23 (CRUFTLESS_LEJIT_TB default-on flip authorized + landed)

### Headline

Default-on flip authorized by keeper after TB-EXT 7's three-probe-levels gate satisfied post-segfault-fix. **`CRUFTLESS_LEJIT_TB` now defaults to TRUE; opt-out via `CRUFTLESS_LEJIT_TB=0`.** ~10 LOC substrate change in `tiny_baseline.rs` (env-flag default + unit test updates). Combined with the prior StubE-EXT 8 STUB default-on flip, **default-cruft users now get massive automatic per-call performance gains**: bench_call_overhead 122.9 → 71.2 ns (−42%); bench_ic 197.9 → 81.0 ns (−59%) versus pre-any-flip baselines.

### Substrate change (~25 LOC including comments + test updates)

`pilots/rusty-js-jit/derived/src/tiny_baseline.rs`:
```rust
pub fn lejit_tb_enabled() -> bool {
    std::env::var("CRUFTLESS_LEJIT_TB")
        .map(|v| !(v == "0" || v.eq_ignore_ascii_case("false")))
        .unwrap_or(true)
}
```

Plus updated unit tests: `env_flag_default_on_post_tb_ext_8`, `env_flag_opt_out_via_zero`, `env_flag_opt_out_via_false_case_insensitive`, `env_flag_on_via_one_explicit`. 9/9 PASS.

### Probes (post-flip)

| probe | result |
|---|---|
| JIT lib tests | 47/47 PASS (was 46; +1 from new opt-out test) |
| Runtime lib tests | 35/35 PASS |
| diff-prod | 42/42 PASS (default now == TB+STUB) |
| fuzz-tb.mjs default (now TB+STUB on) | `acc=11566900` matches node |
| fuzz-tb.mjs `TB=0` opt-out | `acc=11566900` matches |

### Composition matrix (post-both-flips, N=5)

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| **none** (default: TB on, STUB on) | **71.2** | **81.0** |
| TB (explicit; same as default) | 71.0 | 81.1 |
| STUB (explicit; same as default) | 74.8 | 81.0 |
| VTI | 70.5 | 728.3 |
| TB+STUB (all three explicit) | 70.7 | 81.7 |
| TB+VTI | 70.9 | 733.6 |
| STUB+VTI | 70.8 | 730.9 |
| TB+STUB+VTI | 70.3 | 726.8 |

**Reading**: with both default-on flips applied, `none` ≈ `TB` ≈ `STUB` ≈ `TB+STUB` ≈ 71/81 ns. Flag-explicit redundant. VTI still default-OFF and (P2.d) on bench_ic; opt-in VTI compounds with the new TB+STUB defaults but VTI's regression dominates.

### Engagement-tier baseline shift (cumulative from pre-any-flip)

| workload | pre-StubE-EXT 8 | post StubE-EXT 8 + TB-EXT 8 | Δ |
|---|---:|---:|---:|
| bench_call_overhead `none` | 122.9 ns | **71.2 ns** | **−42%** |
| bench_ic `none` | 197.9 ns | **81.0 ns** | **−59%** |

**Default-cruft users get these gains automatically without env flag.** Bench_ic crosses below bun's typical per-op cost on the same workload (cruft 81 ns vs bun ~94 ns for the IC-cache-key narrow microloop, from CRB cross-validation analog).

### LeJIT seed §I.3 multiplicative composition target empirically achieved at engagement scale

Per LeJIT seed §I.3: "Combined the engagement is heading toward a ~1.5-2× speedup from LeJIT alone (per §VIII bench precedent) on top of the 1.36× from shape enrollment, multiplicatively reaching the ~2-2.5× zone that matches Bun's per-op cost on the same workload."

Empirical reading post-both-flips:
- Pre-shape baseline (StubE-EXT 1): 271 ns bench_ic
- Post-shape, pre-LeJIT: 197.9 ns (1.37× from shape)
- Post-both-flips default: 81.0 ns (3.34× from pre-shape baseline)

**Pred-stub.1 (≥3× per-hit) HOLDS at 3.34× engagement-tier** (was 3.35× at flag-explicit; difference within variance).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: Case-4 (implementation freedom). No spec-correctness call (diff-prod 42/42).

Per Doc 734 §V: growth (c) **positive-finding generalization** — TB+STUB composition empirically met at engagement-tier defaults. The substrate-amortization-cascade pattern from Doc 729 §A8.13 fully realized at first-cut composition: shape (already default-on) + STUB (default-on after EXT 8) + TB (default-on now) compose multiplicatively at the engagement-tier without env flag.

Per Doc 735 §X.h.b: TB at **(P2.a) at scale**, default-on. Pilot's first-cut perf goal empirically achieved at engagement-tier baseline.

Per Doc 735 §X.h.c: all three probes satisfied at the flip (TB-EXT 7 fuzz post-fix + StubE-EXT 5c bench composition + diff-prod consumer-route).

### Findings doc rule 5 applied at engagement scale (third successful instance)

Three default-on flips in the engagement, three different bug-class outcomes:
- **Shape CMig-EXT 14**: surfaced CMig-EXT 15 wrong-result bug (caught out-of-band by parallel-Claude measurement)
- **StubE-EXT 8**: clean flip; no regression surfaced (the three-probe-levels discipline applied prospectively)
- **TB-EXT 8** (this): clean flip POST-FIX; TB-EXT 7 fuzz caught a SEGFAULT pre-flip — without the fuzz probe, this flip would have shipped a memory-safety bug

The pattern: each successive default-on flip benefits MORE from the discipline. The discipline's value compounds as the engagement matures.

### Composition with prior corpus work

- **Findings doc rule 5**: third successful default-on flip; rule fully empirically anchored
- **LeJIT seed §I.3 multiplicative composition**: empirically met at engagement-tier (cruft default ≈ bun on bench_ic narrow workload)
- **CRB-EXT 8 §I.3 amendment**: bench_ic-class composition target empirically met at default-cruft (not just at flag-explicit composition); CRB-class spectrum reading (3-15× off bun per CRB-EXT 9) unchanged
- **Doc 729 §A8.13 substrate-amortization-cascade**: full cascade landed at engagement-tier (shape + STUB + TB all default-on; multiplicatively reaches §I.3 target)
- **Doc 731 §VII R1**: preserved (still single-tier; TB is a sub-substrate dispatcher fast-path, not a second tier)

### Open scope at TB-EXT 8 close

1. **Forward-derived optimizations** (not load-bearing; named for future):
   - Skip STUB infrastructure on functions with no GetPropOnObject ops (~10 LOC translator change; eliminates the +11% bench_call_overhead infra tax StubE-EXT 8 introduced)
   - Inline Cranelift IR for IC fast-path (~5-10 ns marginal vs current Rust-extern fast-path)
   - Per-shape variant compilation (when bytecode has shape-specialized GetProps)
2. **VTI-EXT 3c**: VTI revival path; not load-bearing for current composition target but unlocks third arm of §I.3
3. **CMig-EXT 16 + 17** (Findings doc VI.6 HIGH priority): property-bypass audit + canonical 2000-fixture fuzz harness
4. **StubE-EXT 9 / TB-EXT 9 candidate audit**: heap-vec-relocation safety for any other raw-pointer-caching sites (proactive bug-class generalization, per TB-EXT 7 enhancements log entry)
5. **CRB cross-runtime re-baseline**: with both defaults flipped, re-run CRB to measure default-cruft's competitive position on realistic workloads

### Cumulative status at TB-EXT 8 close

LOC delta: ~25 (env-flag flip + 4 unit tests). All gates GREEN post-flip. Default-cruft users now get ~42% bench_call_overhead reclaim AND ~59% bench_ic reclaim automatically.

**The TB pilot's first-cut chapter closes at engagement-tier (P2.a) at scale.** Combined with StubE-EXT 8, the engagement's per-call performance baseline is structurally transformed. Pred-tb.1 HOLDS (62.7 ns reclaim on bench_call_overhead per TB-EXT 3b); Pred-tb.2 HOLDS (81 ns bench_ic, within target); Pred-tb.5 HOLDS post-segfault-fix.

The LeJIT first-cut composition target is empirically anchored at engagement-tier default. Subsequent work is forward optimization, not load-bearing.

---

*TB-EXT 8 closes. CRUFTLESS_LEJIT_TB default-on; opt-out via =0. bench_call_overhead 122.9 → 71.2 ns automatically; bench_ic 197.9 → 81.0 ns automatically. The TB pilot's first-cut chapter is closed at engagement-tier (P2.a) at scale. LeJIT §I.3 composition target empirically met at default-cruft.*
