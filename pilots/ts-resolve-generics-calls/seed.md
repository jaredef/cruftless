# ts-resolve-generics-calls — Resume Vector / Seed

**Locale tag**: `L.ts-resolve-generics-calls` (top-level per Doc 737 §IV)

**Status as of 2026-05-24**: **CHAPTER CLOSED at TRGC-EXT 1 (1 implementation round; standing rule 13's eighth corroboration)**. Parse-success **59.9% → 69.3%** (+9.4 pp; 18% over Pred-trgc.3's ≥8 pp target). Three substrate improvements; mid-round ternary-tracking discovery delivered +7.0 pp on top of the planned +2.4 pp generic rules.

**Historical status (founding)**: WORKSTREAM FOUNDED (TRGC-EXT 0). Third data-driven sub-locale from TCC's failure-table. Spawned per keeper directive at TRCAPS-EXT 1 close. Inspection of representative failures (per Finding TRSLS.2 + TRCAPS.1 discipline) surfaced **two distinct unhandled patterns** under the `generic-call` heuristic tag:

1. **Generic arrow function**: `<T>(params) => body` — `<T>` precedes the param list. Pattern at expression-start position (after `=`, `(`, `,`, `=>`, `?`, `:`, `[`, `{`, return-keyword, etc.).
2. **Generic instantiation / generic method call / generic method declaration**: `NAME<T>(args)`, `new NAME<T>(args)`, `methodName<T>(params) { ... }`. Pattern: Ident + `<` + type-list + `>` + `(`.

The classic `a < b` operator vs `f<T>` generic-call ambiguity is resolved by the look-ahead-to-`>(` check: real comparisons never produce a balanced `<...>(` sequence.

**Workstream**: extend `strip.rs` with two new rules covering the above. Single locale, one implementation round target per standing rule 13.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: `ts-resolve/`, `ts-consumer-corpus/`, `ts-resolve-string-literal-safety/`, `ts-resolve-class-and-param-shapes/`.
**Composes with**:
- [TCC failure-table 2026-05-24 (post-TRCAPS)](../ts-consumer-corpus/results/2026-05-24/failure-table.md) — row 2 `generic-call` at 40 files
- TRCAPS-EXT 1 + TRSLS-EXT 1 — established strip extension patterns + the inspect-before-spec discipline
- [TSR strip.rs](../ts-resolve/derived/src/strip.rs) — substrate to modify
- [docs/standing-rule-13-prospective-application.md](../../docs/standing-rule-13-prospective-application.md) — seventh prospective application

## I. Telos

**Empirical answer to**: do the two identified patterns account for the bulk of the 40 `generic-call`-tagged failures, and does adding strip rules for them lift parse-success ≥8 pp beyond the current 59.9%?

Target: parse-success **59.9% → ≥67.9%** (≥+8 pp).

### I.1 First-cut scope

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. Three concrete rules (added one mid-design after re-inspecting examples):

1. **Generic arrow `<T>(...) =>`** — at expression-start positions (prev-token is one of the expression-start contexts per `expr_or_div_goal`'s RegExp branch), when current token is `<` followed by `match_angle` close followed by `(`, strip the `<...>` region.

2. **Generic call / instantiation / method-decl `NAME<T>(...)`** — when current token is `<` AND prev token is Ident OR `)` OR `]`, AND `match_angle` succeeds, AND the next token after `>` is `(`, strip the `<...>` region.

3. **`new NAME<T>(...)`** — special case of #2; covered by the same rule since `NAME` is Ident.

The look-ahead-to-`>(` filter resolves the `a < b` ambiguity: in `a < b()`, `match_angle` would find no balanced `>` before `;` / `,` / a statement boundary; rule bails.

### I.2 Constraints

```
C1. Existing 35/35 ts-resolve unit tests continue to PASS.
C2. Diff-prod 42/42 PASS.
C3. TCC's currently-OK 224 files remain OK.
C4. TCC parse-success lifts by ≥8 pp (target ≥67.9%).
C5. Real `a < b` operators are NOT mis-stripped — verified by
    regression tests (`x < y && y < z`, `a<b?c:d`).
C6. Per docs/standing-rule-13-prospective-application.md §3:
    all four C-conditions hold.
```

### I.3 Falsifiers

**Pred-trgc.1**: total LOC delta ≤80 (two rules; ~30-50 each).
**Pred-trgc.2**: 35/35 tests + new regressions (≥5 added) all PASS.
**Pred-trgc.3**: TCC parse-success ≥67.9% (+8 pp).
**Pred-trgc.4**: 224 pre-TRGC OK files all remain OK.
**Pred-trgc.5 (DISCIPLINE FALSIFIER)**: closes in ≤3 implementation rounds. Single-round target per the established cadence.

## II. Apparatus + Methodology

Same as prior sub-locales. TRGC-EXT 1 = implementation + regression tests + TCC re-measure + chapter close.

## III. Carve-outs

- Conditional/mapped/keyof types in type-position generics deferred.
- Decorator-attached generic shape deferred.

## IV. Standing artefacts

- `pilots/ts-resolve-generics-calls/seed.md`, `trajectory.md`
- `pilots/ts-resolve/derived/src/strip.rs` (edit)
- `pilots/ts-resolve/derived/tests/strip.rs` (test additions)

## V. Resume protocol

Read seed + trajectory tail. The three rules are well-specified in §I.1. Per the discipline: when investigating post-fix failures, look for TSR substrate bugs (per TRCAPS.1) before assuming a missing TS feature.
