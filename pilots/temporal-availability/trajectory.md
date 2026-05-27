# temporal-availability — Trajectory

## TA-EXT 0 — founding + exemplar suite + baseline (2026-05-25)

**Trigger**: Per keeper directive after the zoom-out read of the canonical full-suite Pin-Art matrix surfaced this coordinate as rank #1 (4,152 fails, ~17.4% of interpreted non-pass).

**Apparatus established**:

- `exemplars/exemplars.txt` — 100 paths stratified-sampled from the 4,152-fixture pool by Temporal sub-class. Class proportions match the underlying pool: ZonedDateTime 20 / PlainDateTime 17 / PlainDate 14 / Duration 12 / PlainYearMonth 10 / PlainTime 10 / Instant 10 / PlainMonthDay 3 / Now 1 / toStringTag 1 / keys.js 1 / getOwnPropertyNames.js 1. Sampled with a fixed seed (0xC0FFEE) for reproducibility.
- `exemplars/run-exemplars.sh` — harness wrapper runner; prints aggregate + per-class breakdown of fails.

**Baseline measurement**:

| Probe | Result |
|---|---|
| Exemplar suite (100 / 4,152 pool) | **PASS=0, FAIL=100 (0.0%)** |
| Top three uncovered classes | ZonedDateTime (20), PlainDateTime (17), PlainDate (14) |
| All 12 Temporal sub-classes uncovered | confirmed |

The 0/100 baseline confirms the cluster's single-decision shape: `globalThis.Temporal` is unbound. Every fail in the cluster surfaces as `ReferenceError`-like at the `availability/missing-global-or-binding` cut. Closing the availability axis at the runtime intrinsic-registration tier is the deeper-layer move (R13 prospective C1-C4 all hold per seed §Methodology).

**Findings**

**Finding TA.1 (single-decision avalanche)**: 4,152 fails behind one missing-global-binding decision. Cluster-coordinate yield-per-decision ratio is empirically extreme here — even a stub registration with no method implementations should flip the cluster's failure-mode distribution off the availability axis and onto the value-semantics/wrong-result axis. The shift itself is the signal that TA-EXT 1 lands the deeper-layer move; the absolute pass count is a secondary read.

**Finding TA.2 (exemplar-suite stratification preserves cluster structure)**: proportional sampling with min-1-per-class produces a 100-test surface that mirrors the 4,152-pool's class distribution. Per-class fail breakdown after TA-EXT 1+ will read directly against the pool's expected yield curve. Standing recommendation: when sampling exemplars from a tier-A cluster, stratify by the most-load-bearing axis of the cluster (here: Temporal sub-class), not by uniform random pick.

**Status**: TA-EXT 0 CLOSED. Apparatus operational; baseline pinned. TA-EXT 1 (registration MVP) is the next rung.

## TA-EXT 1 — Temporal namespace registration MVP (2026-05-26)

**Trigger**: `intl402-availability` reached I402-EXT 17 with no visible
core non-Temporal Intl402 failures left in its exemplar slice. The remaining
Intl402 residual is Temporal or Temporal-coupled DateTimeFormat mass, so the
coherent next move is to extend this already-founded Temporal availability
locale rather than spawn a duplicate Intl/Temporal sibling.

**Change**:

- Added a runtime `Temporal` namespace at the intrinsic registration tier.
- Installed constructor-shaped stubs for `Duration`, `Instant`, `PlainDate`,
  `PlainDateTime`, `PlainMonthDay`, `PlainTime`, `PlainYearMonth`, and
  `ZonedDateTime`.
- Installed `Temporal.Now` with method-shaped stubs for the ISO current-time
  entry points.
- Kept namespace and class/prototype metadata non-enumerable and installed
  `@@toStringTag` so the availability layer matches the built-in namespace
  shape before class semantics are attempted.

**Local verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
target/debug/cruft <temporal-smoke.mjs>
```

Smoke result:

```json
{"temporal":"object","keys":0,"names":"@@toStringTag|Duration|Instant|Now|PlainDate|PlainDateTime|PlainMonthDay|PlainTime|PlainYearMonth|ZonedDateTime","tag":"[object Temporal]","plainDate":"function","now":"function"}
```

The local checkout does not have `env.local` / `T262_ROOT`, so
`exemplars/run-exemplars.sh` could not be re-run here. The next resolver
with a configured test262 checkout should run both
`pilots/temporal-availability/exemplars/run-exemplars.sh` and
`pilots/intl402-availability/exemplars/run-exemplars.sh` to measure the
availability-axis shift.

**Finding TA.3 (extension beats spawn here)**: After I402-EXT 17, the
remaining ECMA-402 exemplar mass no longer names a core Intl substrate
coordinate. It names the Temporal prerequisite that this locale already
owns. Per rule 4 and the locale-positioning audit's apparatus-tax warning,
the correct move is a TA-EXT 1 extension, not a new sibling locale.

**Status**: TA-EXT 1 LANDED locally; exemplar re-measurement pending
configured `T262_ROOT`. TA-EXT 2 should inspect the post-registration
Temporal exemplar failure table and choose the first class-local semantic
coordinate. Spawn a nested locale only if that class-local coordinate has
multi-rung shape.
