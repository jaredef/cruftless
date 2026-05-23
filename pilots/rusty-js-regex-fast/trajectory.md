# rusty-js-regex-fast — Trajectory

Per-RXF-EXT log for the regex perf + memory-leak pilot.

---

## RXF-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 18:12-local. Standalone top-level locale per Doc 737 §IV. Second of four spawns in this round. Dual focus per keeper triage: regex perf + memory leak.

### Trigger

- Keeper-reported memory leak observation (surface unknown; needs reproduction)
- CRB-EXT 9 reading: regex is a component of string_url_sweep's 8.31× cruft/node gap
- Keeper directive enumerated 4 substrate gaps; this pilot addresses regex perf + leak

### Substrate delivered

- `seed.md` (~80 lines): dual-telos (leak + perf), 5 falsifiers Pred-rxf.1-.5, methodology RXF-EXT 0-9, carve-outs
- `trajectory.md` (this file)
- `docs/` + `fixtures/` scaffolds

### Locale registration

Locale count: 17 → 18 after this spawn (11 → 12 top-level). Manifest refresh queued.

### Open scope at RXF-EXT 0 close

1. RXF-EXT 1-3 — leak investigation (reproduce → bisect → fix-or-name)
2. RXF-EXT 4 — perf bench baseline
3. RXF-EXT 5+ — perf substrate moves

---

*RXF-EXT 0 closes. Pilot founded. Leak investigation is the natural first substrate-tier round (bounded; correctness-critical); perf bench baseline follows.*
