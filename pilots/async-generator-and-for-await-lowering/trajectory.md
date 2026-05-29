# async-generator-and-for-await-lowering — Trajectory

## AGFA-EXT 0 — founding + stratified exemplar suite (2026-05-26)

**Trigger**: LPA-EXT 5 partitioned the 10,839-row `ast-to-bytecode/language-lowering` bucket and identified async-generator / for-await as the cleanest fresh language-lowering candidate. LPA-EXT 6 promoted the repartition procedure into an apparatus-tier algorithm. Keeper then directed continuation on the arc.

**Apparatus established**:

- `seed.md` names the baseline-first locale and redirect conditions.
- `exemplars/exemplars.txt` contains a 100-path deterministic stratified sample:
  - 43 `language/statements/for-await-of`
  - 38 `language/expressions/async-generator`
  - 19 `language/statements/async-generator`
- `exemplars/run-exemplars.sh` runs the suite through the Test262 harness wrapper.

**Baseline**:

- Command:
  `T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 pilots/async-generator-and-for-await-lowering/exemplars/run-exemplars.sh`
- Result:
  `PASS=47 FAIL=53 / 100 (47.0%)`
- Failing surface split:
  - 22 `language/expressions/async-generator`
  - 20 `language/statements/for-await-of`
  - 11 `language/statements/async-generator`

**Initial failure partition**:

- AsyncFromSync / job-continuation ordering is present:
  `async-from-sync-iterator-continuation-abrupt-completion-get-constructor.js`
  reaches `"never reached"` instead of catching after scheduled ticks.
- For-await destructuring has a parser/lowering boundary failure:
  `async-func-decl-dstr-array-rest-nested-array-yield-ident-valid.js`
  reports `Invalid destructuring assignment target in for-in/for-of head`.
- Abrupt completions in destructuring iteration are swallowed:
  several `for-await-of` destructuring cases expect async rejection but resolve.
- Async-generator for-await body execution can fail to enter the iteration path:
  multiple `async-gen-dstr-*-async-*` cases report iteration count `0` where `1`
  is expected.
- Async-generator lexical early-error residue exists:
  `await-as-binding-identifier-escaped.js` expects `SyntaxError` but evaluates.
- Async-generator destructuring abrupt paths also miss expected throws.

**Status**: BASELINED. AGFA-EXT 1 should convert this failure partition into a
first substrate target. Current evidence argues against a single broad lowering
edit: at minimum, split parser early-error/destructuring validity from
AsyncFromSync/job-continuation and async-generator protocol execution.

## AGFA-EXT 1 — for-await next/value await bridge (2026-05-26)

**Target selected**: the measured `for-await-of` failures included a concrete
lowering bug: `Stmt::ForOf.await_` was parsed but discarded by bytecode
generation. The loop always used the synchronous `for-of` result-object path,
so async-generator `.next()` Promise results were read directly for `done` and
`value`.

**Substrate move**:

- `rusty-js-bytecode` now routes `for await` loop `next()` results through the
  existing `__await` helper before reading `done` / `value`.
- `rusty-js-bytecode` also awaits the extracted `value` component before
  assigning it to the loop binding. This is a partial AsyncFromSync bridge,
  not the full wrapper protocol.
- `rusty-js-runtime` now gives async-generator instances
  `%AsyncGeneratorPrototype%` and installs `@@asyncIterator` as identity,
  matching the async-iterator surface expected by `for await`.

**Verification**:

- `cargo check -p rusty-js-bytecode -p rusty-js-runtime` passed with existing
  warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing
  warnings.
- `scripts/diff-prod/run-all.sh` remained `42/42 PASS`.
- Exemplar suite:
  `T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 pilots/async-generator-and-for-await-lowering/exemplars/run-exemplars.sh`
  moved from `PASS=47 FAIL=53 / 100 (47.0%)` to
  `PASS=58 FAIL=42 / 100 (58.0%)`.

**Residual split after move**:

