# BBND — Findings (yield analysis)

This document reasons about why this locale closed at extreme yield-per-LOC
compared to today's earlier parser-arc closures, and articulates the
structural conditions under which that yield repeats.

---

## I. The yield comparison

| Locale | LOC landed | Closed tests | Tests / LOC |
|---|---:|---:|---:|
| FHAPV-EXT 1 | ~10 | +16 | 1.6 |
| FORA-EXT 1 | ~5 | +3 | 0.6 |
| SBAP-EXT 1 | ~50 | +5 | 0.1 |
| FHLA-EXT 1 | ~10 | +3 | 0.3 |
| FAOF-EXT 1 | ~5 | +1 | 0.2 |
| ALTA+RPDF-EXT 1 | ~15 | +2 | 0.13 |
| ARTC-EXT 1 | ~25 (incl AST) | +2 | 0.08 |
| **BBND-EXT 1+2** | ~140 | **+95** | **0.68 in raw, ~7x in "tests per locale"** |

The raw tests/LOC ratio is comparable. The dominant comparison axis is
not LOC efficiency but **tests-per-locale-instance**. BBND closed an
entire test262 sub-directory of 95 tests inside one nested locale; the
earlier closures averaged ~5 tests per locale. The amortization of the
locale-spawn overhead (seed + trajectory + manifest + commit) is ~19x
better here.

This document reasons why, and names the structural conditions that
reproduce the property.

---

## II. The five amplifying conditions

The cluster yielded extreme tests-per-locale because **five conditions
all held simultaneously**. When fewer of them hold, the multiplier
contracts proportionally.

### Condition 1 — Spec-rule-to-directory bijection

The test262 sub-directory `language/block-scope/syntax/redeclaration/`
corresponds to exactly one spec production rule (§13.2.1 Block static-
semantics early errors). The TC39 conformance team organized the
directory to enumerate the rule's full surface: 95 generated tests cover
the 8×8 declaration-kind cross product (let / const / var / function /
async-function / generator / async-generator / class) under several
positional shapes (same-block, inner-block-with, inner-block-after,
function-scope-var).

When a directory bijects to a single spec rule, the directory's count is
the rule's full enumerated surface. Implementing the rule closes the
directory.

By contrast, today's earlier closures (FHAPV/FORA/SBAP/FHLA/FAOF/ALTA/
RPDF/ARTC) each addressed one production-level point — a grammar
discrimination, a lookahead constraint, a single early-error edge. Each
production-level point fires across only a few test bodies because
test262 doesn't enumerate Cartesian products around production-level
edges (only around rule-level enumerations).

### Condition 2 — Cross-product test generation by the test262 author

The redeclaration directory's tests are generated procedurally
(`flags: [generated]`). The author enumerated all combinations and let
the harness fan them out into per-file fixtures. Generated cross-
products amplify spec-rule yield by a multiplier equal to the product
of the enumerated axes. Here: 8 decl-kinds × several positional
shapes ≈ 95 fixtures.

For non-generated tests, each fixture is a hand-written probe, and the
yield per spec-rule is a small constant (1-5).

### Condition 3 — One-rule-one-site implementation

§13.2.1 fires at ONE syntactic site: the Block production's static-
semantics phase. The rule does not require coordination across multiple
parser sites, AST node kinds, or tier boundaries. The substrate move is
a single check inserted at one parse function (`parse_block_statement`).

Earlier closures often required per-site application of the same rule
(SBAP-EXT 1's check_pattern_binding_ids walks; SBEA-EXT 1's three call-
site insertions; FHLA-EXT 1's two-site fast-path/expression-head pair).
Multi-site rules cost more LOC per test closed.

### Condition 4 — Apparatus tier above the production

The BBND check operates at the **static-semantics** tier, not the
**grammar production** tier. Static-semantics rules have cross-cutting
reach over multiple syntactic surfaces: the same §13.2.1 rule applies
when the block contains `let/let`, `const/let`, `class/var`,
`function/var` in non-strict, et cetera. Implementing the rule once
captures all these inputs.

Earlier closures operated at the production tier — one grammar
discrimination, one specific edge. Production-tier closures have narrow
reach by construction.

The DAG-projection apparatus surfaces this distinction:
- Production-tier coordinates: high-resolution but narrow.
- Static-semantics-tier coordinates: lower-resolution but cross-cutting.

Yield-per-LOC is dominated by the coordinate's *reach*, not by the
edit's size. Static-semantics rules have higher reach by default.

### Condition 5 — The full-suite Pin-Art matrix surfaced the coordinate

Today's earlier closures were driven by inspecting the curated 45-test
SyntaxError cluster sample and chasing individual test bodies. BBND was
driven by reading the canonical full-suite Pin-Art matrix
(`pilots/test262-categorize/full-suite/.../matrix.md`) and the PEER
exemplar suite's family marginal (heuristics §V row-coherence: 3/3
inspected fails share `block-scope/syntax/redeclaration/`).

