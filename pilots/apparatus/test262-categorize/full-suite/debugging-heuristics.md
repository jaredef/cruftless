# Full-Suite Debugging Heuristics

This document explains how to read the full-suite Pin-Art matrix after FULL-EXT 3's lattice refinement.

The matrix is not a leaderboard of failures. It is a projection of Test262 non-pass records onto the cruftless language-behavior DAG:

```text
test path + result reason
  -> surface
  -> resolver instance
  -> engine tier
  -> engine rung
  -> constraint axis
  -> availability class
  -> cut kind
  -> projection class
  -> failure mode
  -> abstract-operation candidate
  -> Pin-Art coordinate
```

The debugging affordance is compression. A full-suite run with tens of thousands of non-pass records collapses into a few hundred coordinates, and each coordinate is an address in the engine's conformance lattice.

## I. Why the Matrix Is Legible

The matrix is legible because three independent coordinate systems partially align:

- Test262 paths name semantic surfaces: `built-ins/ArrayBuffer`, `language/statements/class`, `intl402/Temporal`, `built-ins/RegExp`.
- Failure reasons name local symptoms: wrong value, missing throw, wrong throw type, missing intrinsic, parse rejection, runner no-output, missing `$262`.
- Cruftless names owning resolver instances: parser, AST-to-bytecode lowering, runtime spec builtins, object internals, regexp, typed arrays, Intl/host, promise/job queue.

FULL-EXT 3 adds the corpus superstructure:

- The **DAG** is the dependency object: surface behavior routes through resolver instances, internal methods, intrinsics, execution contexts, realms, and host hooks.
- The **lattice** is the cut reading: each failure has a rung, availability state, cut kind, and projection.
- The **alphabet** is the stable vocabulary of cut kinds and failure classes visible at this resolution.

The useful question is therefore not "which file failed?" but "which files share a lattice address?"

## II. The Current Coordinate

The primary Pin-Art coordinate is:

```text
resolver :: engine rung :: projection :: failure mode
```

The adjacent fields are not decoration. They decide whether rows are comparable.

| Field | Meaning | Use |
|---|---|---|
| `resolver` | Owning engine resolver instance | Where to begin reading code |
| `rung` | Doc 717-style engine rung | Whether the issue is syntax, internal method, intrinsic, jobs, realm, or host surface |
| `axis` | Doc 729-style constraint axis | Which strategic spectrum the row belongs to |
| `availability` | Whether the surface exists, is partial, absent, policy-deferred, or runner-deferred | Whether this is bug work, chapter work, or apparatus work |
| `cut_kind` | Doc 716-style cut reading | Whether the move is widening, successor refinement, throw-on-use, policy, or measurement residue |
| `projection` | Local symptom class | What kind of substrate rule is likely missing |
| `failure_mode` | Observed failure form | How the test exits |
| `abstract_op` | First-pass ECMA operation candidate | Which spec path to inspect first |

Rows with different `availability` or `cut_kind` should not be ranked against each other. A 4,000-count absent chapter and a 600-count available-surface wrong-result row are different work types.

## III. The First Partition

Always partition before choosing a target.

| Partition | Signals | Action |
|---|---|---|
| Available-surface bug | `availability=available-surface`, cut is widening or successor, projection is value/throw/descriptor/iterator/realm | Inspect examples and find the shared abstract op or resolver rule |
| Absent chapter | `availability=absent-chapter`, often Temporal or large ECMA-402 surface | Spawn/resume subsystem locale; begin with inventory and skeleton |
| Partial surface | `availability=partial` or `partial-chapter` | Determine whether the missing piece is method wiring, data, or semantics |
| Policy/version lane | `policy-deferred` or `policy-or-partial` | Decide policy before coding; Annex B/staging should not silently drive core semantics |
| Measurement residue | `runner-deferred`, `runner/no-output`, or `measurement-residue` | Fix runner/harness apparatus before interpreting as engine conformance |

This is the central rule: compare rows only inside a partition.

## IV. Work Shapes

### A. Implement-Chapter

Signals:

- `availability=absent-chapter`
- `cut_kind=K1/throw-on-use`
- Surfaces include Temporal, broad Intl/ECMA-402, SharedArrayBuffer/Atomics where the subsystem is not present, or Unicode-set RegExp
- Examples point at absent globals, constructors, prototypes, or whole method families

Action:

- Create or resume a subsystem locale.
- Inventory constructor/prototype/static-method surface first.
- Land a minimal skeleton only if it reduces runner ambiguity without pretending semantic parity.
- Follow with semantic slices and re-measure.

Do not mix implement-chapter rows with available-surface bug rows.

### B. Shared-Upstream Bug

Signals:

- `availability=available-surface`
- `cut_kind=widening/value-semantics`, `widening/abrupt-completion`, or `successor/semantic-refinement`
- Same `abstract_op` or `projection` appears across multiple surfaces
- Examples are heterogeneous by path but homogeneous by mechanism

Action:

- Pull at least five records with the same `pin`.
- Read their Test262 frontmatter and assertions.
- Identify the shared abstract operation or compiler convention.
- Patch at the highest shared resolver point, then re-run targeted tests and the matrix.

This is the highest-yield parity lane.

### C. Parser / Early-Error Cluster

