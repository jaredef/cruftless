---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 904fdeee7799b1e940a508fd14ef3195cb65c3c6
target_branch: main
summary: §V.6 deputy apparatus-artifact commit carve-out (per keeper Telegram 10343)
risk_class: apparatus
---
Doc-only update adding a narrow carve-out to §V.6: deputy MAY commit files only within apparatus/deputy/ under deputy authorship. Resolves stranded-artifact failure mode observed when fleet-state files sat untracked for 9 hours during the 2026-05-29 topology round. Substrate, proposal, and archive commits remain resolver-authored per unchanged §V.6. No substrate impact.
