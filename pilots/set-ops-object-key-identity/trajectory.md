# set-ops-object-key-identity — Trajectory

## SOKI-EXT 1 — switch Set ops to map_storage_key (2026-05-25)

**Trigger**: FIPC.1 documented-deviation audit applied to abstract_ops::to_string call sites. Probe: `new Set([a,b]).intersection(new Set([a]))` returned size=0 instead of 1 — all Objects collapsed to `"[object Object]"` storage key in the op, while `__set_data` storage used identity-preserving `__objkey@<oid>` per `map_storage_key`. Set.add and Set.intersection used different key derivations — silent divergence.

**Edits** (~8 lines via targeted sed at `interp.rs`):
- Set.prototype.{union, intersection, difference, symmetricDifference, isSubsetOf, isSupersetOf, isDisjointFrom}_via: 8 call sites switched from `abstract_ops::to_string(&v).as_str().to_string()` to `Self::map_storage_key(&v)`. Now consistent with Set.add/has/delete which already use map_storage_key.
- Three sites accidentally caught by the sed (Date.parse, Array.prototype.join, one Runtime helper) reverted: Date.parse and join now use the dispatching `self.coerce_to_string(v)?` (better than the original static path); the helper restored to original.

**Verification**:
- Probe: `new Set([a,b]).intersection(new Set([a]))` → size=1 ✓ (was 0)
- Probe: `new Set([a,b]).difference(new Set([a]))` → size=1 ✓ (was 2)
- test262 `Set/prototype/{union,intersection,difference,symmetricDifference,is*Of,isDisjointFrom}` (186 tests): **116 pass**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding SOKI.1 (within-type discipline-drift across method surfaces)**: Set.add and Set.intersection — both methods on the same type, both deriving storage keys — used different key-derivation helpers (`map_storage_key` vs `abstract_ops::to_string`). The drift was silent until Object values exercised the divergence. Standing Rule 20 (cross-module discipline-drift) instantiation, scoped to within a single type. Standing recommendation: shared substrate helpers (here `map_storage_key`) must be the canonical entry for all consumers of the same logical operation; per-method ad-hoc derivations are the bug pattern.

**Finding SOKI.2 (sed-sweep collateral risk)**: bulk pattern substitution caught 3 unrelated sites (Date.parse, Array.join, one helper) that matched the pattern but had different semantics. Required manual revert. Standing recommendation: targeted-edit patterns (sed across a file) need a follow-up audit of every match; per-site editing is safer for cross-cutting bug patterns when the pattern's syntactic match outpaces its semantic match.

**Status**: SOKI-EXT 1 CLOSED.
