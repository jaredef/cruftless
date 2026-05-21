# Parity baseline log

Per-date snapshots of the canonical parity sweep
(`legacy/host-rquickjs/tools/parity-measure.sh` against the
`parity-top100.txt` basket of 119 packages).

| Date | Pass | Total | Parity | Delta |
|---|---|---|---|---|
| 2026-05-13 | 105 | 119 | 88.2% | (baseline) |
| **2026-05-21** | **110** | **119** | **92.4%** | **+4.2 pp** |

The 2026-05-21 reading was taken after the rename of `host-v2/` →
`cruftless/` and the landing of PM + caps + JIT chapters. The gain
of 9 cluster packages (yup, io-ts, dayjs, date-fns, neverthrow,
jsonc-parser, ora, yargs, fp-ts) appears to be incidental to the
substrate work that closed the rusty-js-runtime + rusty-js-parser
gaps tonight.

Current 9 failures (2026-05-21):
- **runtypes** — load crash (TypeError on dynamic import)
- **arktype** — load crash
- **superstruct** — 1 missing export (52 vs 51, missing `default`)
- **got** — load crash (asyncIterator prototype null at `@sec-ant/readable-stream`)
- **node-fetch** — 2 missing exports (16 vs 14: `fetch`, `FetchBaseError`)
- **superagent** — load crash (cuid2 `Object.values` missing)
- **entities** — TIMEOUT (cruftless hangs)
- **get-stream** — load crash
- **enquirer** — 21 extra exports (64 vs 43, over-synthesis)

Decomposition:
- Class A (missing exports): superstruct, node-fetch — 2 packages
- Class B (extra exports): enquirer — 1 package
- Class C (load-time crash on dep): runtypes, arktype, got, superagent, get-stream — 5 packages
- Class D (hang): entities — 1 package

The Class C and D failures cluster on a small set of transitive
runtime-semantic gaps (asyncIterator prototype, Object.values missing,
hang condition). Closing one Class C gap probably retires multiple
top-level packages.

To reproduce:

```
RB_BIN=/home/jaredef/rusty-bun/target/release/cruftless \
  nice -n 19 ionice -c3 \
  bash legacy/host-rquickjs/tools/parity-measure.sh \
       legacy/host-rquickjs/tools/parity-top100.txt \
       out.json
```

Wall time: ~25 minutes on Pi-5 under nice -n 19.
