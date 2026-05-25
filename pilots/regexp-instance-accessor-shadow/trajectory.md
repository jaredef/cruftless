# regexp-instance-accessor-shadow — Trajectory

## RIAS-EXT 1 — remove instance accessor shadows + fix lastIndex descriptor (2026-05-25)

**Trigger**: RES audit-2 (Gaps B+C+D).

**Edits** (~35 LOC):
- `regexp.rs::install_regexp_proto_accessor`: read directly from `InternalKind::RegExp(internals)` via per-name dispatch (source/flags from `re.source`/`re.flags`; boolean flags via `re.flags.contains(c)`). Removes dependency on `rt.object_get(this, name)` (which would now find the prototype accessor itself post-shadow-removal).
- `regexp.rs::new_regexp`: delete the 9 `rt.object_set` calls for source/flags/global/.../hasIndices. Install lastIndex via explicit PropertyDescriptor `{w:t, e:f, c:f}` per §22.2.5.1.
- `intrinsics.rs::structured_clone_walk` (RIAS-EXT 1 follow-up): RegExp branch was probing `source`/`flags` via `rt.object_get` — Undefined post-shadow-removal (proto's accessor has `value=Undefined` and `object_get` doesn't invoke getters). Switch to direct `InternalKind::RegExp` detection.

**Verification**:
- Probe: `Object.keys(/x/g)` → `[]` (was 10 keys) ✓
- Probe: `Object.getOwnPropertyDescriptor(r, 'global')` → `undefined` (was own data prop) ✓
- Probe: `Object.getOwnPropertyDescriptor(r, 'lastIndex')` → `{value:0, writable:true, enumerable:false, configurable:false}` ✓
- Probe: `RegExp.prototype.source.get.call({})` → TypeError ✓
- Probe: r.global / r.source / r.flags all still read correctly ✓
- Probe: lastIndex mutation still works ✓
- test262 `RegExp/prototype/` 50-sample: 47/50 (residuals = pre-existing engine gaps)
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42** (after structured_clone follow-up fix)

**Findings**

**Finding RIAS.1 (shadow-removal exposes downstream object_get dependencies)**: pre-existing code that read RegExp properties via `rt.object_get(id, "source")` worked because of the shadow. Removing the shadow surfaces every such site — structured_clone was one. Standing recommendation: bridge-fixes that move state from own-data to behind-accessors must audit `rt.object_get` callers for the moved property names. Grep candidates: `rt.object_get(.*, "source")`, `rt.object_get(.*, "flags")` across the substrate.

**Finding RIAS.2 (accessor-getter as canonical, object_get for raw)**: `rt.object_get` returns descriptor `.value` without invoking getters. For accessor-backed properties, the substrate must EITHER invoke the getter explicitly OR read the underlying internal state directly. In substrate code, the direct-from-internal_kind path is faster and more correct than re-entering the JS getter dispatch.

**Status**: RIAS-EXT 1 CLOSED.
