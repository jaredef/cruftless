---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - e2ed677a0fb6915d097adf5c4434c0864da9876e
target_branch: main
summary: CNSDR-EXT 4 - dual-package default-export namespace mirroring
risk_class: substrate
gates_pre:
  cnsdr_ext4_cluster: 0 PASS / 5 FAIL from focused shape-diff sample
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  focused_loader_tests: cargo test --release -p rusty-js-runtime --test module_loader dual_package_default PASS
  cnsdr_ext4_cluster: 2 PASS / 3 FAIL from focused shape-diff sample
---

## Substrate Moves

This closes the dominant submechanism of the five-package CNSDR cluster authorized by helmsman message `7058f44f-dd1a-4644-bc28-451132d70cb7`.

- **M** = dual-package ESM namespace under-mirroring for bare imports with `main` + `module` and no `exports`.
- **T** = extend the existing dual-package gate so an already-present default export mirrors its own properties into the namespace, matching Bun's sibling behavior to the earlier synthesized-default closure.
- **I** = `pilots/rusty-js-runtime/derived/src/module.rs`, focused module-loader regressions, and the CNSDR trajectory entry recording the mechanism split and deferral.
- **R** = `proj4` and `decimal.js-light` now PASS; the focused five-package CNSDR cluster improved from 0/5 to 2/5.

## Risk Assessment

The main regression risk is broadening namespace mirroring beyond the dual-package shape and disturbing prior ESM/CJS interop closures. The implementation stays inside the already-proven gate: `main` present, `module` present, `main != module`, no `exports`. Existing namespace entries are preserved rather than overwritten, and only the default export's own properties are mirrored. The remaining three cluster rows were explicitly deferred because they are a different mechanism family.

## Composes-With

- Prior dual-package default synthesis closure in `pilots/rusty-js-esm/trajectory.md`
- `pilots/cjs-ns-shape-diff-residual/trajectory.md`
- Focused artifact: `/tmp/cnsdr-ext4-five-results.json`

**APPROVED for push** per Helmsman CNSDR-EXT 4 directive.
