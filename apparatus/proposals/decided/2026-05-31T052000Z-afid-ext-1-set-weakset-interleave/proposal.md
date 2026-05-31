---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "AFID-EXT 1: extend interleave + IteratorClose discipline to Set + WeakSet ctors per ECMA-262 §24.2.1.2 / §24.4.1.2 AddEntriesFromIterable. Closes OOM regressions surfaced by AFID.2 audit; also removes the silent error-swallow that hid spec-mandated TypeError throws (ToObject failures, iterator-not-callable, next() non-Object, WeakSet primitive-value rejection). Single substrate site (the shared Set/WeakSet ctor closure at intrinsics.rs:18159). Reuses iter_close_rt from AFID-EXT 0."
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: 88.7%
gates_post:
  build: "cargo build --release --bin cruft -p cruftless"
  cargo_test_runtime_lib: "74 / 0 / 1 preserved"
  probe_cells:
    - "new WeakSet(infinite-iter yielding primitives) -> close + TypeError (was OOM)"
    - "new Set with next() returning non-Object -> close + TypeError (was silent empty Set)"
    - "new Set(undefined) -> empty Set (regression)"
    - "new Set([1,2,2,3]) dedup (regression)"
    - "new Set(42) -> TypeError (was silent empty Set)"
    - "new Set([1,2,3]) regression"
---

## Substrate

~75 LOC replacing the prior 12-LOC `collect_iterable` block at the shared Set + WeakSet ctor closure (intrinsics.rs:18159, inside the `for collection in &["Set", "WeakSet"]` loop). Interleave loop per ECMA-262 §24.2.1.2 / §24.4.1.2 + AddEntriesFromIterable:

1. Branch on `arg ∈ {undefined, null}`: skip iteration per spec.
2. Else ToObject the arg (or take the Object directly).
3. GetMethod(@@iterator) → call → check Object → store iter_id.
4. GetMethod(next) on iter (cached in next_fn).
5. Loop:
   - call next; check result Object (else close + TypeError); check .done; read .value.
   - WeakSet: check value is Object/Symbol per §24.4.3.1 step 4 (CanBeHeldWeakly); if not, close + TypeError.
   - Insert with abstract_ops::to_string key + dedup; size += 1 if newly inserted.
   - On any abrupt completion in per-element processing, `iter_close_rt(rt, iter_id)` best-effort then return Err(original) (§7.4.9 step 4).

Removes the prior `if let Ok(values) = collect_iterable(...)` silent-swallow, which had hidden spec-mandated TypeErrors (ToObject failures, iterator-not-callable, etc.).

## Surfaced gap (out of scope; documented)

`probe-afid-1.js` cell 6 (`new WeakSet([o1, o2]).has(o1) && .has(o2)`) FAILs at the rewrite AND would have FAILed at baseline (baseline OOMs before reaching cell 6). This is a pre-existing WeakSet storage bug: the impl stores by `abstract_ops::to_string(&v)` key, which collapses all distinct objects to "[object Object]". Sibling issue, separable from AFID-EXT 1's interleave/close scope — surfaces a future locale `weakset-object-identity-storage-discipline/` (or similar) for the storage-key fix.

## Verification

1. `cargo build --release --bin cruft -p cruftless` PASS (~1m 08s).
2. AFID-EXT 1 7-cell probe: 6/7 PASS (cell 6 pre-existing storage bug, see above).
3. `cargo test --release -p rusty-js-runtime --lib`: 74 / 0 / 1 preserved.
4. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8.
5. All probes run under `ulimit -v 2 GiB`. Pi survived.

## Carry-forward (per AFID.2 audit)

Remaining OOM-vulnerable surfaces per the cross-intrinsic survey:
- Promise.all / race / any / allSettled (4 intrinsics; ~200+ LOC across `promise_collect_iterable` + per-method closure tracking). Spawn `promise-iteration-interleave-discipline/` locale.

## Composes-With

- `pilots/array-from-interleave-discipline/` AFID-EXT 0 (substrate prefix: `iter_close_rt` helper + interleave pattern)
- `pilots/iterator-protocol-throw-discipline/` IPTD-EXT 1 (helper-tier discipline)
- `apparatus/docs/predictive-ruleset.md` Rule 17 (segmentation: Set+WeakSet ctors only; Promise.* deferred), Rule 24 (Pin-Art pattern recurrence: interleave + iter_close_rt across runtime intrinsics — second confirmed instance after AFID-EXT 0; Promise.* would be the third)

## Authorization

Per keeper Telegram 10690 ("3"), selecting option (3) from the AFID.2 triage — land Set + WeakSet ctors cheap, defer Promise.* family.
