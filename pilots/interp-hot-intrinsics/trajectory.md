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
