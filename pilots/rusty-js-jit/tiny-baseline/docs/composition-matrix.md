# TB-EXT 4 — composition matrix

*N=5 per (bench × config); median ns/iter. Generated 2026-05-23T09:41:59-07:00.*

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| none | 72.1 | 82.9 |
| TB | 71.8 | 82.5 |
| STUB | 71.3 | 82.7 |
| VTI | 74.6 | 92.6 |
| TB+STUB | 71.5 | 82.2 |
| TB+VTI | 70.3 | 85.9 |
| STUB+VTI | 70.3 | 86.2 |
| TB+STUB+VTI | 70.9 | 85.5 |

## Per-flag contribution (delta from `none`)

