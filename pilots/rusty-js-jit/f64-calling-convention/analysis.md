# rusty-js-jit/f64-calling-convention — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. Diff-prod measures correctness; this locale targets performance measured by CRB. The f64-default calling convention is an internal JIT encoding change that must preserve correctness (no diff-prod regression) while enabling downstream VTI revival.
