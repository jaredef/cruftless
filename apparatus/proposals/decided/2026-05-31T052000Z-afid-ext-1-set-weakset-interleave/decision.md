---
proposal_slug: 2026-05-31T052000Z-afid-ext-1-set-weakset-interleave
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10690
decided_at: 2026-05-31T05:20:00Z
covers_commits:
  - pending
---

## Findings

APPROVED per keeper Telegram 10690 ("3" selecting Set + WeakSet ctors from the AFID.2 triage). Second confirmed instance of the interleave + iter_close_rt pattern at a runtime intrinsic surface.

## Verification

1. Build PASS (~1m 08s).
2. AFID-EXT 1 7-cell probe: 6/7 PASS — OOM closed; silent-swallow TypeError suppression closed; one pre-existing WeakSet object-identity-storage bug unmasked (separable sibling).
3. cargo test --release -p rusty-js-runtime --lib: 74/0/1 preserved.
4. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8.

## Findings surfaced

- **AFID.3**: WeakSet (and likely WeakMap) storage uses `abstract_ops::to_string(&v)` as the key, collapsing distinct objects to "[object Object]". `new WeakSet([o1, o2]).has(o1) && .has(o2)` returns false because o2 overwrites o1 at the same key. Sibling locale candidate. Unrelated to interleave/close discipline.

## Composes-With

- AFID-EXT 0 decision (substrate prefix source)
- IPTD-EXT 1 decision (helper-tier source)
- Carry-forward: Promise-iteration interleave (Promise.all/race/any/allSettled)
