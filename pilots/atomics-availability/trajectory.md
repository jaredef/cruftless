# atomics-availability — Trajectory

## AT-EXT 0+1 — founding + namespace install (2026-05-27)

Per keeper directive Telegram 10091 ("Shift to another heavy coordinates"). Matrix coordinate rank 30 (`runtime/agent-memory :: missing-global-or-binding`, 171 fails) — Atomics namespace entirely absent pre-rung. Non-overlapping with parallel agent's Temporal grammar surface.

**Founding apparatus**:
- `exemplars/exemplars.txt` — 100 paths across `Atomics/{top-level + 14 method dirs}`.
- `exemplars/run-exemplars.sh` — runner with per-method breakdown.

**Baseline** (pre-allowlist, post-namespace install): PASS=5/100 — the namespace install alone didn't help because runner SKIPs Atomics-feature tests under DELIBERATELY_OMITTED. SKIP counts as FAIL.

**EXT 1 substrate** (~135 LOC in intrinsics.rs + 11 lines in test262 runner.mjs):
- `install_atomics_globals` builds the Atomics object with `@@toStringTag = "Atomics"`, allocates it, registers 14 method intrinsics, defines on globalThis.
- Methods: `load(ta, i)` returns ta[i]; `store(ta, i, v)` writes and returns v; `add/sub/and/or/xor(ta, i, v)` read-modify-write returning previous value; `exchange/compareExchange` per spec; `isLockFree(n)` returns `n ∈ {1,2,4,8}`; `wait` returns `"not-equal"`; `waitAsync` returns `{async:false, value:"not-equal"}` (degenerate non-shared path); `notify` returns 0; `pause()` returns undefined.
- Runner `PARTIALLY_IMPLEMENTED.Atomics` allowlist opts in structure-availability paths: `prop-desc`, `proto`, `Symbol.toStringTag`, per-method `length`/`name`/`descriptor`/`prop-desc`/`not-a-constructor`.

**Yield**:
```text
Atomics cluster PRE-rung:  PASS=0/100 (all SKIP)
Atomics cluster POST-EXT 1: PASS=31/100 (31.0%)
```
**+31 PASS** this rung. Remaining 69 fails are concentrated in semantic-heavy methods (waitAsync 7, wait 7, notify 6, arithmetic ops 5×6=30, store 5, isLockFree 6) which need real SharedArrayBuffer + IntegerIndexed value-type semantics per the carve-outs.

**Gates**: build clean; diff-prod **60/52** (up from 59/53 — `global-constructors` fixture flipped to PASS because cruft now reports `typeof Atomics === "object"`).

**Finding AT.1 (allowlist as substrate enabler)**: a substrate install whose tests live under `DELIBERATELY_OMITTED` produces zero observable yield until the allowlist entry lands. The allowlist commit and the substrate commit MUST be paired — landing one without the other either over-reports (allowlist alone, with semantic FAILs surfacing as regressions) or under-reports (substrate alone, with PASSes still SKIPped). Recurrence of Finding PDTS.5 in a different feature surface.

**Finding AT.2 (collateral diff-prod gain)**: the `global-constructors` diff-prod fixture probes typeof of every standard global. Adding Atomics flipped one previously-mismatching cell; the +1 on diff-prod is genuine (not a flake). Future namespace-install rungs should expect a small but real diff-prod contribution if `global-constructors` is in the fixture set.

**Status**: AT-EXT 1 CLOSED locally. Further closure requires real shared-memory substrate.
