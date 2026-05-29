---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - ecc474f3273bc6197a5a07203809a3a44952a281
target_branch: main
summary: SMPT-EXT 4 — strict/generator yield residual closure; 4/4 target test262 parser rows PASS
risk_class: substrate
gates_pre:
  test262_full: 67.6
  test262_sample: 84.8 (re-measure pending)
  targets: 0/4 (latest full-suite parser-owned residual rows)
gates_post:
  test262_full: 67.6 (not re-measured this rung)
  test262_sample: 84.8 (not re-measured this rung)
  targets: 4/4
  build: cargo build --release --bin cruft -p cruftless PASS
  parser_crate_tests: blocked by unrelated legacy_octal_rejected lexer unit
---

## Substrate Moves

Single SMPT-EXT 4 rung in `pilots/strict-mode-parser-tracking/`.

- **M** = four latest parser-owned strict/generator `yield` residuals: strict object shorthand, assignment-pattern object shorthand conversion, class static block inside generator body, and lexical-declaration newline continuation.
- **T** = parser applies the exact `yield` predicate at each grammar coordinate: strict/generator IdentifierReference rejection for shorthand; strict `[~Yield]` context for static blocks; no silent declarator termination before same-line non-separator token.
- **I** = `expr.rs::parse_object_literal` shorthand guard; `stmt.rs::parse_class_body` static-block context boundary; `stmt.rs::parse_variable_statement` missing-initializer continuation guard; SMPT trajectory entry records findings.
- **R** = extends SMPT-EXT 2 strict-mode tracking and SMPT-EXT 3 generator-context tracking without touching R3's non-strict contextual for-head PPAE surface.

## Verification

Target test262 exemplars all PASS under `target/release/cruft` + legacy test262 runner:

- `language/expressions/assignment/dstr/obj-id-identifier-yield-ident-invalid.js`
- `language/expressions/object/identifier-shorthand-yield-invalid-strict-mode.js`
- `language/statements/class/static-init-invalid-yield.js`
- `language/statements/let/syntax/let-newline-yield-in-normal-function.js`

Protective probes:

- `"use strict"; function f() { yield 1; }` -> SyntaxError
- `function f() { var yield = 4; console.log(yield); } f();` -> 4
- `var yield = 4; for ([x = yield] of [[]]) console.log(x);` -> 4

Build: `cargo build --release --bin cruft -p cruftless` PASS.

Parser crate test note: `cargo test --release -p rusty-js-parser` is blocked by pre-existing unrelated lexer unit `tests/spec_golden.rs::legacy_octal_rejected`, which currently accepts `07` as a number token.

## Risk Assessment

The change is parser-only and localized to three grammar coordinates. It does not touch bytecode lowering, runtime semantics, or R3's non-strict PPAE contextual for-head work.

Primary risk was over-rejecting sloppy `yield` as an identifier. The sloppy function and for-of default-value protective probes remained valid, which confirms the new guards stay under the strict/generator predicate rather than reintroducing SMPT-EXT 1's broader function-body heuristic.

**APPROVED for push** per Helmsman same-turn approval and keeper clarification requiring same-turn substrate landing.
