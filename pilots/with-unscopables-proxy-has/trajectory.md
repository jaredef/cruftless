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

## WUPH-EXT 2 — LANDED (2026-05-26)

### Root cause

The `get [Symbol.unscopables]()` residuals were not failing in the `with`
environment-record helper anymore. They were failing before that helper could
observe the object at all: object-literal accessor installation lowered the
computed key expression, but the runtime helper only accepted `Value::String`
keys and installed descriptors into the String bucket. A computed Symbol key
therefore got dropped instead of becoming a Symbol-keyed accessor descriptor.

### Edit

Updated the runtime accessor-install helpers to accept string, symbol, and
number key values through `property_key(v)` before inserting the descriptor.
This keeps object-literal computed accessors on the same PropertyKey substrate
used by computed reads/writes.

### Probes

- Smoke: `get [Symbol.unscopables]()` now installs a Symbol-keyed accessor:
  `Object.getOwnPropertySymbols(env).length` prints `1`, reading
  `env[Symbol.unscopables].x` returns `true`, and the getter call count is `1`.
- `language/statements/with/get-mutable-binding-binding-deleted-in-get-unscopables.js`: PASS.
- `language/statements/with/set-mutable-binding-binding-deleted-in-get-unscopables.js`: PASS.
- Parent WBMS pool: 78/264 -> 80/264 PASS, 8 SKIP unchanged.

### Residual

- `unscopables-get-err.js` / `unscopables-prop-get-err.js` still report
  `ReferenceError` instead of the thrown Test262Error when the identifier is
  read inside a callback function created in the with-body. A direct
  same-frame smoke propagates the thrown `Error`, so the residual points at
  function-closure capture of active with-environments, not the accessor
  `Get` path itself.
- `unscopables-inc-dec.js` still fails, likely because update-expression
  lowering is not yet using the reference-preserving with-object path added for
  assignment.
- The Proxy-env log probes still expose the well-known Symbol display mismatch
  (`@@unscopables` vs `Symbol(Symbol.unscopables)`) and deeper Proxy Set
  `DefineProperty` sequencing.

### Status

WUPH-EXT 2 closes the computed accessor literal blocker for this locale. The
locale remains open; the next coherent rung is either closure capture of active
with-environments or update-expression reference preservation.
