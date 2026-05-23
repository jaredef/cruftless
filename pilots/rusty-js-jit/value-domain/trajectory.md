# rusty-js-jit/value-domain — Trajectory

Per-VD-EXT log for the Φ-encoding extension pilot (closes the value-domain coverage tier per Doc 740 §II.2 + Finding VII.3).

---

## VD-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 21:51-local as the (α) Φ-encoding extension pivot from TL locale's (b-narrow) chapter close. Nested under LeJIT per Doc 737 §IV.

### Trigger

- TL findings.md Finding TL.2 (engagement-promoted as VII.3 at findings.md Addendum V): Φ calling convention encodes only Number + Object; non-Number/Object Values degrade to 0.0.
- TL pilot's (b-narrow) Moves 3+4 structurally blocked at the encoding tier. Pivot to (b-architectural).
- Two co-equal architectural targets surfaced; keeper selected (α) Φ-encoding extension as the load-bearing prerequisite tier for any future Value-non-Number JIT-IC work.

### Substrate delivered

- `seed.md` (~120 lines): telos, 8 constraints C1-C8, 5 falsifiers Pred-vd.1-.5, methodology VD-EXT 0-7, carve-outs.
- `trajectory.md` (this file).
- `docs/` + `fixtures/` scaffolds.

### Locale registration

Locale count: 21 → 22 after this spawn (13 top-level unchanged; 8 → 9 nested under LeJIT). Manifest refresh queued at end of VD-EXT 0.

### Open scope at VD-EXT 0 close

1. **VD-EXT 1** — encoding design doc (NaN-boxing scheme; bit layout; tag values; encoder + decoder reference)
2. **VD-EXT 2** — encoding implementation (extend unbox_arg_f64 + add box_to_value)
3. **VD-EXT 3** — composition probe + fuzz + diff-prod gate
4. **VD-EXT 4-7** — follow-on Value variants + default-on confirmation

### Cumulative status

LOC delta: 0 (apparatus round only).

---

*VD-EXT 0 closes. Pilot founded as the (α) Φ-encoding extension. VD-EXT 1 designs the NaN-boxing scheme.*
