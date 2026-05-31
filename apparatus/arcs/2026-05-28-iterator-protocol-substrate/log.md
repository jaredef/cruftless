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


## 2026-05-31 — Arc enrollment of session-31 iterator-protocol substrate cascade

Per keeper Telegram 10743. Today's IteratorClose / iterator-protocol substrate cascade originated from the IPTD-EXT 0 Pi-OOM regression diagnosis (Telegram 10651) and expanded through 13 substrate landings across 5 new locales + 1 extended sibling. None were arc-enrolled at landing time. This entry retroactively enrolls them under this arc's roster + records the chapter-close findings.

### Enrolled locales (this session)

| Locale | Role in arc | Sessions ext | Status |
|---|---|---|---|
| `iterator-protocol-throw-discipline` (IPTD) | helper-tier IteratorNext/IteratorClose §7.4.5 + §7.4.9 throw discipline | IPTD-EXT 0 NEGATIVE → IPTD-EXT 1 LANDED | LANDED |
| `iterator-close-emission-sites` (ICES) | bytecode-tier IterClose emission at for-of break/return/throw + §7.4.9 step 4 original-throw preservation | ICES-EXT 1, 2, 3, 3.1 LANDED | LANDED (primary scope closed) |
| `array-from-interleave-discipline` (AFID) | runtime-tier interleave + IteratorClose at Array.from, Set/WeakSet ctors, Object/Map.groupBy | AFID-EXT 0, 1, 2, 3 LANDED | LANDED (primary scope closed) |
| `promise-iteration-interleave-discipline` (PIID) | runtime-tier interleave + IteratorClose at Promise.all/race/any/allSettled + AggregateError prototype | PIID-EXT 0, 1+2+3, 4 LANDED | LANDED (primary scope closed) |
| `iterator-helpers-discipline` (IPHD) | runtime-tier laziness + IteratorClose at Iterator.from + 9 Iterator.prototype helpers | IPHD-EXT 0 LANDED | LANDED (drop/flatMap/true-lazy carry-forward) |

### Cross-arc cross-listing (alphabet-exchange per Doc 731)

- `promise-static-ctx-validation` (PSCV) — Promise capability surface is exercised by PIID-EXT 0/1/2/3 (cap.[[Resolve]], cap.[[Reject]] error routing) but the PSCV locale itself scopes Promise.* this-validation and subclass routing. Properly belongs to the `2026-05-31-promise-capability-spec-conformance` arc (drafted concurrently). Cross-listed here because §7.4.9 abrupt-completion routing through IfAbruptRejectPromise was wired in PCRT/PTHEN extensions inside the PIID interleave loops.
- `well-known-symbol-lookup-helper` (WKSL — embedded in interp.rs, not a standalone locale): Symbol-bucket fall-through at find_getter + Op::GetProp. Cross-cuts iterator-protocol (Symbol-keyed @@iterator getters) and property-access discipline broadly. Candidate for cross-arc listing rather than enrollment.

### Cumulative session yield

Pre-session baseline: test262-sample 84.8% canonical (6920 / 7726 runnable).
Post-session-partial (PIID-EXT 0 mark): test262-sample **89.6%** runnable, **+4.8 pp** measured on 2026-05-31 06:33 sample run (`scripts/test262-sample/results/2026-05-31/`).
Post-full-session (after PIID-EXT 1+2+3, PIID-EXT 4, AFID-EXT 3, IPHD-EXT 0, PSCV/PCEXC/PCRT/PTHEN/PCATCH/PFINALLY/PRESOLVE/ACSP/WKSL): unmeasured at arc-enrollment time; predicted +0 to +2 pp additional from this surface mass.

### Arc-tier findings (Phase 5 chapter-close-inspect)

**Finding ITER-ARC.4** (Pin-Art recurrence at Rule 24 threshold) — the interleave + iter_close_rt + propagate-original pattern lands at **11 runtime-tier intrinsic sites** in this session (Array.from, Set ctor, WeakSet ctor, Promise.race, Promise.all, Promise.allSettled, Promise.any, Object.groupBy, Map.groupBy, Iterator.from, 9 Iterator.prototype helpers). Helper promotion (`interleave_iter_with<F>`) was deferred at each rung pending body-shape variance. At 11+ sites the threshold is unambiguously met; deferred candidate for an arc-level Rule-24 LIFT proposal.

**Finding ITER-ARC.5** (§7.4.9 step 4 originally-correct-becoming-wrong) — ICES-EXT 3 introduced spec divergence (close-throw replacing body-throw) and ICES-EXT 3.1 corrected via nested synthetic try-catch. The substrate's first attempt was a structural mistake exposed only by sibling work at AFID-EXT 0 (which got the same constraint right at the Array.from surface). Codifies a cross-locale design-review primitive: when implementing IteratorClose at a new dispatch site, audit existing IteratorClose sites for the §7.4.9 step 4 preservation pattern before scoping the rung. Candidate predictive-ruleset entry.

**Finding ITER-ARC.6** (Symbol-keyed-getter property bifurcation, sympathetic) — WKSL-EXT 0/1 surfaced the Symbol-bucket vs string-bucket find_getter mismatch as a pre-existing latent gap unmasked by today's audit. The for-of slow-path `GetProp @@iterator` returned Undefined for any getter-installed Symbol property. Auto-fix at Op::GetProp + find_getter levels closes the bytecode emission surface but leaves runtime-engine helpers (`__array_extend`, `collect_iterable`) on the string-bucket path. Cross-arc carry-forward.

### Predictions held / refuted

- Pred-arc-iter.1 (every IteratorClose emission site exists per ECMA §7.4.9): **HELD with extension** — pre-arc roster covered 6 sites (destructuring, for-of, for-in, yield*, primitive→ToObject, IPBR canonical); this session adds Array.from + Set/WeakSet ctor + 4× Promise.* + 2× groupBy + Iterator helpers (11 more). Roster expanded from 6 to 17.
- Pred-arc-iter.2 (abrupt-completion paths route IteratorRecord correctly): **HELD with §7.4.9 step 4 refinement** — original-throw preservation now spec-correct everywhere surveyed (post-ICES-EXT 3.1).
- Pred-arc-iter.3 (single-arc closure does not regress other arcs): **HELD** — full regression sweep across 12 probes preserved at every landing; cargo test 74/0/1 preserved throughout.

### Open sub-locales after this enrollment

- `generator-coroutine-suspension` — pre-session OPEN; not advanced.
- `async-generator-and-for-await-lowering` (AGFA) — pre-session OPEN at 96/100 exemplars; not advanced this session.
- **NEW** `iterator-helpers-discipline` carry-forward: drop(n) lazy, flatMap nested-close, true-lazy map/filter — IPHD-EXT 1/2/3 rungs.

### Arc-tier next move

Chapter remains IN PROGRESS pending generator-coroutine + AGFA + IPHD carry-forwards. Substantial substrate consolidation landed; arc-tier mass approaches close-condition but is not yet at it (the AGFA timeout-shaped yield* coroutine rows + IPHD true-lazy iterators remain open).
