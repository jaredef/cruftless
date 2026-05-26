# lexer-goal-symbol-selection — Seed

## Canonical instance of the tokenization-coordinate-shaped locale.

Per the apparatus doc's new §XI.1 (the lexer↔parser feedback edge), the **lexical goal symbol applied at each input position is chosen by the syntactic context**. ECMA-262 §12.9.5 names this explicitly. This locale instantiates the lexical-grammar / tokenization coordinate class against the canonical case: regex/div disambiguation + template-tail re-entry + the implicit constraint that goal-symbol selection should be a single parser-state predicate evaluated at every lex-call boundary, not an ad-hoc decision at scattered call sites.

The keeper's conjecture (Telegram 9784): naming this implicit constraint as a first-class architectural element significantly simplifies construction. The current substrate carries the discipline partially — `bump_regexp` derives goal from `token_completes_expression(prior_token)` — but call-sites that don't go through `bump_regexp` (notably `rewind_lexer_to` recoveries in `stmt.rs:1251` and `refetch_lookahead_with_goal` in `expr.rs:1583`) reveal the gap: rewind is the symptom of "wrong goal chosen, need to redo." Making goal-symbol selection canonical eliminates the rewind class.

## Telos

Materialize the engine-DAG coordinate

```
source-to-tokens / lexical-grammar ::
  E1/lexical-goal-selection ::
  cut/parser-context-conditioned-goal ::
  property/no-rewind-on-goal-mismatch
```

The induced property is **goal-symbol selection as a single named parser-state predicate**, evaluated at every lex-call boundary, with the lexer's per-call API receiving the predicate's output as a directive parameter. The rewind-class of failure mode (lexer tokenized under wrong goal; parser later realizes; rewinds + re-tokenizes) is eliminated because the goal is always derived from the parser's already-committed syntactic state.

## Apparatus

- `pilots/rusty-js-parser/derived/src/lexer.rs:16` — `LexerGoal` enum (InputElementDiv, InputElementRegExp, InputElementTemplateTail). The substrate carrier of the directive parameter.
- `pilots/rusty-js-parser/derived/src/lexer.rs:81` — `next_token(&mut self, goal: LexerGoal)`. The lex-call API that takes the goal as input.
- `pilots/rusty-js-parser/derived/src/parser.rs:847-863` — `bump_regexp`. The site that already derives goal from prior-token completion status via `token_completes_expression`.
- `pilots/rusty-js-parser/derived/src/parser.rs:1006` — `rewind_lexer_to(pos, goal)`. The rewind-class symptom of goal-mismatch.
- `pilots/rusty-js-parser/derived/src/expr.rs:1583` — `refetch_lookahead_with_goal(TemplateTail)`. The template-substitution-close ad-hoc goal switch.
- `pilots/rusty-js-parser/derived/src/stmt.rs:1251` — for-statement rewind to RegExp goal after fast-path bail. The for-statement-context goal switch.

## Methodology

Three rungs, each a Fielding-style constraint added on the prior:

### Rung 1 — Make goal-derivation a single named predicate (LGSS-EXT 1)

