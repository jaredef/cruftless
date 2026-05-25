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

---

## ESNE-EXT 2 — hide `size` on Map/Set/WeakMap/WeakSet instances (2026-05-25)

**Trigger**: ESNE.2 from the prior rung named `size` as the only remaining enumerated leak. Spec wants size as a prototype accessor reading from a hidden slot; substrate currently increments it as an own data property. Minimum-substrate move per Standing Rule 21: hide the own data property via the existing `set_engine_sentinel` helper. The prototype accessor (installed at install_map_and_weakmap) was always falling back to counting `__map_data` properties when no own size existed; with own size still present but non-enumerable, `m.size` reads the own data (matches spec value) while `Object.keys(m)` no longer surfaces it.

**Edits** (~6 LOC):

- `interp.rs::new_empty_set`: install `size` via `set_engine_sentinel(new_set, "size", Value::Number(0.0))` so callers (Set ops) inherit a hidden size.
- `intrinsics.rs` Map/WeakMap ctor: switch ctor `size` install to `set_engine_sentinel`.
- `intrinsics.rs` Set/WeakSet ctor: same.
- `intrinsics.rs::structured_clone_walk` Map + Set wrappers: same.

All subsequent `rt.object_set(id, "size", n)` increment/decrement sites unchanged — the `object_set` update branch preserves attrs (per ESNE.1).

**Verification**:

| Probe | Before | After |
|---|---|---|
| `Object.keys(new Map([["a",1]]))` | `["size"]` | `[]` |
| `Object.keys(new Set([1,2]))` | `["size"]` | `[]` |
| `Object.keys(new WeakMap())` | `["size"]` | `[]` |
| `Object.keys(new WeakSet())` | `["size"]` | `[]` |
| `new Map([["a",1]]).size` | 1 | 1 |
| `new Set([1,2,3]).size` | 3 | 3 |
| `console.log(new Map([["a",1],["b",2]]))` | `Map(2) { a => 1, b => 2 }` | `Map(2) { a => 1, b => 2 }` |
| Set/Map iteration, add/set/delete/clear | works | works |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding ESNE.4 (the existing fallback accessor was always dead code)**: the prototype size accessor at line 3536 was written to fall back to counting `__map_data` storage properties when no own `size` existed. In practice, the own `size` data property was always installed at ctor, so the fallback never fired. The accessor served only as a spec-compliance placeholder for `Object.getOwnPropertyDescriptor(Map.prototype, "size")` — a real concern but separable from the enumeration leak fix. The hidden-data approach this rung uses preserves the existing dead-code path intact; a future rung that wants spec-strict `Object.getOwnPropertyDescriptor(map_instance, "size") === undefined` would need to delete the own data property AND fix every increment site to update via the storage count or through a hidden slot.

**Finding ESNE.5 (carve-back was sized right)**: ESNE.2 originally was tagged "separable next rung" because it looked like it would require refactoring every increment site. The actual fix was 6 LOC because cruft's update-preserves-attrs semantics meant only first-install sites needed conversion. Standing recommendation: when a carve-out is tagged "separable" on size grounds, re-check whether the substrate has invariants that collapse the scope (update-preserves-attrs is the canonical example).

**Status**: ESNE-EXT 2 CLOSED. Map/Set/WeakMap/WeakSet/Date instances now have empty `Object.keys` (matching Node behavior modulo TypedArray indexed-property differences).

---

## ESNE-EXT 3 — hide iterator + generator instance state and methods (2026-05-25)

**Trigger**: probe-for-more-sentinel-leak-sites round identified the iterator family as the dominant residual leak. Pre-fix `Object.keys` on Array/String/Map iterators returned `["_arr","_i","@@toStringTag","@@iterator","next"]`; on Set iterators `["__vals","__idx"]`; on generators `["__gen_arr__","__gen_idx__","next","return","throw"]`. State + methods all enumerable on the instance.

Also caught a convention drift: Array/String/Map iterators used `_arr` / `_i` (single underscore), violating CLAUDE.md's `__X` engine-sentinel naming convention.

**Edits** (~20 LOC across 3 files):

