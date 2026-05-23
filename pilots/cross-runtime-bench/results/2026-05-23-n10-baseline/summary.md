# cross-runtime-bench summary — 2026-05-23

runs per (fixture × runtime): **10**; wall-clock in ms (median of 10); runtimes: node v22.22.0, bun 1.3.11, cruft cruft 0.1.0

| fixture | equality | node (ms) | bun (ms) | cruft (ms) | cruft / node | cruft / bun |
|---|---|---:|---:|---:|---:|---:|
| crypto_sha256_batch | DIFFER | 77.000 | 30.500 | FAIL | - | - |
| json_parse_transform | EQUAL | 122.000 | 94.000 | 2481.000 | 20.34x | 26.39x |
| string_url_sweep | EQUAL | 89.500 | 52.000 | 741.500 | 8.28x | 14.26x |
