---
name: missing-intrinsic-loader-failures
description: Top500 dynamic-import failures where loader execution reaches a missing intrinsic, prototype method, global, or Node/Bun compatibility shim.
type: project
---

# missing-intrinsic-loader-failures — Seed

## Substrate-pilot — top500 loader intrinsic residual cluster (MILF)

Spawned per Helmsman CAACP directive `51049de5-ceb9-4f10-879a-577410955ced`, Round 10. The directive scoped this first rung to Phase 0 + Phase 2 only: found the locale, baseline-inspect the cluster, segment causes, and propose a Phase 3 move shape. No substrate code change belongs in this rung.

## Telos

Dynamic import of popular Node ecosystem packages should not fail because the runtime returns `undefined` where Bun exposes an intrinsic, prototype method, global binding, or Node/Bun compatibility shim.

The initial cluster shape combines two reason families from the top500 sweep:

- `callee is not callable: undefined`
- `Cannot read property <name> of undefined`

This locale determines whether those families are one root cause (for example one missing intrinsic chain) or multiple independent substrate gaps.

## Apparatus

- Directive source data: `/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json`.
- Helmsman update named replacement source data under `/home/jaredef/Developer/cruftless-sidecar/parity-results/`:
  - `cluster-callee-not-callable.json`
  - `cluster-cannot-read-property.json`
  - `parity-results-top500-20260529T111702-refined.json`
- Fallback available in this checkout: `legacy/host-rquickjs/tools/parity-results-cluster-dyn-import.json`, which contains the same named exemplar packages and stack-shaped dynamic-import failures.
- Package sandbox paths in the error messages point under `/media/jaredef/T7/rusty-bun/parity-sandbox/...`.

## Methodology

Phase 2 samples packages from both reason families and classifies the first observable failure into one of:

1. Missing built-in method on Array/Object/String/Buffer/DataView/typed-array prototype.
2. Missing Node-compat shim (`path`, `fs`, `crypto`, `buffer`, streams, events, util, process, etc.).
3. Missing global or web global (`process.X`, `globalThis.X`, `self/window` polyfill, DOM/EventTarget classes).
4. Wrong prototype chain or wrong namespace object shape, where cruft returns an ordinary object or `undefined` but Bun exposes a method-bearing object/function.
5. Non-intrinsic semantic/runtime gap, if neither error family points at a missing intrinsic surface.

## C4 reason-coherence gates

- C1 sibling: holds if at least two rows share the same missing surface and failure shape.
- C2 shape-compat: holds only if one Phase 3 move can fix multiple rows without mixing independent substrate coordinates.
- C3 cost-positive: holds if the proposed substrate move is lower cost than per-package compatibility shims.
- C4 bail-safe: holds if absent source data or package sandbox prevents false precision; such rows stay classified as probe-limited, not substrate-proven.

## Composes-with

- `pilots/dynamic-import-attributes/` for import-call syntax and runtime-stub boundaries.
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` for global/prototype intrinsic installation.
- Node-compat pilots under `pilots/node-*` and host stubs in `cruftless/src/node_stubs.rs`.
- Loader/module substrate in `pilots/rusty-js-runtime/derived/src/module.rs`.

## Status

**Status**: PHASE 2 PROBED WITH BLOCKED EXACT-SOURCE READ. Both the original `/media/jaredef/T7/...` path and the updated `/home/jaredef/Developer/cruftless-sidecar/parity-results/...` paths were unavailable on this host; fallback dynamic-import cluster evidence is recorded in `trajectory.md`. Phase 3 should begin from a fresh read of the directive JSON or the mounted sidecar/T7 parity sandbox.
