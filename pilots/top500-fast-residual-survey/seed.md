# top500-fast-residual-survey - Seed

**Locale tag**: `L.top500-fast-residual-survey` (TFRS).

**Status**: FOUNDED at Round 13b. Measurement + clustering only; no substrate edit in this locale.

**Parent scope**: broad top500 residual triage after the 2026-05-29 11:17 canonical sweep and the subsequent landed closures through CITPT-EXT 3.

## I. Telos

Run a reduced but fresh Bun-vs-cruft parity sample from the current R4 worktree, then identify the largest remaining single-cause cluster worth the next substrate rung.

This locale is not a substrate-fix locale. It exists to answer one chapter-close question: after ATC / super-ctor / TAPD / MILF / CITPT landings, what residual family dominates a fresh representative sample?

## II. Apparatus

- `legacy/host-rquickjs/tools/parity-measure.sh`
- Bun installed locally for this round at `~/.bun/bin/bun`
- Raw result artifact for this run:
  `/home/jaredef/Developer/cruftless-sidecar/results/top500-fast-residual-survey-2026-05-29T215950Z/round13b-r4-results.json`
- Existing residual locales used for interpretation:
  `pilots/cjs-ns-shape-diff-residual/`
  `pilots/dynamic-import-attributes/`
  `pilots/class-inheritance-tdz-parser-tail/`

## III. Methodology

1. Rebase the R4 worktree to current `origin/main`.
2. Build `cruft` from the rebased worktree.
3. Run a scope-down representative sample because the original `/media/jaredef/T7/...` sandbox was not mounted in this Codex namespace.
4. Sample composition:
   - 9 CITPT packages/residuals/controls:
     `arktype`, `redis`, `stylelint`, `prettier`, `csso`, `rehype`, `puppeteer-core`, `svgo`, `config`
   - 10 broader top-tier packages spanning prior clusters:
     `mocha`, `chai`, `mathjs`, `mongoose`, `exceljs`, `readable-stream`, `events`, `winston`, `proj4`, `decimal.js-light`
5. Cluster failures by dominant mechanism, not only by package name.
6. Name the largest single-cause family and route the next move to that locale.

## IV. Carve-Outs

- This locale does not land runtime/parser substrate.
- The sample is representative, not canonical. It is for direction-setting after Round 13b, not for replacing the full top500 sweep.
- Any package that panics the runtime is recorded as its own residual shape unless a broader cluster shares the same mechanism.

## V. Resume Protocol

Read this seed, then `trajectory.md`, then `post-round13-residual-survey.md`. Re-run the sample only if a new batch of landings significantly changed the caller-leak landscape.
