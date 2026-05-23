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
