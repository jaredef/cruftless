# CMig-EXT 0 — Consumer-Site Survey

*Catalogs the ~41 direct `.properties` consumer sites across the runtime crate, classifies each into one of three migration patterns, and orders them by family for CMig-EXT 1+ execution. No code change.*

**Survey snapshot**: 2026-05-23, commit `5a8a0ce7` (post-Shape-EXT 4 close).

## Three migration patterns (recap from seed §I)

- **P1 — Shape-iterate then properties-iterate**: enumeration sites that must see every own-property. Read shape slots first (insertion order), then dictionary entries.
- **P2 — Migrate-on-access**: install paths that introduce non-Shape-eligible descriptors (accessors, non-default attrs). Call `migrate_to_dictionary()` first.
- **P3 — Migrate-on-construct**: container objects whose role is BE-a-dictionary (Map storage, Set storage, listener lists). Allocate Ordinary then immediately migrate, or via the new `Object::new_dictionary()` constructor (CMig-EXT 1).

## Inventory (by file, count of direct `.properties` access)

| file | sites | dominant family |
|---|---:|---|
| interp.rs | 47 | mixed: enumeration + descriptor installs + Set/Map iteration |
| intrinsics.rs | 19 | mostly descriptor installs (accessors + frozen built-ins) |
| value.rs | 12 | internal to Object impl (the shape-aware shims themselves) |
| module.rs | 4 | namespace enumeration |
| iterator.rs | 4 | result-object construction `{value, done}` |
| prototype.rs | 3 | prototype-method installs with specific descriptors |
| regexp.rs | 2 | match-object construction |
| promise.rs | 1 | toStringTag install |
| napi.rs | 1 | host bridge |

Total non-value.rs sites: **81** (count is direct .properties access including reads; the seed estimate of "~41" counted distinct call-sites not method invocations).

Most sites cluster into one of five families; the rest are one-offs.

## Family A — Map/Set internal-storage iteration

**Pattern**: cruft's Map and Set instances carry a separate Object as their internal storage (`__map_data` / `__set_data`). Map.prototype.keys / values / entries / forEach iterate the storage's `.properties` directly. Set.prototype likewise. The storage object today is allocated via `Object::new_ordinary()` — if it carries shape, the iteration sees nothing.

**Sites** (interp.rs):
- `:1156` `set_proto_union_via` — `self.obj(s).properties.iter()` over Set storage
- `:1182` `set_proto_intersection_via` — same
- `:1204` `set_proto_difference_via` — same
- `:1226` `set_proto_symmetric_difference_via` — same
- `:1252` `set_proto_is_subset_of_via` — `self.obj(s).properties.keys()`
- `:1356` Set membership read — `self.obj(storage).properties.values()`
- `:2613` `map_proto_entries_via` — `self.obj(storage).properties.iter()`
- `:2625` Map values — `self.obj(storage).properties.iter()`
- `:2652` Map keys — `self.obj(storage).properties.iter()`

**Migration choice**: P3 (migrate-on-construct). Map/Set storage is a container by role; never benefits from shape. The Map/Set ctors allocate storage via `Object::new_ordinary()` — switch to a new `Object::new_dictionary()` factory (CMig-EXT 1) that returns an Ordinary already migrated to Dictionary. No iteration-site changes required.

**Scope**: 9 read sites at iteration; 4 alloc sites at ctors. All sites unchanged after CMig-EXT 1 because the storage's shape is None and the existing IndexMap path works.

## Family B — Enumeration helpers (Object.keys / values / entries / for-in / JSON)

**Pattern**: ECMA §10.1.11 OrdinaryOwnPropertyKeys + downstream consumers walk an object's own enumerable string keys in insertion order. Cruft's helpers walk `o.properties.iter()`.

