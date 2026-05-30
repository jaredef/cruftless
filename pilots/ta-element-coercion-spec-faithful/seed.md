# ta-element-coercion-spec-faithful — Seed

## Telos

Materialize the engine-DAG coordinate

```
runtime/buffer-typed-array :: E3/intrinsic-object:ecma-262 :: element-coercion/spec-faithful :: error-propagation/Result-threaded
```

Make TypedArray element-set (the user-visible `ta[i] = v` path) spec-faithful per ECMA-262 §10.4.5.16 IntegerIndexedElementSet + §7.1.13 ToBigInt + §7.1.5 ToInt32 family. The substrate is the Result-threaded coercion path that propagates `RuntimeError` from the abstract op to the bytecode SetIndex handler so user code observes spec-correct error semantics (TypeError / SyntaxError / RangeError per the input shape).

## Origin

Promoted from `apparatus/docs/deferrals-ledger.md` Entry 010
(`ta-element-coercion-spec-faithful`, 2026-05-28) on 2026-05-30 per keeper
APPROVED of helmsman proposal
`apparatus/proposals/pending/2026-05-30T160500Z-taecsf-ext-0-tobigint-result-probe/proposal.md`.
Founded by TAECSF-EXT 0 (the probe rung that selected option (ii) — narrow
Result-returning TA element-set dispatcher — over option (i) wide
`object_set_pk → Result` lift).

## Work shape

**Heuristics §IV classification**: D (Runtime Intrinsic Semantics) — element-coercion error semantics.

Three sub-substrates per Entry 010's structure:

- **(a)** Integer-kind coercion via ConvertNumberToTypedArrayElement per §10.4.5 (Int8 / Uint8 / Uint8Clamped / Int16 / Uint16 / Int32 / Uint32). Deferred to post-probe rungs; engine currently stores raw values without spec-faithful clamping / modular reduction.
- **(b)** Float32 canonical-NaN preservation per §6.1.6.2 (NumberToRawBytes for Float32 elements). Deferred to post-probe rungs; engine currently stores the raw f64 without f32 canonical-NaN folding.
- **(c)** BigInt-kind coercion via ToBigInt per §7.1.13 (BigInt64 / BigUint64). **Closed at founding by TAECSF-EXT 0**: the user-visible `ta[i] = v` path on a BigInt-TA receiver routes through `Runtime::typed_array_set_index_checked` which calls `to_bigint(self, &value)?` before delegating to the existing storage path.

## Apparatus

- **Exemplar suite** (to land at EXT 1): paths covering the ~10 BigInt-TA element-set cells from the TAWR pool plus the integer-kind + Float32 sub-substrate rings.
- **Sibling apparatus**: `pilots/typed-array-wrong-result/exemplars/` and `pilots/typed-array-missing-method/exemplars/` are the regression-gate instruments. TAWR moved 63 → 67 / 100 across this locale's founding rung; TAMM held at ≥82 / 100.
- **Probe instrument** (lives in trajectory): the three-case probe
  - `new BigInt64Array(1)[0] = "not a bigint"` → SyntaxError (StringToBigInt failure on a non-BigInt-coercible string per §7.1.13).
  - `new BigInt64Array(1)[0] = 42n` → stores; readback === 42n.
  - `new BigInt64Array(1)[0] = 7` → silently coerces because the engine's `to_bigint` accepts integral Numbers (pre-existing spec deviation from §7.1.13 step "If prim is a Number, throw a TypeError"). Surfaced as a known sub-substrate, not addressed by the founding rung.

## Methodology

Per heuristics §VIII Debugging Rule, every substrate rung against this
coordinate must satisfy:

- large enough to matter — ~10 BigInt-TA cells + ~20 integer-kind cells +
  Float32 NaN ring; well above the spawn threshold.
- coherent across examples — to be verified per rung via the BigInt-TA
  cell family marginal at the close of each EXT.
- comparable within one availability class — yes (single intrinsic-object
  availability, single coercion-error cut at the bytecode SetIndex handler).
