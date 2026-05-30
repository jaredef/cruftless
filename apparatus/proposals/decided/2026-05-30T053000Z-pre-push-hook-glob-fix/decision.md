---
proposal_slug: 2026-05-30T053000Z-pre-push-hook-glob-fix
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10526
decided_at: 2026-05-30T05:30:00Z
covers_commits:
  - 1c5b9da0b38e487252d6a6934886e2c992b110f5
---

## Findings

Approved per keeper directive Telegram 10526 ("1 then 2 then 3") authorizing the three next-up candidates in order; #1 is this hook fix.

Substrate commit `1c5b9da0` replaces `.githooks/pre-push`'s decision-file glob:

```diff
-            for decision in apparatus/proposals/decided/*.md; do
+            decisions=$(find apparatus/proposals/decided -mindepth 1 -maxdepth 2 \
+                -name '*.md' -not -path '*/.*' 2>/dev/null)
+            for decision in $decisions; do
```

The old glob only matched the flat `<slug>.md` shape. Every recent decision lives at the nested `<slug>/decision.md` path, so the gate had been silently no-op'ing for the entire slug-subdir-convention period (every landing since the convention shifted has been effectively un-gated). The `find -maxdepth 2` catches both shapes with no behavior change for the flat case.

## Verification

1. `bash -n .githooks/pre-push` — syntax PASS.
2. Hook simulation against the last five commits as if unpushed: the fixed hook correctly recognized:
   - `cec03e6e` (milf-ext-7) covered by `ea59169f` (nested decision) — PASS.
   - `6c5a3f19` (multi-bridge) covered by `f7d1d4dc` (nested decision) — PASS.
   - `ea59169f`, `f7d1d4dc` covered by being apparatus/proposals/-only commits (the existing path carve-out) — PASS.
   - `3e9ba56d` (R2 fastify kState landing) flagged as **uncovered** — correct: no decision file exists for it in either shape. Surfaces a pre-existing audit gap the broken gate had been masking.
3. The hook still respects `CRUFTLESS_HOOK_BYPASS` for emergency keeper override.

## Side effect surfaced

`3e9ba56d bytecode: let lexical names shadow function self` (R2's fastify-kState landing earlier this session) was pushed without a decision file under the (then-broken) gate. That commit is now retroactively uncovered. Recovery options: (a) author a retroactive decision file covering 3e9ba56d, or (b) leave as a historical pre-fix landing. Recommendation: option (a), since the substrate quality was excellent and we have full diagnostic context. Tracked as a follow-up rung after #2 and #3 in keeper's order.
