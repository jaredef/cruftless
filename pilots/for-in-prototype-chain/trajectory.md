# for-in-prototype-chain — Trajectory

## FIPC-EXT 1 — proto-chain walk in __for_in_keys (2026-05-25)

**Trigger**: GOPD-EXT 1 probe surfaced for-in returning own-only keys on `Object.create(proto)` with inherited enumerable on proto. test262 `verifyEnumerable !== true` cluster (6 in sample) hit by "Inherited property is enumerable (Boolean/Number/... instance)" tests defining accessors on primitive prototypes.

**Edits** (~40 LOC at `intrinsics.rs::__for_in_keys`):
- Coerce receiver via `to_object` for primitives.
- Walk the proto chain: at each level, yield own enumerable string keys via the existing `ordinary_own_enumerable_string_keys` helper. Use a HashSet<String> `seen` for dedup.
- Shadow record: after yielding own enumerable, also add own non-enumerable string names to `seen` so a deeper-proto enumerable entry with the same name is excluded (own non-enum shadows inherited enum per §14.7.5.6).
- Engine-internal sentinels (`__primitive__`, `@@`-prefixed) filtered (consistent with object_keys).

**Verification**:
- Probe: `Object.create({x:1}); for-in` → `["x"]` ✓ (was `[]`)
- Probe: `Object.create({x,y}); o.x="own"; defineProperty(o,"z",{enumerable:false}); p.z="proto-z"; for-in` → `["x","y"]` ✓ (own non-enum z shadows proto z)
- Probe: `new Boolean() with defineProperty(Boolean.prototype,"prop",{enumerable:true})` → for-in finds "prop" ✓
- Probe: `for-in {}` → `[]` (Object.prototype methods are non-enumerable) ✓
- test262 verifyEnumerable cluster (6 in sample): **6 newly pass**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding FIPC.1 (documented spec deviation closes when audit lands)**: the for-in proto-chain skip was documented at `compiler.rs:1837` as a "Spec deviations: Own enumerable string keys only (no proto-chain walk)". Doc'd deviations are tractable substrate gaps masquerading as deliberate carve-outs; the audit-then-close pattern surfaces them. Standing recommendation: periodically grep for "Spec deviation" / "Out of scope" / "deferred" doc comments and audit whether they should be promoted to closed substrate.

**Finding FIPC.2 (shared helper compounds)**: this is the third consumer of `ordinary_own_enumerable_string_keys` (Object.keys, Object.entries, now for-in proto-walk). The helper centralizes the §7.3.22 ordering + `__X`/`@@` sentinel filters. Adding the proto-walk at the consumer site (not in the helper) keeps the helper single-purpose; the proto-walk policy (yield enumerable, shadow non-enumerable) lives where the spec says — at the consumer.

**Status**: FIPC-EXT 1 CLOSED.
