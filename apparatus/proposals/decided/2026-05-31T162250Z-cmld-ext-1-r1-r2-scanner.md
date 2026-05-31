---
proposal_slug: 2026-05-31T162250Z-cmld-ext-1-r1-r2-scanner
decision: APPROVED
arbiter_session: keeper-override-2026-05-31
decided_at: 2026-05-31T16:26:11Z
covers_commits:
  - fbe8f80a2dac452eee75670ca50d2598bc51bc13
---

## Findings

Approved by the keeper acting as arbiter via keeper override.

Verified:

- **Behavior-neutral substrate landing.** Commit `fbe8f80a` adds a new pure module (`cjs_lexer.rs`) and one `pub mod` line. The function has no caller on any engine behavior path, so test262 / sample / diff-prod are necessarily unchanged; gates_post records this honestly rather than asserting a re-run delta. Landing validated dead code before its integration rung is sound — it lets the integration's parity delta be measured in isolation.
- **Gate is appropriate to the rung.** Unit-test fidelity (10/10) against the seed's named cohorts is the correct gate for an unwired pure scanner: R1 detect (debug-class), R2 detect (semver-class), correct-empty (lodash/ms dynamic + function export), computed keys, `__esModule` (both forms), if-block nesting, nested-function exclusion, unparseable→empty. Build clean.
- **Seam resolved empirically (Rule 23).** The AST-vs-text decision was made by inspecting `evaluate_cjs_module` (AST already parsed at `module.rs:1580`, discarded), not asserted — AST-walk chosen for robustness.
- **Scope is honest.** Only R1+R2+`__esModule` claimed; R3–R6 and deeper control-flow nesting explicitly deferred. Real-package symmetric-diff vs Node correctly deferred to the CENP-EXT 1 integration.
- **Single substrate move (Rule 4).** One scanner module; no entanglement with the live namespace path.

Qualifications carried forward (non-blocking): fidelity is so far synthetic-fixture-only — the integration rung MUST measure real-package symmetric-set-diff vs the Node v24.11 baseline before the scanner's name set replaces runtime enumeration, or under/over-detection could regress currently-passing packages. The integration should reuse the already-parsed `_ast_rc` rather than re-parsing.
