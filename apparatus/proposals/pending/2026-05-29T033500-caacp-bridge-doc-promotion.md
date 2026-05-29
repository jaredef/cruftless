---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - cbb1e57505cd4195e487f24ed7c84a8a5646f250
target_branch: main
summary: docs — promote watcher's codex-app-bridge to primary in init-protocol §V; demote tmux to fallback
risk_class: apparatus
---
Documentation-only commit: agent-init-protocol.md §V split into V.1 (Codex Desktop app-server bridge — primary) + V.2 (tmux send-keys bridge — fallback) + shared directive subsection. caacp-server/README.md gains cross-reference. Also writes the APPROVED decision artifact for the watcher's a7836947 (which landed via per-clone-hook-opt-in). No substrate impact.
