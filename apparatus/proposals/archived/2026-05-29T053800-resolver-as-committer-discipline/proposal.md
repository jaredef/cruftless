---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 28e35bd21174f42a3e10d3fd6d922acfbd503690
target_branch: main
summary: agent-init-protocol §V.6 resolver-as-committer discipline (per keeper directives 10325 + 10327)
risk_class: apparatus
---
Doc-only update codifying the per-resolver self-commit discipline validated by the 2026-05-29 EPSUA quartet round. Names helmsman as authorizer (not committer), deputy as relay-only (never committer), substrate-resolver as the sole author of substrate/proposal/archive commits in its own clone. Worktree-hygiene rules consolidated (no add-A, file enumeration, soft-reset for SHA-update, no chicken-and-egg amend). Shared-clone declared unsupported with per-worktree as the upgrade path. No substrate impact.
