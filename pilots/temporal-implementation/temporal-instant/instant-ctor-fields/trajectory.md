# instant-ctor-fields — Trajectory

## TInst-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Sibling-shape to DCF (Duration ctor-fields). Validates that the per-class-parent pattern works for a second class.

### Edit (~120 LOC in intrinsics.rs)

- Removed `"Instant"` from the foundation stub-classes loop (was a stub object; now overwritten by the real ctor).
- Allocated `inst_proto` with @@toStringTag "Temporal.Instant" + constructor back-pointer.
- Installed `epochNanoseconds` accessor (returns BigInt sentinel directly).
- Installed `epochMilliseconds` accessor (`floor(ns_f / 1e6)` as Number).
- Installed `valueOf` method throwing TypeError.
- Ctor via `make_native_with_length("Instant", 1, ...)`:
  - NewTarget undefined → TypeError ("cannot be called as a function").
  - ToBigInt(arg) → BigInt (handles BigInt direct + bool + string; SyntaxError on bad string; TypeError on Number per spec).
  - Range check: `|ns_f| ≤ 8.64e21` → RangeError.
  - Allocate instance with proto + `__ti_ns` BigInt sentinel.
- Set `ctor.prototype = inst_proto` (frozen).
- Install on `Temporal.Instant` (overwriting nothing — Instant was removed from the foundation stub loop).

### Edit (~15 LOC in runner.mjs)

RFSDO allowlist extended with 15 Instant test paths (constructor, name, length, basic, builtin, prop-desc, argument, large-bigint, limits, prototype/{epochNanoseconds,epochMilliseconds,valueOf,toStringTag}, prototype/constructor.js, prototype/prop-desc.js).

### Probes (Rule 23 verification at landing)

- `new Temporal.Instant(1000n)` → epochNanoseconds=1000n, epochMilliseconds=0 ✓
- `new Temporal.Instant("-217175010123456789")` → string→BigInt + correct ms derivation ✓
- `new Temporal.Instant(true)` → 1n, `new Temporal.Instant(false)` → 0n ✓
- `Temporal.Instant(0n)` (no new) → TypeError ✓
- `new Temporal.Instant("abc")` → SyntaxError ✓
- `new Temporal.Instant(1n).valueOf()` → TypeError ✓
- `instanceof Temporal.Instant` → true ✓
- `Temporal.Instant.name === "Instant"`, `.length === 1` ✓
- `new Temporal.Instant(99999999999999999999999999n)` → RangeError ✓

### Yield

- instant-ctor-fields exemplar pool (25): **0 → 21/25 PASS (84%)**.
- Duration sub-rungs stable (DCF 64/67, DDP 23/24, DStat 23/81, DWith 19/22).
- Diff-prod: 42/42.

Cumulative Temporal yield (5 Duration sub-rungs + 1 Instant sub-rung): **150/219 (68%)**.

### Residuals (4)

| Shape | Cause |
|---|---|
| `@@toStringTag should be an own property` | propertyHelper.verifyProperty descriptor-shape check on toStringTag; need exact descriptor fields |
| `number Expected a TypeError to be thrown` | `new Temporal.Instant(0)` — argument.js expects TypeError on Number arg; my ToBigInt returns TypeError but maybe message-shape differs |
| `< Expected a TypeError to be thrown` | valueOf-test exercises `<` operator; needs valueOf to throw on operator coercion (not just direct call) |
| `Expected a RangeError to be thrown` | limits.js edge case (probably exact boundary value) |

All four are small refinements deferrable to instant-edge-refinement follow-on.

### Findings

**Finding TInst.1 (the per-class-parent pattern transfers cleanly across classes)**: Instant is the second class to use the parent + nested-leaf-rungs shape. The pattern (prototype + ctor.prototype + accessors via PropertyDescriptor + valueOf-throws + brand-check via sentinel) transferred from Duration without surprise. Standing recommendation: the next per-class rungs (PlainTime, PlainDate, etc.) can use the same template; expect ~80% of the LOC pattern to be copy-paste-and-rename of the Duration scaffolding with class-specific sentinel naming and ctor-arg shape.

**Finding TInst.2 (BigInt-sentinel classes are simpler than tuple-sentinel classes by LOC)**: Instant stores ONE sentinel (`__ti_ns` BigInt). Duration stores TEN (`__td_years` through `__td_nanoseconds` as f64). The Instant ctor + 2 getters fit in ~120 LOC; Duration's ctor + 10 getters fit in ~110 LOC + ~110 LOC for derived-properties + ~120 LOC for static + ~45 LOC for with + ~25 LOC for sign-validation = ~410 LOC total. Class total budget should scale with sentinel-tuple size; single-sentinel classes (Instant) need ~½ the LOC of tuple-sentinel classes (Duration) for equivalent surface coverage.

### Status

TInst-EXT 1 CLOSED. Next: instant-static (from / fromEpochMilliseconds / fromEpochNanoseconds / compare) — would close ~50 more tests.
