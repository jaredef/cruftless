# rusty-js-jit/tiny-baseline — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. Diff-prod measures correctness; this locale targets performance measured by CRB. The tiny-baseline (Sparkplug-style) replaces the Rust call_function dispatcher for hot small functions; its yield is measured by CRB call-overhead ns/iter benchmarks.
