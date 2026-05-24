# interp-hot-intrinsics — Trajectory

Per-IHI-EXT log for the interp-tier hot-intrinsic-IC table (cross-tier dual of HI).

---

## IHI-EXT 0 — 2026-05-24 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-24 04:31-local as the (d) pivot from string_url_sweep's component A/B probe.

### Trigger

- string_url_sweep CRB fixture: cruft 743 ms / node 90 ms (8.21× cruft/node).
- Component A/B probe identified header normalization loop = **332 ms (77% of cruft's wall-clock)**.
- Header loop body is interp-tier (for-of iterator protocol + multiple String intrinsic dispatches per inner iter).
- OSR + JIT-tier HI table can't fire (for-of body has many ops outside JIT alphabet).
- The structural pattern: interp-tier dispatch overhead per intrinsic call dominates.
- CharCode-EXT 2 established the interp-tier IC pattern for charCodeAt only (ad-hoc); this pilot generalizes to a table.

### Substrate delivered

- `seed.md` (~135 lines): telos, 7 constraints C1-C7, 5 falsifiers Pred-ihi.1-.5, methodology IHI-EXT 0-N+1, starter set + carve-outs.
- `trajectory.md` (this file).
- `docs/` + `fixtures/` scaffolds.

### Locale registration

Locale count: 24 → 25 after this spawn (13 → 14 top-level; 11 nested unchanged). Manifest refresh queued at end of IHI-EXT 0.

### Open scope at IHI-EXT 0 close

1. **IHI-EXT 1** — design doc: per-entry shape + Op::CallMethod dispatch integration + override-safety gate + per-entry LOC estimates.
2. **IHI-EXT 2** — infrastructure round: IcEntry types + IC_TABLE + dispatch integration + charCodeAt migration from CharCode-EXT 2 ad-hoc.
3. **IHI-EXT 3-N** — per-entry rounds: toLowerCase, trim, indexOf, slice.
4. **IHI-EXT N+1** — composition probe + Pred-ihi.* booking.

### Cumulative status

LOC delta: 0 (apparatus round only).

---

*IHI-EXT 0 closes. Engagement's second cross-tier standing instrument founded. IHI-EXT 1 designs per-entry shape + dispatch integration.*
