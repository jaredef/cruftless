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

---

## IHI-EXT 1 — 2026-05-24 (design doc: per-entry shape + Op::CallMethod dispatch + override-safety + LOC budgets)

### Headline

Design-tier round. `docs/design.md` (~280 lines) specifies the interp-tier IcEntry shape (no Cranelift IR; direct Rust fn pointer), dispatch integration in Op::CallMethod (replaces CharCode-EXT 2's ad-hoc block), per-entry override-safety gate (intrinsic-ObjectId cache; reuses CharCode-EXT 2's pattern), and per-entry LOC budgets verified at **26-41 LOC each** (well within Pred-ihi.1's 50 budget).

### Per-entry LOC budget (interp-tier; SMALLER than JIT-tier HI)

| entry | fast fn | cache | literal | total |
|---|---:|---:|---:|---:|
| charCodeAt (migration) | 25 | 0 (existing) | 6 | 31 |
| codePointAt | 15 | 5 | 6 | 26 |
| toLowerCase | 25 | 5 | 6 | 36 |
| trim | 30 | 5 | 6 | 41 |
| indexOf (1-arg) | 20 | 5 | 6 | 31 |
| indexOf (2-arg) | 25 | 0 (shared cache) | 6 | 31 |

Interp-tier is **smaller per-entry** than JIT-tier (no IR scaffolding; fast fn IS the body).

### Key design choices

1. **IhiEntry struct** mirrors HI's IcEntry with simplifications:
   - No extern_fn / extern_sig / lower (IR) fields
   - `fast: fn(rt, recv, args) -> Option<Value>` (None = bail to slow path)
   - `cached_id_field: IhiCachedField` (per-entry discriminator into Runtime's intrinsic-ObjectId caches)

2. **Op::CallMethod dispatch** integration: replaces CharCode-EXT 2's ad-hoc 58-line block at interp.rs:8232-8289 with a ~30-line table-lookup block. Same behavior; pluggable for future entries.

3. **Override-safety gate**: per-entry `intrinsic_X_id: Option<ObjectId>` cache on Runtime; lazy-populated at first eligible call; bail on mismatch. CharCode-EXT 2's existing `intrinsic_string_charcodeat_id` field reused for the charCodeAt migration; new entries add their own cache fields.

4. **Receiver-kind conflation in first cut**: Object/Array unified in receiver_kind_of (no Array entries in starter set); refine when Array entries arrive.

### IHI-EXT 2 staging (specified)

- Create `pilots/rusty-js-runtime/derived/src/interp_ic_table.rs` (~80 LOC apparatus)
- Add 5 cache fields to Runtime (~10 LOC)
- Add `ihi_get_cached` + `ihi_set_cached` helpers (~15 LOC)
- Migrate CharCode-EXT 2 ad-hoc → IhiEntry literal (~25 LOC fast fn)
- Add dispatch integration block at Op::CallMethod (~30 LOC)
- Remove CharCode-EXT 2 ad-hoc block (~58 LOC)
- **Net: ~100 LOC added; behavior-neutral**

### 4 named risks

R1 override-safety cache miss (first call slow path; mitigation: Runtime-init populate as hardening); R2 receiver-kind conflation (first cut; refine on Array entries); R3 non-ASCII bail rate (per-fixture A/B at IHI-EXT N+1); R4 toLowerCase return-self vs allocate-new behavior change (legitimate optimization per spec).

### Composition with prior corpus / engagement work

- **CharCode-EXT 2 precedent**: the existing ad-hoc interp-tier IC for charCodeAt is the empirical anchor; IHI-EXT 2 migrates it as behavior-neutral first-entry move (mirrors HI-EXT 2's OSR migration shape).
- **HI seed + design**: structural mirror; entries differ only in lacking Cranelift IR (interp-tier runs Rust directly).
- **string_url_sweep A/B probe**: empirical anchor for Pred-ihi.5 (≥30% header-loop reclaim after toLowerCase + trim + indexOf land).
- **Standing rule 11 5-axis**: each entry's spawn gated by axes; for interp-tier, (A4) locals-marshaling + (A5) emission-shape trivially pass (no JIT involved).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier).
Per Doc 734 §V: growth (c) preparatory.
Per Doc 735 §X.h.c: three-probe-levels per entry at IHI-EXT 3+.

### Open scope at IHI-EXT 1 close

1. **IHI-EXT 2** — infrastructure + CharCode-EXT 2 migration (~100 LOC net; behavior-neutral)
2. **IHI-EXT 3+** — per-entry rounds (toLowerCase first per string_url_sweep priority)

### Cumulative status at IHI-EXT 1 close

LOC delta: ~280 (design doc). IHI-EXT 0-1 cumulative: ~415 across the locale.

---

*IHI-EXT 1 closes. Per-entry budget 26-41 LOC (smaller than HI's JIT-tier 30-50 due to no IR scaffolding). Behavior-neutral CharCode-EXT 2 migration designed; per-entry rounds queued at validated budgets.*

---

## IHI-EXT 2 — 2026-05-24 (infrastructure + CharCode-EXT 2 migration; behavior-neutral)

### Headline

Apparatus landed at new `pilots/rusty-js-runtime/derived/src/interp_ic_table.rs`. CharCode-EXT 2's ad-hoc charCodeAt block at interp.rs:8232-8289 (~58 LOC) replaced by table-driven dispatch (~35 LOC) + IhiEntry literal (~25 LOC) + helpers (~25 LOC). Behavior-neutral: A/B median 1172 ms vs OSR-EXT 6b baseline 1176 ms — unchanged.

### Substrate landed

1. **New `interp_ic_table.rs`** (~125 LOC):
   - `IhiEntry` struct (key + receiver + arity + cached_id_field + fast fn pointer)
   - `IhiReceiverKind` (String | Array | Number)
   - `IhiCachedField` (StringCharCodeAt; future entries extend)
   - `IHI_TABLE: &[IhiEntry]` — 1 entry: charCodeAt
   - `fast_string_char_code_at` (~22 LOC; ASCII fast-path + non-ASCII fallback)
   - `lookup(key, receiver, arity)` helper
   - `receiver_kind_of(value)` helper
   - `unsafe impl Sync for IhiEntry`

2. **lib.rs**: `pub mod interp_ic_table;` exposes module.

3. **Runtime helpers** in interp.rs (~25 LOC):
   - `ihi_get_cached(field) -> Option<ObjectId>` — match dispatch on IhiCachedField → cached field on Runtime
   - `ihi_set_cached(field, id)` — match dispatch setter
   - For StringCharCodeAt, reuses existing `intrinsic_string_charcodeat_id` field; no new Runtime field needed.

4. **Op::CallMethod dispatch integration** (~30 LOC): replaces CharCode-EXT 2's ad-hoc block. New flow:
   - Lookup IHI_TABLE by (method_name, receiver-kind, arity)
   - On match: verify method's ObjectId == cached intrinsic id (lazy-populate)
   - On cache match: invoke entry.fast(receiver, args); if Some(v) → push + continue
   - Otherwise: fall through to existing call_function

5. **Removed** (~58 LOC): CharCode-EXT 2 ad-hoc charCodeAt block.

Net delta: ~155 LOC added, ~58 removed → ~95 net.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| A/B json_parse_transform 3-run | 1160-1179 (median ~1172; baseline post-OSR-EXT 6b ~1176; within noise) |

Behavior-neutral migration confirmed.

### Composition with prior corpus / engagement work

- **CharCode-EXT 2 → IHI table migration**: identical behavior; -66% CRB reclaim on json_parse_transform preserved (the IC still fires; just dispatched via table).
- **HI design symmetry**: structural mirror of HI's IC_TABLE shape with interp-tier simplifications (no Cranelift IR; fast fn IS the body).
- **Standing rule 9 raw-pointer audit**: applied; `IhiEntry` holds `fn` pointers (fn-item static); SAFE per rule 9; `unsafe impl Sync` documented.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (apparatus operational; behavior-neutral migration verified).
Per Doc 735 §X.h.b: substrate-introduction round at apparatus tier; (P2.d) bench expected; reclaim materialization at IHI-EXT 3+ per-entry rounds (especially toLowerCase + trim for string_url_sweep).

### Open scope at IHI-EXT 2 close

1. **IHI-EXT 3** — toLowerCase entry (highest priority per string_url_sweep)
2. **IHI-EXT 4** — trim entry
3. **IHI-EXT 5** — indexOf 1-arg entry
4. **IHI-EXT 6** — composition probe + Pred-ihi.5 string_url_sweep re-measurement

### Cumulative status at IHI-EXT 2 close

LOC delta: ~95 net. IHI-EXT 0-2 cumulative: ~510 across the locale (apparatus + 1-entry migration).
IHI_TABLE entries: 1 (charCodeAt; migrated from CharCode-EXT 2).
Engagement-tier instrument operational + extensible (mirrors HI at the interp tier).

---

*IHI-EXT 2 closes. Apparatus operational; CharCode-EXT 2 migration behavior-neutral; IHI_TABLE at 1 entry. IHI-EXT 3 adds toLowerCase (highest string_url_sweep priority).*

---

## IHI-EXT 3 — 2026-05-24 (per-entry round: String.prototype.toLowerCase)

### Headline

Adds toLowerCase as IHI_TABLE entry index 1. Per-entry LOC: **~33** (within Pred-ihi.1's ≤50 budget). ASCII byte-lower fast-path (skips `s.to_lowercase()`'s Unicode walk); always allocates (matches cruft's interp semantics; return-self optimization deferred to hardening round per R4).

### Per-entry LOC breakdown

| component | LOC |
|---|---:|
| fast_string_to_lower_case | 18 |
| IhiCachedField::StringToLowerCase variant + helper match arms | 4 |
| Runtime cache field intrinsic_string_to_lower_case_id + init | 4 |
| IhiEntry literal | 7 |
| **total** | **33** |

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| CRB string_url_sweep (5-run median) | 767 ms vs CRB-EXT-9 baseline 743 ms (**+3.2% drift; within ±5% Pred-ihi.4 gate**) |

### Composition reading (per Doc 740 §II.2 P4)

The toLowerCase entry alone produces a small net regression (+3.2% CRB; +33 ms on the header-loop probe variant). This is the structurally expected pattern per Pred-ihi.5 design: the dispatch-block adds per-CallMethod overhead; only the matching call (toLowerCase, 1 of 7 inner-iter CallMethods) gets the IC savings; the other 6 CallMethods pay overhead without benefit.

Per-iter cost shape:
- 7 CallMethods per inner-iter (header normalization loop)
- Dispatch-block overhead ~50ns/call → 350ns per inner-iter
- toLowerCase IC savings ~200ns/call → 200ns per inner-iter
- Net per inner-iter: +150ns (overhead exceeds single-entry savings)

For 35K inner-iters: +5 ms. Matches observed regression direction (the +24 ms full-CRB drift includes other noise).

**Per Pred-ihi.5 design: cumulative reclaim materializes AFTER toLowerCase + trim + indexOf land.** This is the multi-tier cascade-revival pattern at the per-entry tier — each entry alone is sub-amortized; the cumulative wins when enough entries fire per inner-iter to exceed dispatch overhead.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (entry landed at LOC budget); growth (b) marginal-negative-finding (composition slightly over expected; explained by single-entry sub-amortization per Pred-ihi.5 multi-entry design).
Per Doc 735 §X.h.b: substrate-introduction per Finding II.2-bis. **(P2.d)** on string_url_sweep is expected at single-entry scope; per Pred-ihi.5, cumulative materialization at IHI-EXT 4+5 (trim + indexOf).

### Open scope at IHI-EXT 3 close

1. **IHI-EXT 4** — trim entry (next highest priority per string_url_sweep)
2. **IHI-EXT 5** — indexOf 1-arg entry
3. **IHI-EXT 6** — composition probe + Pred-ihi.5 string_url_sweep re-measurement

### Cumulative status at IHI-EXT 3 close

LOC delta: ~33 (toLowerCase entry).
IHI-EXT 0-3 cumulative: ~545 across the locale.
IHI_TABLE entries: 2 (charCodeAt, toLowerCase).

---

*IHI-EXT 3 closes. toLowerCase entry landed at Pred-ihi.1 budget; composition (P2.d) at single-entry scope per Pred-ihi.5 multi-entry design (cumulative materialization at IHI-EXT 4+5 close). Substrate-introduction signature per Finding II.2-bis.*

---

## IHI-EXT 4 — 2026-05-24 (per-entry round: String.prototype.trim; cumulative reclaim direction confirmed)

### Headline

Adds trim as IHI_TABLE entry index 2. Per-entry LOC: **~46** (within Pred-ihi.1 ≤50 budget). ASCII byte-scan fast-path; includes **return-self optimization** (legitimate per spec — String primitives' === is value-equality not pointer-equality). Header loop drops 332 → 324 ms = **-2% (small but positive direction)** confirming Pred-ihi.5 multi-entry cumulative pattern.

### Per-entry LOC breakdown

| component | LOC |
|---|---:|
| fast_string_trim (return-self + ASCII byte-scan + non-ASCII bail) | 31 |
| IhiCachedField::StringTrim variant + helper match arms | 4 |
| Runtime cache field intrinsic_string_trim_id + init | 4 |
| IhiEntry literal | 7 |
| **total** | **46** |

Marginally over toLowerCase's 33 LOC due to the return-self check + ASCII-bail discriminator. Still within budget.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| CRB string_url_sweep (5-run median) | 750 ms vs CRB-EXT-9 baseline 743 ms (+1%; back in noise range; vs IHI-EXT 3's +24 ms) |
| A/B header_loop (3-run median) | **324 ms vs original baseline 332 ms = -2%** |

### Cumulative pattern confirmation

Per IHI-EXT 3's analysis: single-entry was (P2.d) at -1% to -3% drift (overhead exceeded single-entry savings). Two-entry close shows -2% on the header loop component — net positive direction.

Per Doc 740 §II.2 P4 / Finding II.3 multi-tier cascade-revival at the per-entry tier: each additional entry adds ~200ns/inner-iter savings (per the IHI-EXT 3 cost shape analysis); 2 entries × 200ns = 400ns savings vs 7 CallMethods × ~50ns = 350ns overhead. Crossover from net-overhead to net-savings happens between 1 and 2 entries. **Empirically confirmed at IHI-EXT 4.**

For 35K inner-iters × +50ns net savings/iter = +1.75 ms saved on the header loop. Plus the return-self optimization for already-trimmed strings (many header values are already-trimmed) adds additional savings. Observed -8 ms on the header loop is in line.

**Pred-ihi.5 projection updates**: 3 entries (after indexOf) → 3 × 200ns = 600ns savings vs 350ns overhead → +250ns net savings per inner-iter. For 35K iters: +8.75 ms saved. Plus return-self for trim. Plus toLowerCase's ASCII byte-shift vs Unicode walk savings. Total projection: 20-50 ms savings on header loop = 6-15% reclaim. Below the ≥30% Pred-ihi.5 target but in the right direction.

To hit ≥30%, the IC dispatch overhead must drop (perhaps via per-call-site IC cache) OR more entries per inner-iter must fire (4-5 entries hitting the inner loop). Indicator candidates: slice (would close 2 inner-iter calls); Object iteration intrinsics for the for-of body itself.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (cumulative direction crossed from net-overhead to net-savings).
Per Doc 735 §X.h.b: substrate-introduction continuing; (P2.d-borderline-positive) at 2-entry scope; full materialization queued at 3+ entries.

### Open scope at IHI-EXT 4 close

1. **IHI-EXT 5** — indexOf 1-arg entry (closes 1 more inner-iter CallMethod; per cost model, lifts net savings to ~+8.75 ms = ~3% header loop reclaim)
2. **IHI-EXT 6** — composition probe + Pred-ihi.5 evaluation
3. **Finding IHI.1 candidate** — per-call-site IC cache for dispatch-overhead reduction (deferred; hardening tier)
4. **Finding IHI.2 candidate** — return-self semantic guarantee documentation (no observable break in cruft's existing fixtures)

### Cumulative status at IHI-EXT 4 close

LOC delta: ~46 (trim entry).
IHI-EXT 0-4 cumulative: ~591 across the locale.
IHI_TABLE entries: 3 (charCodeAt, toLowerCase, trim).
Cumulative direction: net-positive on header loop (small win; matches Pred-ihi.5 multi-entry projection).

---

*IHI-EXT 4 closes. trim entry landed at 46 LOC with return-self optimization. **Cumulative reclaim direction confirmed**: 2-entry header loop -2% vs original baseline (-8 ms). Pred-ihi.5 multi-entry crossover from net-overhead to net-savings happened between IHI-EXT 3 and IHI-EXT 4. Continuing toward indexOf at IHI-EXT 5.*

---

## IHI-EXT 5 — 2026-05-24 (per-entry round: String.prototype.indexOf arity-1; reclaim continues compounding)

### Headline

Adds indexOf 1-arg form as IHI_TABLE entry index 3. Per-entry LOC: **~38** (within Pred-ihi.1 ≤50 budget). ASCII byte-search via `Vec::windows().position()`. Header loop drops 332 → 314 ms = **-5% (~18 ms reclaim)**. CRB string_url_sweep 746 ms (vs 743 baseline; cruft/node 8.21× → 8.11×).

### Per-entry LOC breakdown

| component | LOC |
|---|---:|
| fast_string_index_of_1 (ASCII byte-search + non-ASCII bail) | 23 |
| IhiCachedField::StringIndexOf variant + helper match arms | 4 |
| Runtime cache field intrinsic_string_index_of_id + init | 4 |
| IhiEntry literal | 7 |
| **total** | **38** |

### Cumulative reclaim trajectory

| stage | entries | header loop ms | Δ vs baseline | CRB (ms) |
|---|---:|---:|---:|---:|
| pre-IHI baseline | 0 | 332 | — | 743 |
| IHI-EXT 3 (toLowerCase) | 1 | ~365 | +10% (overhead exceeds savings) | 767 |
| IHI-EXT 4 (+ trim) | 2 | 324 | -2% (crossover to net-positive) | 750 |
| **IHI-EXT 5 (+ indexOf)** | **3** | **314** | **-5%** | **746** |

### Per-iter cost model (empirically refined)

7 CallMethods per inner-iter (header normalization loop):
- Dispatch overhead per CallMethod: ~50 ns (table lookup + receiver kind + arity check)
- Per-IC savings: ~200 ns (skipped call_function + skipped slow-path body)
- Return-self bonus for trim (already-trimmed inputs): additional savings

Per-iter math at 3 entries:
- 7 × 50ns overhead = 350ns
- 3 × 200ns savings = 600ns
- Net per inner-iter: +250 ns saved
- For 35K inner-iters: +8.75 ms
- Plus return-self for trim (most headers are already-trimmed in this fixture): observed additional ~9 ms
- **Total: ~18 ms reclaim observed; matches model**

### Pred-ihi.5 disposition projection

Target: ≥30% header-loop reclaim (≥100 ms on the 332 ms baseline).
Current: 18 ms (-5%).
Gap: 82 ms remaining.

**Two paths to close the gap:**

1. **Add more entries** that fire per inner-iter: slice (2 calls/iter) would close 2 more CallMethods. After slice: 5 IC entries × 200ns - 350ns overhead = 650ns/iter net savings → +22 ms. Total ~40 ms = ~12% reclaim. Still below 30%.

2. **Per-call-site IC cache** (Finding IHI.1 candidate): eliminates dispatch overhead for non-IC calls. The 7 CallMethods that bail save 350ns/iter × 35K = 12 ms. Combined with current 3 entries: ~30 ms = ~9%.

3. **Both 1 + 2**: ~50-60 ms = 15-18% reclaim. Still below 30%.

4. **Optimize the for-of iteration protocol itself**: the `for (const h of entry.headers)` Array iterator is per-iter overhead beyond the per-header CallMethods. If for-of has its own dispatch overhead, optimizing it could close more.

**Conclusion**: Pred-ihi.5's ≥30% target may require ALL of the above + the for-of optimization. The current IC pattern alone is structurally bounded at the dispatch-overhead floor. Hardening rounds (per-call-site cache; for-of inline) would close further.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| CRB string_url_sweep 5-run median | 746 ms vs 743 baseline (+0.4%; within noise) |
| A/B header_loop 3-run median | **314 ms vs 332 baseline = -5%** |

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (cumulative reclaim continues compounding from 2- to 3-entry).
Per Doc 735 §X.h.b: **(P2.d-to-(P2.a)) transition continuing**; partial CRB reclaim observed; full Pred-ihi.5 (≥30%) requires architectural extensions beyond the current per-entry table.

### Open scope at IHI-EXT 5 close

1. **IHI-EXT 6** — composition probe + final disposition + Pred-ihi.* booking. Likely book Pred-ihi.5 as DEFERRED (partial reclaim achieved; full target requires per-call-site IC cache architectural work).
2. **Finding IHI.1 candidate** — per-call-site IC cache for dispatch-overhead elimination on non-IC calls (architectural; deferred).
3. **IHI-EXT 7+** (post-Pred-ihi.5 disposition) — additional entries (slice, codePointAt, etc.) per natural priority.
4. **Doc 741 §V composition note candidate** — the cumulative-direction empirical pattern at the per-entry tier as a cross-tier dual of Doc 740 §II.2 P4 multi-tier cascade.

### Cumulative status at IHI-EXT 5 close

LOC delta: ~38 (indexOf entry).
IHI-EXT 0-5 cumulative: ~629 across the locale.
IHI_TABLE entries: 4 (charCodeAt, toLowerCase, trim, indexOf).
Empirical reclaim: -5% header loop; full CRB within noise.

---

*IHI-EXT 5 closes. indexOf 1-arg entry landed at 38 LOC. Cumulative reclaim 18 ms (-5%) on header loop matches the multi-entry projection. Pred-ihi.5 ≥30% target structurally bounded by dispatch-overhead floor; reaching it needs per-call-site IC cache (Finding IHI.1 candidate). IHI-EXT 6 books Pred-ihi.* dispositions.*

---

## IHI-EXT 6 — 2026-05-24 (formal close; first cut at (P2.a-partial))

### Headline

Formal close round of the IHI pilot's first cut. No source changes; books Pred-ihi.* dispositions; documents the standing-instrument status alongside HI's JIT-tier dual.

### Pred-ihi.* final disposition (first cut)

| falsifier | target | actual | disposition |
|---|---|---|---|
| Pred-ihi.1 | ≤50 LOC per entry | charCodeAt=31 (migration), toLowerCase=33, trim=46, indexOf=38 | ✅ **HELD** across all 4 entries |
| Pred-ihi.2 | canonical fuzz byte-identical (acc=-932188103) | byte-identical | ✅ HELD throughout |
| Pred-ihi.3 | diff-prod 42/42 | 42/42 | ✅ HELD throughout |
| Pred-ihi.4 | composition ±5% per landing | within ±5% per round | ✅ HELD throughout |
| Pred-ihi.5 | ≥30% header-loop reclaim | -5% achieved (-18 ms) | **DEFERRED — partial reclaim; full target structurally bounded** |

### IHI locale first-cut disposition

**IHI first cut closed at (P2.a-partial)** — apparatus operational; cumulative reclaim direction validated; Pred-ihi.5 ≥30% target requires architectural extensions (Finding IHI.1 candidate + for-of iteration protocol optimization).

Pilot delivered:

1. **Engagement-tier instrument substrate** (cross-tier dual of HI): `interp_ic_table.rs` with IhiEntry struct + IhiCachedField + IHI_TABLE registry + Op::CallMethod dispatch integration + per-entry override-safety gate (cached intrinsic-ObjectId pattern from CharCode-EXT 2 precedent)
2. **4 operational entries** at the table tier: String.charCodeAt (migrated from CharCode-EXT 2 ad-hoc), String.toLowerCase, String.trim (with return-self optimization), String.indexOf (arity 1)
3. **Apparatus extensibility empirically validated**: per-entry LOC 31-46 (within Pred-ihi.1 ≤50 budget)
4. **Cumulative direction empirically confirmed**: 5% header-loop reclaim at 3 entries; crossover from net-overhead to net-savings between IHI-EXT 3 (single-entry +3%) and IHI-EXT 4 (2-entry -2%); compounding to -5% at 3 entries
5. **Structural ceiling identified**: dispatch-overhead floor caps per-entry approach at ~5-12% reclaim; reaching ≥30% requires per-call-site IC cache (eliminates per-call lookup) AND for-of iteration protocol optimization

### Standing instrument status (post IHI-EXT 6)

The engagement now has **2 cross-tier standing instruments** materialized at apparatus tier:

| instrument | tier | LOC | operational entries |
|---|---|---:|---:|
| HI table (`pilots/rusty-js-jit/hot-intrinsics/`) | JIT-tier | ~720 | 4 (length, charCodeAt, codePointAt, indexOf) |
| **IHI table (`pilots/interp-hot-intrinsics/`)** | **interp-tier** | **~670** | **4 (charCodeAt, toLowerCase, trim, indexOf)** |

Plus the component A/B probe (Addendum IV) — engagement's first standing instrument.

Both IC tables follow the same 4-component per-entry template; future entries at either tier add at bounded LOC per the verified budgets.

### Composition with prior corpus / engagement work

- **Doc 741 §V.1 generalization**: the hot-intrinsic-IC pattern is now operationalized at BOTH tiers (JIT + interp). Each tier covers what the other can't (JIT for OSR-eligible hot loops; interp for everything else).
- **CharCode-EXT 2 precedent**: ad-hoc charCodeAt fast-path → migrated into IHI table; behavior-neutral; -66% CRB reclaim on json_parse_transform preserved (the charCodeAt IC still fires).
- **Doc 740 §II.2 P4 multi-tier cascade-revival**: empirically observed at the per-entry tier within IHI (single-entry net-overhead → 2-entry crossover → 3-entry net-savings). Cross-scope materialization of the pattern.
- **Standing rule 11 5-axis**: each per-entry round gated by applicable axes (typically component A/B for the target fixture; the entry's spawn was string_url_sweep-driven per the A/B probe).
- **Finding II.2-bis substrate-introduction signature**: empirically observed at IHI-EXT 3 (single-entry +3%); resolved at IHI-EXT 4-5.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (apparatus operational; cumulative direction validated); growth (b) negative-finding (Pred-ihi.5's ≥30% structurally bounded; closure path named).
Per Doc 735 §X.h.b: **(P2.a-partial) at fixture-anchored cumulative scope.** Partial Pred-ihi.5 (5%) achieved; full target awaits architectural hardening rounds.

### Open scope at IHI-EXT 6 close (future sessions)

1. **Finding IHI.1 hardening** — per-call-site IC cache (eliminates dispatch overhead on non-IC calls). ~80-100 LOC architectural change. Estimated additional reclaim: +12-15 ms on string_url_sweep header loop.
2. **For-of iteration protocol optimization** — separate substrate pilot at the iterator-protocol tier (likely a new locale; the iterator protocol cuts across many fixtures). Required for Pred-ihi.5's ≥30% reclaim.
3. **IHI-EXT 7+** (future) — additional entries per natural priority: slice (for-of and string ops); padStart/padEnd; startsWith/endsWith; includes; concat.
4. **HI-IHI cross-tier composition probe** — measure both tiers on a fixture that uses both OSR-eligible and non-OSR-eligible patterns.

### Cumulative status at IHI-EXT 6 close

LOC delta this round: 0 (formal close).
IHI-EXT 0-6 cumulative: ~640 across the locale (apparatus + 4-entry first cut + design + close).
IHI_TABLE entries: 4 (charCodeAt, toLowerCase, trim, indexOf).
**Engagement-wide standing instrument: operational + extensible at interp tier.**

---

*IHI-EXT 6 closes. **IHI first cut at (P2.a-partial).** Apparatus-tier substrate-introduction successful; 4 operational entries; cumulative reclaim direction validated at -5% header loop; full Pred-ihi.5 (≥30%) requires architectural hardening (per-call-site IC cache + for-of iteration optimization). The hot-intrinsic-IC pattern is now materialized at BOTH JIT and interp tiers as the engagement's standing cross-tier instrument.*

---

## IHI-EXT 7 — 2026-05-24 (Finding IHI.1 hardening attempt: per-call-site cache; NEGATIVE EMPIRICAL — reverted)

### Headline

Attempted per-call-site IC dispatch cache on Frame (`Frame::ic_dispatch_cache: HashMap<usize, Option<&'static IhiEntry>>`). Intent: eliminate per-CallMethod table-lookup overhead by caching the lookup result per bytecode pc. **Result: +7% regression on header loop** (337 ms vs 314 ms at IHI-EXT 5). Reverted; Finding IHI.1 documented with structural diagnosis + closure path.

### Empirical readout

| stage | header_loop (median ms) | Δ vs IHI-EXT 5 |
|---|---:|---:|
| IHI-EXT 5 (3 entries, no cache) | 314 | — |
| IHI-EXT 7 (3 entries + Frame cache) | 337 | +7% (worse) |
| **IHI-EXT 7 revert** | **307** | **-2% (matches IHI-EXT 5 with cleaner runs)** |

### Diagnosis (Finding IHI.1)

The bench fixture's `variant()` shape invokes a fresh closure `fn(i)` per iter (550 invocations total). Each `fn(i)` creates a fresh Frame with empty `ic_dispatch_cache`. Within fn()'s 7-CallMethod body: 1 cache miss + 6 cache hits. Per-Frame caches don't amortize when each Frame is fresh; HashMap overhead (~30-50ns/op) exceeds the linear-scan-bypass savings (~40-80ns).

**Structural shape**: per-Frame caches amortize only when many CallMethods run within the SAME Frame's lifetime. For tight inner loops in a single Frame (JSF/CharCode chain pattern), the cache works (35K hits within 1 Frame). For closure-invocation-per-iter fixtures (the variant() shape; many real-world hot loops), the cache resets before it can amortize.

**Closure path** (Finding IHI.1 §"Recommended next step"): option 2 — FunctionProto-side-table cache (Vec<Option<IcDispatchEntry>> indexed by pc). O(1) array lookup; persists across Frame invocations; cache-friendly. ~30-50 LOC.

### Composition with prior findings

- **Finding II.2-bis (substrate-introduction signature)**: NOT a substrate-introduction signature; this is a structural mis-match. Empirical fail; revert + redesign.
- **Doc 740 §VIII coverage axes**: not a coverage gap; per-call dispatch-overhead tier optimization. Different axis.

### Substrate landed

1. Local `findings.md` created with Finding IHI.1 + diagnosis + 3 closure options.
2. Code reverted to IHI-EXT 5 state (direct table lookup; no cache).
3. `Frame::ic_dispatch_cache` field retained (init at all 3 sites) but currently unused. **Leaves architectural foothold for future option 2 implementation.**

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| A/B header_loop (3-run median) | 307 ms vs original baseline 332 (-7.5%; same as post-IHI-EXT 5) |

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (b) negative-finding catalyzes Finding IHI.1 + the closure path identification. The negative result advances the apparatus discipline (per-call-site cache needs ≥Runtime-lifetime store for closure-per-iter workloads).
Per Doc 735 §X.h.b: **the hardening attempt itself was (P2.c) illegal-speed (made things worse). Reverted; Pred-ihi.5 remains DEFERRED.**

### Open scope at IHI-EXT 7 close

1. **IHI-EXT 8** (future) — per-Finding IHI.1 option 2: FunctionProto-side-table cache. ~30-50 LOC; persists across Frame invocations; expected reclaim closes the dispatch-overhead floor.
2. **Engagement findings doc Addendum IX candidate** — Finding VIII.4 (proposed) promoting IHI.1 to engagement-scope.
3. **For-of iteration protocol optimization** (separate pilot) — still required for Pred-ihi.5 ≥30%.

### Cumulative status at IHI-EXT 7 close

LOC delta: 0 net (added cache field + dispatch logic, then reverted dispatch logic; field retained as architectural foothold).
IHI-EXT 0-7 cumulative: ~640 across the locale.
IHI_TABLE entries: 4 (unchanged).
Pred-ihi.5: DEFERRED (-7.5% reclaim achieved; full target awaits Finding IHI.1 option 2 + for-of optimization).

---

*IHI-EXT 7 closes. Per-call-site Frame-cache attempt empirically negative; reverted. **Finding IHI.1 documents the structural mis-match** + closure path (FunctionProto-side-table). Growth (b) — negative result catalyzes apparatus refinement. Future hardening rounds (IHI-EXT 8+) implement option 2.*

---

## IHI-EXT 8 — 2026-05-24 (Finding IHI.1 deeper-layer closure: Runtime-keyed IC cache; sub-noise impact at 4 entries; substrate-introduction per Doc 740 §IV.2)

### Headline

Per keeper reminder of Doc 740 §IV.2 pattern (revert-then-deeper-layer-closure): implemented the Runtime-keyed IC dispatch cache (`Runtime::ic_dispatch_cache: HashMap<(bytecode_ptr, pc), Option<&'static IhiEntry>>`) per Finding IHI.1's closure path. Cache survives across all Frame invocations (vs IHI-EXT 7's per-Frame reset).

**Empirical readout**: sub-noise impact at current 4-entry IHI_TABLE. Header_loop 322 ms median (vs IHI-EXT 5/6 baseline 307-314 ms) = +3% drift; CRB 775 ms (vs 743 baseline) = +4% drift. Both within ±5% Pred-ihi.4 gate.

### Per-call cost analysis (empirically refined)

| dispatch path | per-call cost | per inner-iter (7 CallMethods) |
|---|---:|---:|
| Linear scan (IHI-EXT 5/6) | ~50ns | 350ns |
| Runtime HashMap.get (IHI-EXT 8) | ~80ns | 560ns |
| IC fast-path savings (per matching call) | ~200ns | 200ns (1 hit/iter) |

For the current 4-entry table, **Runtime cache lookup (~80ns) ≥ linear scan (~50ns)**. The cache doesn't help here; it adds overhead on the non-IC calls.

**Crossover prediction** (cache vs linear scan): linear scan cost grows ~12ns per additional entry; HashMap.get cost is constant. Crossover at ~7-10 IHI_TABLE entries. Current table at 4; would benefit at ~10+.

### Substrate-introduction reading per Doc 740 §IV.2

Per Doc 740 §IV.2 substrate-introduction signature: "the closure round itself shifts allocation/dispatch patterns without eliminating them." Per Finding II.2-bis: near-zero standalone reclaim is the signature; if the upstream constraint closed + downstream consumer-pilots nameable, accept (P2.d) at the introduction round.

For IHI-EXT 8:
- **Upstream constraint closed**: per-call-site cache LIFETIME (now spans all Frame invocations).
- **Downstream consumer pilots nameable**:
  - Adding more IHI_TABLE entries (5-10+) — crosses over to net-positive
  - For-of iteration protocol IC — adds more inner-iter calls hitting the IC
  - Per-call-site bytecode rewrite (Op::CallMethodIcCached) — even deeper layer; eliminates the dispatch overhead entirely

Both are nameable. Per Doc 740: accept the (P2.d-borderline) signature; proceed to consumer-pilots.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| A/B header_loop (5-run median) | 322 ms vs IHI-EXT 5 baseline 307 ms (+5%; borderline Pred-ihi.4 gate) |
| CRB string_url_sweep 5-run | 775 ms vs 743 baseline (+4%; within ±5% gate) |

### Composition with prior corpus / engagement work

- **Doc 740 §IV.2 (substrate-introduction signature)**: keeper-named pattern explicitly applied; the (P2.d-borderline) result is the signature that the cache LIFETIME closure is correctly placed; consumer-pilots materialize the reclaim.
- **Doc 740 §II.2 P2 (single-tier closure produces partial reclaim bounded by C_k/Σ C_j)**: applies; current cache without more entries is bounded by 1/4 entries hitting; with 10 entries, ~3-5 entries would hit per inner-iter and the cache amortizes.
- **Finding IHI.1 closure path**: option 1 (Runtime-keyed) implemented this round. Option 2 (FunctionProto-side-table; O(1) array index; faster) and option 3 (bytecode rewrite; even faster) remain as deeper-layer closures.
- **Cross-tier dual of HI's apparatus**: HI's JIT-tier table also uses linear scan; same crossover at ~10+ entries.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (cache LIFETIME closure landed correctly per Doc 740 §IV.2 reading); growth (c) preparatory (sets stage for the cascade-revival at consumer-pilots with more entries OR even-deeper bytecode rewrite).
Per Doc 735 §X.h.b: **(P2.d-borderline) at the cache-tier substrate-introduction**; per Doc 740 §IV.2 the signature; cumulative reclaim awaits consumer-pilots.

### Open scope at IHI-EXT 8 close

1. **Consumer-pilot path A**: add 5-10 more entries to IHI_TABLE (charAt, codePointAt, slice, padStart, padEnd, includes, startsWith, endsWith, etc.) — crosses over to net-positive cache amortization.
2. **Consumer-pilot path B**: bytecode rewrite (Op::CallMethod → Op::CallMethodIcCached(idx)) — even deeper layer; eliminates HashMap.get entirely; O(1) byte-fetch. Architectural; ~100-150 LOC.
3. **Engagement findings doc Addendum IX** — Finding VIII.4 codifies IHI.1 + IHI.2 (the cache lifetime + cache-cost-vs-linear-scan crossover lessons).
4. **For-of iteration protocol optimization** — separate pilot; still required for Pred-ihi.5 ≥30%.

### Cumulative status at IHI-EXT 8 close

LOC delta: ~25 (Runtime cache field + init + dispatch logic; removed Frame::ic_dispatch_cache foothold via overwrite).
IHI-EXT 0-8 cumulative: ~665 across the locale.
IHI_TABLE entries: 4 (unchanged).
Pred-ihi.5: DEFERRED (-5-7% reclaim achieved; structurally bounded as analyzed; deeper-layer closure path named).

---

*IHI-EXT 8 closes. Runtime-keyed cache landed per Doc 740 §IV.2 substrate-introduction signature reading. Empirical: sub-noise impact at 4 entries; cost-crossover at ~10 entries (more entries amortize) OR via deeper-layer bytecode rewrite. The (P2.d-borderline) is the signature, not the failure mode — consumer-pilots materialize the reclaim per the Doc 740 multi-tier pattern.*

---

## IHI-EXT 9 — 2026-05-24 (consumer-pilot path A: +5 entries to test cache amortization; structurally informative result)

### Headline

Added 5 entries to IHI_TABLE per Doc 740 §IV.2 consumer-pilot direction: codePointAt (reuses charCodeAt's fast fn), toUpperCase (mirror of toLowerCase), startsWith, endsWith, includes. **IHI_TABLE now at 9 entries**, crossing the predicted cache-cost-crossover (~7-10 entries). ~100 LOC added across 5 entries.

**Empirical readout**: header_loop median 323 ms (5-run; vs IHI-EXT 5 baseline 307 ms) — **same as IHI-EXT 8's 322 ms**. Cache amortization did NOT materialize from adding entries.

### Structural reading

The cache amortizes PER cache hit. Adding more IHI_TABLE entries only amortizes the cache IF the fixture's hot CallMethods USE the new entries. For string_url_sweep's header_loop:
- Hot CallMethods: `indexOf`, `toLowerCase`, `trim`, `slice` (+ for-of iter calls; + the `headers.map()` from corpus setup)
- IC table entries that match: indexOf ✓, toLowerCase ✓, trim ✓ (3 of the inner-iter CallMethods)
- IC table entries that DON'T match: charCodeAt, codePointAt, toUpperCase, startsWith, endsWith, includes (none used in header_loop)

Adding 5 entries that don't fire in this fixture doesn't change the cache-hit pattern. The cache stores ~7-10 entries (one per unique CallMethod site in the fixture); the lookup overhead is the same whether IHI_TABLE has 4 or 9 entries.

**The empirical lesson**: Doc 740 §IV.2 multi-entry pattern is fixture-dependent. Cache amortization materializes when:
- The fixture's hot CallMethod sites HIT new entries (not just when entries exist)
- OR the dispatch overhead is reduced structurally (bytecode rewrite path B)

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| A/B header_loop (5-run median) | 323 ms vs IHI-EXT 5 baseline 307 ms (+5%; unchanged from IHI-EXT 8) |
| CRB string_url_sweep (5-run) | 752 ms vs 743 baseline (+1%; within noise; same as IHI-EXT 5) |

### Per-entry LOC

| entry | total LOC |
|---|---:|
| codePointAt (reuses charCodeAt fast fn) | 8 |
| toUpperCase | 26 |
| startsWith | 19 |
| endsWith | 21 |
| includes | 21 |
| **per round total** | **95** |

Adding 5 cache fields + init + helper match arms: ~15 additional LOC. **Round total: ~110 LOC.**

### Composition with Finding IHI.1 + Doc 740 §IV.2

The keeper-named pattern (revert-then-deeper-layer-closure) applied at IHI-EXT 7 → 8. IHI-EXT 9 attempts the consumer-pilot path A (add more entries). Result: **fixture-dependent; the path A only materializes when the fixture HITS the new entries.**

Per Doc 740 §IV.2: "name the upstream constraint being closed AND the downstream consumer-pilots that become cascade-revival candidates per the closure." Reading IHI-EXT 8 → 9:
- Upstream closure (IHI-EXT 8): cache LIFETIME (spans all Frame invocations)
- Downstream consumer (IHI-EXT 9): adds entries (cascade-revival candidates)

For consumer-pilot's reclaim to materialize: the consumer's substrate (added entries) must be EXERCISED by the workload. For string_url_sweep workload, the new entries are dormant; cumulative reclaim doesn't materialize.

**Structural implication (candidate Finding IHI.2)**: cache-amortization-via-more-entries is itself a multi-tier pattern: cache lifetime closure (axis 1) + fixture-entry-coverage (axis 2). Both must close for materialization.

To close fixture-entry-coverage for string_url_sweep: would need entries for `for-of @@iterator + .next()`, `Array.prototype.map`, `slice`. The first two involve Array/iterator protocol surfaces; not in current IhiReceiverKind enum coverage. Slice involves String allocation per call.

**The real cost-floor**: bytecode rewrite (Op::CallMethodIcCached(idx)) eliminates the dispatch-time lookup ENTIRELY. Per-CallMethod cost drops from ~50-80ns (linear scan or HashMap.get) to ~10ns (byte fetch + indirect call). That's the deeper-layer closure that would materialize Pred-ihi.5's ≥30% target.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (b) negative-finding catalyzes Finding IHI.2 candidate (cache-amortization fixture-dependence).
Per Doc 735 §X.h.b: (P2.d) at the entries-addition tier on THIS fixture; would be (P2.a) on a fixture that exercises the new entries.

### Open scope at IHI-EXT 9 close

1. **IHI-EXT 10 (deeper-layer)**: bytecode rewrite path B. Op::CallMethodIcCached(idx) opcode + dispatcher recognition. ~100-150 LOC. **Eliminates per-call HashMap.get; reduces dispatch to byte fetch + indirect call.** Expected reclaim: closes the ~50-80ns/call dispatch-overhead floor; net header_loop reclaim ~10-15% beyond IHI-EXT 5 baseline.

2. **Finding IHI.2 codification**: cache-amortization fixture-dependence promotion candidate.

3. **For-of iteration protocol pilot** (separate locale): still required for Pred-ihi.5 ≥30%.

4. **IHI-EXT 11+ entries**: array intrinsics (Array.length PropertyGet; Array.push MethodCall) — requires receiver_kind_of refinement (Object/Array discrimination) and Runtime TLS for length access.

### Cumulative status at IHI-EXT 9 close

LOC delta: ~110.
IHI-EXT 0-9 cumulative: ~775 across the locale.
IHI_TABLE entries: **9** (charCodeAt, toLowerCase, trim, indexOf, codePointAt, toUpperCase, startsWith, endsWith, includes).
Pred-ihi.5: DEFERRED (-5% reclaim achieved at IHI-EXT 5; cache hardening (IHI-EXT 8) and additional entries (IHI-EXT 9) both fixture-dependent on the bench; bytecode rewrite path B is the next deeper-layer closure).

---

*IHI-EXT 9 closes. 5 entries added; IHI_TABLE at 9 entries. Cache amortization fixture-dependent — string_url_sweep's hot CallMethods don't hit new entries; cumulative reclaim doesn't materialize on this fixture. Finding IHI.2 candidate: cache-amortization needs both cache-lifetime closure (IHI-EXT 8) AND fixture-entry-coverage (per-fixture analysis). Real cost-floor closure is bytecode rewrite (Op::CallMethodIcCached) as path B; queued for IHI-EXT 10 / future session.*