Signals:

- `resolver=source-to-ast/parser-early-error`
- `rung=E1/algorithm-step:syntactic-grammar`
- `projection=parser-form/early-error`
- Paths include `invalid`, `syntax`, `early`, redeclaration, reserved words, binding identifiers, class elements, or module forms

Action:

- Prefer parser-only or parser-plus-AST-flag fixes.
- Build a small witness list from examples.
- Run targeted negative and positive tests.
- Watch for permissiveness regressions.

Parser rows are often clean because their lattice cut is shallow and the expected result is binary: accept or reject.

### D. Runtime Intrinsic Semantics

Signals:

- `rung=E3/intrinsic-object:ecma-262`
- `availability=available-surface` or `absent-or-partial-surface`
- Projections include wrong result, missing method, descriptor shape, iterator protocol, species constructor, prototype chain, or wrong throw

Action:

- Inspect the surface marginal to pick a coherent family.
- Use `abstract_op` to decide whether the fix belongs in a shared abstract op or the specific intrinsic.
- Prefer shared abstract-op fixes when examples cross surfaces.

This lane includes typed arrays, object internals, collection intrinsics, array exotica, regexp, and spec builtins.

### E. Measurement Residue

Signals:

- `availability=runner-deferred`
- `cut_kind=measurement-residue`
- `projection=runner-harness/$262-or-host-hook`
- `failure_mode=runner/no-output`
- Raw result repair reports malformed fragments or split records

Action:

- Fix harness injection, host hooks, JSON escaping, no-output capture, or sidecar runner discipline.
- Keep these rows out of engine parity accounting until the apparatus can observe them cleanly.

FULL-EXT 3 routes `$262` / host-hook failures here so they no longer masquerade as missing ECMA globals.

## V. Row-Coherence Protocol

A row is actionable when several examples share one mechanism.

Use this command shape:

```bash
rg '"pin":"<pin text>"' \
  pilots/test262-categorize/full-suite/results/test262-full-2026-05-25-165734-p2/interpreted.jsonl \
  | head -5
```

Then inspect:

1. `availability`: is this engine work, chapter work, policy work, or runner work?
2. `cut_kind`: is the move widening, successor refinement, throw-on-use, or measurement residue?
3. `abstract_op`: do the examples share a spec operation?
4. `surface`: are the examples one family or many families?
5. `reason`: are they failing the same way?

If five examples share one mechanism, the row can spawn a substrate move. If they do not, split by surface, reason signature, or abstract-op candidate before coding.

## VI. Marginal Reading Order

Read the matrix in this order:

1. **Availability marginal**: decide which universe of work is being considered.
2. **Cut-kind marginal**: decide whether the move is implementation, widening, successor refinement, policy, or apparatus.
3. **Engine-rung marginal**: locate the depth of the cut.
4. **Constraint-axis marginal**: choose the strategic spectrum.
5. **Resolver marginal**: choose the owning implementation area.
6. **Pin-Art coordinate marginal**: choose the candidate row.
7. **Surface marginal**: choose witness fixtures.
8. **Abstract-operation marginal**: choose the spec path to read.

This order prevents the largest chapter or harness row from dominating the next move by raw count.

## VII. Current Strategic Reading

After FULL-EXT 3, the first full run reads as:

| Signal | Reading |
|---|---|
| `R/ast-to-bytecode` is the top bug-shaped axis | Language lowering remains a prime parity lane |
| Temporal is isolated as `E3/intrinsic-object:temporal` | It is implement-chapter mass, not an Intl classifier accident |
| ECMA-402 remains a distinct host-facing lane | Intl work is real but subsystem-shaped |
| `runner-deferred` and `$262` failures are visible | Harness residue can be fixed without confusing engine semantics |
| `available-surface` remains large | There is substantial tractable parity work before choosing giant chapter builds |
| `(unmapped)` abstract-op mass remains high | The next apparatus refinement is better spec-operation mapping |

The clean next spectra are:

1. `R/ast-to-bytecode` available-surface rows.
2. Parser early-error rows.
3. TypedArray / ArrayBuffer available-surface rows.
4. Runner-harness measurement residue.
5. Abstract-op mapping refinement.

Temporal and broad Intl should be chosen deliberately as subsystem work, not accidentally because they dominate raw counts.

## VIII. Debugging Rule

The next substrate move should satisfy:

```text
large enough to matter
  + coherent across examples
  + comparable within one availability class
  + owned by one resolver instance or one shared abstract op
  + not measurement residue unless apparatus hardening is the goal
  + measurable by matrix shift after landing
```

If any term is missing, refine the row before editing code.

## IX. Apparatus Gaps

FULL-EXT 3 closed the original major gaps: projection overload, availability absence, and harness conflation. The remaining gaps are sharper:

- `abstract_op=(unmapped)` is still too large.
- `uncategorized/resolver` still hides Annex B, staging, and harness/policy distinctions that need more path rules.
- `failure_mode=failure/other` still contains parseable runner reason structure.
- `availability=absent-or-partial-surface` should split into missing global, missing constructor, missing static, missing prototype method, and missing internal slot.
- Realm and execution-context rungs need stronger recognition from path metadata and frontmatter features.

The apparatus is doing its job when its own largest rows name the next refinement.
