# plain-date-time-equals — Trajectory

## PDTE-EXT 0+1 — LANDED (2026-05-26)

Edit ~80 LOC: brand-check + string parse (via parse_iso_datetime; requires Z/±HH:MM offset) + PlainDateTime brand → read all 9 fields; tuple compare; Boolean.

Yield: 0 → 20/41 PASS (49%). Diff-prod 42/42.

Residual: equals(string-no-offset) — parse_iso_datetime requires offset for Instant; PDT should accept no-offset form. Defer to pdt-iso-datetime-no-offset-parse rung.

Cumulative Temporal yield post-PDTSC+PDTE: 951/1857 (51%).
