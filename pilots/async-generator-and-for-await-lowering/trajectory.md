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
