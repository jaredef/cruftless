# Deletions Ledger

A standing apparatus-tier record of substrate deletions that reduced complexity, surface contamination, or carrier count without semantic loss. Per keeper directive (Telegram 9800): **deletions are as important as additions for maintaining legibility**. The cybernetic loop has long treated additions as the primary substrate move; this ledger makes deletions equally legible.

## Why a ledger

Cruftless's apparatus has rich machinery for tracking new substrate work (locale seeds + trajectories + manifest + standing-rule additions + findings.md). It has nothing for tracking what got REMOVED. The asymmetry is real and corrosive:

- An addition can be re-read against its trajectory entry to understand the move.
- A deletion appears in git history but loses its trajectory binding once the code is gone. The "why this is OK to remove" reasoning vanishes with the lines.
- A future resolver looking at a simpler-than-expected codebase may not see WHICH constraint, named upstream, made the deletion safe.

This ledger restores the binding. Each entry records:

1. **What was deleted** — file + line range (at deletion commit's parent) + brief shape description
2. **Net LoC delta** — additions vs deletions in the commit
3. **What named constraint made the deletion safe** — the upstream substrate move (named field, named predicate, named hook, named rule) whose presence makes the deleted code redundant
4. **What was simplified by the deletion** — the call-site / coordinate / tier / cluster that became cleaner
5. **Test surface that confirms safety** — gates re-verified + any new probe that confirms the deletion is behavior-preserving

Deletions that are not behavior-preserving (intentional behavior changes, deprecations, revert-then-deeper-layer per rule 13) are recorded separately or in their own locale's trajectory; this ledger is specifically for **constraint-induced deletions**: the named-upstream-constraint pattern made redundant something downstream, and the downstream thing got removed.

## Discipline (append-only)

Per Doc 727 §X basin-stability discipline (same as findings.md): this file is **append-only**. New entries go at the bottom in chronological order. Older entries are never edited; if a deletion turns out to have been wrong, the revert lands as a NEW entry citing the prior with a back-reference. The reviewer's read at the moment of deletion is the artifact.

Entry format:

```markdown
## YYYY-MM-DD — <SHORT-TAG>: <one-line description>

**Commit**: <hash>
**Files**: <path:lines-range, …>
**Net LoC delta**: <+adds / -dels = net>
**Named constraint that made deletion safe**: <constraint name + locale + brief why>
**What was simplified**: <call-site / coordinate / tier / cluster description>
**Gates re-verified**: <list>
**Composes-with**: <related locales / rules / findings>

<2-3 sentence explanation of the deletion's structural shape>
```

---

## Entries

### 2026-05-25 — PPIF-EXT 2: bare-ident for-head fast-path + rewind_lexer_to deletion

**Commit**: (this commit, follows from 1e3ed361 PPIF-EXT 1)
**Files**: `pilots/rusty-js-parser/derived/src/stmt.rs:1178-1254` (77 lines fast-path), `pilots/rusty-js-parser/derived/src/parser.rs::rewind_lexer_to` (18-line method body)
**Net LoC delta**: +47 / -95 = **-48 net**
**Named constraint that made deletion safe**: `Parser::in_disallowed` (PPIF-EXT 1) — the spec's `[+In]`/`[-In]` grammar parameter as parser state, threaded `[-In]` around for-head LHS parse so the precedence climber refuses to consume `in` as a RelationalExpression operator
**What was simplified**: `parse_for_statement`'s expression-head path is now the only path (the fast-path's optimistic-bump-and-rewind dual-path structure collapses to single-path). The `rewind_lexer_to` carrier — one of the two intent-named irreducible carriers documented at apparatus doc §XI.1.b post-LGSS — is eliminated entirely (zero callers).
**Gates re-verified**: diff-prod 42/42 PASS; random-300 prev-PASS 300/300 (0 regressions); SyntaxError curated cluster 45/45 (held)
**Composes-with**: PPIF-EXT 1 (the named-constraint move that made this deletion safe); LGSS-EXT 3 §XI.1.b irreducible-carriers analysis (predicted this deletion as the spinoff's natural endpoint); FAOF-EXT 1 (the `async of` lookahead check; relocated to the expression-head path); FHLA-EXT 1 (the `this`/`super` rejection; already covered by the expression-head path's existing logic)

