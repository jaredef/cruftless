# regexp-conformance — Seed

## Substrate-pilot — regexp cluster parent locale

Spawned from LPA-EXT 3's Class C positioning-gap recommendation. This locale is the parent coordinate for the regexp conformance cluster surfaced by the full-suite Pin-Art matrix.

## Telos

Close the ECMA-262 RegExp conformance cluster as one parent locale with sibling rungs by surface family. Coordinate:

```
runtime/regexp ::
  E3/intrinsic-object:ecma-262 ::
  value-semantics/wrong-result + regexp-semantics ::
  property/RegExp-literal-and-constructor-semantics-match-test262
```

The induced property is that RegExp literals, constructor-created regexps, RegExp.prototype methods, and String.prototype regexp-dispatch methods all agree with the spec across the matrix-surfaced cluster.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/regexp.rs` — RegExp object allocation, constructor, prototype methods, string integration, `regexp_exec`.
- `pilots/rusty-js-runtime/derived/src/regex_hand.rs` — hand-rolled regexp parser/matcher.
- `pilots/rusty-js-runtime/derived/src/value.rs` — `CompiledRegex` representation and dispatch.
- `pilots/rusty-js-parser/derived/src/lexer.rs::read_regex_literal` — literal source-text tokenization when a rung proves lex-tier.
- `pilots/rusty-js-parser/derived/src/parser.rs::derive_lex_goal_after` — div-vs-regexp goal selection when a rung proves parser-goal shaped.
- Matrix anchor: `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-25-TECR-EXT-1-rerun/matrix.md`.
- Positioning-gap anchor: `pilots/apparatus/locale-positioning-audit/findings/positioning-gaps.md` Class C.

## Founding pool

LPA-EXT 3 Class C identified two sibling matrix coordinates:

| Rank | Count | Pin |
|---:|---:|---|
| 19 | 262 | `runtime/regexp :: value-semantics/wrong-result :: SyntaxError` |
| 23 | 229 | `runtime/regexp :: regexp-semantics :: failure/other` |

Total: **491 fails** across two regexp coordinates.

Additional reconnaissance from `language/literals/regexp/` in the 2026-05-25 interpreted full-suite shows a broader literal-regexp slice:

| Slice | Count | Reading |
|---|---:|---|
| `parser-form/early-error :: err:SyntaxError` | 176 | likely regexp static-semantics / accepted-invalid-pattern surface |
| `uncategorized/projection :: err:Test262Error` | 8 | line-separator literal source checks; lex/goal-symbol candidate |
| `value-semantics/wrong-result` | 6 | unicode/runtime matcher behavior |
| misc missing/uncategorized | 3 | inspect per-rung |

The top-level locale stays `regexp-conformance`; narrower source-text rungs may be nested if Rule 23 inspection proves multi-rung shape.

## Methodology

### RC-EXT 0 — baseline-inspection

Before substrate work, sample the 491 matrix rows and the `language/literals/regexp/` slice. Partition failures into:

1. raw lexer failure (`read_regex_literal`, line terminators, flags accumulator),
2. parser goal-symbol failure (division vs regexp literal),
3. regexp static-semantics failure (invalid pattern/flag accepted; expected SyntaxError),
4. runtime matcher semantics (`regex_hand`, captures, unicode, v flag, named groups),
5. prototype/String integration (`regexp_exec`, `.source`, `.flags`, `.test`, `.exec`, split/replace/search/match).

This is the Rule 23 rung. Do not assume the top matrix coordinate names the edit site.

### RC-EXT 1 — first surfaced coordinate

Land the smallest partition from EXT 0 satisfying Rule 11's coverage check. Candidate first rungs:

- named-groups bridge if `.groups`/`.indices` rows dominate and compose with `regex-engine-substrate`;
- static-semantics rejection for accepted-invalid regexp patterns if the `expected SyntaxError, got String` rows share one parser/engine validation site;
- literal line-separator/goal-symbol rung if the lexing slice is coherent.

### RC-EXT N — sibling rungs

Proceed by surface family, not by individual test file. Each rung gets:

- direct probes,
- exemplar subset,
- diff-prod gate if runtime behavior changes,
- parser crate tests if lexer/parser behavior changes,
- trajectory entry with empirical yield.

## Nested candidate: regex-literal-lexing

`regex-literal-lexing` remains a nested candidate under this locale, not a sibling top-level locale, until RC-EXT 0 proves otherwise.

Potential nested scope:

- RegularExpressionLiteral body and flags accumulation,
- U+2028/U+2029 line-terminator rejection in first char, body char, and escaped char positions,
- div-vs-regexp goal selection (`no-magic-asi`-style cases),
- literal source preservation for `.source`.

Existing lexer coverage already includes simple body/flags, character class slash, escapes, and unterminated regex. The nested rung should only spawn if the residuals are coherent enough to amortize the locale apparatus tax.

## Composes-with

- `pilots/regex-engine-substrate/` — current engine-level regexp substrate parent for matcher internals.
- `pilots/rusty-js-regex-fast/` — performance/leak workstream; not a conformance parent.
- `pilots/regexp-proto-test-coercion/`, `pilots/regexp-split-captures-bridge/`, `pilots/regexp-instance-accessor-shadow/` — prior/sibling surface rungs.
- `apparatus/docs/predictive-ruleset.md` — Rule 11, Rule 13, Rule 15, Rule 23 discipline.
- `pilots/apparatus/locale-positioning-audit/` — positioning-gap source.

## Falsifiers

- **Pred-rc.1**: RC-EXT 0 partitions the 491 rows into at least two coherent sibling rungs with shared substrate sites. Falsifier: rows are mostly mutually-exclusive long-tail.
- **Pred-rc.2**: the first implementation rung closes at least 10 full-suite rows with no diff-prod regression. Falsifier: <10 yield or PASS→FAIL in unrelated regexp/String behavior.
- **Pred-rc.3**: `regex-literal-lexing` stays nested unless the literal-source partition alone has multi-rung shape. Falsifier: literal lexing independently dominates the cluster and deserves top-level apparatus.

## Resume protocol

Read `trajectory.md` tail. Then inspect the current full-suite matrix rows for `runtime/regexp` and `language/literals/regexp/` before editing.

## Status

RC-EXT 0 founded the parent locale. RC-EXT 1 closed the split-captures bridge. RC-EXT 2 closed the Annex B `RegExp.prototype.compile` rebind bridge. Continue by selecting the next coherent regexp surface family from the matrix/runtime probe partition, not by isolated files.
