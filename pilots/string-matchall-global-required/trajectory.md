# string-matchall-global-required — Trajectory

## SMGR-EXT 1 — TypeError on non-global RegExp arg (2026-05-25)

**Trigger**: RES audit-2 Gap E.

**Edits** (~8 LOC at `prototype.rs::String.prototype.matchAll`):
- After resolving the regex argument to `InternalKind::RegExp`, check `re.flags.contains('g')`. If missing, throw TypeError per §22.1.3.13 step 4.

**Verification**:
- Probe: `"abc".matchAll(/a/)` → TypeError ✓
- Probe: `"abc".matchAll(/a/g)` → iterator works ✓

**Status**: SMGR-EXT 1 CLOSED.