- 22 `language/expressions/async-generator`
- 11 `language/statements/async-generator`
- 9 `language/statements/for-await-of`

**Next**: remaining failures should not be treated as more generic for-await
lowering. The surface has split: async-generator expression/statement protocol
is now dominant, while the reduced for-await residual points at assignment
pattern coverage and full AsyncFromSync abrupt-completion semantics.

## AGFA-EXT 2 — async-generator reserved binding context (2026-05-26)

**Target selected**: the remaining async-generator failures included a compact
parser/static-semantics group where escaped `await` / `yield` identifiers were
accepted in async-generator function code:

- `language/expressions/async-generator/await-as-binding-identifier-escaped.js`
- `language/expressions/async-generator/named-await-as-binding-identifier-escaped.js`
- `language/expressions/async-generator/yield-as-label-identifier-escaped.js`
- `language/statements/async-generator/await-as-binding-identifier-escaped.js`
- related strict-mode `yield` binding cases

**Substrate move**:

- `rusty-js-parser` now tracks async-function context beside the existing
  generator-context flag.
- Function parameter and body parsing now receive the async-context directive
  for async declarations, async expressions, async methods, and async arrows.
- Binding-identifier validation rejects `await` while in async function code
  and rejects `yield` in generator or strict contexts at the shared binding
  target paths.

**Verification**:

- `cargo fmt -p rusty-js-parser` applied.
- `cargo check -p rusty-js-parser` passed with existing warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing
  warnings.
- `scripts/diff-prod/run-all.sh` remained `42/42 PASS`.
- Exemplar suite moved from `PASS=58 FAIL=42 / 100 (58.0%)` to
  `PASS=63 FAIL=37 / 100 (63.0%)`.

**Residual split after move**:

- 19 `language/expressions/async-generator`
- 9 `language/statements/for-await-of`
- 9 `language/statements/async-generator`

**Next**: the cheap parser early-error group is mostly discharged. Remaining
async-generator rows are dominated by destructuring abrupt-completion
propagation and `yield*` async delegation; those are runtime/protocol rungs,
not further parser reserved-word cleanups.

## AGFA-EXT 3 — async-generator body abrupt completion propagation (2026-05-26)

**Target selected**: the residual async-generator rows contained repeated cases
where the generator body failed during eager execution but `.next()` still
fulfilled. This made destructuring abrupt-completion tests and several
`yield*` delegation tests observe successful completion where Test262 expected
throw or rejection.

**Substrate move**:

- `Runtime::call_function` now preserves a generator body `RuntimeError::Thrown`
  as an engine sentinel on the generated iterator object.
- Generator `.next()` consumes that pending error on first observation.
- Async-generator `.next()` rejects the returned Promise with the pending error;
  sync-generator `.next()` throws it directly.
- The existing `__gen_async__` sentinel is read before value iteration so the
  pending-error path chooses the correct completion channel.

This is still a protocol bridge over the current eager generator execution
model. It does not implement suspended generator execution or full async
`yield*` delegation, but it closes the measured swallow-shape without broadening
the parser or bytecode rungs.

**Verification**:

- `cargo check -p rusty-js-runtime` passed with existing warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing
  warnings.
- `scripts/diff-prod/run-all.sh` remained `42/42 PASS`.
- Exemplar suite moved from `PASS=63 FAIL=37 / 100 (63.0%)` to
  `PASS=66 FAIL=34 / 100 (66.0%)`.

**Residual split after move**:

- 19 `language/expressions/async-generator`
- 8 `language/statements/async-generator`
- 7 `language/statements/for-await-of`

**Next**: continue at the runtime/protocol layer. The remaining mass is still
mostly async-generator expression behavior, with `yield*` async delegation and
assignment-pattern / AsyncFromSync abrupt-completion semantics as the likely
next partitions.

## AGFA-EXT 4 — generator pending-error Error-object shaping (2026-05-26)

