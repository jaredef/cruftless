# non-ctor-registration-discipline — Seed

**Locale tag**: `L.non-ctor-registration-discipline` (top-level)

**Status**: **CLOSED at NACR-EXT 1**.

**Workstream**: ECMA-262 §21.3 — built-in functions not identified as constructors lack a `[[Construct]]` internal slot. `new RegExp.prototype.test()`, `Reflect.construct(String.prototype.replace, ...)`, etc. must throw TypeError. Cruft's `intrinsics.rs::register_method` already enforces this discipline via `make_native_non_ctor`. The parallel `register_method` helpers in `regexp.rs` and `promise.rs` were copies that defaulted to `is_constructor=true`, breaking the discipline for RegExp.prototype.{replace, exec, test, ...} and Promise.prototype.{then, catch, finally}.

**Pre-scoping**: 16 not-a-constructor fixtures fail in the sample. 5 trace to the two buggy register_method helpers; 11 trace to missing methods (Math.f16round, JSON.rawJSON, Map.prototype.getOrInsert/getOrInsertComputed, Promise.allKeyed/allSettledKeyed, etc.) — separate sub-locales.

**Composes with**:
- ECMA-262 §21.3 built-in function classification; §10.2 ECMAScript Function Objects
- Existing `intrinsics.rs::register_method` discipline (already correct, served as model)

## I. Telos

Flip the two buggy `register_method` helpers to set `is_constructor: false`, mirroring the intrinsics.rs precedent.

## II. Apparatus

Edits (~6 LOC):
1. `regexp.rs::register_method`: replace `make_native(name, f)` with `crate::intrinsics::make_native_non_ctor(name, 0, f)`.
2. `promise.rs::register_method`: inline `FunctionInternals { ..., is_constructor: true }` → `is_constructor: false`.

## III. Verification

Probe: `isConstructor(RegExp.prototype.test)` and `isConstructor(String.prototype.replace)` and `isConstructor(Promise.prototype.then)` all return false (were true / mixed).

Exemplar (16 not-a-constructor tests, 5 in-scope for this round): PASS 0 → 5.

Regression: 200 random previously-passing across RegExp/Promise/String dirs: 200/200 preserved.

## IV. Carve-outs

- 11 not-a-constructor tests for MISSING methods (Math.f16round, JSON.rawJSON, Map.prototype.getOrInsert*, Promise.allKeyed/allSettledKeyed): require ADDING the method stubs. Separate sub-locales per method-family.

## V. Status

CLOSED at NACR-EXT 1.
