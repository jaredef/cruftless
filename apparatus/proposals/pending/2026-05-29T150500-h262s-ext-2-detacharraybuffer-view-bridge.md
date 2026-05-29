# H262S-EXT 2 detachArrayBuffer view bridge

**Proposed by**: codex-substrate-resolver-4
**Date**: 2026-05-29
**Target branch**: `main`
**Risk class**: guarded test262 host shim + TypedArray accessor bridge

## Proposed commits

- `b87721c1` - `test262 host: detach typed array backing buffers`

## Scope

Extend the guarded `$262.detachArrayBuffer` host shim so test262 detached-buffer rows that pass a TypedArray view's `.buffer` can exercise the runtime detached-buffer path. The runtime bridge exposes TypedArray view `.buffer` and `.byteOffset` through `Runtime::object_get` for views backed by `Runtime::typed_array_views`.

## Gate report

- `cargo build --release --bin cruft -p cruftless` PASS.
- Named exemplar `built-ins/TypedArray/prototype/lastIndexOf/detached-buffer-during-fromIndex-returns-minus-one-for-undefined.js` PASS.
- 89-row `built-ins/TypedArray/prototype/**/detached-buffer*.js` sweep:
  - before: 8 PASS / 81 FAIL, with 13 `$262.detachArrayBuffer: argument must be an ArrayBuffer` shim failures.
  - after: 61 PASS / 28 FAIL, with 0 shim argument failures.

## Residuals

The remaining 28 failures are runtime TypedArray detached semantics, not host-shim availability: detached receiver TypeError routing for some methods, detached-mid-fromIndex behavior for `includes`/`indexOf`/`lastIndexOf`, `join` detached-mid-separator behavior, `slice` species/custom-constructor detached ordering, and `subarray` detached handling.
