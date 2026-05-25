# via-method-audit — Seed

## Telos

LOAL.3 named hand-written `_via` paths as the locus where ergonomic reorderings of spec steps had introduced silent divergences from the IR-lowered counterparts in `generated.rs`. LOAL-EXT 2 fixed five Array.prototype methods that checked the callable arg before reading length. This locale extends the audit across all 228 `_via` methods in `interp.rs`.

The audit looks for two bug patterns:
1. **Spec-order divergence**: validation that should happen AFTER an observable-side-effect step (length getter, ToObject, etc.) happens before it.
2. **Static-coerce on user-arg**: `abstract_ops::to_string(&arg)` / `to_number(&arg)` at user-argument coercion boundaries where `rt.coerce_to_*` would dispatch @@toPrimitive / Object→primitive per §7.1.17 / §7.1.4 (RPTC.7 bug pattern).

This locale tracks the audit pass + fixes per finding. Per Standing Rule 21 each fix should be minimum-scope and verified.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs` — 228 `_via` methods.
- Pattern 1 grep: `is_callable(...)` line before `try_array_length` / `length_of_array_like` line within a single `_via`.
- Pattern 2 grep: `abstract_ops::to_string(&args` or `abstract_ops::to_number(&args` within `_via` bodies, at user-arg coercion boundaries.
- `Runtime::property_key(v)` helper for PK-aware key extraction.
- `has_property_pk` shape for PK-aware lookup model.

## Methodology

1. Run cb-before-len audit script over all `_via`. Inspect each hit; classify spec-correct vs bug.
2. Run static-to_string audit over `_via` bodies. Classify per RPTC.7.
3. Fix confirmed bugs in batches; verify with focused probes + diff-prod + random 300 per batch.

## Carve-outs

- `_via` methods on String.prototype that use `to_string_strict(this)` first are spec-correct (that's the spec ToObject step for the receiver).
- `_via` methods that take typed args (`this_v: &Value`, `key_v: &Value`) without going through `args[]` indexing are typically internal-call-site IR-direct and not user-callable; their coercion contract is documented in the IR.
- Static coerce in alternative-branch positions (where the value is already type-known) are not bug-pattern instances.

## Composes-with

- LOAL.3 (the audit's trigger): hand-written `_via` paths can silently invert spec-step ordering.
- LOAL.4 (the spec-order principle): observable side effects in early spec steps cannot be reordered around by the substrate.
- RPTC.7 / IPTO (the parallel coercion-divergence pattern): static `abstract_ops::*` at user-arg boundaries are grep-detectable bugs.

## Resume protocol

Read `trajectory.md` tail.
