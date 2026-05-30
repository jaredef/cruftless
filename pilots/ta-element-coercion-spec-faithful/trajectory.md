# ta-element-coercion-spec-faithful — Trajectory

## TAECSF-EXT 0 — LANDED (2026-05-30) — Result-threaded BigInt-TA element-set probe

**Trigger**: Keeper APPROVED of helmsman proposal
`apparatus/proposals/pending/2026-05-30T160500Z-taecsf-ext-0-tobigint-result-probe/proposal.md`
via Telegram 10566 ("Continue. Approved"). The proposal authored action 3 of the
2026-05-30 deferrals-vs-substrate audit (audit-ledger Entry 001), un-deferring
`apparatus/docs/deferrals-ledger.md` Entry 010 by selecting option (ii)
(narrow Result-returning TA dispatcher) over option (i) (wide
`object_set_pk → Result` lift).

**Arc enrollment**: `2026-05-28-array-exotic-substrate` (third in-flight locale at land time alongside TAWR closed at EXT 6 and RBDPA founded at EXT 0).

**Phase 1 (Spawn) per Doc 744 §V.1**:
- **M** = JavaScript `ta[i] = v` on a BigInt64Array / BigUint64Array receiver.
- **T** = `to_bigint(self, &value)?` invoked before storage; coercion error propagates as `RuntimeError` per ECMA-262 §10.4.5.16 IntegerIndexedElementSet + §7.1.13 ToBigInt.
- **I** = new `Runtime::typed_array_set_index_checked(&mut self, id, idx, value) -> Result<bool, RuntimeError>` at `pilots/rusty-js-runtime/derived/src/interp.rs:632`; bytecode `Op::SetIndex` handler (current line ~14130) routes the canonical-numeric-index branch through it before falling back to `object_set_pk`.
- **R** = lattice-meet with `pilots/typed-array-wrong-result/` (Phase-5 inflection at TAWR-EXT 5 surfaced the probe-pending bifurcation; TAWR is the regression-gate cluster); lattice with deferrals-ledger Entry 001 (`bigint-arithmetic-wrongness`) via shared `to_bigint` substrate; DAG ↑ `abstract_ops::to_bigint` (consumed unchanged); DAG ↓ `typed_array_set_index` (unchanged storage delegate).
- **Observability** = ordinary (test262 cell PASS/FAIL transitions; TAWR + diff-prod cluster movement; direct probe assertion).

**Phase 2 (Baseline-inspect)**: pre-rung TAWR 63/100 (post-TAWR-EXT 5 close); pre-rung diff-prod 61/51 (last canonical baseline); pre-rung TAMM 82/100 (post-TAWR-EXT 5). Sample probe: `new BigInt64Array(1)[0] = "bad"` does not throw under the pre-rung engine; the assignment is silently swallowed because `object_set_pk`'s internal `typed_array_set_index` ignores the coercion step entirely.

**Phase 3 (Pin-Art probe if duplicated)**: not invoked — substrate move is single-site (new method + one call-site).

**Phase 4 (Revert-then-deeper-layer if negative)**: not invoked — single round, positive.

**Substrate** (~70 LOC in `pilots/rusty-js-runtime/derived/src/interp.rs`):

1. New method `typed_array_set_index_checked` (lines ~628–656). Returns `Ok(true)` when the receiver is a TA (handled, possibly OOB-noop); `Ok(false)` when not a TA (caller falls through to ordinary property-set); `Err(...)` on coercion failure. BigInt kinds (BigInt64Array, BigUint64Array) are detected via the instance's `"__kind"` internal slot (set at construction by the intrinsic).
2. Call-site update in `Op::SetIndex` handler at the canonical-numeric-index extensible-or-own-property branch (current line ~14130). Routes through the checked path; `Err` propagates per the bytecode op's existing `Result` discipline.

`object_set_pk`'s internal TA call (line ~11455) intentionally left unchanged per carve-out in seed.md.

**Yield**:

```
TAWR cluster PRE-EXT 0:  PASS=63 FAIL=37 / 100 (63.0%)
TAWR cluster POST-EXT 0: PASS=67 FAIL=33 / 100 (67.0%)
TAMM cluster PRE-EXT 0:  PASS=82 / 100 (baseline)
TAMM cluster POST-EXT 0: PASS=86 / 100 (≥82 regression gate satisfied)
diff-prod PRE-EXT 0:     PASS=61 FAIL=51 / 112
diff-prod POST-EXT 0:    PASS=64 FAIL=48 / 112
```

