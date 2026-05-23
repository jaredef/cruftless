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
