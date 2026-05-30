---
helmsman_session: helmsman-2026-05-30-deferrals-action-3
proposed_commits:
  - pending
target_branch: main
summary: TAECSF-EXT 0 — probe rung for deferrals-ledger Entry 010; option (ii) narrow Result-returning TA element-set dispatcher; founds `pilots/ta-element-coercion-spec-faithful/`.
risk_class: substrate
gates_pre:
  diff_prod: 61 PASS / 51 FAIL (last canonical pre-arc-closures baseline)
  tamm_cluster: 82 / 100 (post-TAWR-EXT 5)
  test262_sample: 86.7% per 2026-05-27 canonical
gates_post:
  build: cargo build --release --bin cruft -p cruftless — pending
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib — pending
  diff_prod: ≥ 61/51 (regression gate)
  tamm_cluster: ≥ 82/100 (regression gate)
  probe_cell: new BigInt64Array(1)[0] = "bad" → TypeError (assertion gate)
---

## Substrate Moves

Probe rung for deferrals-ledger Entry 010 (`ta-element-coercion-spec-faithful`). Entry 010 was gated on a probe-rung choice between two `Result`-threading architectures for ToBigInt error-propagation through typed-array element-set:

- **(i)** Lift `object_set_pk` to `Result<(), RuntimeError>`. Wide blast radius across ~20+ call-sites (mapped-arguments, array-length-set, generic property-set). High regression risk on TAMM and on the broader arc.
- **(ii)** Route TA element-set through a dedicated Result-returning path before reaching `object_set_pk`. Narrow blast radius; new dedicated dispatcher; no signature change to the general property-set path.

**This probe lands option (ii).**

Substrate move (target ~60 LOC, single file `pilots/rusty-js-runtime/derived/src/interp.rs`):

1. New method `Runtime::typed_array_set_index_checked(&mut self, id: ObjectRef, idx: usize, value: Value) -> Result<bool, RuntimeError>` (~40 LOC).
   - Dispatch on the view's element-type. For BigInt64 / BigUint64 kinds, call `abstract_ops::to_bigint(self, &value)?` first; the error propagates spec-faithfully.
   - For non-BigInt kinds, this probe punts to the existing storage path unchanged (`ConvertNumberToTypedArrayElement` work for integer + Float32 canonical-NaN deferred to subsequent rungs per Entry 010 sub-substrates (a) and (b)).
   - On successful coercion, delegate to the existing `typed_array_set_index(id, idx, coerced_value)` for storage. Returns `Ok(true)` on TA element-set success; `Ok(false)` when the object is not a TA (the caller falls through to general property-set); `Err(...)` on coercion failure.

2. Call-site update at `object_set_pk` (canonical-numeric-index branch, current line ~11425). Route the canonical-numeric-index key path through `typed_array_set_index_checked` before falling through to the generic set. `Err` propagates; `Ok(true)` returns; `Ok(false)` falls through.

Founds new locale `pilots/ta-element-coercion-spec-faithful/`:
- `seed.md` — telos (Result-threaded ToBigInt error-path for TA element-set per ECMA-262 §10.4.5 IntegerIndexedElementSet + §7.1.13 ToBigInt), apparatus (this probe + the ~10-cell BigInt-TA cell ring), methodology, carve-outs (BigInt-only at founding; integer kinds + Float32 NaN deferred to post-probe rungs), composes-with, resume protocol.
- `trajectory.md` — TAECSF-EXT 0 founding rung with M/T/I/R per Doc 744 §V.1 + post-rung gate readings.

Updates:
- `apparatus/docs/deferrals-ledger.md` — Entry 010 status flipped in-place to PROMOTED; new Entry 018 appended as back-reference per ledger §Discipline.
- `apparatus/arcs/2026-05-28-array-exotic-substrate/arc.md` — sub-locale roster row added for `ta-element-coercion-spec-faithful` as the third in-flight arc locale (alongside TAWR closed at EXT 6 and RBDPA EXT-0 founded 2026-05-30).
- `apparatus/locales/manifest.json` — refresh via `apparatus/locales/discover.sh`. Expected delta 228 → 229 locales (+1 top-level).
- `apparatus/docs/audit-ledger.md` — Entry 001's "Authored actions" section already references action 3 as separate; this proposal's commit will be appended to that entry's actions list (in-place edit on prior entries is allowed per the audit-ledger §Discipline mirror of deferrals-ledger).

