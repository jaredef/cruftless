---
helmsman_session: helmsman-2026-05-31-cenp-principal
proposed_commits:
  - a46cc047399669ea65bda15f145cefa36558eebc
target_branch: main
summary: Found cjs-module-lexer-derived nested locale + Stage-L static export-scanner design (design-only)
risk_class: design
gates_pre:
  test262_full: 72.6% (2026-05-31)
  test262_sample: 89.6% (2026-05-31)
  diff_prod: not rerun (design-only)
  per_locale: { top500-vs-node: 38.7%, top500-vs-bun: 88.0% }
gates_post:
  test262_full: not rerun; design-only commit
  test262_sample: not rerun; design-only commit
  diff_prod: not rerun; design-only commit
  per_locale: { top500-vs-node: 38.7% (baseline, unchanged) }
---

## Substrate moves

Commit `a46cc047` founds the nested `cjs-esm-namespace-pipeline/cjs-module-lexer-derived` (CMLD) locale and lands a design-only scanner architecture. No runtime source changed.

- **M** (Mechanism) — a pure static scanner `cjs_lex(source) -> {names, reexports, esmodule}` that determines a CJS module's named-export set from source text without executing it, functionally equivalent to Node's `cjs-module-lexer`. Six detection rungs enumerated (R1 direct member assignment, R2 object-literal, R3 defineProperty/`__esModule`, R4 transpiler prologue, R5 reexport stars, R6 dynamic-guard negative). It produces the name set; the parent (CENP Stage L) consumes it and applies fixed keys + value binding.
- **T** (Trigger) — CENP-EXT 1 baseline-inspect: Stage L cannot suppress runtime enumeration alone without regressing statically-detectable CJS packages. Fidelity probe across chalk/uuid/debug/semver/lodash/ms confirmed: chalk(12)/uuid(14) are real ESM (out of CJS-lexer scope); debug(9)/semver(40) are static-detectable CJS (lexer targets); lodash(0)/ms(0) are dynamic (lexer correctly empty).
- **I** (Implementation-site) — `pilots/cjs-esm-namespace-pipeline/cjs-module-lexer-derived/{seed.md, design.md, trajectory.md}`; `apparatus/locales/manifest.json` refreshed. Future consumption site named but untouched: `module.rs::populate_cjs_namespace_view_at`.
- **R** (Result) — nested locale founded per Doc 737 §II (multi-rung sub-workstream); manifest refreshed. Detection rungs R1–R6 enumerated but NOT authorized by this proposal; each is its own future proposal with per-rung fidelity measurement.

## Risk assessment (helmsman self-evaluation)

- **No runtime risk** — design-only; gates not rerun by design.
- **Nesting justified (Doc 737 §II).** The lexer has multi-rung shape (six independent detection forms + reexport resolution depth); founding it as a nested locale rather than smuggling it into a single CENP-EXT 1 rung is the correct application of the promotion threshold.
- **Fidelity discipline encoded.** design.md §4 mandates per-rung symmetric-set-diff measurement vs observed Node output, landing a rung only if it strictly reduces aggregate diff without increasing any package's diff (conservative-strip, Rule 14 analogue). This is the mitigation for the parent's named dominant risk (lexer fidelity).
- **Open seam recorded, not resolved**: AST-walk vs text-scan for detection (CMLD-EXT 1 baseline-inspect decides); reexport resolution depth (R5 may warrant its own rung); Node-version pin (v24.11) for the fidelity oracle.
- **Standing rules consulted**: Rule 11 (pre-spawn — nested under CENP, no sibling overlap), Rule 23 (baseline-inspect with empirical probe), Rule 14 analogue (classifier conservative-strip), Doc 737 §II (nested-locale promotion), Doc 581 (derive-not-vendor).

## Composes-with

- **Parent**: `cjs-esm-namespace-pipeline` (CENP) `design.md` — Stage L frame this locale's output feeds.
- **Corpus**: Doc 729 (resolver-instance pattern), Doc 581 (Pin-Art derive-from-constraints), Doc 737 §II (nested locale).
- **Reference (not vendored)**: npm `cjs-module-lexer` documented behavior; Node v24.11 named-export output as fidelity oracle.
- **Apparatus**: Node baseline `legacy/host-rquickjs/tools/parity-results-top500-node-2026-05-31.json`; `parity-measure-ref.sh`.
- **Deferrals-ledger**: none surfaced (rungs enumerated, not deferred).
