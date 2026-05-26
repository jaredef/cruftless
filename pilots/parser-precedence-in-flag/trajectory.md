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

---

## PPIF-EXT 1 — in_disallowed parser-state field + climber gate + for-head save-restore (2026-05-25)

**Trigger**: Keeper directive (Telegram 9796) "continue."

**Edits** (~12 LOC across three files):

1. `parser.rs::Parser` — new `pub(crate) in_disallowed: bool` field, init `false` in `Parser::new`.
2. `expr.rs::peek_binary_op` — entry for `in` extended with the `[+In]` gate: `TokenKind::Ident(s) if s == "in" && !self.in_disallowed => Some((BinaryOp::In, 10, false))`. When `in_disallowed` is set, `in` is treated as a non-operator, terminating the binary-op chain.
3. `stmt.rs::parse_for_statement` — at the expression-head LHS site, save/restore `in_disallowed` around the `parse_expression()` call, setting it to `true` for the LHS parse. The fast-path + rewind stay in place for now (PPIF-EXT 2 deletes them).

**Verification (probes)**:

| Probe | Pre-PPIF | Post-PPIF |
|---|---|---|
| `var x; for (x in {a:1,b:2}) ...` (bare ident) | works | works ✓ |
| `var y; for (y of [10,20]) ...` (for-of bare ident) | works | works ✓ |
| `var o={}; for (o.x in {p:1}) ...` (Member LHS) | **PARSE ERROR** ("expected `;`") | **parses** (runtime body-assignment is a pre-existing downstream gap; not PPIF-introduced) |
| `var a,b; for ([a,b] of [[3,4]]) ...` (destructure) | works | works ✓ |
| `"a" in {a:1}` (in as binary, normal expression) | true | true ✓ |
| `0 in [1,2,3]` (in as binary) | true | true ✓ |
| `for (1 in [1]; ;) {}` (in used in for-init expression — should throw) | throws SyntaxError | throws SyntaxError ✓ |
| `for (var i=0; i<2; i++) ...` (plain for) | works | works ✓ |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**
- SyntaxError cluster (45 tests): **45/45 (held)**

**Findings**

**Finding PPIF.1 (substrate WIN, not just structural cleanup)**: pre-PPIF cruft *failed to parse* `for (o.x in {p:1}) ...` because the bare-ident fast-path only handled the bare-ident shape; the expression-head fallback path went through `parse_expression` which consumed `o.x in {p:1}` as a RelationalExpression (under the implicit [+In] default), leaving the for-statement parser unable to find the expected `;`. After PPIF-EXT 1, [-In] is set during for-head LHS parsing; `parse_expression` correctly returns `o.x` without consuming `in`; the for-statement sees `in` as the ForIn keyword. Parse-shape unblocked. The runtime-tier for-in-with-MemberExpression-LHS bug (cruft assigns to o.x but the assignment is not observed by the body) is a pre-existing downstream gap, NOT introduced by PPIF — it's a runtime/lowering coordinate separate from PPIF's parser-tier coordinate.

**Finding PPIF.2 (amortization conjecture corroborated prospectively)**: the keeper's conjecture (LGSS-pattern amortizes across the engine in downstream tiers) lands at PPIF's first rung in a stronger form than predicted at seed time. The seed predicted "0 net test262 tests; the yield is structural-cleanliness." Empirically, PPIF-EXT 1 *unblocks parse shapes that the bare-ident fast-path workaround could not handle* — specifically, Member/Call/Pattern LHS in for-in/of head when the LHS subexpression contains the `o.x` shape that the fast-path bypasses. The naming-at-the-right-tier moves are not only structural; they make new correctness possible at downstream tiers (Member LHS now parses; the runtime bug behind the "undefined" body assignment is the next coordinate, surfaced for downstream work).

**Status**: PPIF-EXT 1 CLOSED. The `[-In]` grammar parameter is materialized as parser state. PPIF-EXT 2 (eliminate the bare-ident fast-path + its rewind, which becomes redundant now) is the next rung.
