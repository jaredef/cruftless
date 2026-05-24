# parser-permissiveness-audit — Trajectory

## PPA-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive "B" at FODAS-EXT 1 close. T262C matrix cluster #1 (45 tests, arrow-function dstr negative-syntax) + cluster #9 (24 tests, for-of decl-cls) + cluster #11 (22 tests, arrow-function params-duplicate) all trace to parser permissiveness at "no-ReservedWord" sites.

**Reconnaissance summary**:
- 41/45 of cluster #1 use escaped-keyword form (`break`); 4/45 use unescaped form
- `parser.rs:372 parse_binding_identifier` has explicit carve-out ("do not reject reserved-word bindings here; defer to expression grammar") — but the deferred check doesn't exist
- `parser.rs:817` object-pattern shorthand likewise routes through `TokenKind::Ident(name)` without ReservedWord check
- Per §11.6.2, escaped keywords tokenize as IdentifierName but spec still rejects in restricted positions

**Pre-spawn Rule 11 multi-axis check** (parser-tier adaptation):
- (A1) cluster-axis: clusters #1/#9/#11 (91 tests)
- (A2) site-axis: 4-6 spec-enumerated sites (BindingIdentifier, IdentifierReference, LabelIdentifier, strict-additional, module-additional, context-dependent)
- (A3) escape-form-axis: lexer must preserve "had escape" bit
- (A4) cover-grammar-axis: ArrowFormalParameters recovery from CoverParenthesizedExpressionAndArrowParameterList

**Rule 14 mirror identified**: this locale ADDS restriction (opposite direction of standard rule 14). False-positives (rejecting valid programs) are dominant risk. Conservative-site principle (C7): only fire check at sites enumerated from spec text.

**Five Pred-ppa.* + discipline falsifier**:
- Pred-ppa.1: instrument runs; C6 zero PASS→FAIL per round
- Pred-ppa.2: cluster #1 closes in ≤3 rounds
- Pred-ppa.3: clusters #9 + #11 close in ≤3 additional rounds
- Pred-ppa.4: TCC parse-parity holds at 100% across all rounds
- Pred-ppa.5 (Rule 13 discipline): each round ≤2 implementation attempts

**Founding artefacts**: seed.md + trajectory.md + scaffolded dirs. PPA-EXT 1 (escaped-keyword-in-object-binding-shorthand; cluster #1 bulk: 41/45) next.
