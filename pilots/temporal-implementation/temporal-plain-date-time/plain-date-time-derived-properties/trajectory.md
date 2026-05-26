# plain-date-time-derived-properties — Trajectory

## PDTDP-EXT 0+1 — LANDED (2026-05-26)

Edit ~150 LOC: pdt_read_ymd + pdt_iso_week + pdt_getter! macro + 11 getter installations (reuses pda_days_from_civil/pda_is_leap/pda_days_in_month from PDA).

Probes — all 6 expected outcomes ✓ (PDT May 15 2020 → dow=5, doy=136, dim=31, diy=366, leap=true, week=20, yearOfWeek=2020).

Yield: 0 → 30/33 PASS (91%). Diff-prod 42/42.

Cumulative Temporal yield post-PDTDP: 1038/2002 (52%).
