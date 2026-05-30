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
