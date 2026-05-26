# plain-date-conversion — Trajectory

## PDC-EXT 0+1 — LANDED (2026-05-26)

Edit ~120 LOC: 3 conversion methods registered at end of install_temporal (where target prototypes pdt_proto/pmd_proto/pym_proto are in scope).

* toPlainDateTime(timeLike?): coerce arg (undefined → midnight | PT instance | PDT instance (extract time) | property bag) → make_pdt with PD's y/m/d
* toPlainMonthDay(): allocate PMD with this.month/day + refYear=1972
* toPlainYearMonth(): allocate PYM with this.year/month + refDay=1

Two-stage Rule 23: first cut placed PDC inline within PD-install block where pdt_proto/pmd_proto/pym_proto don't exist yet. Build failed; moved to end of install_temporal where all targets are in scope.

Probes — all 4 expected outcomes ✓

Yield: 0 → 20/50 PASS (40%). Diff-prod 42/42.

Cumulative Temporal yield post-PDC: 1271/2483 (51%).
