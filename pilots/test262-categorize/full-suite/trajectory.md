# test262-categorize/full-suite — Trajectory

## FULL-EXT 0 — founding (2026-05-25)

**Trigger**: keeper directive after the full upstream Test262 run completed at parallelism 2. The raw sidecar run produced 53,289 suite paths and revealed two apparatus facts: the engine result needs coordinate interpretation, and the runner JSONL surface itself has split/malformed/no-output records that must be separated from engine failures.

**Founding move**: create a nested Pin-Art apparatus under the parent `test262-categorize` locale.

**Scope**:

- Consume sidecar full-suite JSONL.
- Robustly classify result records even when raw failure reasons contain line breaks.
- Emit engine-coordinate matrices keyed by resolver instance, surface, projection, failure mode, and Pin-Art coordinate.
- Keep raw full results outside the repo.

**Standing command**:

```bash
cargo run --release -p test262-categorize --bin t262-full-pinart -- \
  /home/jaredef/Developer/cruftless-sidecar/results/test262-full-2026-05-25-165734-p2/results.jsonl
```

**Next**: FULL-EXT 1 runs the apparatus on the sidecar result and books the first interpretation.

---

## FULL-EXT 1 — first full-suite interpretation (2026-05-25)

**Input**: `/home/jaredef/Developer/cruftless-sidecar/results/test262-full-2026-05-25-165734-p2/results.jsonl`

**Command**:

```bash
cargo run --release -p test262-categorize --bin t262-full-pinart -- \
  /home/jaredef/Developer/cruftless-sidecar/results/test262-full-2026-05-25-165734-p2/results.jsonl
```

**Artifacts**:

- `results/test262-full-2026-05-25-165734-p2/summary.md`
- `results/test262-full-2026-05-25-165734-p2/matrix.md`
- `results/test262-full-2026-05-25-165734-p2/interpreted.jsonl`

### Raw-result reconciliation

The sidecar run's physical-line summary reported 53,309 emitted lines with 23,180 PASS / 23,500 FAIL / 6,341 SKIP / 247 NO_OUTPUT / 41 malformed lines. The Pin-Art interpreter joins split JSON fragments before parsing. That recovers malformed failure records whose `reason` strings contained raw newlines/quotes.

Parsed record summary:

| Status | Count |
|---|---:|
| PASS | 23,180 |
| FAIL | 23,520 |
| SKIP | 6,341 |
| NO_OUTPUT | 247 |
| malformed fragments | 1 |

Runnable pass rate after parse repair: **49.6%** (23,180 / 46,700).

This confirms Pred-full.3's importance: runner/apparatus surface must be separated from engine semantics. The raw runner's JSONL escaping remains a measurement-substrate fix candidate; the Pin-Art interpreter can still recover enough structure for analysis.

### First engine-coordinate reading

The first matrix produced:

| Axis | Distinct | Top row |
|---|---:|---|
| Pin-Art coordinates | 142 | `runtime/spec-builtins :: uncategorized/projection :: err:ReferenceError-like` (4,459) |
| Resolver instances | 12 | `runtime/spec-builtins` (6,400) |
| Surfaces | 785 | `intl402.Temporal` (2,029) |
| Projections | 7 | `uncategorized/projection` (12,213) |

Top resolver-instance marginal:

| Rank | Resolver | Count |
|---:|---|---:|
| 1 | `runtime/spec-builtins` | 6,400 |
| 2 | `ast-to-bytecode/language-lowering` | 6,246 |
| 3 | `host-intrinsic/intl402` | 3,045 |
| 4 | `runtime/buffer-typed-array` | 2,733 |
| 5 | `uncategorized/resolver` | 1,972 |
| 6 | `source-to-ast/parser-early-error` | 982 |
| 7 | `runtime/regexp` | 865 |

### Findings

**Finding FULL.1 — full-suite interpretation needs a runner-surface lane.** The full run contains `NO_OUTPUT` plus split JSON failure reasons. Those are not ECMA conformance failures. They are measurement-substrate residues and should route to a runner-hardening coordinate.

**Finding FULL.2 — path-only surface ranking is dominated by large unimplemented spec chapters.** `intl402.Temporal`, `Temporal.*`, staging, and Annex B surfaces dominate several marginals. This is true information, but it is not yet the highest-yield substrate work if the next goal is tractable parity progress. The apparatus needs a "chapter availability" dimension so implement-chapter surfaces and bug-shaped surfaces do not compete in one flat rank.

**Finding FULL.3 — `uncategorized/projection` is now the main apparatus gap.** 12,213 interpreted non-pass records land in `uncategorized/projection`. The first interpreter is sufficient for a broad heat map but not yet sufficient for trajectory selection. FULL-EXT 2 should refine projection classes using source metadata + reason signatures: missing global/intrinsic, unsupported syntax, wrong descriptor, wrong throw type, wrong return value, runner surface, skipped feature/chapter.

**Finding FULL.4 — resolver marginal is immediately useful.** Even with projection coarseness, the resolver-instance marginal names the top of the engine spectrum: runtime spec-builtins and AST-to-bytecode language lowering are nearly tied, with host Intl/Temporal and buffer/typed-array as separate large blocks. This satisfies Pred-full.2 at first cut.

### Predicate dispositions

| Predicate | Disposition |
|---|---|
| Pred-full.1 | HELD. Compile + run completed in seconds after build. |
| Pred-full.2 | HELD at resolver marginal: top two resolver instances account for 12,646 interpreted non-pass records. |
| Pred-full.3 | HELD. `NO_OUTPUT` + malformed fragments are separated from engine coordinates. |
| Pred-full.4 | PARTIAL. Top coordinates route to engine tiers, but too many remain `uncategorized/projection`; FULL-EXT 2 refines. |

