# parser-precedence-in-flag — Seed

## Spinoff from LGSS-EXT 3's irreducible-carriers analysis (Finding LGSS.5).

The LGSS locale closed with two irreducible carriers within tokenization-coordinate scope: `enter_template_tail` (forced by lexer byte-boundaries) and `rewind_lexer_to` (forced by single-token lookahead + missing [In] grammar parameter). The latter has a spec-aligned resolution outside LGSS scope: thread ECMA-262's `[+In]` / `[-In]` grammar parameter through the precedence climber. This locale is that spinoff.

The keeper's conjecture (Telegram 9794): the LGSS simplification will amortize across the engine in downstream tiers. This locale tests that conjecture at the precedence-climber tier — the natural next instance of "name an implicit grammar parameter as parser state at the right tier; eliminate surface contamination at downstream sites."

## Telos

Materialize the engine-DAG coordinate

```
tokens-to-AST / parser-form ::
  E1/algorithm-step:syntactic-grammar ::
  cut/grammar-parameter-as-parser-state ::
  property/for-head-LHS-natural-parse-without-rewind
```

The induced property is that for-head LHS parsing **succeeds on first attempt** for every shape the spec admits — no optimistic-bump-then-rewind, no bare-ident fast-path. The precedence climber refuses to consume `in` as a RelationalExpression operator when it sits inside a for-head LHS, because `[-In]` is set in the parser's state.

ECMA-262 §13.10 RelationalExpression production: `RelationalExpression[In, Yield, Await]` is parameterized by the `[+In]` / `[-In]` flag. §13.7.5 ForStatement uses `Expression[~In, ?Yield, ?Await]` in the LHS position of for-in / for-of. The parameter exists in the spec; its threading into the precedence climber is the substrate's articulation.

## Apparatus

- `pilots/rusty-js-parser/derived/src/parser.rs:27-65` — existing parser-state fields (strict_mode, in_generator, in_function_params). New field `in_disallowed: bool` joins this set; init false.
- `pilots/rusty-js-parser/derived/src/expr.rs:154` — `parse_binary_expression(min_prec)` precedence climber. New site: check `self.in_disallowed` before treating `in` as RelationalExpression at precedence 10.
- `pilots/rusty-js-parser/derived/src/expr.rs:199-200` — operator table; entry for `in` is the discrimination site.
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` — the for-head LHS site that should set `[-In]` for the duration of LHS parsing, then restore.
- `pilots/rusty-js-parser/derived/src/stmt.rs:1255+` — the bare-ident fast-path that becomes deletable once `[-In]` propagates correctly. Also: the rewind site at the fast-path bail.

## Methodology

Three rungs, mirroring LGSS's constraint-stack shape.

### Rung 1 — `in_disallowed` parser-state field (PPIF-EXT 1)

Add `pub(crate) in_disallowed: bool` to Parser; init false. Add `save_in_disallowed()` / `restore_in_disallowed()` save-restore helpers (or use a RAII guard struct). Set true around for-head LHS parsing. Precedence climber checks the field; treats `in` as a non-operator (terminating the binary-op chain) when set.

### Rung 2 — eliminate the for-head bare-ident fast-path + rewind (PPIF-EXT 2)

With `[-In]` set during for-head LHS parsing, `parse_expression` naturally refuses `id in obj` as a RelationalExpression and returns the bare ident as the parsed LHS. The check for `in` / `of` after the parsed expression then succeeds on first attempt. The fast-path + its rewind become deletable.

### Rung 3 — audit other for-* positions that use [-In] (PPIF-EXT 3)

`for ( var/let/const VariableDeclarationList ; ... )` — the var-decl-list also takes `[-In]` per §13.7.5. cruft's existing var-decl parsing currently works (it doesn't go through the binary-op precedence climber for the initializer-RHS at the same shape that risks the `in` confusion), but audit + verify.

Also: any other parser context that should disallow `in` as a binary op? Per spec, `[~In]` propagates from for-head into AssignmentExpression initializer parsing. Most sites are `[+In]` (the default). The audit confirms the scope is bounded.

## Carve-outs

- **`for-await` head LHS** — same shape as for-of, same `[-In]` discipline; covered.
- **Class-element initializers, default parameters** — these are `[+In]` per spec; unchanged by this work.
- **The `instanceof` operator** — separate keyword, separate precedence-table entry; not affected by `[-In]`.

## Composes-with

- `pilots/lexer-goal-symbol-selection/` (LGSS) — sibling at the parser tier; this is LGSS's named spinoff per Finding LGSS.5.
- `apparatus/docs/ecma-conformance-...md` §XI.1.b — names this locale as a candidate that would eliminate the `rewind_lexer_to` irreducible carrier.
- `apparatus/docs/predictive-ruleset.md` — R4 (no half-landed moves), R11 (5-axis pre-spawn check — to be run at PPIF-EXT 1 close), R13 prospective application (C1-C4 check at founding).
- Doc 729 §IV — resolver-instance pattern; this locale extends the "directive-as-parser-state" pattern from goal-symbol-selection (LGSS) to grammar-parameter-threading (PPIF).

## Rule 13 prospective application — C1-C4 at founding

- **C1 (sibling closure pattern)**: HOLDS — LGSS's three rungs are the empirical sibling. Same shape: name an implicit grammar parameter, save/restore around scope-bound positions, eliminate the scattered ad-hoc workarounds at downstream sites.
- **C2 (shape-compat with substrate APIs)**: HOLDS — `in_disallowed` joins three existing parser-state fields of the same shape (strict_mode / in_generator / in_function_params). Save-restore pattern exists; precedence climber's operator table allows in-line predicates.
- **C3 (cost-positive when integrated)**: TO BE VERIFIED at PPIF-EXT 1 — the predicate check at the operator-table entry for `in` is one boolean read per binary-op-position; near-zero. The deleted rewind path is per-for-stmt amortization. Expected positive.
- **C4 (bail safety)**: HOLDS — parse-time discrimination, no runtime divergence.

All four conditions hold. Per R13 prospective, expect ≤3-round closure. PPIF-EXT 1 is the immediate substrate move.

## Empirical anchors

Tests that exercise for-head LHS [-In] discipline (target list at PPIF-EXT 1 close):

- `test262/test/language/statements/for-in/head-expr-*` — for-in with expression heads (bare ident, member, complex)
- `test262/test/language/statements/for-of/head-expr-*` — for-of analog
- Any test that today closes via the bare-ident fast-path; PPIF-EXT 2 makes them close through the normal path
- Regression surface: any test where `in` as binary operator inside an expression-statement still works (e.g., `if (key in obj) ...`) — must remain unaffected

## Predicted yield

The locale closes 0 test262 tests directly (the bare-ident fast-path already passes the tests that would otherwise need [+In]; PPIF lands at structural-cleanliness rather than additional-pass). The yield is **surface contamination eliminated from the for-statement parser site**: the fast-path + its rewind disappear, the for-statement becomes a single-path parser.

The conjecture-test: per LGSS's pattern, expect:
- Net executable LoC at the parser tier: small grow (+ a field, + a guard helper) offset by the deletion of the fast-path + rewind (~30 LOC currently).
- Surface contamination eliminated: stmt.rs:1248-1252 (the fast-path bail + comment about rewind recovery) deleted entirely.
- Downstream tier affected: tokens-to-AST handling of for-statement simplifies; bytecode emission for for-stmt unchanged (the AST shape is preserved; only the parse path changes).

## Resume protocol

Read `trajectory.md` tail.
