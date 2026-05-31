---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "PIID-EXT 1+2+3: complete the Promise.* iteration-interleave family — Promise.all (PerformPromiseAll §27.2.4.1), Promise.allSettled (§27.2.4.2), Promise.any (§27.2.4.3). Each refactored to a per-element interleaved next-resolve-then loop via a dedicated promise_*_interleave helper that shares promise_iter_get_iterator + promise_reject_with_error from PIID-EXT 0. Closes Finding PIID.1 (sync-throw-vs-capability-rejection at all four Promise.* statics)."
risk_class: substrate
gates_pre:
  test262_full: null
gates_post:
  build: "cargo build --release --bin cruft -p cruftless"
  cargo_test_runtime_lib: "74 / 0 / 1 preserved"
  probe_cells:
    - "Promise.all([resolved x3]) -> values array"
    - "Promise.all with reject -> rejects"
    - "Promise.all(bad iter) -> close + reject TypeError"
    - "Promise.all(42) -> reject TypeError (was: sync TypeError throw)"
    - "Promise.all([]) -> []"
    - "Promise.allSettled mixed -> statuses array"
    - "Promise.allSettled(bad iter) -> close + reject TypeError"
    - "Promise.allSettled([]) -> []"
    - "Promise.any first fulfilled wins"
    - "Promise.any all-reject -> AggregateError (constructor.name 'Object' — pre-existing Finding PIID.2)"
    - "Promise.any(42) -> reject TypeError (was: sync TypeError throw)"
    - "Promise.any(bad iter) -> close + reject TypeError"
---

## Substrate

Three new `promise_*_interleave` helpers (~150 LOC each) in `pilots/rusty-js-runtime/derived/src/interp.rs`:

- `promise_all_interleave` — body shape per §27.2.4.1.2 PerformPromiseAll. Per element: append undefined to values (length += 1), remaining++, create per-index resolve_element via `promise_all_resolve_element_factory`, chain `.then(resolve_element, cap_reject)`. On done, call `promise_all_maybe_complete_via`.
- `promise_all_settled_interleave` — body shape per §27.2.4.2.3 PerformPromiseAllSettled. Per element: append undefined to values, remaining++, create per-index resolve + reject element factories, chain `.then(resolve_element, reject_element)`. On done, `promise_all_maybe_complete_via`.
- `promise_any_interleave` — body shape per §27.2.4.3.2 PerformPromiseAny. Per element: append undefined to errors, remaining++, create per-index reject_element, chain `.then(cap_resolve, reject_element)`. On done, `promise_any_maybe_reject_via` (AggregateError when all rejected).

All three reuse the PIID-EXT 0 shared helpers: `promise_iter_get_iterator` + `promise_reject_with_error`. Per-element abrupt completion routes through `iter_close_rt` best-effort + capability rejection with the original error per §7.4.9 step 4.

Promise.any additionally migrated from synchronous-throw to capability-rejection for the C.resolve resolution path (matching race/all/allSettled), closing Finding PIID.1.

## Findings surfaced

- **PIID.2** (pre-existing, unmasked): Promise.any's all-reject rejection wraps the errors in an object whose `constructor.name` is `Object`, not `AggregateError`. The `promise_any_maybe_reject_via` helper does not construct a proper AggregateError instance. Sibling locale candidate (AggregateError constructor / error-class branding). Pre-existing; my refactor preserved the call.

## Verification

1. `cargo build --release --bin cruft -p cruftless` PASS (~1m 12s).
2. PIID-EXT 1+2+3 12-cell probe: 11/12 PASS (one pre-existing AggregateError shape failure per PIID.2).
3. `cargo test --release -p rusty-js-runtime --lib`: 74 / 0 / 1 preserved.
4. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8, AFID-EXT 1 7/7, PIID-EXT 0 6/6.

## Composes-With

- PIID-EXT 0 decision (substrate prefix: promise_iter_get_iterator + promise_reject_with_error helpers)
- AFID-EXT 0 (iter_close_rt + interleave pattern)
- IPTD-EXT 1 (helper-tier discipline)
- `apparatus/docs/predictive-ruleset.md` Rule 24 (Pin-Art emit-site coherence: per-method body now structurally identical across 4 Promise.* siblings + 1 Array.from + Set/WeakSet ctor; emit_frame_unwind-style generic helper deferred but threshold met).

## Authorization

Per keeper Telegram 10696 ("Push all and continue") authorizing landing of the full Promise.* family in one rung.
