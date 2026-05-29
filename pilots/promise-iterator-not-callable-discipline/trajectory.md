# promise-iterator-not-callable-discipline - Trajectory

## 2026-05-29 - PIND-EXT 0 - Phase 0 spawn + Phase 2 probe

### Directive

Helmsman directed R3 (`codex-pop-os-20260529t040708`) to found `promise-iterator-not-callable-discipline` under the iterator-protocol substrate arc. Scope is Phase 0 spawn plus Phase 2 baseline inspection only; no runtime substrate edits are authorized in this rung.

### Phase 0

Locale founded at `pilots/promise-iterator-not-callable-discipline/` using a root-path seed/trajectory pair so locale discovery sees it. The founding coordinate targets the Promise combinator `Symbol.iterator` + `not-callable` cells surfaced by the post-EPSUA sample matrix.

Rule 11 pre-spawn coverage:

- **A1 component A/B**: Promise combinator host paths versus shared iterator-protocol helper semantics.
- **A2 op-set**: `Promise.all`, `Promise.allSettled`, `Promise.race`; GetIterator / `@@iterator` callability; `C.resolve` and thenable callability as adjacent Promise-specific checks.
- **A3 value-domain**: non-callable `@@iterator` values, primitive iterator-method assignments, Promise constructor resolve access, thenable shapes returned by constructor resolve.
- **A4 locals-marshaling**: generated Promise static wrappers into `Runtime::{promise_all_via,promise_all_settled_via,promise_race_via}` with `Runtime::current_this()` and argument slices.
- **A5 emission-shape**: likely runtime helper/path refactor inside `pilots/rusty-js-runtime/derived/src/interp.rs`, not parser/lowering work.

### Phase 2 Baseline

Inspected `pilots/apparatus/test262-categorize/results/2026-05-29/matrix.md` and `categorized.jsonl`.

Named top-10 cells:

- Rank 8: `Promise.race` / `feat:Symbol.iterator;flag:async;not-callable` - 14 rows.
- Rank 9: `Promise.allSettled` / `feat:Promise.allSettled;feat:Symbol.iterator;flag:async;not-callable` - 13 rows.
- Rank 10: `Promise.all` / `feat:Symbol.iterator;flag:async;not-callable` - 13 rows.

The exact named cells total 40 rows. Reason segmentation:

- 19/40 (47.5%) route to Promise static/combinator callsite not-callable or `C.resolve is not callable` shaped failures.
- 18/40 (45.0%) route to `@@iterator` method not-callable rows (`iter-assigned-{false,null,number,string,true,undefined}` across all three combinators).
- 3/40 (7.5%) are symbol-assigned iterator rows currently reported through global Symbol callability noise.

If widened to every Promise.all/allSettled/race row whose data shape contains `not-callable`, the cluster is 58 rows: 37/58 (63.8%) Promise static/callsite not-callable, 18/58 (31.0%) iterator-method not-callable, 3/58 (5.2%) other.

### Sampled Failures

Sampled rows across the named 40-row cluster:

- `built-ins/Promise/all/S25.4.4.1_A3.1_T3.js`: static Promise.all callsite reports `callee is not callable: undefined`.
- `built-ins/Promise/all/invoke-resolve-get-error.js`: `Promise.all: C.resolve is not callable`.
- `built-ins/Promise/all/iter-assigned-false-reject.js`: non-callable boolean `@@iterator` throws synchronously instead of rejecting.
- `built-ins/Promise/all/iter-assigned-null-reject.js`: non-callable null `@@iterator` throws synchronously instead of rejecting.
- `built-ins/Promise/allSettled/invoke-resolve-get-error.js`: `Promise.allSettled: C.resolve is not callable`.
- `built-ins/Promise/allSettled/iter-assigned-number-reject.js`: non-callable number `@@iterator` throws synchronously instead of rejecting.
- `built-ins/Promise/race/S25.4.4.3_A2.2_T3.js`: static Promise.race callsite reports `callee is not callable: undefined`.
- `built-ins/Promise/race/iter-assigned-string-reject.js`: non-callable string `@@iterator` throws synchronously instead of rejecting.
- `built-ins/Promise/race/resolve-non-callable.js`: Promise.race resolve path reports callsite not-callable.

### Runtime Cross-Reference

The generated ECMA wrappers in `pilots/rusty-js-runtime/derived/src/generated.rs` call:

- `Runtime::promise_all_via`
- `Runtime::promise_all_settled_via`
- `Runtime::promise_race_via`

Those implementations live in `pilots/rusty-js-runtime/derived/src/interp.rs` around the Promise combinator block:

- `promise_all_via` gets `C.resolve`, calls `crate::intrinsics::collect_iterable`, then wraps each entry via `promise_resolve` and `.then`.
- `promise_all_settled_via` follows the same shape with resolve/reject element factories.
- `promise_race_via` calls `collect_iterable` before checking `C.resolve`, then chains each resolved entry through `.then`.

