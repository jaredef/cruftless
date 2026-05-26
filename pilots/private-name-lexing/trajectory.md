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

## PNL-EXT 1 — 2026-05-26 (private-name lexical/static early-error slice)

Substrate edits:

1. `lexer.rs::is_id_start` now excludes U+200C ZWNJ and U+200D ZWJ. They remain valid IdentifierPart through `is_id_continue`, but are not valid IdentifierStart. This closes the private-name `#\u200C...` / `#\u200D...` negative parse forms.
2. `stmt.rs::parse_class_member_name` rejects `PrivateIdent("constructor")`, implementing the private class element early error that `#constructor` is forbidden.
3. Class fields now use a class-specific terminator check instead of the generic permissive ASI helper. A field without `;`, `}`, or a line terminator before the next token now rejects, closing `#x #y` same-line negative forms.
4. `exemplars/run-exemplars.sh` accepts `PNL_EXEMPLARS_LIST` so PNL can keep the committed 40-fixture smoke set and run broader Rule-23 probes from generated sidecar/tmp lists.

Verification:

```
cargo check -p rusty-js-parser
cargo build --release --bin cruft -p cruftless
pilots/private-name-lexing/exemplars/run-exemplars.sh
PNL_EXEMPLARS_LIST=/private/tmp/pnl-focused.txt pilots/private-name-lexing/exemplars/run-exemplars.sh
```

Results:

| Probe | Before | After |
|---|---:|---:|
| direct `private-accessor-name` lex set | 40/40 | 40/40 |
| focused private-name grammar/static set | 126/194 | 134/194 |

Failure-shape after EXT 1:

- 16 SKIP: async-flag harness limitation.
- 18 FAIL: `test 3` assertion text, likely private-method/static semantics.
- 20 FAIL: runtime/private-brand semantics (`callee is not callable`, ordinary-object brand check, optional-chain private field).
- 6 FAIL: parse-phase static semantics still accepted (`arguments` in private field initializer; undeclared private name in computed property).

Finding PNL.1: the original candidate name was partly stale. Direct PrivateIdentifier lexing is already healthy; the useful lex-tier remainder was small (+8 on the focused set, counting the same-line class-field parser boundary). Remaining action belongs to class-elements static/runtime semantics, not private-name tokenization. Per Rule 23, PNL should stop after this early-error slice unless the keeper wants to spawn a class-elements static-semantics locale.

Status: PNL-EXT 1 landed; lex-tier slice closed for the focused probe. Recommended next coordinate: class-elements-static-semantics, not more private-name lexing.
