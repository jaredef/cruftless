# temporal-availability/plain-datetime-semantics — Seed

## Telos

Materialize the nested Temporal coordinate:

```text
runtime/spec-builtins :: E3/intrinsic-object:temporal ::
PlainDateTime/semantic-availability :: object-shape-and-calendar-time-arithmetic
```

The parent `temporal-availability` locale closed the missing-global
coordinate and advanced the direct Temporal exemplar suite from 0/100 to
71/100. The remaining largest direct bucket is `PlainDateTime` (8 rows).
This nested locale exists because the residual is no longer a binding
availability problem. It is a multi-rung semantic surface spanning
duration arithmetic, rounding, calendar conversion, equality coercion,
and PlainTime string conversion.

## Apparatus

- **Parent suite**:
  `pilots/temporal-availability/exemplars/exemplars.txt`.
- **Focused suite**:
  `pilots/temporal-availability/plain-datetime-semantics/exemplars/exemplars.txt`.
- **Focused runner**:
  `pilots/temporal-availability/plain-datetime-semantics/exemplars/run-exemplars.sh`.
- **Substrate site**:
  `pilots/rusty-js-runtime/derived/src/intrinsics.rs`, Temporal
  intrinsic registration and helper tier.
- **Baseline measurement (founding)**: PASS=0 / 8, FAIL=8 / 8 in the
  focused PlainDateTime residual, inherited from parent TA-EXT 18.

Focused residual at founding:

```text
Temporal.PlainDateTime.prototype.add/argument-string-fractional-units-rounding-mode.js
Temporal.PlainDateTime.prototype.equals/argument-plaindate.js
Temporal.PlainDateTime.prototype.round/roundingmode-halfFloor.js
Temporal.PlainDateTime.prototype.since/float64-representable-integer.js
Temporal.PlainDateTime.prototype.since/roundingmode-halfExpand.js
Temporal.PlainDateTime.prototype.since/roundingmode-halfexpand-default-changes.js
Temporal.PlainDateTime.prototype.withCalendar/calendar-case-insensitive.js
Temporal.PlainDateTime.prototype.withPlainTime/argument-string-without-time-designator.js
```

## Methodology

Standing rule 13 applies in the same prospective shape as the parent
locale, but the deeper-layer closure is now semantic. Avoid treating
every row as a one-off stub. Group by mechanism:

- **PDTS-EXT 0**: founding and residual extraction.
- **PDTS-EXT 1**: PlainTime string conversion into `withPlainTime`,
  likely sibling to TA-EXT 18's PlainTime parser.
- **PDTS-EXT 2**: equality coercion between PlainDateTime and PlainDate
  where the sampled calendar/date slots align.
- **PDTS-EXT 3**: duration arithmetic and rounding surface
  (`add`, `round`, `since`) if the focused residual proves to share a
  common duration-balancing helper.
- **PDTS-EXT 4**: calendar identifier canonicalization / rejection,
  with attention to Intl402 dependency boundaries.

Spawn deeper sub-locales only if one of those mechanisms expands beyond
three coherent rungs or crosses into a distinct substrate tier.

## Carve-outs

- Full Temporal calendar and timezone semantics remain outside this
  nested locale unless a focused PlainDateTime row requires the minimal
  sampled behavior.
- Intl402 DateTimeFormat formatting of PlainDateTime objects remains in
  `intl402-availability`; this locale may unblock object shape, but not
  locale-sensitive presentation.
- Broad full-suite conformance percentages are not the telos. The local
  measure is focused residual closure plus parent-suite non-regression.

## Composes-with

- `pilots/temporal-availability/seed.md` — parent availability locale.
- `pilots/intl402-availability/seed.md` — sibling Intl402 dependency.
- `apparatus/docs/standing-rule-13-prospective-application.md`.

## Resume protocol

Read this seed, then `trajectory.md`. Run the focused exemplar runner,
then the parent Temporal and Intl402 exemplar runners before closing a
rung.