The matrix names coordinates; the heuristics §V protocol identifies
coherent rule-shaped clusters before any code is edited. When the matrix
is the input, the resulting locale targets a coordinate, not a probe.
Coordinate-shaped work captures full cluster surfaces; probe-shaped work
captures probe-adjacent surfaces.

---

## III. The compound effect

Each of the five conditions is individually a 2-5x multiplier on tests-
per-locale. They compose:

```
tests-per-locale ≈
   (spec-rule reach) × (cross-product enumeration) ×
   (single-site implementation) × (static-semantics tier) ×
   (matrix-driven coordinate targeting)
```

When all five hold (as in BBND): ~95 tests per locale.
When 3-4 hold (as in today's larger parser closures like FHAPV): ~10-16
tests per locale.
When 1-2 hold (as in narrow production-edge closures like FAOF): 1-3
tests per locale.

This is not specific to parser-tier work. The same multiplier structure
applies to any substrate tier whose corresponding test262 directory has
a 1:1 spec-rule mapping. Other examples (predicted, not yet measured):
- `built-ins/Temporal/Duration/compare/*` — single rule, cross-product
  of comparison-shape × argument-kind. Spawning the constructor itself
  closes the entire family at once via Condition 1+2+4.
- `built-ins/RegExp/CharacterClassEscapes/*` — one §22.2 rule, cross-
  product over escape kind. High amortization.

---

## IV. The structural prediction

A cluster-shaped locale should be spawned when:

1. The test262 sub-directory bijects to a single spec rule (Condition 1).
2. The fixtures are generated and enumerate a cross-product (Condition 2).
3. The rule can be implemented at a single substrate site (Condition 3).
4. The rule sits at static-semantics tier or above, OR
   at a single intrinsic-availability gate (Conditions 4 or its
   subsystem analog).
5. The matrix or row-coherence inspection identified the rule as the
   shared mechanism across multiple sampled fails (Condition 5).

If all five hold, expect ≥50 tests per locale. If 3-4 hold, expect
10-30. Below 3, prefer a nested rung inside an existing locale rather
than a fresh locale spawn — the locale-spawn overhead (apparatus,
manifest, agent-feedback) won't amortize.

This is the **cluster-coherence multiplier**: a generalization of
Finding BBND.2's narrower formulation. It is offered here as a standing
prediction for future locale-spawn discipline; the empirical track will
accrue as subsequent top-10 batch locales are worked.

---

## V. Connection to standing apparatus

The five conditions composed don't fall out of any single existing
standing rule directly. They are inferable from several together:

- **R4** (no half-landed moves) — when conditions 1+3 hold, half-
  landing is impossible because the rule applies whole.
- **R11 axis A2** (op-set coverage) — Condition 4 generalizes the
  "right tier" insight to "right static-semantics rule, not right
  production edge."
- **R13** (revert-then-deeper-layer) — Condition 4's tier-above-
  production reading is rule 13 applied prospectively: the deeper layer
  is the static-semantics rule, not the syntactic production.
- **R15** (chapter-close-inspect) — BBND-EXT 1 surfaced BBND-EXT 2 via
  exactly this rule's mid-round failure-table inspection.
- **Heuristics §IV.B** (Shared-Upstream Bug) — Condition 4's static-
  semantics tier is the corpus articulation of the "highest shared
  resolver point" the heuristic prescribes.

Considering naming the conditions as standing rule 16 (cluster-
coherence multiplier) — but the empirical evidence base is still one
locale (BBND). The top-10 batch's other locales will validate or
falsify the prediction; rule promotion waits for ≥3 corroborations per
the rule-13 prospective-application discipline.

---

*Authored under BBND-EXT 2 close. Read this file before founding the
next test262-directory-shaped locale; the conditions named here predict
its yield.*
