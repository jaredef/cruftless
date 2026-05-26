---
name: plain-date-time-equals
description: PDT equals(other) — compares 9-field tuple. String coercion via parse_iso_datetime (requires offset; deferred PDT-no-offset variant).
type: project
---

PDTE-EXT 1 LANDED 2026-05-26. ~80 LOC. 20/41 PASS (49%). Residuals: ~10 equals(string-no-offset) deferred (PDT strings lack TZ); ~10 calendar/property-bag.
