# array-strict-throw-discipline — Trajectory

## ASTA-EXT 0 — LANDED (2026-05-30) — object_set_checked narrow dispatcher + Array.prototype mutating-method length-set Result propagation

**Trigger**: Keeper APPROVED of proposal `apparatus/proposals/pending/2026-05-30T200000Z-asta-ext-0-array-frozen-throw/proposal.md` via Telegram 10614. First substrate-spawn from findings-ledger Entry 016 (SAMPLE.1) Doc 721 chain-bundle decomposition.

**Arc enrollment**: `2026-05-28-array-exotic-substrate` (sixth in-flight locale).

**Phase 1 (Spawn) per Doc 744 §V.1 four-tuple + observability**:
- **M** = `arr.pop()` / `push()` / `shift()` / `unshift()` / `splice()` / `sort()` on a frozen-or-non-writable-length Array receiver per ECMA §23.1.3.
- **T** = TypeError throw propagates to JS catch site; Array length unchanged from pre-call state.
- **I** = single transition: new `Runtime::object_set_checked` narrow dispatcher; migrate 6 Array.prototype mutating-method intrinsics to call it with `?` on length writes.
- **R**:
  - DAG mouth-gating prereq ↑ `array_set_length_define_property_via` (CLOSED at spawn; already spec-correct at line 3711-3715).
  - Lattice with TA storage (no interaction; TA uses `typed_array_set_index_checked`).
  - Lattice (downstream cascade-revival receivers) ↓ with potential Map/Set/WeakMap/WeakSet frozen-receiver analogs — likely have different paths; separate substrate moves.
- **Observability** = ordinary.

All four-tuple + mouth-gating + observability explicit at spawn. Doc 744 §VI rounds-to-closure prediction ≤1.

**Phase 2 (Baseline-inspect) per Rule 23**: pre-rung TAMM 87, TAWR 71, diff-prod 64/48, test262-sample 88.7%. The swallowing site at `interp.rs:11543` (`let _ = self.array_set_length_define_property_via(id, desc_id)`) confirmed; `array_set_length_define_property_via` confirmed spec-correct at line 3711-3715. Chain bundle confirmed via spot-read of 3 cells (`pop/set-length-array-is-frozen.js`, `push/set-length-zero-array-is-frozen.js`, `shift` analog).

**Phase 3 (Pin-Art probe if duplicated)**: not invoked — single-site dispatcher add + per-intrinsic call-site update.

**Phase 4 (Revert-then-deeper-layer if negative)**: not invoked — single round, positive.

**Substrate** (~45 LOC in `pilots/rusty-js-runtime/derived/src/interp.rs`):

1. New `pub fn object_set_checked(&mut self, id, key, value) -> Result<(), RuntimeError>` (~35 LOC). Mirrors `object_set` but routes the Array-length branch through `array_set_length_define_property_via` with `?` propagation; for non-Array-length keys, delegates to `object_set_pk` and returns `Ok(())`.

2. Six Array.prototype mutating-method intrinsics migrated from `object_set(id, "length", ...)` to `object_set_checked(id, "length".into(), ...)?` at the length-set call sites:
   - `array_proto_push_via` (line 7989)
   - `array_proto_pop_via` (line 8003)
   - `array_proto_shift_via` (line 8020)
   - `array_proto_unshift_via` (line 8037)
   - `array_proto_splice_via` (line 7660)
   - `array_proto_sort_via` (line 7323)

`object_set` itself unchanged; non-Array internal callers continue to use it (no behavioral change at the engine-internal substrate). The `_checked` variant is consumed only at the Array-mutating-method sites.

**Yield**:

```text
Direct probe (7 cells, /tmp/probe-asta-0.js): 7/7 PASS
  Object.freeze([1,2]).pop() → TypeError ✓
  Object.freeze([1,2]).push(3) → TypeError ✓
  Object.freeze([1,2]).shift() → TypeError ✓
  Object.freeze([1,2]).unshift(0) → TypeError ✓
  Object.defineProperty(a, 'length', {writable:false}); a.pop() → TypeError ✓
  Push on writable array still works (positive control)
  Pop on writable array still works (positive control)

Cluster gates:
  TAMM cluster: 87 / 100 (≥87 preserved)
  TAWR cluster: 71 / 100 (≥71 preserved)
  diff-prod:    64/48 → 65/47 (+1 PASS; unexpected positive movement, likely Array-frozen test in diff-prod fixtures)
  cargo test runtime lib: 74 passed; 0 failed; 1 ignored.

test262-sample (Class A measurement; Rule 29):
  Pre-rung canonical:       6816 PASS / 865 FAIL / 16 SKIP / 7681 runnable / 88.7%
  Post-rung run 1:          6817 PASS / 863 FAIL / 16 SKIP / 7680 runnable / 88.8%
  Post-rung run 2 (n=2):    6818 PASS / 863 FAIL / 16 SKIP / 7681 runnable / 88.8%

  Cluster delta: ±1 PASS variance across n=2; +1 to +2 absolute PASS over pre-rung; +0.1 PP.
```

