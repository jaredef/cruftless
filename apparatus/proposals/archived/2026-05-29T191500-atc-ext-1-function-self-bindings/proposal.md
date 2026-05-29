---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 99e6b1a93f1da38983eeac03ea7c1909f904f144
target_branch: main
summary: ATC-EXT 1 function declaration self-binding split
risk_class: substrate
gates_pre:
  atc_26_package_slice: 0 PASS / 26 FAIL
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  focused_self_reassignment: 2 PASS
  atc_26_package_slice: 13 PASS / 13 FAIL
---

## Substrate Moves

This is the ATC Phase 3 compiler fix authorized by
`helmsman/request/atc-ext-1-compiler-split-fn-decl-vs-fn-expr-r1`.

- **M** = function declaration self-reassignment in generated helper bodies.
- **T** = function declarations use their mutable hoisted declaration binding;
  named function expressions keep their immutable self-name binding.
- **I** = `FunctionSelfNameMode` discriminant threaded through function proto
  construction, plus focused runtime regressions for both sides of the split.
- **R** = Babel/Istanbul/Jest helper function declarations can rewrite their own
  binding without disabling named-function-expression const semantics.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 68 passed, 1 ignored.
- `cargo test --release -p rusty-js-runtime --lib self_reassignment -- --nocapture`
  PASS: 2 passed.
- ATC 26-package slice: 13 PASS / 13 FAIL, +13 over the 0/26 baseline.
- Artifact:
  `/home/jaredef/Developer/cruftless-sidecar/results/atc-ext1-20260529T191328Z/summary.json`.

## Risk Assessment

The change is compiler-local and explicitly discriminates declaration versus
expression construction sites. The adjacent spec guard for named function
expressions is covered by a focused test that still expects the
assignment-to-constant TypeError.

Remaining 13 package failures are no longer assignment-to-constant failures;
they surface downstream TDZ, CJS global, global helper binding, prototype-shape,
derived-constructor, and namespace-shape residuals.

**APPROVED for push** per Helmsman ATC Phase 3 directive.
