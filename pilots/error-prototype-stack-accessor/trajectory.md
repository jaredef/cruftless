# error-prototype-stack-accessor - Trajectory

## 2026-05-29 - EPSA-EXT 0 - Phase 0 spawn + Phase 2 probe

### Directive

Helmsman directed R2 via CAACP message `d32f9cdc-f418-47c4-abc9-56b97be09db8` to spawn `pilots/error-prototype-stack-accessor/` and run a Phase 2 probe for the post-EPSUA top-10 row `Error.prototype.stack` / `feat:error-stack-accessor`. Scope is Phase 0 and Phase 2 only; no runtime substrate edits are authorized in this founding round.

### Phase 0

Locale founded at `pilots/error-prototype-stack-accessor/` with seed and trajectory. This coordinate is narrower than the whole `Error.prototype.stack` pipeline marginal: the 2026-05-29 matrix has a 35-row pipeline marginal, but the rank-3 cell is the exact 22-row `data == feat:error-stack-accessor` subset.

Rule 11 pre-spawn coverage:

- **A1 component A/B**: test262 Error prototype stack accessor expectations versus cruftless Error-family constructor/prototype installation.
- **A2 op-set**: `Object.getOwnPropertyDescriptor(Error.prototype, "stack")`, inherited accessor `[[Get]]`, inherited accessor `[[Set]]`, own stack data property creation/deletion, strict assignment and direct getter/setter calls.
- **A3 value-domain**: Error and NativeError instances, Error prototypes, plain objects, primitives, subclass instances, objects with own data/accessor shadowing, non-extensible receivers.
- **A4 locals-marshaling**: native Error constructor closures, `current_this()` receiver mutation for derived class construction, property descriptor literals, and Object descriptor observability.
- **A5 emission-shape**: runtime intrinsic installation in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`, not parser, bytecode, or JIT work.

### Phase 2 Baseline

Inspected `/home/jaredef/Developer/cruftless-r3/pilots/apparatus/test262-categorize/results/2026-05-29/matrix.md` and `categorized.jsonl`.

Matrix entry:

- Rank 3: `Error.prototype.stack` / `feat:error-stack-accessor` - 22 rows, example `built-ins/Error/prototype/stack/getter-data-property-shadows.js`.

The structure-axis marginal shows 35 total `Error.prototype.stack` rows. The other 13 rows add feature tags such as `cross-realm`, `Proxy`, `Reflect`, `Reflect.construct`, `class`, and `__proto__`; they are adjacent validation coverage for the same accessor substrate, not the founding cell.

Segmentation across the exact 22-row cell:

| Bucket | Count | Share | Shape |
|---|---:|---:|---|
| Prototype accessor missing, getter side | 7 | 31.8% | Tests read `Object.getOwnPropertyDescriptor(Error.prototype, "stack").get` or otherwise require a prototype accessor descriptor. Current descriptor is `undefined`, so the test aborts before receiver semantics are reached. |
| Prototype accessor missing, setter side | 12 | 54.5% | Tests read `.set` from the same missing descriptor, covering own-property creation, empty string round-trip, non-string rejection, receiver validation, writable/accessor shadows, and assignment. |
| Prototype accessor descriptor shape | 1 | 4.5% | `prop-desc.js` expects `get` and `set` functions with non-enumerable/configurable accessor descriptor shape; current getter type is `undefined`. |
| Error instance own stack data shape | 2 | 9.1% | `instance-no-own-stack.js` and `instance-not-enumerable.js` report no own `stack` in the 2026 sample run. Current R2 after EIPD has an own non-enumerable configurable data property on direct constructor-created instances, so these two rows need remeasurement after the accessor lands. |
| Stack-format content | 0 | 0.0% | No exact 22-cell row reaches stack formatting because descriptor lookup fails first. |

C4 passes: the setter-side missing-accessor bucket alone is 12/22 (54.5%), above the 40% coherence threshold. If getter, setter, and descriptor-shape rows are combined as one missing-prototype-accessor mechanism, the dominant mechanism is 20/22 (90.9%).

### Sampled Failures

Sampled rows:

- `getter-data-property-shadows.js`: aborts on missing `Error.prototype.stack` getter before proving own data property shadows inherited accessor.
- `getter-error-instance.js`: aborts on missing getter before checking Error/NativeError instance property access returns a string.
- `getter-error-prototype.js`: aborts on missing getter before checking prototypes return `undefined`.
- `getter-no-error-data.js`: aborts on missing getter before checking non-ErrorData receivers return `undefined`.
- `getter-this-not-object.js`: aborts on missing getter before checking primitive receiver TypeError behavior.
- `prop-desc.js`: reports getter type `undefined`, proving the descriptor itself is absent.
- `setter-creates-own-property.js`: aborts on missing setter before checking setter creates an own `stack` data property.
- `setter-empty-string.js`: aborts on missing setter before checking empty string round-trip.
- `setter-existing-own-property.js`: aborts on missing setter before checking update of an existing own property.
- `setter-non-string-value.js`: aborts on missing setter before checking non-string rejection or memory-release behavior.
- `setter-receiver-is-prototype.js`: aborts on missing setter before checking assignment to `Error.prototype.stack` throws and preserves the accessor.
- `setter-via-assignment.js`: aborts on missing getter/setter path before checking inherited assignment behavior.

### Runtime Cross-Reference

Current implementation:

- `install_error_globals` creates an Error-family prototype and installs non-enumerable `name`, `message`, `constructor`, and `toString`, but does not install `stack` on the prototype.
- The Error constructor body, after EIPD, installs per-instance `message`, `cause`, and `stack` through explicit `{w:true,e:false,c:true}` data descriptors.
- `Error.captureStackTrace` exists as a V8-extension stub and writes a target `stack` property, including a `prepareStackTrace` callback path, but it is constructor-side and does not provide `Error.prototype.stack`.
- `make_error_instance` still constructs internal Error objects with default `set_own` descriptors for `name`, `message`, and `stack`. That helper is used by internal throw/rejection paths and remains an adjacent descriptor-cleanup candidate, not the direct cause of the 22-cell prototype accessor row.

Current R2 descriptor probe:

```json
{"protoStack":null,"ownStack":{"get":"undefined","set":"undefined","value":"string","writable":true,"enumerable":false,"configurable":true},"stackType":"string","ownNames":["message","stack"]}
```

This explains the matrix: exact `feat:error-stack-accessor` rows mostly fail before their deeper receiver-specific assertions because `Object.getOwnPropertyDescriptor(Error.prototype, "stack")` returns `undefined`.

### C4 Decision

C4 passes for a coherent Phase 3 substrate move. Recommended move is an Error prototype stack accessor layer:

- install a non-enumerable configurable accessor property `stack` on every Error-family prototype, using realm-local native getter/setter functions;
- getter: require object receiver, return the receiver's own stack data string when the receiver is an ErrorData-bearing instance, return `undefined` for Error prototypes/plain objects without ErrorData, and preserve own data property shadowing through ordinary `[[Get]]`;
- setter: require object receiver and string value, reject primitives and invalid receiver/value shapes with TypeError, define or update an own non-enumerable configurable writable `stack` data property on valid ErrorData receivers, and respect non-extensible/non-writable/own-accessor/proxy paths via the existing property-definition substrate where possible;
- keep human-readable trace formatting out of the first rung, returning the existing empty string stack payload until a later formatting probe justifies deeper capture work.

Estimated closure: two substrate rungs. Rung 1 should install the prototype accessor descriptor and same-realm getter/setter behavior, expected to close most or all of the 20/22 missing-accessor rows. Rung 2 may be needed for setter edge behavior against non-extensible receivers, own accessors/non-writable data properties, or Proxy/Reflect adjacent rows after the basic accessor is present.

## 2026-05-29 - EPSA-EXT 1 - Error.prototype.stack accessor

### Directive

Helmsman directed R2 via CAACP message `b6cdf81c-772f-4b0d-a420-3ab1a318e716` to implement the Phase 3 move proposed by EPSA-EXT 0: install the `Error.prototype.stack` accessor substrate, keep trace-format content out of scope, and target closure of the 22-cell `feat:error-stack-accessor` cluster.

### Substrate Move

`install_error_globals` now installs a non-enumerable configurable own accessor descriptor on `%Error.prototype%.stack`. NativeError prototypes inherit this accessor through their existing prototype chain; they do not receive their own `stack` descriptor, matching the test262 shape that checks `Object.getOwnPropertyDescriptor(Object.getPrototypeOf(new TypeError()), "stack") === undefined`.

The getter:

- is a non-constructor native function named `get stack`;
- throws TypeError for non-object receivers;
- returns `undefined` for objects without cruftless's `InternalKind::Error` marker, including plain objects and objects inheriting from `Error.prototype`;
- returns the receiver's own string-valued `stack` data property when one exists;
- otherwise returns the current implementation-defined empty string for ErrorData-bearing instances.

The setter:

- is a non-constructor native function named `set stack`;
- throws TypeError for non-object receivers and non-string values;
- throws TypeError when called on `%Error.prototype%` itself;
- creates a default own data property `{value, writable:true, enumerable:true, configurable:true}` when the receiver lacks an own `stack`;
- preserves existing own data-property attributes when writable;
- throws on non-writable own data properties;
- invokes an own stack accessor setter when present and throws when an own accessor lacks a setter.

Fresh Error-family constructor instances and internal `make_error_instance` objects are now marked with `InternalKind::Error` and no longer eagerly install an own `stack` data property. That change is required by the current test262 `sec-properties-of-error-instances` rows: fresh instances reach `stack` via the inherited accessor; user assignment, direct setter calls, and `Error.captureStackTrace` can still create own `stack` data properties.

### Measurement

Build:

- `cargo build --release --bin cruft -p cruftless`: PASS.

Targeted test262 harness run over `/home/jaredef/test262/test/built-ins/Error/prototype/stack/*.js`:

| Probe | EPSA-EXT 0 baseline | EPSA-EXT 1 |
|---|---:|---:|
| Exact matrix cell: `Error.prototype.stack` / `feat:error-stack-accessor` | 0 PASS / 22 FAIL | 22 PASS / 0 FAIL |
| Whole `Error.prototype.stack` directory | 0 PASS / 35 FAIL | 28 PASS / 7 FAIL |

Rebased verification artifact: `/home/jaredef/Developer/cruftless-r2-sidecar/results/epsa-ext1-verify-20260529T160657Z/`.

Exact 22-cell rows now passing:

- `getter-data-property-shadows.js`
- `getter-error-instance.js`
- `getter-error-prototype.js`
- `getter-no-error-data.js`
- `getter-this-not-object.js`
- `instance-no-own-stack.js`
- `instance-not-enumerable.js`
- `prop-desc.js`
- `setter-creates-own-property.js`
- `setter-delete-round-trip.js`
- `setter-empty-string.js`
- `setter-existing-own-property.js`
- `setter-no-argument.js`
- `setter-non-error-receiver.js`
- `setter-non-extensible-receiver.js`
- `setter-non-string-value.js`
- `setter-non-writable-stack.js`
- `setter-own-accessor.js`
- `setter-receiver-is-other-prototype.js`
- `setter-receiver-is-prototype.js`
- `setter-this-not-object.js`
- `setter-via-assignment.js`

Residual wider-directory rows:

- `getter-cross-realm.js` and `setter-cross-realm.js`: realm cloning does not yet provide the foreign `Error` constructor/prototype surface expected by `$262.createRealm()`.
- `getter-receiver-is-proxy.js`: proxy receiver lacks ErrorData but currently forwards the target's ErrorData shape through property access.
- `setter-proxy-trap-rejects.js`, `setter-proxy-trap-throws.js`, `setter-proxy-wrapping-prototype.js`, `setter-receiver-is-proxy.js`: setter Proxy `[[GetOwnProperty]]` / `[[DefineOwnProperty]]` / `[[Set]]` edge routing is deferred to EPSA-EXT 2 as requested.

### Findings

**Finding EPSA.1 (prototype accessor requires explicit ErrorData marker)**: the Phase 2 probe treated the existing own `stack` data property as usable substrate, but the current test262 accessor surface requires fresh instances to have no own `stack` property. A prototype accessor that returns strings for real Error instances and `undefined` for `Object.create(Error.prototype)` therefore needs a runtime ErrorData marker. Cruftless already had `InternalKind::Error`; EPSA-EXT 1 wires Error construction and internal error creation to that marker.

**Finding EPSA.2 (NativeError prototypes inherit, not own, stack)**: installing the accessor on each NativeError prototype would fail the instance-shape checks. The durable coordinate is `%Error.prototype%` plus the existing NativeError-prototype chain.

### Status

EPSA-EXT 1 closes the exact 22-cell post-EPSUA cluster. EPSA remains open only for wider-directory cross-realm and Proxy residuals.
