# intl402-availability — Seed

**Locale tag**: `L.intl402-availability`

**Status**: FOUNDED 2026-05-26. Spawned from the full-suite Pin-Art
ECMA-402 rung after confirming `origin/main` is current at `b0dcd968`.

## Telos

Materialize the engine-DAG coordinate family:

```text
host-intrinsic/intl402 :: E3/intrinsic-object:ecma-402 :: *
```

The current full-suite Pin-Art matrix reports **3,045** interpreted
ECMA-402 failures. The top pins are:

```text
2008 availability/missing-global-or-binding
 382 value-semantics/wrong-result
 259 availability/missing-method-or-intrinsic
  73 abrupt-completion/throw-missing RangeError
  70 abrupt-completion/throw-missing TypeError
```

Cruft already exposes `Intl` plus constructor-shaped stubs for
`DateTimeFormat`, `NumberFormat`, `Collator`, `PluralRules`,
`RelativeTimeFormat`, `ListFormat`, `Segmenter`, `DisplayNames`, and
`Locale` in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`. This
locale is therefore not a pure missing-namespace lane. It is the move
from **consumer-survival stubs** to **ECMA-402-conformant intrinsic
objects**.

## Current Matrix Shape

The matrix splits into two strata:

1. **Temporal-dependent ECMA-402 rows**: `intl402.Temporal` contributes
   **2,029** failures. These are not simply Intl formatting bugs; many
   require real Temporal calendar/timezone objects. They compose with
   `pilots/temporal-availability/` and should not be mistaken for the
   first 402 substrate rung.
2. **Core Intl rows**: `NumberFormat`, `DateTimeFormat`, `Locale`,
   `DurationFormat`, `ListFormat`, `Segmenter`, `RelativeTimeFormat`,
   `DisplayNames`, `Collator`, `PluralRules`, and top-level Intl
   functions. These are the first actionable 402 lane.

Top surfaces from the current matrix:

```text
2029 intl402.Temporal
 204 intl402.NumberFormat
 193 intl402.DateTimeFormat
 131 intl402.Locale
 110 intl402.DurationFormat
  62 intl402.Intl
  60 intl402.ListFormat
  60 intl402.Segmenter
  55 intl402.RelativeTimeFormat
  44 intl402.DisplayNames
  34 intl402.Collator
  32 intl402.PluralRules
```

## Apparatus

- **Exemplar suite**: `exemplars/exemplars.txt` — 100 paths stratified
  from the 3,045-failure ECMA-402 pool with fixed seed `0x402`.
  The sample intentionally preserves the `intl402.Temporal` dominance
  while keeping at least one exemplar from every visible Intl class.
- **Exemplar runner**: `exemplars/run-exemplars.sh` — runs the sample via
  the shared Test262 harness wrapper against Cruft, prints aggregate
  pass/fail, then reports failures by Intl surface.
- **Substrate site**: `pilots/rusty-js-runtime/derived/src/intrinsics.rs`
  for namespace/class/prototype/descriptor moves. Locale-data and
  calendar/timezone substrate may deserve nested locales once the
  class-level first rungs are measured.

## Methodology

Standing rule 13 applies in prospective form, but the deeper-layer move
is different from Temporal:

- **C1 sibling closure pattern**: HOLDS. Existing consumer stubs prove the
  namespace/class/prototype registration path. The missing piece is spec
  shape and semantics.
- **C2 substrate fit**: HOLDS for constructor/prototype/descriptor work.
  Locale-data correctness may require new substrate rather than more
  stubs.
- **C3 cost-positive when integrated**: TO BE VERIFIED. The exemplar
  suite is the immediate falsifier. A valid first rung should shift
  failures away from missing method/descriptor shape and into narrower
  locale-data/value-semantics rows.
- **C4 bail safety**: HOLDS. ECMA-402 is host-intrinsic runtime work; no
  parser changes are expected.

## First Rungs

1. **I402-EXT 0**: Founding, exemplar suite, baseline.
2. **I402-EXT 1**: Baseline-inspection pass. Run exemplars, split fails
   into `intl402.Temporal` versus core Intl. Identify the highest-yield
   core class.
3. **I402-EXT 2**: Descriptor/prototype correctness for the chosen core
   class (`NumberFormat` or `DateTimeFormat` likely first).
4. **I402-EXT 3+**: Class-by-class semantic rungs. Spawn nested locales
   when a class requires its own multi-rung substrate.

## Carve-Outs

- Full CLDR/ICU parity is not the first target. The first target is a
  legible, spec-shaped Intl surface whose remaining failures name the
  missing data/algorithm substrate precisely.
- `intl402.Temporal` rows are tracked here but must compose with
  `temporal-availability`. Do not spend 402 substrate effort trying to
  solve rows whose root cause is missing `globalThis.Temporal`.
- Consumer-survival stubs are allowed only as historical context. Future
  moves in this locale should either improve Test262 behavior or sharpen
  the matrix classification.

## Resume Protocol

Read this seed, then `trajectory.md`. Run:

```sh
pilots/intl402-availability/exemplars/run-exemplars.sh
```

Use the result to choose the next core Intl class, keeping
`intl402.Temporal` failures separate from core ECMA-402 failures.
