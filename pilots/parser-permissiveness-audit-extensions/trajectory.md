# parser-permissiveness-audit-extensions — Trajectory

## PPAE-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: EPSUA-EXT 2; keeper "Continue with no. 5"; mandatory pre-scoping per EPSUA C4 strengthening narrowed scope from ~50 to 17 (in-scope sub-clusters only).

**Edits** (~70 LOC) — see seed §II.

**Verification**:
- Minimal probes: 3/3 GREEN
- Exemplar (17 sub-cluster tests): PASS 0 → 7 (+7); 10 still-fail in different early-error sub-cases (TDZ, let-in-body, dup-decl-in-head) — not in scope per seed §III.
- Regression check across for-of (495), for-in (79), arrow-function (264): 838 → 838, 0 regressed.

### Findings

**Finding PPAE.1**: scope discipline held — projected 17, delivered 7. **41% of in-scope projection** (better than ICOA's 24% of cluster projection). The sub-cluster split (in-stmt = 7 tests / TDZ + let-in-body + dup = 10 tests) was visible in test filenames pre-implementation; could have been segmented further.

**Finding PPAE.2 (methodology strengthening)**: per-FILENAME segmentation within a sub-cluster is sometimes free. `head-(const|let)-bound-names-{in-stmt,fordecl-tdz,let,dup}` — four distinct early errors, each its own sub-sub-cluster. Per-filename inspection ≤ 1 minute; would have produced precise 7-test scope (in-stmt only) for PPAE-EXT 1.

**Finding PPAE.3 (EPSUA arc-tier)**: third constraint under-delivers vs prospective projection (constraint #5: 7 actual vs ~50 projected = 14%; cumulative-vs-projected for EPSUA so far: 13 actual vs ~113 projected = 12%). Pred-epsua.4 (≥2 sub-locales materialize within projection) now requires #2 OR #1 to deliver ≥80% of projection — looking unlikely given the matrix-aggregation pattern repeats.

**Status**: CLOSED at PPAE-EXT 1.

## PPAE-EXT 2 — arrow-param destructure duplicate-binding check (2026-05-25)

**Trigger**: post-MGOI matrix probe. Cluster #5 (19 arrow-function negative-syntax tests); 9 are `cover-no-duplicates-binding-{array,object,rest}-*` variants exercising duplicate-binding through destructure patterns. PPAE-EXT 1's simple-ident-only check missed these.

**Edit** (~10 LOC):
- `expr.rs::parse_arrow_function` dup-check: replace simple `BindingPattern::Identifier` match with `BindingPattern.collect_names()`. Collects all bound names (simple-ident + destructure-pattern leaves + rest-element binding); duplicate check operates on the full set.

**Verification**:
- Minimal probes: `(x, [x])`, `(x, {x})`, `([x], x)`, `(x, x)` all SyntaxError ✓
- Exemplar (9 cover-no-duplicates fixtures): PASS 0 → 9
- Regression on arrow-function previously-passing (271): 271/271 preserved

**Finding PPAE.4** (extension of Finding PPAE.2): per-test-variant segmentation surfaces at the destructure-vs-simple-ident axis. PPAE-EXT 1's 17 in-scope test estimate didn't include the destructure variants; another 9 close at the same substrate site with a one-line collect_names() extension.

**Status**: PPAE-EXT 2 CLOSED.
