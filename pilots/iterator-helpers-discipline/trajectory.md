# iterator-helpers-discipline — Trajectory

## IPHD-EXT 0 — LANDED (2026-05-31) — Iterator.from lazy wrapper + 9 helpers interleaved with iter_close_rt

**Trigger**: Keeper Telegram 10702 ("Let's go with 1"). Surfaced by cross-intrinsic audit after AFID-EXT 3: every Iterator.prototype helper used `drain_iterator` for eager-collect, and Iterator.from itself eager-drained the source before wrapping as an array-iterator. Even `Iterator.from(infinite_iter).take(1).toArray()` OOMed.

**Substrate** (~250 LOC at `pilots/rusty-js-runtime/derived/src/intrinsics.rs`):

1. **`Iterator.from` lazy wrapper** (replaces drain_iterator + make_array_iterator path): the result is an Iterator-prototype-rooted object with `__wrapped_iter` engine sentinel holding the source iter; install `next` (forwards to inner.next) and `return` (forwards to inner.return if present, silent if missing). Preserves the source's IteratorClose semantics across the helper chain.

2. **Per-method interleave** at 9 helpers — `map`, `filter`, `take(n)`, `forEach`, `some`, `every`, `find`, `reduce`, `toArray`. Each calls `next` on `this` directly per iteration with:
   - non-Object next-result → close + TypeError;
   - done → break;
   - cb abrupt completion (or short-circuit true/false/found/take-reached) → `iter_close_rt` then return/propagate.

Helpers NOT covered at EXT 0 (carry-forward):
- `drop(n)` — needs a true lazy iterator that skips n elements on first consumption.
- `flatMap` — nested-iterator close semantics; inner-iter close on outer abrupt.
- True-lazy `map`/`filter` — current impl returns array-iterator after eagerly mapping/filtering. Per spec they should be lazy iterators that defer cb application to consumer.

**Yield**:

```text
IPHD-EXT 0 probe (/tmp/probe-iter-helpers-audit.js): 8/8 PASS

  .forEach cb throw      -> close + propagate         ✓
  .some cb throw         -> close + propagate         ✓
  .every cb throw        -> close + propagate         ✓
  .find cb throw         -> close + propagate         ✓
  .reduce cb throw       -> close + propagate         ✓
  .some short-circuits on first true + closes         ✓
  .find short-circuits + closes                       ✓
  .take(2).toArray()    -> [1,1] + close after 2     ✓

cargo test --release -p rusty-js-runtime --lib: 74 / 0 / 1 preserved.

Regression sweep preserved: IPTD 7/7, cross-consumer 7/7,
ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8, AFID-EXT 1 7/7,
AFID-EXT 3 7/7, PIID-EXT 0 6/6, PIID-EXT 1+2+3 12/12, PIID-EXT 4 9/9.
```

**Phase 3 (Pin-Art if duplicated)** per Rule 24: the per-element interleave + iter_close_rt + cb-throw + short-circuit pattern is now at TEN runtime-tier intrinsic sites: Array.from, Set ctor, WeakSet ctor, 4× Promise.*, Object.groupBy, Map.groupBy, plus Iterator.from + 9 helpers. The Iterator helpers form their own sub-family with the same primitive shape; a per-family LIFT (generic `iter_helper_consume<F1, F2>(this_id, cb, on_value, done_behavior)`) would compress the 9 helper bodies. Deferred pending body-shape variance review across map/filter/take/forEach/some/every/find/reduce/toArray.

**Phase 5 (Chapter-close-inspect)** per Rule 15: three residuals carry-forward to IPHD-EXT 1/2/3:
- **drop**: lazy iterator skipping n on first consume.
- **flatMap**: nested close on outer abrupt + inner exhaustion.
- **True-lazy map/filter**: deferred eager-to-lazy evaluation per spec §27.1.4.

**Status**: IPHD-EXT 0 LANDED. Iterator helpers spec-correct on close-on-cb-throw + short-circuit-close + Iterator.from forwarding-close. Substrate prefix shipped: the lazy-wrapper pattern from Iterator.from is the substrate that EXT 3 (lazy map/filter) will reuse.
