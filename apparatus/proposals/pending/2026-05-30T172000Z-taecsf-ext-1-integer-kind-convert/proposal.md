---
helmsman_session: helmsman-2026-05-30-continue-process
proposed_commits:
  - pending
target_branch: main
summary: TAECSF-EXT 1 — integer-kind `ConvertNumberToTypedArrayElement` per ECMA-262 §10.4.5.16; closes sub-substrate (a) within the TAECSF locale; bundles all 7 integer kinds.
risk_class: substrate
gates_pre:
  diff_prod: 64 PASS / 48 FAIL (post-EXT 0 baseline)
  tamm_cluster: 86 / 100
  tawr_cluster: 67 / 100
gates_post:
  build: cargo build --release --bin cruft -p cruftless — pending
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib — pending
  diff_prod: ≥ 64/48 (regression gate); target ≥ 67/45 (+3 from integer-kind cells)
  tamm_cluster: ≥ 86/100 (regression gate)
  tawr_cluster: ≥ 67/100 (regression gate)
  probe_cells:
    - new Uint8Array(1)[0] = 3.7 → ta[0] === 3 (fractional truncate)
    - new Uint8Array(1)[0] = 300 → ta[0] === 44 (300 mod 256, NOT Rust saturating cast)
    - new Uint8ClampedArray(1)[0] = NaN → ta[0] === 0 (NaN→0 special case)
    - new Uint8ClampedArray(1)[0] = 300 → ta[0] === 255 (saturate)
    - new Int8Array(1)[0] = 130 → ta[0] === -126 (130 mod 256 = 130, ≥128 so subtract 256)
---

## Substrate Moves

Next-rung within `pilots/ta-element-coercion-spec-faithful/` (founded 2026-05-30 by TAECSF-EXT 0). Predicted by Rule 22 (partial-exemplar-closure as substrate-axis discriminator, just promoted to predictive-ruleset.md at commit 6fb6480d): the integer-kind sub-substrate is the next axis sharing the BigInt-TA exemplar from TAECSF-EXT 0's partial closure.

Selects **Option B (all-7-integer-kinds bundle)** over Option A (Uint8Clamp minimum) per Rule 4 (never split a substrate move at the same tier): all integer-kind coercion implements the same `ConvertNumberToTypedArrayElement` abstract op per ECMA-262 §10.4.5.16; the per-kind dispatch lives in a single `match` over `__kind`, making the bundle ~90 LOC total versus ~30 × 7 = ~210 LOC if split into per-kind rungs.

Substrate move (~90 LOC across two files):