**Sites**:
- `interp.rs:1969` (in `enumerable_own_property_keys` or similar) — `o.properties.iter()`
- `interp.rs:2053` (likely Object.values) — `o.properties.iter()`
- `interp.rs:2088` (Object.fromEntries?) — `o.properties.iter()`
- `interp.rs:4901` (Object.getOwnPropertyNames) — `o.properties.iter()` with numeric-index sort
- `interp.rs:4913` Object.getOwnPropertyNames cont. — `o.properties.keys()`
- `interp.rs:4938` Object.getOwnPropertySymbols — `o.properties.keys()` filter Symbol
- `interp.rs:5300` (Reflect.ownKeys?) — `o.properties.keys()`
- `interp.rs:5590` `ordinary_own_enumerable_string_keys` — **already shape-aware per Shape-EXT 4**

**Migration choice**: P1 (shape-iterate then properties-iterate). Each site builds a key/value list and must include shape entries in insertion order before dictionary entries.

**Scope**: 7 sites needing P1; ordinary_own_enumerable_string_keys is the model (already done).

## Family C — Direct accessor / non-default-descriptor installs

**Pattern**: intrinsics-construction sites install accessors (getter/setter) or non-default descriptors (frozen, non-writable) via `o.properties.insert(PropertyKey, PropertyDescriptor { ..., getter: Some(...), ... })`. These bypass `set_own*` and would create cohabitation conflicts if the object is Shaped.

**Sites** (mostly in intrinsics.rs and prototype.rs):
- `intrinsics.rs`: ~15 sites installing accessor properties (Map.prototype.size accessor, Set.prototype.size, Object.prototype.__proto__, etc.).
- `prototype.rs:80` `gen_fn_proto` — install `.prototype` with specific descriptor.
- `prototype.rs:96` `async_gen_fn_proto` — same.
- `prototype.rs:877` `register_method` core helper — installs with internal-default descriptor.
- `regexp.rs:1057` / `:1096` — regex helper installs.
- `iterator.rs:109-124` — install `{value, done}` on iterator result objects.
- `promise.rs:111` Promise.prototype `@@toStringTag` install.
- `napi.rs` — host bridge install.

**Migration choice**: P2 (migrate-on-access). Each site adds `self.obj_mut(target).migrate_to_dictionary();` before the `.properties.insert(...)` call. After migration the install proceeds against Dictionary as today.

**Scope**: ~20 sites. Mechanical edit; each gets a one-line prepend.

**Optimization note for CMig-EXT 2**: a wrapper helper `properties_insert_with_migrate(rt, id, key, desc)` would let us centralize the migration call. Optional; not required for correctness.

## Family D — Descriptor introspection

**Pattern**: Object.getOwnPropertyDescriptor, Object.freeze, Object.isFrozen, Object.isSealed, Object.preventExtensions read `o.properties.values()` to inspect descriptor attributes (writable / enumerable / configurable / accessor presence).

**Sites**:
- `interp.rs:5427` (likely Object.freeze) — `o.properties.values_mut()` to flip configurable: false.
- `interp.rs:5440` (Object.seal?) — same shape.
- `interp.rs:5505` (Object.isFrozen) — `o.properties.values().all(|d| !d.writable && !d.configurable)`.
- `interp.rs:5517` (Object.isSealed) — `o.properties.values().all(|d| !d.configurable)`.
- `interp.rs:1989` `object_get_own_property_descriptor_via` — checks both shape (synthesized descriptor) and properties.
- `interp.rs:2033` `object_get_own_property_descriptors_via` — needs both.

**Migration choice**: hybrid. The read-only introspection paths (Object.isFrozen, Object.isSealed, getOwnPropertyDescriptor) can SYNTHESIZE a user-default descriptor for shape-stored entries — Shape-EXT 4 already documented this pattern in Object::get_own's docstring. The mutating paths (Object.freeze, Object.seal) need P2 migrate-first because they want to flip descriptor attributes that don't apply to shape's user-default invariant.

**Scope**: 4 mutating sites (P2); 2 read sites (synthesize default descriptor for shape entries).

## Family E — Module namespace enumeration

**Sites**:
- `module.rs:1119` namespace enumeration — `self.obj(namespace).properties.iter()`
- `module.rs:1144` namespace key count — `self.obj(namespace).properties.len()`
- `module.rs:1439` placeholder key count — `self.obj(placeholder).properties.len()`
- `module.rs:1484` module pairs — `self.obj(*oid).properties.iter()`

