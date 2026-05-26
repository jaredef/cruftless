# plain-date-time-static — Trajectory

## PDTS-EXT 0+1 — LANDED (2026-05-26)

Edit ~250 LOC in intrinsics.rs:
* parse_iso_pdt: ISO datetime parser tailored for PDT (no-offset friendly; offset accepted and ignored)
* make_pdt(rt, proto, [9-tuple]) helper
* from(item): string | PDT brand → clone | PD brand → time defaults zero | property-bag with required y/m/d
* compare(a, b): tuple compare; -1/0/1

Wires parse_iso_pdt into PDTE's string-path (replaces previous parse_iso_datetime which required offset).

Probes — all 4 expected outcomes ✓

Yield:
  PDTS: 0 -> 56/112 PASS (50%)
  PDTE sibling: 20 -> 21 PASS (no-offset strings)
  Diff-prod 42/42

Cumulative Temporal yield post-PDTS: 1008/1969 (51%). **Broke 1000 PASS for the first time.**
