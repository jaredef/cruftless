# global-builtins-non-enumerable — Seed

## Telos

ECMA-262 §17 baseline + §19 (Global Object): all standard built-in properties on the global object (constructors, namespaces, functions, globalThis itself) are `{w:t, e:f, c:t}`. cruft's `install_global_this` uses `rt.object_set(gt, k, v)` which installs at the default `{w:t, e:t, c:t}`, making every built-in enumerable. test262 `built-ins/{Map,Set,WeakMap,WeakSet,Symbol,Promise,Number,JSON,Math,Error}/prop-desc.js` (et al.) fail with "descriptor should not be enumerable".

Identified via EIPD.1 grep-pattern sweep extended to `built-ins/*/prop-desc` and similar.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_global_this` (line 433).

## Methodology

Replace `self.object_set(gt, k, v)` with `install_global_non_enum(rt, gt, k, v)` that uses `dict_mut().insert` with `PropertyDescriptor{w:t, e:f, c:t}`.

Applies to:
- The entries loop (every global from self.globals).
- `globalThis` self-reference.
- `global` Node alias.
- `Intl` namespace install.

## Carve-outs

- User-installed globals via `globalThis.foo = 1`: routes through Op::SetProperty → object_set_pk with default attrs (enumerable). Correct: only built-ins are non-enumerable.
- `install_global_this_refresh` (re-entry during later install passes): fresh `gt` each call; safe.

## Resume protocol

Read `trajectory.md` tail.

---

## Cross-arc disposition (2026-05-28)

Per `apparatus/docs/coverage-gap-orphan-disposition-2026-05-28.md` §II.2/II.3/II.7 + §III.2 lattice-meet repetition pattern: this locale forms part of the **install-helper-convention triplet** with `engine-sentinel-non-enumerable`, `global-builtins-non-enumerable`, and `via-method-audit`. All three share the install-with-descriptor emit-shape at the engine-runtime tier and are lattice-meet candidates per Doc 744 §IV.2. The triplet enrolls into the future `2026-05-28-property-descriptor-bridge` arc when scaffolded; until then, the cross-reference here makes the meet discoverable.
