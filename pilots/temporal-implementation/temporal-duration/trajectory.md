# temporal-duration — Trajectory

## TDur-EXT 0 — FOUNDED (2026-05-26)

Spawned per keeper directive (Telegram 9881). First per-class parent in the Temporal program. Pool: 559 tests across built-ins + intl402. Sub-rung topology declared in seed.md.

## TDur-EXT 1 — duration-ctor-fields LANDED (2026-05-26)

See `pilots/temporal-implementation/temporal-duration/duration-ctor-fields/trajectory.md` for the substrate move detail. Summary:

- Real Duration constructor + 10 accessor-property getters + valueOf-throws-TypeError + ctor.prototype + @@toStringTag installed on Temporal.Duration (overwrites the foundation stub).
- Yield: 64/67 (95.5%) on the ctor-fields exemplar surface.
- RFSDO-EXT 3 (PARTIALLY_IMPLEMENTED map) added; Duration test paths opt OUT of the Temporal SKIP.

Residuals at this rung (3):
- 2 getter-trace tests (probe brand-check observer pattern; needs a different prototype-method install)
- 1 `Temporal.Duration.from` not callable (next rung, duration-static)
