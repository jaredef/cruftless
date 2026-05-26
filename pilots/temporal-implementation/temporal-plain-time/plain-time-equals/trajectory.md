# plain-time-equals — Trajectory

## PTE-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Fifth sub-rung of temporal-plain-time. Sibling shape to PTS (PlainTime.from).

### Edit (~80 LOC in intrinsics.rs)

`equals(other)`:
- Brand-check `this` via `__pt_hour`.
- If `other` is String → parse via `parse_iso_time` (hoisted via block-scoped fn), compare units.
- If `other` is Object: brand-check (`__pt_hour`) or property-bag (at-least-one-unit + range-validate).
- Compare all 6 unit values; return Boolean.

### Two-stage Rule 23 discovery

First cut: rejected string arg (yielded 10/31). Probe surfaced 8 equals(string) tests in residuals. Wired parse_iso_time (hoisted fn) into equals; yield rose 10 → 21.

### Probes (Rule 23 verification at landing)

- `t1.equals(t1)` → true ✓
- `t1.equals(t1bis)` → true (different object, same fields) ✓
- `t1.equals(t2)` → false (different fields) ✓
- `t1.equals({hour:8,...})` → true (property bag) ✓

### Yield

- plain-time-equals exemplar pool (31): **0 → 21/31 PASS (68%)**.
- Diff-prod: 42/42.

Cumulative Temporal yield post-PTE: **411/658 (62%)**.

### Residual decomposition (10)

| Shape | Count | Destination |
|---|---:|---|
| Annotation rejection (`[!u-ca=...]` critical, `[U-CA=...]` uppercase) | ~4 | plain-time-annotation-validation |
| UTC designator on bare time string | ~2 | strict-time-no-utc |
| Variant minus sign / RFC 3339 edges | ~2 | iso-edge-cases |
| Sibling class callee | 2 | per-class |

### Status

PTE-EXT 1 CLOSED. PlainTime now at 5 sub-rungs: ctor + static + with + string-conversion + equals. Cumulative Temporal yield 62%.
