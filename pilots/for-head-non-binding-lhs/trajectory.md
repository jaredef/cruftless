# for-head-non-binding-lhs — Trajectory

## FHNB-EXT 0 — founding + R13-prospective-check (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9798) "continue with spin off." Spinoff from PPIF-EXT 1's Finding PPIF.2 (the runtime gap surfaced when PPIF-EXT 1 unblocked the parse path for Member-LHS for-in/of). Third locale in the FCA-amortization spinoff chain (LGSS → PPIF → FHNB), each at a deeper substrate tier than the prior (lexer → precedence-climber → bytecode lowering).

**Empirical anchor at founding**: cruft prints "undefined" for `var o={}; for (o.x in {p:1}) console.log(o.x)`; bun prints "p". Confirmed via probe under both runtimes. The "undefined" output traces to cruft's `ForBinding::Pattern` lowering allocating a local slot for an empty-name Identifier (the FHAPV-EXT 1 None-fallback shape), rather than evaluating the LHS to a Reference and PutValue'ing per iteration.

**Locale founded** with the constraint stack mirroring LGSS+PPIF:

- Coordinate: `ast-to-bytecode/language-lowering :: E2/internal-method:execution-semantics :: value-semantics/wrong-result :: property/for-in-of-non-binding-lhs-puts-to-reference`
- Telos: §14.7.5.6 ForIn/OfBodyEvaluation correctly evaluates non-binding LHS to a Reference and PutValues per iteration; the Member/Call/Parenthesized-LHS shape unblocked by PPIF-EXT 1 now binds correctly at runtime.
- Three-rung methodology: FHNB-EXT 1 (AST `AssignmentTarget(Expr)` variant + parser routing), FHNB-EXT 2 (bytecode lowering for the new variant, composing existing assignment-target machinery), FHNB-EXT 3 (extend to ParenthesizedExpression-around-pattern).

**R13 C1-C4 prospective check (per seed §Methodology)**:

