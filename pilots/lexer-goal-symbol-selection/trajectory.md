# lexer-goal-symbol-selection — Trajectory

## LGSS-EXT 0 — founding + canonical-instance articulation (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9784) following the apparatus-doc amendment that added the lexical-grammar coordinate class + the §XI.1 lexer↔parser feedback edge articulation. The keeper's conjecture: we can significantly simplify construction by identifying this implicit constraint as a first-class architectural element.

This locale is the canonical tokenization-coordinate-shaped instance. It names the constraint stack the apparatus doc's new class implies, articulates the implementation options space, and stays in the FOUNDED state until the keeper picks a Rung 2 implementation path.

**Apparatus enumerated**:

- `seed.md` — telos, apparatus, three-rung methodology (extract-derivation-predicate → make-lex-call-boundary-invariant → eliminate-rewind-class), four implementation options (A/B/C/D), empirical anchors, recommendation (Option C as the smallest move).
- No code yet. The locale's purpose at LGSS-EXT 0 is the articulation; substrate work begins at LGSS-EXT 1.

**Status**: LGSS-EXT 0 FOUNDED. Standing-document set is the seed; no trajectory rungs landed. Awaiting keeper direction on the implementation option (Option C recommended) before LGSS-EXT 1 begins.

---

## LGSS-EXT 1 — extract canonical predicate; Option C parser-state field (2026-05-25)

**Trigger**: Keeper directive (Telegram 9786) "continue with Option C."

**Edits** (~30 LOC in `pilots/rusty-js-parser/derived/src/parser.rs`):

1. New `current_lex_goal: LexerGoal` field on `Parser`. Semantics: the goal to use for the NEXT bump's fetch. Initialized in `Parser::new` via `derive_lex_goal_after(&first_lookahead.kind)` immediately after the bootstrap RegExp fetch.

2. New free predicate `derive_lex_goal_after(prev_kind: &TokenKind) -> LexerGoal`. Wraps `token_completes_expression`; for now returns `Div` when prev completes an expression, else `RegExp`. The canonical site for the parser-context-conditioned goal-symbol decision. TemplateTail re-entry is explicitly carved-out to LGSS-EXT 3 (requires template-substitution-depth state on Parser).

3. `bump_regexp` refactored. Instead of computing goal inline from `self.lookahead.kind` per call, fetch uses `self.current_lex_goal`; immediately after, `current_lex_goal := derive_lex_goal_after(&self.lookahead.kind)`. The invariant: at any moment, `self.current_lex_goal` is the correct goal for the immediately-next bump.

4. `rewind_lexer_to` + `refetch_lookahead_with_goal` updated to maintain the invariant — after explicit re-lex under a caller-provided goal, recompute `current_lex_goal` from the new lookahead. These paths still accept an explicit goal argument because they're recoveries (caller knows the right goal for the recovery context); the invariant is preserved at the boundary.

**Verification** (probes confirming the substrate behavior is unchanged):

| Probe | Expected | Result |
|---|---|---|
| `/abc/.test('abc')` (regex literal in fresh-expr) | true | true ✓ |
| `var a=6, b=2; a/b` (division after identifier) | 3 | 3 ✓ |
| `[1,2,3].length / 3` (division after call/member) | 1 | 1 ✓ |
| `(1+2) / 3` (division after `)`) | 1 | 1 ✓ |
| `` `prefix-${/x+/.source}-suffix` `` (template + regex inside subst + TemplateTail re-entry) | "prefix-x+-suffix" | ✓ |
| `` `v=${1+2}; r=${/a/.test('a')}` `` (multi-subst with regex) | "v=3; r=true" | ✓ |
| `` `outer-${`inner-${1+1}`}-end` `` (nested templates) | "outer-inner-2-end" | ✓ |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**
- SyntaxError cluster (45 tests): **45/45 (held)**

**Findings**

**Finding LGSS.1 (named parser-state replaces scattered call-site derivation; no behavior change)**: the refactor consolidates the implicit constraint (goal-symbol selection is a function of prior token's expression-completion status) from one site (bump_regexp's inline derivation) to one named field (current_lex_goal) maintained by one named hook (after-bump derivation via the canonical predicate). All other goal-symbol uses in the parser flow through bump_regexp; the inline computation moved to a state-field update, preserving every per-call observable. Standing recommendation: when a discipline is partially in place at a single site, the substrate move is the addition of the state field that names the discipline, not the extension of the discipline to other sites — the other sites already share the discipline via the centralized path.