1. **`pilots/rusty-js-runtime/derived/src/abstract_ops.rs`** — new `convert_number_to_typed_array_element(value: &Value, kind: &str) -> Result<Value, RuntimeError>` (~60 LOC). Spec-faithful per §10.4.5.16 + §7.1.6–§7.1.8:
   - ToNumber upstream (callable via existing `to_number(rt, value)` or equivalent; this helper is pure-primitive, no Runtime access needed if ToNumber has already been applied by the caller).
   - **Int8 / Uint8 / Int16 / Uint16 / Int32 / Uint32**: NaN/±∞ → 0; else truncate fraction (`trunc()`); explicit modular reduction (`rem_euclid` on the appropriate 2^N); for signed kinds, subtract 2^N if the modular result ≥ 2^(N-1). Critical correctness note: explicit `rem_euclid`-based modulo, NOT Rust's saturating `as` cast (which yields 255 for `300_f64 as u8` and would silently diverge from spec). The DataView setters currently rely on Rust saturating casts (intrinsics.rs:19842–19865) which is a known cross-pillar gap; this helper does not touch DataView storage but establishes the correct coercion pattern for a future DataView migration.
   - **Uint8Clamped**: special-case per §7.1.7 — NaN → 0; ≤0 → 0; ≥255 → 255; else round-half-to-even on the fractional part; clamp to `[0, 255]`.
   - **Float32 / Float64**: out of scope for this rung; helper returns the value unchanged for these kinds (sub-substrate (b) Float32 canonical-NaN remains deferred per the locale's seed.md carve-out).

2. **`pilots/rusty-js-runtime/derived/src/interp.rs`** — update `Runtime::typed_array_set_index_checked` at line ~632 (current EXT 0 site). The existing BigInt branch stays as-is. Add a parallel branch for integer kinds that reads the `__kind` slot, dispatches through the new helper, and propagates `Result`. Float kinds continue to fall through unchanged.

Founds no new locale (TAECSF locale exists). Updates:
- `pilots/ta-element-coercion-spec-faithful/trajectory.md` — TAECSF-EXT 1 entry with M/T/I/R per Doc 744 §V.1; cites Rule 22 axis-split prediction as motivation; records the bundling decision per Rule 4 + Rule 21 (probe-first scoping favored the minimum-viable-defeating substrate at EXT 0; EXT 1 lands the next axis with bundled scope because the dispatch shape is shared).
- `pilots/ta-element-coercion-spec-faithful/findings.md` — author the locale's first `findings.md` extracting TAECSF.1 (narrow dispatcher) from trajectory + adding TAECSF.2 (the integer-kind axis split as a Rule-22 instance + the bundling decision as a Rule-4 instance). This closes the audit-ledger Entry 002 surfaced finding "TAECSF.1 missing findings.md" (the per-locale convention defers extraction until the second productive rung lands; EXT 1 is that rung).

No deferrals-ledger flip required (the locale was promoted at EXT 0 founding). Sub-substrate (b) Float32 canonical-NaN remains a carry-forward within the locale's seed.md.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — must PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — must PASS with no new failures.
3. Direct probe assertions via `cruft /tmp/probe-taecsf-1.js`:
   - `new Uint8Array(1)[0] = 3.7; ta[0]` → `3`
   - `new Uint8Array(1)[0] = 300; ta[0]` → `44` (NOT 255)
   - `new Uint8ClampedArray(1)[0] = NaN; ta[0]` → `0`
   - `new Uint8ClampedArray(1)[0] = 300; ta[0]` → `255`
   - `new Int8Array(1)[0] = 130; ta[0]` → `-126`
4. Regression gate — TAMM cluster (`pilots/typed-array-missing-method/exemplars/run-exemplars.sh`): must remain ≥86/100.
5. Regression gate — TAWR cluster: must remain ≥67/100.
6. Regression gate + yield — diff-prod (`scripts/diff-prod/run-all.sh`): must remain ≥64/48; expect +3 to +4 PASS from the integer-kind fixtures.

## Risk Assessment

- **Blast radius**: contained. The new helper is purely additive in `abstract_ops.rs`; the only call-site is the existing `typed_array_set_index_checked`. `typed_array_set_index` (unchecked, used by `object_set_pk` internal callers) remains unchanged per the EXT 0 carve-out.
- **Rust-cast vs spec-modulo divergence**: the spec mandates explicit modular reduction; Rust's `as` cast from f64 to integers saturates. This proposal explicitly uses `rem_euclid` to match spec. The DataView setters (intrinsics.rs) continue to use saturating casts — they are out of scope for this rung but flagged as a separate gap for a future DataView coercion-faithfulness rung.
- **Performance**: new helper call adds one method dispatch + one `match` per integer-kind TA assignment. Negligible on most workloads; if hot inner loops with tight TA assignments regress, mitigation is inlining the dispatch into `typed_array_set_index_checked` directly (no function-call overhead).
- **Rule 22 prediction-falsification check**: if the residual TypedArrayConstructors / Set fixture pool does not flip by the predicted ~3-4 cells, Rule 22's axis-split prediction is partially falsified for this locale; the rung's Phase 5 chapter-close-inspect surfaces the next axis (likely Float32 canonical-NaN at sub-substrate (b), or a non-coercion sub-substrate not yet enumerated). Per Rule 13, any negative yield triggers revert + deeper-layer diagnosis.
- **Float kinds out of scope**: confirmed in the helper signature. Float32 is NOT touched; sub-substrate (b) carry-forward preserved.

## Composes-With

- `apparatus/docs/predictive-ruleset.md` — Rule 4 (never split a substrate move; motivates bundling), Rule 21 (probe-first scoping; explains why EXT 0 went narrow and EXT 1 goes wider on a shared dispatch), Rule 22 (axis-split prediction; this rung is the prediction's first test). All three rules just consolidated (rules 4 originally, 21 + 22 today at commit 6fb6480d).
- `pilots/rusty-js-runtime/derived/src/abstract_ops.rs` — new helper alongside existing `to_number`, `to_bigint`, `to_boolean`, etc.
- `pilots/rusty-js-runtime/derived/src/interp.rs` — `typed_array_set_index_checked` (the EXT 0 dispatcher) gains the integer-kind branch.
- `pilots/ta-element-coercion-spec-faithful/` — the founded locale (EXT 0); EXT 1 trajectory entry + first `findings.md`.
- `pilots/typed-array-wrong-result/` — sibling locale; regression-gate cluster.
- `pilots/typed-array-missing-method/` — sibling locale; regression-gate cluster.
- `apparatus/arcs/2026-05-28-array-exotic-substrate/` — arc that hosts TAECSF; no roster change (the locale is already enrolled).
- `apparatus/docs/findings-ledger.md` — Entry 005 (TAECSF.1) will gain a second corroboration via the new `findings.md` extraction; promotion-readiness flips from "one-more-observation" to "ready" pending arbiter or keeper sign-off (separate cycle).

## Authorization

Awaiting keeper (or arbiter, when appointed) APPROVED decision per the triumvirate operational protocol §II proposal+veto workflow. Keeper directive Telegram 10576 ("Continue further") authorizes this proposal authorship within the helmsman's substrate-steering scope; landing is gated on the decision.
