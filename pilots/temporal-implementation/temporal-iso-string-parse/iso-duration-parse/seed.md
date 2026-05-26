---
name: iso-duration-parse
description: First sub-rung of temporal-iso-string-parse. Hand-written state-machine parser for ECMA-262 §11.8.1 ISO 8601 duration strings.
type: project
---

# iso-duration-parse — Seed

## Leaf shared-substrate sub-locale under `pilots/temporal-implementation/temporal-iso-string-parse/`.

Consumed by Temporal.Duration.from(string) and Temporal.Duration.compare(string). Indirectly raises yield on those sites.

## Telos

`parse_iso_duration(s: &str) -> Option<[f64; 10]>` parses per §11.8.1:

```
Sign?      'P'      Date?              ('T' Time)?
[+-]?       P      (n 'Y')?
                   (n 'M')?   ← month
                   (n 'W')?
                   (n 'D')?            ('H' | 'M' | 'S' designators; M=minutes here)
```

Returns Some([y, mo, w, d, h, mi, s, ms, us, ns]) on success; None on parse failure (caller throws RangeError).

## Constraints honored

- Sign prefix `+` / `-` / U+2212 (MINUS SIGN) per spec.
- `P` is case-insensitive; `T` is case-insensitive.
- Designators must appear in spec order; each at most once.
- At least one designator required (bare `P` and `PT` reject).
- If `T` is present, at least one time designator required.
- Fractional part allowed on smallest designator only (using `.` or `,` per spec); after a fractional designator, no more designators may follow.

## Constraints DEFERRED (residual ~7 records in DStat)

- **Fractional propagation**: spec allows fractional H/M to propagate into smaller units (e.g., `PT1.5H` → 1h 30m). Current implementation stores fractional in the unit's slot and caller's integer-validation rejects. Closes in `iso-fractional-propagation/` sub-rung.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `parse_iso_duration` defined as inline fn within the Duration install block. ~120 LOC.
- Caller sites: Duration.from(string) and Duration.compare(string) string-arg branches.
- **Exemplar suite**: none (measured via sibling Duration rung deltas).

## Status

IDP-EXT 1 LANDED 2026-05-26. +5 sibling yield (DStat 23→27, DDP 23→24). Duration ISO-string parse functional for all spec-conforming forms except fractional H/M propagation.