The probe indicates the missing substrate may be one layer deeper than the initial locale name suggests: Promise combinators need abrupt completions from `C.resolve`, `@@iterator`, and `then` acquisition routed through the Promise capability rejection path according to the Promise combinator algorithms, not merely a raw IsCallable check.

### C4 Decision

C4 passes for the narrowed named 40-row matrix scope, but it passes with two competing >40% buckets rather than one dominant cause: Promise static/callsite not-callable at 47.5% and iterator-method-not-callable at 45.0%.

The Phase 3 substrate move should therefore be a Pin-Art probe over the shared Promise combinator helper shape before implementation. Recommended move: factor the common Promise.all/allSettled/race iterable acquisition path so abrupt completions from `GetIterator` / non-callable `@@iterator` are captured and routed through `cap_reject`, while preserving the separate `C.resolve` and `.then` callability checks. Do not assume a single line IsCallable fix closes the whole 40-row cluster.

Estimated closure: 1 probe/design rung plus 1 substrate rung for the iterator-method rejection path; a second substrate rung may be needed for the Promise static/callsite `C.resolve`/method-call rows if they remain after the iterator path lands.

## 2026-05-29 - PIND-EXT 1 - Phase 3 design rung

### Directive

Helmsman directed R3 to land a design-only Phase 3 rung before runtime substrate. Constraint: no `interp.rs` or `intrinsics.rs` modifications in this round. Output is `design.md` plus this trajectory entry.

### Design Result

Authored `design.md` to discriminate the two adjacent buckets surfaced in PIND-EXT 0:

- 19/40 (47.5%) Promise combinator callsite / `C.resolve` not-callable.
- 18/40 (45.0%) `@@iterator` method not-callable.
- 3/40 (7.5%) symbol-assigned iterator noise.

The design chooses a staged Promise-local helper rather than changing global `collect_iterable` behavior. The proposed helper catches abrupt completions from `crate::intrinsics::collect_iterable`, rejects the Promise capability through `cap_reject`, and lets the combinator return `capability_promise` instead of throwing synchronously.

### Edit Sites For Next Rung

