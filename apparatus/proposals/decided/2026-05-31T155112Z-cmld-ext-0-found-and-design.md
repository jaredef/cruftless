---
proposal_slug: 2026-05-31T155112Z-cmld-ext-0-found-and-design
decision: APPROVED
arbiter_session: keeper-override-2026-05-31
decided_at: 2026-05-31T15:52:15Z
covers_commits:
  - 4129a6f44b066e4f2de1b409c6fc27ef517c0735
---

## Findings

Approved by the keeper acting as arbiter via keeper override (pre-instantiation keeper-substituted-approval path).

Verified:

- **Design-only, no runtime risk.** Commit `4129a6f4` touches only the new nested locale `pilots/cjs-esm-namespace-pipeline/cjs-module-lexer-derived/` (seed/design/trajectory) and `apparatus/locales/manifest.json` (refresh). Gates legitimately not rerun.
- **Nesting is correct per Doc 737 §II.** The static export scanner has genuine multi-rung shape (six detection forms R1–R6 + reexport resolution depth); founding it as a nested locale under CENP rather than smuggling it into a single CENP-EXT 1 rung is the right application of the promotion threshold.
- **Baseline-inspect is empirically grounded (Rule 23).** The fidelity probe across chalk/uuid/debug/semver/lodash/ms correctly partitions the scanner's job: real-ESM packages excluded (they route to the ESM finalize path), static-detectable CJS as the target, dynamic CJS as the correct-empty case.
- **Fidelity discipline encoded (Rule 14 analogue).** Per-rung symmetric-set-diff measurement vs observed Node output, landing a rung only if it strictly reduces aggregate diff without increasing any package's diff. This directly mitigates the parent CENP design's named dominant risk (lexer fidelity).
- **No over-reach.** Detection rungs R1–R6 are enumerated but not authorized; each carries its own future proposal + measurement. The AST-vs-text seam and reexport-resolution-depth are recorded as open, to be decided at CMLD-EXT 1 baseline-inspect.

Qualifications carried forward (non-blocking): the Node-version pin (v24.11) for the fidelity oracle must be recorded with each measurement; R5 reexport resolution may warrant its own rung; the derive-not-vendor carve-out must hold (npm cjs-module-lexer read for behavior only).
