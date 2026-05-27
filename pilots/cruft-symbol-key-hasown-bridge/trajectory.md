# cruft-symbol-key-hasown-bridge - Trajectory

## SKHB-EXT 0 - Founding baseline (2026-05-27)

**Trigger**: `apparatus/locales/CANDIDATES.md` named a small untapped bridge:
built-ins expose `@@toStringTag` descriptors through
`Object.getOwnPropertyDescriptor`, but own-property presence checks miss the
same key when the caller passes `Symbol.toStringTag`.

**Baseline probe**:

```text
Math desc=true has=false
JSON desc=true has=false
Temporal desc=true has=false
Temporal.PlainTime desc=true has=false
```

**Finding SKHB.1 (descriptor path is the sibling oracle)**:
`Object.getOwnPropertyDescriptor` already resolves the well-known Symbol to
the legacy `@@toStringTag` string-key descriptor. The bug is not descriptor
installation; it is the own-property presence path failing to mirror the
transitional Symbol-to-string-key fallback.

**Status**: SKHB-EXT 0 CLOSED by SKHB-EXT 1.

## SKHB-EXT 1 - Own-property symbol bridge (2026-05-27)

**Change**: Added `Runtime::has_own_property_key` as the shared own-property
presence bridge for string and Symbol property keys. `Object.prototype
.hasOwnProperty` now routes through it, and `Object.hasOwn` now preserves
Symbol keys instead of coercing them through string-only lookup.

**Finding SKHB.2 (presence must mirror descriptor fallback)**:
The transitional `@@toStringTag` storage convention was already visible to
descriptor lookup and to the broader `has_property_pk` path. Own-property
presence needed the same two-step relation: first check the exact property key,
then, for Symbol keys, check the legacy string slot named by the symbol.

**Focused probe after change**:

```text
Math desc=true hop=true hasOwn=true
JSON desc=true hop=true hasOwn=true
Temporal desc=true hop=true hasOwn=true
Temporal.PlainTime desc=true hop=true hasOwn=true
real-symbol hop=true hasOwn=true
```

**Regression sweep**:

```text
Temporal exemplars: PASS=100 FAIL=0 / 100  (100.0%)
```

**Status**: SKHB-EXT 1 CLOSED. The locale remains useful as a reminder that
legacy well-known-symbol storage has to compose consistently across descriptor,
presence, and has-property paths until the object model is fully migrated.
