# rusty-js-jit/hot-intrinsics — Trajectory

Per-HI-EXT log for the hot-intrinsic-IC table engagement-wide instrument.

---

## HI-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-24 02:39-local as the engagement-wide instrument materialization (option δ from OSR-EXT 6b's forward-options offer). Generalizes the OSR-EXT 6b + CharCode-EXT 2 hot-intrinsic-IC pattern (validated on charCodeAt with -66% CRB reclaim on json_parse_transform) into a multi-intrinsic TABLE — a reusable apparatus that future pilots extend with new intrinsics at bounded per-entry LOC.

### Trigger

- The 2026-05-23 architectural-pivot session demonstrated the hot-intrinsic-IC pattern at OSR-EXT 6b. Doc 741 §V.1 noted the pattern generalizes. Keeper directive: "Now create the engagement wide instrument."
- The pattern's reusability requires a registration-based apparatus where each new intrinsic costs ~30-50 LOC (vs ~150 LOC ad-hoc per OSR-EXT 6b's first-cut shape for charCodeAt).
- Starter set of 6 intrinsics enumerated per realistic-workload frequency.

### Substrate delivered

- `seed.md` (~120 lines): telos, 7 constraints C1-C7, 5 falsifiers Pred-hi.1-.5, methodology HI-EXT 0-N+1, starter set + carve-outs.
- `trajectory.md` (this file).
- `docs/` + `fixtures/` scaffolds.

### Locale registration

Locale count: 23 → 24 after this spawn (13 top-level unchanged; 10 → 11 nested under LeJIT). Manifest refresh queued at end of HI-EXT 0.

### Open scope at HI-EXT 0 close

1. **HI-EXT 1** — design doc: IcEntry struct + registration shape + parse-table-lookup + IR-lowering-dispatch + per-entry LOC estimates for the starter set.
2. **HI-EXT 2** — infrastructure round: IcEntry struct + table registry + parse-table dispatch + translator integration. charCodeAt + length entries migrated from OSR-EXT 6/6b's ad-hoc form into the table.
3. **HI-EXT 3-N** — per-entry additions (charAt; codePointAt; Array.length; Array.push; …).
4. **HI-EXT N+1** — composition probe + final disposition + Pred-hi.* booking.

### Cumulative status

LOC delta: 0 (apparatus round only).

---

*HI-EXT 0 closes. Engagement-wide instrument pilot founded. HI-EXT 1 designs the table apparatus; HI-EXT 2 implements infrastructure + migrates existing charCodeAt/length entries; HI-EXT 3-N adds starter-set entries per round.*
