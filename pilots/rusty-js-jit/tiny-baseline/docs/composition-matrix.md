# TB-EXT 4 — composition matrix

*N=5 per (bench × config); median ns/iter. Generated 2026-05-23T08:12:27-07:00.*

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| none | 136.1 | 144.4 |
| TB | 71.3 | 81.7 |
| STUB | 124.1 | 143.7 |
| VTI | 124.0 | 740.2 |
| TB+STUB | 71.1 | 81.4 |
| TB+VTI | 70.9 | 748.4 |
| STUB+VTI | 124.1 | 763.9 |
| TB+STUB+VTI | 71.4 | 791.3 |

## Per-flag contribution (delta from `none`)

