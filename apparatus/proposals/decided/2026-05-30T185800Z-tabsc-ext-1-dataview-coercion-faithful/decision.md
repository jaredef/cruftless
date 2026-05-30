---
proposal_slug: 2026-05-30T185800Z-tabsc-ext-1-dataview-coercion-faithful
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10604
decided_at: 2026-05-30T19:05:00Z
covers_commits:
  - 944a22dd
---

## Findings

Approved per keeper Telegram 10604 ("Approved") in the helmsman-keeper dyad (no arbiter appointed; keeper-substituted per triumvirate operational §II.4). First substrate move under the "implement against findings" mandate from Telegram 10602; direct application of Rules 27 + 28 (promoted at cycle 2, commit 1a0bdc29) + Rule 21 (probe-first) + Rule 4 (never split).

Substrate commit `944a22dd` lands ~15 LOC in `pilots/rusty-js-runtime/derived/src/intrinsics.rs` at `data_view_set_number` (lines 21968-21979). The integer branch now dispatches on `(signed, byte_count)` to map to a TA-kind string ("Int8Array" / "Uint8Array" / "Int16Array" / "Uint16Array" / "Int32Array" / "Uint32Array") and calls `abstract_ops::number_to_raw_bytes(kind, &Value::Number(n))` for spec-faithful modular reduction per ECMA-262 §6.1.6.1 + §7.1.6–§7.1.8.

Float branch (`if float`) unchanged — Rust's `f64 → f32` cast is spec-correct for floats.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS (1m 06s).
2. `cargo test --release -p rusty-js-runtime --lib` — PASS: 74/0/1 ignored.
3. `bin/cruft` refreshed from `target/release/cruft` (binary-staleness mitigation).
4. Direct probe assertions (7-cell suite, `/tmp/probe-tabsc-1.js`): 7/7 PASS.
   - `setUint8(0, 300); getUint8(0)` → `44` (modular; NOT 255 saturating).
   - `setInt8(0, 130); getInt8(0)` → `-126` (NOT 127 saturating).
   - `setUint32(0, 4294967301); getUint32(0)` → `5` (NOT 0xFFFFFFFF).
   - `setInt32(0, NaN); getInt32(0)` → `0` (NaN→0 per spec).
   - `setUint16(0, 3.7); getUint16(0)` → `3` (fractional truncate).
   - `setFloat64(0, 3.14); getFloat64(0) === 3.14` → `true` (pass-through preserved).
   - `setFloat32(0, 1.5); getFloat32(0) === 1.5` → `true` (pass-through preserved).
5. Regression gates — TAMM 87, TAWR 71, diff-prod 64/48 — all preserved.

## Cascade-revival pattern realized

This rung is a Doc 739 cascade-revival of the original DataView-coercion-faithfulness gap. The architectural prerequisite (byte storage) was closed at TABSC-EXT 0 (commit f2107bb6); this rung consumes the substrate prefix `number_to_raw_bytes` at ~15 LOC of dispatch glue. Per Doc 739 (B3), the cascade was observable only post-landing; this rung's clean closure validates the cascade-revival framing.

## Substrate-prefix amortization (Finding TABSC.2 candidate)

`number_to_raw_bytes` was authored once at TABSC-EXT 0 and is now consumed by three sites within one locale's scope:
1. TA write path inside TABSC-EXT 0 (`typed_array_set_index`).
2. Cascade-revived TAECSF integer-kind coercion (zero TAECSF-side cost).
3. This rung's DataView setters (~15 LOC dispatch glue).

Per Finding IR.33 cumulative substrate amortization, this is the expected compounding shape. Recorded as Finding TABSC.2 candidate (substrate prefix amortization across cascade-revival-driven rungs); one-more-observation pending cross-locale instance.

## Named follow-up

Cluster gates did not move at this rung — the DataView coercion test262 cells live in paths likely not in the curated TAMM/TAWR/diff-prod pools. Full test262 sample re-run would surface the cluster movement; suggested as a future measurement-discipline rung when the engagement next runs the canonical sample.

Remaining sub-substrates in TABSC scope: `__kind` typed-enum promotion (co-yield optimization; non-precondition); any future cells that surface direct buffer access (Atomics, crypto, compression).

**APPROVED for push** per keeper-substituted authorization Telegram 10604.
