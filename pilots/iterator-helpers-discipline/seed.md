# iterator-helpers-discipline (IPHD) — Seed

## Telos

```
runtime/iterator-helpers :: E2/internal-method:abstract-op :: interleave-and-close :: short-circuit + cb-throw + lazy-wrapper
```

at the ES2025 Iterator helpers surface: `Iterator.from`, `Iterator.prototype.{map, filter, take, drop, flatMap, toArray, reduce, forEach, some, every, find}`. Each must per ECMA-262 §27.1.4:
- Iterate the source lazily via IteratorStep (no eager drain).
- Short-circuit + call IteratorClose on early-termination paths (`some`-true, `every`-false, `find`-truthy, `take(n)` reached, `flatMap` inner-throw, etc.).
- Call IteratorClose on cb abrupt completion + propagate the original throw per §7.4.9 step 4.
- Forward IteratorClose through Iterator.from-wrapper to the underlying source.

## Origin

Founded 2026-05-31 per keeper Telegram 10702 ("Let's go with 1"). Surfaced during the keeper's audit chain after AFID-EXT 3: probing Iterator.from + Iterator.prototype helpers showed all 12 methods using `drain_iterator` for an eager-collect anti-pattern. Even simple `Iterator.from(infinite_iter).take(1).toArray()` OOMed.

Tier-distinct from AFID (Array.from / Set ctor / Promise.* / groupBy): those intrinsics directly consume external iterables. IPHD covers the Iterator-prototype methods themselves, including the laziness contract.

## Work shape

**Heuristics §IV classification**: D (Runtime abstract-op semantics) at the Iterator-helpers surface.

Two distinct substrate moves:

1. **`Iterator.from` lazy wrapper**: replace eager-drain-then-array-iterator with a true wrapper object holding `__wrapped_iter` engine sentinel. `next` forwards to inner.next; `return` forwards to inner.return (silent if missing). Preserves the source's IteratorClose semantics across the helper chain.

2. **Per-method interleave**: every helper (map/filter/take/forEach/some/every/find/reduce/toArray) operates on `this` directly via repeated `next` calls instead of eager drain. Per-method abrupt-completion behavior:
   - `forEach`/`reduce`/`map`/`filter`/`toArray`: iterate to natural exhaustion; close on cb-throw or non-Object next-result.
   - `some`/`every`/`find`: close on short-circuit (truthy/falsy/found) AND on cb-throw.
   - `take(n)`: close after n elements (short-circuit) AND on next-result violation.

## Apparatus

- **Direct probe**: `/tmp/probe-iter-helpers-audit.js` (8 cells). 8/8 PASS post-rung.
- **Runtime helper**: `crate::intrinsics::iter_close_rt` reused from AFID-EXT 0.

## Methodology

Per Rule 17 segmentation: IPHD-EXT 0 covers Iterator.from + 9 helpers (map, filter, take, forEach, some, every, find, reduce, toArray). Deferred to follow-up rungs:
- **IPHD-EXT 1** (Iterator.prototype.drop): requires lazy implementation since drop returns an iterator that skips n elements lazily.
- **IPHD-EXT 2** (Iterator.prototype.flatMap): nested-iterator close semantics; inner-iter close on outer abrupt completion.
- **IPHD-EXT 3** (true lazy map/filter): currently EXT 0 returns an array-iterator after eagerly mapping/filtering. Per spec these should be lazy iterators that defer cb evaluation. Substrate is similar to Iterator.from's wrapper but with per-element cb application.

## Carve-outs

- **AsyncIterator.prototype helpers**: out of scope (separate proto + async dispatch surface).
- **Iterator.prototype.@@toStringTag** + **drop** + **flatMap**: carry-forward.

## Composes-with

- AFID-EXT 0 (`iter_close_rt` helper)
- IPTD-EXT 1 (helper-tier IteratorClose discipline)
- `apparatus/docs/predictive-ruleset.md` Rule 17 (per-method segmentation), Rule 24 (Pin-Art recurrence at runtime-tier interleave sites)

## Resume protocol

Read seed.md then trajectory.md. IPHD-EXT 0 is the founding rung (Iterator.from wrapper + 9 helpers interleaved). EXT 1+2+3 are carry-forwards.
