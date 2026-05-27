# rusty-js-jit/value-tag-inline — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. Diff-prod measures correctness; this locale targets performance measured by CRB. The value-tag-inline pilot replaces extern-helper-based unbox/rebox with inline tag-check + extract via Cranelift IR for hot property-access paths. Yield measured by CRB ns/iter benchmarks.
