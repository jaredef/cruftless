# Resolver-Axis Gap Partition

LPA repartition output for a 49-gap resolver-axis snapshot. The source commit
was cited by the keeper as `600afe6`, but that object was not available in this
checkout at audit time. This document therefore records the keeper-provided
partition as apparatus input, not as a locally reproduced matrix pass. A future
rerun should replace the provenance section with the exact commit, run id, and
fixture list once the object is reachable.

## I. Baseline Inputs

- Source: keeper-provided summary of the recent heuristics commit.
- Commit object: `600afe6` cited, not reachable from local `origin/main` at
  audit time.
- Gap count: 49.
- Explicit source markers: `█`, documented in source before this work.
- Implicit probe-collision markers: `░`, discovered by fixture probes.
- Implicit constraint count: 25 of 49.

## II. Bucket Selector

Selector shape:

```
resolver-instance x axis cells from the 49-gap heuristics snapshot
```

This is not yet reproducible from local `interpreted.jsonl`. The reproducible
selector must be added when the source commit or artifact is available.

## III. Partition Summary

| Cell | Count | Dominant mechanism | Disposition |
|---|---:|---|---|
| Instance 2 x Axis H | 24 | bootstrap resolver install-sequence totality | `baseline-first` |
| Instance 4 x Axis R | 13 | AST-to-bytecode boundary-integrity underspecification | `audit-first` |
| Instance 4 x Axis O | 7 | operator-level unconsumed spec directives | `baseline-first` |

The three cells cover 44 of the 49 gaps. The remaining 5 gaps were not
partitioned in the keeper summary and remain uncategorized pending access to
the source artifact.

## IV. Arc A: Bootstrap Host Install Totality

**Cell**: Instance 2 x Axis H.

**Count**: 24 gaps.

**Mechanism hypothesis**: the bootstrap resolver's install sequence is missing
host built-in stages. This is a totality-of-consumption violation at the
bootstrap resolver: host directives exist in the desired compatibility surface
but are absent from, or installed at insufficient fidelity by, bootstrap.

**Examples named in the snapshot**:

- Absent host built-ins: `DataView.prototype.setInt8`,
  `util.isDeepStrictEqual`, `Readable.from`, `process.ppid`,
  `console.assert`.
- Wrong-fidelity built-ins: `Buffer.concat` returning `[object Object]`,
  `util.types.isDate` returning `false`, `assert.fail` producing a `String`
  rather than an `AssertionError`.

**Reading**: 33 of the 49 total gaps are totality violations. Axis H's 24 gaps
are the highest-coherence portion: the install sequence needs additional
stages, grouped by namespace.

**Disposition**: `baseline-first`.

**First baseline probe shape**:

1. Build a focused 24-fixture host-surface exemplar list.
2. Group by namespace: `DataView`, `Buffer`, `util`, `assert`, `stream`,
   `process`, `console`.
3. Run under current `cruft` and Bun/Node oracle as appropriate for the host
   surface.
4. Inspect whether each failure is absent install, wrong descriptor shape,
   wrong return type, or wrong error object.

**Redirect condition**: if a namespace gap is already owned by an active
surface locale, route as a scope extension rather than founding a sibling.

## V. Arc B: AST-To-Bytecode Boundary Integrity

**Cell**: Instance 4 x Axis R.

**Count**: 13 gaps.

**Mechanism hypothesis**: the AST-to-bytecode resolver has boundary leaks and
unconsumed directives. The emitted artifact fails to preserve semantic
directives that must cross from parse/AST into runtime-visible execution.

**Examples named in the snapshot**:

- Private fields leaking into `Object.keys`.
- Tagged-template `strings.raw` not populated.
- Direct `eval` not capturing outer `const` bindings.
- Destructuring not using the iterator protocol.
- Generator suspension deferred.
- RegExp named capture groups not populated.

**Reading**: this matches the design-level signal that multiple cuts on the
AST-to-bytecode resolver address boundary integrity, not isolated runtime
bugs. The boundary specification is structurally underspecified.

**Disposition**: `audit-first`.

**First audit shape**:

1. Split the 13 rows into boundary families: private slots, template objects,
   lexical environment capture, iterator-protocol lowering, generator
   suspension, RegExp match metadata.
2. Check existing locales for ownership, especially class/private-field,
   template/string, eval/binding, destructuring/iterator, generator/async, and
   RegExp conformance locales.
3. Emit child candidates only for families not already owned.

**Redirect condition**: if more than half of the rows are absorbed by active
locales, do not found a broad AST-boundary locale. Add scope-extension notes to
the owning locales instead.

## VI. Arc C: Operator Directive Completion

**Cell**: Instance 4 x Axis O.

**Count**: 7 gaps.

**Mechanism hypothesis**: individual operator-level spec directives are not
fully consumed by the AST-to-bytecode/runtime path. Fan-out is narrower than
Axis R; each row looks like a single-rung substrate fix.

**Examples named in the snapshot**:

- `finally` return override.
- `Symbol.toPrimitive` hint dispatch.
- `Object.seal` / `Object.preventExtensions` strict-mode enforcement.
- `String.prototype.normalize`.
- Position-argument string methods.

**Disposition**: `baseline-first`.

**First baseline probe shape**:

1. Build one focused fixture per named operator gap, plus one sibling fixture
   per family where available.
2. Classify each row by owning helper or opcode path.
3. Decide whether to batch as an operator-semantics queue or route rows to
   existing built-in locales.

**Redirect condition**: if a row sits wholly in a runtime intrinsic body rather
than AST-to-bytecode lowering, route it to the corresponding runtime built-in
locale.

## VII. Explicit vs Implicit Constraint Reading

The bidirectional Pin-Art reading is load-bearing:

- `█` markers are explicit gaps: documented in source before this work.
- `░` markers are implicit constraints: discovered only when fixture probes
  collided with the engine surface.

The snapshot reports 25 implicit constraints out of 49 gaps. These are higher
apparatus value than explicit omission markers because they expose decision
edges that were not represented in the source's own declared gap surface. In
Doc 707 terms, the backward direction of the Pin-Art probe is doing work:
fixtures discover unmodeled constraints that would otherwise remain invisible
until user-facing failure.

## VIII. Candidate Queue Edits

Add Tier N entries:

- `bootstrap-host-install-totality`: `baseline-first`.
- `ast-bytecode-boundary-integrity-audit`: `audit-first`.
- `operator-directive-completion`: `baseline-first`.

These are candidates, not founded locales. The missing source commit and the
mixed ownership risk both require Rule-23 baseline inspection before any
substrate locale hardens.

## IX. Findings

**Finding LPA.15 (resolver-axis cells expose mechanism coherence beyond raw
surface rows)**: the 49-gap snapshot concentrates into two hot cells and one
narrow operator queue. Instance 2 x Axis H is bootstrap install totality;
Instance 4 x Axis R is boundary integrity; Instance 4 x Axis O is operator
directive completion. The resolver-axis view is therefore a useful
repartition lens: it distinguishes install-sequence gaps, boundary-spec gaps,
and single-rung operator gaps that raw surface labels would blur.

**Finding LPA.16 (implicit probe-collision constraints are first-class
apparatus output)**: 25 of the 49 gaps were implicit constraints discovered by
fixture collision. These rows are not merely failures; they are newly surfaced
decision-basis edges. Future repartition artifacts should preserve the
explicit/implicit marker instead of collapsing both into the same count.