- owned by one resolver instance or one shared abstract op — `to_bigint`
  (BigInt) + ConvertNumberToTypedArrayElement (integer) + NumberToRawBytes
  (Float32 NaN) are the three shared abstract ops; each rung pulls 5+
  records per heuristics §V row-coherence.
- not measurement residue — confirmed (founding rung's substrate move at
  `interp.rs::typed_array_set_index_checked` is named-mechanism).
- measurable by matrix shift after landing — yes (TAWR moved +4 at EXT 0
  founding; diff-prod moved +3 PASS; subsequent rungs will report deltas
  in the same instruments).

Per heuristics §V, before any substrate edit:

```
rg -l 'BigInt64Array\|BigUint64Array' /path/to/test262/test/built-ins/TypedArray*
```

Inspect cells before claiming shared mechanism; split before editing if
sub-substrates diverge.

## Carve-outs

- **Coordinate scope at founding is BigInt-TA element-set**. Integer-kind
  ConvertNumberToTypedArrayElement + Float32 canonical-NaN preservation are
  sibling sub-substrates within this locale's telos but addressed by
  post-founding rungs.
- **Spec deviation in `to_bigint` for Number input** is a known issue (the
  engine accepts integral Numbers where §7.1.13 specifies TypeError).
  Addressing it here would cascade into BigInt constructor + asIntN /
  asUintN / Atomics BigInt overloads. Routed as either a separate locale
  or a sibling rung within the lattice-meeting Entry 001
  (`bigint-arithmetic-wrongness`) once that deferral promotes.
- **`object_set_pk` internal callers** continue to use the unchecked TA
  path (line 11455). The probe explicitly does NOT modify `object_set_pk`'s
  signature; internal infrastructure assignments to TA receivers (rare)
  preserve current behavior. If a downstream rung surfaces a need for
  spec-faithful coercion via the internal path, it lifts to option (i)
  per Entry 010's bifurcation; that escalation is named in the trajectory.
- **Proxy-target TA assignment** (the line 14080-ish proxy fallback branch
  in Op::SetIndex) is not routed through the checked path at founding.
  Surfaced for follow-up if a test262 cell flags it.

## Composes-with

- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`
- `apparatus/docs/predictive-ruleset.md` (rules 4, 5, 6, 11, 13, 15 most
  relevant; rule 6 surface-completeness audit applies for the integer-kind
  sub-substrate when ConvertNumberToTypedArrayElement lands)
- `pilots/typed-array-wrong-result/` (sibling; TAWR-EXT 5 + 6 closed the
  Phase-5 inflection that motivated the probe-pending bifurcation; TAWR
  remains the regression-gate cluster)
- `pilots/typed-array-missing-method/` (sibling; TAMM regression gate)
- `pilots/resizable-buffer-detection-per-access/` (sibling; founded
  2026-05-30 from deferrals-ledger Entry 009)
- `apparatus/arcs/2026-05-28-array-exotic-substrate/` (enrolling arc;
  third in-flight locale alongside TAWR closed and RBDPA founded)
- `apparatus/docs/deferrals-ledger.md` Entry 010 (origin) + Entry 018
  (PROMOTED back-reference)
- `apparatus/docs/audit-ledger.md` Entry 001 (authoring audit)

## Resume protocol

Read `trajectory.md` tail; the founding rung is TAECSF-EXT 0 (2026-05-30).
First post-founding rung target: integer-kind ConvertNumberToTypedArrayElement
sub-substrate (a) — promote `typed_array_set_index_checked` to dispatch
non-BigInt kinds through a §10.4.5.16 step 5 implementation rather than
storing raw values. Expected single-rung close.

**Status**: FOUNDED 2026-05-30 by TAECSF-EXT 0 (helmsman session, keeper directive Telegram 10566). Founding rung LANDED with positive yield on TAWR + diff-prod; sub-substrates (a) integer-coercion and (b) Float32 canonical-NaN pending.
