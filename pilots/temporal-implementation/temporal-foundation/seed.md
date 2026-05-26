---
name: temporal-foundation
description: Foundation rung of the Temporal program. Registers the Temporal namespace + Now sub-namespace + stub class identifiers (Instant, PlainDate, etc.) as frozen globals. No operative methods.
type: project
---

# temporal-foundation — Seed

## Nested sub-locale under `pilots/temporal-implementation/`.

Per Rule 23 verification at TN-EXT 0: the 3 Temporal.Now tests are NOT the smallest viable surface — they all require IANA-TZ-string parsing. The actual smallest viable substrate move is `temporal-foundation`: register the Temporal namespace + class scaffolding so subsequent rungs have a place to install methods.

## Telos

After TF-EXT 1:
- `typeof Temporal === "object"` ✓
- `Temporal.Now`, `Temporal.PlainDate`, `Temporal.Instant`, ... exist as objects ✓
- `Object.prototype.toString.call(Temporal.PlainDate) === "[object Temporal.PlainDate]"` ✓
- Calling `Temporal.Now.instant()` (and other Now methods) throws a TypeError with message `Temporal.Now.X not implemented (Tier-L stub)` ✓
- All other Temporal class methods are absent (lookup → undefined; calling → TypeError "not a function") — to be installed by per-class sub-locales.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — new function called from `install_intrinsics` after install_json.
- **Exemplar suite**: none. Foundation has no test surface of its own; it is pure infrastructure validated by downstream rungs.

## Methodology

### TF-EXT 1 — Temporal namespace + class stubs (LANDED)

~50 LOC in intrinsics.rs:
- Allocate `Temporal` object; set `@@toStringTag` to "Temporal".
- Allocate `Temporal.Now` object; register 6 stub methods (plainDateTimeISO, zonedDateTimeISO, instant, plainDateISO, plainTimeISO, timeZoneId) that throw TypeError "not implemented (Tier-L stub)".
- For each of the 8 classes (Instant, PlainDate, PlainTime, PlainDateTime, PlainMonthDay, PlainYearMonth, Duration, ZonedDateTime), allocate a stub object with `@@toStringTag` set to "Temporal.X" and install as `Temporal.X`.
- Insert `Temporal` into globals.

The stub-method-throws-TypeError pattern is the standing handshake between this foundation rung and the per-class rungs: a class-rung overwrites the stub with its real implementation. Downstream-rung test runs reveal which stubs are still active by the TypeError text.

### Probes (Rule 23 verification at landing)

- `typeof Temporal` → `object` ✓
- `typeof Temporal.Now`, `typeof Temporal.PlainDate` → `object` ✓
- `Object.prototype.toString.call(Temporal)` → `"[object Temporal]"` ✓
- `Object.prototype.toString.call(Temporal.PlainDate)` → `"[object Temporal.PlainDate]"` ✓
- `try { Temporal.Now.instant() } catch (e) {}` → TypeError "Temporal.Now.instant not implemented (Tier-L stub)" ✓

## R13 prospective C1-C4

- C1 (sibling): HOLDS — `install_math`, `install_json`, `install_reflect` all use the same namespace registration pattern.
- C2 (shape-compat): HOLDS — additive global install.
- C3 (cost-positive): HOLDS — single function, no recursion.
- C4 (bail-safe): HOLDS — engine works exactly as before for non-Temporal code; Temporal is a leaf namespace.

## Composes-with

- Parent `pilots/temporal-implementation/` — articulates the rung sequence.
- Sibling `pilots/temporal-implementation/temporal-now/` — first per-class rung; will overwrite the Now stub methods.
- `pilots/apparatus/runner-features-skip-deliberate-omissions/` — TF-EXT 1 does NOT remove the Temporal flag from RFSDO (the 3 Now tests still fail at TypeError; per-class rungs flip the SKIP decision).

## Status

TF-EXT 1 LANDED 2026-05-26. Apparatus validated: Temporal exists as a namespace, class stubs in place, downstream rungs can install per-class methods by name without touching the foundation. Next: TN-EXT 1 (temporal-now) — substantial substrate (TZ string parsing), separate session.
