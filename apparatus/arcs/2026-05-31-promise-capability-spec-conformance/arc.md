---
arc: 2026-05-31-promise-capability-spec-conformance
trigger: Session-31 cascade after the IPTD/ICES/AFID/PIID iterator-protocol chain surfaced sympathetic Promise-capability + Promise.prototype spec divergences across the four Promise.* statics, the three Promise.prototype methods, NewPromiseCapability, and the Promise constructor itself. Empirically anchored in the 2026-05-31 06:33 test262-sample bucket analysis (Promise was the largest remaining FAIL cluster at 190 cells, dwarfing every other built-in) and the keeper directive Telegram 10709 + 10743 (the latter authorizing arc-tier consolidation).
opened: 2026-05-31
closed: IN PROGRESS
close_condition: every Promise.* static + Promise.prototype.* + NewPromiseCapability + Promise constructor produces Node-faithful behavior on the ctx-non-object / ctx-ctor / capability-executor / capability-resolve-throws / invoke-then-getter / this-value-obj-coercible / SpeciesConstructor / invoke-then-delegation / thenable-resolution / x.constructor-check / AggregateError-construction test families; zero PASS→FAIL regressions per round per Finding T262C.5 default discipline; arc closes when the Promise test262 sample bucket drops below 50 cells (from pre-arc 190).
---

# PCSC: Promise Capability Spec Conformance Arc

## I. Provenance

Founded 2026-05-31 per keeper Telegram 10743 authorizing apparatus-tier consolidation of the session-31 substrate cascade. Originated as a sympathetic surface during the iterator-protocol arc's PIID landings: PIID-EXT 0+1+2+3+4 (Promise.* iteration interleave + IteratorClose + AggregateError) routed capability rejection through the same `promise_reject_with_error` helper, but the broader Promise-capability surface (this-validation, subclass routing, capability-executor invariants, then-getter abrupt completion, SpeciesConstructor, thenable resolution) was discovered during the post-PIID test262-sample fail-bucket analysis and landed as a coherent sub-program over ~3 hours (Telegrams 10709 through 10733).

The substrate landed before the arc was scaffolded — this arc.md retroactively coordinates the sub-locales/extensions under one arc-tier coordinate, codifies the cumulative findings, and sets the close condition.

## II. Telos

**Empirical answer to**: when the Promise capability + prototype surface is brought to Node-faithful spec conformance (six Promise.* statics + three prototype methods + the constructor + NewPromiseCapability), does the Promise test262-sample bucket compress from 190 cells toward the ~50-cell floor predicted by the post-session-31 incremental measurement?

### II.1 Resume-vector projection (sub-locale ordering, retroactive)

Session-31's landing order, with arc-roster annotation:

| Order | Sub-locale / EXT | Substrate site | Cells closed (estimate) | Risk at landing |
|---:|---|---|---:|---|
| 1 | PSCV-EXT 0 (Promise.* this-Object check) | `promise.rs` 6 registrations | ~36 (6×6 ctx-non-object) | minimal |
| 2 | PSCV-EXT 1 (subclass routing via NewTarget + cap.[[Resolve/Reject]]) | `promise.rs` Promise ctor + resolve/reject closures | ~6 (ctx-ctor) | bounded |
| 3 | PCEXC-EXT 0 (capability-executor-called-twice) | `interp.rs` `new_promise_capability` | ~5 (5 directories) | minimal |
| 4 | PCRT-EXT 0 (cap.resolve throws → IfAbruptRejectPromise) | `interp.rs` PIID interleavers | ~4 (Promise.all + allSettled) | minimal |
| 5 | PTHEN-EXT 0 (then-getter throws → close + reject) | `interp.rs` PIID interleavers ×4 | ~4 (all/race/any/allSettled) | minimal |
| 6 | PCATCH-EXT 0 (Promise.prototype.catch ToObject coercion) | `interp.rs` `promise_catch_via` | ~4-6 (Boolean/Number/String/Symbol) | minimal |
| 7 | PTHEN-EXT 1 (Promise.prototype.then SpeciesConstructor) | `interp.rs` `promise_then_via` | ~6-10 (ctor-null/throws/poisoned/custom) | medium (touches every then) |
| 8 | PFINALLY-EXT 0 (Invoke(this, "then") delegation) | `interp.rs` `promise_finally_via` | ~16 (invokes-then-* + species + subclass-reject-count) | medium |
| 9 | PRESOLVE-EXT 0 (thenable resolution per §27.2.1.4) | `promise.rs` `resolve_promise` | ~5-8 (arg-poisoned-then + resolve-poisoned-then-immed/deferred) | bounded |
| 10 | PRESOLVE-EXT 1 (Promise.resolve x.constructor === C) | `promise.rs` resolve closure + reject closure | ~3-5 (arg-uniq-ctor + capability-invocation-error) | minimal |

