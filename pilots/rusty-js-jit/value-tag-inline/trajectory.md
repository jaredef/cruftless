# LeJIT-Ψ (value-tag-inline) — Trajectory

Per-VTI-EXT log for the LeJIT-Ψ value-tag-inline pilot. Sibling sub-workstream to LeJIT-Σ; both children of `pilots/rusty-js-jit/`. Reads seed.md first.

---

## VTI-EXT 0 — 2026-05-23 (workstream founding)

### Headline

Apparatus-tier round. Pilot LeJIT-Ψ founded per Doc 737 §IV + keeper's pivot directive after LeJIT-Σ StubE-EXT 5b's bench measurement surfaced that the shape-substrate cascade (Doc 729 §A8.13) had absorbed most of LeJIT-Σ's IC-only contribution.

Per LeJIT seed §I.3's recognition: shape (1.36×) + LeJIT-Σ (~1.3-1.5×) + LeJIT-Ψ (~1.2-1.4×) ≈ the 3× Pred-stub.1 target multiplicatively. This pilot is the second arm of the composition.

### Substrate delivered

- `pilots/rusty-js-jit/value-tag-inline/seed.md` (~145 lines) — telos, apparatus (composes with parent LeJIT crate + sibling stub-emitter via multiplicative composition per LeJIT seed §I.3), methodology with VTI-EXT 0-8 staging, carve-outs (aarch64 only, Number tag only first cut, arg-coercion path only, no f64 NaN-box), composition table showing multiplicative pilot contributions, falsifiers Pred-vti.1-.5.
- `pilots/rusty-js-jit/value-tag-inline/trajectory.md` (this file).
- `pilots/rusty-js-jit/value-tag-inline/docs/` scaffold for VTI-EXT 1 + 2 outputs.

### Locale registration

Per Doc 737 §IV: nested locale at coordinate `pilots/rusty-js-jit/value-tag-inline/` (depth 2). Parent: `L.rusty-js-jit` (LeJIT). Sibling: `L.rusty-js-jit/stub-emitter` (LeJIT-Σ).

The engagement's third prospective-spawn case of the session (after consumer-migration + stub-emitter). Locale count: 12 → 13 after this round.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (founding round).

Per Doc 734 §V: growth (a) tier-relocation — the value-tag-inline tier was pre-filed at LeJIT seed §I.2 item 4 + §I.3; the spawn now makes it an active locale. Growth (c) positive-finding generalization — LeJIT-Σ EXT 5b's empirical bench surfaced the substrate-amortization-cascade reading; this pilot is structurally one of the arms named in that reading.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization**: LeJIT-Ψ + LeJIT-Σ + shape cascade compose multiplicatively, not additively, as predicted by Doc 729's vertical-recursion claim.
- **Doc 731 §VII R1**: preserved by construction (straight-line Cranelift IR, no second JIT tier).
- **Doc 735 §X.h.c three-probe-levels**: bench probe at VTI-EXT 1; consumer-route at VTI-EXT 4 (diff-prod + test262 gates); fuzz at VTI-EXT 7.
- **Doc 737 §IV locale-as-coordinate**: third prospective-spawn case of the session.
- **Doc 738 §II source-tier conventions**: identifiers will fit (`value_tag_inline.rs` per §II.e; `emit_inline_number_check` snake_case per §II.b; no `_via` because JIT-emitter-side).

### Open scope at VTI-EXT 0 close

1. **VTI-EXT 1** — Pre-emission bench probe. Extend `bench_ic.rs` (or create `bench_typed_i64.rs`) to measure per-call arg-coercion cost on a tight typed-i64 hot loop. Output: `docs/bench-baseline.md`.
2. **VTI-EXT 2** — Inline tag-check emitter design. Output: `docs/inline-design.md`.
3. **VTI-EXTs 3-8** per the seed §III methodology.

### Cumulative status at VTI-EXT 0 close

LOC delta: 0 (apparatus-tier). docs/ scaffold: 1 (empty dir). Locale registered.

