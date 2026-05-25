# set-prototype-brand-check — Trajectory

## SPBC-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper directive "New top level locale as coherent" after RPTP-EXT 2 close. Probe of remaining 226 TypeError-not-thrown failures surfaced Set.prototype brand-check as the most coherent sub-cluster (28 tests, single substrate pattern: brand-check missing at 7 method entry points).

**Edits** (~25 LOC):
- `interp.rs::require_set_brand` helper.
- Boilerplate replacement at 7 set-method `_via` entry points (union, intersection, difference, symmetricDifference, isSubsetOf, isSupersetOf, isDisjointFrom).

**Verification**:
- Minimal repro: GREEN
- Exemplar (28 in-scope brand-check tests): PASS 0 → **14**
- Regression on Set/prototype (256 previously-passing): 0

### Findings

**Finding SPBC.1**: brand-check uniform substitution at 7 entry points; per-sub-spec-section projection 14/28 = 50%. Lower than the 100% pattern seen at ASCD/ACDPD/RPTP because the "in-scope" estimate was too coarse — half the tests use Set-like-class with inherited __set_data, which my brand-check accepts. Spec requires a stricter brand-check (e.g., checking this is a *direct* Set instance, not a subclass).

**Finding SPBC.2 (per-test-variant segmentation)**: even within "brand-check" sub-sub-cluster, two distinct upstream causes:
- `{}` / `Object` receiver (~14, closed by `__set_data` check)
- Set-like-class with inherited __set_data (~14, needs stricter brand)

**Status**: CLOSED at SPBC-EXT 1.

## SPBC-EXT 2 — WeakSet brand discrimination (2026-05-25)

**Trigger**: keeper "Continue as coherent". Same workstream (Set.prototype brand-check) at the next sub-sub-cluster: 7 tests where the receiver is a WeakSet (which shares cruftless's `__set_data` sentinel but is a different spec brand).

**Edits** (~15 LOC):
1. `intrinsics.rs` Set/WeakSet ctor: when `is_weak_proto`, mark instance with `__is_weakset = true` (parallel to existing __is_weakmap).
2. `interp.rs::require_set_brand`: reject if `__is_weakset` is true.
3. `interp.rs::set_this_and_storage`: same WeakSet rejection for basic Set methods (add/has/delete/clear/forEach).

**Verification**:
- SPBC-targeted (28 in-scope brand-check tests): PASS 14 → **19** (+5)
- Regression on Set/prototype (256 previously-passing): 0

### Findings

**Finding SPBC.3**: WeakSet shares `__set_data` sentinel with Set in cruftless's storage design; the brand discrimination requires an explicit __is_weakset marker. Pattern mirrors __is_weakmap from a prior round.

**Remaining 9 fails** in-scope:
- 6 set-methods called-with-object: the OTHER arg validation (Set-like protocol on the arg, not the receiver) — different upstream.
- 2 entries/values WeakSet receiver: registered via inline closures (not set_this_and_storage); separate fix.
- 1 sundry.

**Status**: CLOSED at SPBC-EXT 2.

## SPBC-EXT 3 — carve-back + per-proto wrappers (2026-05-25)

**Trigger**: post-SPBC-EXT-2 sweep found 15 regressions in WeakSet/prototype/{add,delete,has}. Diagnosis: `set_this_and_storage` is shared between Set.prototype and WeakSet.prototype basic-method registrations; the WeakSet-rejection added at SPBC-EXT 2 broke WeakSet's own methods.

**Edits** (~25 LOC):
1. `interp.rs::set_this_and_storage`: remove the WeakSet rejection (relocate to per-proto wrappers per the same pattern Map adopted post-EXT-81).
2. `intrinsics.rs` Set/WeakSet ctor: introduce `set_brand_chk` closure using captured `is_weak_proto`; wrap add/has/delete registrations per-proto. Set proto rejects WeakSet receivers; WeakSet proto rejects non-WeakSet receivers.

**Verification**:
- 15 WeakSet/prototype regressions: all recovered (15/15 now PASS again).
- SPBC's prior gain (Set basic methods rejecting WeakSet receiver via Set.prototype.X.call(weakset)) preserved.
- 0 net change on currently-passing.

### Finding SPBC.4

The shared-impl + per-proto-registration shape is a recurring pattern in cruft (Set/WeakSet sharing set_proto_*, Map/WeakMap sharing map_proto_*). Brand-check discipline requires the wrapper at registration, NOT in the impl (because the impl can't know which proto routed the call). Sibling pattern to MPBC; both stabilized this round.

**Status**: CLOSED at SPBC-EXT 3.
