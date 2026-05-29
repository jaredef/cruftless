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