**Finding LGSS.2 (gates-green confirms the implicit constraint was correctly named)**: that diff-prod / random-300 / SyntaxError-cluster all hold identical numbers post-refactor empirically confirms that `derive_lex_goal_after` captures every case the previous inline `if token_completes_expression(&self.lookahead.kind) { Div } else { RegExp }` captured, and the parser-state invariant (`current_lex_goal` is always-current) holds across all observed call paths. The construction simplification predicted at LGSS-EXT 0 lands without behavior change — exactly what naming the implicit constraint should produce.

**Status**: LGSS-EXT 1 CLOSED. The canonical predicate exists; the parser-state field carries the invariant. LGSS-EXT 2 (eliminate explicit goal arguments at lex-call API) and LGSS-EXT 3 (eliminate rewind-class + fold TemplateTail into the predicate via template-substitution-depth state) are the successor rungs.

---

## LGSS-EXT 2 — eliminate LexerGoal from parser-tier method boundaries (2026-05-25)

**Trigger**: Keeper directive (Telegram 9788) "continue with ext 2."

**Edits** (~25 LOC in `parser.rs` + 1-line update at each of two call sites):

1. `refetch_lookahead_with_goal(goal: LexerGoal)` → **renamed** to `enter_template_tail()`. The new method takes no argument, hardcodes `LexerGoal::TemplateTail` internally, and is named after the parser-tier intent (entering TemplateTail at a template-substitution close). The goal becomes implicit in the call name — only one place in the parser legitimately re-enters TemplateTail, so the rename is unambiguous.

2. `rewind_lexer_to(pos: usize, goal: LexerGoal)` → **signature simplified** to `rewind_lexer_to(pos: usize)`. The goal is hardcoded to `LexerGoal::RegExp` internally. Rewind sites are by construction rewinding to a fresh-expression-head position (the only existing site, `stmt.rs:1251`, rewinds to before a previously-bumped identifier inside a for-paren — a primary-expression position where RegExp is the spec-correct choice).

3. Call-site updates:
   - `pilots/rusty-js-parser/derived/src/stmt.rs:1251` — `self.rewind_lexer_to(saved_pos_before_id)` (was `..., LexerGoal::RegExp`).
   - `pilots/rusty-js-parser/derived/src/expr.rs:1583` — `self.enter_template_tail()` (was `self.refetch_lookahead_with_goal(LexerGoal::TemplateTail)`).

4. The doc-comment on `derive_lex_goal_after` updated to reference the new `enter_template_tail` name.

After LGSS-EXT 2, the only sites in cruft that mention `LexerGoal` outside of `lexer.rs` itself are:

- `Parser::new` bootstrap (one fetch, the legitimate "no prior context" base case using `LexerGoal::RegExp`).
- Three internal methods on `Parser` (`bump_regexp`, `enter_template_tail`, `rewind_lexer_to`) that consult `LexerGoal` themselves but do not require callers to.
- The `LexerGoal` import in `parser.rs` (necessary for the bootstrap + internal methods).

No external (non-parser-crate) caller constructs a `LexerGoal` literal. The parser-tier method boundary is lex-call-boundary-invariant per Rung 2's telos: callers express intent (rewind here, enter template tail) and the parser maps intent to the goal directive.

**Verification**:

| Probe | Result |
|---|---|
| All 8 LGSS probes from EXT 1 (regex literals, division forms, templates with regex/multi-subst/nested) | identical output (held) |
| diff-prod | **42/42 PASS, 0 FAIL** |
| random-300 prev-PASS | **300/300, 0 regressions** |
| SyntaxError cluster | **45/45 (held)** |

**Findings**

**Finding LGSS.3 (intent-named methods retire goal arguments)**: the two parser-tier methods that previously accepted `LexerGoal` arguments now express their parser-tier intent in their names (`enter_template_tail`, `rewind_lexer_to`). The goal becomes a consequence of the intent, not a parameter to it. The lexer's low-level API (`Lexer::next_token(goal)`) still accepts a goal — that's the correct tier for a directive parameter. Standing recommendation: when a low-level resolver-instance accepts a directive parameter (per Doc 729 §IV), the higher-tier methods that wrap it should express callers' intent and translate intent to directive internally — exposing the directive at the high tier leaks resolver-instance-tier concerns upward.

