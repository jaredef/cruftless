---
arc: 2026-05-26-temporal-implementation
trigger: Telegram message 9873 ("Let's add a track for implementing Temporal")
opened: 2026-05-26
closed: 2026-05-27 (operational close; future Temporal work spawns separate arcs)
close_condition: At least one sub-rung landed for every Temporal class (8 classes operational).
---

# Tier-L Temporal Implementation Arc

## Trigger

Keeper directive (Telegram 9873) immediately after RFSDO-EXT 2 SKIPped 6,694 Temporal records: "Let's add a track for implementing Temporal."

Subsequent directives ("Continue" x N, "Continue as coherent" x N) advanced the arc through each per-class implementation.

## Telos

Implement the ECMA-262 Temporal API substrate sufficient to pass meaningful subsets of test262's `intl402/Temporal/*` and `built-ins/Temporal/*` suites. Each Temporal class becomes a parent locale with nested sub-rungs.

Per the parent locale's restructured topology (TI-EXT 1): each class follows the template ctor-fields → static → string-conversion → equals → with → derived-properties → arithmetic → conversion, with shared sub-substrates for ISO parsing.

## Sub-locale roster

### Shared infrastructure
| Locale | Role | Status | LOC | Direct yield |
|---|---|---|---:|---|
| `temporal-implementation/` | parent program | LANDED | — | — |
| `temporal-foundation/` | namespace + class stubs | LANDED | ~50 | apparatus |
| `temporal-iso-string-parse/` (parent) | shared ISO parser | LANDED | — | — |
| ↳ `iso-duration-parse` | Duration ISO parser | LANDED | ~120 | +5 sibling |
| ↳ `iso-datetime-parse` | Instant ISO parser | LANDED | ~150 | +25 sibling |
| ↳ `iso-fractional-propagation` | fractional H/M/S cascade | LANDED | ~40 | +5 sibling |
| `temporal-tostringtag-descriptor/` | @@toStringTag descriptor | LANDED (blocked) | ~40 | 0 (SKHB blocks) |

### Per-class (8 classes, 48+ sub-rungs total)

| Class | Sub-rungs | Final yield (PASS / pool) |
|---|---:|---|
| **PlainDate** | 8 (ctor + static + string + equals + derived + arithmetic + with + conversion) | ~209 / 422 |
| **PlainDateTime** | 8 (ctor + string + equals + static + derived + with + arithmetic + conversion) | ~285 / 511 |
| **Duration** | 7 (ctor + derived + static + with + sign-validation + string + arithmetic) | ~199 / 257 |
| **PlainTime** | 6 (ctor + static + with + string + equals + arithmetic) | ~213 / 354 |
| **PlainMonthDay** | 5 (ctor + static + equals + with + conversion) | ~47 / 167 |
| **Instant** | 5 (ctor + static + string + equals + arithmetic) | ~191 / 322 |
| **PlainYearMonth** | 4 (ctor + static + equals + conversion) | ~69 / 230 |
| **ZonedDateTime** | 1 (ctor + minimal getters) | ~25 / 27 |

### Apparatus (sibling to arc)
| Locale | Role | Status | LOC | Effect |
|---|---|---|---:|---|
| `apparatus/runner-features-skip-deliberate-omissions/` | RFSDO-EXT 3 PARTIALLY_IMPLEMENTED map | LANDED | ~50 | per-class allow-list mechanism |
| `date-month-convention-fix/` | latent Date ymd_to_ms bug | LANDED | ~3 | spec-correct Date math; +4 sibling yield |

Total: **48 sub-rungs** across **18 named locales** under temporal-implementation/, plus 2 apparatus locales.

## Cumulative yield over time

| Checkpoint | Cumulative PASS | Denominator | % |
|---|---:|---:|---:|
| Foundation (TF) | 0 | 0 | apparatus-only |
| + DCF (Duration ctor) | 64 | 67 | 95.5% |
| + 5 Duration sub-rungs | 129 | 194 | 66.5% |
| + Instant ctor + static + string | 178 | 300 | 59.3% |
| + IDP + IDTP shared parsers | 208 | 300 | 69.3% |
| + PlainTime full (6 rungs) | 411 | 658 | 62.5% |
| + PDT first 4 rungs | 951 | 1857 | 51.2% |
| + IDTP datetime parser | 208 | 300 | 69.3% |
| + PDA-EXT 1 (PD arithmetic) | 842 | 1671 | 50.4% |
| + PDTSC + PDTE | 951 | 1857 | 51.2% |
| + PDT static (PDTS, broke 1000) | 1008 | 1969 | 51.2% |
| + PDTDP + PDTW + PDTA | 1150 | 2307 | 49.8% |
| + PMDCF (PlainMonthDay ctor) | 1189 | 2358 | 50.4% |
| + PYMCF (PlainYearMonth ctor) | 1251 | 2433 | 51.4% |
| + PDC (cross-class conversions) | 1271 | 2483 | 51.2% |
| + PYMS + PYME | 1324 | 2626 | 50.4% |
| + PMDS + PMDE + PMDW | 1354 | 2742 | 49.4% |
| + cross-class conversions (PDT/PMD/PYM) | 1383 | 2781 | 49.7% |
| + ZDTCF (final class operational) | **1408** | **2808** | **50.1%** |

