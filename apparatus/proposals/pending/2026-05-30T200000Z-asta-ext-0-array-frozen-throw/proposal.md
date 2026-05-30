---
helmsman_session: helmsman-2026-05-30-sample1-substrate-spawn
proposed_commits:
  - pending
target_branch: main
summary: ASTA-EXT 0 — Array strict-throw discipline; founds `pilots/array-strict-throw-discipline/`. Add fallible `object_set_checked` variant; route Array.prototype mutating-methods' length-set sites through it so frozen-target TypeErrors propagate per ECMA §23.1.3 + §10.4.2.1 ArraySetLength step "If newLenDesc.[[Writable]] is false, return false (with Throw)".
risk_class: substrate
gates_pre:
  test262_sample: 88.7% (6816 PASS / 865 FAIL / 16 SKIP / 7681 runnable) per 2026-05-30 canonical
  diff_prod: 64 PASS / 48 FAIL
  tamm_cluster: 87 / 100
  tawr_cluster: 71 / 100
gates_post:
  build: pending
  runtime_lib_tests: pending (74/0/1 ignored baseline)
  diff_prod: ≥ 64/48 (regression gate)
  tamm_cluster: ≥ 87 (regression gate)
  tawr_cluster: ≥ 71 (regression gate)
  probe_cells:
    - Object.freeze([1,2]).pop() throws TypeError
    - Object.freeze([1,2]).push(3) throws TypeError
    - Object.freeze([1,2]).shift() throws TypeError
    - Object.freeze([1,2]).unshift(0) throws TypeError
    - Object.defineProperty([1,2], 'length', {writable:false}); arr.pop() throws TypeError
  predicted_unlock: 12-15 cells (Doc 721 Step 4 U; gated population 15; conservative U after Step 5 cross-pipeline-completeness adjustment)
---

## Substrate Moves

Next coherent substrate move per keeper directive Telegram 10612, driven by findings-ledger Entry 016 (SAMPLE.1) Doc 721 chain-bundle analysis. The cross-family missing-TypeError-throw pattern decomposes into multiple sub-bundles (Rule 17 pre-scoping segmentation); the largest substrate-shaped sub-bundle is **Array-mutating-methods on frozen targets** (15 cells visible in the canonical sample: pop, push, shift, unshift, splice, flat, flatMap; plus probable analogs at other Array methods).

### Doc 721 chain-walk

Step 1 gated population G: 15 cells in `built-ins/Array/prototype/{pop,push,shift,unshift,flat,flatMap}/{set-length-*,target-array-*}.js` sharing the reason "Expected a TypeError to be thrown".

Step 2 chain walk for `Object.freeze([1,2]).pop()`:
1. JS call → bytecode CallMethod → `array_proto_pop_via` (`interp.rs:7994`).
2. `array_proto_pop_via` computes new length; calls `self.object_set(id, "length", Value::Number(...))` (`interp.rs:8003`).
3. `object_set` → `object_set_pk` (`interp.rs:11422`).
4. `object_set_pk` detects Array+length key at line 11524-11546; calls `self.array_set_length_define_property_via(id, desc_id)` with `let _ =` — **error swallowed**.
5. `array_set_length_define_property_via` correctly throws TypeError per spec at line 3711-3715 (`!cur_writable && new_len != old_len`), but the result never reaches the bytecode handler.

Step 3 highest shared layer: the `let _ =` swallowing site at `object_set_pk` line 11543. All 15 cells share this chain; closure here cascades the bundle.

Step 4 predicted unlock: |G|=15. Step 5 cross-pipeline-completeness adjustment: 12-15 cells expected to flip (some cells may have additional secondary TypeError-throw obligations elsewhere in their test bodies; conservative U=12).

### Doc 744 four-tuple + observability

- **M**: `arr.pop() / push() / shift() / unshift() / splice() / ...` on a frozen-or-non-writable-length Array receiver per ECMA §23.1.3.
- **T**: TypeError throw propagates to JS catch site; the Array length is unchanged from pre-call state (per §10.4.2.1 ArraySetLength returns false; Set(..., Throw=true) throws).
- **I**: single substrate transition — introduce `Runtime::object_set_checked(id, key, value) -> Result<(), RuntimeError>` that mirrors `object_set` but propagates the Array-length error. Migrate the 5+ Array.prototype mutating-method intrinsics to call `object_set_checked` with `?` on length writes.
- **R**:
  - DAG mouth-gating prereq ↑ `array_set_length_define_property_via` (CLOSED at spawn; already correctly throws at line 3711-3715).
  - Lattice with TA storage (no interaction; TA uses its own checked dispatcher per TAECSF-EXT 0).
  - Lattice (downstream cascade-revival receivers) ↓ with potential Map/Set/WeakMap/WeakSet frozen-receiver analogs (sibling Doc-721 sub-bundles in the SAMPLE.1 chain; may cascade-revive if they share the `object_set` swallowing site, but likely have different paths — separate substrate moves).
- **Observability**: ordinary.

All four-tuple + mouth-gating + observability explicit at spawn. Per Doc 744 §VI rounds-to-closure: ≤1 round.

### Substrate scope

~40 LOC across two files:

