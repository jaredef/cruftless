---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "PIID-EXT 0: rewrite Promise.race per ECMA-262 §27.2.4.5 + PerformPromiseRace as interleaved per-element GetIterator + Promise.resolve + .then chain, with IteratorClose on abrupt completion + error-to-capability-rejection plumbing. Founds locale pilots/promise-iteration-interleave-discipline/. Adds Runtime helpers promise_iter_get_iterator + promise_reject_with_error + promise_race_interleave; the first two are shared across the four Promise.* siblings (carry-forward rungs PIID-EXT 1/2/3 for all/allSettled/any)."
risk_class: substrate
gates_pre:
  test262_full: null
gates_post:
  build: "cargo build --release --bin cruft -p cruftless"
  cargo_test_runtime_lib: "74 / 0 / 1 preserved"
  probe_cells:
    - "race finite [resolved,rejected] -> first wins"
    - "race([]) returns a Promise (still pending)"
    - "race(42) -> reject TypeError (was: sync TypeError throw)"
    - "race(iter next non-Object) -> close + reject TypeError"
    - "race(iter next throw) -> reject with thrown error"
    - "race(generator) -> first yielded wins"
---

## Substrate

`promise_race_via` (interp.rs:5159) replaces the eager `promise_collect_iterable_or_reject` + for-loop with a call to the new `promise_race_interleave` helper. Three new helpers:

- `promise_iter_get_iterator(iter_v) -> Result<ObjectRef, RuntimeError>`: Rust-side GetIterator. ToObject the arg if primitive; resolve @@iterator via PropertyKey::Symbol with fall-through; call with this=arg; require Object result.
- `promise_reject_with_error(cap_reject, err)`: convert RuntimeError to JS error object + call cap_reject. Mirrors the prior error-coercion in `promise_collect_iterable_or_reject`. Factored out for sharing across the four Promise.* siblings.
- `promise_race_interleave(iter, ctor, promise_resolve, cap_resolve, cap_reject)`: per ECMA-262 §27.2.4.5 + PerformPromiseRace. Loop: next; check Object (else close + reject); check .done; read .value; Promise.resolve(value) (close + reject on err); value.then(cap_resolve, cap_reject) (close + reject on err).

~180 LOC total across one file (interp.rs).

## Findings surfaced

- **PIID.1**: synchronous-throw-instead-of-capability-rejection class. Promise.race fixed; siblings (Promise.all/any/allSettled) still synchronously throw on iterator errors instead of rejecting their capability. Surfaces a standing-rule candidate: runtime-tier intrinsics whose return type is a Promise must convert all internal errors to capability rejections.

## Verification

1. Build PASS (~1m 08s).
2. PIID-EXT 0 6-cell probe: 6/6 PASS.
3. cargo test --release -p rusty-js-runtime --lib: 74/0/1 preserved.
4. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8, AFID-EXT 1 7/7.

## Carry-forward

- PIID-EXT 1: Promise.all interleave (Resolve Element closure + remaining counter + values array).
- PIID-EXT 2: Promise.allSettled interleave (settle-this-index closure).
- PIID-EXT 3: Promise.any interleave (Reject Element closure + AggregateError).

Each rung mirrors the same body shape; per-method differences are in the .then-chain construction.

## Composes-With

- AFID-EXT 0 (substrate prefix: `iter_close_rt` + interleave pattern)
- IPTD-EXT 1 (helper-tier discipline source)
- `apparatus/docs/predictive-ruleset.md` Rule 17 (segmentation: Promise.race only), Rule 24 (Pin-Art recurrence at runtime-tier interleave sites)

## Authorization

Per keeper Telegram 10694 ("Continue") authorizing the Promise.* family entry.
