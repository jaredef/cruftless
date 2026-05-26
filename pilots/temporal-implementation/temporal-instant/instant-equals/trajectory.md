# instant-equals — Trajectory

## IE-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Fourth sub-rung of temporal-instant. Sibling shape to PTE.

### Edit (~40 LOC in intrinsics.rs)

`equals(other)`:
- Brand-check `this` via `__ti_ns`.
- If `other` is String → parse_iso_datetime → compose f64 ns.
- If `other` is Object → read `__ti_ns` BigInt → f64.
- Compare f64 ns; return Boolean.

### Probes (Rule 23 verification at landing)

- `a.equals(a)` → true ✓
- `a.equals(b)` → false (different ns) ✓
- `b.equals(c)` → true (same ns, different objects) ✓
- `a.equals("2009-02-13T23:31:30.123456789Z")` → true ✓

### Yield

- instant-equals exemplar pool (30): **0 → 18/30 PASS (60%)**.
- Diff-prod: 42/42.

Cumulative Temporal yield post-IE: **429/688 (62%)**.

### Residual decomposition (12)

| Shape | Count | Destination |
|---|---:|---|
| Extended year format (`-271821-04-20`) | ~2 | iso-extended-year-parse |
| HH-only time form (`T00Z` without `:MM`) | ~2 | iso-compact-time-parse |
| Critical-flag annotation rejection | ~3 | iso-annotation-validation |
| Multiple annotation rejection | ~2 | same |
| Sub-minute offset variants | ~3 | iso-offset-edge-cases |

### Status

IE-EXT 1 CLOSED. Instant now at 4 sub-rungs. Cumulative Temporal yield 62%.
