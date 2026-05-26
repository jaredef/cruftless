# duration-with — Trajectory

## DWith-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Spawned per keeper directive (Telegram 9887). Fourth sub-rung of temporal-duration; sibling shape to DCF/DDP/DStat.

### Edit (~45 LOC in intrinsics.rs)

Added `with(durationLike)` method to dur_proto:
- Brand-check via `read_units(rt, this_id)` (TypeError on non-Duration).
- Argument primitivity check: must be `Value::Object(_)`; else TypeError.
- Loop over 10 unit names; for each present property, ToNumber + finite + integer validate; override `units[i]`.
- Require at-least-one-recognized-unit-name (TypeError if none).
- Allocate new Duration via shared `make_duration`.

### Edit (~3 LOC in runner.mjs)

RFSDO allowlist extended with `/Temporal/Duration/prototype/with/`.

### Probes (Rule 23 verification at landing)

- `d.with({hours: 99, months: 88})` → fields override per-unit; others preserved ✓
- `d.with(undefined)`, `d.with("P1D")`, `d.with(7)` → TypeError ✓
- `d.with({nonsense: 1})`, `d.with({sign: 1})` → TypeError ✓

### Yield

- duration-with exemplar pool (22): **0 → 17/22 PASS (77%)**.
- Diff-prod: 42/42 maintained.
- Cumulative Temporal yield post-DWith: 126 PASS / 194 exemplars (65%).

### Residual decomposition (5)

| Shape | Count | Destination |
|---|---:|---|
| Sign-uniformity ("mixed signs throw RangeError") | 3 | duration-sign-validation follow-on (would close DCF + DStat + DWith residuals together) |
| Option-coercion order ("fails after fetching primitive value") | 2 | spec-strict coercion-order refinement |

### Findings

**Finding DWith.1 (the inline-helper pattern keeps each sibling rung at ~50 LOC)**: DWith is now the third sibling rung (after DCF/DDP/DStat) to reuse `read_units` and `make_duration`. Each new rung costs ~50 LOC for the unique logic plus ~3 LOC for the RFSDO carve-out. Per-class total LOC tracking against the Temporal program's ~900-1100 LOC-per-class estimate: temporal-duration is now at ~330 LOC across 4 sub-rungs (foundation 50 + ctor 110 + derived 110 + static 120 + with 45 = ~435 LOC; with shares 70 LOC of inline helpers). Original estimate was conservative; on-track for ~700-900 LOC total.

**Finding DWith.2 (sign-uniformity validation is a cross-rung residual that warrants its own rung)**: 3 of DWith's 5 residuals are sign-uniformity (Duration's invariant that all non-zero units share sign). DCF + DStat have similar residuals. Standing recommendation: spawn `duration-sign-validation` as a single-rung that adds the uniformity check to ctor / from / with in one place — would close residuals across multiple sibling rungs. Pattern: a cross-cutting validation rule that touches multiple rungs justifies its own rung even though it's not a method.

### Status

DWith-EXT 1 CLOSED. Next ripe: duration-sign-validation (small, cross-cutting close of ~3 residuals here + 4+ elsewhere) OR temporal-iso-string-parse (shared substrate, larger leverage but more substantial).
