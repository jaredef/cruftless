---
name: with-unscopables-proxy-has
description: Object Environment Record HasBinding for `with`: dispatch Proxy `has`, then apply Symbol.unscopables exclusion before binding identifier references.
type: project
---

# with-unscopables-proxy-has — Seed

## Substrate-pilot — Tier K, environment-record residual nested under WBMS

Spawned from `pilots/with-body-multi-statement-parse/` after WBMS-EXT 2 moved
`with` from parser stub to typed AST + bytecode/runtime support. WBMS residuals
are no longer a single parse bucket; this locale isolates the object-environment
`HasBinding` predicate used by `with`.

## Telos

Implement the ECMA-262 Object Environment Record `HasBinding(N)` behavior for
`with` environments:

1. `HasProperty(bindings, N)` must use the ordinary internal `[[HasProperty]]`
   path, including Proxy `has` trap dispatch and its invariants.
2. If the object environment is a `with` environment and the binding is found,
   read `bindings[Symbol.unscopables]`; if it is an object and
   `ToBoolean(unscopables[N])` is true, the binding is excluded and lookup
   falls through to outer environments.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs`
  - `resolve_with_name_base`
  - `load_with_name`
  - `store_with_name`
  - `Op::In` and `Reflect.has` as existing Proxy `has` exemplars
- `pilots/with-body-multi-statement-parse/exemplars/exemplars.txt`
  - parent pool for residual yield checks
- Targeted Test262 probes:
  - `built-ins/Proxy/has/call-with.js`
  - `built-ins/Proxy/has/return-true-target-prop-exists-using-with.js`
  - `built-ins/Proxy/has/trap-is-undefined-using-with.js`
  - `language/statements/with/symbol-unscopables.js` and adjacent
    unscopables records present in the sidecar checkout

## Baseline (FOUNDING)

WBMS-EXT 2 parent pool: 73/264 PASS, 183 FAIL, 8 SKIP.

Focused residual probe before founding: 112 records across Proxy `has`,
unscopables-named with tests, legacy Sputnik with-global cases, and control-flow
cases yielded 7 PASS / 105 FAIL. The first three Proxy `has` records all fail
because `with` lookup bypasses Proxy `has`:

- `built-ins/Proxy/has/call-with.js` → `attr is not defined`
- `built-ins/Proxy/has/return-true-target-prop-exists-using-with.js` → trap call
  count remains `0`
- `built-ins/Proxy/has/trap-is-undefined-using-with.js` → fallback target
  lookup misses `length`

## Methodology

### WUPH-EXT 1 — object-environment HasBinding

Add a narrow runtime helper for `with` binding lookup. It should:

- accept `&mut self` because Proxy traps and unscopables getters can execute JS;
- reuse the existing Proxy `has` invariant helper rather than extending the
  read-only `has_property` fast path;
- return `Result<bool, RuntimeError>` so revoked proxies and trap errors
  propagate through identifier lookup;
- leave non-`with` fast paths unchanged.

## R13 prospective C1-C4

- C1 (sibling): HOLDS — `Op::In` and `Reflect.has` already materialize Proxy
  `has` dispatch and invariant checks.
- C2 (shape-compat): HOLDS — WBMS centralized `with` object-environment lookup
  in three runtime helper call sites.
- C3 (cost-positive): HOLDS for correctness yield, but not claimed as a
  performance move; `with` is already a slow-path surface.
- C4 (bail-safe): HOLDS — ordinary objects keep the proto-chain lookup, missing
  `handler.has` falls back to target `HasProperty`, and non-object
  `@@unscopables` is ignored per spec.

## Carve-outs

- Global object aliasing (`this.p1 = 1` legacy Sputnik cases) belongs to a
  separate `global-this-binding` locale.
- Call-base receiver preservation, direct/indirect eval environments,
  destructuring order, and abrupt cleanup remain separate WBMS residual
  sub-surfaces.

## Resume protocol

Read this seed, then `trajectory.md` tail, then the parent WBMS trajectory tail.
Treat the locale as closed only after the targeted Proxy `has` probes and
unscopables probes move for the expected reason and the WBMS parent pool is
remeasured.
