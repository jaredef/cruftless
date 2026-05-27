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
