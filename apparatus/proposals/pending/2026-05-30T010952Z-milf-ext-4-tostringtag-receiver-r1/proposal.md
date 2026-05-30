---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - df8e5899497816841aa457b4e77a9c391ab03b48
target_branch: main
summary: MILF-EXT 4 - Symbol.toStringTag descriptor receiver closure
risk_class: substrate
gates_pre:
  mongoose_smoke: FAIL receiver='toStringTag'
  slonik_smoke: FAIL receiver='toStringTag'
gates_post:
  focused_regression: cargo test --release -p rusty-js-runtime --test run_golden typed_array_tostringtag_descriptor_is_visible_by_symbol_key PASS
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  mongoose_smoke: advanced past receiver='toStringTag' to CJS parent-directory resolution residual
  slonik_smoke: PASS
---

## Substrate Moves

This closes the toStringTag receiver bug authorized by helmsman message `8fa1a44f-53f9-49e0-be40-636d83cfff9f`.

- **M** = `Object.getOwnPropertyDescriptor(proto, Symbol.toStringTag)` missed the existing typed-array `"@@toStringTag"` accessor because descriptor reflection lacked the well-known-symbol transitional fallback already used by property reads.
- **T** = route ordinary descriptor reflection through `get_own_property_descriptor_pk` and add Symbol-to-string fallback there.
- **I** = `pilots/rusty-js-runtime/derived/src/interp.rs`, focused `run_golden` regression, MILF trajectory, and deferral ledger entry for the newly surfaced CJS parent-directory resolution residual.
- **R** = `slonik` import PASS; `mongoose` no longer fails on `receiver='toStringTag'` and advances to a distinct `require("..")` resolution gap.

## Risk Assessment

The change is constrained to own-property descriptor reflection. The fallback only applies after true symbol-key lookup misses, and only to the existing transitional string key represented by the symbol's stored description. This aligns descriptor reflection with existing `object_get_pk` and `find_getter_pk` behavior rather than introducing package-specific behavior.

## Composes-With

- `pilots/missing-intrinsic-loader-failures/trajectory.md`
- `apparatus/docs/deferrals-ledger.md` Entry 012
- Sidecar artifacts:
  - `/home/jaredef/Developer/cruftless-r1-sidecar/results/milf-ext4-r1/post-repro.txt`
  - `/home/jaredef/Developer/cruftless-r1-sidecar/results/milf-ext4-r1/post-smoke.txt`

**APPROVED for push** per Helmsman MILF-EXT 4 directive.
