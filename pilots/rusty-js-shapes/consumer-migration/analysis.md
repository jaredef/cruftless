# rusty-js-shapes/consumer-migration — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. Diff-prod measures correctness; this locale targets performance measured by CRB. Consumer-migration makes ~41 direct-.properties sites shape-aware so that Object::new_ordinary() can default to shaped mode without regressing diff-prod (the first enrollment attempt regressed 39 to 31/42 PASS). The gate is diff-prod non-regression, not diff-prod improvement.
