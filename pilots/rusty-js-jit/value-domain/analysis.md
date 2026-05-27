# rusty-js-jit/value-domain — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. Diff-prod measures correctness; this locale targets performance measured by CRB. The value-domain pilot extends the f64-default calling convention to encode non-Number/non-Object Value variants (String, Boolean, etc.), enabling downstream JIT-IC pilots. Yield measured by CRB.
