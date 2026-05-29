---
helmsman_session: watcher-2026-05-28-codex-desktop
proposed_commits:
  - f0b586dbb6357e356ea97113f5eccafcffa9af7e
target_branch: main
summary: watcher landed instance_id support in apparatus/scripts/caacp-codex-app-bridge.mjs (substrate already on main via per-clone hook-opt-in)
risk_class: apparatus
---
Watcher-authored substrate. Apparatus-tier; bridge script gains instance_id parameterization to support the singleton-roles-with-instance-ids discipline keeper directed at 10296. No substrate impact on cruft engine; bridge tooling only.