1. `pilots/rusty-js-runtime/derived/src/interp.rs` (~25 LOC):
   - New `pub fn object_set_checked(&mut self, id: ObjectRef, key: String, value: Value) -> Result<(), RuntimeError>` (narrow dispatcher; mirrors `object_set` but routes the Array-length branch through `array_set_length_define_property_via` with `?` propagation; for non-Array-length keys, delegates to `object_set_pk` and returns `Ok(())`).
   - Update `array_proto_pop_via` (line 8003), `array_proto_shift_via` (line 8020), `array_proto_unshift_via` (line 8038ish), and any other length-writing Array intrinsic in `interp.rs` to use `object_set_checked` with `?`.

2. `pilots/rusty-js-runtime/derived/src/prototype.rs` and/or intrinsics.rs if other Array.prototype mutating methods register elsewhere — migrate them similarly.

Founds new locale `pilots/array-strict-throw-discipline/`:
- `seed.md` — telos (Array.prototype mutating methods throw TypeError per spec on frozen / non-writable-length targets); apparatus (the 15-cell gated population G); methodology; carve-outs (Map/Set/WeakMap/WeakSet sibling sub-bundles deferred to separate locales; Promise-receiver-validation sub-bundle deferred; non-Array intrinsic throw discipline deferred).
- `trajectory.md` — ASTA-EXT 0 founding rung with M/T/I/R per Doc 744 + Doc 721 chain-walk citation + Rule 22 axis-discriminator citation.

Arc enrollment: candidate enrollment in `2026-05-28-array-exotic-substrate` (the established array-substrate arc); this locale's telos sits at the lattice-meet of array-exotic + throw-discipline.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — PASS (74/0/1 baseline).
3. Refresh `bin/cruft`.
4. Direct probe (5 cells):
   - `try { Object.freeze([1,2]).pop(); "NO_ERROR" } catch(e) { e.constructor.name }` → `"TypeError"`
   - `try { Object.freeze([1,2]).push(3); "NO_ERROR" } catch(e) { e.constructor.name }` → `"TypeError"`
   - `try { Object.freeze([1,2]).shift(); "NO_ERROR" } catch(e) { e.constructor.name }` → `"TypeError"`
   - `try { Object.freeze([1,2]).unshift(0); "NO_ERROR" } catch(e) { e.constructor.name }` → `"TypeError"`
   - `let a = [1,2]; Object.defineProperty(a, 'length', {writable:false}); try { a.pop(); "NO_ERROR" } catch(e) { e.constructor.name }` → `"TypeError"`
5. Cluster gates: TAMM ≥87, TAWR ≥71, diff-prod ≥64/48 — regression gates only; no movement expected at TAMM/TAWR/diff-prod (cells live in test262-sample pool).
6. **Closure signal per Rule 29**: re-run test262-sample. Predicted +12 to +15 PASS (88.7% → ~88.9–89.0%). Rule 29 n=2 byte-identity declares the new measurement canonical.

## Risk Assessment

- **Blast radius**: contained. The new `object_set_checked` is purely additive; only Array.prototype mutating-method intrinsics migrate to use it. Non-Array internal callers continue to use the existing `object_set` (no behavioral change).
- **Rule 27 + Rule 28 alignment**: the rectification site is Tier 5 (runtime dispatch) at the Array-prototype intrinsic, not upstream. Consistent with Rule 28's "dispatch tier or storage tier, never upstream."
- **Rule 21 (probe-first)**: the substrate move uses the existing `array_set_length_define_property_via` (already spec-correct); the rung's marginal cost is the Result-threading dispatcher + per-intrinsic call-site update. ~40 LOC.
- **No engine-architectural conflict**: unlike TAECSF-EXT 1, this rung does not touch a shared-state invariant. The error propagation is a pure-additive Result-thread; no view-aliasing risk.
- **Cluster-gate prediction explicit**: test262-sample is the canonical measurement; per Rule 29 + DET.2 (Class A), n=2 byte-identity after substrate land declares the new rate.

## Composes-With

- `apparatus/docs/predictive-ruleset.md` Rules 4, 11, 13, 17 (pre-scoping segmentation; ASTA scopes against the sub-bundle, not the full SAMPLE.1 cross-family count), 18 (brand-check at registration wrapper), 20 (cross-module reason-shape coherence), 21 (probe-first; reuse existing helper), 22 (axis-discriminator; ASTA is the Array sub-axis of the SAMPLE.1 multi-axis bundle), 27 (substrate-spec-correctness; previous attempts may have stalled at the swallowing site), 28 (rectification at dispatch tier), 29 (post-land measurement via n=2).
- Corpus Doc 721 (chain-walk applied to SAMPLE.1).
- `apparatus/docs/findings-ledger.md` Entry 016 (SAMPLE.1 — this rung is the Array sub-axis substrate move).
- `apparatus/arcs/2026-05-28-array-exotic-substrate/` — candidate arc enrollment.

## Authorization

Awaiting keeper APPROVED. The substrate move is narrow (~40 LOC), the predicted yield is concrete (+12 to +15 PASS on the canonical sample = ~0.2-0.3 percentage points), and the discipline composition is dense (5+ predictive-ruleset rules + Doc 721 chain-walk + Doc 744 four-tuple + Rule 29 measurement protocol). Subsequent ASTA rungs would address sibling Doc-721 sub-bundles (Map/Set frozen-receiver; Promise dispatcher receiver-validation; Object.assign throw-propagation; for-of iterator-protocol + put-const).
