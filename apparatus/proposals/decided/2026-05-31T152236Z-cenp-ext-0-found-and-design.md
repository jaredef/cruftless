---
proposal_slug: 2026-05-31T152236Z-cenp-ext-0-found-and-design
decision: APPROVED
arbiter_session: keeper-override-2026-05-31
decided_at: 2026-05-31T15:25:40Z
covers_commits:
  - 34a4b8badd6dd2f1215056b16c0661c4cba595dd
---

## Findings

Approved by the keeper acting as arbiter via keeper override (no arbiter session adjudicated; per the operational protocol's pre-instantiation keeper-substituted-approval path).

Verified:

- **Design-only, no runtime risk.** Commit `34a4b8ba` touches only `pilots/cjs-esm-namespace-pipeline/` (new locale: seed/design/trajectory) and `apparatus/locales/manifest.json` (refresh). No source under `pilots/*/derived/src/` or `cruftless/src/` changed. Gates legitimately not rerun.
- **Move-shape correction recorded (Rule 23).** The proposal explicitly inspects and rejects the naive "297 fn-intrinsic = one bug, +30pts" frame, recognizing the existing fn-lift as intentional Bun parity. This is the correct application of baseline-inspect-at-founding; carrying the wrong frame forward would have mis-scoped the implementation rungs.
- **Architecture is spec-grounded.** Stage L (Node-faithful static export-name set) is justified by the ESM Parse→Link→Evaluate phase ordering and the link-time-name-determination requirement for live bindings / circular imports — not a preference. Consistent with Doc 731 (alphabet purity upstream as the bound on downstream complexity).
- **Bun investment preserved.** The prior CNSDR/P53/P38 heuristics are demoted into Stage W (additive, gated), not deleted; recoverable and auditable as one stage.
- **No over-reach.** The four implementation rungs (CENP-EXT 1–4) are enumerated but explicitly not authorized by this proposal; each will carry its own gates and proposal.

Qualifications carried forward to the implementation rungs (not blocking this design landing): cjs-module-lexer fidelity must be derived under Pin-Art discipline against observed Node output (under-detection would regress named-import compat vs Node itself); demotion blast-radius must be measured against the CNSDR Bun-shape analysis; the CJS and ESM sites must apply the pipeline consistently.
