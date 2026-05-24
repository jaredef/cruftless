# strict-mode-destructuring-refs — Resume Vector / Seed

**Locale tag**: `L.strict-mode-destructuring-refs` (top-level)

**Status as of 2026-05-24**: **WORKSTREAM FOUNDED (SMDR-EXT 0)**. First substrate-fix sub-locale downstream of T262C's first matrix measurement. Targets the second-highest cluster in T262C's failure matrix (43 tests).

**Workstream**: fix the compiler bug at `pilots/rusty-js-bytecode/derived/src/compiler.rs:1361` where `ForBinding::Pattern` (standalone for-of head, no `var`/`let`/`const`) auto-allocates each bound name as a fresh local. Per ECMAScript 13.7.5.5 / 13.15.2 (Runtime Semantics: DestructuringAssignmentEvaluation), those names are ASSIGNMENT-TARGET REFERENCES; in strict mode an unresolvable target throws ReferenceError per §9.1.1.4.4 SetMutableBinding step 6.

**Empirically reproduced** (pre-fix):
```
"use strict";
function f() { for ([unresolvable] of [[]]) {} }
f()  // expected: ReferenceError. Actual: silently completes (auto-allocates local).
```
Plain assignment `unresolvable = 5` correctly throws ReferenceError. The bug is destructure-specific.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: TCC, TXC, T262C (the engagement's instrument-tier locales). This is the first SUBSTRATE-fix sub-locale spawned downstream of T262C's matrix.
**Composes with**:
- [T262C trajectory](../test262-categorize/trajectory.md) Finding T262C.3 — 4 of top 6 clusters are destructuring-binding edge cases.
- [T262C matrix row 2](../test262-categorize/results/2026-05-24/matrix.md) — 43 tests.
- ECMA-262 §13.7.5.5 (for-of runtime semantics) + §13.15.2 (DestructuringAssignmentEvaluation) + §9.1.1.4.4 (SetMutableBinding strict-mode ReferenceError).

## I. Telos

**Empirical answer to**: does routing `ForBinding::Pattern` through `emit_destructure_assign` (assignment-target references) instead of pre-allocating + `emit_destructure` close the 43-test `for-of-destructuring-ReferenceError-on-unresolvable` cluster?

The bench-anchored target: TCC parse-parity stays at 100%; TXC stays at 70.9%; diff-prod 42/42 holds; canonical fuzz byte-identical; test262-sample lifts by ≥40 tests (the 43-test cluster + likely cascade unlocks).

### I.1 First-cut scope

Per standing rule 13: design the fix at the deeper layer. Two paths considered:

- **(a) Patch in emit_destructure** to detect when names should be REFERENCES (no kind annotation) and route through StoreGlobal. Invasive; emit_destructure is BindingPattern-driven; references are Expr-driven.
- **(b) At the ForBinding::Pattern site, lower the BindingPattern to an Expr AssignmentPattern and call emit_destructure_assign** instead. Surgical; respects the existing split between binding-pattern (emit_destructure) and assignment-pattern (emit_destructure_assign).

(b) is the right tier. The Pin-Art-derived split between emit_destructure and emit_destructure_assign already encodes the semantic distinction; the bug is that ForBinding::Pattern is using the wrong half.

### I.2 Constraints

```
C1. cargo test --release --workspace continues to pass (no
    regressions in other crates).
C2. TCC parse-parity stays at 100%.
C3. TXC execute-parity stays at ≥70.9%.
C4. diff-prod 42/42 PASS.
C5. canonical fuzz acc=-932188103 byte-identical.
C6. For-of with `var`/`let`/`const` destructuring head — UNCHANGED.
    The fix only changes the no-keyword head path.
C7. Per Doc 740 §IV.2 + Standing rule 14 conservative-strip: if
    BindingPattern→Expr conversion fails or returns a non-trivial
    transform, BAIL (fall back to current behavior).
```

### I.3 Falsifiers

**Pred-smdr.1**: ≤80 LOC delta.
**Pred-smdr.2**: cargo test --workspace + diff-prod + canonical fuzz + TCC + TXC all hold or improve.
**Pred-smdr.3**: minimal-repro test passes (`"use strict"; for ([x] of [[]]) {}` throws ReferenceError).
**Pred-smdr.4**: test262-sample lifts by ≥40 tests (cluster + cascade).
**Pred-smdr.5 (DISCIPLINE — rule 13)**: closes in ≤3 implementation rounds.

## II. Apparatus + Methodology

- Edit at `pilots/rusty-js-bytecode/derived/src/compiler.rs` line ~1361.
- Existing instruments: TCC + TXC + test262-sample + diff-prod + canonical fuzz.
- Re-measure test262-sample after the fix.

Methodology:
1. **SMDR-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **SMDR-EXT 1** — implementation + minimal-repro test + gate re-measure + test262 re-measure + chapter close.

## III. Carve-outs

- ForBinding::Decl (with var/let/const) path UNCHANGED.
- Object-pattern targets (Expr::Object) inside the standalone pattern are handled by emit_destructure_assign's existing Object branch.
- Default-initializer in pattern leafs (`for ([x = 1] of ...)`) inherits emit_destructure_assign's existing default-handling.

## IV. Standing artefacts

- `pilots/strict-mode-destructuring-refs/seed.md`, `trajectory.md`
- Edit at `pilots/rusty-js-bytecode/derived/src/compiler.rs`
- Possible new test fixture at `pilots/strict-mode-destructuring-refs/fixtures/`

## V. Resume protocol

Read seed + trajectory tail. The substrate fix is bounded to compiler.rs:1361; the path is `ForBinding::Pattern(other)` branch where pre-allocation happens. Replace pre-allocation + emit_destructure with a BindingPattern→Expr conversion + emit_destructure_assign call. Verify with minimal repro + test262-sample re-measure.
