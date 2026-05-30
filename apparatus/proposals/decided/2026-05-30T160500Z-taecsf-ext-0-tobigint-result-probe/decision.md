---
proposal_slug: 2026-05-30T160500Z-taecsf-ext-0-tobigint-result-probe
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10566
decided_at: 2026-05-30T16:30:00Z
covers_commits:
  - 1e86ed67
---

## Findings

Approved per keeper Telegram 10566 ("Continue. Approved") in the helmsman-keeper dyad (no arbiter appointed; keeper-substituted approval per triumvirate operational §II.4 pre-arbiter-instantiation carve-out). Proposal authored as action 3 of the 2026-05-30 deferrals-vs-substrate audit (audit-ledger Entry 001); selects option (ii) per deferrals-ledger Entry 010's bifurcation.

Substrate commit `1e86ed67` lands:

1. New `Runtime::typed_array_set_index_checked(...) -> Result<bool, RuntimeError>` at `pilots/rusty-js-runtime/derived/src/interp.rs:632`. BigInt64Array / BigUint64Array dispatched via the instance `__kind` internal slot through `abstract_ops::to_bigint`; spec-faithful error propagation per ECMA-262 §10.4.5.16 IntegerIndexedElementSet + §7.1.13 ToBigInt.
2. Bytecode `Op::SetIndex` handler routes the user-visible `ta[i] = v` path through the checked dispatcher at the canonical-numeric-index extensible-or-own-property branch; ordinary property-set preserved on `Ok(false)`.

`object_set_pk` signature deliberately left unchanged. Internal callers (line 11455) continue to use the unchecked path per the seed-recorded carve-out.

Founds `pilots/ta-element-coercion-spec-faithful/` (TAECSF-EXT 0). Flips `apparatus/docs/deferrals-ledger.md` Entry 010 to PROMOTED in-place; appends Entry 018 back-reference. Enrolls in `apparatus/arcs/2026-05-28-array-exotic-substrate/` as the third in-flight locale alongside TAWR (closed at EXT 6) and RBDPA (founded at EXT 0).

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS (1m 07s).
2. `cargo test --release -p rusty-js-runtime --lib` — PASS: 74 passed; 0 failed; 1 ignored.
3. Direct probe assertions via `cruft /tmp/probe-taecsf-0.js`:
   - `new BigInt64Array(1)[0] = "not a bigint"` → throws `SyntaxError: Cannot convert "not a bigint" to a BigInt`. The proposal's gate text specified TypeError, but the spec-correct error per §7.1.13 ToBigInt step "If prim is a String... If n is undefined, throw a SyntaxError exception" is SyntaxError; the probe achieves spec-faithfulness, not the proposal's slightly-wrong assertion. Recorded in trajectory.md.
   - `new BigInt64Array(1)[0] = 42n` → stores; readback `=== 42n`. ✓
   - `new BigInt64Array(1)[0] = 7` → silently coerces to `7n` (pre-existing `abstract_ops::to_bigint` accepts integral Numbers where §7.1.13 specifies TypeError). Surfaced as sub-substrate (c) for future-rung work; not addressed by the founding rung.
4. Regression gate — TAMM cluster: `pilots/typed-array-missing-method/exemplars/run-exemplars.sh` → 86/100, above the ≥82 gate.
5. Regression gate — TAWR cluster: `pilots/typed-array-wrong-result/exemplars/run-exemplars.sh` → 67/100, up from 63/100 baseline (+4 PASS this rung).
6. Regression gate — diff-prod: `scripts/diff-prod/run-all.sh` → 64/48, up from 61/51 baseline (+3 PASS this rung).

## Cumulative yield

The 2026-05-30 deferrals audit produced three commits (b8249fb5 un-defer 009 + founding; e2e75f80 supersede 014 via MILF-EXT 7.1; 1e86ed67 un-defer 010 + probe + founding) and one apparatus addition (6456c35e audit-ledger). Arc `2026-05-28-array-exotic-substrate` advanced from 2 in-flight locales to 3 (TAWR closed; RBDPA + TAECSF founded). Manifest grew 227 → 229 locales across the session.

## Named follow-up

Carry-forward sub-substrates within `pilots/ta-element-coercion-spec-faithful/`:
- (a) integer-kind `ConvertNumberToTypedArrayElement` per §10.4.5.16 — next rung within the locale.
- (b) Float32 canonical-NaN preservation per §6.1.6.2 — next rung within the locale.
- (c) Number→BigInt spec deviation in `abstract_ops::to_bigint` (lattice with deferrals-ledger Entry 001 `bigint-arithmetic-wrongness`) — cross-locale; addressed when Entry 001 promotes.

Finding TAECSF.1 (narrow dispatcher beats wide signature lift for Result-threading) is a standing-rec candidate for promotion to `apparatus/docs/predictive-ruleset.md` after the second observation per Doc 727 §X duplication-as-Pin-Art-signal.

## Worktree state at decision time

Surfaced-but-not-acted by this session (consistent with the audit-ledger Entry 001 surfaced-findings list):
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` has 9 uncommitted lines unrelated to this proposal.
- Stray directory `pilots/pilots/test262-categorize/` present; appears to be an accidental nested path.
- CAACP arbiter-inbox + helmsman-outbox + ack files for the 2026-05-29 stage-b-verify message remain untracked (consistent with deferrals-ledger Entry 011 workaround state).

**APPROVED for push** per keeper-substituted authorization Telegram 10566.
