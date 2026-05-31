# promise-await-dictionary-discipline (PAKD) — Seed

## Telos

```
runtime/promise-static :: E3/intrinsic-object:ecma-262-stage1 :: dictionary-iteration :: per-key-resolve-with-null-proto-result
```

Implement the await-dictionary Stage 1 proposal: `Promise.allKeyed(obj)` and `Promise.allSettledKeyed(obj)`. Input is an Object (own enumerable string keys); output is a null-proto Object with the same keys mapped to resolved values (allKeyed) or `{status, value/reason}` entries (allSettledKeyed).

## Origin

Founded 2026-05-31 per keeper Telegram 10755 authorizing Stage 1 proposal triage following post-session-31 test262 sample (89.6% → 90.8%, Promise cluster compressed 190 → 103 of which 35 were Promise.allKeyed/allSettledKeyed unimplemented).

## Scope

35 test262 cells. Locale is single-rung (PAKD-EXT 0); follow-up rungs only if test262 surfaces spec edge cases.

## Carve-outs

- Iterable input (not Object) — out of scope; spec requires Object.
- Subclass routing — uses NewPromiseCapability(C) like the existing Promise.* statics; no special handling.

## Composes-with

- `promise-iteration-interleave-discipline` (PIID; sibling Promise.* iteration shape)
- `promise-static-ctx-validation` (PSCV; shares this-Object check + C.resolve resolution)
- Arc: `2026-05-31-promise-capability-spec-conformance` (PCSC)
