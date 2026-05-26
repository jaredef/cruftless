# locale-positioning-audit — Trajectory

## LPA-EXT 0 — founding (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9802), opened in response to the apparatus §XI.1.b amendment necessitated by PPIF-EXT 2's deletion. That amendment was the first instance where a sibling locale's close dissolved a prior locale's claimed-irreducible carrier without the apparatus auto-noticing; this locale exists to systematize the audit of such drift.

**Apparatus established**:

- `seed.md` — telos, three-rung methodology, trigger conditions, carve-outs, composes-with, R13 prospective check.
- No findings docs yet; the `findings/` subdirectory will house per-rung output (`stale-claims.md`, `spinoff-chains.md`, `positioning-gaps.md`).

**Initial inventory** (snapshot at founding):
- Locale count: 109 (per `apparatus/locales/manifest.json`).
- Deletions ledger entries: 2 (PPIF-EXT 2; LGSS-EXT 2).
- Visible spinoff chains: 1 confirmed (LGSS → PPIF → FHNB).
- Apparatus-doc amendments triggered by sibling-locale drift this session: 1 (apparatus §XI.1.b, this commit + the prior).

**R13 prospective C1-C4 all hold (per seed §Methodology)**:

- C1: Doc 727 basin-stability + Doc 415 retraction-ledger are corpus-tier siblings.
- C2: manifest is JSON, locales are markdown — both grep-able and walkable.
- C3: TBV at LPA-EXT 1; bounded by locale count.
- C4: append-only; never edits prior trajectories.

**Status**: LPA-EXT 0 FOUNDED. LPA-EXT 1 (stale-claim survey) is the first substantive rung; runs on-trigger rather than on a schedule, so the first run waits for the next deletion-ledger entry or keeper directive.

**Findings**

**Finding LPA.0 (the apparatus's drift class is bounded by its own discipline)**: cruftless's discipline is heavy on append-only artifacts (findings.md, trajectories, deletions ledger, this locale's findings/). The drift class the audit catches is specifically the kind that append-only doesn't catch: claims in prior artifacts that become STALE because sibling work elsewhere dissolved their basis. The audit's role is to surface staleness, not to correct it (correction is the original locale's amendment-by-new-trajectory-entry move; the audit just makes the case for amendment legible). Standing recommendation: any apparatus discipline whose primary mechanism is append-only is structurally vulnerable to claim-staleness; a co-running audit locale is the dyadic-ascent counterpart that restores coherence.