**+4 PASS on TAWR**, **+3 PASS on diff-prod**, **+4 PASS on TAMM** (latter is co-yield from prior interim work but holds the regression gate). No negative on any instrument.

**Probe assertions** (the proposal's gate cells):
- `new BigInt64Array(1)[0] = "not a bigint"` → throws SyntaxError("Cannot convert \"not a bigint\" to a BigInt"). **Spec-faithful**: per §7.1.13 ToBigInt step "If prim is a String... If n is undefined, throw a **SyntaxError** exception." The proposal's gate text said "TypeError"; the spec-correct error is SyntaxError for StringToBigInt failure. Probe terminus achieved.
- `new BigInt64Array(1)[0] = 42n` → stores cleanly; readback `=== 42n`. ✓
- `new BigInt64Array(1)[0] = 7` → silently coerces to `7n`. **Pre-existing engine spec deviation** in `abstract_ops::to_bigint` (accepts integral Numbers where §7.1.13 specifies TypeError). Surfaced as a known sub-substrate; not addressed by the founding rung (named in seed.md Carve-outs).

**Gates**: build PASS (`cargo build --release --bin cruft -p cruftless`); runtime lib tests 74/0/1 ignored (`cargo test --release -p rusty-js-runtime --lib`); TAMM 86/100 (≥82 gate); diff-prod 64/48 (≥61/51 gate); sanity intact.

**Tag**: `taecsf-ext-0-tobigint-result-probe`.

**Finding TAECSF.1 (Result-threading via narrow dispatcher beats wide signature lift)**: when a coercion abstract op needs to propagate its error from a deep storage path through a non-Result-returning intermediate function, prefer a narrow new dispatcher that lives in the Result-returning caller's frame over lifting the intermediate's signature. The narrow dispatcher's blast radius is the named call-site; the wide lift's blast radius is every caller of the intermediate (here ~hundreds for `object_set`). Standing rec: when a substrate rung would require lifting a function's return-type from `T` to `Result<T, E>` and the function has more than ~10 callers, evaluate whether a dispatcher-at-the-Result-site achieves the same terminus before lifting.

**Phase 6 (deferral surfacing)**: the founding rung surfaces three carry-forward sub-substrates within this locale's scope: (a) integer-kind ConvertNumberToTypedArrayElement per §10.4.5.16; (b) Float32 canonical-NaN preservation per §6.1.6.2; (c) Number→BigInt spec deviation in `abstract_ops::to_bigint` (lattice with deferrals-ledger Entry 001). Sub-substrates (a) and (b) are in-locale post-rung work; (c) is lattice cross-reference to Entry 001's BigInt namespace and routed when Entry 001 promotes.

**Status**: TAECSF-EXT 0 LANDED. Arc-tier accumulation: third productive locale in `2026-05-28-array-exotic-substrate`. Cumulative arc yield post this rung: TAWR 67/100 + TAMM 86/100 + RBDPA pending. Sub-substrates (a) + (b) queued for next rungs.

## TAECSF-EXT 1 — NEGATIVE (Rule 13 revert) (2026-05-30) — integer-kind ConvertNumberToTypedArrayElement attempt

**Trigger**: Keeper APPROVED of proposal `apparatus/proposals/pending/2026-05-30T172000Z-taecsf-ext-1-integer-kind-convert/proposal.md` per Telegram 10578 ("Approved"). Next-rung within this locale per Rule 22 axis-split prediction (integer-kind sub-substrate (a) sharing the BigInt-TA exemplar).

**Arc enrollment**: `2026-05-28-array-exotic-substrate` (no roster change — locale already enrolled).

**Phase 1 (Spawn)**:
- **M** = JavaScript `ta[i] = v` on Int8/Uint8/Uint8Clamped/Int16/Uint16/Int32/Uint32 Array receiver.
- **T** = spec-faithful ConvertNumberToTypedArrayElement per ECMA-262 §10.4.5.16 + §7.1.6–§7.1.8 (modular reduction via explicit `rem_euclid`, NOT Rust's saturating `as` cast; Uint8Clamp NaN→0 + saturation + round-half-to-even).
- **I** = new `abstract_ops::convert_number_to_typed_array_element(&Value, &str) -> Value` (~60 LOC); integer-kind branch in `Runtime::typed_array_set_index_checked` paralleling the EXT 0 BigInt dispatch (~15 LOC).
- **R** = lattice with TAMM regression-gate cluster; sibling within the locale to EXT 0's BigInt branch; DAG ↑ `abstract_ops::to_number`.
- **Observability** = ordinary (TAMM + TAWR + diff-prod cluster movement; direct probe assertions on 5 spec cells).

**Phase 2 (Baseline-inspect)**: pre-rung TAMM 86/100, TAWR 67/100, diff-prod 64/48 (re-verified against `bin/cruft` freshly refreshed from `target/release/cruft` after binary-staleness measurement-error was detected mid-rung).

**Phase 3 (Pin-Art probe if duplicated)**: not invoked at this rung.

**Phase 4 (Revert-then-deeper-layer-closure)** — **invoked per Rule 13**:

Direct probe assertions all PASS (10/10 in `/tmp/probe-taecsf-1.js`): Uint8 wrap 300→44, Int8 wrap 130→-126, Uint8Clamped NaN→0, Uint8Clamped saturate 300→255, round-half-to-even 254.5→254, etc. The substrate move at the bytecode `Op::SetIndex` site behaves spec-faithfully for the named integer kinds.

But **TAMM regressed 86 → 83 (-3 PASS)** under the EXT 1 substrate. Diff of cluster fail-set pre vs post identifies the three regressions:

1. `built-ins/TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js`
2. `built-ins/TypedArrayConstructors/internals/Set/conversion-operation-consistent-nan.js`
3. `built-ins/TypedArray/prototype/some/BigInt/values-are-not-cached.js`

**Mechanism diagnosis (in-progress; partial)**: two BigInt-TA tests + one Float NaN test regressed. None of these are integer-kind tests. The substrate edit's BigInt branch is byte-equivalent to EXT 0's; the Float fall-through (`_ => value`) is also byte-equivalent. Yet the cluster regression is real and reproducible across three independent runs. The regression is not in the direct probe cells; it surfaces only in the test262 internals/Set + internals/GetOwnProperty descriptor paths + the values-are-not-cached cache invalidation path. Hypotheses worth checking at EXT 2:
- (a) `self.object_get(id, "__kind")` now executes on every TA element-set including BigInt-TA dispatched paths; if `object_get` walks the prototype chain with a side-effect tracker (cache invalidation, property-miss log), it may invalidate something the BigInt-TA cache tests rely on.
- (b) The `kind` String allocation in the new code (`s.as_str().to_string()`) may interact with a refcount-sensitive code path in BigInt-TA's value-cache test.
- (c) The Set/conversion-operation-consistent-nan test may be sensitive to how NaN is stored; the fall-through arm of the new match is structurally identical but the surrounding match arm ordering changes branch prediction or codegen in a way that surfaces a pre-existing latent NaN-bit-pattern divergence.

**Per Rule 13 prospective application**: substrate edit at `interp.rs::typed_array_set_index_checked` reverted to EXT 0 form. The `abstract_ops::convert_number_to_typed_array_element` helper retained on disk as the substrate prefix that the EXT 2 deeper-layer closure will consume (per Finding IR.33 cumulative substrate amortization). Diff-prod fixture (`scripts/diff-prod/fixtures/typed-arrays/exec.mjs`) overflow cells kept commented with an updated note pointing at this rung's NEGATIVE result and the deferred deeper-layer closure.

**Yield**:

```text
TAMM cluster PRE-EXT 1 / POST-REVERT:  PASS=86 FAIL=14 / 100 (86.0%)
TAMM cluster POST-EXT 1 (substrate engaged):  PASS=83 FAIL=17 / 100 (83.0%) — REGRESSION
TAWR cluster (stable across rung):  PASS=67 FAIL=33 / 100 (67.0%)
diff-prod (post-revert):  PASS=64 FAIL=48 / 112 — gate intact
```

**Net rung yield: 0 PASS (revert preserves EXT 0 baseline).** The substrate move did not land.

**Gates (post-revert)**: build PASS; runtime lib 74/0/1 ignored; TAMM 86/100; TAWR 67/100; diff-prod 64/48. Direct probe assertions no longer PASS for the integer-kind cells (the dispatch is reverted) but BigInt EXT 0 regression check stays clean.

**Tag**: `taecsf-ext-1-integer-kind-NEGATIVE-rule13-revert`.

**Finding TAECSF.2 (a-spec-faithful-direct-probe-can-coexist-with-test262-cluster-regression-via-non-obvious-shared-state)**: when a substrate edit passes the direct probe assertions designed by the proposal but fails the cluster regression-gate, the mechanism is rarely in the substrate edit's named scope; spot-read residuals per Rule 22 and look for shared state at the dispatch site (prototype-chain reads, cache invalidation, refcount sensitivity, match-arm codegen) that the substrate edit incidentally exercises. The revert + diagnose-then-deeper-layer rung is the discipline; immediate forward-pursuit is the anti-pattern.

**Phase 5 (chapter-close-inspect at NEGATIVE)**: TAECSF-EXT 1 is a Rule-13 instance — verify (3 stable runs), diagnose (3 named regressions; hypotheses (a)/(b)/(c)), revert (interp.rs to EXT 0 form), identify deeper-layer (read the 3 failing tests to confirm which hypothesis holds, redesign the dispatch site to either avoid the touched-shared-state or migrate to a different routing point such as `typed_array_set_index` interior rather than `_checked` wrapper).

**Phase 6 (deferral emission)**: EXT 2 is queued as the deeper-layer closure rung. Open hypotheses (a)/(b)/(c) named above; first action at EXT 2 is to spot-read the three regressing test262 cells to converge on the correct hypothesis. The proposal at `apparatus/proposals/pending/2026-05-30T172000Z-taecsf-ext-1-integer-kind-convert/proposal.md` remains in pending/ as the historical record of the original design; the EXT 2 proposal will supersede it with a refined dispatch design.

**Status**: TAECSF-EXT 1 REVERTED locally (Rule 13). The `abstract_ops::convert_number_to_typed_array_element` helper remains on disk as substrate prefix per Finding IR.33; the integer-kind dispatch wiring at `interp.rs` reverted to EXT 0 form. Locale's TAECSF-EXT 2 queued for the deeper-layer closure.

## TAECSF-EXT 1.1 — convergent diagnosis (2026-05-30) — engine-architectural root cause

**Trigger**: Keeper directive Telegram 10580 ("Yes, investigate") to converge on hypothesis (a) / (b) / (c) named in EXT 1 NEGATIVE trajectory entry. Re-applied EXT 1 substrate temporarily; ran single failing test262 cell with stderr capture; spot-read the harness path to identify the regression mechanism.

**Method**: re-applied integer-kind dispatch in `typed_array_set_index_checked`; rebuilt cruft; ran `built-ins/TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js` via test262 runner. Output: `Expected SameValue(«42», «42n») to be true`. Traced upstream: test uses `testWithBigIntTypedArrayConstructors` → `makeResizableArrayBuffer(BigInt64Array, [42n, 43n])` (test262/harness/testTypedArray.js:114) → `copyIntoArrayBuffer(resizable, fixed)` → `for (var i = 0; i < srcView.length; i++) destView[i] = srcView[i]` where both views are Uint8Array views over BigInt64-shaped buffers. This is the `Op::SetIndex` path; under EXT 1, the Uint8Array kind dispatches to `convert_number_to_typed_array_element` which calls `to_number(Value::BigInt(42n))` → 42.0 → stores Value::Number(42) in the cell that the subsequent BigInt64Array view re-reads as 42 (Number, not 42n BigInt).

**Convergent finding**: NONE of hypotheses (a) (object_get prototype-chain side effect), (b) (kind String allocation refcount), (c) (match-arm codegen / branch prediction) was correct. The actual root cause is engine-architectural: `ArrayBufferData.data: Vec<Value>` stores Values at byte indices rather than actual bytes. Multiple views aliased to the same buffer share Value cells. Coercion at SetIndex breaks the view-aliasing pass-through invariant the test262 harness exploits for resizable-buffer setup. Recorded as Finding TAECSF.3 in this locale's `findings.md`.

**Implication**: TAECSF-EXT 2 cannot land integer-kind coercion at the SetIndex dispatcher alone. The deeper-layer closure is a precursor architectural rung migrating `ArrayBufferData.data` to `Vec<u8>` with NumberToRawBytes encoding per §6.1.6.1 — substrate scope larger than this locale's telos; likely founded as a sibling locale `typed-array-byte-storage-conformance` rather than continued as TAECSF-EXT 2.

**Action**: re-reverted the EXT 1 substrate (interp.rs restored to EXT 0 BigInt-only form). Built and copied fresh binary. Gates: TAMM 86, TAWR 67, diff-prod 64/48 (post-revert baseline preserved). Sub-substrates (a) integer-kind and (b) Float32 canonical-NaN within this locale's seed jointly DEFERRED on the precursor architectural rung. Finding TAECSF.3 promoted-readiness: one-more-observation, candidate apparatus standing-rule on substrate-spec-correctness-vs-engine-architecture conflict.

**Status**: TAECSF-EXT 1.1 convergent diagnosis CLOSED. The EXT 1 NEGATIVE result has its mechanism identified; the locale's progress is gated on a precursor architectural rung outside the locale's current scope.
