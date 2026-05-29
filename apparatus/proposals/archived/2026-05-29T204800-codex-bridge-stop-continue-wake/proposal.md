---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - f6ffe710a5a005b4cdd88097e864c2c9049868af
target_branch: main
summary: Codex bridge stop-continue wake primitive (per watcher 2026-05-29 design + keeper Telegram 10446/10449)
risk_class: apparatus
---
Bridge enhancement: per-directive active ledger + thread.status polling + CAACP CONTINUE re-injection on idle/notLoaded with 60s/120s/3x throttles. No runtime substrate impact. Helmsman-authored per keeper authorization given watcher offline + deputy offline.
