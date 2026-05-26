# parser-precedence-in-flag — Trajectory

## PPIF-EXT 0 — founding + R13-prospective-check (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9794) testing the conjecture that the LGSS simplification pattern amortizes across the engine in downstream tiers. PPIF is the spinoff named at LGSS-EXT 3 + folded into the apparatus doc at §XI.1.b as the candidate that would eliminate the `rewind_lexer_to` irreducible carrier.

**Locale founded** with the constraint stack mirroring LGSS:

- Coordinate: `tokens-to-AST / parser-form :: E1/algorithm-step:syntactic-grammar :: cut/grammar-parameter-as-parser-state :: property/for-head-LHS-natural-parse-without-rewind`
- Telos: thread ECMA-262's `[+In]` / `[-In]` grammar parameter through the precedence climber so for-head LHS parsing succeeds on first attempt without the bare-ident fast-path + rewind.
- Three-rung methodology: PPIF-EXT 1 (add `in_disallowed` parser-state field + save-restore + climber check), PPIF-EXT 2 (eliminate bare-ident fast-path + rewind), PPIF-EXT 3 (audit other for-* positions taking `[-In]`).

**R13 C1-C4 prospective check (per seed §Methodology)**:

- C1 (sibling closure pattern): HOLDS — LGSS's three rungs are the empirical sibling. Same shape applied to a different implicit grammar parameter.
- C2 (shape-compat with substrate APIs): HOLDS — `in_disallowed` joins strict_mode / in_generator / in_function_params (same shape; existing save-restore pattern).
- C3 (cost-positive when integrated): TBV at PPIF-EXT 1; expected positive (predicate is one boolean per binary-op-position; near-zero) with amortizing per-for-stmt cleanup yield.
- C4 (bail safety): HOLDS — parse-time discrimination, no runtime divergence.

All four conditions hold prospectively. Per R13 thirteenth-corroboration discipline, expect ≤3-round closure.

**Status**: PPIF-EXT 0 FOUNDED. Awaiting PPIF-EXT 1 substrate move (the named-field + climber-check edit).
