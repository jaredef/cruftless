---
name: temporal-tostringtag-descriptor
description: Cross-cutting fix to make @@toStringTag property descriptors spec-conformant ({w:f, e:f, c:t}) across every Temporal class prototype + namespace.
type: project
---

# temporal-tostringtag-descriptor — Seed

## Cross-cutting sub-locale under `pilots/temporal-implementation/`.

Per Finding PTCF.2: the `@@toStringTag should be an own property` residual recurred across Duration / Instant / PlainTime ctor-fields rungs. Spawned as a single rung that fixes the descriptor shape at the foundation.

## Telos

`Object.getOwnPropertyDescriptor(Temporal.X.prototype, Symbol.toStringTag)` must return `{value: "Temporal.X", writable: false, enumerable: false, configurable: true}` per spec §11.x.5. cruft's `set_own_frozen` helper emits `{w:f, e:f, c:f}` (the constants-table convention). The fix: install via `dict_mut().insert` with explicit `PropertyDescriptor { writable: false, enumerable: false, configurable: true, ... }`.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — 6 sites where `set_own_frozen("@@toStringTag", ...)` is called (Temporal namespace, Temporal.Now, the 5 class stubs, Duration proto, Instant proto, PlainTime proto). Replace each with `dict_mut().insert` + correct descriptor.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/Duration/prototype/toStringTag/` (the others were already in).
- **Exemplar suite**: 3+ prop-desc tests across the three Temporal class prototypes.

## TTSTD-EXT 1 — descriptor fix (LANDED)

Replaced 6 `set_own_frozen` calls with `dict_mut().insert(PropertyDescriptor { ..., configurable: true })`. Descriptor verified spec-correct via `Object.getOwnPropertyDescriptor`.

## TTSTD residuals — blocked by symbol-key hasOwnProperty bridge

Spec-correct descriptor is in place, but the prop-desc tests still fail. Investigation: `Object.prototype.hasOwnProperty.call(proto, "@@toStringTag")` returns `true`; `Object.prototype.hasOwnProperty.call(proto, Symbol.toStringTag)` returns `false`. cruft's symbol-key path does NOT bridge `Symbol.toStringTag` to the literal `"@@toStringTag"` string-key the prototype stores under. Object.getOwnPropertyDescriptor DOES bridge (probes return the right descriptor for both keys), but hasOwnProperty does not. propertyHelper.verifyProperty uses hasOwnProperty internally, so the test fails despite the descriptor being correct.

**Standing recommendation**: spawn `cruft-symbol-key-hasown-bridge` follow-on locale to fix `Object.prototype.hasOwnProperty` for symbol-key lookups against `@@`-string-stored properties. Wider impact than Temporal — Math, JSON, every other namespace using `@@toStringTag` shares this bug.

## Status

TTSTD-EXT 1 LANDED 2026-05-26. Descriptor shape spec-correct across 6 Temporal sites; net yield 0 because deeper symbol-key bridge bug blocks the prop-desc tests. Yield will materialize once cruft-symbol-key-hasown-bridge lands.