**Target selected**: after AGFA-EXT 3, the next residual inspection showed a
smaller but coherent error-shape bug. Generator-body abrupt completions from
engine-side `TypeError`, `ReferenceError`, `RangeError`, and `SyntaxError`
were preserved, but converted to strings before async-generator `.next()`
rejected. Test262 rows using `assert.throws(TypeError, ...)` or constructor
identity checks therefore observed `String` instead of the expected Error
object kind.

**Substrate move**:

- The generator pending-error capture path now mirrors the existing try/catch
  conversion used by `run_frame`: catchable engine-side errors are routed
  through `make_error_instance`.
- The async-generator `.next()` rejection now carries a JS Error object with
  the named constructor prototype when the underlying body failure was
  `TypeError`, `ReferenceError`, `RangeError`, or `SyntaxError`.
- Non-catchable internal errors still fall back to diagnostic strings; that
  keeps this rung scoped to the measured catchable-completion shape.

**Verification**:

- `cargo check -p rusty-js-runtime` passed with existing warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing
  warnings.
- `scripts/diff-prod/run-all.sh` remained `42/42 PASS`.
- Exemplar suite moved from `PASS=66 FAIL=34 / 100 (66.0%)` to
  `PASS=68 FAIL=32 / 100 (68.0%)`.

**Residual split after move**:

- 19 `language/expressions/async-generator`
- 8 `language/statements/async-generator`
- 5 `language/statements/for-await-of`

**Next**: do not keep widening this catchable-error bridge blindly. The
remaining failures now cluster around deeper async-generator parameter
destructuring, `yield*` async delegation, and two parser/static early-error
rows (`arguments` binding and escaped `yield` label).

## AGFA-EXT 5 — async-generator inline binding and label early errors (2026-05-26)

**Target selected**: residual inspection after AGFA-EXT 4 isolated two
parser/static rows that were not runtime protocol failures:

- `language/expressions/async-generator/early-errors-expression-binding-identifier-arguments.js`
- `language/expressions/async-generator/yield-as-label-identifier-escaped.js`

Both bypassed the central `parse_binding_identifier` checks added in
AGFA-EXT 2. Function-expression names and labelled-statement labels were
constructed inline, so strict-mode `arguments` and generator-context `yield`
rules did not fire on those paths.

**Substrate move**:

- Function declaration and expression name parsing now applies the same
  strict/generator/async contextual checks as binding identifiers for
  `eval`, `arguments`, `yield`, and `await`.
- Labelled-statement parsing now rejects `yield` in generator or strict
  contexts and `await` in async-function contexts before constructing the
  label node.

**Verification**:

- `cargo check -p rusty-js-parser` passed with existing warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing
  warnings.
- Targeted Test262 rows both passed as negative parse tests.
- `scripts/diff-prod/run-all.sh` remained `42/42 PASS`.
- Exemplar suite moved from `PASS=68 FAIL=32 / 100 (68.0%)` to
  `PASS=70 FAIL=30 / 100 (70.0%)`.

**Residual split after move**:

- 17 `language/expressions/async-generator`
- 8 `language/statements/async-generator`
- 5 `language/statements/for-await-of`

**Next**: the cheap parser/static rows are now discharged from the visible
residual. Remaining movement should return to deeper runtime/lowering
partitions: async-generator parameter destructuring, `yield*` async
delegation, and AsyncFromSync abrupt-completion semantics.

## AGFA-EXT 6 — object destructuring object-coercible guard and async rejection shaping (2026-05-26)

**Target selected**: after AGFA-EXT 5, the residual for-await bucket still
contained two object-binding rows:

- `language/statements/for-await-of/async-func-dstr-let-obj-init-null.js`
- `language/statements/for-await-of/async-func-dstr-var-obj-init-undefined.js`

Both executed `for await` over null/undefined values with an empty object
binding pattern. The lowering emitted no property read for `{}`, so the
required object-coercible check never fired and the async function resolved.

**Substrate move**:

