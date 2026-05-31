# array-from-interleave-discipline — Trajectory

## AFID-EXT 0 — LANDED (2026-05-31) — interleave iteration + IteratorClose on mapfn-throw + iter_close_rt helper

**Trigger**: Direct chapter-close-inspect carry-forward from ICES residual probe (post-ICES-EXT 3). Probing Array.from with a mapfn that throws on the first element of an infinite iter OOMed at 889 MB under `ulimit -v 1 GiB`. Diagnosis: `array_from_via` eagerly collected the iterable before applying mapfn, so mapfn-throws were unreachable on infinite iters. Keeper APPROVED via Telegram 10684 ("Continue").

**Substrate** (~80 LOC across two files in `pilots/rusty-js-runtime/derived/src/`):

1. `intrinsics.rs`: new `iter_close_rt(rt, iter_id) -> Result<(), RuntimeError>` (~25 LOC). Mirrors the JS-callable `__destr_iter_close` helper at the Rust-caller surface. Returns silently for null/undefined .return; throws TypeError for non-callable non-null/undefined; calls .return() and checks Object result otherwise.
2. `interp.rs:6589` `array_from_via` (~55 LOC delta): branch on iterable-vs-array-like upfront. Iterable branch rewritten as a per-element loop:
   - Call iter.next(); check result is Object (else TypeError); read .done; if done, break.
   - Read .value; if mapping, call mapfn(value, k) — on Err, call `iter_close_rt(self, iter_id)` (best-effort; close-thrown errors do NOT shadow the original per §7.4.9 step 4) and propagate the original error.
   - Write the mapped (or raw) value into the out array at index k; increment k.
3. Array-like / string branches preserved unchanged at the second half of `array_from_via`.

**Phase 2 (Baseline-inspect)** per Rule 23: confirmed `collect_iterable` (intrinsics.rs:21669) was the eager-collection site. Verified collect_iterable is still used elsewhere (other spread paths); the AFID rung does not remove or modify collect_iterable, only routes Array.from off it.

**Phase 3 (Pin-Art if duplicated)** per Rule 24: not invoked at this rung. The interleave pattern recurs at other intrinsic surfaces (Array spread, Map/Set constructors, Promise.all/allSettled iteration), but they consume different surrounding logic (Map ctor pairs key/value, Promise.all queues async results). Centralizing across them would require a generic `interleave_iter<F>` helper; deferred until 2+ more sites materialize.

**Phase 4 (Revert-then-deeper-layer if negative)** per Rule 13: not invoked — closure positive on first probe.

**Phase 5 (Chapter-close-inspect)** per Rule 15:

**Finding AFID.1** — ICES-EXT 3 close-throw spec divergence surfaced (cross-locale): the ICES-EXT 3 synthetic catch stub for for-of body throw is:

```
catch_pos:
  StoreLocal <thrown>
  emit_iter_close_call iter_slot   ← if this throws, propagates immediately
  LoadLocal <thrown>
  Throw                            ← never reached if close threw
```

This makes close-thrown errors REPLACE the body-thrown error. Per ECMA-262 §7.4.9 step 4, when `completion.[[Type]] is throw`, IteratorClose returns the ORIGINAL completion even if GetMethod/Call of `iter.return` themselves throw. The for-of body-throw + non-callable iter.return case should propagate the body-thrown error, not the close TypeError. The probe at /tmp/probe-ices-ext-3.js cell 4 asserted the WRONG behavior and was confirmed PASS by the wrong-shape substrate; both probe and substrate need correction.

AFID-EXT 0 implements the spec-correct semantics (`let _ = iter_close_rt(...)` discards the close error) at the Array.from surface. The ICES-EXT 3 fix is the dual: wrap the bytecode-tier close call in a nested synthetic try-catch that swallows close-thrown values before re-throwing the spilled original. Surfaces as a separate rung in the ICES locale (candidate ICES-EXT 3.1 or EXT 4 per keeper directive); not bundled into AFID per Rule 17 (locale segmentation).

**Finding AFID.2** — eager-collect-then-process is a recurring anti-pattern at runtime-tier intrinsics that consume iterables. Confirmed instances: `array_from_via` (pre-rung; rewritten by AFID-EXT 0), `array_proto_concat_via` (probe-pending), `array_proto_flat_via` (probe-pending). Candidate for cross-intrinsic audit at the findings-disposition cycle.

**Yield**:

```text
AFID-EXT 0 probe (/tmp/probe-afid-0.js): 8/8 PASS
  Array.from mapFn throw closes iter (length 1, threw=true)        ✓
  Interleaved next/mapFn (order n0,m0,n1,m1,n2,m2)                 ✓
  Array.from([1,2,3]) -> [1,2,3] (regression)                      ✓
  Array.from(Set) (regression)                                     ✓
  Array.from("abc") -> ["a","b","c"] (regression)                  ✓
  mapFn args (value, index) correct                                ✓
  mapFn throw + non-callable iter.return: original Error wins      ✓
  next() non-Object -> TypeError (no OOM)                          ✓

Regression sweep (3 prior probes, all under ulimit -v 2 GiB):
  Original 7-cell IPTD probe:           7/7 preserved
  Cross-consumer 7-cell probe:          7/7 preserved
  ICES-EXT 2 6-cell probe:              6/6 preserved
  ICES-EXT 3 9-cell probe:              9/9 preserved (cell 4 still asserts WRONG-shape; Finding AFID.1)
```

**Status**: AFID-EXT 0 LANDED. Array.from no longer OOMs on infinite iters when mapfn throws. Interleaved iteration spec-correct. Close-on-mapfn-throw spec-correct (original throw preserved per §7.4.9 step 4). Finding AFID.1 (ICES-EXT 3 spec divergence on close-throw shadowing) surfaced for follow-up.
