# annexb-string-utilities — Trajectory

## ASU-EXT 0 — founding + EXT 1 substrate (2026-05-27)

Per keeper directive Telegram 10089 ("Shift to another high yield locale"). Matrix coordinate scanned post-TAMM closure: rank 13 (`runtime/spec-builtins :: missing-global-or-binding`, 406 fails) had AnnexB-built-ins among its representative shapes; rank-13 example `annexB/built-ins/escape/argument_bigint.js`. Per directive 10070 (avoid parallel-agent surface): parallel agent has been working Temporal/PlainTime grammar tranche (commit 2e56d61a "Close PlainTime from grammar tranche") — AnnexB legacy globals are non-overlapping.

**Founding apparatus**:
- `exemplars/exemplars.txt` — 59 stratified paths across `annexB/built-ins/{escape (16), unescape (19), Date (24)}`.
- `exemplars/run-exemplars.sh` — runner with per-target breakdown.

**Baseline + EXT 1 substrate (combined per rule 23 fast-close)**:

Pre-install probe confirmed `escape` / `unescape` undefined globals and `Date.prototype.toGMTString` undefined slot.

EXT 1 substrate (~85 LOC in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`):
- `escape(s)` per §B.2.1.1: UTF-16 walk; keep `[A-Za-z0-9@*_+-./]`, else `%XX` for code points ≤ 0xff, else `%uXXXX`.
- `unescape(s)` per §B.2.1.2: UTF-16 walk; `%uXXXX` → 16-bit code unit, `%XX` → 8-bit code unit, else pass-through. Lossless across the encode/decode pair for code points in the BMP.
- `Date.prototype.toGMTString` per §B.2.3.4: same function object semantics as `Date.prototype.toUTCString` (delegates to `generated::date_prototype_to_utc_string`).

**Yield**:
```text
AnnexB cluster POST-EXT 1: PASS=37 FAIL=22 / 59 (62.7%)
  remaining: 12 Date, 5 escape, 5 unescape
```

The pre-install baseline was effectively the failure floor for the escape (16) + unescape (19) subset since both globals errored at lookup; Date had partial pass with toGMTString missing. Estimated pre-install yield ≤19/59. Post-install +18.

**Gates**: build clean; diff-prod 59/53 (parity preserved).

**Finding ASU.1 (residual is format-string-shaped, not slot-shaped)**: the 12 Date residuals exercise `toGMTString` output format — they fail because cruft's underlying `date_prototype_to_utc_string` emits `1970-01-01 00:00:00 GMT` while spec requires RFC 7231 `Thu, 01 Jan 1970 00:00:00 GMT`. The 5+5 escape/unescape residuals likely probe edge cases (BigInt arg coercion, surrogate-pair round-trip, exact `%XX` casing). These belong to a Date-format locale + a separate string-encoding locale, not this one. Per the seed's carve-outs.

**Status**: ASU-EXT 1 CLOSED locally. Locale at +18 yield from this rung; further closure of the residual requires substrate moves outside the AnnexB-utilities slot-installation scope.
