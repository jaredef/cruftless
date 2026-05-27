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
