---
proposal_slug: 2026-05-31T041500Z-ices-ext-1-for-of-break-iterclose
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10676
decided_at: 2026-05-31T04:25:00Z
covers_commits:
  - pending  # filled at landing commit
---

## Findings

APPROVED per keeper Telegram 10676 ("Continue with 1") authorizing the IPTD-EXT 1 chapter-close carry-forward into ICES-EXT 1.

## Verification

1. `cargo build --release --bin cruft -p cruftless`: PASS (~1m 12s).
2. Original 7-cell IPTD probe: 7/7 PASS (cells 3 + 6 now close after break per spec §14.7.5.6 step 5).
3. Cross-consumer 7-cell probe: 7/7 PASS preserved.
4. Labelled-break probe: emits `["B","A"]` — innermost-first close order matches spec.
5. Residual probe: confirms ICES-EXT 2 (return path) + ICES-EXT 3 (throw path) carry-forward, both unaffected by EXT 1.
6. All dev runs under `ulimit -v 2097152`. Pi survived.

## Composes-With

- `apparatus/proposals/decided/2026-05-31T035300Z-iptd-ext-1-foroffast-and-emit-site-audit/decision.md`
- `pilots/iterator-close-emission-sites/trajectory.md` ICES-EXT 1 entry
- `pilots/iterator-protocol-throw-discipline/` (IPTD locale; this is the chapter-close-inspect carry-forward landing)
- Carry-forward to ICES-EXT 2 (for-of return) + ICES-EXT 3 (for-of throw)