The fast-path existed because `parse_expression` under the implicit `[+In]` default consumed `id in obj` as a RelationalExpression, tripping `expected ';'` in the for-statement parser. The workaround was to optimistically bump the identifier, peek for `in`/`of`, and rewind on miss. PPIF-EXT 1 named the missing grammar parameter as parser state; with `[-In]` threaded around for-head LHS parsing, the precedence climber refuses to consume `in`, the LHS parse returns the bare ident, and the for-statement sees `in` as the ForIn keyword on first try. The workaround becomes structurally redundant; deletion is behavior-preserving. The lexer↔parser feedback edge's intent-named carrier count drops from 2 to 1 (only `enter_template_tail` remains; see apparatus §XI.1.b).

---

### 2026-05-25 — LGSS-EXT 2: `LexerGoal` arguments at parser-tier method signatures

**Commit**: 253c26f5
**Files**: `pilots/rusty-js-parser/derived/src/parser.rs` (signature changes to `rewind_lexer_to` and `refetch_lookahead_with_goal` → `enter_template_tail`); `pilots/rusty-js-parser/derived/src/stmt.rs:1251`, `pilots/rusty-js-parser/derived/src/expr.rs:1583` (call-site arg removal)
**Net LoC delta**: +21 / -6 = +15 net (executable +4 / doc +11)
**Named constraint that made deletion safe**: `Parser::current_lex_goal` + `derive_lex_goal_after` (LGSS-EXT 1) — goal-symbol selection became a parser-state-maintained invariant; the per-call goal argument became derivable from parser state, not from caller knowledge
**What was simplified**: external (non-parser-crate) callers of the parser-tier lookahead-management methods no longer construct `LexerGoal` literals. 2 → 0 external `LexerGoal` mentions; 2 → 0 method-signature goal arguments at the parser-tier boundary. The directive-parameter that should live at the lexer's low-level API (per Doc 729 §IV resolver-instance pattern) no longer leaks upward.
**Gates re-verified**: diff-prod 42/42, random-300 0 regressions, SyntaxError cluster 45/45 held
**Composes-with**: LGSS-EXT 1 (the named-constraint precursor); Doc 729 §IV (the resolver-instance discipline this deletion enacts at the API boundary); Findings LGSS.3 + LGSS.4

Pre-LGSS the goal-symbol-selection discipline existed at the wrong tier: downstream-tier callers (`stmt.rs`, `expr.rs`) had to know about `LexerGoal` and construct the right value at each call. Naming the constraint at the parser-state tier (LGSS-EXT 1) made the discipline parser-internal; intent-named methods (`enter_template_tail`, `rewind_lexer_to`) then absorbed the directive parameter as an implementation detail. This is a SURFACE deletion rather than a CODE deletion — what was removed is the leakage of resolver-instance-tier concerns into higher tiers; raw LoC grew slightly (cost of naming the constraint), but the contaminated-tier count dropped from 3 to 1.

---

### Future entries

Append below as deletions land. Each entry's format follows the template at the file head. The ledger is consulted on locale-close to ask: *did this locale's substrate move enable a downstream deletion?* If yes, the deletion is recorded; if not, the locale's value is named-addition-only (which is fine — but the asymmetry-check matters).

The ledger's growth rate is a standing observation: a healthy engagement should see deletion entries paced approximately with named-addition entries. An engagement that adds without deleting accumulates carriers; an engagement that deletes without naming new constraints loses discipline. The balance is the cybernetic loop's legibility property.
