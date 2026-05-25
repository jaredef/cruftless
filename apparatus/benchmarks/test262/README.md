# test262 Benchmarks

This directory records Test262 benchmark runs as apparatus artifacts.

Each dated run directory should include:

- `summary.md` — human-readable run metadata and tallies.
- `paths.txt` — absolute upstream test paths used for the run.
- `results.jsonl` — one JSON-ish result record per path as emitted by the runner.

The current runner is `legacy/host-rquickjs/tests/test262/runner.mjs`, invoked through
the `cruft` release binary with `T262_TEST_PATH` and `T262_HARNESS_DIR` set per test.

