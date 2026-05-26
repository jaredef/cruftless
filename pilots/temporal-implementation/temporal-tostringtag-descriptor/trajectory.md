# temporal-tostringtag-descriptor — Trajectory

## TTSTD-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Spawned per Finding PTCF.2 standing-rec. Cross-cutting rung — first Tier-L rung where the substrate is correct but yield is blocked by an unrelated cruft bug.

### Edit (~40 LOC in intrinsics.rs)

Replaced 6 `self.obj_mut(X).set_own_frozen("@@toStringTag", ...)` calls with `self.obj_mut(X).dict_mut().insert("@@toStringTag", PropertyDescriptor { ..., configurable: true })`:
- Temporal namespace
- Temporal.Now sub-namespace
- 5 class stubs (PlainDate, PlainDateTime, PlainMonthDay, PlainYearMonth, ZonedDateTime) — looped
- Temporal.Duration.prototype
- Temporal.Instant.prototype
- Temporal.PlainTime.prototype

Plus 1 LOC in runner.mjs adding `/Temporal/Duration/prototype/toStringTag/` to the RFSDO allowlist (Instant + PlainTime were already in).

### Probes (Rule 23 verification at landing)

- `Object.getOwnPropertyDescriptor(Temporal.Duration.prototype, Symbol.toStringTag)` → `{value: "Temporal.Duration", writable: false, enumerable: false, configurable: true}` ✓ (was c:f, now c:t)
- Same for Instant.prototype and PlainTime.prototype ✓
- `Temporal.Duration.prototype[Symbol.toStringTag]` → `"Temporal.Duration"` ✓

### Yield + the blocker

Net yield: **0 tests**. The prop-desc tests still FAIL despite the descriptor being spec-correct.

Investigation:
- `Object.prototype.hasOwnProperty.call(proto, "@@toStringTag")` → `true` ✓
- `Object.prototype.hasOwnProperty.call(proto, Symbol.toStringTag)` → `false` ✗

cruft stores `@@toStringTag` as a literal string key. `Object.getOwnPropertyDescriptor` bridges `Symbol.toStringTag` → `"@@toStringTag"` correctly (probes confirm). But `Object.prototype.hasOwnProperty` does NOT bridge. propertyHelper.verifyProperty internally calls hasOwnProperty as its first check — so the test fails before reaching the descriptor comparison.

### Findings

**Finding TTSTD.1 (spec-correct substrate moves can have zero yield when blocked by a deeper bug)**: TTSTD-EXT 1 is a real spec-correctness fix — the descriptor went from {w:f, e:f, c:f} to {w:f, e:f, c:t} per §11.x.5. But the test it was meant to close still fails because a downstream bug (Object.prototype.hasOwnProperty's symbol-key bridge) gates the check. Standing recommendation: when a cross-cutting fix produces zero immediate yield, audit the test's call chain (verifyProperty → hasOwnProperty → ...) — the failing predicate may be elsewhere in the chain than where the substrate targets. Don't revert correct fixes just because yield is zero; document the blocker and commit.

**Finding TTSTD.2 (cruft's `@@`-string convention vs Symbol-key API contract is inconsistent)**: cruft's pattern of storing well-known symbols under `@@name` string keys works for some APIs (Object.getOwnPropertyDescriptor's symbol-key path bridges) but not others (Object.prototype.hasOwnProperty's symbol-key path doesn't). This is wider than Temporal — Math.@@toStringTag, JSON.@@toStringTag, etc. all share the issue. Standing recommendation: spawn `cruft-symbol-key-hasown-bridge` follow-on locale (top-level under pilots/, not nested in Temporal) to make hasOwnProperty bridge symbol → @@-string consistently with getOwnPropertyDescriptor.

### Status

TTSTD-EXT 1 CLOSED. Substrate fix in place; descriptor spec-correct. Yield blocked by cruft-symbol-key-hasown-bridge (follow-on candidate, top-level locale). Expected yield once unblocked: 3+ Temporal prop-desc tests + every other namespace's similar test (Math, JSON, etc.) that happens to be exercised.
