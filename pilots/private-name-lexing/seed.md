# private-name-lexing — Seed

## Substrate-pilot — fourth tokenization-above-IR spawn from the brief.

Per keeper directive 2026-05-26: update tokenization candidates, read the top-level apparatus docs, then begin the `private-name-lexing` locale. Tier-I substrate locale per refreshed CANDIDATES.md ((tt)).

## Telos

ECMA-262 PrivateIdentifier / PrivateName lexing conformance for class private elements and private member access. Coordinate:

```
tokens-to-AST / lex-tier ::
  E1/lex-tier ::
  cut/private-identifier-tokenization ::
  property/cruft-tokenizes-and-rejects-private-identifiers-at-the-spec boundary
```

Rule-23 founding survey found that cruft already has the first substrate piece:

- `TokenKind::PrivateIdent(String)` exists.
- `lexer.rs` tokenizes `#` followed by `read_identifier_name()` as `PrivateIdent`.
- `stmt.rs::parse_class_member_name` accepts `PrivateIdent` as a class member name.
- `expr.rs::consume_member_property` accepts `PrivateIdent` as `obj.#name`.

So this locale is not founded as "add PrivateIdent token kind." Its first rung is to measure where the current implementation diverges: escape/Unicode PrivateIdentifier forms, `# constructor`/reserved private names, whitespace-after-`#` early errors, private-name-in-scope static semantics, and optional-chain/member-expression adjacency. The lex-tier candidate remains live only if the baseline failures cluster at the `#` tokenization boundary; otherwise Rule 23 redirects to class-elements parser/static-semantics work.

## Apparatus

- `pilots/rusty-js-parser/derived/src/lexer.rs`:
  - `Lexer::next_token` `#` branch (PrivateIdentifier entry)
  - `read_identifier_name`
  - `consume_identifier_codepoint`
  - `is_id_start` / `is_id_continue`
- `pilots/rusty-js-parser/derived/src/token.rs::TokenKind::PrivateIdent`
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_class_member_name`
- `pilots/rusty-js-parser/derived/src/expr.rs::consume_member_property`
- Test262 upstream sidecar:
  - `language/statements/class/elements/private-accessor-name/`
  - `language/expressions/class/elements/private-accessor-name/`
  - `language/*/class/elements/syntax/{valid,early-errors}/grammar-privatename-*`
  - `language/*/class/elements/*private*name*.js`

Founding survey: 40 direct `private-accessor-name` fixtures and 194 focused `private-name` / `privatename` class-elements fixtures across statement and expression class forms. Broader private class-element paths are much larger and mix runtime semantics with lexing; they are out of the initial exemplar set unless EXT 0 shows the lex-tier pool is too narrow.

## Methodology

### PNL-EXT 0 — baseline-inspection

Create a focused exemplar set from the 40 direct `private-accessor-name` fixtures plus the syntax/grammar `privatename` fixtures. Run through the Test262 runner with `T262_ROOT` and inspect failures by reason.

Classify failures into:

1. **Lex-tier**: `parse: lex error: InvalidIdentifier`, escaped form incorrectly rejected/accepted, whitespace after `#`, invalid ZWJ/ZWNJ handling.
2. **Parser-form**: private name token exists but class-element/member-expression grammar rejects the construct.
3. **Static semantics**: duplicate private names, undeclared private names, `#constructor`, private names in computed-property positions.
4. **Runtime semantics**: brand checks, field initialization, getter/setter semantics, evaluation order.

Only bucket 1 is this locale's narrow lexing substrate. Buckets 2-4 either become nested rungs or redirect to a class-elements locale per Rule 23.

### PNL-EXT 1 — private identifier lexical boundary, if EXT 0 confirms it

Likely candidate moves:

- Ensure `#` followed by an escaped IdentifierStart (`#\u0061`) is accepted or rejected exactly as PrivateIdentifier grammar requires.
- Ensure `#` followed by whitespace, punctuator, or non-IdentifierStart rejects before parser-form recovery.
- Verify ZWNJ/ZWJ and non-ASCII IdentifierPart handling mirrors IdentifierName after the `#`.

### PNL-EXT 2 — parser/static-semantics redirect, if EXT 0 shows lexing already holds

If failures are mostly class-element grammar or AllPrivateIdentifiersValid, record the redirect in this locale's trajectory and either:

- spawn a nested `private-name-static-semantics/` locale, or
- attach the work to an existing class-elements locale if one is already active.

## Carve-outs

- Full private field/method runtime semantics are not lexing. They remain out of scope unless Rule 23 redirects.
- Decorators using private member expressions are out of scope for the first cut; they require decorator grammar support.
- Class-fields runtime initialization and brand-check semantics are out of scope for PNL-EXT 1.

## Composes-with

- `docs/engagement/tokenization-above-ir-candidate-brief.md` — source brief that identified this candidate.
- `apparatus/docs/predictive-ruleset.md` Rule 23 — founding baseline-inspection is load-bearing because existing code already contains `PrivateIdent`.
- `pilots/identifier-tokenization/` — sibling tokenization locale; shares IdentifierName escape/Unicode handling questions.
- `pilots/string-literal-and-escape-conformance/` and `pilots/line-terminator-conformance/` — adjacent lex-tier conformance locales.
- `pilots/apparatus/tokenizer-error-classification-refinement/` — apparatus split that should surface any private-name lex errors as `availability/missing-lex-feature`.

## R13 prospective C1-C4 at founding

- C1 (sibling): PARTIAL — IDT/LTC/SLEC establish lex-tier one-site conformance moves, but private names cross class-element syntax and static-semantics boundaries.
- C2 (shape-compat): HOLDS for lexing — `PrivateIdent` and `read_identifier_name` already exist.
- C3 (cost-positive): TBV at PNL-EXT 0 — expected positive only if failures are lex-tier rather than runtime/private-brand semantics.
- C4 (bail safety): HOLDS for lex-only rejection/acceptance changes; TBV if redirected to static semantics.

## Resume protocol

Read `trajectory.md` tail. First action is PNL-EXT 0 exemplar construction + baseline run.

## Status

PNL-EXT 0 FOUNDED. Baseline not yet measured. Founding survey shows the lexing substrate exists; immediate work is Rule-23 baseline-inspection to decide whether this remains a lex-tier locale or redirects to class-elements parser/static-semantics.
