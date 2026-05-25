# string-proto-method-length-and-split-limit — Seed

## Telos

Two coupled substrate gaps surfaced by matrix 2026-05-25 rank 18 (String.prototype.split no-feature-tag, 11):

1. **SPML.1 — function.length**: `String.prototype.{split, replace, replaceAll}.length` was 0 instead of the spec-mandated 2 (§22.1.3.{17,18,21}). The regexp module installs these methods AFTER prototype.rs's arity-2 stubs and uses a local `register_method` helper that hardcodes length=0.

2. **SPML.2 — split limit semantics**: ECMA-262 §22.1.3.21 step 6/7 — `limit = (limit === undefined) ? 2^32 - 1 : ToUint32(limit)`. cruft treated NaN as "no limit" instead of 0, so `"hello".split(/l/, "hi")` returned `["he","","o"]` (limit "hi" → NaN → ignored) instead of `[]` (NaN → ToUint32 → 0 → empty).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/regexp.rs` — installs `replace`, `replaceAll`, `split` on String.prototype (lines 786-852). Local `register_method` (line 1053) forces length=0.
- `crate::intrinsics::register_intrinsic_method` — spec-correct alternative that takes length param.

## Methodology

1. Switch the three String.prototype installations to use `register_intrinsic_method` with length=2.
2. Replace split's NaN-falls-through-to-Option<None> limit handling with `ToUint32` semantics (NaN/non-finite → 0; otherwise `trunc().rem_euclid(2^32) as u32`).
3. Early-return [] when limit==0.

## Carve-outs

- separator-override-tostring side-effect-ordering (3 tests): needs ToString(separator)-before-ToUint32(limit) evaluation order; separate sub-locale.
- separator-regexp.js (1 test): regex pattern unsupported — distinct RegExp substrate.

## Resume protocol

Read `trajectory.md` tail.
