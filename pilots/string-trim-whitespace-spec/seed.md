# string-trim-whitespace-spec — Seed

**Locale tag**: `L.string-trim-whitespace-spec` (top-level)

**Status**: **CLOSED at SPTW-EXT 1**.

**Workstream**: ECMA-262 §12.2 WhiteSpace + §12.3 LineTerminators + §22.1.3.30.1 TrimString — `String.prototype.trim/trimStart/trimEnd` recognize a specific set including U+FEFF (ZWNBSP/BOM, retained by ES spec for backward compatibility but excluded from Unicode's White_Space property), NBSP (U+00A0), USP set (Zs category). Cruft's trim delegated to Rust's `str::trim` (Unicode White_Space) and the IC fast path had a narrow ASCII-only set; both surfaced 22 test262 failures.

**Trigger**: post-CP-arc trajectory continuation; the trim cluster surfaced as #5 (22 tests, single coherent root cause) in the matrix.

**Composes with**:
- ECMA-262 §12.2 / §12.3 / §22.1.3.30.1
- [Finding T262C.4 + EPSUA.6](../ecmascript-parity-shared-upstream-arc/trajectory.md) — discriminator + sub-cluster sizing

## I. Telos

Fix String.prototype.trim/trimStart/trimEnd to recognize the full ES-spec whitespace + line-terminator set. Substrate is two-tier:
- Slow path: `string_proto_trim_via` (etc.) used Rust's `str::trim` (Unicode White_Space property; omits ZWNBSP).
- Fast path: `fast_string_trim` IC entry had narrow ASCII set + an early-return bug that bailed-as-success when non-ASCII strings had no ASCII-whitespace prefix/suffix.

## II. Apparatus + Methodology

R = {is_es_whitespace_or_lineterm helper, slow-path trim/trimStart/trimEnd switch, IC fast-path carve-back}.

Edits (~35 LOC):
1. `interp.rs`: module-fn `is_es_whitespace_or_lineterm(char) -> bool` matching §12.2 + §12.3 exactly; `es_trim`, `es_trim_start`, `es_trim_end` wrappers.
2. `interp.rs::string_proto_trim_via` / `_start_via` / `_end_via`: switch from `str::trim*` to `es_trim*`.
3. `interp_ic_table.rs::fast_string_trim`: move the `!s.is_ascii() → bail` check BEFORE the trim-byte-scan + early-return. Pre-fix the early-return "no trim needed" fired on non-ASCII strings whose first/last byte happened to be the leading byte of a multi-byte UTF-8 sequence (not in the ASCII WS set), leaving NBSP/BOM unstripped.

## III. Carve-outs

- The "trim called with non-string receiver" test (`String.prototype.trim.call(argObj) === "[object Arguments]"`) is a separate concern (Object.prototype.toString brand surface; not whitespace classification). Out of SPTW scope.
- TypedArray / DataView whitespace handling: N/A.

## IV. Verification

Probes (all GREEN):
- " abc ".trim() → "abc" (ASCII, unchanged behavior)
- "\u{00A0}abc\u{00A0}".trim() → "abc" (NBSP stripped; was NOT before)
- "\u{FEFF}abc\u{FEFF}".trim() → "abc" (BOM stripped; was NOT before)

Exemplar (built-ins/String/prototype/trim/* fixtures, 22 failing pre-fix): PASS 0 → **21** (+21).
Remaining 1: trim-called-on-Arguments-object — unrelated brand-toString concern.

Regression: String/prototype previously-passing (494): 494/494 preserved.

## V. Status

CLOSED at SPTW-EXT 1.
