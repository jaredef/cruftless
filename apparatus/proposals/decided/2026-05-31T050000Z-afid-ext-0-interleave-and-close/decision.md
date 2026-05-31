---
proposal_slug: 2026-05-31T050000Z-afid-ext-0-interleave-and-close
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10684
decided_at: 2026-05-31T05:00:00Z
covers_commits:
  - pending
---

## Findings

APPROVED per keeper Telegram 10684 ("Continue"). Founds locale `pilots/array-from-interleave-discipline/`. Closes the runtime-tier OOM regression at Array.from with mapfn-throw on infinite iters.

## Verification

1. Build PASS (~1m 06s).
2. AFID-EXT 0 8-cell probe: 8/8 PASS.
3. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, ICES-EXT 3 9/9.
4. Manifest refresh: 232 → 233.

## Findings surfaced

- **AFID.1** ICES-EXT 3 close-throw spec divergence (cross-locale; ICES bytecode tier replaces body-thrown with close-thrown; spec §7.4.9 step 4 says original wins). Candidate ICES-EXT 3.1 fix surface.
- **AFID.2** eager-collect-then-process anti-pattern at runtime-tier iterable-consuming intrinsics. Candidate cross-intrinsic audit.

## Composes-With

- IPTD-EXT 1 decision (helper-tier discipline source)
- ICES-EXT 1/2/3 decisions (bytecode-tier sibling)
- Doc 721 SAMPLE.1 sub-bundle decomposition
