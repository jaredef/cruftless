---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - bfcee498097e399ba3c6b83990b42d2b298969da
target_branch: main
summary: Round 13b R4 fast residual survey and next-cluster identification
risk_class: apparatus
gates_pre:
  canonical_top500: 2026-05-29T11:17:02 = 754 PASS / 79.5% runnable
  local_scope: no fresh post-CITPT top500 sweep in this worktree
gates_post:
  rebase: git fetch origin main && git rebase origin/main PASS
  build: cargo build --release --bin cruft -p cruftless PASS
  parity_sample: 7 PASS / 12 FAIL / 0 SKIP from 19 packages
  dominant_cluster: CNSDR namespace-shape-diff = 5 packages
---

## Substrate Moves

This is a measurement-and-triage rung only, authorized by helmsman message `1a067a1b-93ad-4db2-98b9-9221492f3740`.

- **M** = refresh a representative post-Round-13 parity picture from the R4 worktree and identify the largest residual family.
- **T** = found a broad survey locale, run a reduced Bun-vs-cruft sample, persist the raw artifact, and route the next move to the dominant existing locale.
- **I** = new locale `pilots/top500-fast-residual-survey/`, refreshed `apparatus/locales/manifest.json`, and a findings report naming CNSDR as the next caller-leak.
- **R** = the reduced sample showed 7 PASS / 12 FAIL, with the largest coherent failure family being namespace-shape diffs (`readable-stream`, `events`, `winston`, `proj4`, `decimal.js-light`).

## Risk Assessment

The risk is interpretive, not substrate-semantic: a reduced sample can mis-rank the global residual landscape if the package mix is poor. This rung mitigated that by mixing CITPT controls, prior dynamic-import/shape-diff probes, and mainstream package surfaces. The filesystem namespace initially lacked the helmsman-specified `/media/jaredef/T7/...` sandbox and `bun` on `PATH`; the rung closed both gaps without fabricating data, using the directive's allowed scope-down path and a local Bun install.

## Composes-With

- `pilots/cjs-ns-shape-diff-residual/`
- `pilots/class-inheritance-tdz-parser-tail/`
- `pilots/dynamic-import-attributes/`
- Raw artifact: `/home/jaredef/Developer/cruftless-sidecar/results/top500-fast-residual-survey-2026-05-29T215950Z/round13b-r4-results.json`

**APPROVED for push** per Helmsman Round 13b directive.
