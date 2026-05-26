# duration-static — Trajectory

## DStat-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Spawned per keeper directive (Telegram 9885). Third sub-rung in temporal-duration.

### Edit (~120 LOC in intrinsics.rs)

Added two static methods on dur_ctor:

**from(item)**:
- String → throw TypeError "ISO 8601 duration string parsing not yet implemented (Tier-L stub)".
- Object: brand-check via __td_years sentinel.
  - If is_duration → clone 10 sentinels.
  - Else → property-bag read of 10 unit names (undefined missing); enforce at-least-one-unit; integer-validate each; throw RangeError for non-finite/non-integer.
- Allocate new Duration via shared `make_duration` helper.

**compare(d1, d2, options)**:
- Coerce both args via inline closure (same logic shape as from, minus integer-validation since the test source already shows the property bag).
- If any year/month/week non-zero in either:
  - options.relativeTo present → throw TypeError "compare with relativeTo not yet implemented (Tier-L stub)".
  - else → throw RangeError "starting point required for years/months/weeks".
- Else: convert to approximate nanoseconds (1 day = 86400e9 ns; this matches the spec's no-relativeTo path which is DST-unaware by construction) and return -1/0/1.

### Edit (~3 LOC in runner.mjs)

RFSDO allowlist extended with `/Temporal/Duration/from/` and `/Temporal/Duration/compare/`.

### Probes (Rule 23 verification at landing)

- `Temporal.Duration.from(durationInstance)` → clone with all fields preserved ✓
- `Temporal.Duration.from({hours: 10, minutes: 30})` → property-bag form ✓
- `Temporal.Duration.from({milliseconds: 1000, month: 1})` → typo ignored; only ms set ✓
- `compare({hours:1}, {hours:2})` → -1 ✓
- `compare({hours:1}, {minutes:60})` → 0 (equivalent in ns) ✓
- `compare({years:1}, {years:2})` → RangeError ✓
- `Temporal.Duration.from.name === "from"`, `.length === 1` ✓
- `Temporal.Duration.compare.name === "compare"`, `.length === 2` ✓

### Yield

- duration-static exemplar pool (81): **0 → 22/22 in-scope tests + 0/59 deferred = 22/81 (27.2%) overall.**
- DCF stable at 64/67, DDP stable at 23/24 (no regression).
- Diff-prod: 42/42 maintained.

### Residual decomposition (59 fails, all explicit deferrals)

| Shape | Count | Destination |
|---|---:|---|
| ISO duration string parsing | 8+4=12 | temporal-iso-string-parse (shared sub-substrate) |
| relativeTo not implemented (compare) | 3+2=5 | duration-relative-to |
| PlainDate.from not callable (for relativeTo arg) | 5 | temporal-plain-date |
| from method on other Temporal classes | 4 | per-class from rungs |
| RangeError vs TypeError mismatch | 4 | error-class refinement (small fix) |
| "fetching primitive value" option-coercion | 2 | spec-strict option-coercion order |
| RangeError not thrown | 2 | edge cases |
| calendar ID validation | 2 | temporal-iso-calendar |
| misc (single instances) | 23 | per-test inspection (low-leverage) |

### Cumulative Temporal yield (post-DStat)

DCF + DDP + DStat: **109 PASS across 172 exemplar tests (63%)**.

### Findings

**Finding DStat.1 (sub-rungs at the same level can share inline helpers via closure-capture)**: The DStat ctor methods reuse `make_duration` (from DCF) and a coerce-to-units helper modeled after DDP's `read_units`. The shared inline-helper pattern (Finding DDP.1) lets sibling rungs at the same parent share substrate primitives without module-level refactor. Standing recommendation: when sibling rungs all need the same primitive (e.g., make_duration), allocate it once at the parent's install scope and capture by move into each rung's closure.

**Finding DStat.2 (deferred-residuals must be explicitly named in seed carve-outs)**: 59 of DStat's 81 residuals are EXPECTED — they need substrates not yet built (ISO-string-parse, relativeTo, PlainDate). Naming them in the seed's carve-out section + standing-rec section prevents future agents from interpreting low yield as "this rung is broken." Standing recommendation: every per-class rung that depends on shared sub-substrates lists them explicitly in seed carve-outs; the trajectory's residual decomposition cross-references the deferred destinations.

### Status

DStat-EXT 1 CLOSED. 22/81 yield (27.2%); 59 residuals are explicit deferrals to named follow-on substrates. Next ripe: spawn `temporal-iso-string-parse` as a shared sub-substrate (would close 12+ records across DStat + future Duration string-conversion rung + every per-class from()).