The pilot's locale exists; VTI-EXT 1 begins with the bench baseline.

---

*VTI-EXT 0 closes. The third LeJIT-tier locale is founded; VTI-EXT 1 measures the arg-coercion baseline.*

---

## VTI-EXT 1 — 2026-05-23 (pre-emission call-overhead bench baseline)

### Headline

Bench probe activated. Hand-built `function id(x) { return x; }` driven through the current dispatch path; 1M iterations on the Pi target. **Baseline: 127.1 ns/iter** (127.077 ms elapsed). 

**Per-iter cost decomposition** vs LeJIT-Σ's bench_ic exposes that the dispatcher dominates:

| bench | per-iter | components |
|---|---:|---|
| `bench_call_overhead` (this) | 127 ns | dispatcher + arg-coerce + JIT id + rebox |
| `bench_ic` post-shape | 199 ns | as above + IC GetPropOnObject dispatch |
| `bench_ic` pre-shape (StubE-EXT 1) | 271 ns | as above + IndexMap probe |

Inferred components:
- IC GetPropOnObject dispatch: 199 − 127 = **72 ns** (LeJIT-Σ's target).
- Shape fast-path savings vs IndexMap: 271 − 199 = **72 ns** (already realized).
- Dispatcher + arg-coerce + JIT-id-body + rebox: ~127 ns total. The Rust dispatcher (`call_function`) is ~95% of this per StubE-EXT 2 §3 decomposition.
- **JIT preamble arg-coerce specifically: ~5-15 ns** (the LeJIT-Ψ target).

### Substrate landed

- `cruftless/examples/bench_call_overhead.rs` (~95 LOC) — bench harness. Minimal id(x) FunctionProto; JIT-compiled at threshold=1; 1M-iter wall-clock measurement.
- `pilots/rusty-js-jit/value-tag-inline/docs/bench-baseline.md` (~80 lines) — bench protocol; Pi baseline 127 ns; composition reading vs bench_ic; cost decomposition; honest Pred-vti.1 reading; dispatcher-as-largest-single-cost observation.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (observe-only). Per Doc 734 §V: growth (c) positive-finding generalization preparatory — the baseline is the empirical anchor for VTI-EXT 6's (P2) categorization.

### Honest finding: Pred-vti.1 may be tight against arg-coerce-specific cost

The arg-coerce-specific cost is ~5-15 ns of the 127 ns total. The seed's Pred-vti.1 target (≥20 ns reduction) may not be reachable from tag-check inlining alone. The 20-ns threshold was set per LeJIT seed §I.3's "~1.2-1.4× contribution from VTI" estimate; on the 127 ns baseline that's 25-50 ns saved, which requires VTI's emission to also collapse some adjacent JIT-preamble costs (not just the tag-check).

**Sharpening for VTI-EXT 4 measurement**: report against both (a) the absolute per-iter cost on `bench_call_overhead` post-VTI, and (b) Pred-vti.4's composition target (bench_ic under shape + LEJIT_STUB + LEJIT_VTI ≤ 120 ns/iter). The composition target is the load-bearing one.

### Stronger structural finding: the dispatcher is the largest single cost

The Rust `call_function` dispatcher accounts for ~95% of the 127 ns bench. **Neither LeJIT-Σ nor LeJIT-Ψ targets this**; the dispatcher refactor (LeJIT seed §I.2 item 5: tiny-fn fast-baseline) does. Per the §I.3 composition table, the dispatcher refactor's expected contribution is 1.5-2× — the largest single arm of the four-pilot composition.

**The recognition**: LeJIT-Ψ alone cannot reach the 3× target either. The dispatcher refactor is the remaining required arm of the multiplicative composition. The current seed §I.3 composition table already names this; today's measurement confirms it empirically at the call-overhead tier.

### Open scope at VTI-EXT 1 close

1. **VTI-EXT 2** — Inline tag-check emitter design. Must first establish cruft's Value encoding (NaN-box? tagged enum? something else); the inline IR shape depends on it. Output: `docs/inline-design.md`.
2. **VTI-EXT 3** — Scaffold `value_tag_inline.rs` module.
3. **VTI-EXTs 4-8** — wire + measure + fuzz + default-on per the seed §III.
4. **Suggestion (future)**: spawn `pilots/rusty-js-jit/tiny-baseline/` as the fourth LeJIT-tier nested locale, per LeJIT seed §I.2 item 5 + this round's empirical finding that the dispatcher is the dominant cost. Pre-file for now; spawn when the substrate calls.

### Cumulative status at VTI-EXT 1 close

LOC delta: ~95 (bench harness + docs). diff-prod 42/42 unchanged.

The call-overhead substrate baseline is 127 ns/iter on the Pi. Combined with bench_ic's 199 ns (IC included) and 127 ns (no IC), the per-iter cost is now decomposed across four named components: dispatcher (~120, the dominant), JIT id body + rebox (~5), arg-coerce (~5-15, VTI's target), IC dispatch (~72, LeJIT-Σ's target).

---

*VTI-EXT 1 closes. Baseline 127 ns/iter. The decomposition confirms LeJIT seed §I.3's composition reading + sharpens it: the dispatcher refactor (item 5) is the largest single arm and likely required for the 3× target. VTI-EXT 2 designs against the ~5-15 ns arg-coerce budget.*

---

## VTI-EXT 2 — 2026-05-23 (inline tag-check emitter design)

### Headline

Apparatus-tier design round. Read cruft's Value encoding cold, surfaced a structural recognition that re-frames the pilot's telos, evaluated three options per Doc 735 §X.h.b discipline, recommended Option A (push arg-coerce into JIT emission via layout pinning). Output: `docs/inline-design.md` (~140 lines).

### Findings

**Value is a tagged Rust enum, NOT NaN-boxed, NOT `#[repr]`-pinned.** Eight variants (Undefined, Null, Boolean, Number, String, BigInt, Symbol, Object), mixed-size payloads. Rustc's layout algorithm picks discriminant placement; the resulting offsets are stable per-build but not specified. Inline discriminant read from JIT-emitted code is undefined behavior absent layout pinning.

**The original LeJIT-Ψ framing assumed a tag-check inside the JIT body that does not exist.** Per `interp.rs:8420-8438` the dispatcher calls `unbox_arg(&args[0])` BEFORE invoking the JIT body; the JIT receives a pre-unboxed `i64`. The "one inline branch-on-tag where Cranelift routes through a function-call abstraction" (LeJIT seed §I.2 (b)) describes a substrate that the current calling-convention has elided into Rust dispatcher code.

### Three options under Doc 735 §X.h.b

| Option | Move | Δ ns/iter (est.) | Sub-case risk |
|---|---|---:|---|
| **A** — push arg-coerce into JIT via `#[repr(C, u8)]` + JIT-prologue tag-check | ~135 LOC + new DeoptReason::WrongArgTag | 5-10 | (P2.c) layout-drift bounded by const assertion |
| **B** — unsafe tag-read in Rust dispatcher (no JIT-side change) | ~20 LOC unsafe | 1-3 | (P2.d) misfit with pilot telos |
| **C** — recognize VTI as (P2.d), pivot to tiny-baseline spawn | 0 (at VTI) | — | (P2.b) wrong-stratum-composition if tiny-baseline needs VTI as substrate |

**Recommendation**: Option A. Aligned with pilot telos; layout pinning is a substrate-amortization-cascade enabler per Doc 729 §A8.13 (consumed by IC stub + dispatcher refactor + VTI); composition with tiny-baseline remains intact since the four §I.3 arms are structurally independent.

### Pred-vti.1 update proposed

Original Pred-vti.1: ≥20 ns reduction on arg-coerce. Today's bench reading shows this is unreachable from VTI alone (arg-coerce-specific cost is ~5-15 ns of 127 ns dispatcher total). Updated target: ≥5 ns reduction on `bench_call_overhead`. Load-bearing falsifier shifts to Pred-vti.4 (composition target ≤120 ns on bench_ic under shape + LEJIT_STUB + LEJIT_VTI), which is unchanged.

### Substrate landed

- `pilots/rusty-js-jit/value-tag-inline/docs/inline-design.md` (~140 lines): Value-encoding finding, three-option analysis, Option A recommendation with cost/risk/composition reading, VTI-EXT 3a/3b staging.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (design-tier round). Per Doc 734 §V: growth (b) negative-finding amendment — empirical reading from VTI-EXT 1 + cold-read of cruft's Value layout contradicted the LeJIT seed §I.2 (b) framing's implicit assumption that a tag-check exists inside the JIT body. The pilot's telos is preserved but the substrate move that realizes it is now named (Option A's layout pinning + JIT-prologue tag-check emission), where before it was elided. Per Doc 738 §III cross-axis consistency: the structural recognition was reachable from reading the dispatcher's identifier conventions (`unbox_arg` is at Runtime-tier, signature `&Value -> i64`, called from dispatcher; `jit_fn.func.call1` takes `i64` — the convention names the elision at sight).

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization-cascade**: layout pinning pays once, consumed by IC stub + dispatcher refactor + VTI. Per the seed §I.3 multiplicative reading, this is the second substrate-introduction round in the LeJIT-tier; shape enrollment was the first.
- **Doc 735 §X.h.b four sub-cases**: Option A is (P2.a) candidate; Option B is (P2.d) misfit; Option C is honest (P2.d) recognition. Three-probe-levels discipline (§X.h.c) applies at VTI-EXT 4+ (bench + consumer-route + fuzz).
- **Doc 737 §IV pre-filing**: tiny-baseline pre-file remains; VTI-EXT 2's reading sharpens the case for spawning it as a sibling pilot (LeJIT-Τ?) but defers the spawn pending VTI-EXT 4+'s empirical composition reading.
- **Doc 738 §II conventions**: VTI-EXT 3 scaffold lands at `pilots/rusty-js-jit/derived/src/value_tag_inline.rs` (§II.e pillar-path). Emitter functions follow §II.b post-§A8.32 receiver-discriminated form (no `_via` suffix; these are JIT-emitter functions, not Runtime-dispatching helpers).

