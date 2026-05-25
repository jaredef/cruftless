# let-as-bound-name-in-lexical — Trajectory

## LABNL-EXT 1 — `let` forbidden as bound name in let/const decl (2026-05-25)

**Trigger**: NSPS.2 sibling list. test262 `for-in/head-let-bound-names-let.js` + `head-const-bound-names-let.js` (`flags: [noStrict]`, universal rule). Spec §13.3.1.1: "It is a Syntax Error if the BoundNames of LexicalDeclaration contains 'let'."

**Edits** (~30 LOC):

- `stmt.rs`: new `Parser::check_no_let_bound_name(kind, &target) -> Result<(), ParseError>` helper. Walks `target.collect_names()` (reused from PPAE-EXT 3's dup-binding pass). `Var` kind exempt per spec; `Let`/`Const` reject "let" at the first matched leaf.
- 4 call sites wired:
  - `parse_variable_statement` declarator loop (line 253)
  - `parse_for_statement` destructure-head variant (line 651)
  - `parse_for_statement` more-declarators loop (line 723)
  - `parse_for_statement` plain-id head (line 717) — inline check since this path constructs `BindingIdentifier` directly without going through parse_binding_target.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `let let = 1` | accepted | SyntaxError |
| `const let = 1` | accepted | SyntaxError |
| `let [let] = [1]` (destructure leaf) | accepted | SyntaxError |
| `let {a: let} = {a:1}` (renamed leaf) | accepted | SyntaxError |
| `for (let let in {}) {}` (for-in head plain-id) | accepted | SyntaxError |
| `for (const let in {}) {}` | accepted | SyntaxError |
| `var let = 1` (sloppy var still legal) | works | works |
| test262 SyntaxError cluster (45 tests) | 9/45 | **13/45** |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding LABNL.1 (collect_names is the canonical walker for BoundNames-based checks)**: this is the second locale to use `BindingPattern::collect_names()` for a spec early error (PPAE-EXT 3 was first, for dup-binding). The walker correctly visits destructure leaves including renamed object aliases (`let {a: let}` finds the bound `let`, not the key `a`). Standing recommendation: BoundNames-of-Pattern early errors should always use collect_names rather than reimplementing the walk.

**Finding LABNL.2 (for-head plain-id path needs a fourth check site)**: parse_for_statement's plain-id head constructs `BindingIdentifier { name, span }` inline without going through parse_binding_target. This is the same path SBEA-EXT 1 had to special-case. Standing recommendation echo: any new binding-name validation rung must check all four paths — parse_binding_identifier, parse_binding_target, parse_for_statement plain-id, and (added by NSPS-EXT 1) per-param ID parse via parse_function_parameters. Audit recommendation: at the next binding-validation rung, grep for `BindingIdentifier {` inline constructions and confirm each is covered.

**Status**: LABNL-EXT 1 CLOSED. Cluster 9/45 → 13/45.
