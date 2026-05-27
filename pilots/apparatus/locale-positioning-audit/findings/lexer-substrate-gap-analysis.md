# Lexer Substrate Gap Analysis — LPA-EXT 8 derivative

Gap analysis of the tokenization / lexical grammar layer (ECMA-262 §11–§12), the most foundational resolution layer. Every higher layer (parser, bytecode compiler, runtime) depends on correct tokenization. Gaps here propagate upward through the entire engine.

Baseline inputs:

- Lexer source: `pilots/rusty-js-parser/derived/src/lexer.rs` (1,302 lines)
- Token types: `pilots/rusty-js-parser/derived/src/token.rs` (150 lines)
- Diff-prod suite: 112 fixtures, 58 PASS / 54 FAIL
- Test262 matrix: `source-to-ast/parser-early-error` = 903 failures (rank 5)
- Active tokenization locales: 7 (NLC, IDT, SLEC, PNL, LTC, LGSS, LEP)

---

## I. Lexer Architecture Summary

The lexer operates on `&[u8]` (UTF-8 source bytes), emitting `Token` structs with `TokenKind`, `Span`, and `preceded_by_line_terminator` (the ASI signal). The caller selects goal-symbol (`Div` / `RegExp` / `TemplateTail`) at each lex call, per §12.1.

**What the lexer handles today:**

| §12 Production | Status | Notes |
|---|---|---|
| WhiteSpace (§12.2) | Complete | Tab, VT, FF, SP, BOM, Unicode Zs category |
| LineTerminator (§12.3) | Complete | LF, CR, CRLF, U+2028 LS, U+2029 PS (LTC-EXT 1) |
| SingleLineComment | Complete | `//` to LT including LS/PS |
| MultiLineComment | Complete | `/* */` with LT tracking |
| Hashbang | Complete | `#!` at source start |
| IdentifierName (§12.7) | Partial | ASCII correct; non-ASCII permissive (accepts all ≥U+00A0 as ID_Start/ID_Continue instead of checking Unicode tables) |
| UnicodeEscapeSequence in identifiers | Complete | `\uNNNN` and `\u{N...}` with ID_Start/ID_Continue validation |
| NumericLiteral (§12.8) | Complete | Decimal, hex, binary, octal, separators, BigInt, exponent, legacy octal rejection in strict mode |
| StringLiteral (§12.9) | Complete | All escape sequences, hex/unicode escapes, surrogate pair assembly, legacy octal in strict mode, line continuation, JSON-superset (U+2028/U+2029 literal) |
| TemplateLiteral (§12.9) | Complete | Head/middle/tail, cooked+raw, CR normalization, forbidden-escape tagged-template support (cooked=None) |
| RegularExpressionLiteral (§12.9) | Complete | Character class bracket tracking, escape in body, flag collection |
| Punctuator (§12.7) | Complete | All operators including `?.`, `??`, `??=`, `&&=`, `\|\|=`, `>>>` |
| PrivateIdentifier | Complete | `#name` lexing |

---

## II. Confirmed Green (diff-prod PASS)

These lexer subsystems produce byte-identical output to Bun:

| Fixture | Lexer surface exercised |
|---|---|
| `asi-rules` | `preceded_by_line_terminator` bit, ASI-relevant token positions, return+newline, do-while, method chaining |
| `unicode-identifiers` | Non-ASCII identifiers (CJK, Greek, combining marks), `\u{N}` escape in identifiers, computed property with Unicode key |
| `numeric-literals` | All radixes (0b, 0o, 0x), numeric separators, BigInt suffix, exponents, -0, MAX_SAFE_INTEGER |
| `regex-division-ambiguity` | Goal-symbol selection: `/regex/` after keywords vs `/` division after idents/parens/brackets, regex flags |
| `template-literals` | Template head/middle/tail, interpolation, escapes (existing fixture, stable) |
| `expression-precedence` | All punctuator tokens emitted correctly, precedence-sensitive token sequences |
| `string-ops` | String literal content for 3,125 bytes of output (existing fixture, stable) |

**Reading**: the core token emission pipeline is sound. Identifiers, numbers, strings, templates, regex, and punctuators all tokenize correctly for the patterns exercised.

