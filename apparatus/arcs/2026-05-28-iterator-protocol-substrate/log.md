# Iterator Protocol Substrate Arc — Event Log

Append-only per `apparatus/docs/arc-as-coordinate.md` §F.

---

## 2026-05-28 — Arc scaffolded

Per keeper directive Telegram 10158 + Plan-agent back-fit. Arc spawned to subsume nine pre-existing top-level iterator-protocol locales. Roster + telos + close-condition recorded in arc.md.

Pure arc scaffolding. Per Doc 745 candidate's SIPE-T fractal correspondence, this scaffold is the arc-tier Phase 1 emission for the enrolled roster.

Next event candidate: per-locale chapter-close in one of the OPEN locales (generator-coroutine-suspension or AGFA async-iterator).

## 2026-05-28 — AGFA-EXT 7 direct-eval fallback scanner UTF-8 guard

- Continued under `async-generator-and-for-await-lowering/` after selecting
  this arc as the non-conflicting active arc.
- Current AGFA baseline was `PASS=79 FAIL=21 / 100`.
- Seven async-generator parser/static rows produced blank harness output; direct
  execution showed a panic in `eval_var_scoped_declarations_fallback` when the
  byte scanner called `keyword_at` inside a multibyte `«` character from the
  Test262 harness text.
- Fixed `keyword_at` to reject non-character-boundary offsets before slicing.
- Targeted panic rows now report JSON PASS as negative SyntaxError tests.
- AGFA exemplar suite moved to `PASS=86 FAIL=14 / 100`.
- `cargo check -p rusty-js-runtime` and `cargo build --release --bin cruft -p
  cruftless` passed with existing warnings.
- After the sidecar root was corrected, `scripts/diff-prod/run-all.sh`
  completed at `PASS=61 FAIL=51 / 112`.

## 2026-05-28 — AGFA-EXT 8 for-await assignment-pattern parser distinction

Continued AGFA residual reduction under the iterator-protocol arc. The selected
row was the expression-headed for-await pattern `[...[x[yield]]]`, where the
leaf is a member assignment target. Parser-side conversion to BindingPattern
correctly failed, but the for-await assignment-head grammar permits an
AssignmentPattern.

Added a conservative assignment-pattern validator for expression-headed
for-in/of array/object heads and routed valid non-binding patterns to
`ForBinding::AssignmentTarget`. Targeted row now PASSes; AGFA suite moved from
`86/100` to `87/100`; diff-prod remained `61/112`.

Arc finding: for-await sits at the parser/lowering alphabet exchange: the same
cover array/object syntax must be classified as BindingPattern for declaration
heads and AssignmentPattern for expression heads. Member-target leaves are the
observable discriminator.


## 2026-05-28 — AGFA-EXT 9 for-await fast-path bypass and AsyncFromSync constructor get

Continued the remaining for-await AsyncFromSync abrupt-completion row. The
array-backed `for await (var x of [p])` case was still taking `ForOfFastNext`,
which bypassed both await bridges and stored the raw Promise before entering the
body.

Disabled `ForOfFastNext` for `await_` for-of heads and added a dedicated
`__async_from_sync_value` internal helper for the value continuation. The helper
observes the Promise `constructor` getter before delegating to the existing
await-settling logic.

Result: the row no longer reaches the loop body, but remains FAIL because the
catch handler is scheduled before `tick 2` rather than after it. AGFA exemplars
were neutral at `87/100`; diff-prod remained `61/112`. Arc finding: the next
AsyncFromSync step is promise-job ordering, not iterator fast-path selection.

## 2026-05-28 — AGFA-EXT 10 async-generator yield* protocol delegation

Continued AGFA against the `yield*` residual mass with bounded per-row harness
runs. The v1 `__yield_delegate__` helper still treated all generator delegation
as sync iteration: it read `@@iterator`, skipped accessor semantics via raw
`object_get`, and swallowed iterator protocol errors as ordinary loop exits.

Added a `gen_async_stack` alongside the generator yields stack so the helper can
distinguish async-generator delegation. `__yield_delegate__` now prefers
`@@asyncIterator` for async generators, falls back to `@@iterator` only when
absent, reads protocol properties through `read_property`, and propagates
protocol TypeErrors instead of breaking the loop.

Result: representative TypeError rows for non-object async iterator results,
non-object sync fallback results, and non-callable `next` now PASS. AGFA moved
from `87/100` to `91/100`; residual split is 5 expression async-generator, 2
statement async-generator, and 2 for-await rows. Diff-prod stayed at the known
`61/112` baseline. Two remaining yield-star rows are still timeout-shaped under
the bounded failure-list loop, so deeper delegation coroutine work remains
separate from this eager-helper correction.

## 2026-05-28 — AGFA-EXT 11 yield* well-known symbol accessors and terminal value

Continued the async-generator `yield*` residuals after AGFA-EXT 10. Direct JS
`obj[Symbol.asyncIterator]` accessor reads were already correct, but
`__yield_delegate__` still looked up only the transitional string aliases. That
missed computed well-known-symbol accessors and let the generator body continue
to the sentinel `Test262Error` instead of rejecting with the thrown reason.

Added `Runtime::read_property_pk` for PropertyKey-aware accessor dispatch and
changed `__yield_delegate__` to resolve `Symbol.asyncIterator` / `Symbol.iterator`
from the global Symbol constructor before falling back to `"@@..."` strings.
Also moved iterator-result `value` reading before the `done` break so terminal
`IteratorValue` abrupt completions are observable.

Result: four abrupt-completion yield-star rows now PASS and AGFA moved from
`91/100` to `96/100`; diff-prod remained at `61/112`. Residual AGFA is now four
rows: three timeout-shaped yield-star coroutine/thenable cases and one
for-await async-generator destructuring-resume case.


## 2026-05-28 — AGFA-EXT 10 async-function rejection job ordering

Closed the AsyncFromSync abrupt-completion row that AGFA-EXT 9 had reduced to a
microtask-ordering failure. Async function abrupt completion now rejects the
returned promise via an `AsyncFunctionReject` microtask instead of settling it
immediately at call return.

Targeted Test262 row now PASSes. AGFA exemplars moved from `87/100` to `95/100`;
diff-prod remained `61/112`. Arc finding: the remaining AGFA mass is
async-generator suspension / `yield*` protocol, while the for-await
AsyncFromSync continuation is now past parser/lowering and constructor-get
observation.
