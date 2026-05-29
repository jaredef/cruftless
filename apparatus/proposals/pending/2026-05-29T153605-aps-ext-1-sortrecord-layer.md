---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - b4163965e69b150a8e297c9e0320cd425e715ce4
target_branch: main
summary: APS-EXT 1 SortRecord layer for Array.prototype.sort
risk_class: substrate
gates_pre:
  test262_sort_target: 0 PASS / 26 FAIL in post-EPSUA Array.prototype.sort pipeline rows
  build: null
  per_locale:
    array-prototype-sort: C4-positive 19-row precise accessor/prototype bucket open
gates_post:
  test262_sort_target: 25 PASS / 1 FAIL across 26 rows; precise bucket 19 PASS / 0 FAIL
  build: cargo build --release --bin cruft -p cruftless PASS
  per_locale:
    array-prototype-sort: APS-EXT 1 trajectory recorded
---

## Substrate Moves

Commit `b4163965e69b150a8e297c9e0320cd425e715ce4` lands APS-EXT 1.

- **M** = Array.prototype.sort SortRecord layer in `Runtime::array_proto_sort_via`.
- **T** = post-EPSUA sort precise accessor/prototype bucket plus adjacent sparse deletion.
- **I** = `pilots/rusty-js-runtime/derived/src/interp.rs::array_proto_sort_via` and `pilots/array-prototype-sort/trajectory.md`.
- **R** = Sort now collects present elements via accessor-aware `HasProperty` + `Get`, separates present non-`undefined` values from explicit `undefined` and absent holes, writes through Set, deletes trailing absent slots, and avoids non-Array array-like length mutation.

## Risk Assessment

The change is narrow to Array.prototype.sort. It reuses existing runtime primitives (`has_property_with_proxy`, `spec_get`, `reflect_set_via`, `reflect_delete_property_via`) rather than creating a parallel property protocol. An Array-only length floor is added only when setter side effects shrink length below the highest written element; this avoids restoring the prior non-Array `length` mutation bug.

Verification:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- Target 26-row sort set: 25 PASS / 1 FAIL.
- Precise accessor/prototype bucket: 19 PASS / 0 FAIL.
- Full `built-ins/Array/prototype/sort/*.js` mirror: 49 PASS / 5 FAIL versus baseline 28 PASS / 26 FAIL, with no previously passing rows regressed.

## Residuals

`call-with-primitive.js` remains as the only failure in the 26-row target set: BigInt/Symbol primitive receiver ToObject behavior. Four additional full-directory residuals involve resizable ArrayBuffer / typed-array-adjacent sort behavior and are outside APS-EXT 1.
