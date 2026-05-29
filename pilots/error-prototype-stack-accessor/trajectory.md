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
