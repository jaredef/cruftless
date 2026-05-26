# duration-derived-properties — Trajectory

## DDP-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Spawned per keeper directive (Telegram 9883). Second sub-rung in temporal-duration; sibling to DCF-EXT 1.

### Edit (~110 LOC in intrinsics.rs::install_temporal)

Refactored two helpers inline within the install closure:
- `read_units(rt, this_id)` — brand-check + read all 10 sentinels; returns `[f64; 10]` or TypeError.
- `make_duration(rt, proto, units)` — allocate a new instance with proto + 10 sentinels.

Then added four members on dur_proto:
- `sign` accessor: brand-check + read units; return `units.iter().find(|&&u| u != 0.0).map_or(0.0, |&u| u.signum())`.
- `blank` accessor: brand-check + read units; return `units.iter().all(|&u| u == 0.0)`.
- `abs()` method: brand-check + read units; mutate to `u.abs()`; allocate new Duration.
- `negated()` method: brand-check + read units; mutate to `-u` (preserving -0 → 0 normalization); allocate new Duration.

### Edit (~5 LOC in runner.mjs)

RFSDO PARTIALLY_IMPLEMENTED Temporal allowlist extended with 4 path-prefixes:
- `/Temporal/Duration/prototype/sign/`
- `/Temporal/Duration/prototype/blank/`
- `/Temporal/Duration/prototype/abs/`
- `/Temporal/Duration/prototype/negated/`

### Probes (Rule 23 verification at landing)

- `new Temporal.Duration(1,2,3,...).sign` → 1 ✓
- `new Temporal.Duration(-1,...).sign` → -1 ✓
- `new Temporal.Duration().sign` → 0, `.blank` → true ✓
- `d.abs().{years,months,weeks,hours}` → all positive ✓
- `d.negated().{years,months,hours}` → signs flipped ✓

### Yield

- duration-derived-properties exemplar pool (24): **0 → 23/24 PASS (95.8%)**.
- duration-ctor-fields stable at 64/67 (no regression).
- Diff-prod: 42/42 maintained.

Single residual: `Temporal.Duration.from` not callable (one test in abs/negated probes it); belongs to duration-static rung.

### Cumulative Temporal yield (after DDP)

DCF + DDP: 87 PASS across 91 exemplars (95.6%) on temporal-duration sub-rungs landed so far.

### Findings

**Finding DDP.1 (factoring read_units + make_duration as inline helpers within install_temporal works well for sibling-rung implementation)**: The two helpers are reused by every method/getter that needs unit access + every method that returns a new Duration. This pattern will repeat for arithmetic / round / total / with. Standing recommendation: keep helpers inline within the install closure rather than module-level — they have closure-captured access to proto and other install-scope identifiers; refactor to module-level only when the install function exceeds ~500 LOC.

### Status

DDP-EXT 1 CLOSED. Next: duration-static (from / compare) — would close the single residual here + ~60 more tests. Or duration-string-conversion (toString / toJSON / toLocaleString) — needs temporal-iso-string-parse first.
