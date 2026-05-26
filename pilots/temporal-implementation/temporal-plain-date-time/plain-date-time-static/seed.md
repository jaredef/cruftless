---
name: plain-date-time-static
description: PDT from + compare with PDT-friendly ISO datetime parser (no-offset accepted).
type: project
---

PDTS-EXT 1 LANDED 2026-05-26. ~250 LOC. parse_iso_pdt accepts ISO datetime without offset; offset Z/±HH:MM accepted and IGNORED per spec (PDT has no TZ). from(item) supports string/PDT/PD/property-bag. compare(a, b) tuple-compare. Also wires parse_iso_pdt into PDTE's string path (no-offset friendly).

56/112 PASS (50%). PDTE +1 sibling yield.