Extract `derive_lex_goal(parser_state) -> LexerGoal` as the canonical predicate. Inputs: prior-token kind (for div/regex), current parse context (for template-tail re-entry — true iff we just consumed a `}` inside a TemplateMiddle's substitution). Replace every call-site that constructs a `LexerGoal` ad-hoc with a call to this predicate.

The predicate's body is §12.9.5 made executable:

```rust
fn derive_lex_goal(&self) -> LexerGoal {
    if self.in_template_substitution_close() {
        LexerGoal::TemplateTail
    } else if token_completes_expression(&self.lookahead.kind) {
        LexerGoal::Div
    } else {
        LexerGoal::RegExp
    }
}
```

Closure condition: every `self.lx.next_token(...)` call in the parser receives a goal derived through `derive_lex_goal`, never an inline literal.

### Rung 2 — Make goal-derivation lex-call-boundary-invariant (LGSS-EXT 2)

The lexer's `next_token` becomes `next_token(&mut self)` (no goal parameter). The lex API queries the parser's current state for goal via callback or shared state. The implicit-constraint-named-explicit collapses to one place: the parser is the master of the lex loop and the goal is a function of parser state.

Implementation choice (deferred to LGSS-EXT 2 closure): callback-vs-shared-state-vs-state-machine; see §Potential implementations below.

Closure condition: zero call sites where `LexerGoal` is passed as an argument.

### Rung 3 — Eliminate rewind-class (LGSS-EXT 3)

With goal-derivation canonical and lex-call-boundary-invariant, `rewind_lexer_to` becomes used only for the cases that are genuinely speculative (not goal-mismatch). The for-statement bail at `stmt.rs:1251` and the template re-entry at `expr.rs:1583` should both reduce to single-token-fetches with the parser's current goal — no rewind needed.

Closure condition: `rewind_lexer_to` is either deleted (if no remaining users) or restricted to the documented speculative-parse class only (cover-grammar resolution, where the rewind is over MULTIPLE tokens, not one).

## Carve-outs

- **JSX / TSX goals**: not in cruftless's ECMAScript surface; goal-symbol selection for JSX would compose at the parser-context predicate but is out of scope here.
- **Asi as goal-selection**: ASI is a parser-side decision (insert virtual semicolon based on prior-token + LT + grammar rule). It is NOT a goal-symbol decision; it is a downstream token-stream-interpretation decision. Distinct coordinate, separate locale candidate.
- **Numeric-literal lexing**: BigInt suffix, separators, and hex/octal/binary lexing are per-goal token-form decisions, not goal-symbol selection. They live in the lexical-grammar class but are below this locale's telos.

## Composes-with

- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md` §XI + §XI.1 — the coordinate class this locale instantiates.
- `apparatus/docs/agent-feedback-schema.md` — for cross-resolver review when implementation choices land.
- `docs/fca-instances/resolver-instance-directive-free-artifact.md` — the lexer↔parser pair is a resolver-instance pair; the directive parameter resolves the cycle per Doc 729 §IV.
- Doc 729 — resolver-instance pattern; this locale is its instance at the lexer/parser boundary.
- Today's parser arc: ALTA-EXT 1 (NoLT-before-=>) reads against this class because line-terminator tracking is a per-token lexer obligation that the parser consumes; the constraint stack is similar.

## Potential implementations

Three architectural options for Rung 2. Each is a different rendering of the same "goal-as-parser-state" decision; trade-offs are listed.

### Option A — Callback parameter at lex-call site

The lexer receives a closure that captures parser state and returns the goal:

```rust
impl Lexer {
    pub fn next_token<F: FnOnce(&Self) -> LexerGoal>(&mut self, goal_fn: F) -> Result<Token, LexError>;
}

// Parser:
let tok = self.lx.next_token(|_| self.derive_lex_goal())?;
```

- **Constraint added**: the lexer never owns goal-symbol-selection logic; the parser always does.
- **Induced property**: the lexer becomes goal-agnostic; testable in isolation by passing static-goal closures.
- **Trade-off**: a closure per call is allocation-free in Rust (zero-cost) but adds verbosity at call sites.

### Option B — Shared state field on Parser, queried by lexer via reference

The lexer holds a `&Parser` (or a goal-state-holder), reads goal at each `next_token`:

```rust
pub struct Lexer<'src, S: GoalSource> {
    /* ... */
    goal_source: S,
}
trait GoalSource { fn lex_goal(&self) -> LexerGoal; }
impl GoalSource for Parser<'_> { fn lex_goal(&self) -> LexerGoal { self.derive_lex_goal() } }
```

- **Constraint added**: a trait boundary names the goal-source contract; multiple goal-source implementations possible (test harness, IR pipeline, etc.).
- **Induced property**: the lexer is fully decoupled from the parser type; goal-source becomes a swappable resolver-instance directive.
- **Trade-off**: requires borrow-checker discipline (lexer holds shared ref to goal source); restructuring needed.

### Option C — Parser-state field driven by per-token hook

The parser maintains `current_lex_goal: LexerGoal`; updates it via a single `after_bump` hook called on every token consumption:

```rust
impl Parser {
    fn bump(&mut self) -> Result<Token, ParseError> {
        let cur = std::mem::replace(&mut self.lookahead, self.lx.next_token(self.current_lex_goal)?);
        self.after_bump(&cur);
        Ok(cur)
    }
    fn after_bump(&mut self, prev: &Token) {
        self.current_lex_goal = if self.in_template_substitution_close() {
            LexerGoal::TemplateTail
        } else if token_completes_expression(&prev.kind) {
            LexerGoal::Div
        } else {
            LexerGoal::RegExp
        };
    }
}
```

- **Constraint added**: goal-derivation runs exactly once per consumed token; the lex API still takes goal as parameter but the parser owns the maintenance.
- **Induced property**: zero rewind-for-goal-mismatch (goal is always already correct when next_token is called); call sites stay unchanged.
- **Trade-off**: requires that `in_template_substitution_close` be representable as parser state (true with a small depth-stack); easy.

### Option D — Spec-derived table

A static lookup table per (prior_token_kind, parser_context_bits) → LexerGoal. The table is generated from §12.9.5's text and is the executable form of the spec rule.

- **Constraint added**: goal-derivation becomes data, not code; correspondence to §12.9.5 becomes visible.
- **Induced property**: spec correspondence (apparatus doc §VIII) made literal; the table IS the spec rule.
- **Trade-off**: table size + maintenance overhead; harder to express context predicates that don't fit a finite enum.

### Recommendation

**Option C** is the smallest substrate move that delivers Rung 2's property without re-architecting the lexer/parser type relationship. The keeper's conjecture (significant simplification by naming the implicit constraint) is most directly realized by Option C: one new field + one new hook + every existing call site stays. The rewind sites in `stmt.rs:1251` and `expr.rs:1583` collapse to no-ops because the goal is already current by the time the next `bump` fires.

Option D is the strongest spec-correspondence move but adds maintenance. Worth considering as a successor rung after Option C lands and the engagement has empirical evidence that the predicate's surface is finite.

## Empirical anchors (test262 surface)

Tests that exercise lexer-goal-selection directly, gathered for the locale's exemplar suite at trajectory time:

- `test262/test/language/literals/regexp/early-err-*` — regex-literal early errors (only valid under InputElementRegExp goal)
- `test262/test/language/expressions/template-literal/*` — template-tail re-entry through nested substitutions
- `test262/test/built-ins/RegExp/S15.10.1_A1_T*` — regex literal vs division ambiguity at expression positions
- `test262/test/language/expressions/division/*` — division operator post-call-expression / post-identifier
- `test262/test/language/expressions/postfix-increment/regexp-vs-division.js` (and variants) — the canonical disambiguation tests

Pool size estimable from a single grep against the full-suite interpreted.jsonl; expected ~150-300 tests total across the canonical surface. Exemplar suite to be assembled at LGSS-EXT 1 close.

## Resume protocol

Read `trajectory.md` tail (none yet — LGSS-EXT 0 is this seed); read `docs/design/implementation-options.md` (sibling design doc) if/when the implementation choice surfaces a decision-quality question.

## Status

LGSS-EXT 0 FOUNDED. No substrate work yet landed. Locale exists to name the coordinate, articulate the constraint stack, and enumerate the implementation options for the keeper's review of which Rung 2 approach to take.
