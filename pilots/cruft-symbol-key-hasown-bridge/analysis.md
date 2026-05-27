# cruft-symbol-key-hasown-bridge - Analysis

This locale is not a diff-prod locale; it is a focused ECMA/test262 bridge
surfaced by Temporal and built-in descriptor tests. The empirical signal is a
direct contradiction between descriptor presence and own-property presence for
the same well-known Symbol key.

| Probe | Baseline | Current |
|---|---|---|
| `Object.getOwnPropertyDescriptor(Math, Symbol.toStringTag)` | descriptor | descriptor |
| `Object.prototype.hasOwnProperty.call(Math, Symbol.toStringTag)` | false | true |
| `Object.hasOwn(Math, Symbol.toStringTag)` | false | true |
| `Object.getOwnPropertyDescriptor(Temporal, Symbol.toStringTag)` | descriptor | descriptor |
| `Object.prototype.hasOwnProperty.call(Temporal, Symbol.toStringTag)` | false | true |
| `Object.hasOwn(Temporal, Symbol.toStringTag)` | false | true |

The owner is `pilots/rusty-js-runtime/derived/src/interp.rs`, specifically
the own-property key lookup for Object has-own methods.

The legible relation is now:

```text
ToPropertyKey(input)
  -> exact own-property lookup
  -> if Symbol, legacy string-slot fallback for well-known-symbol storage
```

That fallback is intentionally narrow. It preserves ordinary Symbol own
properties while making the legacy `@@` storage convention observable through
the same own-property APIs that already observe descriptors.
