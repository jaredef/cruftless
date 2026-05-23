# rusty-js-jit/top-level — Trajectory

Per-TL-EXT log for the top-level JIT extension pilot.

---

## TL-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 20:52-local as the (b-narrow) instantiation of Doc 740's multi-tier reading. Nested under LeJIT per Doc 737 §IV.

### Trigger

- JSF-EXT 10 CRB measurement: cumulative -12% across JSF M1-M4 + CharCode-EXT 1+2; cruft/node 17.93×; residual checksum loop 1480 ms.
- Doc 740 §III.4 multi-tier closure analysis: substrate + dispatch closed; remaining cost lives at interp loop dispatch per iter.
- Recon (2026-05-23 ~20:48-local): identified 3 JIT alphabet gaps (PushConst, GetProp, CallMethod) preventing the inner for-loop body from JIT-eligibility; no OSR mechanism; module bytecode never enters JIT.

### Substrate delivered

- `seed.md` (~135 lines): telos, 8 constraints C1-C8, 5 falsifiers Pred-tl.1-.5, methodology TL-EXT 0-7, carve-outs.
- `trajectory.md` (this file).
- `docs/` + `fixtures/` scaffolds.

### Locale registration

Locale count: 20 → 21 after this spawn (12 top-level unchanged; 7 → 8 nested). Manifest refresh queued at end of TL-EXT 0.

### Open scope at TL-EXT 0 close

1. **TL-EXT 1** — design doc enumerating per-move substrate plan.
2. **TL-EXT 2-5** — implementation per the design.
3. **TL-EXT 6** — composition probe + CRB final disposition.

### Cumulative status

LOC delta: 0 (apparatus round only).

---

*TL-EXT 0 closes. Pilot founded as (b-narrow) first cut. TL-EXT 1 designs the per-move substrate plan.*
