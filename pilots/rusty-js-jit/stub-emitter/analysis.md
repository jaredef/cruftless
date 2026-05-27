# rusty-js-jit/stub-emitter — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. Diff-prod measures correctness; this locale targets performance measured by CRB. The IC stub emitter is a hand-rolled aarch64 property-access fast path consuming shape pointers; its yield is measured by CRB per-property-access ns/iter, not by behavioral parity.
