# plain-date-static — Trajectory

## PDS-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Second sub-rung of temporal-plain-date.

### Edit (~150 LOC in intrinsics.rs)

- `parse_iso_date(&str) -> Option<(i64, i64, i64)>`: parse YYYY-MM-DD with optional T/t/space-prefixed tail (time + offset + annotations ignored). Leap-year-aware day validation.
- `make_plain_date(rt, proto, y, m, d)`: allocate instance with 4 sentinels (year/month/day/calendar="iso8601").
- `from(item)`: string → parse_iso_date; PlainDate brand → clone; property bag → range-validate + calendar check; missing y/m/d → TypeError.
- `compare(a, b)`: tuple compare on (year, month, day); -1/0/1.

### Probes

- `from("1976-11-18")` → y=1976, m=11, d=18 ✓
- `compare(d1, d2)` (d1 earlier) → -1 ✓
- `compare(d1, d1)` → 0 ✓
- `from({year, month, day})` → bag form ✓
- `from(plainDate)` → clone ✓
- `from("2020-05-15T12:30:00Z")` → date portion extracted, time ignored ✓
- `from("invalid")` → RangeError ✓

### Yield

- plain-date-static exemplar pool (113): **0 → 49/113 PASS (43%)**.
- plain-date-ctor-fields sibling yield: **28 → 33 (+5)** (from-not-callable residuals closed).
- Diff-prod: 42/42.

Cumulative Temporal yield post-PDS: **691/1317 (52%)**.

### Residual decomposition (64)

| Shape | Count | Destination |
|---|---:|---|
| Property bag without year/month/day (uses monthCode etc.) | 8 | plain-date-property-bag-extension |
| ZonedDateTime callee (test relativeTo) | 4 | per-class |
| Calendar annotation validation | 2 | iso-annotation-validation |
| valueOf in old test pattern | 2 | per-test |
| Object.getOwnPropertyDescriptors not coercible | 2 | likely propertyHelper issue |
| RangeError edge cases (extended year, etc.) | ~46 | iso-date-edge-cases + extended-year-parse |

### Status

PDS-EXT 1 CLOSED. PlainDate now at 2 sub-rungs.
