# rusty-js-shapes — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. Diff-prod measures correctness; this locale targets performance measured by CRB. Hidden classes (shared shape descriptors, transition trees, shape-ptr + slot-index IC targets) are a performance substrate. The correctness invariant is no diff-prod regression after shape enrollment; the yield is measured by CRB property-access benchmarks.
