# LeJIT Enhancements Log

*Cross-locale empirical log spanning the LeJIT parent pilot and its nested sub-pilots (LeJIT-Σ stub-emitter, LeJIT-Ψ value-tag-inline, future LeJIT-Τ tiny-baseline, future LeJIT-Σ' x86_64 stub-emitter). Each entry records a measurable change to the LeJIT substrate's behavior — performance, semantics, infrastructure — with provenance back to the originating locale trajectory.*

## Discipline

Per the keeper's 2026-05-23 directive: this log captures both kinds of results that fall out of LeJIT-tier substrate work.

**Anticipated results** — outcomes the locale's seed or trajectory predicted in advance. Logged as a short one-paragraph row referencing the originating locale rung. The trajectory entry is the load-bearing record; this log is the cross-locale index.

**Unanticipated results** — outcomes no locale rung predicted. Logged verbosely (cause, mechanism, measurement, hypothesis about why the prediction missed, implication for forward work). The verbose form preserves the substrate-amortization-cascade signal per Doc 729 §A8.13 + Doc 734 §V.c positive-finding generalization. Unanticipated results often surface a corpus-tier or locale-tier framework refinement; verbose logging makes the recognition retrievable.

**Composition with the trajectory record**: each locale's `trajectory.md` records its rung-by-rung substrate moves. This enhancements log is the cross-locale slice that asks "what changed about LeJIT itself, measured how, predicted where." A reader at any point can read this log to get the LeJIT pilot's running empirical state without traversing four nested trajectories.

---

## 2026-05-23 — VTI-EXT 3a: Value layout pinning gives 5 ns/iter bench reclaim **[UNANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/value-tag-inline/trajectory.md` → VTI-EXT 3a (substrate-introduction round per Doc 729 §A8.13).

**Substrate change**: Added `#[repr(C, u8)]` to `pub enum Value` in `pilots/rusty-js-runtime/derived/src/value.rs` + eight `VALUE_TAG_*` discriminant constants + `VALUE_NUMBER_PAYLOAD_OFFSET` + compile-time const assertions + four runtime unit tests. ~70 LOC total. Pure layout pinning; no JIT-emitter changes, no calling-convention changes, no semantic changes to Rust callers.

**Measurement**:

```
bench_call_overhead pre-pinning  (VTI-EXT 1, 2026-05-23): 127.1 ns/iter
bench_call_overhead post-pinning (VTI-EXT 3a, 2026-05-23): 122.0 ns/iter
                                                       Δ:  −5.1 ns (−4.0%)
```

Workload: 1M iterations of `id(Number(42))` dispatched via `Runtime::call_function`. Same hardware (Pi target), same build profile (release), same workload, single-run measurement at each side (variance characterization deferred to VTI-EXT 6).

**Why this was unanticipated**: VTI-EXT 2's design doc (`pilots/rusty-js-jit/value-tag-inline/docs/inline-design.md` §3) estimated Option A's reclaim at "5-10 ns" but attributed the reclaim to the JIT-side prologue tag-check + payload-extract that VTI-EXT 3b will emit. The substrate-introduction round (3a) was framed as enabling apparatus — pin the layout so the closure round (3b) can emit against a known offset — with no per-iter cost reduction expected from the pinning alone.

The bench shows otherwise. The 5 ns reclaim arrived at the substrate-introduction tier, one round earlier than the closure round would have produced it. Either:

1. **`#[repr(C, u8)]` produces a more cache-friendly Value layout than rustc's default for `pub enum Value` without repr attributes.** The default layout for a Rust enum with mixed-size payloads (1-byte bool / 8-byte f64 / Rc<T> pointers) uses an opaque discriminant choice that may have produced a larger size or worse alignment than the explicit `repr(C, u8)` form. Smaller / better-aligned values would improve cache locality for the `Vec<Value>` argument array, the closure_v Rc clone, and the call_function dispatcher's match codegen. 5 ns/iter on a 1M-iter workload is consistent with one fewer cache line touched per call.

2. **Match codegen on the pinned discriminant is tighter.** Rustc's match-codegen for `#[repr(u8)]` enums emits a jump-table on the discriminant byte directly; for the default representation it must first extract the discriminant through whatever niche-or-tag scheme rustc picked. The `unbox_arg(&Value) -> i64` helper called at every dispatch site is exactly this shape. A tighter match would account for some of the reclaim.

3. **Measurement variance.** Single-run measurement. A ±5 ns variance band would put the result inside noise. Variance characterization is deferred to VTI-EXT 6; until then this finding is provisional.

**Hypothesis** (to be confirmed at VTI-EXT 6): cause is some mix of (1) and (2). The pinning's per-iter benefit is real but the magnitude needs multi-run variance bounding to be load-bearing.

**Implication for forward work**:

- **Per Doc 729 §A8.13 substrate-amortization-cascade reading**: the cascade arrived at the substrate-introduction tier in VTI as it did in shapes (LeJIT seed §I.3's recognition). The pattern recurs: each LeJIT-tier substrate-introduction round produces unanticipated per-iter cost reduction in addition to the apparatus it enables. The pattern is now suspected at two of two cases (shapes Shape-EXT 4 enrollment → 26%; VTI layout pinning → 4%). LeJIT seed §I.3 should be updated to predict the cascade explicitly at future substrate-introduction rounds (e.g., tiny-baseline's first apparatus move).

- **Per Doc 734 §V.c positive-finding generalization**: the recognition is corpus-original at the LeJIT-tier scale but consistent with §I.3's broader claim. No new corpus articulation needed; an amendment to LeJIT seed §I.3 records the second-case corroboration.

- **Per Doc 735 §X.h.c three-probe-levels**: bench probe alone is NECESSARY but not sufficient. diff-prod 42/42 GREEN is the consumer-route probe at the runtime-semantics tier; the bench reclaim hasn't been validated under non-monomorphic workloads (the Number-only id(x) bench). The fuzz probe (VTI-EXT 7) will exercise the variance + non-monomorphic cases. Until both land, the 5 ns reclaim is provisional — the right (P2) sub-case categorization per §X.h.b cannot be assigned yet (could be (P2.a) strict-win, could be measurement variance).

- **For VTI-EXT 3b**: the closure round's expected reclaim from the JIT-prologue emission itself must now be re-read against a 122 ns baseline (not 127 ns). If VTI-EXT 3b adds another ~5 ns reclaim, total VTI contribution would be ~10 ns, near the upper end of inline-design.md §3 Option A's 5-10 ns estimate. If 3b adds nothing, then 3a captured the entire VTI contribution and the seed's §I.3 composition reading needs further sharpening (VTI's arm shrinks; tiny-baseline's arm becomes even more load-bearing).

**Provenance**:
- Trajectory: `pilots/rusty-js-jit/value-tag-inline/trajectory.md` VTI-EXT 3a (close)
- Design doc: `pilots/rusty-js-jit/value-tag-inline/docs/inline-design.md` §3 Option A
- Bench harness: `cruftless/examples/bench_call_overhead.rs`
- Pre-pinning measurement: VTI-EXT 1 bench, 127.1 ns
- Post-pinning measurement: VTI-EXT 3a re-run, 122.0 ns

---

## Template — for future entries

### `<date>` — `<locale-tag>` `<round-id>`: `<one-line headline>` **[ANTICIPATED]**

**Locale**: <path> → <round> (motivating reason).

**Substrate change**: <one-paragraph description of what landed>.

**Predicted-by**: <which trajectory or seed rung predicted this and how>.

**Measurement**: <numbers, gates, regression status>.

**Provenance**: <bench harness + locale path>.

---

### `<date>` — `<locale-tag>` `<round-id>`: `<one-line headline>` **[UNANTICIPATED]**

**Locale**: <path> → <round> (motivating reason).

**Substrate change**: <description>.

**Measurement**: <numbers>.

**Why this was unanticipated**: <what the seed/trajectory said vs what the substrate did>.

**Hypothesis**: <plausible mechanism, gated by what would falsify or confirm>.

**Implication for forward work**: <bulleted list per active locale this affects>.

**Provenance**: <full chain of files + measurements>.

---

*This log is append-only. Entries are not edited after their round closes; subsequent rounds that overturn a finding land a new entry referencing the prior one. Per Doc 727 §X, the basin-stability discipline applies here too: a retracted finding becomes a corpus-tier amendment when its retraction is itself instructive.*
