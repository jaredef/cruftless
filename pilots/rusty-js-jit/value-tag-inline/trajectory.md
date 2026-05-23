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
