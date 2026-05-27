# iter-protocol-bytecode-rewrite — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. This locale is a performance optimization (bytecode-rewrite IC for the for-of dispatch pattern, eliminating per-.next() iterator-result-object allocation for Array/String receivers). Diff-prod measures byte-for-byte stdout parity, not execution speed. The empirical anchor is the CRB string_url_sweep benchmark, where the locale achieved -14.7% additional wall-clock reduction.
