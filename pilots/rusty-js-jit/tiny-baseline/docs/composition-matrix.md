# TB-EXT 4 — composition matrix

*N=5 per (bench × config); median ns/iter. Generated 2026-05-23T08:39:04-07:00.*

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| none | 71.2 | 81.0 |
| TB | 71.0 | 81.1 |
| STUB | 74.8 | 81.0 |
| VTI | 70.5 | 728.3 |
| TB+STUB | 70.7 | 81.7 |
| TB+VTI | 70.9 | 733.6 |
| STUB+VTI | 70.8 | 730.9 |
| TB+STUB+VTI | 70.3 | 726.8 |

## Per-flag contribution (delta from `none`)