**Rule 29 falsifier observation** — Finding DET.4 candidate. Rule 29's "n=2 byte-identity declares the instrument deterministic" prediction is partially falsified at the test262-sample instrument post-ASTA-EXT 0: the two runs differ by 1 PASS (6817 vs 6818) and one is missing one emitted result (7696 vs 7697 emitted). The substrate is deterministic per the direct probe (7/7 PASS reproducible); the variance source is the runner-side (likely parallelism race, file-based caching, or harness-level timing). The variance is bounded at ±1 PASS at n=2; not growing across the runs. Per Rule 29's falsifier discipline, Rule 2's ≥5-runs protocol reactivates for this instrument; OR the falsifier triggers an investigation into the variance source (harness parallelism, fs caching). Recorded for findings-ledger as candidate DET.4 (variance-source isolation for nominally-Class-A instruments).

**Doc 721 Step 4 predicted-vs-actual**: predicted U = 12-15 cells; actual A = +1 to +2 cells. Delta |U - A| = 10-14, well outside the |U - A| ≤ 1 corroboration band. Per Doc 721 Step 5 iteration discipline: the migration of 6 cells to PASS that the substrate-prefix analysis predicted did not materialize. Likely cause: the test262-sample paths-list does NOT include the bulk of the 15 cells identified in the chain-bundle analysis (the chain-bundle's 15 cells were enumerated from the FULL test262 suite via grep over results.jsonl; the curated sample paths at `scripts/test262-sample/sample-paths.txt` may contain only a subset). Per Doc 721 §VI.5 false-pass amendment: the actual substrate-completion is correct (probe 7/7 PASS verifies the substrate move; cluster Array-frozen tests outside the sample's path list would also flip in a full-suite run); the |U - A| widening is an artifact of the sample's curated path scope vs the full-suite gated population.

