---
helmsman_session: helmsman-2026-05-30-implement-against-findings
proposed_commits:
  - pending
target_branch: main
summary: TABSC-EXT 1 — DataView coercion-faithfulness; replace Rust saturating `as` casts in `data_view_set_number` with spec-modular ToInt8/Uint8/Int16/Uint16/Int32/Uint32 per ECMA-262 §7.1.6/§7.1.8 via the existing `abstract_ops::number_to_raw_bytes` helper (substrate prefix from TABSC-EXT 0).
risk_class: substrate
gates_pre:
  diff_prod: 64 PASS / 48 FAIL (post-TABSC-EXT 0 baseline)
  tamm_cluster: 87 / 100
  tawr_cluster: 71 / 100
gates_post:
  build: cargo build --release --bin cruft -p cruftless — pending
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib — pending (74/0/1 ignored baseline)
  diff_prod: ≥ 64/48 (regression gate)
  tamm_cluster: ≥ 87 (regression gate)
  tawr_cluster: ≥ 71 (regression gate)
  probe_cells:
    - new DataView(new ArrayBuffer(1)).setUint8(0, 300); getUint8(0) === 44 (NOT 255 saturating)
    - new DataView(new ArrayBuffer(1)).setInt8(0, 130); getInt8(0) === -126 (NOT 127 saturating)
    - new DataView(new ArrayBuffer(4)).setUint32(0, 4294967296 + 5); getUint32(0) === 5 (NOT 0xFFFFFFFF)
---

## Substrate Moves

First substrate move under the "implement against findings" mandate (keeper directive Telegram 10602). Direct application of:

- **Rule 27** (Substrate-spec-correctness vs engine-architecture conflict, promoted at cycle 2 commit 1a0bdc29): the previous attempt at DataView coercion-faithfulness was blocked by the same `Vec<Value>` architectural constraint that blocked TAECSF-EXT 1. TABSC-EXT 0 (commit f2107bb6) closed the constraint; the spec-faithful fix is now structurally landable as a Doc 739 cascade-revival pattern of TAECSF-EXT 1.
- **Rule 28** (Dynamic-typing pipeline starts type-specific alphabet at runtime introspection): the rectification site for DataView coercion is at Tier 5/6 (the runtime dispatch + storage tier). This proposal lands at exactly that tier per Rule 28's prediction.
- **Rule 21** (probe-first scoping for substrate cost): the substrate move uses the existing `abstract_ops::number_to_raw_bytes` helper authored in TABSC-EXT 0; no new abstract op needed.
- **Rule 4** (never split a substrate move): all 6 integer DataView setters bundled (setInt8/setUint8/setInt16/setUint16/setInt32/setUint32). Float setters (setFloat32/setFloat64) unchanged — their coercion via Rust `n as f32` / `n as f64` is spec-correct.

### Doc 744 four-tuple (M, T, I, R) + observability

- **M**: `dv.setIntN(offset, value)` / `dv.setUintN(offset, value)` user-visible JS calls per ECMA-262 §25.3 DataView.
- **T**: spec-faithful per §6.1.6.1 NumberToRawBytes + §7.1.6–§7.1.8 ToInt8/Uint8/Int16/Uint16/Int32/Uint32 — modular reduction (mod 2^N), NOT Rust saturating casts.
- **I**: single substrate transition — replace the integer-cast branches in `data_view_set_number` (intrinsics.rs:21968-21979) with calls to `abstract_ops::number_to_raw_bytes(kind, &Value::Number(n))` where `kind` maps the method name to the corresponding TA-kind string ("Int8Array" / "Uint8Array" / etc.).
- **R**: DAG mouth-gating prereq ↑ `abstract_ops::number_to_raw_bytes` (CLOSED at spawn; landed in TABSC-EXT 0). Lattice (downstream) ↓ with TA storage path (already spec-faithful post-TABSC-EXT 0; this rung's coercion matches the TA path's coercion, restoring byte-tier symmetry between DataView writes and TA writes over aliased buffers).
- **Observability**: ordinary. No scaffold.

All four-tuple elements + mouth-gating prereq + observability explicit at spawn. Per Doc 744 §VI rounds-to-closure formula: closure prediction ≤1 round.

### Substrate scope

~15 LOC in `pilots/rusty-js-runtime/derived/src/intrinsics.rs` at `data_view_set_number` lines 21968-21979. Map method_name + signed + byte_count to TA kind:
- `("setInt8", _, 1)` → "Int8Array"
- `("setUint8", _, 1)` → "Uint8Array"
- `("setInt16", _, 2)` → "Int16Array"
- `("setUint16", _, 2)` → "Uint16Array"
- `("setInt32", _, 4)` → "Int32Array"
- `("setUint32", _, 4)` → "Uint32Array"

Each branch's `bytes[..N].copy_from_slice(&(n as <T>).to_le_bytes())` becomes a single call to `abstract_ops::number_to_raw_bytes(kind, &Value::Number(n))` returning the canonical 8-byte LE representation; `bytes[..byte_count].copy_from_slice(&encoded[..byte_count])`.

Float branches unchanged.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — must PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — must PASS (74/0/1 ignored).
3. Refresh `bin/cruft` from `target/release/cruft` (binary-staleness mitigation).
4. Direct probe assertions:
   - `new DataView(new ArrayBuffer(1)); dv.setUint8(0, 300); dv.getUint8(0)` → `44`
   - `new DataView(new ArrayBuffer(1)); dv.setInt8(0, 130); dv.getInt8(0)` → `-126`
   - `new DataView(new ArrayBuffer(4)); dv.setUint32(0, 4294967301); dv.getUint32(0)` → `5`
   - Float pass-through: `dv.setFloat64(0, 3.14); dv.getFloat64(0) === 3.14` → `true`
5. Regression gates: TAMM ≥87, TAWR ≥71, diff-prod ≥64/48.

## Risk Assessment

Per Rule 27, this rung is a Doc 739 cascade-revival pattern of the original DataView-coercion-faithfulness gap. The architectural prerequisite (byte storage) is closed. The rung is narrow (15 LOC), uses an existing helper (substrate prefix from TABSC-EXT 0), and touches a single function. The cluster-regression risk surfaced at TAECSF-EXT 1 NEGATIVE does not apply here because the storage architecture no longer relies on the `Vec<Value>` view-aliasing-pass-through invariant.

**One specific risk**: `data_view_get_number` (lines 21902-21933) reads bytes back as Numbers via Rust `as f64` casts. Those casts are spec-correct for the read direction (the byte representation already encodes the value's modular result; reading + cast-to-f64 yields the spec-faithful Number). No change needed at the getter.

## Composes-With

- `apparatus/docs/predictive-ruleset.md` Rules 4, 11, 13, 21, 27, 28.
- `pilots/rusty-js-runtime/derived/src/abstract_ops.rs::number_to_raw_bytes` (TABSC-EXT 0 substrate prefix).
- `pilots/typed-array-byte-storage-conformance/` — this rung is TABSC-EXT 1.
- `apparatus/arcs/2026-05-28-array-exotic-substrate/` — fourth in-flight locale (this rung within).
- `apparatus/docs/findings-ledger.md` Entries 012 (Rule 27) + 013 (Rule 28) — this rung is a direct application of both.

## Authorization

Awaiting keeper APPROVED. Per the session's established discipline, this is the first substrate move under the "implementing against findings" mandate. Subsequent rungs may continue inside TABSC (e.g., `__kind` typed-enum optimization) or pivot to other findings-driven substrate (Doc 721 Step 5 iteration on the 17 still-gated cells from the original chain bundle).
