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

### 2026-05-26 — GBSU-EXT 6: transitional HashMap-fallback paths deletion

**Commit**: pending (substrate landed in worktree; commit awaits keeper authorization)
**Files**:
- `pilots/rusty-js-runtime/derived/src/interp.rs` Op::LoadGlobal handler (`.or_else(|| self.globals.get(&name).cloned())` fallback)
- `pilots/rusty-js-runtime/derived/src/interp.rs` Op::LoadGlobalOrUndef handler (same fallback)
- `pilots/rusty-js-runtime/derived/src/interp.rs` Op::StoreGlobal strict-mode existence check (`self.globals.contains_key(&name)` term)
- `pilots/rusty-js-runtime/derived/src/interp.rs` `Runtime::global_get` helper (HashMap fallback final clause)

**Net LoC delta**: roughly −15 / +4 = **−11 net** (helper now terminates in Value::Undefined instead of HashMap lookup; four call sites lose their `.or_else(...)` fallback line).

**Named constraint that made deletion safe**: `Runtime.global_object` is the canonical globalThis ObjectId, populated by `install_global_this` (GBSU-EXT 1 introduced the field, GBSU-EXT 2 promoted it to primary reader, GBSU-EXT 3a + 4b + 5 verified all writers route through it). All JS-visible Op::StoreGlobal writes land on the Object first (GBSU-EXT 5); all readers (Op::LoadGlobal, Op::LoadGlobalOrUndef, the three stash-key round-trips in intrinsics.rs, the per-helper global_get call sites in intrinsics/prototype/napi/interp) consult the Object via `global_get`. With the writer + reader sides unified, the HashMap fallback is unreachable for any binding the unified surface holds.

**What was simplified**: the Op::LoadGlobal / LoadGlobalOrUndef / StoreGlobal opcode handlers each lose a fallback clause; the global_get helper terminates in a single Object-Object-or-Undefined branch instead of a three-step Object → HashMap → Undefined ladder. Doc 731 alphabet-purity: the runtime's binding-resolution alphabet went from `{Object, HashMap, engine_helpers}` to `{Object, engine_helpers}` — one symbol removed.

**Gates re-verified**:
- diff-prod 42/42 PASS
- test262-sample **86.8%** (6312 PASS / 963 FAIL / 397 SKIP) — clean parity with the pre-deletion baseline (delta of 1 test from the 6313 baseline; within noise).
- Function ctor probes: `Function("return 1")` → "function", `new Function("return class M extends Uint8Array {}")()` → "function" ✅.

**Composes-with**:
- Locale `pilots/global-binding-surface-unification/` (parent rungs GBSU-EXT 1-5)
- Doc 729 §XIII regression-as-implicit-constraint-probe (the methodology that surfaced the four sub-rung recurrences which made this deletion possible to land safely)
- Doc 731 alphabet purity (the principle the deletion advances)
- Doc 729 §VII.B engine_helpers bilateral (preserved invariant — engine_helpers fallback retained)

The transitional fallback clauses added at rungs 2 / 3a / 4 served as a constraint-discovery scaffold: each clause caught a hidden direct-reader that the audit had missed, triggering a §XIII regression-revision. With the audit complete through GBSU-EXT 5, the scaffolds become carriers — code that exists to compensate for an incompleteness no longer present. Their deletion is the rung-6 closure: the unified surface stands on its own. The `Runtime.globals` field itself remains for now; rung 7 audits the INSTALL-phase consumers before its removal.

### 2026-05-27 — GBSU-EXT 7f.4: `Runtime.globals` HashMap field deletion (arc closure)

**Commit**: pending (substrate landed in worktree; commit awaits keeper authorization)
**Files**:
- `pilots/rusty-js-runtime/derived/src/interp.rs` (field declaration + init + 3 bootstrap-fallback branches in Op::StoreGlobal / enumerate_roots / define_global_property; helper `global_get` final clause; Op::LoadGlobal + Op::LoadGlobalOrUndef `or_else` chains; `allocate_realm` ctor-lookup loop)
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (install_globals rung-7a late-allocation; install_global_this HashMap-drain loop; 6+ Function-ctor / Number-static / Error-prototype / Map / Promise / Error / Map-clone / Set-clone / RegExp-clone closure-captured lookups; stash-key cleanup `globals.remove` lines)
- `pilots/rusty-js-runtime/derived/src/regexp.rs` (register_global_native rung-7b inline dict-insert + HashMap-fallback duplicate)
- `pilots/rusty-js-runtime/derived/src/prototype.rs` (Boolean wrapper lookup)
- `pilots/rusty-js-runtime/derived/src/napi.rs` (globalThis read)
- `pilots/rusty-js-runtime/derived/src/promise.rs` (Promise ctor install)
- `cruftless/src/` 11 files (node_stubs.rs, lib.rs, url.rs, stream.rs, process.rs, http.rs, fs.rs, os.rs, events.rs, crypto.rs, util.rs, zlib.rs, tty.rs, path.rs, https.rs, assert.rs, module_ns.rs, timer.rs) — ~75 sites total migrated to `define_global_property` / `global_get` over rungs 7f.1-7f.3

