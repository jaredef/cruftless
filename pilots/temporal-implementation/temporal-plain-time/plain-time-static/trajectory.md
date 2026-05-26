# plain-time-static — Trajectory

## PTS-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Second sub-rung of temporal-plain-time. Sibling shape to DStat / TIS with PlainTime-specific ISO time parsing.

### Edit (~200 LOC in intrinsics.rs)

- Inline `parse_iso_time(&str) -> Option<[i64; 6]>` parser:
  - Optional date prefix YYYY-MM-DDT/t/space (datetime time-extraction per §11.7.1).
  - Optional leading T/t.
  - HH:MM required; HH:MM:SS optional; fractional via . or , (1-9 digits) — pads to nanoseconds; splits into ms/μs/ns slots.
  - Optional trailing Z/z or ±HH:MM offset (ignored for PlainTime per spec).
  - Range checks per §11.7.2.
- `from(item)`: PlainTime brand-check (clone via __pt_*), or property-bag (read 6 unit names, at-least-one required, range-validated), or string (parse_iso_time + sentinel install).
- `compare(a, b)`: inline `to_ns_of_day` helper for each arg (PlainTime, propertybag, or string); compute nanoseconds-of-day; return -1/0/1.

### Two-stage Rule 23 discovery

First cut (without offset/annotation suffix support): 47/83.
Second cut (added annotation acceptance via `[…]` consumption): 44/83 — REGRESSION. Probe surfaced spec tests that REQUIRE rejection of capital/critical annotations (`[U-CA=...]`, `[!u-ca=...]`). Reverted annotation acceptance; kept the offset (Z/±HH:MM) handling and the date-prefix support.
Final: 46/83 (+19 from offset support, -22 from annotation deferral — net positive).

### Probes (Rule 23 verification at landing)

- `Temporal.PlainTime.from(plainTimeInstance)` → clone with all 6 fields ✓
- `Temporal.PlainTime.from("08:44:15.321")` → {hour:8, minute:44, second:15, millisecond:321} ✓
- `Temporal.PlainTime.from({hour:10, minute:30})` → property bag form ✓
- `Temporal.PlainTime.compare("08:44:15", "14:23:30")` → -1 ✓
- `Temporal.PlainTime.compare("08:00", "08:00")` → 0 ✓
- `Temporal.PlainTime.compare(plainTimeObj, "10:00")` → 1 ✓
- `Temporal.PlainTime.from("invalid")` → RangeError ✓

### Yield

- plain-time-static exemplar pool (83): **0 → 46/83 PASS (55%)**.
- Diff-prod: 42/42 maintained.
- plain-time-ctor-fields stable at 32/34.

Cumulative Temporal yield post-PTS: **286/417 (69%)**.

### Residual decomposition (37 fails)

| Shape | Count | Destination |
|---|---:|---|
| Bracketed annotation rejection (`[!u-ca=...]`, `[U-CA=...]`) | 4 | plain-time-annotation-validation (per-spec rejection) |
| Leap-second `23:59:60` rejection | 2 | iso-time-leap-second-handling (spec deferred semantics) |
| Compact basic-form date / time forms | ~3 | iso-compact-form-parse |
| TypeError on wrong option types | 2 | spec-strict option-coercion |
| callee not callable Object (sibling class stubs) | 3 | per-class ctors |
| Tests requiring options.overflow or other options | ~10 | plain-time-options-handling |
| ISO time edge cases | ~13 | per-test inspection |

### Findings

**Finding PTS.1 (annotation grammar must be validated, not just consumed)**: First cut treated bracketed `[…]` as "skip to `]`". Spec requires validating annotation keys (lowercase only) and rejecting `[!key=val]` (critical annotation with unknown key). Accepting annotations blindly produced -3 yield because the rejection tests started passing the rejected-source check. Standing recommendation: bracketed annotation handling needs a separate sub-rung with proper grammar enforcement; deferring "annotation present" to a strict-validating parser is correct.

**Finding PTS.2 (per-class ISO parsers diverge from each other)**: IDP (Duration), IDTP (Instant datetime), and PTS's parse_iso_time (PlainTime time) are three separate parsers each tailored to its class. They share style but not code. Eventually a temporal-iso-parse-unified rung may factor them. For now, ~120-200 LOC each is the per-class cost; the LOC budget is acceptable.

### Status

PTS-EXT 1 CLOSED. Cumulative Temporal yield 69%. Next: plain-time-with (single method, ~30 tests) OR iso-fractional-propagation (Duration deferrals) OR temporal-tostringtag-descriptor (cross-cutting).