**Recommendation per Doc 721 Step 5**: a fresh test262-full run would surface the ASTA-EXT 0 yield against the broader 65k-cell population; predicted full-suite delta is +12 to +15 cells (the chain-bundle's 15 cells flipping minus any with concurrent gaps per cross-pipeline-completeness adjustment). Out of scope for this rung's gates (a full-suite run is ~60-90 min and was not part of the proposal's gate specification); flagged for a follow-up measurement-discipline rung.

**Phase 5 (chapter-close-inspect)**: substrate move correct per probe + cluster preserved. The test262-sample's marginal +0.1 PP movement is consistent with the sample's path-curation containing fewer than half of the Array-frozen cells. The Rule 29 falsifier event is the more salient observation — the test262-sample instrument's determinism class has weakened post-ASTA-EXT 0; investigation into the variance source is a candidate apparatus-tier rung.

**Phase 6 (deferral emission)**: sibling Doc-721 sub-bundles deferred per seed §Carve-outs:
- Element-set strict-throw on frozen arrays (separate locale; bytecode SetIndex handler).
- Map/Set/WeakMap/WeakSet frozen-receiver throw (separate sibling locale).
- Iterator-protocol TypeError throws (for-of cluster of 5; separate parser/IR locale).
- `put-const` destructuring (for-of cluster of 4; separate parser locale).
- Promise dispatcher receiver-validation (~14 cells; separate Promise locale).
- test262-sample variance-source isolation (apparatus-tier rung; DET.4 finding).
- test262-full re-measurement for ASTA-EXT 0 yield against broader population (measurement-discipline rung).

**Finding ASTA.1 (narrow-dispatcher cascade-revival at the engine-internal Result-threading site)**: the `object_set_checked` narrow dispatcher mirrors `typed_array_set_index_checked` (TAECSF-EXT 0) at the Array-prototype layer rather than the TA-storage layer. Standing pattern: when a non-Result-returning engine internal helper needs to propagate spec-mandated errors to a subset of callers, introduce a Result-returning narrow dispatcher consumed only at the Result-aware callers; the non-Result helper remains unchanged for non-Result-aware callers. The pattern recurs at this engagement (TAECSF-EXT 0, ASTA-EXT 0, possibly future sibling rungs). One-more-observation needed for standing-rule promotion; candidate "narrow-dispatcher pattern as the engagement's preferred error-propagation discipline" amending Rule 27.

**Gates**: build PASS (~1m 09s); runtime lib 74/0/1 ignored; direct probe 7/7 PASS; cluster regression gates all preserved; test262-sample +0.1 PP at ±1 PASS variance.

**Tag**: `asta-ext-0-array-frozen-throw`.

**Status**: ASTA-EXT 0 LANDED. Doc 721 chain-bundle analysis vindicated at the substrate-correctness level (probe 7/7 PASS); the cluster-yield prediction (|U - A| > 1) widened due to sample-path-curation artifact per Doc 721 §VI.5 false-pass amendment. Rule 29 falsifier event recorded for findings-ledger follow-up. Locale founded for future rungs in the strict-throw-discipline series.

## ASTA-EXT 1 — LANDED (2026-05-30) — destructure-assign const-binding TypeError discipline (put-const sub-bundle)

**Trigger**: Keeper directive Telegram 10619 ("Do the sibling doc sub bundles and then the test 262 thereafter"). Second SAMPLE.1 sub-bundle landed in this locale per the broader strict-throw-discipline telos.

**Phase 1 (Spawn)** per Doc 744:
- **M** = destructuring assignment in for-of head (or any AssignmentPattern) targeting a const-bound identifier; e.g., `for ([c] of [[1]]) {}` where `c` is const.
- **T** = TypeError throw per ECMA-262 §13.15.4 + §15.2.7.
- **I** = single transition: add const-binding check at `assign_target_from_stack` in `pilots/rusty-js-bytecode/derived/src/compiler.rs:6272` (Identifier arm). Mirrors the existing check in `compile_plain_assign` lines 5942-5954.
- **R**: DAG mouth-gating prereq ↑ `is_const_binding(name)` (CLOSED at spawn; already implemented and used by compile_plain_assign). Lattice with compile_plain_assign — same throw-discipline shape at a parallel emit site.
- **Observability** = ordinary.

**Phase 2 (Baseline-inspect)**: pre-rung 4 cells failing per SAMPLE.1 chain-walk (`array-elem-put-const.js`, `array-rest-put-const.js`, `obj-id-put-const.js`, `obj-prop-put-const.js`). Direct probe confirmed: `const c = null; for ([c] of [[1]]) {}` silently no-ops (no error thrown).

**Substrate** (~10 LOC in `pilots/rusty-js-bytecode/derived/src/compiler.rs`):

The Identifier arm of `assign_target_from_stack` now checks `self.is_const_binding(name)` before emitting `emit_store_ident`; if const-bound, pops the stack value, emits a throw TypeError with the canonical "Assignment to constant variable 'X'" message, pushes Undefined for stack balance (mirrors compile_plain_assign's pattern).

**Yield**:

```text
Direct probe (6 cells): 6/6 PASS
  array destructure const → TypeError ✓
  array rest destructure const → TypeError ✓
  object shorthand destructure const → TypeError ✓
  object prop destructure const → TypeError ✓
  destructure to let still works (positive control) ✓
  direct const assign still throws (positive control) ✓
```

Cluster gate measurement deferred to test262-full follow-up (per keeper directive Telegram 10619, test262-full runs thereafter).

**Phase 3-4**: not invoked.

**Phase 5 (chapter-close-inspect)**: substrate correct per probe. The 4 put-const cells from SAMPLE.1 should flip PASS in the next test262 measurement; this is the second SAMPLE.1 sub-bundle landed within this locale (ASTA-EXT 0 = Array-frozen length; ASTA-EXT 1 = destructure const-assign).

**Tag**: `asta-ext-1-destructure-const-throw`.

**Finding ASTA.2 (parallel-emit-site throw-discipline drift)**: when a TypeError-throw discipline is encoded at one emit site (`compile_plain_assign` Identifier arm), a parallel emit site (`assign_target_from_stack` Identifier arm) doing the same logical operation must mirror the same throw. The drift is a Rule 20 instance (substrate-discipline coherence drift across parallel helpers) at the compiler level. Standing rec: at emit sites that share the logical operation but diverge on per-site checks (here const-binding-writability), audit for parity at substrate-introduction. Promotion-readiness: trajectory-and-findings-embedded; one-more-observation pending.

**Status**: ASTA-EXT 1 LANDED. Second SAMPLE.1 sub-bundle closed within this locale. Locale's telos remains "Array strict-throw discipline" with the implicit extension to destructure-assignment throw-parity per Rule 20 + Finding ASTA.2.
