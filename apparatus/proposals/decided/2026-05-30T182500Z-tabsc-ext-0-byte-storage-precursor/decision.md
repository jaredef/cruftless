---
proposal_slug: 2026-05-30T182500Z-tabsc-ext-0-byte-storage-precursor
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10596
decided_at: 2026-05-30T18:30:00Z
covers_commits:
  - f2107bb6
---

## Findings

Approved per keeper Telegram 10596 ("Approved") in the helmsman-keeper dyad (no arbiter appointed; keeper-substituted approval per triumvirate operational §II.4 pre-arbiter-instantiation carve-out). The proposal was authored after the four-corpus-doc reading directed by keeper Telegram 10584 (Doc 721 cross-pipeline diagnostic + Doc 739 single-tier cascade-revival + Doc 741 multi-tier pipeline-connection + Doc 744 pipeline-form discovery).

Substrate commit `f2107bb6` lands the precursor architectural rung at Tier 6 (storage) identified by the pipeline-alphabet-audit (commit 6783bba6) + Finding TAECSF.3 (commit a20e3966). Migration: `ArrayBufferRecord.data: Vec<Value>` → `Vec<u8>` per ECMA-262 §6.1.6.1; per-kind NumberToRawBytes encoding at write, RawBytesToNumeric decoding at read; view-aliasing pass-through preserved at the byte tier.

Substrate scope ~210 LOC across `interp.rs` (~80) + `abstract_ops.rs` (~90) + `intrinsics.rs` (~40). The proposal's risk-5/6/7 substrate-start grep verification ran clean: only two direct `buf.data[N] = …` write sites (TA storage + DataView setter), four `TypedArrayViewRecord` insert sites, three `ArrayBufferRecord` insert sites; no `cruftless/src/` direct buffer access; Buffer-shim uses JS-level dispatch and cascades automatically; Atomics not exercising direct buffer access at this engagement's maturity.

Founds `pilots/typed-array-byte-storage-conformance/` (TABSC-EXT 0) under arc `2026-05-28-array-exotic-substrate`. Manifest 229 → 230 locales (+1 top-level).

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS (1m 05s).
2. `cargo test --release -p rusty-js-runtime --lib` — PASS: 74 passed; 0 failed; 1 ignored.
3. **Binary-staleness mitigation**: `cp target/release/cruft /home/jaredef/bin/cruft` confirmed via md5sum equality before measurement gates (the discipline carry-forward from TAECSF-EXT 1 measurement-discipline incident).
4. **View-aliasing probe** (the three TAECSF-EXT 1 NEGATIVE cells): 3/3 PASS.
   - `TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js`: FAIL → PASS.
   - `TypedArrayConstructors/internals/Set/conversion-operation-consistent-nan.js`: FAIL → PASS.
   - `TypedArray/prototype/some/BigInt/values-are-not-cached.js`: FAIL → PASS.
5. **Cascade probe** (`/tmp/probe-taecsf-1.js`, 10-cell BigInt + integer + Uint8Clamp suite): 10/10 PASS, including the proposal's named gate cells (Uint8 wrap 300→44, Int8 wrap 130→-126, Uint8Clamped NaN→0, saturate 300→255, round-half-to-even 254.5→254). Doc 739 (B3) empirically validated.
6. Regression gate — TAMM cluster: 86 → 87 (+1 PASS; ≥86 satisfied).
7. Regression gate — TAWR cluster: 67 → 71 (+4 PASS; ≥67 satisfied).
8. Regression gate — diff-prod: 64/48 → 64/48 (stable; ≥64/48 satisfied).

## Empirical surplus vs prediction

Doc 740 §II.2 P4 and Doc 741 predicted "substrate-introduction signature (≈0% cluster movement)" at the precursor rung. Actual: +5 cluster cells at this rung alone (+4 TAWR, +1 TAMM) plus the 3 TAECSF-EXT 1 NEGATIVE cells flipping FAIL → PASS plus the 10/10 cascade probe.

Surfaced as candidate Finding TABSC.1 amending Doc 740 §II.2 P4: "cumulative reclaim may materialize at the precursor rung itself when the precursor's substrate is structurally complete for the downstream tier's requirements." In this engagement instance, byte storage IS both the upstream constraint-closure AND the byte-encoding tier per §6.1.6.1; the cascade materialized at the precursor rung rather than at a separate subsequent rung. Promotion-readiness: one-more-observation; recorded in the locale's trajectory.md for future apparatus-tier consolidation.

## Cumulative session yield

Today's session (2026-05-30) closed three deferrals (009 PROMOTED, 014 SUPERSEDED, 010 PROMOTED), founded four locales (RBDPA + TAECSF + audit-ledger + findings-ledger + findings-disposition-protocol + TABSC), introduced three apparatus-tier documents (audit-ledger, findings-ledger, findings-disposition-protocol), wired them into canonical enumerations, consolidated rules 17–22 into predictive-ruleset.md, ran the pipeline-alphabet-audit, read four corpus docs (721 + 739 + 741 + 744), authored the precursor architectural proposal, and landed it with empirical cascade-revival validation.

Gates net positive across the session: TAWR 63 → 71 (+8); diff-prod 61/51 → 64/48 (+3 PASS); TAMM 82 → 87 (+5); manifest 227 → 230 locales (+3). The byte-storage migration is the load-bearing closure that unblocks the array-exotic arc's remaining residual cells.

## Named follow-up

- **DataView coercion-faithfulness** (TAECSF-EXT 1 proposal §Risk Assessment named the Rust saturating-cast vs spec-modular-reduction divergence in DataView setters at `intrinsics.rs:19842-19865`): now structurally addressable since DataView setters operate at the byte tier. Candidate follow-up rung within TABSC or sibling locale.
- **`__kind` typed-enum promotion** (proposal §Risk #4 + seed §Carve-outs): co-yield optimization deferred. Promote `TypedArrayKind` enum on `TypedArrayViewRecord` to avoid String-comparison dispatch hot-path.
- **TAECSF sub-substrates (a) integer-kind + (b) Float32 canonical-NaN**: per Doc 739 cascade-revival, these are now correct as-is via the upstream storage-tier closure. The TAECSF locale's progress is no longer blocked by the architectural constraint; future TAECSF rungs may surface additional sub-substrates (e.g., proxy-target TA assignment, captured-slot interactions) but the named (a) + (b) are RESOLVED.

## Findings-ledger updates pending

- Finding TAECSF.3 (engine-architectural Value-cell-aliasing) has now received its **second observation** via this rung's empirical validation. Promotion-readiness flips from "one-more-observation" to "ready" for standing-rule status pending arbiter or keeper sign-off.
- Finding APP.PIPELINE-1 (dynamic-typing pipeline starts type-specific alphabet at runtime) has now received its **second observation** via this rung's instantiation of the rectification-at-Tier-6 prediction. Promotion-readiness flips similarly.
- Finding TABSC.1 (cascade-revival materializes at precursor rung when precursor IS the byte-encoding tier) is new at this rung; trajectory-and-findings-embedded; one-more-observation.

These promotions are deferred to a separate follow-up rung at keeper direction.

**APPROVED for push** per keeper-substituted authorization Telegram 10596.
