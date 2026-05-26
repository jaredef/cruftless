---
name: plain-date-time-string-conversion
description: PDT toString/toJSON/toLocaleString — format 'YYYY-MM-DDTHH:MM:SS[.fff]' (no TZ designator).
type: project
---

PDTSC-EXT 1 LANDED 2026-05-26. ~80 LOC: pdt_read_all + pdt_to_iso_string composing date + 'T' + time portions with sub-second fractional + trim. 31/64 PASS (48%).
