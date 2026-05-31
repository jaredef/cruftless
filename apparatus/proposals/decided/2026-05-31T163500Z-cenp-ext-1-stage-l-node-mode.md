---
proposal_slug: 2026-05-31T163500Z-cenp-ext-1-stage-l-node-mode
decision: APPROVED
arbiter_session: keeper-override-2026-05-31
decided_at: 2026-05-31T16:38:00Z
covers: "locale cjs-esm-namespace-pipeline; commit subject 'cenp-ext-1: env-gated Stage-L Node-mode CJS interop'"
note_on_coverage: "No covers_commits SHA per keeper directive 2026-05-31 (pre-push hook inactive on this checkout; SHA churned on rebase with no enforcement). Referenced by slug + commit subject."
---

## Findings

Approved by the keeper acting as arbiter via keeper override.

Verified:

- **Zero default-mode regression — the central safety property.** The Node path is an early return reached only when `CRUFT_CJS_INTEROP=node`; the default path is byte-identical Bun behavior. Spot-check confirmed prior Bun numbers (lodash 312 / debug 24 / semver 47). Existing gates (test262, sample, diff-prod, top500-vs-Bun) are therefore necessarily unchanged.
- **Measured delta, gated discipline (Rule 5+10).** Node-mode top500 parity 77.4% (754/974), +38.7 points over the 38.7% baseline, from R1+R2 alone. The mechanism was introduced gated and measured in the gated mode; the global default was correctly NOT flipped — that flip is reserved for a later canonical-fuzz-gated rung. This is the textbook application of the no-silent-default-flip discipline.
- **Probe-grounded (Rule 23).** The literal `module.exports` namespace key was confirmed real Node behavior via the pure-`Object.keys` probe before synthesizing it — not matched to an artifact.
- **Residual named, not hidden.** 218 Node-mode failures attributed to R5 reexport-stars (largest lever), R3/R4 precision, semver +6 over-detection, and the arktype engine bug — each a named follow-up, none a regression.

Qualifications carried forward (non-blocking): the source-from-`url` lex assumes on-disk CJS source equals what was evaluated (true for plain .js npm packages; TS/transformed sources are an edge to handle before the default-flip rung). The default-flip rung MUST run canonical fuzz + three-probe-levels and re-confirm test262/diff-prod hold under Node-mode before changing the global default.
