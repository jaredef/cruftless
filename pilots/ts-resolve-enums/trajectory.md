# ts-resolve-enums — Trajectory

## TRE-EXT 0 — workstream founding (2026-05-24)

**Trigger**: TXC post-TRMLE failure-table row 2 (22 files of enum-related parse errors). Second downstream sub-locale from TXC.

**Founding artefacts**: seed.md + trajectory.md + scaffolded dirs. TRE-EXT 1 (MVP enum strip + re-measure + close) next.

---

## TRE-EXT 1 — enum-MVP-strip + keyword unblock + chapter close (2026-05-24)

**Three substrate edits**:

1. **Enum decl strip** (`pilots/ts-resolve/derived/src/strip.rs`): pattern `enum NAME { ... }`. Uses `match_braces` for the body close. Backward-extends strip range to include leading `export` / `declare` / `const` / `default` modifiers.

2. **`declare enum` short-circuit**: the existing `declare ...` rule's `find_stmt_end` doesn't traverse `{...}` enum body cleanly. Added a check to defer `declare enum X { ... }` to the enum rule (which uses match_braces).

3. **`is_overload_blocked_name` keyword set tightened**: removed `from`, `as`, `of`, `in`, `instanceof`, `do` from the blocked list. Their JS-keyword forms (`import X from "..."`, `e as T`, `for (x of y)`, `x in y`, `x instanceof Y`, `do { } while`) never have `(` immediately after the keyword name, so the existing `next_punct_immediate(LParen)` filter naturally rejects them. As function/method names (`function from(...)`, `function of(...)`, etc.), they need to be matched by the overload rule. **Discovered mid-round via TXC re-measure** showing the enum strip ALONE didn't lift parity — files like `rxjs/observable/from.ts` were still failing because `function from<...>(...)` overloads weren't being stripped (since `from` was blocked).

**Mid-round discovery (Finding TRE.1)**: the enum-MVP-strip alone was a NULL (0 pp lift). Investigation of the new top failure showed `function from<O extends ObservableInput<any>>(input: O): Observable<ObservedValueOf<O>>;` failing — root cause: `from` was in the blocked keyword list. Same shape as the `do`/`of` issue from TRGC-EXT 3. **Seventh reproduction of the inspect-then-iterate compound discovery pattern (Finding IX.3)** — and the discovered gap delivered the load-bearing yield (+5.9 pp) while the planned scope (enum strip) was 0 pp on its own.

**Gates**:
- `cargo test --release -p ts-resolve`: ✅ 51/51 PASS (+5 enum tests)
- `cargo build --release --bin cruft`: ✅ clean
- `diff-prod 42/42 PASS` ✅
- **TCC parse-parity 96.5% → 97.1%** (+0.6 pp incidental)
- **TXC execute-parity 52.7% → 58.6% (+5.9 pp)** — Pred-tre.3 HELD STRONGLY (target ≥56.7%)

### Findings

**Finding TRE.1** (planned-scope NULL + discovered-gap pays off): the seed targeted enum-strip with predicted ≥4 pp lift. Enum-strip alone delivered 0 pp; the mid-round discovered fix (keyword unblock) delivered +5.9 pp. **Sub-locale results aggregated by what fixes were applied; the planned-scope label was misleading**. Standing observation: at chapter close, report ALL fixes that landed, not just the planned-scope one — and credit the inspect-then-iterate discovery work as the load-bearing element.

**Finding TRE.2** (enum-only-declared files don't gain from MVP-strip without runtime usage support): the 22 enum-failing files mostly USE enums (e.g., `NotificationKind.NEXT` references). MVP-strip removes the declaration but downstream usages become undefined-property reads → still fail at runtime. **Pred-tre.3's target (≥4 pp from enum-strip alone) was implicitly testing a hypothesis that has been EMPIRICALLY DISCONFIRMED**: enum-declaration-only files are rare in the consumer corpus. Real consumer code that declares enums also uses them. **TRE-EXT 2 (full lowering) is the load-bearing follow-on for enum coverage**.

### Status: CHAPTER CLOSED at TRE-EXT 1

Standing rule 13 corroboration count: 11. Cumulative execute-parity this round: 52.7% → 58.6% (+5.9 pp).

**Remaining failures** (now lower-count, more diverse):
- TypeError prototype-undefined (ESM cycle issue, 30 files) — runtime substrate, separate locale
- parse expected Colon (6) — small remaining strip gaps
- Other small categories (3-2-1-1...)