### Open scope at VTI-EXT 2 close

1. **VTI-EXT 3a** — Substrate-introduction round per Doc 729 §A8.13: `#[repr(C, u8)]` on `pub enum Value`, const assertions for NUMBER_TAG + payload offset, calling-convention switch to `*const Value` in JitFn signatures. ~50 LOC. Workspace test should remain GREEN (the repr change is layout-only; no semantic change to Rust callers).
2. **VTI-EXT 3b** — Closure round: emit JIT-prologue tag-check + payload-extract behind `CRUFTLESS_LEJIT_VTI=1`. ~80 LOC. New DeoptReason::WrongArgTag variant.
3. **VTI-EXTs 4-8** per the seed §III methodology.

### Cumulative status at VTI-EXT 2 close

LOC delta: 0 (design-tier round). docs/ now contains bench-baseline.md + inline-design.md.

The pilot's substrate-introduction round (VTI-EXT 3a) is staged. The structural recognition from VTI-EXT 2 — that the original framing's "tag-check inside JIT body" does not exist and must be created via Option A's calling-convention shift — sharpens the pilot's substrate work without retracting the telos. The composition reading at LeJIT seed §I.3 is preserved; VTI's expected contribution is revised downward (5-10 ns) and the load-bearing falsifier is Pred-vti.4 (composition target).

---

*VTI-EXT 2 closes. Option A recommended; layout-pinning substrate-introduction round queued at VTI-EXT 3a. Honest budget acknowledges VTI alone reclaims 5-10 ns; the 3× target requires composition with tiny-baseline (LeJIT seed §I.2 item 5) per the §I.3 multiplicative reading.*
