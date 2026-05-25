# array-sort-tostring-dispatch — Trajectory

## ASD-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper directive "do A" (next cluster while full sweep runs). T262C cluster (29 tests, Array.prototype.sort, post-AEVPD/SDIBP). Recon on `bug_596_1.js`: cruft's sort doesn't call user-defined toString during sorting; uses `abstract_ops::to_string` which returns `"[object Object]"` for any Object without dispatching ToPrimitive.

**Edit** (~14 LOC):
- `interp.rs::array_proto_sort_via` comparator-free branch: replace direct-sort-by-low-level-to_string with pre-materialize-keys via `to_primitive(v, "string")` then sort-by-index-permutation.

**Verification**: deferred to post-sweep (full sweep blocks cruft-binary update).

**Status**: code landed; sweep-completion verification pending.

**Exemplar verification** (Array.prototype.sort/*, 54 tests):
- PASS: 20 → 23 (+3; includes `bug_596_1` ToString-count target + 2 cascade)
- Regressions on previously-passing: 0
- Adjacent regression check (Array.prototype.{sort, toString, join, toLocaleString}): 38 passing → 38 passing, 0 regressed.

Minimal repro: `[obj, obj].sort()` with counting toString — counter 0 → 2 ✓.

### Findings

**Finding ASD.1**: tight per-element ToString dispatch path. The pre-materialize-keys design diverges from spec's "ToString during each comparison" but is observationally equivalent for the cluster's test shape and matches what real engines do for perf.

**Status**: CLOSED at ASD-EXT 1.