## Verification

To run before requesting arbiter sign-off (or keeper sign-off in lieu, per Stage-2 carve-outs):

1. `cargo build --release --bin cruft -p cruftless` — must PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — must PASS with no new failures.
3. Direct probe assertions:
   - `node -e 'const ta = new BigInt64Array(1); try { ta[0] = "not a bigint"; console.log("NO_ERROR"); } catch (e) { console.log(e.constructor.name); }'` via `cruft` must print `TypeError`.
   - `ta[0] = 42n` must succeed and `ta[0] === 42n` must be true.
4. Regression gate — TAMM cluster: run `pilots/typed-array-missing-method/exemplars/run-exemplars.sh` post-substrate; must remain ≥ 82/100.
5. Regression gate — diff-prod: `scripts/diff-prod/run-all.sh`; must remain ≥ 61/51 at the runtime-semantics probe.
6. Optional follow-up: re-categorize the BigInt-TA failure cluster from the canonical test262 full-suite results once a fresh run lands; cite delta in TAECSF-EXT 1.

## Risk Assessment

- **Blast radius**: narrow. The only call-site touched in `object_set_pk` is the canonical-numeric-index branch, which already gates on a TA-shape precondition; adding the `Result`-returning dispatcher there does not alter behavior for non-TA targets.
- **`object_set_pk` hot-path concern**: the new branch executes one extra method dispatch when the key parses as a canonical numeric index. Risk is minimal but not zero. No CRB micro-benchmark exists for this exact path; if regression is detected post-land, revert per Rule 13 and consider deferred-coercion design.
- **Spec-faithfulness scope**: probe only addresses BigInt-TA. Integer-kind coercion (sub-substrate (a) per Entry 010) and Float32 canonical-NaN preservation (sub-substrate (b)) are explicitly out of scope; subsequent rungs in the founded locale close them.
- **Lattice with Entry 001 (`bigint-arithmetic-wrongness`)**: shared `to_bigint` abstract op. If Entry 001 un-defers on a near cycle, the shared substrate becomes a minor refactor surface (estimated negligible). The probe does not block Entry 001's un-defer path.
- **Negative-result handling**: if the probe regresses TAMM or diff-prod, Rule 13 applies: verify negative, diagnose, revert via git, identify the deeper-layer closure, implement as next rung. Trajectory entry + diagnosis retained.

## Composes-With

- `pilots/rusty-js-runtime/derived/src/abstract_ops.rs` (lines 305–353) — `to_bigint` abstract op, already spec-correct; consumed unchanged.
- `pilots/rusty-js-runtime/derived/src/interp.rs` — site of `typed_array_set_index` (current ~604–625) + `object_set_pk` (current ~11425). Both touched in the probe.
- `pilots/typed-array-wrong-result/` — sibling locale, closed at TAWR-EXT 6 REVERT (Phase-5 inflection); TAWR-EXT 6's revert specifically named the deeper-layer Result-threading work as the next-rung target. This probe is that deeper-layer rung at a sibling coordinate.
- `pilots/typed-array-missing-method/` — sibling locale; regression gate.
- `pilots/ta-element-coercion-spec-faithful/` — locale founded by this proposal.
- `apparatus/docs/deferrals-ledger.md` Entry 010 + new Entry 018 — un-defer + back-reference.
- `apparatus/docs/audit-ledger.md` Entry 001 — authoring audit (deferrals-vs-substrate 2026-05-30).
- `apparatus/arcs/2026-05-28-array-exotic-substrate/arc.md` — arc enrollment.

## Authorization

Awaiting keeper (or arbiter, when appointed) APPROVED decision per the triumvirate operational protocol §II proposal+veto workflow. Keeper directive Telegram 10564 ("Write a proposal first and then continue") authorizes this proposal authorship; landing is gated on the decision.
