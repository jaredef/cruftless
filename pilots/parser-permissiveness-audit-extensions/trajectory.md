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

## PPAE-EXT 3 — for-in/of head BoundNames-dup check (2026-05-25)

**Trigger**: matrix pre-scope per Standing Rule 17. `head-(const|let)-bound-names-dup.js` tests check that `for (const [x, x] of [])` is SyntaxError per §14.7.1.2 BoundNames-must-be-unique. cruft accepted.

**Edit** (~12 LOC):
- `compiler.rs::Stmt::ForOf` + `Stmt::ForIn`: insert a HashSet-based dup check on head's collect_names() result before the existing PPAE-EXT 1 head-vs-body conflict check.

**Verification**:
- Minimal probes: `for (const [x, x] of [])`, `for (let [x, x] of [])`, `for (const {x, y: {x}} of [])` all SyntaxError ✓
- Exemplar (2 head-bound-names-dup fixtures): PASS 0 → 2
- Regression on for-in/of previously-passing (603): 603/603 preserved

**Status**: PPAE-EXT 3 CLOSED.

## PPAE-EXT 4 — arrow-param ReservedWord check + is_unconditional_reserved_word split (2026-05-25)

**Trigger**: post-PPAE-EXT 3 matrix scan; `arrowparameters-bindingidentifier-identifier-futurereservedword.js` (`var af = enum => 1`) should be SyntaxError per §11.6.2 (enum is unconditional FutureReservedWord). cruft accepted. Initial attempt using broad `is_reserved_word` regressed the noStrict `(yield) => 1` test because is_reserved_word folded strict-only reserveds into the always-reserved set.

**Edits** (~22 LOC):
- `parser.rs`: new `is_unconditional_reserved_word(name)` covering ECMA-262 §11.6.2 Keyword set EXCLUDING the strict-mode-only / context-only reserveds (yield, let, static, implements, interface, package, private, protected, public, await). The broad `is_reserved_word` retained for sites that want both sets (object-binding shorthand, etc.).
- `expr.rs::parse_arrow_function`: switch from is_reserved_word to is_unconditional_reserved_word. Mode-gated reserveds (yield/let/await) defer to SMPT-EXT 1's deferred strict-mode tracking.

**Verification**:
- `var af = enum => 1` → SyntaxError ✓
- `var af = (yield) => 1; af(1)` → 1 (noStrict valid) ✓
- Regression on arrow-function previously-passing (271): 271/271 preserved
- In-scope futurereservedword: 1/2 pass (the unconditional-enum case; the strict-only-yield case needs SMPT-EXT 2 strict-mode tracking)

**Finding PPAE.5**: the is_reserved_word / is_unconditional_reserved_word split is the substrate-level surfacing of the strict-mode vs unconditional distinction. Per Standing Rule 18 (brand-check at the registration wrapper, not in shared impl), the predicate's two halves correspond to two distinct contexts; checks at each call site must use the right half. SMPT-EXT 2 (full strict-mode parser tracking) would unify these contexts.

**Status**: PPAE-EXT 4 CLOSED.
