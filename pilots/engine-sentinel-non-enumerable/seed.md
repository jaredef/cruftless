# engine-sentinel-non-enumerable — Seed

## Telos

Per CLAUDE.md's source-identifier coordinate convention, `__name`-prefixed engine-internal sentinels (`__map_data`, `__set_data`, `__is_weakmap`, `__is_weakset`, `__date_ms`, `__cruftless_http_*`, etc.) are documented as "engine-internal sentinel, non-enumerable." In practice, cruft installs them via `rt.object_set` which uses the default descriptor `{w:t, e:f, c:f}` ... wait, the default is `{w:t, e:t, c:t}` (enumerable + configurable). The convention is not enforced at the substrate level; every sentinel leaks through `Object.keys`, `for-in`, JSON.stringify, structured-clone enumeration, and the new console.log inspect formatter.

Identified by CLIF.3 (console-log-inspect-formatter's recommendation to close the broader Object.keys leak rather than just filter at the formatter).

Probe state (post-CLIF-EXT 1):
```
Map      keys: ["__map_data","size"]
Set      keys: ["__set_data","size"]
WeakMap  keys: ["__map_data","size","__is_weakmap"]
WeakSet  keys: ["__set_data","__is_weakset","size"]
Date     keys: ["__date_ms"]
```

The `size` entries are a separate concern (spec says `size` is an accessor on the prototype, not an own data property; substrate increment/decrement code mutates it directly). Deferred to a sibling locale; this rung handles only the `__X` sentinels.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs` — Date setter methods (~10 sites writing `__date_ms`).
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` — Map/Set/WeakMap/WeakSet ctors (~6 sites writing the sentinels at first install).
- `cruftless/src/http.rs` — `__cruftless_http_*` sentinels per HS-EXT 5 + the agent-feedback concern (1).

## Methodology

1. Add `Runtime::set_engine_sentinel(id, name, value)` helper in interp.rs that installs via `dict_mut().insert(PropertyKey::String, PropertyDescriptor{w:t, e:f, c:f})`.
2. Convert FIRST-install sites of the five sentinels above to use it. Subsequent `rt.object_set(id, "__X", new_v)` UPDATE paths preserve existing attrs (object_set_pk's update branch only changes the value), so they remain unchanged.
3. Apply the same helper to `cruftless/src/http.rs` per the rusty-js-http-server agent-feedback concern (1) — closes that follow-on without spawning a separate sub-locale.

## Carve-outs

- `size` as own data property on Map/Set: separate sibling locale (`map-set-size-accessor-only`). Spec wants size as a prototype accessor reading from a hidden slot; substrate currently mutates it on the instance. Different shape.
- `__primitive__` on String/Number/Boolean wrappers: already filtered at most sites (object_keys filters it explicitly); leave alone.
- `@@`-prefixed Symbol storage keys: separate concern (Symbol bucket has its own path).

## Composes-with

- CLIF.3 (this locale's trigger): formatter filters at output; this rung filters at install.
- RIAS-EXT 1 (RegExp instance accessor shadow): same convention applied to RegExp internals; established the dict_mut + explicit descriptor pattern.
- EIPD-EXT 1 / GBNE-EXT 1: same {w:t, e:f, c:t} pattern for Error message/cause/stack and global built-ins. This rung uses {w:t, e:f, c:f} since these are TRULY engine-internal (configurable would allow user delete + reinstall).
- rusty-js-http-server `agent-feedback.md` concern (1): folded in as a one-touch follow-on.

## Resume protocol

Read `trajectory.md` tail; if `agent-feedback.md` is present, read its head + most recent review per apparatus/docs/agent-feedback-schema.md.