- Object binding and assignment destructuring now emit an explicit
  `__destr_object_check(src)` guard before property/rest lowering.
- The guard throws `TypeError` for `null` and `undefined`, including empty
  object patterns that otherwise perform no `Get`.
- Async-function body-error rejection now mirrors the generator pending-error
  bridge for catchable engine errors, converting `TypeError`, `RangeError`,
  `ReferenceError`, and `SyntaxError` into JS Error objects before rejecting
  the returned Promise.

**Verification**:

- `cargo check -p rusty-js-bytecode` passed with existing warnings.
- `cargo check -p rusty-js-runtime` passed with existing warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing
  warnings.
- The two targeted for-await object-binding rows passed.
- `scripts/diff-prod/run-all.sh` remained `42/42 PASS`.
- Exemplar suite moved from `PASS=70 FAIL=30 / 100 (70.0%)` to
  `PASS=72 FAIL=28 / 100 (72.0%)`.

**Residual split after move**:

- 17 `language/expressions/async-generator`
- 8 `language/statements/async-generator`
- 3 `language/statements/for-await-of`

**Next**: the remaining for-await rows now point at AsyncFromSync abrupt
completion and assignment-pattern parsing. Async-generator residual remains
dominated by parameter destructuring and `yield*` async delegation; do not
re-enter `yield*` without a timeout-safe design because the prior probe hung
the AGFA runner.

## AGFA-EXT 7 — direct-eval declaration fallback UTF-8 boundary guard (2026-05-28)

**Target selected**: current exemplar re-run showed `PASS=79 FAIL=21 / 100`,
with seven parser/static async-generator rows producing blank harness output
rather than JSON. Direct execution showed a runtime panic in
`eval_var_scoped_declarations_fallback`:

```text
start byte index ... is not a char boundary; it is inside '«'
```

The panic was triggered by Test262 harness text containing non-ASCII assertion
quotes. The fallback scanner walked the source by byte offset and called
`keyword_at` at every byte; `keyword_at` then sliced `source[offset..]` even
when `offset` was not a UTF-8 character boundary.

**Substrate move**:

- `keyword_at` now computes `end = offset + keyword.len()` once.
- It returns false unless both `offset` and `end` are valid character
  boundaries and `end <= source.len()`.
- The post-keyword boundary check slices from `source[end..]`.

This keeps the fallback scanner byte-oriented while preventing invalid UTF-8
slice panics. The fix is intentionally runtime-local; it does not broaden into
async-generator `yield*` delegation.

**Verification**:

```text
cargo check -p rusty-js-runtime: PASS (existing warnings)
cargo build --release --bin cruft -p cruftless: PASS (existing warnings)
```

Targeted panic rows now emit JSON PASS as negative SyntaxError tests:

```text
language/expressions/async-generator/await-as-binding-identifier-escaped.js PASS
language/expressions/async-generator/early-errors-expression-binding-identifier-arguments.js PASS
language/statements/async-generator/yield-identifier-strict.js PASS
```

Exemplar suite moved:

```text
AGFA exemplars PRE-EXT 7:  PASS=79 FAIL=21 / 100 (79.0%)
AGFA exemplars POST-EXT 7: PASS=86 FAIL=14 / 100 (86.0%)
```

Residual split after move:

- 8 `language/expressions/async-generator`
- 3 `language/statements/async-generator`
- 3 `language/statements/for-await-of`

**Gate**: after the sidecar root was corrected, `scripts/diff-prod/run-all.sh`
completed against `/home/jaredef/Developer/cruftless-sidecar/results/diff-prod`
at `PASS=61 FAIL=51 / 112`.

**Next**: remain on AGFA only if targeting the reduced for-await AsyncFromSync
tail or a timeout-safe async-generator `yield*` probe. The UTF-8 scanner panic
is closed.

## AGFA-EXT 8 — for-await assignment-pattern member target admission (2026-05-28)