**Status**: FULL-EXT 1 closes. Apparatus exists and produces the first engine-coordinate view. Next coherent move is projection refinement, not substrate fix selection yet.

---

## FULL-EXT 2 — debugging heuristics articulation (2026-05-25)

**Trigger**: keeper observation that the matrix appears extremely legible, with directive to reason about this legibility.

**Artifact**: `debugging-heuristics.md`

**Core articulation**: the matrix is legible because Test262's semantic path layout, cruftless's resolver-instance architecture, and the runner's failure reasons form three mutually reinforcing coordinate systems. Joining them collapses 23,768 interpreted non-pass records into 142 Pin-Art coordinates, with 12 resolver-instance marginals and 785 surface marginals. That compression is the debugging affordance.

**Heuristics added**:

1. Choose by work shape, not largest row.
2. Filter implement-chapter mass before choosing a bug-shaped target.
3. Inspect row coherence across at least five examples before coding.
4. Use resolver marginals for strategy, surface marginals for examples, Pin-Art coordinates for substrate moves.
5. Treat runner/no-output and malformed JSONL as measurement-substrate residue.

**Strategic conclusion**: next coherent move is either projection refinement or a deliberately chosen spectrum such as AST-to-bytecode language lowering, parser early errors, TypedArray/ArrayBuffer continuation, or runner hardening. Temporal/Intl should be treated as implement-chapter work rather than competing directly with bug-shaped rows.

---

## FULL-EXT 3 — corpus-informed lattice refinement (2026-05-25)

**Trigger**: keeper directive to refine `full_pinart.rs` after reading the corpus documents on alphabet / DAG / lattice.

**Code artifact**: `derived/src/bin/full_pinart.rs`

**Refinement**: the interpreter now emits corpus-faithful lattice fields instead of relying on the original coarse `resolver :: projection :: failure_mode` coordinate alone.

New per-record dimensions:

- `rung`: Doc 717-style engine rung, including parser algorithm step, execution-semantics internal method, ECMA-262 intrinsic object, ECMA-402 intrinsic object, Temporal chapter, execution-context jobs, object internals, and realm.
- `axis`: Doc 729-style constraint axis, including `R/ast-to-bytecode`, `R/parser-form`, `N/namespace-object-surface`, `S/symbol-identity`, `M/module-resolution`, `O/operator-semantics`, `H/host-builtins-ecma402`, and runtime semantics.
- `availability`: surface-state partition: available surface, absent chapter, absent-or-partial surface, partial chapter, policy-deferred, policy-or-partial, and runner-deferred.
- `cut_kind`: Doc 716-style cut reading: `K1/throw-on-use`, widening value semantics, widening abrupt completion, successor semantic refinement, version/policy cut, parser successor, and measurement residue.
- `abstract_op`: first-pass abstract-operation candidate such as `SameValue`, `GetMethod`, `SpeciesConstructor`, `GetIterator`, `OrdinaryDefineOwnProperty`, `ToObject`, `ToString`, or `RuntimeSemantics/Evaluation`.

**Runner separation**: `$262` / host-hook failures are now routed to `runner-harness/$262-or-host-hook` with `availability=runner-deferred` and `cut_kind=measurement-residue`. This prevents harness incompleteness from masquerading as missing ECMA surface.

**Temporal separation**: built-ins Temporal now lands at `E3/intrinsic-object:temporal` and `E/eval-runtime-semantics:temporal-chapter` rather than being folded into ECMA-402. Intl/Temporal interaction under `intl402/Temporal` remains ECMA-402-host-facing.

**Fresh interpretation result**:

| Axis | Distinct after FULL-EXT 3 |
|---|---:|
| Pin-Art coordinates | 246 |
| Resolver instances | 12 |
| Engine rungs | 9 |
| Constraint axes | 9 |
| Availability classes | 8 |
| Cut kinds | 7 |
| Surfaces | 785 |

**Reading**: the coordinate count increased because the interpreter split formerly overloaded buckets into true lattice coordinates. This is desirable. The matrix is now more navigable: implement-chapter mass, harness residue, bug-shaped available-surface work, and policy/version cuts no longer compete inside one flat rank.

---

## FULL-EXT 4 — debugging heuristics reformalization (2026-05-25)

**Trigger**: keeper directive to reformalize the debugging heuristics document after the FULL-EXT 3 classifier refinement.

**Artifact**: `debugging-heuristics.md`

**Reformalization**: the heuristics document now treats the matrix as a language-behavior DAG/lattice address space rather than a coarse ranked failure list. It updates the interpretation chain to include `rung`, `axis`, `availability`, `cut_kind`, and `abstract_op`, and it makes comparability explicit: rows should be ranked only inside the same availability and cut-kind class.

**Operational rules added**:

- Partition before choosing a target: available-surface bug, absent chapter, partial surface, policy/version lane, or measurement residue.
- Read marginals in order: availability, cut kind, engine rung, constraint axis, resolver, Pin-Art coordinate, surface, abstract-operation candidate.
- Treat `$262` / host-hook failures as runner-harness apparatus work until the runner can observe them cleanly.
- Treat Temporal and broad Intl as deliberate subsystem work, not accidental next targets by raw count.
- Use the row-coherence protocol to prove that at least five examples share one mechanism before editing code.

**Remaining apparatus gaps**: `abstract_op=(unmapped)`, `uncategorized/resolver`, `failure/other`, coarse `absent-or-partial-surface`, and weak realm/execution-context recognition.