**Migration choice**: ModuleNamespace objects are not Ordinary (`InternalKind::ModuleNamespace` per value.rs:387). They never get Shaped per the carve-out (shape only Ordinary). These sites are correct as-is; no migration required.

**Scope**: zero migration needed. Document for completeness.

## Family F — Direct array-index installs

**Sites**:
- `interp.rs:1739` / `:1783` / `:1795` / `:1852` — Array spread / destructure pattern initialization.
- `interp.rs:1926` — Array spread continuation.
- `interp.rs:2126` / `:2144` / `:2160` — Object literal `{0: v0, 1: v1, ...}` initialization for spread targets.
- `interp.rs:6231` — `object_set_pk` insert path (already shape-aware via §6210 dispatch).
- `interp.rs:6092` / `:6122` / `:6187` — `object_get_pk` and related read paths.

**Migration choice**: Arrays bypass shape per carve-out (new_array starts shape=None). Array-related sites are correct. The Object-literal `{0:v0,...}` sites at :2126/2144/2160 install via direct `properties.insert` and warrant P2 migrate-first IF the receiver could be Shaped — but `Op::SetProp` semantics for integer-keyed properties should route through `object_set_pk` which now dispatches correctly.

**Scope**: minimal. Review during CMig-EXT 7 for any residual sites that bypass dispatch.

## Migration call-graph and CMig ordering

```
CMig-EXT 1  →  Object::new_dictionary() factory
                |
CMig-EXT 2  →  Family C (~20 sites)         [P2 migrate-on-access]
                                              [independent; can run in parallel with CMig-EXT 3]
CMig-EXT 3  →  Family A (Map/Set storage)   [P3 migrate-on-construct via CMig-EXT 1's factory]
                                              [depends on CMig-EXT 1]
CMig-EXT 4  →  Family B (enumeration)       [P1 shape-iterate]
                                              [independent]
CMig-EXT 5  →  Family D (introspection)     [hybrid: synthesize for reads, P2 for mutations]
                                              [independent]
CMig-EXT 6  →  (Family E reviewed; zero changes)
CMig-EXT 7  →  Family F residual review
CMig-EXT 8  →  Enrollment flip in Object::new_ordinary()
                                              [DEPENDS on CMig-EXT 1-7 complete + green gates]
CMig-EXT 9  →  Pred-shape.4 first integration measurement
```

CMig-EXTs 2, 4, 5 are independent and can be reordered; current ordering is biased toward landing low-risk-of-regression families first.

## Risk register

- **R1 — Family C might have hidden non-default-descriptor installs not caught by `.properties.insert`**. Mitigation: grep for `PropertyDescriptor { ... getter:` / `... setter:` / `... writable: false` / `... enumerable: false` directly to catch literal-construction sites.
- **R2 — CMig-EXT 8's enrollment flip might surface a consumer site the survey missed.** Mitigation: incremental — flip enrollment in test-only mode first via `CRUFTLESS_SHAPE_ENROLL=1` env flag, run diff-prod + test262-sample, identify regressions, classify per Doc 730 §XVI, fix and re-test. Permanent flip lands only when all gates hold.
- **R3 — Pred-shape.4's 80% target might be missed if user-code idioms construct objects that immediately trigger migration (delete-then-add patterns, accessor installs).** Mitigation: measure actual enrollment rate on the diff-prod corpus; if below 80%, document the dominant migration triggers and decide whether to lift the carve-out for any of them (e.g., support shape-with-accessor descriptors as a future closure round).

## Forward to CMig-EXT 1

CMig-EXT 1 lands `Object::new_dictionary()` — a one-line factory that returns `Object::new_ordinary()` with `shape: None` (already the current default), or, post-CMig-EXT 8 when default flips to Shaped, returns an Ordinary explicitly held to Dictionary form.

LOC estimate for CMig-EXT 1: ~10 (the factory + a doc comment).

---

*CMig-EXT 0 closes. The consumer-site landscape is mapped; the migration plan is ordered; CMig-EXT 1 begins when keeper directs.*