**Target selected**: after AGFA-EXT 7, the for-await residual still included one
parser-shaped row:

- `language/statements/for-await-of/async-func-decl-dstr-array-rest-nested-array-yield-ident-valid.js`

The source uses an expression-headed for-await assignment pattern:

```js
for await ([...[x[yield]]] of [[86]]) { ... }
```

This is not a binding pattern: the leaf target is a member expression. It is,
however, a valid destructuring assignment target in the `for await (... of ...)`
assignment-head grammar. The parser was forcing every array/object head through
`expr_to_binding_pattern`; member-expression leaves made that conversion return
`None`, and the pattern literal path reported `Invalid destructuring assignment
target in for-in/for-of head`.

**Substrate move**:

- Added a conservative `is_valid_assignment_pattern_expr` validator for
  expression-headed assignment patterns.
- When an array/object for-in/of head cannot become a binding pattern but is a
  valid assignment pattern, route it to `ForBinding::AssignmentTarget(e)` rather
  than SyntaxError.
- Kept the existing invalid-pattern rejection for malformed pattern literals
  (rest-not-last, trailing comma after spread, invalid leaves).

**Verification**:

- `cargo fmt -p rusty-js-parser` applied.
- `cargo check -p rusty-js-parser` passed with existing warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing warnings.
- Targeted for-await row now PASSes.
- AGFA exemplar suite moved from `PASS=86 FAIL=14 / 100 (86.0%)` to
  `PASS=87 FAIL=13 / 100 (87.0%)`.
- `scripts/diff-prod/run-all.sh` completed at `PASS=61 FAIL=51 / 112`.

**Finding AGFA.8**: for-await assignment heads need a parser distinction
between BindingPattern and AssignmentPattern. A cover array/object literal with
member-expression leaves is invalid as a binding pattern but valid as an
assignment pattern; rejecting it at parse time hides downstream for-await
assignment semantics.

**Residual split after move**:

- 8 `language/expressions/async-generator`
- 3 `language/statements/async-generator`
- 2 `language/statements/for-await-of`

**Next**: the remaining for-await rows are AsyncFromSync abrupt-completion and
async-generator destructuring value propagation. The dominant mass remains
`yield*` async delegation; enter that only with a timeout-safe design.


## AGFA-EXT 9 — AsyncFromSync value bridge fast-path escape hatch (2026-05-28)

**Target selected**: the remaining non-`yield*` for-await row:

- `language/statements/for-await-of/async-from-sync-iterator-continuation-abrupt-completion-get-constructor.js`

Pre-move behavior skipped the AsyncFromSync value continuation entirely for
array-backed `for await (var x of [p])`: `ForOfFastNext` wrote the raw promise
into the loop binding and jumped directly to the body. The row therefore logged
`never reached` instead of taking the catch path.

**Substrate move**:

- Disabled the fused `ForOfFastNext` path when `await_` is true so for-await
  heads always execute the slow path's iterator-result await and value await
  bridge.
- Split the existing `__await` helper body into `await_settled_value` and added
  an internal `__async_from_sync_value` helper for the value leg.
- `__async_from_sync_value` performs the observable `constructor` get on
  internal Promise values before delegating to `await_settled_value`, matching
  the abrupt-completion site in AsyncFromSyncIteratorContinuation.

**Verification**:

- `cargo check -p rusty-js-runtime -p rusty-js-bytecode` passed with existing
  warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing warnings.
- Target row behavior moved from:
  `Actual [start, never reached, tick 1, tick 2] ...`
  to:
  `Actual [start, tick 1, catch, tick 2] ...`.
- AGFA exemplars remained neutral at `PASS=87 FAIL=13 / 100 (87.0%)`.
- `scripts/diff-prod/run-all.sh` completed at `PASS=61 FAIL=51 / 112`.

