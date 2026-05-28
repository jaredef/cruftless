---
arc: 2026-05-28-parser-early-error-conformance
trigger: Plan agent's back-fit analysis 2026-05-28 (keeper directive Telegram 10158); empirical: PCR-EXT 1 missing-parser-feature coordinate (~471 fails) + spinoff Chain 1 (LGSS → PPIF → FHNB)
opened: 2026-05-28
closed: IN PROGRESS
close_condition: PCR matrix's "missing-parser-feature" coordinate < 50 records; per-locale exemplar suites green; cross-locale findings on the production-boundary early-error verdict pattern recorded.
---

# Parser Early Error Conformance Arc

## Trigger

Plan agent's back-fit (2026-05-28, per keeper Telegram 10158) identified eleven top-level locales sharing the parser-tier mouth-terminus: token stream → AST plus early-error verdicts at spec-mandated production boundaries per ECMA-262 §13.x. Empirically anchored in the PCR-EXT 1 matrix coordinate (~471 fails on missing-parser-feature) + the LGSS → PPIF → FHNB spinoff chain documented in `apparatus/locales/spinoff-chains.md`.

## Telos

Subsume the eleven parser-tier locales under one arc with explicit (M, T, I, R) per Doc 744. The arc-tier mouth is "token stream per lex-tier-conformance arc's terminus"; the arc-tier terminus is "AST + early-error verdict at every spec-mandated production boundary per §13.x"; the arc-tier interior is the Statement/Expression productions + for-head + arrow / rest / tagged-template parse rules + class-elements static semantics; the arc-tier relations: DAG ↑ `2026-05-28-lex-tier-conformance` (this arc's mouth is the lex arc's terminus); DAG ↓ bytecode emit (the bytecode consumes this arc's AST); lattice with the strict-mode-bound-names arc (production-boundary early errors depend on strictness context).

## Sub-locale roster

| Locale | Role in arc | Status pre-arc |
|---|---|---|
| `parser-precedence-in-flag` (PPIF) | precedence-in flag through expressions | LANDED (chain spinoff from LGSS) |
| `for-head-non-binding-lhs` (FHNB) | for-head LHS-not-binding early error | LANDED (chain spinoff from PPIF) |
| `for-head-this-super-target` | for-head this/super target restriction | LANDED |
| `for-head-assignment-pattern-validity` | for-head AssignmentPattern validity | LANDED |
| `for-of-rhs-is-assignment-expression` | for-of RHS is AssignmentExpression | LANDED |
| `for-of-async-lookahead` | for-of async lookahead | LANDED |
| `arrow-misc-early-errors` | arrow miscellaneous early errors | LANDED |
| `rest-param-trailing-comma` | rest-param trailing-comma rejection | LANDED |
| `array-pattern-rest-trailing-comma` | array-pattern rest trailing-comma rejection | LANDED |
| `yield-in-function-params` | yield in function-params rejection | LANDED |
| `statement-declaration-in-body-position` | SD-in-body-position early error | LANDED |
| `tagged-template-object-construction` | tagged-template Object boundary | LANDED |

## Cumulative yield

Pre-arc: per-locale closures of the LGSS → PPIF → FHNB chain + the for-head cluster + class-elements-static-semantics. Aggregated yield rendered as engagement-wide rate movement at PCR matrix coordinate refinement re-measurements.

## Cross-arc relations

- **DAG ↑ `2026-05-28-lex-tier-conformance`**: this arc's mouth is the lex arc's terminus (token stream + bit-flags). Strict DAG composition.
- **DAG ↓ `rusty-js-bytecode`**: the bytecode pillar consumes this arc's AST.
- **Lattice with `2026-05-28-strict-mode-bound-names`** (proposed): production-boundary early errors depend on strictness context.
- **Alphabet-exchange ↑ `2026-05-28-iterator-protocol-substrate`**: for-head productions parse into iterator-protocol mouth.

## Cross-locale findings

To be promoted. Initial sketch:

**Finding PEC.1 (pending)**: spinoff-chains.md's LGSS → PPIF → FHNB recurrence is the arc's empirical signature: a single production-boundary clarification at one site (LGSS goal-symbol selection) cascades into adjacent-site early-error verdicts (PPIF precedence-in flag; FHNB non-binding LHS). The chain-spinoff pattern is the arc's expected sub-locale spawn cadence.

## Status

IN PROGRESS — scaffolded 2026-05-28 per keeper directive 10158. Per the Plan agent's recommendation, this is the fifth new arc to be back-fit (high roster density + dense cross-locale spinoff chain documented at `apparatus/locales/spinoff-chains.md`).
