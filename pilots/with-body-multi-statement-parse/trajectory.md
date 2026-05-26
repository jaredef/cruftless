# with-body-multi-statement-parse — Trajectory

## WBMS-EXT 0 — FOUNDING (2026-05-26)

Spawned per keeper directive (Telegram 9855) as Cluster B of Tier K. Sub-cluster of the Punct(RBrace) failure shape inside missing-syntax-feature (264 of the 323 Punct(RBrace) records — drilled by reason text + verified probe).

Baseline: 0/264 PASS. Verified probe shape:
- `with(p){x;}` → parses OK (single line)
- `with(p){\n x \n}` → `parse: unexpected token in expression: Punct(RBrace) @byte11`
- `with({}){}` → parses OK (empty body)

## WBMS-EXT 1 — LANDED (2026-05-26)

### Root cause (Rule 23 verification-probe at founding)

cruft's `with` statement is implemented via `Stmt::Opaque` — the `with` branch in `parse_statement` calls `skip_to_top_terminator()` to byte-skip the entire statement, returning Opaque. `skip_to_top_terminator` tracks paren/brace/bracket depth and bumps tokens until a top-level terminator (`;` at depth 0, EOF, or ASI-fallback on LT-preceded next token at depth 0) closes the statement.

The bug: when `}` decrements `depth_brace` from 1 to 0 (closing the with-body), the code falls through to the ASI fallback check. The fallback fires because:
- depth_paren=0, depth_brace=0 (just dropped), depth_bracket=0
- `}` itself is preceded by LT (when body has `\n` before close)
- start position is the `with` keyword, not the `}`, so the `start != span.start` guard passes
- → BREAK

This breaks BEFORE bumping the `}`, leaving it as lookahead. The outer statement-list loop then tries to parse a new statement starting with `}`, which the expression parser rejects with `unexpected token in expression: Punct(RBrace)`.

### Edit (~12 LOC in stmt.rs::skip_to_top_terminator)

