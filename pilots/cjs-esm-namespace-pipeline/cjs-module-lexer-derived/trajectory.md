# cjs-module-lexer-derived - Trajectory

## CMLD-EXT 0 — founding + fidelity probe + detection-rung design (2026-05-31, design-only)

**Trigger**: CENP-EXT 1 baseline-inspect determined that Stage L cannot be done by suppressing runtime enumeration alone — `debug` (9) and `semver` (40) are CJS packages whose named exports Node detects *statically*; a suppress-without-lexer shortcut would regress them to default-only. The lexer is the Stage-L centerpiece and has multi-rung shape → nested locale per Doc 737 §II. Authored under helmsman appointment (keeper-substituted approval).

**Baseline-inspect (Phase 2)**: probed chalk/uuid/debug/semver/lodash/ms — source assignment pattern vs Node's named-export view. Findings: chalk(12)/uuid(14) are real ESM (`export` syntax) and route through the ESM finalize path, NOT this CJS lexer; debug(9)/semver(40) are statically-detectable CJS (lexer targets); lodash(0)/ms(0) are dynamic CJS (lexer correctly emits nothing). This bounds the lexer's job to statically-detectable member assignments + correct emptiness on dynamic assignment.

**Artifact**: `seed.md` + `design.md` (CjsExportSet output contract; six detection rungs R1–R6; Pin-Art per-rung fidelity-measurement method; AST-vs-text seam question deferred to CMLD-EXT 1). No runtime substrate.

**Result (R)**: nested locale founded; manifest refreshed. The detection rungs R1–R6 are enumerated but NOT authorized by this founding round; each is a future proposal. First implementation rung (CMLD-EXT 1) opens with the AST-vs-text seam decision + R1 (direct member assignment) baseline.

**Composes-with**: parent CENP `design.md` (pipeline frame), Doc 581 (Pin-Art derive-from-constraints), npm `cjs-module-lexer` (behavior reference, not vendored), the Node baseline `parity-results-top500-node-2026-05-31.json`.
