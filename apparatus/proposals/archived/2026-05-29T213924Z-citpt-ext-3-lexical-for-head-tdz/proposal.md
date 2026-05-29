---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 6ca22e0859a413bfc537bfb0030004d237f2a5ce
target_branch: main
summary: CITPT-EXT 3 - lexical for-head TDZ seeding closure
risk_class: substrate
gates_pre:
  citpt_ext3_smoke: 5 PASS / 4 FAIL from 9 packages, with css-tree failing on lexical for-head TDZ
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  targeted_tests: t10b_forof_object_entries_destructure_head PASS; t10c_forin_empty_lexical_head_does_not_false_tdz PASS; t11_object_rest PASS
  citpt_ext3_smoke: 6 PASS / 3 FAIL from 9 packages, residual failures non-TDZ-scoped
---

## Substrate Moves

This closes CITPT-EXT 3 for the R4 replacement path authorized by `helmsman/response/citpt-ext-3-r4-replacement-landing-authorization`.

- **M** = lexical `for-of` / `for-in` head TDZ seeding for loop-head bindings.
- **T** = switch lexical loop-head seed writes from `PushTDZ + StoreLocal` to `PushTDZ + InitLocal`, preserving the TDZ sentinel instead of false-consuming it through assignment semantics.
- **I** = the bytecode compiler `for-of` and `for-in` lexical binding emit sites, one new regression test for empty lexical `for-in` head behavior, and the locale trajectory entry recording the re-diagnosis and closure.
- **R** = the css-tree shaped package failure disappears; 6 of 9 packages now pass in the CITPT-EXT 3 smoke, and the 3 residual failures are outside TDZ scope.

## Risk Assessment

The edit is tightly scoped to two compiler seed paths that are the sibling shape of the earlier destructure-init closure, but they sit on loop-head lowering where false initialization can silently perturb lexical semantics. The verification mix covers both the originally failing `for-of` destructure head and the newly added `for-in` empty lexical head case, plus a nearby destructuring test and the runtime lib suite. Residual package failures were inspected and attributed to parser and filesystem surfaces rather than TDZ, which lowers the risk of scope confusion at close.

## Composes-With

- CITPT-EXT 2 destructure-init sibling closure.
- Locale: `pilots/class-inheritance-tdz-parser-tail/`.
- Authorization message `8fd175f0-0c4a-4260-bd2c-4caac47a9c99`.

**APPROVED for push** per Helmsman CITPT-EXT 3 directive.