- C1 (sibling closure pattern): HOLDS — LGSS + PPIF as parser-tier siblings; FHLA + FHAPV as for-head LHS validity siblings.
- C2 (shape-compat with substrate APIs): HOLDS — AST extension follows existing ForBinding shape; lowering composes existing compile-as-assignment-target machinery.
- C3 (cost-positive when integrated): TBV at FHNB-EXT 2; expected positive (per-iter cost is one assignment evaluation, comparable to bun's reference).
- C4 (bail safety): HOLDS — IsValidSimpleAssignmentTarget check at parse time prevents invalid LHS from reaching the runtime.

All four conditions hold prospectively.

**Status**: FHNB-EXT 0 FOUNDED. Awaiting FHNB-EXT 1 substrate move (AST variant + parser routing). The substrate move is multi-tier (AST + parser + bytecode); per R4 the three rungs should land as a coherent unit (no partial commit that leaves the AST extended but lowering unimplemented), so the keeper may want to gate the next move on multi-tier scope review before substrate begins.

**Findings**

**Finding FHNB.0 (the spinoff chain reaches three tiers and ten total LOC of named constraint)**: LGSS named current_lex_goal (1 field) + derive_lex_goal_after (1 predicate) at lexer-tier; PPIF named in_disallowed (1 field) + climber-gate (1 line) at precedence-climber tier; FHNB will name AssignmentTarget(Expr) variant (1 AST shape) + for-in/of-lowering branch (~40 LOC) at bytecode-tier. Each rung adds ONE named carrier and eliminates a class of downstream surface contamination. The keeper's conjecture (Telegram 9794) is being empirically corroborated as a stack: the FCA pattern at any tier surfaces the NEXT tier's named-constraint candidate, and the constraint at THIS tier remains small (1-2 carriers each) only because the upstream tiers have correctly absorbed their own concerns. This is Doc 729's resolver-instance pattern operating reflexively at the engagement-discipline tier.

## FHNB-EXT 1/2 — AssignmentTarget carrier + for-in/of PutValue lowering (2026-05-27)

**Trigger**: Keeper selected the FHNB lane to avoid collision with the
active async-generator / for-await locale.

**Substrate move**:

- Added `ForBinding::AssignmentTarget(Expr)` to the AST. This names the
  spec distinction between a pre-existing binding pattern (`for (x of xs)`)
  and a non-binding assignment target (`for (obj.x of xs)`).
- Parser expression-headed `for-in` / `for-of` now converts:
  - identifiers / array / object cover patterns to `ForBinding::Pattern`,
  - member / parenthesized-member targets to `ForBinding::AssignmentTarget`,
  - invalid non-pattern expressions such as `1` to SyntaxError.
- Bytecode lowering for `for-of` stores the iteration value into a hidden
  temporary, then routes it through the existing `assign_target_from_stack`
  PutValue helper. The fused `ForOfFastNext` path is disabled for assignment
  targets because it writes directly to a local slot and would bypass the
  PutValue bridge.
- Bytecode lowering for `for-in` mirrors the same bridge after `keys[i]`
  is loaded.

**Verification**:

```text
cargo check -p rusty-js-ast -p rusty-js-parser -p rusty-js-bytecode
cargo build -p cruftless --bin cruft
cargo test -p rusty-js-runtime --test for_in t07_member_lhs_assignment_target -- --nocapture
cargo test -p rusty-js-runtime --test iteration 't06' -- --nocapture
```

Direct probes:

```text
var o={}; for (o.x in {p:1}) console.log(o.x); console.log("after", o.x);
=> p / after p

var o={}; for (o.x of [10,20]) console.log(o.x); console.log("after", o.x);
=> 10 / 20 / after 20

var o={a:[]}; var i=0; for (o.a[i++] of [3,4]) {} console.log(o.a[0], o.a[1], i);
=> 3 4 2

for (1 in {a:1}) {}
=> CompileError("parse: Invalid left-hand side in for-in/for-of head ...")
```

`scripts/diff-prod/run-all.sh` returned 41/42 PASS. The single failure was
`async-iteration`, whose fixture prints the expected JSON and then reports an
unhandled promise rejection in the active async-generator/for-await surface:
`TypeError("callee is not callable: undefined [argc=1]")`. That is outside
FHNB's assignment-target carrier; this lane intentionally does not widen into
the async-generator locale.

**Finding FHNB.1 (empty-name fallback retired)**: the old expression-head
fallback converted every non-pattern LHS into `BindingIdentifier { name: "" }`.
That made valid member LHS shapes silently drop writes, and invalid source
shapes parse too far. The new carrier removes both effects: valid non-binding
LHS targets flow to PutValue, invalid non-targets reject at parse.

**Finding FHNB.2 (fast path is not neutral for semantic carriers)**:
`ForOfFastNext` is correct only when the destination is a local slot. Once the
destination is a Reference-producing target, the fused path would skip the
Reference/PutValue operation. The fast path must be conditional on the carrier
shape, not merely on iteration protocol shape.

**Status**: FHNB-EXT 1/2 CLOSED as one coherent multi-tier move. FHNB-EXT 3
remains: parenthesized pattern cover grammar, e.g. `for (({a}) of xs)`, if the
surface is still open after current parser cover-grammar behavior is measured.

## FHNB-EXT 3 — parenthesized-head cover-grammar audit (2026-05-27)

**Trigger**: Continue trajectory after FHNB-EXT 1/2. The seed predicted a
possible follow-on for parenthesized pattern cover grammar:
`for (({a}) of xs)` / `for (([a]) of xs)`.

**Measurement**:

```text
node -e 'var a=0; for (({a}) of [{a:7}]) {} console.log(a);'
=> SyntaxError: Invalid destructuring assignment target

node -e 'var a=0; for (([a]) of [[9]]) {} console.log(a);'
=> SyntaxError: Invalid destructuring assignment target

cruft /tmp/fhnb-paren-object.js
=> CompileError("parse: Invalid left-hand side in for-in/for-of head ...")

cruft /tmp/fhnb-paren-array.js
=> CompileError("parse: Invalid left-hand side in for-in/for-of head ...")

node /tmp/fhnb-paren-ident.js
=> 8

cruft /tmp/fhnb-paren-ident.js
=> 8

cruft /tmp/fhnb-paren-member.js
=> 5
```

**Finding FHNB.3 (seed prediction corrected)**: parenthesized destructuring
patterns are not a valid follow-on target for this locale. The assignment
target audit splits cleanly:

- parenthesized identifier is valid and already works,
- parenthesized member target is valid and works through
  `ForBinding::AssignmentTarget`,
- parenthesized array/object destructuring in this position is invalid and
  cruft already rejects it with the same class of SyntaxError as Node.

The correct closure is therefore no substrate edit. Adding
`Expr::Parenthesized` unwrapping to `expr_to_binding_pattern` here would be a
regression: it would accept syntax the reference engine rejects.

**Status**: FHNB-EXT 3 CLOSED by measurement. Locale FHNB is CLOSED for the
non-binding LHS carrier. Future work, if any, should spawn from measured
for-in destructuring declaration gaps or async-generator/for-await surfaces,
not from FHNB's assignment-target coordinate.
