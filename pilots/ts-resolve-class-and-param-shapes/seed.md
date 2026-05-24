# ts-resolve-class-and-param-shapes — Resume Vector / Seed

**Locale tag**: `L.ts-resolve-class-and-param-shapes` (top-level per Doc 737 §IV)

**Status as of 2026-05-24**: **CHAPTER CLOSED at TRCAPS-EXT 1 (1 implementation round; standing rule 13's seventh corroboration)**. Parse-success lifted 47.1% → **59.9%** (+12.8 pp; 28% over Pred-trcaps.3's ≥10 pp target). Six concrete substrate improvements landed; mid-round investigation surfaced three additional substrate bugs (regex goal, ternary disambig, method-shorthand paren-depth gating) beyond the seed's three planned rules.

**Historical status (founding)**: WORKSTREAM FOUNDED (TRCAPS-EXT 0). Second data-driven sub-locale from TCC's failure-table. Spawned per keeper directive ("No v2 it's all green field") at TRSLS-EXT 1 chapter close — naming corrected from the originally-proposed `ts-resolve-classes-v2` to drop the misleading v2 suffix (all sub-locale work is greenfield, not refactoring a prior locale).

**Per Finding TRSLS.2 discipline**: inspection of representative failures BEFORE committing to category names. Three real failure patterns surfaced:

1. **Destructured-pattern parameter annotations** (`function f({a, b}: T): void`) — example `ajv/lib/compile/errors.ts` byte 1921: the inner `}: T` annotation is not stripped because `is_annotation_colon` doesn't accept `RBrace` as a prev-anchor. This was incorrectly tagged "method-return-annotation" but the actual cause is the destructured-param shape.

2. **Generics on `extends` clause + `implements` keyword** (`class X<T> extends Y<U> implements Z`) — example `rxjs/src/internal/Subject.ts` byte 778: my decl-head-generics rule handles `class NAME<T>` but not `extends NAME<T>`. And `implements NAME, NAME, ...` is a TS-only clause that needs stripping. Incorrectly tagged "access-modifier" but cause is unrelated.

3. **Member access modifiers and `readonly` in class bodies** (`public x: T`, `private _y`, `readonly z: T`) — these are TS-only keyword modifiers on class fields; need stripping (token replacement with whitespace).

**Workstream**: extend `strip.rs` with three new rules covering the above. Single locale, one implementation round target (per standing rule 13's expected ≤3 rounds).

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: `ts-resolve/`, `ts-consumer-corpus/`, `ts-resolve-string-literal-safety/`.
**Composes with**:
- [TCC failure-table 2026-05-24 (post-TRSLS)](../ts-consumer-corpus/results/2026-05-24/failure-table.md) — empirical anchor
- [TRSLS-EXT 1 Finding TRSLS.2](../ts-resolve-string-literal-safety/trajectory.md) — categorization-validation discipline that drove inspection-before-spec
- [TSR strip.rs](../ts-resolve/derived/src/strip.rs) — substrate to modify
- [docs/standing-rule-13-prospective-application.md](../../docs/standing-rule-13-prospective-application.md) — sixth prospective application; ≤3 rounds

## I. Telos

**Empirical answer to**: do the three identified strip-rule gaps account for the majority of post-TRSLS failures, and does adding rules for them lift parse-success ≥10 pp beyond the current 47.1%?

The bench-anchored target: parse-success **47.1% → ≥57.1%** (≥+10 pp).

### I.1 First-cut scope

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. Three concrete rules:

1. **Destructured-pattern parameter annotation** — extend `is_annotation_colon`: when prev-anchor is `RBrace` OR `RBracket`, AND the matching `LBrace`/`LBracket` was preceded by `LParen`/`Comma` (i.e., we're in a function parameter list), AND `skip_type` lands on `,`/`)`/`=` — accept as annotation.

2. **`extends` / `implements` clauses on class declarations** —
   - `extends NAME<T,...>`: after the `extends` keyword in a class-decl context, strip any `<...>` immediately following the type name (generic args). Same `match_angle` helper used for `function NAME<T>` / `class NAME<T>`.
   - `implements TYPE, TYPE, ...`: when `implements` Ident appears after `extends NAME[<...>]` OR directly after `class NAME[<T>]`, strip the whole `implements ...` clause up to the next `{` (class body opener).

3. **Class-member modifiers** — at top-level of a class body (brace_stack top is `Block` AND the enclosing brace was preceded by `class NAME[<T>][ extends ...][ implements ...]`), strip the keywords `public`, `private`, `protected`, `readonly`, `abstract`, `override`, `static` BEFORE a member name. (Note: `static` is valid JS; do NOT strip it — only the TS-only modifiers.)

### I.2 Constraints

```
C1. Existing 25/25 ts-resolve unit tests continue to PASS.
C2. Diff-prod 42/42 PASS.
C3. TCC's currently-OK 176 files remain OK (no regression).
C4. TCC parse-success lifts by ≥10 pp (target ≥57.1%).
C5. Per docs/standing-rule-13-prospective-application.md §3: all
    four C-conditions hold for these rules — sibling anchor (existing
    strip rules), shape compat (same Scanner machinery), cost-positive
    (each rule is small; bench-impact via TCC re-measure expected
    favorable), bail-safe (TSR's strip never crashes the parser; worst
    case is a downstream PARSE error visible in TCC).
```

### I.3 Falsifiers

**Pred-trcaps.1**: total LOC delta ≤120 (three rules; ~30-50 each).

**Pred-trcaps.2**: TSR 25/25 tests continue to PASS; new regression tests for each rule added.

**Pred-trcaps.3**: TCC re-measure shows parse-success ≥57.1% (+10 pp).

**Pred-trcaps.4**: TCC currently-OK count (176) does not regress (zero false-strip).

**Pred-trcaps.5 (DISCIPLINE FALSIFIER per docs/standing-rule-13-prospective-application.md §5)**: locale closes in ≤3 implementation rounds. Sixth prospective application; single-round target.

## II. Apparatus

- **Code change**: `pilots/ts-resolve/derived/src/strip.rs`
- **Regression tests**: `pilots/ts-resolve/derived/tests/strip.rs`
- **Re-measurement**: `cargo run --release -p ts-consumer-corpus --bin tcc-measure`

## III. Methodology

1. **TRCAPS-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **TRCAPS-EXT 1** — implementation of three rules + regression tests + TCC re-measure + chapter close.

## IV. Carve-outs

- Constructor-parameter shorthand (`constructor(public x: T)`) NOT in this locale's scope — requires body rewrite, which is qualitatively different from strip. Deferred to a follow-on.
- Import-type / Export-type clauses NOT in this locale's scope; small file count (5).
- Decorator stripping NOT in this locale's scope; needs separate handling (9 files).
- Conditional/mapped/keyof types still deferred.

## V. Standing artefacts

- `pilots/ts-resolve-class-and-param-shapes/seed.md`, `trajectory.md`
- `pilots/ts-resolve/derived/src/strip.rs` (edit)
- `pilots/ts-resolve/derived/tests/strip.rs` (test additions)

## VI. Resume protocol

Read this seed, then trajectory.md tail. The three rules are well-specified in §I.1; implementation lands in strip.rs's step + is_annotation_colon. Re-run TCC; expect parse-success to lift ≥10 pp. Categories in the post-TRCAPS failure table will name the next sub-locale's scope.
