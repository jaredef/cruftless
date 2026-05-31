# top500-fast-residual-survey - Trajectory

## 2026-05-29 - Round 13b - Founding remeasure + residual clustering

Founded per helmsman directive `1a067a1b-93ad-4db2-98b9-9221492f3740`.

### Worktree + build

- Worktree: `/home/jaredef/Developer/cruftless-r4`
- Branch: `resolver-r4-main`
- Rebase: `git fetch origin main && git rebase origin/main` PASS
- Build: `cargo build --release --bin cruft -p cruftless` PASS

### Scope-down justification

The directive's original parity sandbox path `/media/jaredef/T7/rusty-bun/parity-sandbox/` was not mounted in this Codex filesystem namespace. Per the directive's allowed scope-down path, the measurement used the stock parity harness with a fresh local sandbox at `/tmp/parity-sandbox`.

`bun` was also absent from `PATH` at session start, so this round installed Bun locally to `~/.bun/bin/bun` before running the parity harness.

### Sample

Nineteen packages:

- CITPT residuals + controls:
  `arktype`, `redis`, `stylelint`, `prettier`, `csso`, `rehype`, `puppeteer-core`, `svgo`, `config`
- broader prior-cluster probes:
  `mocha`, `chai`, `mathjs`, `mongoose`, `exceljs`, `readable-stream`, `events`, `winston`, `proj4`, `decimal.js-light`

### Result

- 7 PASS
- 12 FAIL
- 0 SKIP
- parity: 36.8%

Raw artifact copied to:
`/home/jaredef/Developer/cruftless-sidecar/results/top500-fast-residual-survey-2026-05-29T215950Z/round13b-r4-results.json`

### Dominant cluster

The largest coherent family is not a hard load failure. It is the namespace-shape-diff family:

- `readable-stream`
- `events`
- `winston`
- `proj4`
- `decimal.js-light`

These five packages all import successfully under cruft, but the exported namespace shape diverges from Bun. That makes CNSDR the next caller-leak by count in this representative sample.

### Secondary clusters

- filesystem / package-path assumptions:
  `redis`, `stylelint`
- prototype / host-method completeness:
  `chai`, `exceljs`
- single-package residuals:
  `arktype` parser/generic resolution, `mongoose` toStringTag/get path, `mathjs` runtime panic in sort comparator

### Recommendation

Route the next substrate move to `pilots/cjs-ns-shape-diff-residual/`, not back to CITPT. CITPT controls held: 6 of the 9 CITPT packages in this sample passed (`prettier`, `csso`, `rehype`, `puppeteer-core`, `svgo`, `config`), and the remaining CITPT residuals stayed in their already-known non-TDZ families.

---

## TFRS-EXT — full top500 sweep on fresh main (2026-05-31)

**Trigger**: keeper directive to run the full top500 npm parity sweep on a freshly rebuilt cruft against current `main`.

**Apparatus**: `legacy/host-rquickjs/tools/parity-measure.sh` over `parity-top500.txt` (1,026 packages), Bun 1.3.14 as the oracle, `PARITY_SANDBOX` on the sidecar. Raw artifact: `legacy/host-rquickjs/tools/parity-results-top500-2026-05-31.json` (also at sidecar `results/top500/top500-2026-05-31T064711Z.json`).

**Headline**: 1,026 total. PASS 813, MATCH_OK_ERR_BOTH 44, FAIL 114, TIMEOUT 3, SKIP 52 (install failures). **Parity 88.0%** (857 / 974 runnable); 83.5% counting install-failures in the denominator.

**FAIL clusters (114)**: cruft empty/crash 19 (mathjs, exceljs, iconv, keccak, ava, biome); one-off cruft-ERR 18; both-OK shape-diff 31 (15 rb>bun, 8 same-count, 8 rb<bun); cannot-read-property 11 (mnemonist, brotli, aws-sdk); CJS-wrapper parse 7 (typeorm, 3x rollup-plugin, tsdown); callee-not-callable 7 (chai, @octokit/request, pm2, ts-loader); X-is-not-defined 5 (node-forge, apollo-client); cruft-OK/bun-ERR cruft-wins 3 (later, collections, sentry); misc one-offs (ast-types/recast missing-name, protobufjs Object.create, sinon regexp, nock _http_common missing, @databases/pg private-field).

**Read**: ~18 of the 114 "failures" are plausibly Bun being wrong, not cruft (15 cases where cruft exposes MORE exports than Bun + 3 outright cruft-wins). This motivates the next move: re-run the basket against Node as the reference engine, not Bun. Highest-leverage real fixes: cruft empty/crash (19), callee-not-callable (7, single mechanism), CJS-wrapper parse (7, single parser gap).
