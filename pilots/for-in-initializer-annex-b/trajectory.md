# for-in-initializer-annex-b — Trajectory

## FII-EXT 0 — FOUNDING (2026-05-26)

Spawned per keeper directive (Telegram 9865) from the missing-syntax-feature disambiguation map, alongside HLCL. Keeper said "begin work on the first" — first is HLCL; FII spawned but execution deferred.

Pool: 7 fixtures from `annexB/language/statements/for-in/` + sibling dirs, all emitting `parse: expected Semicolon` because cruft's for-head parser treats `var X = init in obj` as a classic for-statement and bails at the missing `;`.

Baseline: 0/7 PASS. Verified probe:
```js
(function() {
  var effects = 0;
  for (var a = ++effects in {});
  assert.sameValue(effects, 1);
})();
```
cruft: `parse: expected Semicolon @byte5994`.

## FII-EXT 1 — LANDED (2026-05-26)

### Edits (~45 LOC in stmt.rs::parse_for_statement)

1. **Initializer parse under [-In]**: when parsing `var X = Initializer`, set `Parser::in_disallowed = true` around the AssignmentExpression parse so that `in` remains a separator token rather than being consumed as a RelationalExpression operator. Restore the prior value after.

2. **Annex-B carve-out branch**: after the Initializer is parsed, if `init.is_some() && kind == Var && !strict_mode && self.is_ident("in")`, emit the for-in via a Block-rewrite:
   ```
   { var a = init; for (a in right) body }
   ```
   This lowers as:
   - `Stmt::Variable { var a = init }` — hoists `a` + evaluates Initializer once.
   - `Stmt::ForIn { left: ForBinding::Pattern(Identifier(a)), right, body }` — bare-name for-in (no re-var).

   The Block reuses the existing var-hoist + for-in paths at runtime unchanged.

### Probes (Rule 23 verification at landing)

- `for (var a = ++effects in {}) {…}; console.log(effects, a)` → `1 1` ✓ (init evaluated once, a available after)
- `for (var a = ++effects in {x:1,y:2}) console.log(a)` → `x` `y` `done 1` ✓
- `"use strict"; for (var a = 0 in {})` → REJECTS ✓ (strict gate)
- `for (let a = 0 in {})` → REJECTS ✓ (let not in carve-out)
- `for (var i = 0; i < 3; i++) console.log(i)` → `0 1 2` ✓ (classic for unaffected)

### Yield

- **Canonical FII surface** (`annexB/language/statements/for-in/*.js`, 7 files): **0 → 6/7 PASS (+6, 86%)**.
- The 7th (`bare-initializer.js`) is a negative test for `for (a = 0 in {})` (bare assignment, not var-decl) which expects parse-time SyntaxError; cruft currently accepts and runs $DONOTEVALUATE. That's an expression-headed for-in path issue, not the var-decl path FII targets. Follow-on candidate: `for-in-bare-assignment-rejection` (parser tightening).
- Diff-prod: 42/42 maintained.
- Cross-locale regression sweep (8 locales): all unchanged.

### Findings

**Finding FII.1 (Annex-B carve-outs at the parser must rewrite, not just accept)**: A pure parser accept (emit `Stmt::ForIn` with the initialized declarator as LHS) would have left the initializer unevaluated — runtime ForIn lowering doesn't consume Decl-with-init. The Block-rewrite lowering (hoisted var-decl + bare-name for-in) reuses two existing runtime paths and avoids a new emission. Standing recommendation: when an Annex-B parser carve-out adds an alternate grammar that the existing runtime can already lower, prefer a parser-side rewrite into existing AST shapes over adding a new AST variant.

**Finding FII.2 (Initializer-precision around [+In]/[-In] is essential)**: The first cut omitted the `[-In]` toggle around the Initializer parse; the AssignmentExpression consumed `init in obj` as a RelationalExpression, so the carve-out branch never saw `in`. Standing recommendation: any parser-tier carve-out that depends on a specific keyword being visible at lookahead must coordinate with the [+In]/[-In] state — set `in_disallowed = true` around the relevant subparse.

### Status

FII-EXT 1 CLOSED. 6/7 on canonical surface. Residual is `bare-initializer.js` (expression-headed for-in not in FII scope); follow-on `for-in-bare-assignment-rejection` left as a candidate.
