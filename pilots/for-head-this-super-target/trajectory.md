# for-head-this-super-target — Trajectory

## FHLA-EXT 1 — reject `this`/`super` at for-in/for-of LHS (2026-05-25)

**Trigger**: Top-row inspection cluster A (3 of 8 residual after SBAP-EXT 1). Spec §13.15.1: `this`/`super` are not valid SimpleAssignmentTargets. cruft's bare-ident fast-path at parse_for_statement tokenized `this` as `TokenKind::Ident("this")` and bound it as a BindingIdentifier named "this", silently accepting `for (this of [])`. The expression-head path never saw the expression because the fast-path swallowed it.

**Edits** (~10 LOC across two sites in `stmt.rs::parse_for_statement`):

1. Bare-ident fast-path keyword exclusion (line ~867): extended the exclusion list with `"this" | "super"`. These now fall through to the expression-head path which parses them as `Expr::This`/`Expr::Super`.
2. Expression-head conversion (line ~917): before `expr_to_binding_pattern`, unwrap any `Expr::Parenthesized` layers and reject if the inner expression is `Expr::This`/`Expr::Super`. Returns ParseError with "Invalid left-hand side in for-in/for-of head".

**Verification**:

| Probe | Before | After |
|---|---|---|
| `for (this of [])` | parses (binds opaque "this") | SyntaxError |
| `for ((this) of [])` | parses | SyntaxError |
| `for (this in {})` | parses | SyntaxError |
| `for (o.x of [1,2])` (Member) | parses | parses |
| `for (f().x of [3,4])` (Call.Member) | parses | parses |
| `for (x of [5,6])` (plain ident) | parses | parses |
| test262 SyntaxError cluster | 37/45 | **40/45** (+3) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding FHLA.1 (tokenization-stratum mismatch hides a spec rule)**: `this` is tokenized as `TokenKind::Ident("this")` rather than a distinct keyword token, so any ident-shaped fast-path that does not enumerate-and-reject the reserved-word forms silently mis-binds them. The bug class is symmetric to the SBEA arc but at a different boundary: SBEA caught the strict-mode reserved-word check at parse_binding_identifier; FHLA catches it at the for-stmt fast-path's keyword exclusion list. Standing recommendation: when a token kind is overloaded (Ident-token carrying keyword names), every consumer that pattern-matches on Ident must explicitly enumerate the keyword exclusions for its context; relying on a downstream check is unsafe because the downstream may never see the token.

**Finding FHLA.2 (per-conversion paren-unwrap is the lightest fix)**: `for ((this) of …)` parses through the expression-head path because `(` isn't an ident-token, so the fast-path doesn't fire. The expression-head receives `Expr::Parenthesized{expr: Box<Expr::This>}`. A 3-line while-let unwrap before the assignment-target check covers arbitrary nesting (`for (((this)) of …)`). Cheaper than threading a normalization pass through the AST.

**Status**: FHLA-EXT 1 CLOSED. Cluster 37/45 → 40/45.
