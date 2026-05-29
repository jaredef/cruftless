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
