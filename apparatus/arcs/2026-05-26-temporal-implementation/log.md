# 2026-05-26-temporal-implementation — log

Append-only event log. One entry per rung landing or directive received.

## 2026-05-26 morning — arc opens

- Telegram 9873: keeper directive "Let's add a track for implementing Temporal" after RFSDO-EXT 2 SKIPped 6,694 Temporal records.
- Spawned `temporal-implementation/` parent + first sub-locale `temporal-now/`. Original 9-rung sequence articulated.
- TI-EXT 1 restructure per Telegram 9879: per-class topology with nested parents + 4 shared sub-substrates planned. RFSDO sync protocol formalized.

## 2026-05-26 — foundation + Duration

- TF-EXT 1: temporal-foundation (namespace + class stubs). Rule 23 redirect: Now's 3 tests need IANA TZ parsing; spawn foundation as actual smallest-viable rung. 0 in-locale yield; apparatus validated.
- DCF-EXT 1: Duration ctor + 10 getters + valueOf-throws. 64/67 PASS (95.5%). Two-stage Rule 23: missing ctor.prototype property surfaced via verification probe; +39 yield from one line.
- DDP-EXT 1: sign + blank + abs + negated. 23/24.
- DStat-EXT 1: from + compare. 22/81 (deferred residuals: ISO-string + relativeTo).
- DWith-EXT 1: with. 17/22.
- DSV-EXT 1: cross-cutting uniform-sign validation. +3 across siblings.

## 2026-05-26 — Instant + shared ISO substrates

- TInst-EXT 1: instant-ctor-fields. 21/25 PASS (84%). Pattern transfers cleanly from Duration.
- TIS-EXT 1: instant-static (from + fromEpochMilliseconds + fromEpochNanoseconds + compare). 28/81.
- IDP-EXT 1: iso-duration-parse shared substrate. +5 across Duration siblings.
- IDTP-EXT 1: iso-datetime-parse shared substrate. +25 sibling yield on instant-static. Two-stage Rule 23: discovered + fixed cruft's latent ymd_to_ms month-convention bug (DMCF spawned + landed inline).

## 2026-05-26 — PlainTime full

- PTCF-EXT 1: ctor + 6 getters + valueOf-throws. 32/34 (94%).
- PTS-EXT 1: from + compare with ISO time parser. 46/83.
- PTW-EXT 1: with. 12/22.
- PTSC-EXT 1: toString/toJSON/toLocaleString. 26/54.
- PTE-EXT 1: equals. 21/31.
- PTA-EXT 1: arithmetic (add/subtract/since/until with 24h wrap). 82/214.

## 2026-05-26 — PlainDate

- PDCF-EXT 1: ctor + 5 getters + leap-aware day validation. 28/38.
- PDS-EXT 1: from + compare + ISO date parser. 49/113.
- PDSC-EXT 1: toString/toJSON/toLocaleString. 27/33.
- PDE-EXT 1: equals. 18/40.
- PDDP-EXT 1: 11 calendar getters (dayOfWeek/dayOfYear/inLeapYear/weekOfYear/yearOfWeek/...). 27/33.
- PDA-EXT 1: calendar arithmetic (add/subtract/since/until). 79/248.
- PDW-EXT 1: with. 10/25.

## 2026-05-26 — PDT + IFP

- IFP-EXT 1: fractional propagation in ISO duration parser. +5 cross-sibling.
- TTSTD-EXT 1: @@toStringTag descriptor fix. 0 yield (SKHB blocker).
- PDTCF-EXT 1: PDT ctor + 9 getters. 48/56.
- PDTSC + PDTE: PDT string-conversion + equals. 31/64 + 20/41.
- PDTS-EXT 1: PDT static + sibling parser switch. Broke 1000 PASS milestone (1008/1969). 56/112.
- PDTDP-EXT 1: PDT 11 calendar getters. 30/33.
- PDTW-EXT 1: PDT with. 12/30.
- PDTA-EXT 1: PDT arithmetic. 100/275.

## 2026-05-27 (post-midnight) — PMD/PYM/conversions/ZDT

- PMDCF-EXT 1: PlainMonthDay ctor + 3 getters + toString. 39/51.
- PYMCF-EXT 1: PlainYearMonth ctor + 10 getters + toString. 62/75.
- PDC-EXT 1: PD conversion methods (toPlainDateTime/toPlainMonthDay/toPlainYearMonth). 20/50. Rule 23 redirect: cross-class needs late binding to target prototypes.
- PYMS + PYME: PYM from + compare + equals. 34/103 + 19/40.
- PMDS + PMDE + PMDW: PMD from + equals + with. 12/59 + 12/36 + 6/21.
- Cross-class conversions (PDT.toPlainDate/Time, PMD/PYM.toPlainDate). 29/39.
- ZDTCF-EXT 1: ZonedDateTime v1 ctor + 4 getters. 25/27.

## Close-condition met

All 8 Temporal classes operational at ctor + at least 1 sub-rung. Arc CLOSED.

Final cumulative: **1408 PASS / 2808 exemplars opted in (50.1%)**.

## Subsequent directives that did NOT extend this arc

- Telegram 9965 ("Should we continue on temporal or...") → diminishing-returns assessment; led to Tier M registration.
- Telegram 9967 ("Let's ensure these are in the manifest for candidates") → Tier-M candidates registered.
- Telegram 9969 ("formalize the arc as indicating many locales...") → spawned `arc-as-coordinate` apparatus doc + this backfill.
