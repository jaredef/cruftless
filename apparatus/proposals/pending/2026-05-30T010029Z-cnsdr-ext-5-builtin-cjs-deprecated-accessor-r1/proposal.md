---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 7adce814375d9b79741db5153d2c50dda321f9ae
target_branch: main
summary: CNSDR-EXT 5 - builtin CJS namespace and deprecated-accessor residuals
risk_class: substrate
gates_pre:
  cnsdr_ext5_cluster: 0 PASS / 3 FAIL from focused readable-stream/events/winston shape sample
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  focused_loader_tests: cargo test --release -p rusty-js-runtime --test module_loader PASS
  cnsdr_ext5_cluster: 3 PASS / 0 FAIL from focused shape sample
---

## Substrate Moves

This closes the CNSDR residual cluster authorized by helmsman message `1bd02782-6e30-4371-b9ab-918d52853d6a`.

- **M** = builtin-shaped package namespace parity for `readable-stream` and `events`, plus package-specific deprecated-accessor overexposure for `winston`.
- **T** = split the closure by resolver identity: alias `readable-stream` to a package-shaped builtin namespace, complete the `events` builtin namespace keys, and filter only the Bun-suppressed deprecated `winston` keys from CJS namespace population.
- **I** = `cruftless/src/{stream,events,lib}.rs`, `pilots/rusty-js-runtime/derived/src/module.rs`, focused module-loader regressions, sidecar package-shape probe, and CNSDR trajectory update.
- **R** = `readable-stream`, `events`, and `winston` now match the Bun-measured namespace key surfaces; adjacent EXT 4 packages `proj4` and `decimal.js-light` held.

## Risk Assessment

The main regression risk is overbroad CJS namespace filtering or mutation of generic builtin module surfaces. The implementation constrains both axes:

- `readable-stream` uses a distinct `node:readable-stream` compatibility object, leaving `node:stream` unchanged.
- `events` additions are host-builtin namespace keys already present in Bun's module surface.
- The deprecated-accessor filter is gated by package identity from the resolved `node_modules` URL and only applies to `winston`.

## Composes-With

- `pilots/cjs-ns-shape-diff-residual/trajectory.md`
- Prior CNSDR-EXT 4 dual-package default-export namespace mirroring
- Sidecar artifact: `/home/jaredef/Developer/cruftless-r1-sidecar/results/cnsdr-ext5-r1/post-cruft-shapes.jsonl`

**APPROVED for push** per Helmsman CNSDR-EXT 5 directive.