- `iterator.rs::make_array_iterator`: rename `_arr` → `__arr`, `_i` → `__i` to follow the `__X` convention. Switch state installs + `@@toStringTag` install via `set_engine_sentinel` (non-enumerable). The internal `install_self_returning_iterator` + `install_next` helpers also switched to set_engine_sentinel for the `@@iterator` + `next` installs.
- `iterator.rs::make_string_iterator`: post-construction, override `@@toStringTag` to `"String Iterator"` (was inheriting Array Iterator's label from make_array_iterator).
- `intrinsics.rs` Set iterator (set values()/keys()/entries() path): switch `__vals` / `__idx` installs to set_engine_sentinel. The `next` method is already installed via `register_intrinsic_method` (non-enumerable per existing convention) so untouched.
- `interp.rs` generator-construction path: switch `__gen_arr__` / `__gen_idx__` state + `next` / `return` / `throw` / `@@iterator` method installs to set_engine_sentinel.
- `interp.rs` Op::IterInit fast-path (line ~9173): update three reads/writes from `_arr` / `_i` to `__arr` / `__i` to match the rename.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `Object.keys([1,2].values())` | `["_arr","_i","@@toStringTag","@@iterator","next"]` | `[]` |
| `Object.keys([1,2].keys())` | same | `[]` |
| `Object.keys([1,2].entries())` | same | `[]` |
| `Object.keys("abc"[Symbol.iterator]())` | same | `[]` |
| `Object.keys(new Map([['a',1]]).entries())` | same | `[]` |
| `Object.keys(new Set([1]).values())` | `["__vals","__idx"]` | `[]` |
| `Object.keys(gen())` (function* gen) | `["__gen_arr__","__gen_idx__","next","return","throw"]` | `[]` |

Iterator + generator behavior preserved:
- Array .values()/.keys()/.entries() returned via Array.from: correct values
- for-of on Array, String, Set, Map: works
- Generator .next()/.return()/.throw(): works; for-of on generator works
- Map iterator: Array.from(map) returns [[k,v],...]

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Pre-existing bug surfaced (not regression)**: `[..."abc"]` returns `[]` because `__array_extend` → `collect_iterable` requires `Value::Object` and rejects `Value::String`. for-of on string and `Array.from("abc")` work (they go through different paths that ToObject-wrap the string first). Out of scope for this rung; flag for a future locale.

**Findings**

**Finding ESNE.6 (convention drift was load-bearing)**: Array/String/Map iterators used `_arr` / `_i` (single underscore) — a different convention from `__X`. The drift mattered: CLIF-EXT 1's formatter filter (`!k.starts_with("__")`) wouldn't have caught `_arr` had it remained, and any audit grep for `__X` sentinel sites would have missed them. Standing recommendation: at locale entry, when a substrate carries multiple sentinel-naming conventions, the audit move is to converge first (rename to canonical `__X`) before applying the apparatus-tier fix; without convergence the same fix has to be re-applied per-convention.

**Finding ESNE.7 (instance-vs-prototype shape is the next axis)**: iterator instances carry `next` / `@@iterator` / `@@toStringTag` as own properties. Real spec puts them on `%ArrayIteratorPrototype%` / `%StringIteratorPrototype%` / etc., with only state on the instance. Hiding the instance copies non-enumerable (this rung) closes the `Object.keys` leak but leaves a separate spec-shape gap: `Object.getOwnPropertyDescriptor(iter, "next")` returns a desc, but real Node returns undefined (since `next` lives on the prototype). The architectural fix is to introduce `%ArrayIteratorPrototype%` and similar, install methods there, point iterator instances at it via `.proto`. ~150-250 LOC across iterator.rs + intrinsics.rs; separate locale candidate `iterator-prototype-routing`.

**Finding ESNE.8 (probe-for-more found 6 distinct sentinel-source modules)**: the iterator + generator paths spread across 3 files (iterator.rs, intrinsics.rs Set/Map iterator, interp.rs generator). Cumulative locale-scope ~20 LOC. The remaining leak sources identified by probe-for-more are TypedArray / ArrayBuffer / DataView accessor-shadow (parallel to RIAS-EXT 1's RegExp work), AbortController / AbortSignal accessor-shadow, and URL accessor-shadow. Each is a separable sibling locale; each shares the shape of "instance carries own data properties that spec mandates as prototype accessors" (per RIAS-EXT 1's pattern).

**Status**: ESNE-EXT 3 CLOSED. Iterator + generator leak family closed; locale at three rungs covering Map/Set/WeakMap/WeakSet/Date instances + all iterator families + the http-server agent-feedback follow-on.

**Next-rung candidates** (per ESNE.7 + ESNE.8):

1. `iterator-prototype-routing` (~150-250 LOC) — move iterator methods to dedicated prototypes; matches real spec shape.
2. `typedarray-buffer-view-accessor-shadow` — TypedArray + ArrayBuffer + DataView length/byteLength/byteOffset/buffer as prototype accessors (same shape as RIAS-EXT 1).
3. `abort-and-url-accessor-shadow` — AbortController/AbortSignal/URL property surface as prototype accessors.
