# array-length-setter-truncation — Trajectory

## ALST-EXT 1 — route `arr.length = N` through ArraySetLength (2026-05-25)

**Trigger**: matrix 2026-05-25 rank 1 (Object.defineProperty no-feature-tag, 37) + ranks 7-10 (Array.prototype.{forEach,reduce,indexOf,map}, 16+16+18+14) all touch the same shape: `arr.length = N` during iteration must truncate. Probe: `var a=[0,1,2,"last"]; a.length=3; a.hasOwnProperty("3")` returned `true` (bug — should be `false`).

**Edits** (~22 LOC):
- `pilots/rusty-js-runtime/derived/src/interp.rs::Runtime::object_set_pk`: pre-match branch when key=="length" and `internal_kind == Array`. Allocates a transient `{value: N}` descriptor object and dispatches to `generated::array_set_length`. Errors silently ignored at this entry (object_set_pk has no error channel; the defineProperty path retains throwing behavior). The deeper §10.4.2.1 algorithm (already proven under the defineProperty entry) handles ToUint32 + truncation loop + non-configurable-element stuck-throw.

**Verification**:
- Probe: `a.length=3; a.has3 = false; a[3] = undefined` ✓
- Exemplar (30 prev-failing forEach/filter/map/reduce/indexOf with "testResult"/"length"/"accessed" in reason): PASS 0 → **9**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42 PASS**

**Findings**

**Finding ALST.1 (predictive-ruleset rule 13 instantiation)**: the §10.4.2.1 ArraySetLength algorithm was already a deeper-layer closure routed only from the defineProperty resolver entry. The assignment resolver entry was the shallow path; promoting it to the same closure (via a transient descriptor allocation) closed 9 tests at ~22 LOC without touching the algorithm itself. This is the canonical revert-then-deeper-layer-closure pattern from the standing rule 13 document: don't write a parallel truncation routine for object_set_pk; route to the existing closure.

**Finding ALST.2 (sub-cluster decomposition)**: the rank-1+8+9+10 cells decompose into:
- length-mutation-truncates-during-iteration (~9 of 30 in exemplar): **closed by ALST-EXT 1**
- inherited-accessor-on-Array.prototype['N'] (~6 of 30): unrelated shared-upstream — Array.prototype's accessor properties not consulted on filter/map/forEach. Distinct substrate locale.
- length-throwing-set-in-strict (deferred ALST-EXT 2): object_set_pk has no throw channel.

**Status**: ALST-EXT 1 CLOSED.