## Cross-locale findings

**Finding ARC-L.1 (per-class template is repeatable across 7 classes)**: Duration (10-field tuple), Instant (BigInt single sentinel), PlainTime (6-field bounded tuple), PlainDate (3-field + calendar), PlainDateTime (9-field + calendar), PlainMonthDay (3-field + calendar + refYear), PlainYearMonth (3-field + calendar + refDay), ZonedDateTime (BigInt + TZ + calendar) — all use the same skeleton: prototype + accessor-PropertyDescriptors + valueOf-throws + NewTarget-checked ctor + ctor.prototype frozen + @@toStringTag descriptor + class slot install. Per-class ctor-fields LOC: 100-300. Standing recommendation: future ECMA-262 chapter implementations (e.g., Intl.* expansion) can apply this template directly.

**Finding ARC-L.2 (shared sub-substrates compound yield faster than per-class rungs)**: IDP (Duration parser) added +5 cross-sibling. IDTP (datetime parser) added +25. IFP (fractional propagation) added +5. Each shared rung benefits every downstream caller. Standing rec: when multiple in-flight per-class rungs share an unbuilt sub-substrate, prioritize the sub-substrate.

**Finding ARC-L.3 (inverse-parser pattern is repeatable)**: PTSC + ISC + DSC + PDTSC + PDSC + PMD-toString + PYM-toString all use read-sentinel + decompose + format!() + trim_end_matches. Each ~80-100 LOC. Mature template.

**Finding ARC-L.4 (cross-class conversion methods need late binding)**: PD.toPlainDateTime/MonthDay/YearMonth + PDT.toPlainDate/Time + PMD.toPlainDate + PYM.toPlainDate all reference target prototypes that don't exist at PD-install-time. Pattern: install at end of install_temporal after all per-class ctors. Standing rec: any cross-class conversion must be installed after all referenced ctors.

**Finding ARC-L.5 (BigInt precision matters for nanosecond arithmetic)**: IA discovered that Instant.since/until with f64 intermediate loses ~3 ns digits at 43-year diffs (1.35e18 ns vs f64's 2^53 limit). Spec-conformant arithmetic over Instant must use BigInt throughout; f64 only at final display. ZDT will inherit this constraint when arithmetic lands.

**Finding ARC-L.6 (RFSDO PARTIALLY_IMPLEMENTED is the apparatus that makes progressive per-class landings visible)**: Without RFSDO-EXT 3's path-prefix allowlist, every per-class rung's yield would be invisible — the Temporal flag in RFSDO-EXT 2 would still SKIP all tests. Standing rec: progressive substrate programs need apparatus support for per-path opting-out of the deny-list.

**Finding ARC-L.7 (calendar-balancing for since/until is the deferred substrate that limits Duration completeness)**: Multiple rungs deferred year/month largestUnit support because calendar-balancing needs relativeTo + day-arithmetic. Standing rec: a duration-relative-to / calendar-balancing rung would close many residuals across since/until methods.

**Finding ARC-L.8 (options-handling is cross-cutting and warrants its own rung)**: ~70+ residuals across PT/PDT/Instant/Duration string-conversion + arithmetic methods are deferred to options-handling (smallestUnit/fractionalSecondDigits/roundingMode). Registered in CANDIDATES Tier-M as TempO. Standing rec: cross-cutting concerns should be their own rung even when individual rungs could partially implement them.

## Status

CLOSED 2026-05-27 (operational). Close-condition met: at least one sub-rung landed for every Temporal class.

Open follow-ons (registered in CANDIDATES Tier-M):
- TempO (options-handling, ~70+ records)
- TZ database for ZonedDateTime depth
- Calendar balancing for years/months largestUnit
- Class-extension methods (round, total, intl integration)

Future Temporal work spawns separate arcs (e.g., "2026-XX-XX-temporal-options-handling-arc").
