# private-field-runtime-slots — Seed

## Substrate-pilot — runtime redirect from private-name-lexing / CESS.

Spawned 2026-05-26 after `private-name-lexing` and `class-elements-static-semantics` closed the lexing and parse-time early-error slices. The remaining focused private-name probe exposed a runtime representation bug: private fields were stored as ordinary string properties named `"#x"`.

## Telos

Runtime storage for private class elements that does not leak through ordinary property reflection. Coordinate:

```
AST-to-bytecode / runtime-object-model ::
  E2/private-class-elements ::
  cut/private-slots-transitional ::
  property/private-fields-not-own-string-properties
```

Immediate surfaced cases:

- `Object.prototype.hasOwnProperty.call(instance, "#x")` must be false for private fields.
- Existing private method/accessor tests must remain reachable while the runtime transitions away from name-mangled ordinary properties.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/value.rs::Object` — adds a private-field storage map traced by GC but excluded from ordinary property-key paths.
- `pilots/rusty-js-runtime/derived/src/interp.rs::{object_get, object_set, Op::GetProp, Op::SetProp}` — routes compiler-generated `#name` property ops through private storage.
- `pilots/rusty-js-runtime/derived/src/interp.rs::call_function` — preserves generator terminal returns and wraps async-generator `next()` results in Promises for the eager transitional iterator.
- `pilots/rusty-js-runtime/derived/src/promise.rs::resolve_promise` — assimilates returned Promise objects so chained async test262 continuations observe the unwrapped value.
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::__install_method__` — installs private methods into private storage when the key begins with `#`.
- `pilots/rusty-js-bytecode/derived/src/compiler.rs` — marks private-member continuations whose receiver expression contains an optional chain and preserves async/generator flags for class methods.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — executes async-flag tests through a quiet `$DONE` shim and the runtime await pump.
- PNL focused probe: 194 private-name / class-elements test262 fixtures.

## Methodology

### PFRS-EXT 1 — narrow private slot map

Lift compiler-generated private field/method storage out of ordinary properties:

1. add `Object.private_fields`,
2. route `Op::SetProp` / `object_set` for `#name` keys into that map,
3. route private reads through own private storage, then prototype private storage for the current transitional private-method lowering,
4. keep private accessor descriptors reachable through the existing accessor path.

### PFRS-EXT 2 — optional-chain private continuation

Bridge `o?.c.#f` while the parser/compiler still represents the outer private member as an ordinary member over an optional-chain subexpression:

1. tag private-member bytecode reads whose receiver expression contains an optional chain,
2. let a `#name` read over the optional short-circuit `undefined` produce `undefined`,
3. preserve TypeError for ordinary missing private slots on non-nullish objects.

### PFRS-EXT 3/4 — generator and async class method flag preservation

Close the generator and async-method residuals that became visible after private slots landed:

1. preserve `is_async` and `is_generator` when lowering class methods,
2. seed the existing eager generator iterator with a non-undefined body return when the generator produced no `yield` values,
3. wrap async-generator `next()` results in resolved Promises,
4. let the test262 runner execute async-flag tests without leaking harness print output into its single JSON line,
5. assimilate Promise fulfillment values so chained test262 async continuations receive the unwrapped result.

## Carve-outs

- This is not yet a full ECMA private-brand model. Prototype private-method lookup is a compatibility bridge for the current lowering, not the final brand semantics.
- Optional-chain private-field runtime semantics are only closed for the surfaced `o?.c.#f` continuation shape.
- Async/generator support is only closed for the eager, no-suspension class-method cases surfaced by the focused PNL probe.

## Composes-with

- `pilots/private-name-lexing/` — parent exemplar probe.
- `pilots/class-elements-static-semantics/` — parser/static semantics redirect that removed parse-phase blockers before runtime residuals became visible.
- `pilots/rusty-js-runtime/derived/tests/omega_5_w.rs` — existing acceptance notes explicitly named name-mangled private fields as a v1 deviation.

## Resume protocol

Read `trajectory.md` tail. Rebuild `cruft`, run the 40-path PNL smoke and the 194-path focused PNL probe. Inspect residuals by reason before extending the slot model.

## Status

PFRS-EXT 4 landed locally. Runtime/bytecode compile; release binary keeps direct PNL at `40/40` and moves the focused PNL list from `178/194` to `194/194`.