**Finding LGSS.4 (a single-instance method-name carries the goal unambiguously)**: `enter_template_tail` works as an intent-named method because there is only ONE legitimate use case for TemplateTail re-entry in the parser (close of a template substitution). When a rare goal has only one legitimate consumer, the consumer's method name IS the goal-symbol carrier. Standing recommendation: count consumers before extracting a goal parameter; if N=1, the consumer method's name is the better carrier than the parameter.

**Status**: LGSS-EXT 2 CLOSED. Parser-tier boundary is lex-call-goal-invariant. LGSS-EXT 3 (eliminate the rewind site and fold TemplateTail into derive_lex_goal_after via Parser-side template-substitution-depth) is the final cleanup rung; reduces the apparatus to one named predicate at one named hook with zero special-case methods.

---

## LGSS-EXT 3 — analysis closure: two carriers are irreducible within LGSS scope (2026-05-25)

**Trigger**: Keeper directive (Telegram 9790) "continue."

**Posture**: No code changes. EXT 3 was sketched at LGSS-EXT 0 as "eliminate the rewind site + fold TemplateTail into derive_lex_goal_after." On close investigation, both reductions require architectural moves outside LGSS's tokenization-coordinate scope. Per standing rule 4 (no half-landed moves), this trajectory entry closes the locale with an honest analysis rather than landing a partial reduction that wouldn't yield.

### Why `enter_template_tail` cannot fold into `derive_lex_goal_after`

The proposed fold: track `template_subst_depth` on Parser; in `derive_lex_goal_after`, return TemplateTail when depth>0 AND lookahead is RBrace; the next `bump` automatically re-lexes the would-be-RBrace as Template{Middle/Tail}.

The structural obstruction: cruft's lexer emits Template{Middle/Tail} **starting at the `}` byte itself**. The `}` is the leading delimiter of a TemplateMiddle/Tail token, not its predecessor. If the parser bumps the RBrace and then asks the lexer to fetch a Template{Middle/Tail} from the AFTER-`}` position, the lexer emits a Template token without its leading `}` — wrong shape, wrong cooked-string boundaries.

The only correct sequence is: while lookahead is RBrace (in template-substitution-close context), **re-lex at the same byte position** under TemplateTail goal. The re-lex replaces the RBrace lookahead with a Template{Middle/Tail} token; the parser then bumps that. `enter_template_tail` is exactly this operation.

The re-lex-at-same-position is structurally distinct from bump's normal forward fetch. It cannot be performed as a side-effect of `after_bump` because `after_bump` runs AFTER the bump has already replaced the lookahead via forward fetch. The decision-and-action ordering is: "while inspecting the current lookahead (still RBrace), conditionally re-lex at its position before any further bump."

Folding `enter_template_tail` into `derive_lex_goal_after` therefore requires either (a) restructuring the lexer's TemplateMiddle/Tail emission to not include `}` as the leading byte (deep lexer change; affects raw/cooked-string byte boundaries), or (b) introducing a "pre-bump hook" that runs before bump's forward fetch and can re-lex the current lookahead (deep parser-machinery change). Both are outside LGSS scope.

### Why `rewind_lexer_to` cannot be eliminated without a non-LGSS substrate move

The proposed elimination: don't optimistically bump the bare-identifier at `stmt.rs:1251`; instead, peek-without-bumping, then bump only after the `in`/`of` is confirmed.

The structural obstruction: cruft has one-token lookahead. The bare-ident fast-path needs to inspect the token AFTER the ident to decide between fast-path (ident IS the for-binding) and slow-path (ident is the start of a more complex expression). Without two-token lookahead, the parser must commit to bumping the ident before it can see what follows. If the follow turns out to be wrong, the rewind is the recovery.

The spec-aligned alternative is the [+In] grammar parameter (§13.7.5 ForStatement). With [+In] threaded through the precedence climber, `parse_expression` inside a for-head's LHS position would refuse to consume `id in obj` as a RelationalExpression; the bare-ident-then-`in` shape would parse naturally without the fast-path. But [+In] threading is its own architectural move:

- Touches every parser entry that takes precedence: `parse_assignment_expression`, `parse_conditional_expression`, the binary-op climber.
- Adds an `in_disallowed: bool` state-field or method-parameter.
- Affects parse-error messages at the in-disallowed boundary.
- Has its own correctness surface (every `in` usage cross the parse).

