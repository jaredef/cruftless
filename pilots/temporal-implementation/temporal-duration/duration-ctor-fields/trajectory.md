# duration-ctor-fields — Trajectory

## DCF-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

### Trigger

Keeper directive (Telegram 9881) to continue the Temporal program after the TI-EXT 1 restructure. First per-class substrate rung — validates the per-class-parent / leaf-rung nesting pattern.

### Edit (~110 LOC in intrinsics.rs::install_temporal)

1. **Prototype** allocation + @@toStringTag = "Temporal.Duration".
2. **10 accessor-property getters** for `years` / `months` / `weeks` / `days` / `hours` / `minutes` / `seconds` / `milliseconds` / `microseconds` / `nanoseconds`. Each getter:
   - Allocates a non-ctor native named `get {unit}`.
   - Brand-checks via `__td_{unit}` sentinel presence on the receiver; throws TypeError if absent.
   - Returns the sentinel value.
   - Installed via `PropertyDescriptor { getter: Some(...), writable: false, enumerable: false, configurable: true }`.
3. **valueOf** method that throws TypeError "Temporal.Duration valueOf cannot be used; use compare()".
4. **Constructor** via `make_native_with_length("Duration", 0, ...)`:
   - If `rt.current_new_target.is_none()`, throw TypeError ("cannot be called as a function").
   - For each of 10 args: ToNumber; reject non-finite + non-integer; normalize -0 → 0.
   - Allocate instance with `proto = Some(dur_proto)` and 10 sentinels.
5. **ctor.prototype = dur_proto** (frozen). Required for `instanceof` checks and `Temporal.Duration.prototype` accesses.
6. **dur_proto.constructor = ctor** (internal).
7. **Overwrite** `Temporal.Duration` (foundation stub) with the real ctor.

### Edit (~50 LOC in runner.mjs)

RFSDO-EXT 3 (TI.4 protocol): added `PARTIALLY_IMPLEMENTED` map of feature → path-substring allowlist. A test whose path contains any allowlisted substring for a deny-listed feature opts OUT of the SKIP. Initial Temporal allowlist: 30+ Duration ctor/getter/valueOf paths + 4 foundation intro paths.

### Probes (Rule 23 verification at landing)

- `Temporal.Duration()` (no new) → TypeError ✓
- `new Temporal.Duration(1,2,3,4,5,6,7,8,9,0)` → reads back `1 2 3 4 5 6 7 8 9 0` ✓
- `new Temporal.Duration()` → all 10 fields = 0 ✓
- `new Temporal.Duration(1).valueOf()` → TypeError ✓
- `Temporal.Duration.name === "Duration"`, `.length === 0` ✓
- `new Temporal.Duration(1) instanceof Temporal.Duration` → true ✓
- `Temporal.Duration.prototype` → object with toStringTag "Temporal.Duration" ✓

### Yield

- **duration-ctor-fields exemplar pool (67)**: 0 → **64 PASS** (95.5%) (was 0 SKIP via Temporal flag → now opted into testing via RFSDO-EXT 3).
- Diff-prod: 42/42 maintained.
- Cross-locale regression sweep (7 locales): all unchanged.

### Two-stage discovery (Rule 23 at landing)

First cut after building yielded 25/67. Verification-probe surfaced the `prototype` property gap — the ctor.prototype property wasn't being set, so `Temporal.Duration.prototype === undefined`. Tests calling `Object.getOwnPropertyDescriptor(Temporal.Duration.prototype, "years")` and `instanceof Temporal.Duration` both failed on this. Added `self.obj_mut(dur_ctor).set_own_frozen("prototype".into(), Value::Object(dur_proto))`. Yield rose 25 → 64 (+39 from one missing property).

### Residuals (3)

- 2 getter-trace tests in `built-ins/Temporal/Duration/prototype/{years,...}/branding.js` style — they probe a specific observer-pattern trace `[get years.valueOf, call years.valueOf]` indicating the test expects the getter to be invoked AND its valueOf to be called. cruft's getter invocation doesn't emit such a trace. Belongs to a brand-check-observer rung; not DCF.
- 1 `Temporal.Duration.from` not callable — static method, belongs to `duration-static/` rung.

### Findings

**Finding DCF.1 (ctor.prototype is the bridge between accessor-getter installs and `instanceof`)**: First cut installed dur_proto with all 10 getters and the ctor with the closure-captured proto_for_ctor reference. But cruft's `instanceof` (and `Object.getOwnPropertyDescriptor` on the prototype as accessed via `Temporal.Duration.prototype`) reads `ctor.prototype` directly. Without setting it, both fail despite the accessor-getter installation being correct. Standing recommendation: class-ctor installs must set ctor.prototype as a frozen own-property pointing to the prototype object — this is the externally-observable bridge. The internal proto_for_ctor closure capture only handles instance creation, not external prototype access.

**Finding DCF.2 (RFSDO PARTIALLY_IMPLEMENTED is the apparatus piece that makes progressive Temporal landings visible)**: Without the carve-out, every Duration test would still SKIP via the Temporal flag, and the 64-test yield would be invisible to the matrix. RFSDO-EXT 3 surfaces the yield WITHOUT removing the broader Temporal flag (which still correctly SKIPs the other 8 classes). Standing recommendation: as each per-class ctor rung lands, extend the PARTIALLY_IMPLEMENTED allowlist for that class's covered paths. Future categorizer runs will show progressive Temporal yield as the program advances.

### Status

DCF-EXT 1 CLOSED. 64/67 yield. Next sub-rungs in the temporal-duration topology can land independently against the now-real Duration class. Recommended next: `duration-derived-properties` (sign / blank / abs / negated) — small surface, sibling-shape to ctor-fields.
