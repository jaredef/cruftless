# interp-hot-intrinsics — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. This locale is a performance optimization (interp-tier IC table for hot intrinsic method calls like toLowerCase, trim, indexOf, slice), not a correctness fix. Diff-prod measures byte-for-byte stdout parity, not execution speed. The empirical anchor is the CRB string_url_sweep benchmark.
