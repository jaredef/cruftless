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

---

## HI-EXT 2 — 2026-05-24 (infrastructure round; 2-entry migration; behavior-neutral)

### Headline

Table apparatus landed at JIT crate's new `ic_table.rs` module. charCodeAt + length entries migrated from OSR-EXT 6/6b's ad-hoc form into the table. Behavior-neutral: synth do-while still 11ms; json_parse_transform OSR still compiles + invokes; correctness probes GREEN. ~280 LOC added (ic_table.rs); ~150 LOC removed from translator.rs (ad-hoc paths); net delta ~130 LOC.

### Substrate landed

1. **New `pilots/rusty-js-jit/derived/src/ic_table.rs`** (~190 LOC):
   - `IcEntry` struct + `IcEntryKind` (PropertyGet | MethodCall {arity}) + `ReceiverKind` (String | Array | Number)
   - 2 entries: String.length (PropertyGet) + String.charCodeAt (MethodCall arity 1)
   - Per-entry extern fns: `ic_string_len` + `ic_string_char_code_at` (relocated from OSR-EXT 6/6b's osr_* names)
   - Per-entry sig builders + lower fns
   - `pub static IC_TABLE: &[IcEntry]` registry
   - `lookup_by_key(key) -> Option<u8>` helper
   - `lower_ic_method_resolve(...)` helper for the GetProp side of method-call pair
   - `unsafe impl Sync for IcEntry` (fn pointers + extern_ptr are SAFE per rule 9)

2. **Module exposure**: `pub mod ic_table;` in lib.rs.

3. **translator.rs refactor**:
   - 3 ParsedOp variants removed (GetPropLength, GetPropCharCodeAt, CallMethodCharCodeAt)
   - 3 new variants added (IcPropertyGet(u8), IcMethodResolve(u8), IcMethodCall(u8))
   - Op::GetProp parse arm consults IC_TABLE via lookup_by_key
   - Op::CallMethod parse arm uses backward-scan over parsed list for IcMethodResolve with matching arity (initial lookback-at-last attempt failed because `LoadLocal arg` sits between GetProp and CallMethod; fix: scan backwards)
   - Ad-hoc `has_getprop_length` + `has_callmethod_charcodeat` scans replaced by `ic_entry_used` Vec scan
   - Ad-hoc extern symbol pre-bind + sig declarations replaced by per-entry loop
   - Ad-hoc FuncRef declarations replaced by per-entry Vec<Option<FuncRef>>
   - 3 translate arms collapsed to 2 arms (IcPropertyGet | IcMethodCall via entry.lower; IcMethodResolve via lower_ic_method_resolve)
   - 2 osr_string_* extern fns removed from translator.rs (now in ic_table.rs)

### Migration verification (behavior-neutral)

| probe | pre-migration (post OSR-EXT 6b) | post-migration | Δ |
|---|---|---|---|
| canonical fuzz (acc=-932188103) | GREEN | GREEN | unchanged |
| diff-prod 42/42 | GREEN | GREEN | unchanged |
| JIT lib tests | 38/38 | 38/38 | unchanged |
| synth do-while wall-clock | 10-11ms | 10-11ms | unchanged |
| json_parse_transform OSR compile | OK + INVOKE FIRED | OK + INVOKE FIRED | unchanged |
| A/B json_parse_transform (3-run sample) | 1176 median | 1270 median | within noise |
| CRB json_parse_transform | 834 ms | (not re-bench; behavior identical) | unchanged by construction |

### Per-entry LOC budget verified (HI-EXT 3+ template)

| component | length entry LOC | charCodeAt entry LOC |
|---|---:|---:|
| extern fn | 5 | 12 |
| extern_sig builder | 4 | 5 |
| lower fn | 11 | 13 |
| IcEntry literal | 8 | 8 |
| **per entry total** | **28** | **38** |

Within Pred-hi.1's ≤50 LOC budget. Future entries (HI-EXT 3+) add at same scale.

### Composition with prior corpus / engagement work

- **Doc 741 §V.1 generalization**: this round materializes the generalization. The pattern is now a reusable table; future entries register via the static.
- **OSR-EXT 6/6b**: ad-hoc entries migrated; OSR-EXT 6b's -66% CRB reclaim preserved (behavior-neutral).
- **Standing rule 11 5-axis (Addendum VIII)**: rule still gates per-entry spawn; the table is the apparatus, not a substitute for the discipline.
- **Standing rule 9 raw-pointer audit**: IcEntry holds *const u8 (fn-item static); SAFE per rule 9. `unsafe impl Sync` documented.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (apparatus landed + behavior-neutral migration verified); enables per-entry follow-on rounds at bounded LOC.
Per Doc 735 §X.h.b: substrate-introduction round at apparatus level; (P2.d) bench expected per Finding II.2-bis; reclaim materialization at HI-EXT 3+ per-entry rounds where entries deliver per-synthetic-fixture speedup.

### Open scope at HI-EXT 2 close

1. **HI-EXT 3** — per-entry round adding String.prototype.charAt (similar shape to charCodeAt; ~35 LOC)
2. **HI-EXT 4** — per-entry round adding String.prototype.codePointAt (handles surrogate pairs; ~40 LOC)
3. **HI-EXT 5** — per-entry round adding Array.prototype.length (PropertyGet for Array receiver; ~30 LOC)
4. **HI-EXT 6** — per-entry round adding Array.prototype.push (MethodCall arity 1, mutating; ~45 LOC)
5. **HI-EXT N+1** — composition probe + final disposition + Pred-hi.* booking
6. **Finding HI.1 hardening** — override-safety gate (skip in first cut; codify if/when real-world override surfaces)

### Cumulative status at HI-EXT 2 close

LOC delta: ~280 added (ic_table.rs) + ~150 removed (translator.rs ad-hoc) = ~130 net.
HI-EXT 0-2 cumulative: ~680 across the locale (apparatus + design + 2-entry migration).
Engagement instrument now extensible.

---

*HI-EXT 2 closes. Table apparatus landed; 2-entry migration behavior-neutral; per-entry LOC budget verified (28+38 ≤ 50 each). Pred-hi.1 closed. HI-EXT 3+ adds starter-set entries at bounded LOC.*
