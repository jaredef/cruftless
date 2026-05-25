# array-create-data-property-discipline — Trajectory

## ACDPD-EXT 0 — workstream founding (2026-05-25)

**Trigger**: keeper directive "Spawn" after ASCD-EXT 1's mid-implementation pivot surfaced this as the sibling-required substrate (Finding ASCD.2 — wiring slice/splice through array_species_create regressed target-array-non-writable; reverted to carve-out pending ACDPD).

**Strategic framing**: this locale is the substrate dual of ASCD. ASCD wired the *constructor-selection* path; ACDPD fixes the *per-element-write* path. The pair together enables spec-correct slice/splice/map/filter species behavior (per §23.1.3.{28,30,32} steps that CreateDataPropertyOrThrow each element).

**Pre-scoping probe**: ~14 target-array-non-writable / non-extensible / non-configurable tests across map/filter/slice/splice/flat/flatMap (per Finding ASCD.3 decomposition).

**Pre-spawn Rule 11 5-axis check**:
- (A1) component A/B: single substrate site (per-element write path); call sites enumerable from grep
- (A2) op-set coverage: object_set vs define_own_property; helpers exist
- (A3) value-domain: descriptor flags (writable/enumerable/configurable)
- (A4) locals-marshaling: N/A (runtime)
- (A5) emission-shape: N/A
- (A6 EPSUA-extended): spec sections enumerated (§7.3.6 CreateDataPropertyOrThrow; §10.1.6 [[DefineOwnProperty]]; §23.1.3.{28,30,32} per-method element-write steps)

**Four Pred-acdpd.* + discipline falsifier** (see seed §I.3).

**Founding artefacts**: seed.md + this trajectory.md + scaffolded dir. ACDPD-EXT 1 (implementation + exemplar + close) next.

### Status

ACDPD-EXT 0 founded. Implementation pending keeper authorization.
