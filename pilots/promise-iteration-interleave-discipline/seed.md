# promise-iteration-interleave-discipline (PIID) — Seed

## Telos

Materialize the engine-DAG coordinate

```
runtime/promise-static :: E2/internal-method:abstract-op :: interleave-iter-and-then :: IteratorClose-on-abrupt + error-to-rejection
```

at the four Promise iteration intrinsics: `Promise.all`, `Promise.race`, `Promise.any`, `Promise.allSettled`. ECMA-262 §27.2.4 mandates that each iterate the input lazily via IteratorStep + Promise.resolve + .then chaining, with iter_close on any abrupt completion during the loop (per §27.2.4.1.1 PerformPromiseAll Step 8 + sibling abstract ops). Errors during iteration are converted to capability rejections, not synchronous throws.

## Origin

Founded 2026-05-31 per keeper Telegram 10694 ("Continue"). Surfaced during AFID.2 audit (cross-intrinsic survey of `collect_iterable` callers): all four Promise.* static methods route through `promise_collect_iterable_or_reject` which eagerly drains the iterable before any per-element processing. PIID-EXT 0 founds the locale with Promise.race; siblings Promise.all/any/allSettled remain rungs.

## Work shape

**Heuristics §IV classification**: D (Runtime abstract-op semantics) at four sibling Promise.* intrinsic surfaces.

Per-intrinsic per-element loop:
1. GetIterator(items) — on error, reject capability.
2. Loop:
   - IteratorStep(iter) — on error, reject capability (no close: §27.2.4 + §7.4.9 — IteratorNext failures don't trigger close in spec).
   - Read .done; if true, finalize (Promise.race: return; Promise.all: resolve capability with values; Promise.any: reject AggregateError if all rejected; Promise.allSettled: resolve with results).
   - Read .value.
   - Call Promise.resolve(C, value) — on error, IteratorClose + reject capability.
   - Per-method continuation: Promise.race chains .then(cap_resolve, cap_reject); Promise.all chains a Resolve Element closure + cap_reject; Promise.any chains cap_resolve + a Reject Element closure; Promise.allSettled chains a settle-this-index closure + cap_resolve. On error during chain setup, IteratorClose + reject capability.
3. Errors during the loop body that are not iter-step errors invoke IteratorClose-best-effort then reject the capability with the ORIGINAL error per §7.4.9 step 4.

## Apparatus

- **Direct probe**: per-intrinsic probe (e.g. `/tmp/probe-piid-0.js` for Promise.race; future probes per rung).
- **Runtime helpers**:
  - `Runtime::promise_iter_get_iterator(iter_v)` — Rust-side GetIterator returning the iter object id.
  - `Runtime::promise_reject_with_error(cap_reject, err)` — convert RuntimeError into a JS error object and invoke cap_reject. Mirrors the prior `promise_collect_iterable_or_reject` error-coercion path.
  - `Runtime::promise_race_interleave(...)` — per-method body (analog for each Promise.* sibling at its rung).
  - Reuses `crate::intrinsics::iter_close_rt` from AFID-EXT 0.

## Methodology

Per Doc 744 four-tuple + Rule 17 segmentation: scope is the four Promise.* iteration intrinsics. Carve out async-iter helpers, Promise constructor itself (synchronous executor), Promise.try, Promise.withResolvers.

Per Rule 24 Pin-Art recurrence: the interleave + iter_close_rt + error-to-rejection pattern recurs across four siblings. If all four land cleanly, the LIFT candidate is a generic `promise_iterate_with_capability<F>` helper. Defer until 2+ rungs land to gauge per-method signature variance.

## Carve-outs

- **Infinite-iter OOM is by-spec**: Promise.race/all/any/allSettled per spec iterate until done. An infinite iter is supposed to be infinite. PIID does NOT add short-circuit-on-capability-settlement (would be a spec divergence).
- **Async iterables** (for-await over Promise.* results): out of scope. Promise.allAsync is not a spec method.
- **Subclass C.resolve / C.then customization**: preserved at the same surface — the spec_get + is_callable + cap_resolve/cap_reject capture remains unchanged across the rewrite.

## Composes-with

- `pilots/array-from-interleave-discipline/` AFID-EXT 0/1/2 (substrate prefix: `iter_close_rt` helper + interleave pattern; second + third + fourth siblings at runtime-tier iterable consumption)
- `pilots/iterator-protocol-throw-discipline/` IPTD-EXT 1 (helper-tier `__destr_iter_close` discipline)
- `apparatus/docs/predictive-ruleset.md` Rule 17 (per-method segmentation), Rule 24 (Pin-Art recurrence at 3+ runtime-tier interleave sites)
- Doc 721 SAMPLE.1 chain-bundle (Promise.* iteration is in scope for the chain-bundle's runtime-tier shape).

## Resume protocol

Read `seed.md` then `trajectory.md`. PIID-EXT 0 is the founding rung (Promise.race only). Remaining rungs:
- **PIID-EXT 1**: Promise.all interleave (Resolve Element closures + remaining counter).
- **PIID-EXT 2**: Promise.allSettled interleave.
- **PIID-EXT 3**: Promise.any interleave (Reject Element closures + AggregateError).

Each rung mirrors the same body shape as Promise.race; the per-method differences are in the .then-chain construction.
