# temporal-foundation — Trajectory

## TF-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

### Trigger

Rule 23 verification at TN-EXT 0 surfaced that Temporal.Now's 3 tests are NOT the apparatus-validation rung the parent locale described — they all require IANA-TZ-string parsing. Spawned this foundation rung as the actual smallest viable substrate move.

### Edit (~50 LOC in pilots/rusty-js-runtime/derived/src/intrinsics.rs)

- New `install_temporal()` function, called from `install_intrinsics` after `install_json()`.
- Allocates `Temporal` namespace object + `Temporal.Now` sub-object + 8 class stubs (Instant, PlainDate, PlainTime, PlainDateTime, PlainMonthDay, PlainYearMonth, Duration, ZonedDateTime).
- Each Now method (6 total) registers as a TypeError-throwing stub with message `Temporal.Now.X not implemented (Tier-L stub)`.
- Each class stub gets `@@toStringTag` set to "Temporal.X".

### Yield

- Apparatus validated. No test yield (no exemplar surface; foundation is pure infrastructure).
- Diff-prod: 42/42 maintained.
- The 3 Temporal.Now tests remain SKIPped via RFSDO-EXT 2's `Temporal` flag (downstream rungs will flip the SKIP decision).

### Findings

**Finding TF.1 (Rule 23 surfaced a missing rung)**: TN-EXT 0's parent locale described temporal-now as "smallest viable, 3 tests; validates apparatus before bigger classes." Rule 23 verification-probe — reading the 3 tests — revealed they require IANA-TZ string parsing, which is substantial substrate by itself. The actual smallest viable rung is `temporal-foundation`: namespace + class stubs, no semantic methods, no test yield, but apparatus validated. Standing recommendation: when a parent locale's "smallest first" rung sequence is articulated from test-count alone, Rule 23 verification at each rung's founding may surface that test-count is not test-difficulty; insert a foundation rung if needed.

**Finding TF.2 (stub-method-throws-TypeError is the handshake between foundation and per-class rungs)**: A class-rung's substrate work overwrites the stub with the real implementation; the TypeError text identifies which methods are still stubbed at any point during the program. This is a self-documenting progress signal — running the failing Temporal tests at any point shows exactly which substrate is missing. Standing recommendation: for multi-rung programs that install structured surfaces (Temporal, future Intl, future Atomics), use throw-with-named-stub-message as the handshake instead of installing nothing.

### Status

TF-EXT 1 CLOSED. Foundation in place; per-class rungs can now install methods by name. Next: temporal-now (substantial — requires temporal-tz-string-parse or equivalent substrate first; consider spawning that as a sibling sub-locale).