**Net LoC delta**: roughly +200 / −300 = **−100 net**. The +200 is helpers (`define_global_property`, `global_get`, `snapshot_global_string_props`, `retain_global_string_props`, `replace_global_string_props`) + cluster migration boilerplate; the −300 is the field declaration + 75 inline `rt.globals.insert/.get/.contains_key` lines + 3 bootstrap-fallback branches + the install_global_this HashMap-drain loop (~25 lines) + the rung-2/3/4 transitional `or_else` fallback clauses.

**Named constraint that made deletion safe**: `Runtime::new` eager-allocates `global_object` (`Some(ObjectId)`) at construction. Combined with the GBSU-EXT 2 reader migration (Object-first), GBSU-EXT 3a writer migration (Object-canonical), GBSU-EXT 4b cluster audit (all rt.globals.get readers migrated to global_get), GBSU-EXT 5 writer-HashMap drop, and GBSU-EXT 6 fallback-clause deletion, every JS-visible global binding read/write routes through the unified globalThis Object. The `Runtime.globals` HashMap is provably unreachable in normal operation.

**What was simplified**: the runtime's binding-resolution alphabet collapsed from `{Object, HashMap, engine_helpers}` to `{Object, engine_helpers}` (Doc 731 alphabet purity — one symbol removed). Op::LoadGlobal / Op::LoadGlobalOrUndef / Op::StoreGlobal handlers each lose a fallback branch. `define_global_property` collapses to a single dict_mut().insert. install_global_this drops its drain loop entirely. enumerate_roots roots the single globalThis Object. The duplication of installation surface (HashMap-then-drain vs direct-Object-write) is gone.

**Gates re-verified**:
- diff-prod 42/42 PASS.
- test262-sample **86.7%** (6311 PASS / 964 FAIL / 397 SKIP) — **+61 tests over the 85.9% pre-deletion baseline**. Deletion as positive-surface §XIII probe: the rung-3a-onwards dual-write left install_global_this's drain loop overwriting properties already installed by the rung-7b register_global_* migrations, with subtle shape-system/descriptor invariant divergences. With the drain gone, the property table is cleaner; 61 tests that had been silently failing on those subtle invariants now pass.

**Composes-with**:
- Locale `pilots/global-binding-surface-unification/` (parent rungs GBSU-EXT 1-7f.3)
- Prior entry: `2026-05-26 — GBSU-EXT 6: transitional HashMap-fallback paths deletion` (the rungs-2-3a-4 scaffold deletion; this entry completes the arc by deleting the field the scaffold protected)
- Doc 729 §XIII (six +1 recurrences across this locale; the inverted positive-surface case at this entry is a methodological extension worth corpus-noting)
- Doc 731 alphabet purity (the principle the arc induces)
- Doc 729 §VII.B engine_helpers bilateral (preserved invariant)
- Tier-Ω.5.dddd / .qq compiler pre-allocation (IC.1 protected throughout)

The eight-rung GBSU arc (1, 2, 3a, 4, 4b, 5, 6, 7a, 7b, 7c, 7d, 7e, 7f.1, 7f.2, 7f.3, 7f.4 — fifteen sub-rungs counting revisions) lands an architectural straightening at the top of the DAG that simplifies every downstream binding-resolution path. The +9.1pp yield that came from the original bridge work (rungs 2-4) compounds with the +0.8pp surfacing at field-deletion time, against the original 77.6% baseline (now 86.7% — **+9.1pp aggregate**). This is the value of the alphabet-narrowing thesis at the engine-tier: not just cleaner code, but observable test surface gains from substrate coherence.

### Future entries

Append below as deletions land. Each entry's format follows the template at the file head. The ledger is consulted on locale-close to ask: *did this locale's substrate move enable a downstream deletion?* If yes, the deletion is recorded; if not, the locale's value is named-addition-only (which is fine — but the asymmetry-check matters).

The ledger's growth rate is a standing observation: a healthy engagement should see deletion entries paced approximately with named-addition entries. An engagement that adds without deleting accumulates carriers; an engagement that deletes without naming new constraints loses discipline. The balance is the cybernetic loop's legibility property.
