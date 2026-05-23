# cross-runtime-bench summary — 2026-05-23

runs per (fixture × runtime): **10**; wall-clock in ms (median of 10); runtimes: node v22.22.0, bun 1.3.11, cruft cruft 0.1.0

| fixture | equality | node (ms) | bun (ms) | cruft (ms) | cruft / node | cruft / bun |
|---|---|---:|---:|---:|---:|---:|
| arith_tight_loop | EQUAL | 201.000 | 98.500 | 335.500 | 1.67x | 3.41x |
| crypto_sha256_batch | DIFFER | 79.000 | 31.500 | FAIL | - | - |
| json_parse_transform | EQUAL | 121.000 | 93.500 | 2489.500 | 20.57x | 26.63x |
| string_url_sweep | EQUAL | 90.000 | 51.000 | 747.500 | 8.31x | 14.66x |
