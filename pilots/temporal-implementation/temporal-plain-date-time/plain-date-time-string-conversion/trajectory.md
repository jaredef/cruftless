# plain-date-time-string-conversion — Trajectory

## PDTSC-EXT 0+1 — LANDED (2026-05-26)

Edit ~80 LOC: pdt_read_all + pdt_to_iso_string; 3 methods dispatch. Year ±YYYYYY for outside [0000, 9999]; sub-second fractional 9-digit zero-pad + trim.

Probes — all 4 expected outcomes ✓ including:
  PDT(2020,5,15,14,30,45) -> '2020-05-15T14:30:45'
  PDT(2020,5,15,14,30,45,123,456,789) -> '...45.123456789'
  PDT(2020,5,15) -> '2020-05-15T00:00:00'

Yield: 0 -> 31/64 PASS (48%). Diff-prod 42/42.
