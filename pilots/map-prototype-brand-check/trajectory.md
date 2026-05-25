# map-prototype-brand-check — Trajectory

## MPBC-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper directive "Continue and also start sweep". Sibling to SPBC at the Map/WeakMap surface.

**Edits** (~20 LOC):
- `intrinsics.rs` Map/WeakMap ctor block: `brand_chk` closure captured per proto-registration. get/set/has/delete wrapped to throw on receiver-vs-proto mismatch (Map proto rejects WeakMap receiver; WeakMap proto rejects non-WeakMap receiver).

**Verification**: minimal probe GREEN. Exemplar pending sweep-completion (full sweep in flight blocks ~/bin/cruft binary update).

**Status**: code landed locally; verification pending.

## MPBC-EXT 1 — verification (2026-05-25; post-sweep deploy)

**Exemplar** (6 Map brand-check fixtures from pre-sweep baseline):
- PASS: 0 → **4** (the get/set/has/delete cross-proto cases)
- 2 remaining: context-is-weakmap-object-throws (different code path), 1 sundry.

**Regression**: 0 across Map/prototype (existing-passing). WeakSet regressions recovered by sibling SPBC-EXT 3.

### Status

CLOSED at MPBC-EXT 1.
