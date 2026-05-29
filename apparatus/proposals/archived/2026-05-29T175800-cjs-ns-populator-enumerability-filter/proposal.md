---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 9b7475f1c74d6cd80f50a17b1a83661c82028f20
target_branch: main
summary: CJS-to-ESM namespace populator skips non-enumerable own properties (principled fix, per keeper Telegram 10417)
risk_class: substrate
---
Single-edit fix to pilots/rusty-js-runtime/derived/src/module.rs::populate_cjs_namespace_view_at. Skips non-enumerable own props by default; preserves name/length/prototype lift list and Rung-7 super_proto carve-out. Expected +~145 PASS on top500 (caller-leak cluster). Helmsman-authored substrate per keeper Telegram 10417 explicit authorization given fleet Codex rate-limit.
