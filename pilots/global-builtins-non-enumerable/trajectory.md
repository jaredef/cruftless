# global-builtins-non-enumerable — Trajectory

## GBNE-EXT 1 — install built-in globals with {w:t, e:f, c:t} (2026-05-25)

**Trigger**: EIPD.1 grep-pattern sweep extended to `built-ins/*/prop-desc` revealed a 14-test cluster all with "descriptor should not be enumerable" failures — every standard built-in (Map, Set, WeakMap, WeakSet, Symbol, Promise, Number, JSON, Math, Error, parseInt, parseFloat, ...) installed enumerable on globalThis.

**Edits** (~25 LOC at `intrinsics.rs::install_global_this`):
- Replace the entries loop's `self.object_set(gt, k, v)` with `dict_mut().insert(PropertyKey::String(k), PropertyDescriptor{w:t, e:f, c:t})`.
- Same for `globalThis` self-reference and `global` Node alias.

ECMA-262 §17 baseline: all standard built-in properties on the global object are non-enumerable, writable, configurable. User-installed globals (`globalThis.foo = 1`) continue to route through Op::SetProperty → enumerable default per CreateDataPropertyOrThrow.

**Verification**:
- Probe: `Object.getOwnPropertyDescriptor(globalThis, "Map")` → `{value:func, w:t, e:f, c:t}` ✓ (was `{e:t}`)
- Probe: `globalThis.userFoo = 1; desc(globalThis, "userFoo")` → enumerable (user assignment unchanged) ✓
- Probe: `globalThis`, `Intl` both non-enumerable ✓
- test262 "descriptor should not be enumerable" prev-fails (14): **12 newly pass**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding GBNE.1 (single-site fix yields a 14-test cluster)**: when the bug pattern is in a shared install-site (here `install_global_this`'s entries loop), one substrate move closes the entire cluster. 25 LOC for 12 tests is the highest yield-per-LOC of this session's 16-move arc. Standing recommendation per RES.3 (bridge-audit cumulative): periodic grep for `object_set(.*, "<spec-non-enum-name>"...)` against the §17 baseline-attrs list is high-yield substrate hygiene.

**Finding GBNE.2 (single-flag spec invariant covers a wide surface)**: §17's "Every other data property described in clauses 19 through 28 and in Annex B.2 has the attributes {w:t, e:f, c:t}" is a spec-level invariant that covers most of the global object's built-in surface. A substrate that gets this attribute set right at the install site is correct-by-construction for the entire built-in surface; one that gets it wrong fails one test per surface. The asymmetry favors enforcing the invariant once, not per-name.

**Status**: GBNE-EXT 1 CLOSED.