This is the right substrate move long-term; it's outside LGSS's tokenization-coordinate scope and warrants its own locale.

### Spinoff locale candidate

**`parser-precedence-in-flag`** — implement the spec's `[+In]` / `[-In]` parameter on AssignmentExpression / ConditionalExpression / BinaryOpClimber productions. Telos: eliminate the bare-ident-for-head fast-path + its rewind, plus close any other sites where the precedence climber's lack of [In] tracking forces ad-hoc workarounds. Pre-spawn check: enumerate every cruft-parser site that conditionalizes on whether `in` is a binary op vs a for-binding keyword; sample 5-10 cases; confirm shared mechanism per heuristics §V. CANDIDATES queue at apparatus/locales/.

### What LGSS reduces the apparatus to

After EXT 1 + EXT 2:

| Surface | Mechanism | Count |
|---|---|---:|
| Named predicate for Div/RegExp | `derive_lex_goal_after` free function | 1 |
| Parser-state field maintaining current goal | `Parser::current_lex_goal` | 1 field |
| Per-bump hook that updates the state field | inline tail of `bump_regexp` | 1 hook |
| Intent-named methods for special re-lex | `enter_template_tail` + `rewind_lexer_to` | 2 methods |
| External callers constructing LexerGoal literals | (none outside parser crate) | 0 |
| Bootstrap fetch with explicit RegExp goal | `Parser::new` (legitimate "no prior context" base case) | 1 |

This is the minimum the lexer↔parser feedback edge can be reduced to within cruft's current parser architecture. The two intent-named methods (`enter_template_tail`, `rewind_lexer_to`) are the irreducible structural carriers per the analyses above. Their irreducibility is genuine (not laziness); each has a documented obstruction whose removal is its own substrate work.

**Finding LGSS.5 (structural irreducibility within scope marks the locale boundary)**: a tokenization-coordinate locale's closure condition is reduction to the minimum mechanism set; when remaining carriers are blocked by orthogonal substrate constraints (here: lexer byte-boundaries for templates, single-token lookahead for the for-fast-path), the honest closure documents the obstructions, surfaces the spinoff locale candidates that would address them, and marks the locale CLOSED. Standing recommendation: pre-spawn checks (rule 11) should include "what carries the locale's scope, what borders sit on other locales' scopes"; locale boundaries are themselves coordinates worth naming.

**Finding LGSS.6 (the §XI.1 back-edge has irreducible carriers; the apparatus doc should reflect this)**: the apparatus-doc amendment at §XI.1 articulated the lexer↔parser back-edge as resolved by goal-as-directive-parameter, with the resolver-instance discipline making each lex call stage-deterministic. The empirical close of LGSS shows the resolution holds at the bump-loop tier but TWO sites legitimately carry the back-edge via intent-named methods (template re-entry; for-fast-path bail). A follow-on apparatus-doc edit could refine §XI.1 to name these as the bounded irreducible carriers, distinguishing them from the unbounded ad-hoc goal-passing the amendment was originally guarding against. (Pending keeper review; not landed unilaterally per the apparatus tier-separation discipline.)

**Status**: LGSS-EXT 3 CLOSED (analysis only; no code). **Locale LGSS CLOSED** after 3 rungs. The lexer↔parser feedback edge in cruft is now expressed as one named predicate + one named field + one named hook + two intent-named special-case methods, with no external callers constructing LexerGoal literals. The construction simplification predicted at LGSS-EXT 0 (keeper's conjecture) lands fully: the implicit constraint is named, the carriers are bounded, the irreducible-carrier obstructions are documented and surface as spinoff locale candidates.

**Findings**

**Finding LGSS.0 (the substrate already carries the discipline partially)**: cruft's parser at `pilots/rusty-js-parser/derived/src/parser.rs:847-863` (`bump_regexp`) already derives goal from prior-token completion status via `token_completes_expression`. The discipline is partially in place but inconsistently applied — three call sites that do NOT route through `bump_regexp` reveal the gap: `parser.rs:70` (initial-lookahead bootstrap, hardcodes RegExp), `stmt.rs:1251` (for-statement bail uses rewind with explicit RegExp goal), `expr.rs:1583` (template-substitution close uses refetch with explicit TemplateTail goal). Each is a different ad-hoc instance of the same decision the canonical predicate would centralize. Standing recommendation: when a discipline is partially in place at one call site, the gap at other call sites is the load-bearing finding; the substrate move is centralization, not extension.
