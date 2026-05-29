---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 5d80b9e6103f3de9872f479ce2c17643d67bf0e2
target_branch: main
summary: refine CJS-NS exclusion to legacy fn-internals only (caller/arguments) per empirical es-shim probe signal
risk_class: substrate
---
Refinement of prior 9b7475f1 fix. Blanket-non-enumerable-skip regressed 20 es-shim packages; refined to exclude only 'caller' + 'arguments' (legacy non-spec fn-internals). Matches bun's actual implicit constraint per empirical post-fix sweep + es-shim source inspection.
