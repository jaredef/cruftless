# 2026-05-26-missing-syntax-feature-concentration — log

## 2026-05-26 — arc opens

- Telegram 9847 → TECR-EXT 2 lift surfaced 1015 missing-syntax-feature records.
- Telegram 9851: "Let's hone in on missing syntax features."
- Top-5 cluster survey: HDSB 475, WBMS 264, IMM 76, DIA 41, CAR 44.
- Telegram 9853: "Add these all to the CANDIDATES.md doc and then begin with A."
- Tier K added to CANDIDATES.md.

## Sub-rung landings

- HDSB-EXT 1 (Annex B B.3.4 parser carve-out): 150/475. Rule 23 verification at landing — all 4 carve-outs correct (sloppy if-body, strict-rejects, generator-rejects, for-body-rejects).
- WBMS-EXT 1 (skip_to_top_terminator brace-bodied bump-and-return fix): 37/264. Root cause: ASI fallback ordering bug.
- IMM rediagnosed → RFSDO-EXT 1: 0/76 → 76/76 SKIP (apparatus redirect).
- DIA-EXT 1 (import.attributes parser extension): 40/41.
- CAR Rule 23 baseline: redirected to WBMS-EXT 2 (all 44 records are with-runtime-semantics residuals already covered by WBMS deferral).

## Close-condition met

All 5 candidates triaged 2026-05-26. Arc CLOSED.