- `pilots/rusty-js-runtime/derived/src/interp.rs::promise_all_via`
- `pilots/rusty-js-runtime/derived/src/interp.rs::promise_all_settled_via`
- `pilots/rusty-js-runtime/derived/src/interp.rs::promise_race_via`
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::collect_iterable` remains unchanged for the first substrate rung; wrap it from Promise code instead.

### Rung Plan

Recommended next substrate rung: close the `@@iterator` method not-callable rejection path first. It has the clearest Promise-combinator spec shape and should cover the 18 exact `iter-assigned-{false,null,number,string,true,undefined}` rows across `Promise.all`, `Promise.allSettled`, and `Promise.race`.

Predicted yield:

- Rung 4a: up to +18 PASS on the named 40-row cluster by routing iterator-acquisition abrupt completions through `cap_reject`.
- Rung 4b: up to +19 PASS if the static/`C.resolve` bucket proves to require the same reject-through-capability treatment.
- Rung 4c: 0-3 PASS for symbol-assigned iterator residuals if they remain.

Estimated closure: two substrate rungs, with a possible third cleanup rung for symbol residuals.

## 2026-05-29 - PIND-EXT 2 - Rung 4a Promise-local iterable rejection

### Directive

Helmsman directed R3 to land the Phase 3 design's first substrate rung: add a Promise-local wrapper around `crate::intrinsics::collect_iterable` for `Promise.all`, `Promise.allSettled`, and `Promise.race`, preserving global `collect_iterable` behavior.

### Substrate Move

Added `Runtime::promise_collect_iterable_or_reject` in `pilots/rusty-js-runtime/derived/src/interp.rs`. The helper calls the existing eager `collect_iterable`; on JS-thrown values or catchable engine-side TypeError/RangeError/ReferenceError/SyntaxError it constructs the corresponding rejection value, calls the Promise capability reject function, and returns `None` so the combinator returns the capability promise rather than throwing synchronously.

Wired the helper into:

- `Runtime::promise_all_via`
- `Runtime::promise_all_settled_via`
- `Runtime::promise_race_via`

`Promise.race` now reads and validates `C.resolve` before iterable collection, matching the order already used by `Promise.all` and `Promise.allSettled`. The shared `crate::intrinsics::collect_iterable` function is unchanged, so non-Promise consumers retain synchronous abrupt-completion behavior.

### Measurement

Build gate:

- `cargo build --release --bin cruft -p cruftless`: PASS

Targeted test262 measurement against `/home/jaredef/test262`:

- Named 40-row PIND cluster after the rung: 33 PASS / 7 FAIL.
- Bucket B exact rows, `iter-assigned-{false,null,number,string,true,undefined}-reject.js` across `Promise.all`, `Promise.allSettled`, and `Promise.race`: 18/18 PASS, +18 against the Phase 2 failure matrix.
- Symbol-assigned iterator residual rows: 3/3 PASS; the helper also routed this path through capability rejection, so the predicted Rung 4c is likely unnecessary.
- Adjacent same-shape `iter-arg-is-{false,number,true}-reject.js` rows plus two static-callsite rows also PASS under the named matrix, giving broader named-cluster movement than the conservative design predicted.
- Remaining named failures: 7/40, dominated by `C.resolve is not callable` plus `Promise.allSettled/iter-arg-is-poisoned.js`. These remain outside Rung 4a's iterator-acquisition wrapper and justify Rung 4b.

Adjacent regression smoke:

- PASS: `Promise.all/resolve-non-thenable.js`
- PASS: `Promise.all/iter-arg-is-string-resolve.js`
- PASS: `Promise.allSettled/resolves-to-array.js`
- PASS: `Promise.allSettled/resolved-all-fulfilled.js`
- PASS: `Promise.allSettled/resolved-all-mixed.js`
- PASS: `Promise.race/S25.4.4.3_A4.1_T1.js`
- PASS: `Promise.race/iter-arg-is-string-resolve.js`

### Finding

PIND-EXT 2 confirmed the Phase 3 factoring but widened its observed yield: `iter-assigned-symbol` and `iter-arg-is-*` rows were not separate enough to require a third cleanup rung. The remaining actionable coordinate is the Promise constructor resolve/callability bucket: route spec-required abrupt completions from `C.resolve` lookup/callability through capability rejection without changing static-method callsite errors that should remain synchronous.

## 2026-05-29 - PIND-EXT 3 - Rung 4b C.resolve closure

### Directive

Helmsman directed R3 to close the remaining `C.resolve` not-callable/getter-abrupt residuals in `Promise.all`, `Promise.allSettled`, and `Promise.race`. Constraint: preserve Rung 4a's `promise_collect_iterable_or_reject` helper and leave global `crate::intrinsics::collect_iterable` unchanged.

### Substrate Move

Updated only the combinator-local `C.resolve` retrieval and callability checks:

- `Promise.all`, `Promise.allSettled`, and `Promise.race` now retrieve `resolve` with accessor-aware `spec_get`.
- If the `resolve` getter throws a JS value, the combinator calls the capability reject function with that exact value and returns the capability promise.
- If the retrieved `resolve` value is not callable, the combinator constructs a TypeError rejection reason, calls capability reject, and returns the capability promise.

Rung 4a's `Runtime::promise_collect_iterable_or_reject` is unchanged, and `crate::intrinsics::collect_iterable` is unchanged.

The test262 runner's async drain also received a narrow apparatus fix: it captures `Promise.resolve.bind(Promise)` before evaluating the test body and uses the captured resolver for post-test microtask draining. Without that, C.resolve tests that intentionally poison global `Promise.resolve` pass at the runtime level but fail the runner after the test body has completed.

### Measurement

Build gate:

- `cargo build --release --bin cruft -p cruftless`: PASS

Targeted test262 measurement against `/home/jaredef/test262`:

- Named 40-row PIND cluster after Rung 4b: 39 PASS / 1 FAIL.
- Bucket A C.resolve residual rows: 6/6 PASS after this rung, +6 against the Rung 4a targeted result.
- Remaining named residual: `built-ins/Promise/allSettled/iter-arg-is-poisoned.js`, which expects an abrupt completion from an accessor on `@@iterator` itself. Current shared `collect_iterable` reads `@@iterator` with raw `object_get`, so the accessor is not invoked. This is not a C.resolve residual and was deliberately left outside Rung 4b's allowed edit surface.

Adjacent regression smoke:

- PASS: `Promise.all/resolve-non-thenable.js`
- PASS: `Promise.all/iter-arg-is-string-resolve.js`
- PASS: `Promise.allSettled/resolves-to-array.js`
- PASS: `Promise.allSettled/resolved-all-fulfilled.js`
- PASS: `Promise.allSettled/resolved-all-mixed.js`
- PASS: `Promise.race/S25.4.4.3_A4.1_T1.js`
- PASS: `Promise.race/iter-arg-is-string-resolve.js`

Two broader thenable-value smoke rows remain pre-existing failures and were not regressions from this rung:

- `Promise.all/resolve-thenable.js`
- `Promise.race/resolve-non-thenable.js`

### Finding

PIND-EXT 3 closes the C.resolve bucket but surfaces one final non-C.resolve Promise iterator-acquisition residual. A follow-up Rung 4c is warranted only if Helmsman widens scope to the `@@iterator` accessor-getter path: either Promise-local acquisition must call `spec_get(..., "@@iterator")`, or global `collect_iterable` must be lifted to accessor-aware GetIterator semantics after auditing non-Promise consumers.
