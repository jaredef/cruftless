# super-reference-lowering — Seed

**Locale tag**: `L.cruft-lowering-feature-gaps.super-reference-lowering`

**Status**: FOUNDED 2026-05-28 as the first child of
`cruft-lowering-feature-gaps/`.

## Telos

Close the `super` subset of the current
`availability/missing-lowering-feature` coordinate:

```text
compile: super reference outside of a class
compile: bare `super` reference is only valid as `super(...)` or `super.method(...)`
compile: super reference in a class with no `extends` clause
compile: super(...) outside of a class
```

At founding this subset accounts for **96/113** rows in
`test262-full-2026-05-27-161641`, and **22/32** rows in the parent exemplar
suite.

## Apparatus

- **Parent locale**: `pilots/cruft-lowering-feature-gaps/`.
- **Arc**: `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/`.
- **Exemplar suite**: `exemplars/exemplars.txt`, 22 current `super` rows from
  the parent CLFG baseline.
- **Runner**: `exemplars/run-exemplars.sh`, delegating to the parent runner
  with a child-specific list.
- **Primary substrate site**:
  `pilots/rusty-js-bytecode/derived/src/compiler.rs`.

## Mechanism Hypothesis

The compiler currently treats `super` as class-frame-only, but the failing rows
show at least three context propagation edges:

1. **Object-literal concise/accessor methods**: legal `super.prop` requires a
   HomeObject-like frame even outside class syntax.
2. **Direct eval nested in class methods/fields**: `super` inside direct eval
   must see the surrounding method's super context.
3. **Invalid `super` operations**: `delete super.x`, bare `super`, and
   non-derived `super` cases should reach the correct SyntaxError/TypeError
   behavior rather than compiler rejection.

## Methodology

This child remains baseline-first for one more rung before editing compiler
code:

1. Run the 22-path child exemplar suite.
2. Classify rows into object-literal home-object, direct-eval context capture,
   delete/bare-super early-error routing, no-extends class checks, and
   derived-constructor eval-super-call.
3. Select the first smallest coherent substrate move. Prefer HomeObject frame
   materialization if object-literal rows are local and do not require eval
   environment work.
4. Record redirects explicitly if direct-eval `super` rows belong under the
   eval environment-instantiation arc.

## Carve-outs

- This locale does not own general object-literal computed property semantics;
  candidate `object-literal-computed-property-semantics` remains broader and
  deferred.
- This locale does not own all class residuals; `class-lowering-residual-
  repartition` remains audit-first.
- This locale does not own for-in destructuring heads, update-target failures,
  or complex-assignment target lowering from the parent pool.
- This locale does not own parser-level `this`/`super` validity in for-heads;
  that was closed by `for-head-this-super-target/`.

## First Rungs

1. **SRL-EXT 0**: child spawn and exemplar suite.
2. **SRL-EXT 1**: classify the 22 failures by context edge and choose the
   first substrate owner.

## Resume Protocol

Read this seed, then `trajectory.md`, then the parent locale tail. Run:

```sh
pilots/cruft-lowering-feature-gaps/super-reference-lowering/exemplars/run-exemplars.sh
```
