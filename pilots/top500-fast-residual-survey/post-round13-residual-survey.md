# Post-Round13 Residual Survey

## Scope

Fresh reduced parity sample from the rebased R4 worktree after the Round 13 landings through `CITPT-EXT 3`.

- Worktree: `/home/jaredef/Developer/cruftless-r4`
- Sample size: 19 packages
- Harness: `legacy/host-rquickjs/tools/parity-measure.sh`
- Raw results:
  `/home/jaredef/Developer/cruftless-sidecar/results/top500-fast-residual-survey-2026-05-29T215950Z/round13b-r4-results.json`

## Distribution

| Bucket | Count | Packages |
|---|---:|---|
| PASS | 7 | `prettier`, `csso`, `rehype`, `puppeteer-core`, `svgo`, `config`, `mocha` |
| FAIL - namespace shape diff after successful import | 5 | `readable-stream`, `events`, `winston`, `proj4`, `decimal.js-light` |
| FAIL - filesystem / package metadata path assumptions | 2 | `redis`, `stylelint` |
| FAIL - host/prototype method surface missing | 2 | `chai`, `exceljs` |
| FAIL - singleton runtime/parser residuals | 3 | `arktype`, `mongoose`, `mathjs` |

## Key observations

### 1. CNSDR is the largest remaining single-cause cluster in this sample

Five of the twelve failures were not hard import failures. They loaded and produced a namespace, but the namespace shape diverged from Bun:

- `readable-stream`: Bun 26 keys vs cruft 17 keys
- `events`: Bun 17 keys vs cruft 7 keys
- `winston`: Bun 39 keys vs cruft 47 keys
- `proj4`: Bun namespace differs from cruft's function-plus-metadata surface
- `decimal.js-light`: Bun namespace differs from cruft's `Decimal` plus `default` surface

This is larger than any other single family in the reduced sample, so the next substrate move should target `pilots/cjs-ns-shape-diff-residual/`.

### 2. CITPT no longer dominates the caller-leak surface

The TDZ family does not reassert itself in this sample. The known CITPT controls still pass:

- `prettier`
- `csso`
- `rehype`
- `puppeteer-core`
- `svgo`
- `config`

The remaining three CITPT-associated residuals stayed in their previously diagnosed non-TDZ families:

- `arktype`: `ParseError: 'generic' is unresolvable`
- `redis`: `package.json read failed ... (in-call='require')`
- `stylelint`: `readFileSync: No such file or directory`

### 3. There is one new high-value singleton correctness bug

`mathjs` triggers a runtime panic:

```text
user-provided comparison function does not correctly implement a total order
```

This is not the next cluster by count, but it is a strong correctness signal because it is a Rust panic rather than a normal JS-visible error.

## Recommended next move

### Primary

Continue `CNSDR` with a substrate rung aimed at the five-package namespace-shape family in this sample.

Why this over other families:

- it is the largest coherent cluster by count
- the imports already succeed, so the work is closer to namespace finalization/shim policy than to deep eval completion
- it composes directly with the existing `pilots/cjs-ns-shape-diff-residual/` locale instead of requiring a brand-new substrate family

### Secondary follow-ups after CNSDR

1. filesystem/path-assumption cluster:
   `redis`, `stylelint`
2. host/prototype surface cluster:
   `chai`, `exceljs`
3. singleton panic:
   `mathjs`

## Standing finding

**Finding TFRS.1**: after the ATC / super-ctor / TAPD / MILF / CITPT landings, the top residual pressure in a fresh representative sample shifts away from hard parser/TDZ failures and back toward namespace-shape parity. Standing recommendation: after any future parser/runtime closure batch, run a fast scope-down parity sample before selecting the next locale. The dominant family can change even when the raw FAIL count remains high.
