---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - e7fe945c1f75f83180b9ff0d980a2a388f74433d
target_branch: main
summary: env.example + init-protocol §V.1 — document CODEX_APP_* env vars + thread/resume refinement surfaced by watcher
risk_class: apparatus
---
Per keeper 10286 ("Yes" to my offer). Adds three CODEX_APP_* env var families to env.example (CODEX_APP_SERVER_WS, CODEX_APP_TOKEN_FILE, CODEX_APP_THREAD_<ROLE>) per watcher's local config. Adds one-paragraph env-var pointer + thread/resume note to init-protocol §V.1. No substrate impact.
