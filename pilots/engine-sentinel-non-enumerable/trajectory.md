# engine-sentinel-non-enumerable — Trajectory

## ESNE-EXT 1 — hide __X sentinels on Map/Set/WeakMap/WeakSet/Date/http (2026-05-25)

**Trigger**: CLIF.3 (console-log-inspect-formatter) recommended a sweep that re-installs every `__X` engine sentinel via `dict_mut + {w:t,e:f,c:f}` per CLAUDE.md source-identifier convention. The console formatter filters `__`-prefixed keys at the OUTPUT layer; this rung closes the leak at the INSTALL layer, so `Object.keys` / `for-in` / `JSON.stringify` / structured-clone enumeration all see clean instances.

The rusty-js-http-server agent-feedback concern (1) named the same pattern for `__cruftless_http_*`; folded into this rung as a one-touch follow-on rather than a separate locale.

**Edits** (~30 LOC):

- `interp.rs`: new `Runtime::set_engine_sentinel(id, name, value)` helper. Inserts via `dict_mut() + PropertyDescriptor { w:t, e:f, c:f }`. Subsequent `object_set` updates preserve attrs (the update branch only mutates value), so call sites that re-write the sentinel (Set.clear, Map.clear, Date setters) need no change.
- First-install sites converted (5 in interp.rs + intrinsics.rs, 4 in cruftless/src/http.rs):
  - `interp.rs::new_empty_set` (`__set_data`)
  - `intrinsics.rs` Map/WeakMap ctor (`__map_data`, `__is_weakmap`)
  - `intrinsics.rs` Set/WeakSet ctor (`__set_data`, `__is_weakset`)
  - `intrinsics.rs` Date ctor + structured_clone Date copy (`__date_ms`)
  - `intrinsics.rs::structured_clone_walk` Map/Set wrappers
  - `http.rs::make_server_object` (`SERVER_SLOT`, `__cruftless_http_bound_addr`, `__cruftless_http_handler`)
  - `http.rs::make_response_object` (`HEADERS_SLOT`, `BODY_SLOT`, `__cruftless_http_ended`)

**Verification**:

| Probe | Before | After |
|---|---|---|
| `Object.keys(new Map([["a",1]]))` | `["__map_data","size"]` | `["size"]` |
| `Object.keys(new Set([1,2]))` | `["__set_data","size"]` | `["size"]` |
| `Object.keys(new WeakMap())` | `["__map_data","size","__is_weakmap"]` | `["size"]` |
| `Object.keys(new WeakSet())` | `["__set_data","__is_weakset","size"]` | `["size"]` |
| `Object.keys(new Date())` | `["__date_ms"]` | `[]` |

All Map/Set/Date/WeakMap methods still functional (`set`/`get`/`has`/`delete`/`clear`/`size`/`setHours`/etc.) — the update path through `object_set` preserves the non-enum/non-config attrs since the entry already exists.

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding ESNE.1 (install-attrs vs update-attrs path)**: cruft's `object_set_pk` update branch preserves descriptor attrs (only mutates `.value`). This means the install-time descriptor sets the floor: once `__X` is installed with `{w:t,e:f,c:f}`, all subsequent `rt.object_set` writes update value while keeping non-enumerable + non-configurable. No need to convert every WRITE site; only every FIRST-INSTALL site. Bug-pattern grep is `rt.object_set(.*, "__[a-z_]+"\.into\(\), ...)` at sites that allocate the receiver.

**Finding ESNE.2 (size remains the separable next rung)**: `size` on Map/Set/WeakMap/WeakSet still leaks because it's stored as own data property (spec wants it as a prototype accessor). Substrate code increments/decrements `this.size` directly. A separate locale (`map-set-size-accessor-only`) would either hide it via `{w:t,e:f,c:f}` (simplest, preserves existing increment code) or refactor to a real accessor reading the storage's property count. Recommend hide-as-non-enumerable first; accessor refactor as a later rung once spec compliance for `Object.getOwnPropertyDescriptor(map, "size")` returning undefined is needed.

**Finding ESNE.3 (sentinel-pattern compounds across host modules)**: the same `__X` install-time pattern lived in three modules (interp.rs Set, intrinsics.rs Map/Set/Date, http.rs). A single Runtime helper closed all three. The pattern would extend cleanly to any future host module that needs engine-internal per-instance state: register the helper as the canonical install path, document the convention in CLAUDE.md (already present), and the grep-pattern stays mechanical.

**Status**: ESNE-EXT 1 CLOSED.

**Closes follow-on**: `pilots/rusty-js-http-server/agent-feedback.md` Review 1 concern (1) (engine-sentinel enumeration leak on HTTP server / response objects). The running summary at the head of that file should be updated to reflect closure on next entry.
