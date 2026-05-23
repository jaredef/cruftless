# cross-runtime-bench summary — 2026-05-23

runs per (fixture × runtime): **30**; wall-clock in ms (median of 30); runtimes: node v22.22.0, bun 1.3.11, cruft cruft 0.1.0

| fixture | equality | node (ms) | bun (ms) | cruft (ms) | cruft / node | cruft / bun |
|---|---|---:|---:|---:|---:|---:|
| crypto_sha256_batch | DIFFER | 77.000 | 31.000 | FAIL | - | - |
| json_parse_transform | EQUAL | 121.000 | 94.000 | 2474.500 | 20.45x | 26.32x |
| string_url_sweep | EQUAL | 90.000 | 51.000 | 752.500 | 8.36x | 14.75x |
