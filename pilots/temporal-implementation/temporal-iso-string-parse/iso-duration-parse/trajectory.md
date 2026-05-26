# iso-duration-parse — Trajectory

## IDP-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

First shared-substrate rung in the Temporal program.

### Edit (~120 LOC in intrinsics.rs)

`parse_iso_duration(&str) -> Option<[f64; 10]>` hand-written state-machine parser:
- Optional sign (+/-/U+2212).
- Mandatory `P` (case-insensitive).
- `consume_part` helper: iterates designators in fixed order; each parsed number followed by its designator advances `next_d` past consumed designators (enforcing order + uniqueness).
- `parse_number` helper: parses integer part + optional `.`/`,` fractional part; returns `has_fractional` flag.
- `fractional_taken` Option tracks the slot that holds the fractional; if set, no more designators allowed.
- Date-part designators (Y/M/W/D → slots 0/1/2/3); time-part designators (H/M/S → slots 4/5/6).
- Validates: at-least-one designator present; `T` without time designators rejected; parser must reach end of string.

Wired into:
- `Temporal.Duration.from(string)` — replaces the prior deferral; parses + integer-validates + uniform-sign-checks.
- `Temporal.Duration.compare(string)` — replaces the deferral; parses but skips integer-validate (compare handles fractional-rejected case).

### Probes (Rule 23 verification at landing)

- `Temporal.Duration.from("P1Y2M3W4D")` → {years:1, months:2, weeks:3, days:4} ✓
- `Temporal.Duration.from("PT1H30M")` → {hours:1, minutes:30} ✓
- `Temporal.Duration.from("P1Y2M3W4DT5H6M7S")` → all 7 fields set ✓
- `Temporal.Duration.from("-P1Y")` → years=-1 ✓
- `Temporal.Duration.from("invalid")` → RangeError ✓
- `Temporal.Duration.from("P")` → RangeError (no designator) ✓
- `Temporal.Duration.from("PT")` → RangeError (T without time designator) ✓
- `Temporal.Duration.from("P1H1Y")` → RangeError (wrong order: H is time-only, can't follow P without T) ✓

### Yield

- duration-static: 23 → 27 (+4)
- duration-derived-properties: 23 → 24 (+1) — a from-string roundtrip test
- duration-ctor-fields: 64 unchanged
- duration-with: 19 unchanged
- **Total +5 across Duration sub-rungs**

Diff-prod: 42/42.

Cumulative Temporal yield post-IDP: 183/300 (61%).

### Residual decomposition (7 fails specifically attributable to my parser)

| Shape | Cause | Destination |
|---|---|---|
| `fractional unit out of position` | tests use fractional H/M which spec allows (with propagation to seconds); my parser stores fractional in unit slot and caller's integer-validate rejects | iso-fractional-propagation sub-rung |

Other DStat residuals (~50) are explicit deferrals to other sub-substrates (compare-relativeTo, PlainDate.from, calendar IDs) — not IDP's territory.

### Findings

**Finding IDP.1 (hand-written state-machine parsers fit per-rung budget when grammar is regular)**: ~120 LOC for a 4-designator + sign + fractional ISO duration parser. Comparable parsers for ISO datetime (more grammar variants) likely need 200-300 LOC. Standing recommendation: when a parser's BNF is regular (no recursion, fixed designator alphabet), prefer hand-written state-machine over a parser-combinator library — the LOC budget fits per-rung and the dependency surface is zero.

**Finding IDP.2 (shared-substrate rungs unblock siblings without exemplar surface of their own)**: IDP's "yield" is measured by the +5 across sibling Duration rungs, not by passing IDP-owned tests. Standing recommendation for parent trajectory: track shared-substrate yield as a cross-sibling delta in the parent locale's status; don't expect leaf shared-substrate rungs to have meaningful in-locale exemplar counts.

### Status

IDP-EXT 1 CLOSED. Duration ISO string parse functional. Next sub-rung in temporal-iso-string-parse: iso-datetime-parse (would unlock 15+ Instant from-string + 9 Instant compare-string + foundation for PlainDate.from / PlainDateTime.from / etc.).
