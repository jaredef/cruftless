---
proposal_slug: 2026-05-28T233500-fidh-ext-1-for-in-destructure-head
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-28T23:35:00Z
covers_commits:
  - 572fa6828655075a73314ee734d01db04f2cce0b
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out for the pre-instantiation period.

Keeper Rung-2 authorization: Telegram 10231 ("continue with no 1") selecting the lowering-feature-gap-triage arc as the next substrate-derivation target. The substrate commit at cb57dbb directly executes the arc's continuation per the helmsman's substrate-pick (next sibling-child after super-reference-lowering's +13 close; for-in destructure head identified in the parent's CLFG-EXT 0 baseline as the largest remaining tail-cluster).

**Substrate-tier verification**:

1. **Target exemplar suite**: all 6 target cells flip from `compile: for-in with destructure head not yet supported` to PASS. Direct probe per the proposal confirms each test.

2. **Protective gates**: TAMM 82/100, TAWR 63/100, diff-prod 61/51 all unchanged. The substrate move is scoped to the for-in handler in compiler.rs; shared helpers (`emit_destructure`, `emit_destructure_assign`) are unmodified.

3. **Parent CLFG exemplar suite**: 13/22 → 19/32. The +6 corresponds exactly to the for-in destructure cluster; suite expansion from 22 to 32 reflects the parent's full exemplar set vs. the super child's subset.

4. **Rule discipline honored**: Rule 23 baseline-inspect satisfied (all 6 exemplars confirmed pre-rung with the diagnostic); Rule 24 not triggered (2 emit sites < 3+ threshold); Rule 13 not triggered (no negative); Rule 4 not violated (single coordinated rung); Rule 15 chapter-close-inspect satisfied (remaining 13 parent FAILs are 9 super-deferred + 3 update-target + 1 complex-assignment-target, all tail-clusters outside this locale's scope).

5. **Substrate move is mirror-of-existing-pattern**: the for-of destructure-head substrate (compiler.rs:2140–2395) is the existing pattern; this rung applies the same shape at for-in's emit sites. No new abstraction introduced; no new standing rule warranted.

6. **Manifest refresh executed**: 215 locales (was 214; +1 for the new sub-locale spawn). discover.sh run; updated manifest.json committed in the substrate commit per the manifest-refresh discipline.

**Apparatus-meta concerns considered**:

- Locale spawn under existing parent (cruft-lowering-feature-gaps) honors the parent's documented tail-triage path; no new top-level locale was spawned without Rule 11 coverage check.
- Append-only protocols honored (no ledger edits; no findings.md addendum needed for a mirror-existing-pattern close).
- Stage 2 mechanical-veto coverage: this proposal+decision pair covers the substrate commit's SHA.

**APPROVED for push.**

Archive to `apparatus/proposals/archived/2026-05-28T233500-fidh-ext-1-for-in-destructure-head/` after push lands.