**Finding AGFA.9**: the array fast path was hiding all for-await value-await
semantics. After routing through the slow path, the remaining target gap is not
iterator selection or constructor observation; it is promise-job ordering. The
catch is delivered one microtask too early (`catch` before `tick 2`). Closing
this row requires an async-function rejection scheduling step rather than more
for-of parser/lowering work.

**Residual split after move**:

- 8 `language/expressions/async-generator`
- 3 `language/statements/async-generator`
- 2 `language/statements/for-await-of`

**Next**: either implement timeout-safe promise-job scheduling for the
AsyncFromSync abrupt-completion rejection, or leave AGFA for a different
iterator-protocol locale. Avoid `yield*` async delegation without a timeout-safe
design.

## AGFA-EXT 10 — async-generator yield* protocol selection and error propagation (2026-05-28)

**Target selected**: the larger async-generator `yield*` residual cluster after
AGFA-EXT 9 left the suite at `PASS=87 FAIL=13 / 100`.

Representative pre-move rows:

- `language/expressions/async-generator/yield-star-getiter-async-returns-number-throw.js`
- `language/expressions/async-generator/yield-star-getiter-sync-returns-number-throw.js`
- `language/expressions/async-generator/yield-star-next-not-callable-undefined-throw.js`

The compiler lowers `yield* expr` to `__yield_delegate__(expr)`. The helper was
still a v1 eager collector: it always looked up `@@iterator`, used raw
`object_get` instead of accessor-aware `Get`, and swallowed iterator protocol
errors by breaking the loop. Async-generator `yield*` therefore missed
`@@asyncIterator`, skipped abrupt getters, and converted protocol violations
into later synthetic failures.

**Substrate move**:

- Added a parallel `Runtime::gen_async_stack` beside `gen_yields_stack`, pushed
  on generator entry and popped on exit, so helper code can distinguish async
  generator delegation from sync generator delegation without changing the
  bytecode ABI.
- Updated `__yield_delegate__` to use `@@asyncIterator` first for async
  generators, falling back to `@@iterator` only when absent.
- Routed iterator method/`next`/result property reads through `read_property`
  so getters are observable.
- Propagated iterator protocol errors instead of swallowing them as loop
  termination.

**Verification**:

- `cargo build --release --bin cruft -p cruftless` passed with existing
  warnings.
- Targeted TypeError rows now PASS:
  - `yield-star-getiter-async-returns-number-throw.js`
  - `yield-star-getiter-sync-returns-number-throw.js`
  - `yield-star-next-not-callable-undefined-throw.js`
- AGFA exemplar suite moved from `PASS=87 FAIL=13 / 100 (87.0%)` to
  `PASS=91 FAIL=9 / 100 (91.0%)`.
- `scripts/diff-prod/run-all.sh` completed at the existing baseline
  `PASS=61 FAIL=51 / 112`.

**Residual split after move**:

- 5 `language/expressions/async-generator`
- 2 `language/statements/async-generator`
- 2 `language/statements/for-await-of`

Two remaining yield-star rows still produce no JSON under the bounded
per-row loop and should be treated as timeout-shaped until the delegation
coroutine model is deeper than this eager-collector helper.

## AGFA-EXT 11 — yield* well-known symbol accessors and terminal value get (2026-05-28)

**Target selected**: the remaining abrupt-completion `yield*` rows where the
helper still missed accessor throws and terminal `value` throws:

- `language/expressions/async-generator/named-yield-star-getiter-async-get-abrupt.js`
- `language/expressions/async-generator/named-yield-star-getiter-sync-get-abrupt.js`
- `language/statements/async-generator/yield-star-getiter-async-undefined-sync-get-abrupt.js`
- `language/expressions/async-generator/named-yield-star-next-call-value-get-abrupt.js`

After AGFA-EXT 10, direct JS `obj[Symbol.asyncIterator]` accessor reads were
correct, but `__yield_delegate__` still probed only the transitional string
aliases (`"@@asyncIterator"` / `"@@iterator"`). Computed well-known-symbol
accessors installed under the actual Symbol key were therefore skipped and the
generator body continued to the sentinel `Test262Error`.

