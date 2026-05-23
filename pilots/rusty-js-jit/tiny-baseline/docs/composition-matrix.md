# TB-EXT 4 — composition matrix

*N=5 per (bench × config); median ns/iter. Generated 2026-05-23T08:32:01-07:00.*

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| none | 121.1 | 146.5 |
| TB | 73.5 | 81.1 |
| STUB | 121.4 | 146.8 |
| VTI | 121.3 | 737.4 |
| TB+STUB | 71.1 | 81.3 |
| TB+VTI | 70.6 | 733.7 |
| STUB+VTI | 120.5 | 728.5 |
| TB+STUB+VTI | 70.4 | 737.8 |

## Per-flag contribution (delta from `none`)

