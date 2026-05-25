# error-instance-property-descriptors — Trajectory

## EIPD-EXT 1 — non-enumerable message/cause/stack + drop per-instance name (2026-05-25)

**Trigger**: RES audit-2 follow-on sweep. Error instances surfaced `["message","name","stack"]` enumerable own properties; test262 `built-ins/Error/{message_property, cause_property, prop-desc}` failed with "descriptor should not be enumerable".

**Edits** (~25 LOC at `intrinsics.rs::install_error_globals` ctor body):
- Helper `install_non_enum(rt, id, k, v)` inlines `dict_mut().insert` with PropertyDescriptor `{w:t, e:f, c:t}`.
- `message` install: use install_non_enum (also: skip install when arg is Undefined — spec mandates this; prototype's `message=""` serves the default).
- `cause` install: use install_non_enum.
- `stack` install: use install_non_enum.
- DROP the per-instance `name` install. `Error.prototype.name` is already set non-enumerable at line ~4836 (`set_own_internal`), and each NativeError sets its own proto name. Subclass override (`class E extends Error { constructor(m){super(m); this.name="E"}}`) still works because user's `this.name=` assignment goes through Op::SetProperty → own data property.
- RPTC.7 bug-pattern cleanup: `abstract_ops::to_string(msg)` → `rt.coerce_to_string(msg)?` (msg may be an Object with toString).

**Verification**:
- Probe: `Object.keys(new Error("hi", {cause:42}))` → `[]` (was `["message","name","stack"]` + maybe `cause`) ✓
- Probe: `Object.getOwnPropertyDescriptor(e, "message")` → `{value:"hi", w:t, e:f, c:t}` ✓
- Probe: `Object.getOwnPropertyDescriptor(e, "name")` → undefined (lives on proto) ✓
- Probe: `e.name` → "Error" (via proto) ✓; `(new TypeError("x")).name` → "TypeError" ✓
- Probe: subclass `class E extends Error { constructor(m){super(m); this.name="E"} }` → ee.name = "E" ✓
- test262 Error/message,cause,stack,prop-desc (9 in scope): 5 → **7**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding EIPD.1 (default rt.object_set produces silent spec-divergence)**: every `rt.object_set(id, name, v)` for a property that the spec mandates a non-default descriptor for is a silent observable-shape bug. The bug surfaces only when test262 (or user code via Object.keys/JSON.stringify/for-in) probes the descriptor. Bug pattern is grep-detectable: `rt.object_set(id, "<spec-non-enum-name>"...)`. Candidates for sweep across the substrate: `message`, `cause`, `stack`, `__map_data`, `__set_data`, `__date_ms`, TypedArray `length`/`byteLength`/`byteOffset`, etc.

**Predicts**: a hygiene sweep with explicit per-property descriptor enforcement closes a long-tail cluster of "descriptor should not be enumerable" failures. Standing recommendation: at substrate-property-install sites, default to explicit PropertyDescriptor literals rather than rt.object_set's silent defaults.

**Finding EIPD.2 (per-instance shadows of per-prototype defaults)**: the per-instance `name` install was a shadow of `Error.prototype.name`. Pre-fix, every instance had its own `name` (correct value, wrong descriptor); the proto's `name` was never consulted. The shadow was free — but the descriptor mismatch surfaced. Pattern echoes RIAS-EXT 1 (RegExp instance shadow of prototype accessors). Standing recommendation: when a constructor installs per-instance the same value the prototype already carries, drop the install — let prototype serve.

**Status**: EIPD-EXT 1 CLOSED.
