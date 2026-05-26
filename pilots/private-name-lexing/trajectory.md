# private-name-lexing — Trajectory

## PNL-EXT 0 — 2026-05-26 (founding survey; Rule-23 baseline pending)

Locale founded after refreshing the tokenization candidate register and rereading the required apparatus docs:

- `AGENTS.md`
- `apparatus/docs/repository-apparatus.md`
- `apparatus/docs/predictive-ruleset.md`
- `apparatus/docs/standing-rule-13-prospective-application.md`
- `apparatus/docs/agent-feedback-schema.md`
- `docs/engagement/tokenization-above-ir-candidate-brief.md` (explicit keeper-relevant source for this candidate)

Founding code survey:

- `lexer.rs` already maps `#` + `read_identifier_name()` to `TokenKind::PrivateIdent`.
- `token.rs` already carries `PrivateIdent(String)`.
- `stmt.rs::parse_class_member_name` consumes `PrivateIdent` for class member names.
- `expr.rs::consume_member_property` consumes `PrivateIdent` for `obj.#name`.
- `expr.rs` also has a v1 brand-check parser accommodation for bare `#name in this`.

This makes the candidate subtler than the original brief's "PrivateIdentifier tokenization" phrasing. The token kind is not missing. The EXT 0 load-bearing question is whether current behavior fails at:

1. the lexical boundary (`#` + IdentifierName, escapes, ZWNJ/ZWJ, whitespace rejection),
2. parser-form acceptance for class elements / member expressions,
3. static semantics (`#constructor`, duplicate declarations, AllPrivateIdentifiersValid), or
4. runtime private-brand/field semantics.

Test262 survey:

- Direct `private-accessor-name` fixtures: 40 files across `language/statements/class/elements/private-accessor-name/` and `language/expressions/class/elements/private-accessor-name/`.
- Focused `private-name` / `privatename` class-elements fixture set: 194 files across statements and expressions.
- Broader paths containing `private` number in the thousands, but mix runtime semantics and async/generator/destructuring surfaces; exclude from first exemplar set until the focused Rule-23 run determines the tier.

Focused baseline:

`pilots/private-name-lexing/exemplars/run-exemplars.sh` over the 40 direct `private-accessor-name` fixtures:

```
PASS=40 FAIL=0 / 40 (100.0%)
```

Interpretation: the narrow PrivateIdentifier lexical boundary for common, `$`, `_`, ZWNJ, ZWJ, U+2118, and escaped forms is already accepted across declaration and expression class forms. This substantially weakens the original "missing private-name lexing" hypothesis. Remaining candidate value likely lives in grammar/static-semantics fixtures (`grammar-privatename-*`, whitespace-after-`#`, `#constructor`, duplicate private names, AllPrivateIdentifiersValid), not direct lex acceptance.

Next rung:

PNL-EXT 0 baseline-inspection proper:

1. Extend the exemplar set from the 40 direct accessor-name fixtures to the 194 focused syntax/grammar private-name paths.
2. Run through the Test262 runner using `T262_ROOT`.
3. Cluster failures by reason into lex/parser/static/runtime buckets.
4. If bucket 1 dominates, proceed with PNL-EXT 1 lex-boundary fix. If not, record the Rule-23 redirect and spawn/attach the class-elements target.

Status: founded, no substrate code changed.
