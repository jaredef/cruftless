# identifier-tokenization — Seed

## Substrate-pilot — second tokenization-above-IR spawn from the brief.

Per keeper directive (Telegram 9824) "continue with candidate tokenization." Tier-I substrate locale per refreshed CANDIDATES.md ((qq)).

**Note**: this locale's seed was authored AFTER its baseline-inspection (per Rule 23) surfaced the actual substrate gap. The narrative reflects the corrected mechanism per findings.md Addendum XV; the initial baseline-inspection mis-read (attributing all 115 fails to a non-existent eval-error-class issue) is documented in the trajectory.

## Telos

§11.6.2 ReservedWord exclusion at BindingIdentifier positions. Coordinate:

```
tokens-to-AST / parser-form ::
  E1/lex-tier ::
  cut/reserved-word-as-binding-identifier ::
  property/cruft-rejects-reserved-word-as-binding-identifier
```

ECMA-262 §11.6.2.1: "It is a Syntax Error if the StringValue of IdentifierName is the same String value as the StringValue of any ReservedWord except for yield or await." Cruft currently accepts ReservedWords as BindingIdentifier in var/let/const/function/class/parameter positions (verified by direct audit; see Finding NLC.2 in findings.md Addendum XV).

The induced property is correct rejection of `var break = 1`, `function break() {}`, etc., across every BindingIdentifier consumption site.

## Apparatus

- `pilots/rusty-js-parser/derived/src/parser.rs::parse_binding_identifier` — primary BindingIdentifier consumption site; centralized ReservedWord check goes here.
- BindingIdentifier consumers (var-decl, let-decl, const-decl, function-decl, class-decl, function-params, arrow-params, destructure-leaves, catch-param, import-spec, etc.) — each routes through parse_binding_identifier; centralization closes all in one move.
- **Exemplar suite**: `pilots/identifier-tokenization/exemplars/exemplars.txt` — 268 fixtures from `language/identifiers/`.
- **Baseline measurement (FOUNDING, 2026-05-25)**: **PASS=153, FAIL=115 (57.1%)**. All 115 fails share shape "expected SyntaxError, got String" — the symptom of cruft's parser ACCEPTING reserved-word-as-identifier source that should be rejected (per Finding NLC.0-revised + NLC.2).

## Methodology

Single rung (multi-rung shape possible if §11.6.2's edge cases warrant nesting).

### IDT-EXT 1 — centralize ReservedWord check in parse_binding_identifier

Edit `pilots/rusty-js-parser/derived/src/parser.rs::parse_binding_identifier` to reject every Keyword/ReservedWord token. Per the existing `is_reserved_word` predicate (already used in object-pattern-shorthand and arrow-param strict checks), the discrimination machinery exists; the addition is one check at the BindingIdentifier production site.

Strict-mode additional reserved words (yield/let/static/implements/etc. per §13.1.1.1) are already handled at their respective sites (SBEA, SMPT). IDT-EXT 1's scope is the ALWAYS-RESERVED word set per §11.6.2.

**Expected yield**: 115 tests in this locale + likely additional fails in adjacent locales (function-declaration tests, class-name tests, parameter tests). The centralized check propagates without per-site duplication.

### IDT-EXT 2 — escaped-form coverage

Verify the §11.6.2 check fires for ESCAPED reserved words (`\u{62}\u{72}\u{65}\u{61}\u{6b}` → `break`). Lex tier must preserve had-escape; parse tier must apply ReservedWord exclusion to the cooked identifier. If the lex tier's identifier-cooking already produces "break" for the escape, the parse tier check from EXT 1 catches it; if not, need lex-tier had-escape preservation + parse-tier cooked-form check.

Direct audit (Finding NLC.2): cruft accepts escaped `break` as ident → likely the lex tier IS cooking to "break" but parse tier doesn't reject. EXT 1 should close.

## Carve-outs

- **Property names** (`{break: 1}`, `obj.break`) — these allow ReservedWord per spec. The check must fire at BindingIdentifier, not at IdentifierName generally.
- **yield/await contextual handling** — yield in generators, await in async; the strict-mode-extended reserved set is already handled by SBEA/YIFP. IDT-EXT 1 doesn't disturb that.
- **Unicode-ID range conformance** (the `part-unicode-N.0.0-*` test category, ~half of the 268) — covered by current cruft lex; passes per baseline (likely most of the 153 PASSes are these).

## Composes-with

- `apparatus/docs/predictive-ruleset.md` Rule 23 — this locale is Rule 23's empirical anchor (the locale-as-probe pattern surfaced and corrected via baseline-inspection iteration).
- `pilots/numeric-literal-conformance/` (NLC) — sibling Tier-I locale whose corrected mechanism (NLC.0-revised) shares the diagnostic "got String" shape but resolves at a different substrate site.
- SBEA-EXT 1, SMPT, YIFP — sibling rungs that handle strict-mode-extended + generator/async-extended reserved words; IDT-EXT 1 handles the always-reserved baseline set they extend.
- `docs/engagement/tokenization-above-ir-candidate-brief.md` — the brief that spawned this locale.
- `pilots/rusty-js-jit/findings.md` Addendum XV — the correction that establishes this locale's actual scope.

## R13 prospective C1-C4 at founding

- C1 (sibling): HOLDS — SBEA-EXT 1's strict-mode-extended check is the empirical sibling at parse_binding_identifier.
- C2 (shape-compat): HOLDS — is_reserved_word predicate exists; parse_binding_identifier centralizes consumption.
- C3 (cost-positive): expected positive (one predicate check per binding-id site).
- C4 (bail safety): HOLDS — parse-time discrimination; no runtime divergence.

## Resume protocol

Read `trajectory.md` tail.

## Status

IDT-EXT 0 FOUNDED. Baseline 153/268 (57.1%). Founding finding: per Rule 23 baseline-inspection, the actual substrate gap is at `parse_binding_identifier` for ReservedWord rejection (NOT eval-error-class as initially mis-read). IDT-EXT 1 is the immediate substrate move.
