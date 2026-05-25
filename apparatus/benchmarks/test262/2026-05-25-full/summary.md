# test262 Full Suite — 2026-05-25

## Run

- Repository: `/home/jaredef/Developer/cruftless`
- Cruft binary: `target/release/cruft`
- Test262 root: `/home/jaredef/test262`
- Test262 commit: `7e115f46ac64340827d505fa928ad436cb7ba5a6`
- Runner: `legacy/host-rquickjs/tests/test262/runner.mjs`
- Harness: `/home/jaredef/test262/harness`
- Parallelism: 8
- Per-test timeout: 10 seconds

## Results

- Suite paths: 53,289
- Results emitted: 53,289
- PASS: 22,812
- FAIL: 23,870
- SKIP: 6,341
- TIMEOUT/no-output: 265
- Malformed/other result lines: 1
- Runnable pass rate: 48.6% (22,812 / 46,948)

## Notes

One result line was the literal value `2`, not a result object with a `status` field.
It is counted as malformed/other in this summary.

The full suite was run after installing upstream TC39 test262 into `/home/jaredef/test262`.
The repository's curated sample lane remains separate under `scripts/test262-sample/`.

