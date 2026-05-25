# get-own-property-descriptor-accessor-shape — Trajectory

## GOPD-EXT 1 — Option<Value> as field-presence discriminator (2026-05-25)

**Trigger**: bridge-audit follow-on. test262 cluster `desc1.hasOwnProperty('get') !== true` (4 tests) — Object.getOwnPropertyDescriptor returns data-shape descriptor for properties defined as accessors with both `get:undefined, set:undefined`.

**Edits** (~50 LOC across interp.rs):
- `defineProperty` accessor branch: `final_getter = if has_get_key { Some(getter) } else { existing_getter }` — preserves `Value::Undefined` as `Some(Undefined)`. Old conditional collapsed `has_getter` (only-callable) to None, losing the spec-mandated "field is present" bit.
- `find_getter` / `find_getter_pk` / `find_setter` / `find_setter_pk`: filter Some(Undefined) → None for the invocation path. Only callable getters/setters return Some.
- New `is_accessor_at(id, key)` / `is_accessor_at_pk(id, key_pk)`: walks proto chain; returns true if descriptor's getter OR setter field is present (Some, regardless of value).
- Write sites (Op::SetProp, Op::SetIndex): after `find_setter` returns None, check `is_accessor_at` — if accessor descriptor with no callable setter, silent no-op (sloppy) or TypeError (strict) per §10.1.9.4 step 2.b. Pre-fix the write fell through to `object_set` (data write).
- Reflect.set: accessor-with-no-setter returns `false` (no throw, per Reflect convention).
- Promise.catch "then" lookup: add is_callable check before invoking getter (Some(Undefined) shouldn't be invoked).
- Op::CallMethod proto-walk accessor probe (line 7989): only count callable getters in `has_accessor`; matches `find_getter` semantics.

**Verification**:
- Probe: `Object.defineProperty(o, "x", {get:undefined, set:undefined, c:t}); Object.getOwnPropertyDescriptor(o, "x")` → `{get, set, enumerable, configurable}` shape (was `{value, writable, ...}`) ✓
- Probe: regular accessor still invokes ✓
- Probe: redef accessor with new getter ({get: getFunc}) preserves existing setter ✓
- Probe: write to accessor-with-no-setter is silent no-op in sloppy, TypeError in strict ✓
- test262 defineProperty/gOPD cluster (14 prev-fails): **4 newly pass**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding GOPD.1 (Option<Value> semantic carries the discriminator)**: PropertyDescriptor.getter/setter as `Option<Value>` carries the spec-mandated "is field present in descriptor" bit cleanly when `None` means "field absent" (data property) and `Some(_)` means "field present, with this value (callable, undefined, or other)". Pre-fix's conflation of `Some(Value::Object(_))` with "is accessor" lost the explicit-undefined case. The fix is a semantic shift — no struct change required — but downstream consumers split into two camps: invocation paths (find_getter, find_setter) want callable-only; introspection paths (gOPD, is_accessor_at) want field-presence. The fix carves the two query shapes apart.

**Finding GOPD.2 (semantic shifts surface downstream callers)**: an invariant change to a low-level data type (Option semantics on PropertyDescriptor) surfaces every callsite that relied on the old discriminator. Found by panic at Op::CallMethod (unwrap on None from filtered find_getter where the proto-walk's accessor-probe still used old `is_some()` semantics). Standing recommendation echoes RIAS.1: when changing the meaning of a low-level Option, audit callsites BEFORE rather than via panic.

**Status**: GOPD-EXT 1 CLOSED.