In the `RBrace` match arm, after decrementing `depth_brace`: if all three depths are now 0, this `}` closes the brace-bodied statement. Bump it and return immediately (mirroring the semicolon branch's bump-and-return).

### Probes (Rule 23 verification at landing)

- `with(p){\n console.log(x); \n}` → parses ✓ (no error)
- `with(p){\n if(true){\n console.log(x);\n }\n}` → parses ✓ (nested braces correctly tracked)
- `with(p){console.log(x);}` → parses ✓ (single-line still works)
- `var x=1\n;var y=2;\nconsole.log(x,y);` → `1 2` ✓ (LT-separated stmts at top level unaffected)

### Yield

- WBMS exemplar pool: **0 → 37/264 PASS (+37, 14.0%)**.
- Diff-prod: 42/42 maintained.
- Cross-locale regression sweep:
  - numeric-literal-conformance: 147 (unchanged)
  - identifier-tokenization: 261 (unchanged)
  - string-literal-and-escape-conformance: 59 (unchanged)
  - line-terminator-conformance: 31 (unchanged)
  - hoistable-declaration-as-statement-body: 150 (unchanged)

### Residual decomposition (227 fails)

All residuals are real with-semantics tests — they expect `with(p){x}` to BIND `x` from `p`. cruft's Stmt::Opaque emits a no-op at runtime, so the body never executes. Top shapes:
- 16 `Expected SameValue(0, 2)` — with-scope binding not picking up p.x
- 13 `result === 1. Actual: undefined`
- 11 `result === "value". Actual: undefined`
- 10 `scope.x === 1. Actual: undefined`
- 9-7-6-6 various `pN is not defined` / `scope.x ===` shapes

All collapse to: with-runtime-semantics is unimplemented. Belongs to WBMS-EXT 2 / a separate with-runtime-semantics locale.

### Findings

**Finding WBMS.1 (skip_to_top_terminator's ASI fallback ordering bug)**: The helper's general structure (track depth, fall back to ASI on top-level LT-preceded token) is sound for semicolon-terminated stubs but mis-ordered for brace-bodied stubs. The brace branch decremented depth BEFORE bumping AND BEFORE the ASI check, so the close-brace was visible as "lookahead at depth 0 preceded by LT" — the exact ASI trigger. The fix mirrors the semicolon branch's structure (bump-and-return on completion). Standing recommendation: depth-tracking helpers that mix "skip to terminator" with ASI fallback should treat closing-brace-at-depth-0 the same as `;`-at-depth-0 — both are statement completions, not ASI candidates.

**Finding WBMS.2 (parser-tier fix unblocks parse-only tests; semantics is a separate substrate)**: Same shape as HDSB.1 — a parser-only carve-out closes the parse-failure cluster but does not address the runtime semantics the tests actually probe. The 14% direct-yield here is lower than HDSB's 31.6% because the with-statement pool is dominated by semantic-probing tests; HDSB's pool included many tests probing parser-only behavior of the if-with-function-decl form. Standing recommendation: when scoping a parser-tier locale from a matrix coordinate, estimate direct-yield by the fraction of pool tests that probe parser-only vs runtime-semantic behavior — this fraction can vary 10×.

### Status

WBMS-EXT 1 CLOSED. WBMS-EXT 2 (real with-runtime-semantics — Stmt::With AST + bytecode emission + ScopeChain extension) deferred as a separate substantial-scope locale.

## WBMS-EXT 2 — LANDED (2026-05-26)

### Root cause

WBMS-EXT 1 left `with` as a parser-only stub: `parse_statement` accepted the
surface by returning `Stmt::Opaque`, and the bytecode compiler emitted no body.
That closed the RBrace recovery bug but left semantic tests unchanged. The
next residual layer needed a real statement node and a runtime object
environment stack.

The first implementation pass (`LoadWithName` / `StoreWithName`) closed direct
lookup and simple object writes, but Test262 exposed the deeper PutValue shape:
the assignment reference is fixed at LHS evaluation time. If the RHS deletes
the property, or a getter deletes it during compound assignment, the final
write must still target the originally-resolved object environment record
rather than falling through to a local/global binding.

### Edit

- Added `Stmt::With { object, body, span }` to `rusty-js-ast`.
- Parser now lowers `with (expr) statement` with `parse_with_statement` instead
  of byte-skipping to `Stmt::Opaque`.
- Bytecode gained:
  - `EnterWith` / `ExitWith` for the per-frame object-environment stack.
  - `LoadWithName` / `StoreWithName` for dynamic identifier lookup/store.
  - `ResolveWithName`, `LoadWithNameRef`, `StoreWithNameRef` for
    reference-preserving assignment and compound assignment inside `with`.
- Compiler tracks `with_depth` and emits dynamic identifier bytecodes inside
  with-bodies. Var-hoist collectors recurse through `Stmt::With` bodies.
- Runtime `Frame` gained `with_env_stack`; interpreter lookup walks that stack
  first, then locals, upvalues, globals, and engine helpers.

### Probes

- Direct object lookup/store:
  - `var o = { x: 1 }; var y = 0; with (o) { y = x + 2; x = 7; }`
  - observed `3:7`
- Compound assignment to object environment:
  - `var x = 1; var o = { x: 2 }; with (o) { x += 5; }`
  - observed `1:7`
- Reference-preserving assignment after RHS delete:
  - `var scope = {x: 1}; with (scope) { x = (delete scope.x, 2); }`
  - observed `2`
- Test262 probes:
  - `language/expressions/assignment/S11.13.1_A5_T1.js` PASS
  - `language/expressions/compound-assignment/S11.13.2_A5.10_T1.js` PASS

### Yield

- WBMS exemplar pool: **37 -> 73/264 PASS (+36, 27.7% total)**.
- Build: `cargo check -p rusty-js-runtime` PASS.
- Build: `cargo build --release --bin cruft -p cruftless` PASS.

### Residual decomposition

Remaining 183 FAIL / 8 SKIP are no longer dominated by parser no-op. The visible
families from the first failing rows:

- Proxy `has` trap and ordinary `HasProperty` integration for object environment
  records.
- `@@unscopables` exclusion.
- global-this/global-object aliasing (`this.p1 = 1` does not bind global `p1`
  in the current runtime, so older Sputnik with-global cases remain confounded).
- direct/indirect eval environment-record behavior inside `with`.
- CallExpression base object preservation (`with`-resolved functions need the
  correct receiver).
- destructuring assignment/binding target evaluation order under `with`.
- abrupt completion cleanup for `return` / `break` / `throw` inside a with-body
  is not yet structurally guaranteed by the straight `EnterWith; body; ExitWith`
  lowering.

### Findings

**Finding WBMS.3 (with identifier assignment needs a reference, not just a
late name store)**: Plain and compound identifier assignment inside `with`
cannot be modeled as "evaluate RHS, then look up the name again." Per PutValue,
the reference base is selected when the left-hand side is evaluated. Test262's
delete-during-RHS and getter-deletes-property cases are a precise probe for
this layer. The bytecode tier now carries a narrow reference-preserving path
only for with identifiers, keeping the broader local/global fast paths unchanged.

**Finding WBMS.4 (with semantics opens adjacent environment-record surfaces)**:
Once `with` is no longer a no-op, residuals distribute across real semantic
neighbors rather than one stub: Proxy `has`, `@@unscopables`, global object
binding, eval environment records, call base references, destructuring order,
and abrupt cleanup. Standing recommendation: do not treat WBMS residuals as a
single with bucket from here; choose the next nested locale by the specific
environment-record sub-surface.

### Status

WBMS-EXT 2 CLOSED as a first runtime-semantics rung. Candidate next rungs:
with-unscopables-proxy-has, with-call-base-reference, with-eval-environment,
with-abrupt-cleanup, or global-this-binding.
