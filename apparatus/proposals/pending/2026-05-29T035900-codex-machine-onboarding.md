---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 295bce0531b50120127c87f4c02d591f6ad52a1d
target_branch: main
summary: docs — Codex Machine Onboarding Protocol (watcher's authoring; landed via per-clone hook opt-in)
risk_class: apparatus
gates_pre: { test262_full: 67.6, test262_sample: 84.8, diff_prod: 61/51 }
gates_post: { test262_full: 67.6 (unchanged), test262_sample: 84.8 (unchanged), diff_prod: 61/51 (unchanged) }
---
Substrate: docs-only. Watcher landed apparatus/docs/codex-machine-onboarding-protocol.md (204 lines, IX sections: preconditions, choose role+identity, register with sidecar, locate codex thread, start app-server bridge, validate wake path, fallbacks, safety+policy, current watcher instantiation). Cross-references added at agent-init-protocol §II step 5 + caacp-server/README.md §3 + §"Cybernetic bridges". No substrate impact.
