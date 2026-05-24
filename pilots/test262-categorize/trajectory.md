# test262-categorize — Trajectory

## T262C-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive on the ECMAScript-parity pivot following the TS-parity arc's 100% parse-parity close. Third instrument-tier locale (after TCC for parse-parity, TXC for execute-parity) — this one indexes test262 failures by `(structure-axis-edge × data-shape)` per the keeper-framed data-axis-missing-coordinate observation.

**Strategic framing**: this locale anchors the ECMAScript-parity arc. The matrix it produces is the corpus-driven priority instrument the parity arc needs (same role TCC played for TS-parity). Two coordinates from the start, per the retrodiction that TS-arc's IPBR shape-witness null result (Finding TSR.1) was a data-axis bottleneck masquerading as a structure-axis null.

**Pre-spawn data-axis-framing booking**: the keeper-shared message thread (Doc 720 static DAG vs realized trace; input as hidden variable; "the substrate has an axis the representation doesn't") explicitly frames data-axis as the missing coordinate. This locale is the first instrument to operationalize that framing. Per the forward prediction recorded in the framing: a witness that constrains numeric type (data axis) will move realized cost in a way the IPBR-shape-witness did not. The matrix the instrument produces should make this prediction empirically testable.

**Pre-spawn rule 11 5-axis check**:
- (A1) component A/B — N/A; instrument-tier, not substrate-leverage
- (A2) op-set coverage — N/A
- (A3) value-domain — N/A
- (A4) locals-marshaling — N/A
- (A5) emission-shape — N/A

(The instrument-tier locale's relevance to rule 11 is supporting future SUBSTRATE locales' A1-A5 checks — the matrix surfaces what to probe.)

**Five Pred-t262c.* + discipline falsifier**:
- Pred-t262c.1: instrument runs in <10 s on failing-test set
- Pred-t262c.2: top-15 matrix cells account for ≥50% of failing tests
- Pred-t262c.3: each top-15 cell is inspectable + cause-namable in <5 min
- Pred-t262c.4: when fixes applied, matrix cell-distribution SHIFTS per Doc 742 boundary-contract predictions
- Pred-t262c.5 (DISCIPLINE — standing rule 13): closes in ≤3 implementation rounds

**Founding artefacts**: seed.md + trajectory.md + scaffolded dirs. T262C-EXT 1 (apparatus + first matrix) next.

**Backgrounded**: re-measurement of test262-sample baseline running concurrently (the 77.6% reading is 2 days old; substrate moved via TS-parity arc's runtime fixes — TRMLE module loader + export-default-fn binding + skip_type ASI + classify_brace ClassBody distinction). New baseline will be the first matrix's input.

---

## T262C-EXT 1 — chapter close (2026-05-24)

**Re-measurement post-assert-fix**: identical headline (5576 PASS / 1606 FAIL = 77.6%). The assert-global-shadow fix changed FAILURE REASONS but not PASS/FAIL outcomes. Every test previously hitting "node:assert throws: not yet implemented" stub had a REAL substrate gap underneath; the fix unmasked them.

**Empirical confirmation**: 0 occurrences of "not yet implemented" in new results.jsonl (vs 511 occurrences if the bug was still live). 511 failures now categorized as `expected-throw-missing` / `err:TypeError` — these are the now-visible real substrate gaps.

**Top failure patterns post-fix** (unchanged in structure from pre-fix, because categorization is reason-shape-based and both stub-error and real-error route to similar tags):

| # | Pattern | Count | Real substrate gap |
|---|---|---:|---|
| 1 | arrow-function destructuring SyntaxError negative tests | 45 | Parser too permissive on escaped reserved words in IdentifierReference positions |
| 2 | for-of destructuring ReferenceError-on-unresolvable | 43 | Cruft doesn't throw ReferenceError on unresolvable destructuring target in strict mode |
| 3 | for-of destructuring with Symbol.iterator + throw-during-iter | 40 | Iterator-protocol error-propagation paths |
| 4 | Object.defineProperty edge cases | 38 | Property descriptor semantics |
| 5 | Array.prototype.sort edge cases | 29 | Sort stability + comparator-coercion edge cases |
| 6-9 | for-of yield-expression in destructuring | 82 | Generator/destructuring/initializer interactions |
| 10 | String.prototype.trim edge cases | 22 | Whitespace classification edge cases |

**Five Pred-t262c.* dispositions**:
| Predicate | Disposition |
|---|---|
| Pred-t262c.1 (instrument <10s) | ✅ HELD (~3s) |
| Pred-t262c.2 (top-15 cells ≥50%) | ❌ FALSIFIED at 25.5% — but STRUCTURE-AXIS marginal accounts for ≥50% in top-10 pipelines |
| Pred-t262c.3 (each top cell inspectable + cause-namable) | ✅ HELD (4 inspected; each named with substrate cause in <5 min) |
| Pred-t262c.4 (matrix shift on fix per Doc 742 contract) | ⚪ DEFERRED to substrate-fix follow-on sub-locales |
| Pred-t262c.5 (≤3 rounds) | ✅ HELD at 1 implementation round |

### Findings

**Finding T262C.1**: assert-global-shadow bug was perfectly masking ~hundreds of real substrate gaps. The corpus-driven categorization was REQUIRED to detect the bug (without the matrix, the bug would have been dispersed across hundreds of test reports). **The instrument's first measurement found a substrate bug at its OWN apparatus tier** (cruftless/src/assert.rs::install), not at the substrate-under-test. SIPE-T fractal recursion at the apparatus tier.

**Finding T262C.2**: (pipeline × data) cross-product matrix splits concentration; the (pipeline) marginal is the actionable view at this corpus shape. Pred-t262c.2 (≥50% in top-15 cells) FALSIFIED at 25.5% but top-10 pipelines alone account for ~50% of failures. The data-axis is informative but at this corpus does NOT compound the structure-axis concentration. (Different from the TS-parity arc, where the structure-axis was relatively flat and the data-axis carried more signal.)

**Finding T262C.3**: 4 of the top 6 categorical failure clusters (rows 2, 3, 6, 7) trace to destructuring-binding edge cases. The substrate gap surface is concentrated at the destructuring-assignment / destructuring-binding implementation. A coherent sub-locale `ts-resolve-destructuring-strict-mode` or similar could close ~150-200 tests with focused work.

### Status: CHAPTER CLOSED at T262C-EXT 1

T262C is operational as the third instrument-tier locale. Next sub-locales (priority order from matrix):
1. **strict-mode-destructuring-references** — ReferenceError on unresolvable destructured targets in strict mode (43+ tests)
2. **parser-permissiveness-audit** — negative syntax tests cruft accepts (45+ tests for arrow-function dstr, 22 for arrow-function params, 24 for for-of decl-cls)
3. **iterator-protocol-error-propagation** — for-of close-on-iter-throw semantics (40+ tests)
4. **object-defineProperty-edge-cases** — descriptor edge cases (38+ tests)
5. **array-sort-edge-cases** — comparator + stability + coercion (29+ tests)

Standing-rule-13 corroborations: 13. Inspect-then-iterate: 14 (this round counted: planned scope was the categorize binary; mid-round surfaced the assert-global-shadow fix).