The terminal-value row exposed a second ordering gap: the helper read `done`
and broke before reading `value`, but `yield*` must perform `IteratorValue` on
the terminal result.

**Substrate move**:

- Added `Runtime::read_property_pk`, a PropertyKey-aware accessor-dispatching
  get that mirrors `read_property` for symbol keys.
- Taught `__yield_delegate__` to resolve `Symbol.asyncIterator` /
  `Symbol.iterator` from the global `Symbol` constructor and use
  `read_property_pk`, with the old string aliases as fallback.
- Read iterator result `value` before checking `done` so terminal value getters
  are observable.

**Verification**:

- `cargo build --release --bin cruft -p cruftless` passed with existing
  warnings.
- The four targeted abrupt rows now PASS.
- AGFA exemplar suite moved from `PASS=91 FAIL=9 / 100 (91.0%)` to
  `PASS=96 FAIL=4 / 100 (96.0%)`.
- `scripts/diff-prod/run-all.sh` stayed at the existing baseline
  `PASS=61 FAIL=51 / 112`.

**Residual split after move**:

- 2 `language/expressions/async-generator` timeout-shaped rows
- 1 `language/statements/async-generator` timeout-shaped row
- 1 `language/statements/for-await-of` async-generator destructuring resume row

**Next**: the remaining yield-star rows are coroutine/thenable scheduling
shape, while the for-await row still requires async-generator resume values to
flow through destructuring initializer `yield`.


## AGFA-EXT 10 — async-function rejection job ordering (2026-05-28)

**Target selected**: continue the AGFA-EXT 9 AsyncFromSync row after the fast-path
bypass and constructor-get helper moved behavior from `never reached` to the
correct rejection path but with the catch reaction one microtask too early.

Target row:

- `language/statements/for-await-of/async-from-sync-iterator-continuation-abrupt-completion-get-constructor.js`

**Substrate move**:

- Async function abrupt completion now settles the returned promise through an
  `AsyncFunctionReject` microtask rather than rejecting immediately at call
  return.
- This preserves the existing synchronous body execution model while matching
  the Promise job ordering required by the AsyncFromSync abrupt-completion row:
  an already-queued `.then` job runs first, its chained `.then` is appended,
  and only then does the async-function rejection enqueue the caller's catch
  reaction.

**Verification**:

- Targeted AsyncFromSync row now PASSes.
- `cargo check -p rusty-js-runtime -p rusty-js-bytecode` passed with existing
  warnings.
- `cargo build --release --bin cruft -p cruftless` passed with existing warnings.
- AGFA exemplars moved from `PASS=87 FAIL=13 / 100 (87.0%)` to
  `PASS=95 FAIL=5 / 100 (95.0%)`.
- `scripts/diff-prod/run-all.sh` completed at `PASS=61 FAIL=51 / 112`.

**Residual split after move**:

- `language/statements/for-await-of/async-gen-decl-dstr-array-rest-nested-obj-yield-expr.js`
  still fails on async-generator yield/resume value propagation.
- Three checked residuals timeout in async-generator `yield*` delegation:
  `named-yield-star-next-then-non-callable-string-fulfillpromise.js`,
  `yield-star-async-next.js`, and
  `yield-star-next-then-non-callable-number-fulfillpromise.js`.
- The exemplar runner reports one additional async-generator failure in the same
  residual family; treat the remaining AGFA mass as async-generator suspension /
  `yield*` protocol, not for-await parser/lowering.

**Finding AGFA.10**: after the for-await slow-path bridge is in place,
AsyncFromSync abrupt completion is governed by async-function promise settlement
ordering. Immediate rejection of the returned async-function promise is too
early; one queued async-function rejection job gives the caller's catch reaction
the correct relative position in the Promise job queue.

**Next**: leave AGFA unless entering async-generator suspension with a
strict timeout-safe design. The remaining rows are no longer narrow for-await
lowering fixes.
