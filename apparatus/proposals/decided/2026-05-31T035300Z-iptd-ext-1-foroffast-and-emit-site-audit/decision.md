---
proposal_slug: 2026-05-31T035300Z-iptd-ext-1-foroffast-and-emit-site-audit
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10669
decided_at: 2026-05-31T03:58:00Z
covers_commits:
  - pending  # filled at landing commit
---

## Findings

APPROVED per keeper Telegram 10669 ("Approved"). Deeper-layer closure per Rule 13 following IPTD-EXT 0 NEGATIVE (Pi-OOM regression; see sibling decision doc).

## Verification

1. `cargo build --release --bin cruft -p cruftless`: PASS (~1m 10s).
2. Direct probe (original 7-cell): 5/7 PASS — cells 1 + 2 (the OOM cells) close; cells 3 + 6 surface as carry-forward to `iterator-close-emission-sites` (plain-for-of IteratorClose-on-break, not in EXT 1's scope).
3. Direct probe (cross-consumer 7-cell): 7/7 PASS — helper-tier reinstate covers destructuring, array-spread, Array.from, spread-call, yield*, and destructuring-rest at the single dispatcher.
4. Forensic gate: all dev probes run under `ulimit -v 2097152`; no allocation exceeded ~50 MiB on any cell. Pi survived.
5. Manifest refresh: 231 → 232 (IPTD locale enrolled).
6. Cluster gates (TAMM, TAWR, diff-prod) + test262-sample regression sweeps: not run per CLAUDE.md no-auto-sweeps discipline. Surfaced to keeper for direction at land time.

## Doc 721 predicted-vs-actual + Rule 29 considerations

EXT 1's yield is +0 to +5 on test262-sample (per the EXT 0 false-pass amendment caveat: sample paths may not include the bulk of the chain-bundle cells). The substrate correctness is verified by the 12/14 direct-probe PASS.

## Findings surfaced

- **IPTD.1** parallel-emit-site coherence at helper-tier dispatcher (6 surfaces share `__destr_iter_step`).
- **IPTD.2** forensic-gate as substrate-rung component when touching loop-allocation paths.

Both are candidates for the next findings-disposition cycle; cross-locale recurrence with ASTA.2 noted for IPTD.1.

## Composes-With

- `apparatus/proposals/decided/2026-05-30T235300Z-iptd-ext-0-iterator-protocol-throws/decision.md` (the negative this closure resolves).
- `apparatus/docs/predictive-ruleset.md` Rule 13, Rule 17, Rule 20, Rule 23.
- `pilots/iterator-protocol-throw-discipline/trajectory.md` IPTD-EXT 1 entry.
- Carry-forward: `pilots/iterator-close-emission-sites/` (cells 3 + 6 of original probe).
- Carry-forward: async-iter §7.4.6 post-`__await` type-check (deferred per proposal scope).
