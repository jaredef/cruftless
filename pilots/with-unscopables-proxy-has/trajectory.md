# with-unscopables-proxy-has — Trajectory

## WUPH-EXT 0 — FOUNDING (2026-05-26)

Spawned as the first nested WBMS residual locale after WBMS-EXT 2 closed the
first runtime-semantics rung. The parent residual now names several deeper
environment-record surfaces; this locale isolates the `with` binding predicate:
`HasProperty` with Proxy `has` dispatch plus `@@unscopables` exclusion.

Baseline:
- Parent WBMS pool after WBMS-EXT 2: 73/264 PASS, 183 FAIL, 8 SKIP.
- Focused 112-record residual probe: 7 PASS / 105 FAIL.
- First visible Proxy `has` failures:
  - `built-ins/Proxy/has/call-with.js` fails with `attr is not defined`.
  - `built-ins/Proxy/has/return-true-target-prop-exists-using-with.js` leaves
    trap call count at 0.
  - `built-ins/Proxy/has/trap-is-undefined-using-with.js` fails to fall through
    to target property lookup.

Hypothesis: WBMS-EXT 2 correctly added a `with_env_stack`, but its binding
predicate uses `Runtime::has_property`, a read-only proto-chain walker. That
bypasses the existing Proxy `has` substrate and has no `@@unscopables` check.

Next rung: WUPH-EXT 1 should introduce a `with`-specific `HasBinding` helper
and route `resolve_with_name_base`, `load_with_name`, and `store_with_name`
through it.

## WUPH-EXT 1 — PARTIAL LANDED (2026-05-26)

### Root cause

WBMS-EXT 2 routed `with` identifier lookup through a plain proto-chain helper:
`Runtime::has_property`. That helper is correct for sparse-array style
presence checks, but it is not the Object Environment Record predicate. It
does not dispatch Proxy `has`, cannot propagate revoked/trap errors, and does
not perform the `Symbol.unscopables` exclusion step.

After the first Proxy `has` probe moved, two more pieces of the same
environment-record operation surfaced:

- `GetBindingValue` and `SetMutableBinding` re-check `HasProperty` after
  `HasBinding` selected the object environment record.
- Once a Proxy binding object is selected, value reads and writes use internal
  `[[Get]]` / `[[Set]]`, not raw descriptor reads/writes.

### Edit

- Added `has_property_with_proxy` for the mutable Proxy-aware `HasProperty`
  path with `apply_proxy_has_invariant`.
- Added `spec_get_pk` so `with` can fetch `Symbol.unscopables` via a
  property-key-aware `[[Get]]` path.
- Added `with_object_has_binding`, applying `Symbol.unscopables` exclusion
  after a successful `HasProperty`.
- Routed `resolve_with_name_base`, `load_with_name`, `store_with_name`,
  `LoadWithNameRef`, and `StoreWithNameRef` through the new object-environment
  helpers.
- Added the second `HasProperty` check required by `GetBindingValue` and
  `SetMutableBinding`.

### Probes

- `built-ins/Proxy/has/call-with.js`: FAIL -> PASS.
- `built-ins/Proxy/has/return-true-target-prop-exists-using-with.js`: FAIL -> PASS.
- Smoke: `var x="outer"; var o={x:"inner"}; o[Symbol.unscopables]={x:true}; with(o){console.log(x);}` prints `outer`.
- Smoke: Proxy `has` + `get` through `with` prints bound value and observes
  trap calls.
- Parent WBMS pool: 73/264 -> 78/264 PASS, 8 SKIP unchanged.

### Residual WUPH-local blockers

- `built-ins/Proxy/has/trap-is-undefined-using-with.js` now isolates an
  Array.prototype `length` descriptor gap rather than Proxy `has` dispatch.
- Proxy-env Test262 log probes now have the right operation shape but expose
  the engine's well-known Symbol stringification convention:
  actual `get:@@unscopables`, expected `get:Symbol(Symbol.unscopables)`.
- Tests using `get [Symbol.unscopables]()` in object literals still do not call
  the getter; the object-literal computed accessor surface is a separate parser
  / lowering gap adjacent to, but below, this HasBinding rung.
- Proxy `Set` sequencing still lacks the full `Reflect.set` /
  `getOwnPropertyDescriptor` / `defineProperty` observable trace expected by
  the deepest SetMutableBinding proxy probes.

### Finding

**Finding WUPH.1 (with HasBinding is a three-operation envelope, not a single
presence check)**: Object Environment Record lookup first performs
Proxy-aware `HasProperty`, then `Get(@@unscopables)`, then later
`GetBindingValue` / `SetMutableBinding` performs another `HasProperty` before
the value operation. Collapsing this to one boolean check is enough for simple
with semantics but fails every proxy-observable probe.

### Status

WUPH-EXT 1 is a partial closure. The core HasBinding rung is in place and
produces +5 parent-pool yield. The locale remains open for the adjacent
well-known Symbol display, computed accessor literal, Array.prototype length,
and Proxy Set sequencing blockers.