**Cumulative estimated yield: ~90-100 cells closed across the Promise cluster from the post-PIID baseline.**

### II.2 Constraints (engagement-level discipline carried in)

```
C1. Each sub-extension follows Doc 740 multi-tier closure (Finding T262C.5).
C2. Each sub-extension exemplar-verifies before authorization; full-sweep on
    keeper directive only.
C3. Each sub-extension's verification includes a regression probe over all
    prior PCSC + adjacent PIID/IPTD probes.
C4. Per Finding T262C.6: probe per-cluster failure-REASON heterogeneity
    before scoping each sub-extension.
C5. Each chapter-close inspection reports the per-arc cumulative cell delta
    measured against the 2026-05-31 06:33 sample baseline.
```

## III. Sub-locale roster

| Locale | Role in arc | Status |
|---|---|---|
| `promise-static-ctx-validation` (PSCV) | Promise.* this-Object check + subclass routing via NewTarget | LANDED (EXT 0 + EXT 1) |
| `promise-capability-executor-discipline` (PCEXC; embedded fix, no standalone locale) | NewPromiseCapability executor already-called invariant | LANDED in `new_promise_capability` |
| `promise-capability-resolve-throws-routing` (PCRT; embedded fix) | done-branch cap.[[Resolve]] abrupt → cap.[[Reject]] via IfAbruptRejectPromise | LANDED in PIID interleavers |
| `promise-then-invocation-abrupt-routing` (PTHEN; embedded fix) | then-getter throws → close + capability reject across 4 statics + SpeciesConstructor at Promise.prototype.then | LANDED in PIID interleavers + `promise_then_via` |
| `promise-prototype-receiver-coercion` (PCATCH; embedded fix) | Promise.prototype.catch ToObject-coerces primitive this per §27.2.5.1 + Invoke (GetV) | LANDED in `promise_catch_via` |
| `promise-prototype-finally-then-delegation` (PFINALLY; embedded fix) | Promise.prototype.finally returns Invoke(this, "then", thenFinally, catchFinally) per §27.2.5.3 | LANDED in `promise_finally_via` |
| `promise-resolve-thenable-discipline` (PRESOLVE; embedded fix) | §27.2.1.4 thenable resolution + §27.2.4.7 step 1 x.constructor === C short-circuit + Promise.reject spec path | LANDED in `resolve_promise` + Promise.resolve/reject closures |

Five of seven are embedded fixes (no standalone seed.md+trajectory.md) — they live inside the existing `interp.rs` / `promise.rs` substrate sites and are tracked under this arc's roster + their per-rung proposals/decisions under `apparatus/proposals/decided/`. Per the orphan-disposition protocol's "close-as-locale-singleton" disposition for arc-relevant single-substrate-site fixes that don't warrant a separate locale scaffolded directory.

Cross-listed with `2026-05-28-iterator-protocol-substrate` (lattice): PCRT and PTHEN both touch the PIID interleave loops; the IteratorClose half is the iter-protocol arc's, the capability-reject half is this arc's.

## IV. Apparatus

