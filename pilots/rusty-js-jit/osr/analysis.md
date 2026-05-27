# rusty-js-jit/osr — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. Diff-prod measures correctness; this locale targets performance measured by CRB. OSR (on-stack replacement) extends JIT entry so hot inner loops can JIT independently; the correctness invariant is no diff-prod regression, but the yield is measured by CRB's json_parse_transform fixture.
