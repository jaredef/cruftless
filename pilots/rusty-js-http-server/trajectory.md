# rusty-js-http-server — Trajectory

Per-HS-EXT log for the HTTP server pilot.

---

## HS-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 18:12-local. Standalone top-level locale per Doc 737 §IV. Fourth of four spawns in this round.

### Trigger

- Keeper-named substrate gap: HTTP server APIs currently stubbed
- Blocks Node packages (express, fastify, koa, etc.) from running on cruft
- Multi-session substrate workstream; this round is apparatus-tier founding only

### Substrate delivered

- `seed.md` (~85 lines): telos (HTTP/1.1; single-connection first cut; Express compat target), 5 falsifiers Pred-hs.1-.5, methodology HS-EXT 0-9, carve-outs (HTTP/1.1 only; buffered body; no concurrent connections at first cut)
- `trajectory.md` (this file)
- `docs/` + `fixtures/` scaffolds

### Locale registration

Locale count: 19 → 20 after this spawn (12 → 13 top-level; 7 nested unchanged). Manifest refresh queued.

### Open scope at HS-EXT 0 close

1. HS-EXT 1 — survey existing TCP/socket substrate
2. HS-EXT 2 — HTTP/1.1 parser design
3. HS-EXT 3-9 per seed §III

---

*HS-EXT 0 closes. Pilot founded. Largest scope of the 4 spawns this round; multi-session substrate workstream when activated.*