---

## III. Confirmed Gaps (diff-prod FAIL or source-inspection)

### Gap L.1 — Permissive Unicode ID_Start/ID_Continue tables

**Source**: `lexer.rs:1270–1301`

```rust
fn is_id_start(cp: u32) -> bool {
    // ...
    // Permissive: accept any non-ASCII code point as ID_Start. v2
    // will gate against the Unicode ID_Start property table.
    cp >= 0xA0 && ...
}
```

The lexer accepts ALL code points ≥ U+00A0 as IdentifierStart (excluding whitespace, LS, PS, ZWNJ, ZWJ, BOM). The spec requires ID_Start from Unicode's `Derived_Core_Properties` — many non-ASCII code points (e.g., mathematical operators, box-drawing, emoji, private-use-area) should be rejected but are accepted.

**Diff-prod signal**: `unicode-identifiers` PASSES because the fixture uses valid Unicode identifiers (CJK, Greek, combining marks). The gap is invisible for valid programs but would surface as false acceptance of invalid identifiers (parser-permissiveness).

**Test262 signal**: `language/identifiers` = 115 failures, owned by the `identifier-tokenization` locale.

**Propagation**: parser accepts identifiers that should be SyntaxError. Downstream: variable names that are invalid per spec work in cruft but not in strict conformance.

**Locale**: `identifier-tokenization/` — active, aware of this gap. The seed notes the permissive fallback; the locale's substrate work is on ReservedWord exclusion, not on the Unicode table replacement.

### Gap L.2 — String.raw / tagged template raw field

**Source**: `lexer.rs:886–965` (template segment reader)

The lexer correctly populates both `cooked` and `raw` fields on `TokenKind::Template`. The `raw` field preserves backslash-escape sequences verbatim. However, the **bytecode compiler** does not thread the `raw` field through to the runtime's template object — `strings.raw` is `undefined` at runtime.

**Diff-prod signal**: `tagged-template-raw` FAILS — `has_raw: false`, `raw: null`. `string-escapes` FAILS — `String.raw` returns cooked output instead of raw.

**Test262 signal**: embedded in the language-lowering bucket (class of `value-semantics/wrong-result` rows for tagged template tests).

**Propagation**: this is NOT a lexer gap — the lexer produces the raw data. The gap is in the compiler (does not emit the raw array into the template object) and the runtime (does not create the frozen `raw` property on template call-site objects per §13.2.8.3). Classified here because it is the highest-profile gap whose root data originates at the lex tier.

**Locale**: no dedicated locale. Candidate: scope extension of `string-literal-and-escape-conformance/` or a new `tagged-template-object-construction/` locale.

### Gap L.3 — String escape identity-escape passthrough in strict mode

**Source**: `lexer.rs:879`

```rust
_ => out.push(c as char),
```

The final fallback in `read_string_escape` passes through any unrecognized character after `\` as itself. Per §12.9.4.1, in strict mode, the only valid single-character escapes are the named set (`n`, `r`, `t`, `b`, `f`, `v`, `0`, `'`, `"`, `\`) plus hex/unicode forms and line continuation. Any other `\X` (e.g., `\a`, `\q`, `\z`) should be a SyntaxError in strict mode but is silently accepted.

**Diff-prod signal**: `string-escapes` FAILS — the `identity` test shows `\a === "a"` etc. all pass in both engines, but the FAIL is actually on the `String.raw` output (gap L.2 above). The identity-escape gap is invisible in diff-prod because Bun also accepts `\a` in `.mjs` (Bun deviation from spec, or the test exercises sloppy mode via `new Function`). This gap is measurable only via test262 negative tests.

**Test262 signal**: part of the `source-to-ast/parser-early-error` 903-row bucket at rank 7. The `string-literal-and-escape-conformance/` locale tracks this.

