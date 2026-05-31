# cjs-module-lexer-derived - Trajectory

## CMLD-EXT 0 — founding + fidelity probe + detection-rung design (2026-05-31, design-only)

**Trigger**: CENP-EXT 1 baseline-inspect determined that Stage L cannot be done by suppressing runtime enumeration alone — `debug` (9) and `semver` (40) are CJS packages whose named exports Node detects *statically*; a suppress-without-lexer shortcut would regress them to default-only. The lexer is the Stage-L centerpiece and has multi-rung shape → nested locale per Doc 737 §II. Authored under helmsman appointment (keeper-substituted approval).

**Baseline-inspect (Phase 2)**: probed chalk/uuid/debug/semver/lodash/ms — source assignment pattern vs Node's named-export view. Findings: chalk(12)/uuid(14) are real ESM (`export` syntax) and route through the ESM finalize path, NOT this CJS lexer; debug(9)/semver(40) are statically-detectable CJS (lexer targets); lodash(0)/ms(0) are dynamic CJS (lexer correctly emits nothing). This bounds the lexer's job to statically-detectable member assignments + correct emptiness on dynamic assignment.

**Artifact**: `seed.md` + `design.md` (CjsExportSet output contract; six detection rungs R1–R6; Pin-Art per-rung fidelity-measurement method; AST-vs-text seam question deferred to CMLD-EXT 1). No runtime substrate.

**Result (R)**: nested locale founded; manifest refreshed. The detection rungs R1–R6 are enumerated but NOT authorized by this founding round; each is a future proposal. First implementation rung (CMLD-EXT 1) opens with the AST-vs-text seam decision + R1 (direct member assignment) baseline.

**Composes-with**: parent CENP `design.md` (pipeline frame), Doc 581 (Pin-Art derive-from-constraints), npm `cjs-module-lexer` (behavior reference, not vendored), the Node baseline `parity-results-top500-node-2026-05-31.json`.

## CMLD-EXT 1 — R1 + R2 detection scanner (2026-05-31)

**Trigger**: implement the Stage-L static export-name scanner's first detection rungs.

**Baseline-inspect (seam resolution)**: `evaluate_cjs_module` already parses the CJS source into an AST (`module.rs:1580`, currently discarded as `_ast_rc`); `source` + `url` are in scope. Seam decided: **AST-walk, not text-scan** — more robust (no comment/string/template false positives, design.md §6 risk) and the parse already happens. The CJS body lives inside the synthesized wrapper `export default (function(exports, module, require, __filename, __dirname){ <source> })`; the scanner re-wraps + parses + extracts that inner function body.

**Substrate**: new pure module `pilots/rusty-js-runtime/derived/src/cjs_lexer.rs` exposing `cjs_lex(source) -> CjsExportSet { names, esmodule }`. Detection: **R1** (`exports.NAME =`, `module.exports.NAME =`, incl. computed string keys), **R2** (`module.exports = { … }` literal member names), plus `__esModule` flag (assignment + `Object.defineProperty(exports, "__esModule", …)`). Walk descends into block + if-branch nesting but NOT nested function bodies (matches Node's shallow static scope). Registered `pub mod cjs_lexer` in the runtime crate lib.rs.

**Gate**: 10/10 unit tests pass (debug-class R1 detect, semver-class R2 detect, lodash/ms-class correctly empty, computed-key, `__esModule` both forms, if-block nesting, nested-function exclusion, unparseable→empty). `cargo build --release --bin cruft` clean. test262/diff-prod unchanged by design — the function is UNUSED by the engine until CENP-EXT 1 wires it into `populate_cjs_namespace_view_at`.

**Result (R)**: R1+R2 scanner landed + validated in isolation. Not yet integrated. Next: R3 (defineProperty names beyond `__esModule`) / R4 (transpiler prologue) / R5 (reexport stars) as further CMLD rungs, and the parent CENP-EXT 1 integration that consumes `cjs_lex` and measures the top500-vs-Node parity delta.

**Composes-with**: parent CENP Stage-L; the discarded `_ast_rc` at `module.rs:1580` (the integration can reuse it instead of re-parsing); Doc 581 (Pin-Art derive-from-constraints).
