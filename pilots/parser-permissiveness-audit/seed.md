# parser-permissiveness-audit — Resume Vector / Seed

**Locale tag**: `L.parser-permissiveness-audit` (top-level)

**Status as of 2026-05-24**: **WORKSTREAM FOUNDED (PPA-EXT 0)**. Multi-round parser-tier locale spawned per T262C matrix cluster #1 (45-test arrow-function-dstr escaped-keyword) + cluster #9 (24-test for-of decl-cls negative-syntax) + cluster #11 (22-test arrow-function params-duplicate). Substrate gap: cruft's parser is systematically permissive at "no-ReservedWord" sites where ECMA-262 requires SyntaxError.

**Workstream**: enumerate the spec's "IdentifierReference but not ReservedWord" / "BindingIdentifier" / similar restriction sites; audit cruft's parser for each; add precise rejection logic (Rule 14 mirror: ADDING restriction requires false-positive caution — must reject only what spec rejects, not valid programs).

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: TCC, TXC, T262C, FODAS.
**Composes with**:
- [T262C matrix](../test262-categorize/results/2026-05-24/matrix.md) — clusters #1, #9, #11 (91 tests total)
- [Doc 742](../../docs/corpus-ref/742-the-resolver-instance-pattern-at-full-strength-downstream-dispatch-and-upstream-elision-as-doc-729s-empirical-refinements-from-a-typescript-parity-research-arc.md) §V — upstream elision: parser-tier should reject what spec rejects, not defer to runtime
- [Doc 740](../../docs/corpus-ref/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) (P4) — likely multi-tier; each restriction site is a sub-axis
- standing rule 14 mirror — adding restriction; false-positive risk dominant
- ECMA-262 §11.6.2 (Keywords + ReservedWord taxonomy), §13.1.1 (Identifiers static semantics), §14.7.5.1 (for-of head Early Errors), §15.7.1 (Class Definitions Early Errors)

## I. Telos

**Empirical answer to**: does an enumerated-site audit of cruft's parser's "no-ReservedWord" check coverage, with precise per-site fixes, close the parser-permissiveness clusters in T262C without false-positive regressions on valid programs?

### I.1 First-cut scope (PPA-EXT 1)

Per Rule 11 multi-axis check at parser-tier:
- (A1) cluster-axis: matrix-identified targets are clusters #1, #9, #11 (91 tests). Cluster #1 alone is 45 (arrow-function dstr).
- (A2) site-axis: enumerate spec sites where IdentifierReference/BindingIdentifier exclude ReservedWord. Initial list:
  1. `BindingIdentifier` (§13.1.1.1 SS:1) — function/class names; var/let/const declarators; catch param; for-binding head
  2. `IdentifierReference` (§13.1.1.1 SS:2) — bare identifier expressions; object pattern shorthand; array element shorthand
  3. `LabelIdentifier` (§13.1.1.1 SS:3) — break/continue labels
  4. Strict-mode additional reserveds: `implements`, `interface`, `let`, `package`, `private`, `protected`, `public`, `static`, `yield`
  5. Module-mode additional reserveds: `await`
  6. Async-context: `await`; generator-context: `yield`
- (A3) escape-form-axis: §11.6.2 says escaped keywords tokenize as IdentifierName but spec rejects them in restricted positions. The lexer must preserve the "had escape" bit.
- (A4) cover-grammar-axis: ArrowFormalParameters is recovered from CoverParenthesizedExpressionAndArrowParameterList; the no-ReservedWord check fires when the cover resolves to ArrowFormalParameters.

### I.2 Constraints

```
C1. cargo test --release --workspace stays GREEN. No regressions in
    other crates.
C2. TCC parse-parity stays at 100%. The audit must not over-reject
    valid TS source.
C3. TXC execute-parity stays at ≥70.9%.
C4. diff-prod 42/42 PASS.
C5. canonical fuzz acc=-932188103 byte-identical.
C6. test262-sample: zero PASS→FAIL transitions per round. Each round
    must be net-positive on the cluster's target tests while preserving
    all currently-passing tests.
C7. Conservative-site principle (Rule 14 mirror): apply the check ONLY
    at sites enumerated from spec text. If a site is ambiguous, defer
    to a later round with more spec evidence.
C8. Per-round enumeration: each round closes one (A2) site or one
    (A3)/(A4) axis component. No round attempts the full enumeration.
```

### I.3 Falsifiers

**Pred-ppa.1**: instrument runs cleanly; per-round test262-sample re-measure preserves all currently-passing tests (C6).
**Pred-ppa.2**: cluster #1 (45 tests, arrow-function dstr) closes in ≤3 rounds.
**Pred-ppa.3**: clusters #9 + #11 (46 tests combined, for-of decl-cls + arrow-function params-duplicate) close in ≤3 additional rounds.
**Pred-ppa.4**: TCC parse-parity holds at 100% across all rounds (C2). If any round flips a TCC test, the round's site-axis enumeration was wrong and the round reverts.
**Pred-ppa.5 (DISCIPLINE — Rule 13)**: each round closes in ≤2 implementation attempts. Negative results trigger Rule 13 revert + deeper-layer-closure.

## II. Apparatus + Methodology

- Edits at `pilots/rusty-js-parser/derived/src/parser.rs` + `expr.rs` per site.
- Reserved-word lookup table (compile-time): the set of identifiers spec §11.6.2 names ReservedWord + strict-additional + context-dependent.
- "Had escape" bit on the lexer's Ident token: required for (A3).
- Re-measure test262-sample after each round (~30 min); inspect per-test diff per Rule 15.

Methodology:
1. **PPA-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **PPA-EXT 1** — escaped-keyword-in-object-binding-shorthand (cluster #1's bulk: 41/45). Add "had escape" bit to lexer's Ident token + reject in object-pattern shorthand.
3. **PPA-EXT 2** — unescaped-keyword + arrow-single-ident-head (closes residual of #1).
4. **PPA-EXT 3** — for-of decl-cls (cluster #9, 24 tests): `for (X of ...)` where X is `class`-prefixed decl.
5. **PPA-EXT 4** — arrow-function params-duplicate (cluster #11, 22 tests): duplicate parameter names in arrow heads (§15.2.1 Early Errors).
6. **PPA-EXT N** — additional sites surfaced by post-round failure-table inspection (Rule 15).

## III. Carve-outs

- Property NAMES (not values) in object literals + class bodies allow ReservedWord per §13.2.5.1 — UNCHANGED.
- `as` keyword in import/export specs is contextually-allowed — UNCHANGED.
- `get`, `set`, `async`, `of`, `from`, `target`, `meta` are contextual keywords — UNCHANGED.
- Module-mode `await` reservation deferred to a later round (separate site-axis).

## IV. Standing artefacts

- `pilots/parser-permissiveness-audit/seed.md`, `trajectory.md`
- Edits at `pilots/rusty-js-parser/derived/src/{parser,expr,lexer}.rs`
- Per-round minimal-repro fixtures at `pilots/parser-permissiveness-audit/fixtures/`

## V. Resume protocol

Read seed + trajectory tail. The audit is enumeration-driven per (A2). Each round closes one site; per-round test262-sample re-measure validates (C6 zero-regression). Failures trigger Rule 13.
