# plain-date-time-arithmetic — Trajectory

## PDTA-EXT 0+1 — LANDED (2026-05-26)

Edit ~350 LOC: pdt_duration_units + pdt_add_apply (composes PD calendar arithmetic on years+months+weeks+days then PT time arithmetic with day-carry) + add/subtract/since/until methods. since/until output Duration with days+hours+minutes+seconds+sub-second; default largestUnit ("auto" → days+time).

Probes — all 4 expected outcomes ✓ including:
  PDT(2020,5,15,14,30,45) + 1h30m -> '2020-05-15T16:00:45'
  PDT(2020,5,15,23,30) + 2h -> '2020-05-16T01:30:00' (day-carry)
  since() over 366-day diff (2021 - 2020) -> 366 days

Yield: 0 -> 100/275 PASS (36%). Diff-prod 42/42.

Cumulative Temporal yield post-PDTA: 1150/2307 (50%).
