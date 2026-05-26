# temporal-iso-string-parse — Trajectory

## TISP-EXT 0+1 — FOUNDED + first sub-rung LANDED (2026-05-26)

Spawned per keeper directive (Telegram 9895) following Finding TIS.1's standing-rec to prioritize ISO-string-parse as the unblocker for static methods.

## IDP-EXT 1 — iso-duration-parse LANDED (2026-05-26)

See `iso-duration-parse/trajectory.md` for substrate detail. Summary:
- `parse_iso_duration(&str) -> Option<[f64; 10]>` hand-written state-machine parser. ~120 LOC.
- Wired into Temporal.Duration.from(string) + Temporal.Duration.compare(string), replacing the prior "Tier-L stub" deferral with real parses.
- Yield: +5 across Duration sub-rungs (DStat 23→27, DDP 23→24). DCF + DWith unchanged.
- Diff-prod 42/42 maintained.

Cumulative Temporal yield post-IDP: 183/300 (61%).

### Finding TISP.1 (shared sub-substrate rungs yield is sibling-distributed, not in-locale)

iso-duration-parse has no exemplar surface of its own — its impact is measured by the +5 yield across DStat and DDP. Standing recommendation: track shared-substrate yield in the parent locale's trajectory as a cumulative cross-sibling delta, not as a per-rung "this many tests passed."
