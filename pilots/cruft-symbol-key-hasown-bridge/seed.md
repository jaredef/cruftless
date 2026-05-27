# cruft-symbol-key-hasown-bridge - Seed

## Telos

Close the representation bridge between well-known Symbol keys and cruft's
legacy `@@name` string-key storage for own-property presence checks.
`Object.getOwnPropertyDescriptor(obj, Symbol.toStringTag)` already finds
built-in `@@toStringTag` descriptors, but
`Object.prototype.hasOwnProperty.call(obj, Symbol.toStringTag)` and
`Object.hasOwn(obj, Symbol.toStringTag)` miss them.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs`:
  `object_proto_has_own_property_via`, `object_has_own_via`,
  `Runtime::property_key_of`, and `has_property_pk`.
- Focused probes over `Math`, `JSON`, `Temporal`, and Temporal prototypes
  with `Symbol.toStringTag`.
- Existing `via-method-audit/` locale for ToPropertyKey discipline context.

## Methodology

1. Baseline the descriptor/hasOwn mismatch on built-in objects that store
   `@@toStringTag` as a string key.
2. Mirror the existing `has_property_pk` transitional Symbol-to-`@@` fallback
   at the own-property check boundary.
3. Preserve real Symbol-bucket own properties and string-key shape-aware
   lookup.
4. Verify focused probes plus Temporal exemplars.

## Carve-outs

- This locale does not migrate the object storage model to canonical
  Symbol-key storage.
- This locale does not alter `Object.getOwnPropertyDescriptor`; it is the
  already-correct sibling oracle.
- Proxy own-property invariants are out of scope unless a focused probe shows
  the same bridge missing through Proxy traps.

## Composes-with

- `via-method-audit/` VMA-EXT 1: ToPropertyKey at Object.prototype method
  boundaries.
- `temporal-implementation/temporal-tostringtag-descriptor/`: installs
  spec-correct `@@toStringTag` descriptors that this bridge makes observable
  through hasOwn checks.
- `engine-sentinel-non-enumerable/`: prior `@@` string-key representation
  constraints.

## Resume protocol

Read `trajectory.md` tail, then run the focused `Symbol.toStringTag` hasOwn
probes.