- Test262 sample baseline: `scripts/test262-sample/results/2026-05-31/` (89.6% runnable; Promise FAIL bucket 190 cells pre-PSCV).
- Failure-bucket histogram analysis: ad-hoc Python on `results.jsonl` (Telegram 10709 + 10719 + 10723 + 10727 + 10729 reports).
- Substrate sites: `pilots/rusty-js-runtime/derived/src/promise.rs` (static method closures + Promise ctor + `resolve_promise`); `pilots/rusty-js-runtime/derived/src/interp.rs` (`promise_then_via`, `promise_catch_via`, `promise_finally_via`, `new_promise_capability`, four `promise_*_interleave` helpers, `promise_reject_with_error` helper, `promise_iter_get_iterator` helper, `make_aggregate_error_via`).
- Direct probes: `/tmp/probe-pscv.js` (39/39), `/tmp/probe-pcexc.js` (8/8), `/tmp/probe-pcrt.js` (2/2), `/tmp/probe-pthen.js` (4/4), `/tmp/probe-pcatch.js` (7/7), `/tmp/probe-pthen-species.js` (6/6), `/tmp/probe-pfinally.js` (14/14), `/tmp/probe-presolve.js` (5/5), `/tmp/probe-presolve-1.js` (6/6).

## V. Predictions

- **Pred-pcsc.1**: arc closure compresses the Promise sample FAIL bucket from 190 to <50.
- **Pred-pcsc.2**: each sub-extension closes in 1 round per Finding T262C.5 default discipline.
- **Pred-pcsc.3**: zero PASS→FAIL regressions across the cumulative landing chain (10 substrate landings).
- **Pred-pcsc.4** (Finding T262C.4 refinement): each sub-extension targets a single failure-reason cluster at the cell level; arc-tier cascade is the SUM of per-extension cell-deltas, not a single shared-upstream multiplier.

## VI. Carve-Outs

- **Promise.allKeyed / Promise.allSettledKeyed** (Stage 1/2 proposals) — out of scope; the test262 sample has 17 + 18 fails there but these are likely unimplemented features rather than spec-divergence. Sibling locale candidate.
- **Symbol.species @@-getter resolution edge case** (PTHEN-EXT 1 §3.2 carry-forward) — when species is defined via `static get [Symbol.species]() { return X; }`, the fast-path id-compare doesn't match because the Symbol-keyed lookup misses. Cross-listed with `well-known-symbol-lookup-helper` (WKSL); closes at WKSL-EXT 2 or this arc's EXT-S rung, whichever takes the Symbol-bucket fall-through deeper.
- **Asynchronous deep-async-thenable resolution** (PRESOLVE-EXT 0 deferred microtask job) — current impl invokes thenable.then synchronously; full §27.2.1.4 PromiseResolveThenableJob microtask deferral is a separable rung.
- **Promise.any AggregateError CTOR error-cause chain** — PIID-EXT 4 wires AggregateError prototype + branding; the cause chain (`new AggregateError(errors, message, { cause })`) is not exercised by PIID-EXT 4. Carry-forward.

## VII. Composes-with

- `2026-05-28-iterator-protocol-substrate` (cross-listing for PCRT + PTHEN at the PIID interleave site)
- `apparatus/docs/predictive-ruleset.md` Rule 13 (Pi-OOM revert led to the whole cascade; Promise discovery was downstream), Rule 17 (per-method segmentation across 6 statics + 3 prototypes), Rule 23 (founding-time tier-mismatch — Pi-OOM dramatized the cost of skipping baseline-inspect), Rule 24 (Pin-Art emit-site coherence across 11 runtime intrinsics, of which 4 are Promise.* statics)
- Doc 729 (resolver-instance pattern: Promise capability is a per-call resolver), Doc 731 (alphabet purity at upstream Promise.* statics bounds downstream Promise.prototype.* complexity)
- `apparatus/proposals/decided/` 2026-05-31T140000Z through 2026-05-31T154500Z (10 PCSC decision docs)
