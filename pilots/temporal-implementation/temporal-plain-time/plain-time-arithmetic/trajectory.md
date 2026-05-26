# plain-time-arithmetic — Trajectory

## PTA-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Sixth sub-rung of temporal-plain-time. Sibling shape to IA.

### Edit (~250 LOC in intrinsics.rs)

- `duration_to_subday_ns_pt(rt, v)`: coerce arg; reject year/month/week (days+ OK); compose total ns including days*24h.
- `pt_ns_of_day(rt, id)`: read 6 PT sentinels; compose ns-of-day.
- `pt_from_ns_of_day(rt, proto, ns)`: rem_euclid(NS_PER_DAY) then decompose to 6 unit sentinels.
- `diff_to_pt_duration(rt, dur_proto, diff_ns)`: normalize to (-12h, +12h]; decompose to signed-magnitude hours+min+sec+sub-second Duration.
- `coerce_pt_to_ns(rt, v)`: PlainTime brand-check / property-bag / ISO time string via parse_iso_time.
- add/subtract/since/until methods dispatch to these helpers.

### Probes (Rule 23 verification at landing)

- `new PlainTime(15,23,30).add(Duration(0,0,0,0,1,30))` → 16:53:30 ✓
- `.subtract(Duration(...,1,30))` → 13:53:30 ✓
- `new PlainTime(23,30).add(Duration(0,0,0,0,2))` → 1:30 (wrapped) ✓
- `new PlainTime(15,30).since(new PlainTime(14,0))` → 1h 30m ✓
- `t.add(Duration(1))` (year) → RangeError ✓

### Yield

- plain-time-arithmetic exemplar pool (214): **0 → 82/214 PASS (38%)**.
- Diff-prod: 42/42.

Cumulative Temporal yield post-PTA: **577/1098 (53%)**.

### Residual decomposition (132)

Same shape as IA-EXT 1: options.largestUnit/smallestUnit/roundingMode (~70+), TypeError on wrong option types, sibling annotation/IDTP edges.

### Findings

**Finding PTA.1 (per-class arithmetic pattern transfers across classes)**: IA + PTA both use the same skeleton: coerce_arg + read_field_sentinels + compute_total + dispatch_to_construct. Per-class LOC ~250. Standing recommendation: future Duration.add/subtract (and the eventual PlainDate.add/subtract that adds calendar arithmetic) reuse the same shape; calendar adds compose with the existing fields-based path.

### Status

PTA-EXT 1 CLOSED. PlainTime now at 6 sub-rungs (parity with Duration count: ctor + static + with + string + equals + arithmetic). Cumulative Temporal yield 53%.
