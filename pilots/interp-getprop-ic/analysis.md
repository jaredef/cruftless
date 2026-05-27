# interp-getprop-ic — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. This locale is a performance optimization (interp-tier inline cache for Op::GetProp method-resolve dispatch), not a correctness fix. Diff-prod measures byte-for-byte stdout parity, not execution speed. The empirical anchor is the CRB string_url_sweep benchmark, not diff-prod.
