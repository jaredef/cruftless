---
name: plain-date-with
description: Seventh sub-rung of temporal-plain-date. Implements with(dateLike) partial-update.
type: project
---

# plain-date-with — Seed

Sibling shape to PTW/DWith. `with({year?, month?, day?})` returns new PlainDate with overridden fields; rejects Temporal-class instances; range-validates merged y/m/d.

PDW-EXT 1 LANDED 2026-05-26. 10/25 PASS (40%). Residuals: options.overflow + spec coercion order.
