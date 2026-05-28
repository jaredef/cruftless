---
proposal_slug: 2026-05-28T234000-clfg-ext-4-update-target-cover-id
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-28T23:40:00Z
covers_commits:
  - 72ca802019c654628e9e49f95ec21912c25a7a70
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out.

Keeper Rung-2 authorization: Telegram 10233 ("continue the arc"). The substrate commit at 72ca8020 executes the next sibling-child closure of the lowering-feature-gap-triage arc per the parent CLFG seed's named tail-cluster triage (update-target after for-in-destructure-head closed in FIDH-EXT 1).

**Substrate-tier verification**:

1. **Target exemplar suite**: 3 parent-list exemplars (postfix-decrement, postfix-increment, prefix-decrement target-cover-id) + 1 adjacent (prefix-increment, not on list) all flip FAIL → PASS. Direct probe confirms.

2. **Protective gates**: TAMM 82/100, TAWR 63/100, diff-prod 61/51 all unchanged. Substrate change is 3 LOC at compiler.rs:6671; scoped to compile_update's Parenthesized unwrap.

3. **Parent CLFG suite**: 19/32 → 26/32 (+7). Helmsman correctly attributes only +3 to this rung; the +4 incidental came from rebase onto 4f3bd525's super-direct-eval threading. Yield attribution honest in the trajectory entry and the proposal manifest.

4. **Pattern alignment**: the substrate move is the canonical "apply an established codebase pattern at a previously-omitted site" close. Every other Expr handler in compiler.rs (lines 518, 688, 3716, 3984, 5973, 6008, 6258) already unwraps Parenthesized; compile_update was an omission. The fix is to add the missing unwrap.

5. **Rule discipline honored**: Rule 4 single coordinated rung; Rule 15 chapter-close-inspect satisfied (6 remaining FAILs identified: 5 super-direct-eval runtime cells deferred per existing policy, 1 complex-assignment-target compile cell as a small remaining tail).

6. **No new abstraction introduced**: 3-line unwrap recursion; no new helper, no new trait, no new tier. Substrate amortization via pattern reuse.

**Apparatus-meta concerns considered**:

- Parent locale's CLFG-EXT 3 entry records the for-in-destructure-head spawn+close (FIDH-EXT 1) per the parent-tier accounting discipline; CLFG-EXT 4 is the parent-tier entry for this rung. Both entries honor the parent's existing trajectory style (CLFG-EXT 0 baseline, CLFG-EXT 1 super-child founding, CLFG-EXT 2 super-child progress).
- Stage 2 mechanical-veto coverage: this proposal+decision pair covers the substrate commit's SHA.

**APPROVED for push.**

Archive to `apparatus/proposals/archived/2026-05-28T234000-clfg-ext-4-update-target-cover-id/` after push lands.
