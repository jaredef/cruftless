---
helmsman_session: helmsman-2026-05-31-cenp-principal
proposed_commits:
  - 3ff3ddca0654a1ff6d7d9fdaf16ea3a7d5e9f9a0
target_branch: main
summary: CMLD-EXT 1 — R1+R2 static CJS export-name scanner (pure, unit-tested, unwired)
risk_class: substrate
gates_pre:
  test262_full: 72.6% (2026-05-31)
  test262_sample: 89.6% (2026-05-31)
  diff_prod: not rerun
  per_locale: { top500-vs-node: 38.7%, top500-vs-bun: 88.0% }
gates_post:
  test262_full: unchanged (scanner unused by engine; no behavior path touched)
  test262_sample: unchanged (same)
  diff_prod: unchanged (same)
  per_locale: { cjs_lexer-unit-tests: 10/10 pass, build: clean }
---

## Substrate moves

Commit `3ff3ddca` lands the first detection rungs of the CMLD Stage-L static export-name scanner.

- **M** (Mechanism) — new pure module `pilots/rusty-js-runtime/derived/src/cjs_lexer.rs`: `cjs_lex(source) -> CjsExportSet { names, esmodule }`. AST-walk (not text-scan): re-wraps the source as `evaluate_cjs_module` does, parses, extracts the wrapper function body, and detects **R1** (`exports.NAME =`, `module.exports.NAME =`, incl. computed string keys) + **R2** (`module.exports = { … }` literal members) + `__esModule` flag (assignment and `Object.defineProperty(exports,"__esModule",…)`). Descends into block + if-branch nesting; does NOT descend into nested function bodies. Registered `pub mod cjs_lexer` in the crate lib.rs.
- **T** (Trigger) — CMLD-EXT 0 design (Stage-L name-set producer); seam-resolution baseline-inspect found `evaluate_cjs_module` already parses the CJS AST (`module.rs:1580`, discarded as `_ast_rc`), so AST-walk is both available and more robust than text-scan.
- **I** (Implementation-site) — `pilots/rusty-js-runtime/derived/src/cjs_lexer.rs` (new); `pilots/rusty-js-runtime/derived/src/lib.rs` (`pub mod cjs_lexer`); CMLD trajectory entry.
- **R** (Result) — R1+R2 scanner validated in isolation; **unused by the engine** (no call site in the namespace path). Integration is the parent CENP-EXT 1 rung. R3/R4/R5/R6 are further CMLD rungs.

## Risk assessment (helmsman self-evaluation)

- **Behavior-neutral landing.** The scanner is dead code w.r.t. the running engine — `cjs_lex` has no caller in `populate_cjs_namespace_view_at` or anywhere on a behavior path. Therefore test262, test262-sample, and diff-prod are necessarily unchanged; gates_post records this rather than asserting a re-run delta. This is intentional: the scanner is landed and unit-validated before the integration rung wires it, so integration can be measured in isolation.
- **Gate is unit-test fidelity.** 10/10 tests pass, covering the seed's cohorts: R1 detect (debug-class), R2 detect (semver-class), correct-empty (lodash/ms dynamic + function export), computed string keys, `__esModule` (both forms), if-block nesting, nested-function exclusion, unparseable→empty. `cargo build --release --bin cruft -p cruftless` clean.
- **Scope honesty.** Only R1+R2+`__esModule` are claimed. R3 (general defineProperty names), R4 (transpiler prologue), R5 (reexport stars), R6 (dynamic-guard hardening) are explicitly out of this rung. Deeper control-flow nesting (loops/try/switch) is not yet walked — documented as acceptable for R1/R2 because CJS export assignments are near-top-level; a future rung extends nesting if a cohort needs it.
- **Forward risk (not incurred here)**: fidelity vs Node is only unit-tested against synthetic fixtures so far; real-package symmetric-set-diff measurement happens at the CENP-EXT 1 integration against the top500 Node baseline. Under/over-detection would surface there.
- **Standing rules consulted**: Rule 4 (single substrate move — one scanner module), Rule 14 analogue (classifier conservative-strip — additive detection, validated per-case), Rule 23 (baseline-inspect resolved the AST/text seam empirically before writing).

## Composes-with

- **Parent**: CENP `design.md` (Stage L consumes `cjs_lex`); CMLD `design.md` (rung decomposition R1–R6).
- **Reuse opportunity recorded**: the discarded `_ast_rc` at `module.rs:1580` — CENP-EXT 1 integration can pass that AST instead of re-parsing.
- **Corpus**: Doc 581 (Pin-Art derive-from-constraints), Doc 729 (resolver-instance pattern).
- **Apparatus**: Node baseline `parity-results-top500-node-2026-05-31.json` (the fidelity oracle for the integration rung).
- **Deferrals-ledger**: none (R3–R6 enumerated as named future rungs, not deferred candidates).
