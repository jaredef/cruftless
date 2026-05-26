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

## VI. The Fielding correspondence — constraint accumulation as induced property

Per keeper directive 2026-05-25 (Telegram 9776): the five-condition
multiplier reads directly against Roy Fielding's constraint-accumulation
thesis (REST dissertation, ch. 5). Fielding derives REST not by naming
what REST *is* but by stacking architectural constraints in sequence —
client-server, statelessness, cacheability, uniform interface, layered
system, code-on-demand — and observing that each added constraint
*induces* an architectural property (visibility, scalability,
reliability, simplicity, extensibility) that the previous configuration
did not have.

The construction is isomorphic to what the five conditions name here:

| Fielding | BBND |
|---|---|
| Architectural style derived by composing constraints | Locale yield derived by composing the five conditions |
| Each constraint, alone, narrows the design space | Each condition, alone, narrows what counts as a "well-shaped cluster locale" |
| Each constraint *induces* a property absent from prior configurations | Each condition *induces* a yield property absent from a non-coordinate-shaped locale |
| The induced property set GROWS with each added constraint | The yield grows multiplicatively as conditions stack |
| REST = the named composition of six constraints | A high-yield cluster locale = the named composition of the five conditions |
| Removing one constraint reduces the induced property class | Removing one condition contracts the multiplier proportionally |

Fielding's central move is that **architectural properties are not
intrinsic to components**; they are *induced* by the constraint stack
the components inhabit. Cruftless's central move (per the apparatus
articulation doc) is that **conformance yield is not intrinsic to
substrate edits**; it is *induced* by the coordinate the edit addresses,
which is itself constituted by the constraint stack the locale inhabits.

The two are the same structural claim at different abstraction levels:

- **Fielding**: architectural property as constraint-induced.
- **BBND finding**: cluster yield as constraint-induced.
- **Standing-rule-13 prospective application**: closure-roundtrip-count
  as constraint-induced (C1-C4 constitute the conditions; their
  conjunction induces ≤3-round closure).
- **Doc 736 capability-passing runtime**: supply-chain-attack-
  impossibility as constraint-induced (no ambient authority + sealed
  handles + no env access ⇒ architecturally impossible attack).
- **Doc 729 resolver-instance pattern**: directive-free downstream
  artifact as constraint-induced (per-tier directive consumption +
  no-residue-carry ⇒ stage-determinism).
- **Standing ruleset itself**: predictive-coverage map as constraint-
  induced (15 rules compose to PREVENT 11 named bug classes per the
  ruleset's §Predictive coverage map; removing any rule contracts the
  prevented bug-class set).

The cruftless apparatus is **a Fielding-style constraint-accumulation
pattern instantiated at the substrate-engineering tier** rather than at
the network-architecture tier. The corpus has been articulating this
implicitly across multiple docs (729, 730, 736, 737, 740, 741, 742); the
explicit Fielding correspondence makes the inductive structure visible.

### The thesis sharpened

State the correspondence as a single proposition:

> **The accumulation-of-constraints thesis** (working title; corpus
> candidate). For any well-shaped engagement substrate, the
> material-yield property of a substrate move is induced by the
> composition of named constraints the move inhabits, not by the move
> itself. Adding a constraint to the move's inhabited stack induces a
> new yield property; removing one contracts the yield-property set.
> Multi-condition substrate work is multiplicatively-yielding because
> the induced-property set is the product of per-condition contributions,
> not the sum.

This generalizes:
- Fielding's REST derivation (constraints → architectural properties).
- Cruftless's standing-rule-13 prospective application (C1-C4 →
  ≤3-round closure).
- BBND's five-condition multiplier (5 conditions → 95-test single-
  locale yield).
- Doc 736 capability-passing (constraint stack → impossibility-class
  property).

The corollary for engagement-discipline:

> **Constraint visibility is yield discipline.** A locale's seed.md
> should name the constraints its inhabited stack imposes; absent that
> naming, the induced-property set cannot be predicted before the work
> lands, and locale-spawn decisions degrade to count-driven heuristics
> (heuristics §III explicitly warns against this — "compare rows only
> inside a partition" is a statement about constraint partitions, not
> count partitions).

### Implication for next-locale-spawn discipline

When considering spawning a locale, enumerate the constraints the locale
will inhabit BEFORE committing the apparatus tax (seed + trajectory +
manifest + agent-feedback). If three or more named constraints from a
shared catalog hold, the locale will yield multiplicatively. If fewer,
land as a nested rung. The catalog for cluster-shaped locales is the
five conditions above; for performance-shaped locales the catalog is
rule 11's five coverage axes; for capability-shaped locales it is Doc
736's authority-composition predicates.

The naming itself is the discipline. A locale that names its constraint
stack at seed time exposes its induced-property set to falsification at
trajectory time — which is what the cybernetic loop requires.

### Corpus candidate

Promote to corpus doc 743 (working title: "The accumulation-of-
constraints thesis: induced-property composition as the substrate-
engineering analog of Fielding's REST derivation"). Locate alongside
Doc 730 (vertical recurrence) and Doc 736 (capability-passing) — both
of which the new doc generalizes. Cross-reference Fielding's REST
dissertation chapter 5 as the original-domain articulation.

Pending keeper review.

---

*Authored under BBND-EXT 2 close, extended under keeper directive 9776
linking the multiplier to Fielding constraint-accumulation. Read this
file before founding the next test262-directory-shaped locale; the
conditions named here predict its yield, and the §VI articulation
predicts the conditions' general form.*
