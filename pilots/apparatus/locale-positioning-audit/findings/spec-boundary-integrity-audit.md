# Spec Boundary Integrity Audit

LPA follow-on to `resolver-axis-gap-partition.md` for the Instance 4 x Axis R
cell. The resolver-axis partition marked this cell `audit-first` because the
13 gaps shared a boundary-integrity signal but likely overlapped active
locales. This document performs that ownership audit.

## I. Inputs

- Source artifact: `pilots/apparatus/locale-positioning-audit/findings/resolver-axis-gap-partition.md`.
- Candidate queue entry: `ast-bytecode-boundary-integrity-audit`.
- Cell: Instance 4 x Axis R.
- Count: 13 gaps.
- Mechanism hypothesis: AST-to-bytecode emitted artifacts fail to preserve
  semantic directives into runtime-visible behavior.

Named examples from the source partition:

1. Private fields leaking into `Object.keys`.
2. Tagged-template `strings.raw` not populated.
3. Direct `eval` not capturing outer `const` bindings.
4. Destructuring not using the iterator protocol.
5. Generator suspension deferred.
6. RegExp named capture groups not populated.

## II. Existing Locale Coverage

| Boundary family | Current owner | Audit read | Disposition |
|---|---|---|---|
| Private fields leak through ordinary property reflection | `private-field-runtime-slots/` plus `class-elements-static-semantics/` | PFRS explicitly targets private storage that does not leak through ordinary reflection and has closed the focused private-name probe. | absorbed |
| Tagged-template `strings.raw` | no focused locale found | `string-literal-and-escape-conformance/` is lex-tier string literal work, not template object construction. This boundary remains unowned. | baseline-first candidate |
| Direct `eval` outer lexical capture | partial adjacency only | `lex-error-propagation-to-eval-surface/` owns lex-error propagation through eval, not environment capture. This boundary remains unowned. | baseline-first candidate |
| Destructuring via iterator protocol | partial adjacency in AGFA residuals | `async-generator-and-for-await-lowering/` has destructuring abrupt-completion and for-await rows, but not general destructuring iterator protocol. Needs exemplar split before deciding parent/child. | baseline-first candidate |
| Generator suspension deferred | `async-generator-and-for-await-lowering/` plus prior generator residuals | AGFA documents the eager generator bridge and explicitly carves out full suspended generator execution. The problem is known and owned as residual direction there. | scope-extension |
| RegExp named capture groups | `regexp-conformance/` | RC is the parent locale for regexp conformance and names captures/named groups as candidate first-rung shape. | scope-extension |

## III. Audit Result

The broad `ast-bytecode-boundary-integrity-audit` candidate should not be
spawned as a substrate locale. More than half of the named families are already
owned or adjacent to active parent locales. A broad AST-boundary locale would
duplicate existing scope and blur different boundary mechanisms.

The useful result is a split:

1. **Absorbed / scope-extension families**
   - Private fields reflection leak: absorbed by PFRS/CESS.
   - Generator suspension: scope-extension under AGFA or a generator-specific
     child when the suspended-execution model is funded.
   - RegExp named captures: scope-extension under RC.

2. **Fresh baseline-first candidates**
   - Tagged-template object boundary.
   - Direct eval lexical environment capture.
   - Destructuring iterator protocol boundary.

## IV. Candidate Arcs

### Arc A: Tagged Template Object Boundary

**Candidate name**: `tagged-template-object-boundary`.

**Mechanism hypothesis**: AST-to-bytecode lowering must construct the template
object pair with cooked strings and a non-mutable `.raw` array. Missing
`strings.raw` means the directive did not survive the parser/AST boundary into
the runtime call artifact.

**First baseline probe shape**:

1. Run focused fixtures from `language/expressions/tagged-template/` and
   `language/template-literals/`.
2. Inspect whether failures are parser escape/cooked-string issues, template
   object allocation, `.raw` descriptor shape, or call argument ordering.
3. Spawn only if object allocation/descriptor shape dominates.

**Redirect condition**: if failures are malformed escape handling or raw source
preservation, route to string/template literal lexing instead.

### Arc B: Direct Eval Lexical Capture

**Candidate name**: `direct-eval-lexical-capture`.

**Mechanism hypothesis**: direct eval must execute against the caller's lexical
environment, including `const`/`let` bindings. Failure to capture outer const
bindings is a boundary leak between AST/eval-call classification and runtime
environment selection.

**First baseline probe shape**:

1. Build examples for direct eval reading outer `const`, shadowing, and
   indirect eval contrast.
2. Include strict and sloppy variants.
3. Classify failure as parser direct-eval marking, bytecode call lowering,
   runtime eval entry environment, or environment-record lookup.

**Redirect condition**: if failures are lex/parse error propagation through
eval, route to `lex-error-propagation-to-eval-surface/`.

### Arc C: Destructuring Iterator Protocol Boundary

**Candidate name**: `destructuring-iterator-protocol-boundary`.

**Mechanism hypothesis**: array binding/assignment patterns must consume the
iterator protocol, including iterator close and abrupt completion behavior.
If lowering treats destructuring as array-index access, the iterator directive
is consumed at the wrong boundary.

**First baseline probe shape**:

1. Sample array binding and assignment destructuring fixtures with custom
   iterables, abrupt `next`, `return`, and elisions/rest.
2. Compare ordinary destructuring with for-of / for-await destructuring rows.
3. Classify whether the owner is general destructuring lowering or the
   for-await/AsyncFromSync path already owned by AGFA.

**Redirect condition**: if async-only rows dominate, keep the work under
`async-generator-and-for-await-lowering/`; if synchronous destructuring rows
dominate, found a general destructuring boundary locale.

## V. Candidate Queue Edits

Update Tier N:

- Mark `ast-bytecode-boundary-integrity-audit` as audited/split, not a future
  broad locale.
- Add three baseline-first child candidates:
  - `tagged-template-object-boundary`.
  - `direct-eval-lexical-capture`.
  - `destructuring-iterator-protocol-boundary`.

## VI. Findings

**Finding LPA.17 (boundary-integrity is a parent signal, not a substrate
locale by itself)**: Instance 4 x Axis R correctly identified a design-level
boundary-integrity smell, but its named examples map to distinct resolver
boundaries. The right apparatus move is ownership partition, not a broad
substrate locale.

**Finding LPA.18 (more than half of the boundary examples are already owned)**:
private fields are absorbed by PFRS/CESS, generator suspension is an AGFA
scope-extension, and RegExp named captures belong under RC. The `audit-first`
disposition prevented duplicate locale founding.

**Finding LPA.19 (three fresh boundary candidates remain after absorption)**:
tagged-template object construction, direct eval lexical capture, and
destructuring iterator protocol each have a plausible single boundary
mechanism and a concrete baseline probe. They should proceed independently,
not as one AST-boundary locale.

