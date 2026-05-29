---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - af206417a1a0c76f13a32160a6ad1d7be87bc98f
target_branch: main
summary: ATC-EXT 2 restore function-declaration self-name slot as Let (closes ramda regression while preserving ATC-EXT 1 + spec)
risk_class: substrate
---
One-line compiler fix to compile_function_proto_with_name_hint: function-declaration self-name slot reinstated as Let (was removed in 99e6b1a9; ramda PASS→FAIL regression in post-R11 sweep is the probe signal). Three-way verification clean.