**Propagation**: programs with `\a`-style escapes parse successfully when they should throw SyntaxError in strict mode. Benign for ecosystem compat (most real code doesn't use invalid escapes), but a conformance gap.

**Locale**: `string-literal-and-escape-conformance/` — active, covers this gap.

### Gap L.4 — Regex flag validation absent at lex tier

**Source**: `lexer.rs:1023–1029`

```rust
while let Some(c) = self.peek_byte() {
    if is_identifier_part_byte(c) {
        self.pos += 1;
    } else {
        break;
    }
}
```

The lexer collects regex flags as any sequence of `is_identifier_part_byte` characters without validation. Invalid flags (e.g., `/abc/xyz`, duplicate flags `/abc/gg`) are not rejected at lex time. The spec (§12.9.5) defines RegularExpressionFlags as a sequence of `IdentifierPartChar`, with validation deferred to static semantics (§22.2.1.1) — so the lexer's behavior is technically correct per the grammar, but the engine must validate flags somewhere downstream.

**Diff-prod signal**: `regexp-ops` and `regexp-advanced` PASS (valid flags work). `regexp-lookbehind-unicode` FAILS on other grounds (lookbehind not implemented in the regex engine). Invalid-flag rejection is not currently tested by diff-prod.

**Propagation**: if the parser or runtime doesn't validate, invalid flags produce silent wrong behavior or crashes rather than SyntaxError.

**Locale**: `regexp-conformance/` — active, covers regex surface.

### Gap L.5 — Sloppy-mode script-goal lexing not implemented

**Source**: `lexer.rs:9–11`

```
//! Module-only in v1: legacy octal integer/escape sequences and
//! HTML-comment Annex B extensions are rejected outright.
```

The lexer is module-goal-only. Sloppy script-goal features are unavailable:

- HTML-style comments (`<!-- ... -->` and `-->` as single-line comment) per Annex B §B.1.3
- Legacy octal integers (`0777`) accepted in sloppy mode
- `\8` and `\9` non-octal decimal escapes accepted in sloppy mode

The parser works around this via `new Function()` (which compiles as sloppy-mode function code) for some sloppy features, but the lexer itself cannot produce tokens for script-goal source.

**Diff-prod signal**: `directive-prologues` FAILS — sloppy-mode `arguments` aliasing, `eval` var leak. `with-scoping` crashes — `with` is implemented via bytecode ops but the parser path for sloppy-mode `with` may have lexer-adjacent limitations.

**Test262 signal**: `annexB.language` = 734 failures in the language-lowering bucket.

**Propagation**: blocks all Annex B language features, all script-goal test262 tests, and all sloppy-mode-only behaviors.

**Locale**: `annexB-language-semantics` in CANDIDATES.md (not yet founded).

### Gap L.6 — No Unicode property escape support in regex

The lexer emits regex bodies as raw strings (`TokenKind::Regex { body, flags }`). The regex body is not parsed at lex time — it's passed to the runtime's regex engine. However, the runtime regex engine (`pilots/regex-engine-substrate/`) does not support:

- `\p{Letter}`, `\p{Number}`, `\P{...}` (Unicode property escapes, §22.2.1)
- Lookbehind assertions `(?<=...)` and `(?<!...)`
- `hasIndices` flag (`d`) — `match.indices` population

**Diff-prod signal**: `regexp-lookbehind-unicode` FAILS — lookbehind, Unicode property escapes, `hasIndices` all diverge.

**Propagation**: programs using Unicode property escapes or lookbehind get wrong results or regex compilation failures.

**Locale**: `regex-engine-substrate/` — active.

---

## IV. Propagation Map

Each lexer-tier gap propagates through specific higher layers:

```
L.1 (permissive Unicode tables)
  → parser accepts invalid identifiers
    → bytecode compiles invalid bindings
      → test262: 115 parser-early-error fails (identifier surface)

L.2 (strings.raw not threaded to runtime)
  → compiler omits raw array from template object
    → runtime: tagged template strings.raw = undefined
      → String.raw built-in broken
        → diff-prod: tagged-template-raw FAIL, string-escapes FAIL (raw output)

L.3 (identity escape passthrough in strict)
  → parser accepts \a, \q, etc. in strict mode
    → test262: parser-early-error fails (string escape surface)
      → benign for ecosystem (most code avoids invalid escapes)

L.4 (regex flag validation absent)
  → parser does not validate
    → runtime: invalid flags may produce silent wrong behavior
      → test262: regexp conformance fails

L.5 (no sloppy script-goal)
  → all Annex B language features blocked
    → all sloppy-mode-only test262 tests fail
      → diff-prod: directive-prologues FAIL, with-scoping crash

L.6 (no Unicode property escapes / lookbehind)
  → regex compilation failures or wrong results
    → diff-prod: regexp-lookbehind-unicode FAIL
```

---

## V. Existing Locale Coverage

| Gap | Locale | Status | Coverage |
|---|---|---|---|
| L.1 | `identifier-tokenization/` | Active | Covers ReservedWord exclusion; Unicode table replacement deferred to v2 |
| L.2 | (none) | Candidate | Lex tier produces raw data; gap is compiler + runtime threading |
| L.3 | `string-literal-and-escape-conformance/` | Active | Covers legacy octal + strict escape rejection |
| L.4 | `regexp-conformance/` | Active | Covers regex surface broadly |
| L.5 | (candidate) `annexB-language-semantics` | Not founded | Blocks 734 test262 rows |
| L.6 | `regex-engine-substrate/` | Active | Covers regex engine internals |

Additional tokenization locales not gap-linked:

| Locale | Scope | Diff-prod signal |
|---|---|---|
| `numeric-literal-conformance/` | §12.8 NumericLiteral rejection | All numeric-literal fixtures PASS |
| `private-name-lexing/` | PrivateIdentifier lexing | `private-field-encapsulation` exercises this (FAIL is runtime brand-check, not lex) |
| `line-terminator-conformance/` | §11.3 LT at all lex sites | `asi-rules` PASS confirms LF/CR ASI; U+2028/U+2029 in comments/strings handled per LTC-EXT 1 |
| `lexer-goal-symbol-selection/` | §12.1 goal-symbol architecture | `regex-division-ambiguity` PASS confirms correct goal selection |
| `lex-error-propagation-to-eval-surface/` | Lex errors reaching eval catch | `eval-lexical-capture` crash blocks measurement |

---

## VI. Leverage Ranking

Gaps ordered by downstream propagation impact:

1. **L.5 (sloppy script-goal)** — blocks 734 Annex B rows + all sloppy-mode diff-prod failures. Highest row count but architecturally deep (lexer + parser + compiler all assume module-goal).

2. **L.2 (strings.raw threading)** — blocks tagged-template-raw, String.raw, and any tag function that reads `.raw`. Fix is compiler + runtime, not lexer. Medium row count but high ecosystem visibility (String.raw is common).

3. **L.1 (Unicode ID tables)** — blocks 115 identifier test262 rows. Fix is mechanical (import a precomputed Unicode table). Low ecosystem impact (invalid identifiers are rare in real code).

4. **L.6 (regex Unicode property escapes + lookbehind)** — blocks regex conformance rows. Fix is in the regex engine substrate, not the lexer. Medium ecosystem impact (lookbehind increasingly common).

5. **L.3 (strict identity escape)** — blocks parser-early-error rows for string escapes. Low ecosystem impact.

6. **L.4 (regex flag validation)** — low impact; flags are mostly validated by the regex engine at compile time.

---

## VII. Recommendations

1. **Do not found a new lexer locale.** The 6 active tokenization locales cover all identified gaps. The highest-leverage moves are scope extensions, not new spawns.

2. **L.2 (strings.raw) is the highest-leverage fix reachable without architectural changes.** The lexer already produces the data. The fix is threading `raw` through the compiler into the template call-site object, then populating the frozen `.raw` property at runtime. Candidate: scope extension of `string-literal-and-escape-conformance/` or a new compiler-tier locale.

3. **L.5 (sloppy script-goal) requires architectural work.** The lexer, parser, and compiler all assume module-goal. Adding script-goal support is a multi-rung arc, not a single fix. This aligns with LPA-EXT 5 Arc C (Annex B language semantics).

4. **L.1 (Unicode tables) is a clean mechanical fix.** Import `unicode-ident` crate or a precomputed table. The `identifier-tokenization/` locale should absorb this as a scope extension once the ReservedWord exclusion work is complete.
