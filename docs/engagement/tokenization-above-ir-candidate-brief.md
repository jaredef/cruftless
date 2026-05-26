# Tokenization-Above-IR Candidate Brief

Per keeper directive (Telegram 9818): zoom in on tokenization above the ECMA IR; identify candidates for development.

## Current state

cruft's lexer (`pilots/rusty-js-parser/derived/src/lexer.rs`, ~1,125 LOC) already handles:

- 3 lexical goal symbols: `InputElementDiv`, `InputElementRegExp`, `InputElementTemplateTail` (LGSS-canonical, since LGSS-EXT 1+2 landed).
- LineTerminator tracking via `preceded_by_line_terminator` per-token flag (consumed by ALTA-EXT 1's NoLT-before-=> check).
- 9 lex-error kinds: UnterminatedString, UnterminatedTemplate, UnterminatedRegex, UnterminatedComment, InvalidEscape, InvalidNumeric, InvalidIdentifier, LegacyOctalInModule, UnexpectedChar.
- Numeric literals: Decimal, Hex, Binary, Octal (without separator/BigInt-suffix audit).
- Unicode ID start/part (via `is_id_start` / `is_id_continue` — basic ranges).

## test262 surface (count of `.js` fixtures per lex-tier subdirectory)

| Path | Count |
|---|---:|
| `language/identifiers` | 268 |
| `language/literals/regexp` | 238 |
| `language/literals/numeric` | 157 |
| `language/literals/string` | 73 |
| `language/line-terminators` | 41 |
| `language/identifier-resolution` | 14 |
| `language/punctuators` | 11 |

**Total: ~802 fixtures at the lexer/tokenization tier.** A coordinate-shaped locale targeting any of these subdirectories has a single test262-directory-to-spec-rule correspondence (per BBND's Finding BBND.2: when a test262 sub-directory shares its name with a single spec production, yield-per-LOC is typically extreme).

## Already-covered or in-flight tokenization-tier work

- **LGSS** — goal-symbol selection (CLOSED, 3 rungs).
- **ALTA-EXT 1** (inside `arrow-misc-early-errors/`) — NoLT before `=>` (CLOSED).
- **regexp-conformance** (Tier-E candidate (u) per refreshed CANDIDATES.md) — wraps both regexp-semantics matrix coordinates; covers the regex-engine surface but NOT specifically the regex-literal-lexing rules.

## Gap analysis from the categorizer

PCR-EXT 1+2 surfaced `availability/missing-parser-feature` (471 fails at rank 11), of which lex-tier specifics break down (per just-completed bucket analysis):

- 21 numeric-literal lex errors (`identifier directly after numeric literal`, etc.)
- 8 string-escape lex errors (malformed UTF-8, etc.)
- 5 regex-literal lex errors (unterminated regex, line terminator in regex, missing-from-char-class)
- 1 private-name lex error (expected identifier after `#`)
- 531 post-lex parser errors (HoistableDeclaration-as-Statement-body etc.)
- 562 angle-bracket parser errors (TypeScript/JSX fixtures)

The lexer-tier "parse:" surface in the missing-parser-feature coordinate is small (~35 fails), but that's because cruft's lexer is generally permissive — many lex-tier failures don't surface as parse errors at all; they surface as wrong-value or missing-throw at downstream rungs, hidden inside the broader ast-bytecode-wrong-result (1,244) or wrong-throw (622) coordinates.

The right read: **the lex-tier work-shape isn't "close more parse: errors"; it's "find the wrong-result downstream coordinates whose root cause is at the lex tier and surface them as their own named coordinates."**

## Candidates (5 proposed, ranked by yield-per-LOC × cluster-coherence-multiplier conditions)

### 1. `numeric-literal-conformance` 🟢 RIPE

**Telos**: §12.8 NumericLiteral correctness. Covers numeric separators (`1_000`), BigInt suffix (`123n`), legacy octals (`0777` non-strict), hex/binary/octal literals, decimal-exponent edge cases.

**Pool**: 157 test262 fixtures in `language/literals/numeric/`. Plus the 21 lex-tier "parse:" reasons surfaced by PCR + likely many wrong-value tests hidden in ast-bytecode-wrong-result.

**Cluster-coherence-multiplier conditions** (per BBND findings §IV):
- ✓ C1: directory bijects to §12.8 NumericLiteral (single spec rule)
- ✓ C2: test262 generates many variants per literal form (cross-product)
- ✓ C3: lex-tier one-site implementation in lexer.rs
- ✓ C4: lexer-tier static-semantics (no runtime work)
- ✓ C5: matrix-driven (PCR-EXT 2 surfaced `compile: ...` and `parse: lex error: InvalidNumeric` patterns)

**All 5 conditions hold.** Expected ≥50 tests per locale per the cluster-coherence multiplier.

**LOC estimate**: ~30-50 (numeric lex extension + BigInt-suffix support if missing + separator validation).

### 2. `string-literal-and-escape-conformance` 🟢 RIPE

**Telos**: §12.9 StringLiteral cooked/raw separation, escape decoding (`\u{XXXX}`, surrogate pairs, lone surrogates, hex escapes, line continuations).

**Pool**: 73 in `language/literals/string/` + many wrong-value tests cross-cutting.

**Cluster-coherence-multiplier**: C1+C2+C3+C4 hold; C5 partial (8 visible "parse:" reasons + likely larger wrong-value subset).

**LOC estimate**: ~40-80.

### 3. `identifier-tokenization` 🟢 RIPE

**Telos**: §11.6 IdentifierName + ReservedWord + UnicodeID ranges + had-escape preservation. The `let in` / `break` escaped-keyword surface that the prior parser-permissiveness arc (A3 axis) named is **a tokenization concern**: the lexer must preserve a "had-escape" bit on identifier tokens so the parser's reserved-word gate can reject escaped reserved-words.

**Pool**: 268 in `language/identifiers/` — largest single lex-tier subdir.

**Cluster-coherence-multiplier**: C1+C2+C3 hold (§11.6 spec rule, generated tests, single lex site + parser gate consumers). C4 holds (tokenization). C5 holds (the escaped-keyword tests are well-known matrix-surfaced even if not in current top-30 per name).

**LOC estimate**: ~30-50 LOC for had-escape preservation; + variable for unicode-id range additions.

### 4. `regex-literal-lexing` 🟡 PROBED (composes with regexp-conformance)

**Telos**: §12.9.5 RegularExpressionLiteral lex production. Separate from regex-engine *semantics* — this is just lex-tier acceptance of the pattern + flags + line-terminator rejection inside literal.

**Pool**: ~238 in `language/literals/regexp/`. Composes with regexp-conformance (Tier-E candidate (u)); could be a nested rung inside that locale OR a sibling.

**LOC estimate**: ~20-40 (lexer-side regex pattern accumulator + flag set).

### 5. `private-name-lexing` ⚪ HYPOTHETICAL

**Telos**: §13.3 PrivateIdentifier `#name` tokenization for class private members.

**Pool**: small visible surface (1 in PCR's parse: bucket) but large potential in `language/statements/class/elements/` test262 directories. Cruft's existing LexErrorKind::InvalidIdentifier suggests partial handling; needs survey.

**LOC estimate**: ~30 LOC if existing class machinery accepts the tokens; more if class-elements need broader extension.

### Apparatus-tier candidate (sixth)

### 6. `tokenizer-error-classification-refinement` (apparatus-pilot, sibling to PCR) 🟢 RIPE

**Telos**: extend PCR's categorizer to split the `availability/missing-parser-feature` projection class into lex-tier vs syntax-tier sub-classes. Today these collapse together; sharpening them would name lex-tier substrate work explicitly per the apparatus §XI lexical-grammar coordinate class.

**Composes with**: PCR-EXT 2's missing-lowering-feature pattern — same shape applied at the lex tier.

**LOC estimate**: ~15 LOC in `full_pinart.rs::projection_axis` to add `availability/missing-lex-feature` vs `availability/missing-syntax-feature` discrimination.

## Recommendation (priority-ordered)

1. **`numeric-literal-conformance`** — all 5 cluster-coherence conditions hold; highest yield-per-LOC; clean single-spec-rule directory; small substrate move.

2. **`tokenizer-error-classification-refinement`** — apparatus-tier; ~15 LOC; surfaces named lex-tier coordinates for downstream substrate work. Should land BEFORE candidates 3-5 so they have proper coordinate visibility.

3. **`identifier-tokenization`** — largest test262 sub-directory (268 tests); high yield if cluster-coherence multiplier holds.

4. **`string-literal-and-escape-conformance`** — moderate pool but escape-decoding edge cases are well-defined.

5. Defer `regex-literal-lexing` until regexp-conformance starts (candidate (u) Tier-E); likely a nested rung.

6. Defer `private-name-lexing` until class-elements work is on the critical path.

## Decision points for the keeper

- **Which candidate to spawn first?** Recommendation: tokenizer-error-classification-refinement (apparatus, then) → numeric-literal-conformance (substrate).
- **Add to CANDIDATES.md as Tier I (tokenization-tier substrate) + Tier J (apparatus extension)?** Per the manifest-refresh discipline established by the LPA-EXT 1 sweep.
- **Should regex-literal-lexing nest under regexp-conformance, or stand alone?** Per Doc 737 §II promotion threshold; depends on whether regexp-conformance's first cut needs the lex-tier work or only the engine-semantics work.

Standing by for direction.
