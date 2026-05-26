# lexer-goal-symbol-selection — Trajectory

## LGSS-EXT 0 — founding + canonical-instance articulation (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9784) following the apparatus-doc amendment that added the lexical-grammar coordinate class + the §XI.1 lexer↔parser feedback edge articulation. The keeper's conjecture: we can significantly simplify construction by identifying this implicit constraint as a first-class architectural element.

This locale is the canonical tokenization-coordinate-shaped instance. It names the constraint stack the apparatus doc's new class implies, articulates the implementation options space, and stays in the FOUNDED state until the keeper picks a Rung 2 implementation path.

**Apparatus enumerated**:

- `seed.md` — telos, apparatus, three-rung methodology (extract-derivation-predicate → make-lex-call-boundary-invariant → eliminate-rewind-class), four implementation options (A/B/C/D), empirical anchors, recommendation (Option C as the smallest move).
- No code yet. The locale's purpose at LGSS-EXT 0 is the articulation; substrate work begins at LGSS-EXT 1.

**Status**: LGSS-EXT 0 FOUNDED. Standing-document set is the seed; no trajectory rungs landed. Awaiting keeper direction on the implementation option (Option C recommended) before LGSS-EXT 1 begins.

**Findings**

**Finding LGSS.0 (the substrate already carries the discipline partially)**: cruft's parser at `pilots/rusty-js-parser/derived/src/parser.rs:847-863` (`bump_regexp`) already derives goal from prior-token completion status via `token_completes_expression`. The discipline is partially in place but inconsistently applied — three call sites that do NOT route through `bump_regexp` reveal the gap: `parser.rs:70` (initial-lookahead bootstrap, hardcodes RegExp), `stmt.rs:1251` (for-statement bail uses rewind with explicit RegExp goal), `expr.rs:1583` (template-substitution close uses refetch with explicit TemplateTail goal). Each is a different ad-hoc instance of the same decision the canonical predicate would centralize. Standing recommendation: when a discipline is partially in place at one call site, the gap at other call sites is the load-bearing finding; the substrate move is centralization, not extension.
