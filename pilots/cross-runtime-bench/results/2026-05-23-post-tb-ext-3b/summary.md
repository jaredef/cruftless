# cross-runtime-bench summary — 2026-05-23

runs per (fixture × runtime): **10**; wall-clock in ms (median of 10); runtimes: node v22.22.0, bun 1.3.11, cruft cruft 0.1.0

| fixture | equality | node (ms) | bun (ms) | cruft (ms) | cruft / node | cruft / bun |
|---|---|---:|---:|---:|---:|---:|
| arith_tight_loop | EQUAL | 201.500 | 99.000 | 334.500 | 1.66x | 3.38x |
| crypto_sha256_batch | DIFFER | 81.500 | 33.000 | FAIL | - | - |
| json_parse_transform | EQUAL | 121.500 | 95.500 | 2434.000 | 20.03x | 25.49x |
| string_url_sweep | EQUAL | 90.500 | 50.000 | 743.000 | 8.21x | 14.86x |
