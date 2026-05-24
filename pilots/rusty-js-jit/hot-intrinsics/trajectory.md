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

---

## HI-EXT 1 — 2026-05-24 (design doc: table apparatus + per-entry LOC budget)

### Headline

Design-tier round. `docs/design.md` (~280 lines) specifies the IcEntry struct + static IC_TABLE registry + parse-table dispatch + translate-time IR-lower dispatch + extern pre-bind + per-entry use detection + override-safety gate (first-cut deferred). 4 risks named.

### Per-entry LOC breakdown

| component | LOC |
|---|---:|
| extern fn (ASCII fast-path or property read) | 10-15 |
| extern_sig builder | 3-5 |
| lower IR fn (bitcast + mask + call + push) | 15-25 |
| IcEntry literal in IC_TABLE | 5-7 |
| **total per entry** | **30-50** |

Closes Pred-hi.1 (≤50 LOC per entry).

### ParsedOp encoding

3 new variants replace OSR-EXT 6b's 3 ad-hoc variants:
- `ParsedOp::IcPropertyGet(u8)` — index into IC_TABLE; for PropertyGet kind
- `ParsedOp::IcMethodResolve(u8)` — GetProp side of MethodCall pair
- `ParsedOp::IcMethodCall(u8)` — CallMethod side; lookback-paired with prior IcMethodResolve

### Composition with OSR-EXT 6b

HI-EXT 2 migrates OSR-EXT 6/6b's ad-hoc charCodeAt + length entries into the table. Migration is behavior-neutral (no observable change); -66% CRB reclaim preserved.

### 4 named risks

R1 override safety (first cut deferred — Finding HI.1 candidate); R2 receiver-kind mismatch (tag-check guard deferred); R3 parse-time lookback brittleness (source-read verifies cruft's bytecode shape); R4 fn-pointer lifetime (SAFE per rule 9).

### Composition with prior corpus / engagement work

- **Doc 741 §V.1**: the pattern generalizes; this design materializes the generalization as a registration apparatus
- **OSR-EXT 6b empirical anchor**: charCodeAt + length entries preserved through migration
- **Standing rule 11 (5-axis)**: each new entry's spawn gated by rule 11 (component A/B usually not needed since the entry's contribution is bounded to its intrinsic; op-set + value-domain + locals-marshaling + emission-shape axes checked per entry)
- **Standing rule 9 (raw-pointer audit)**: applied; extern_ptr is fn-item static → SAFE
- **Findings II.2-bis substrate-introduction signature**: HI-EXT 2 will be flat-bench (migration preserves behavior); HI-EXT 3+ per-entry rounds may show small wins on per-entry synthetic fixtures

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier).
Per Doc 734 §V: growth (c) preparatory — design's per-entry LOC budget anchors Pred-hi.1; the apparatus enables compounding engagement value beyond per-pilot ad-hoc work.
Per Doc 735 §X.h.c: three-probe-levels per entry at HI-EXT 3+.

### Open scope at HI-EXT 1 close

1. **HI-EXT 2** — infrastructure round: IcEntry/IcEntryKind/ReceiverKind types + IC_TABLE static + 2-entry migration (charCodeAt + length) + parse/translate dispatch + extern pre-bind apparatus. ~200 LOC; net delta ~50 after removing ad-hoc OSR-EXT 6b code.
2. **HI-EXT 3-N** — per-entry rounds (one or two entries per round).

### Cumulative status at HI-EXT 1 close

LOC delta: ~280 (design doc). HI-EXT 0-1 cumulative: ~400 across the locale.

---

*HI-EXT 1 closes. Table apparatus designed; per-entry LOC budget 30-50. HI-EXT 2 implements infrastructure + migrates charCodeAt + length.*
