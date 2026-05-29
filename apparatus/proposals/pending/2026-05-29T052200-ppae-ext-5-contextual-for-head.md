---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 778d4db6f61beaaaf0c270a2394faa0615873b3a
target_branch: main
summary: PPAE-EXT 5 contextual for-head parser discrimination
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: null
  diff_prod: 0/112 (instrument unavailable in clone state)
  per_locale:
    PPAE-targeted: 84/101 PASS, 5 FAIL, 12 SKIP
    TAMM: 82/100
    TAWR: 63/100
gates_post:
  test262_full: null
  test262_sample: null
  diff_prod: 0/112 (instrument unavailable in clone state)
  per_locale:
    PPAE-targeted: 89/101 PASS, 0 FAIL, 12 SKIP
    TAMM: 82/100
    TAWR: 63/100
---

## Substrate moves

Commit `778d4db6f61beaaaf0c270a2394faa0615873b3a` lands PPAE-EXT 5.

- **M** = parser for-head discrimination around contextual tokens (`async`, `let`) and expression-headed assignment targets.
- **T** = ECMA-262 for-in/of grammar accepts valid non-strict LeftHandSideExpression heads while preserving negative SyntaxError rows (`async of` in ordinary for-of, `let of`, escaped `of`).
- **I** = `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement`: narrow `let` declaration detection to avoid swallowing sloppy `let in`; gate `async of` on ordinary for-of + raw spelling; route expression-headed identifiers and members through `ForBinding::AssignmentTarget`.
- **R** = sibling to PPAE-EXT 1 contextual-keyword rawness and PPIF/FAOF lookahead discipline. Explicitly outside R4 SMPT strict/generator predicate scope.

## Risk assessment (helmsman self-evaluation)

Primary risk is false acceptance of negative for-of lookahead cases. Protective probes verified `language/statements/for-of/head-lhs-let.js` and `language/statements/for-of/escaped-of.js` still PASS as SyntaxError negatives.

Secondary risk is regression in prior PPAE arrow early errors. Representative rows for duplicate parameters, future reserved words, `eval`, and `arguments` remain PASS.

`scripts/diff-prod/run-all.sh` returned `PASS=0 FAIL=112`, a uniform infrastructure/configuration failure pattern in this clone state. The result was not used as a semantic gate.

## Composes-with

- `apparatus/arcs/2026-05-25-ecmascript-parity-shared-upstream/`
- `pilots/parser-permissiveness-audit-extensions/`
- `pilots/strict-mode-parser-tracking/` by boundary only: R4 owns strict/generator residuals.
