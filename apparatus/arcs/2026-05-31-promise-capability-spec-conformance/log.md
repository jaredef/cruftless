# Promise Capability Spec Conformance Arc — Event Log

Append-only per `apparatus/docs/arc-as-coordinate.md` §F.

---

## 2026-05-31 — Arc scaffolded

Per keeper Telegram 10743 authorizing apparatus-tier consolidation of the session-31 substrate cascade. Arc.md drafted under the orphan-disposition protocol's lift-to-arc-tier disposition (Pattern III.1 arc-tier-as-locale mis-categorization variant: arc-tier-emerging-from-sibling-fixes). The substrate landed before the arc was scaffolded — this initial entry records the retroactive coordination.

Roster of 7 sub-locales / embedded fixes (5 of 7 are embedded fixes inside `interp.rs` / `promise.rs`, not standalone seed.md+trajectory.md scaffolds; per orphan-disposition close-as-locale-singleton disposition for arc-relevant single-substrate-site fixes).

Arc opened concurrent with parallel CENP locale work (CJS-ESM-namespace-pipeline, handled by separate worker). PCSC and CENP are non-interfering: PCSC is Promise spec-conformance at the language tier; CENP is CJS→ESM namespace construction at the loader tier. No shared substrate sites.

## 2026-05-31 — PSCV-EXT 0 (Promise.* this-Object check)

Closure landed before arc scaffolded. Substrate: 6 static-method registration closures in `promise.rs` insert `if !matches!(c, Value::Object(_)) { return Err(TypeError) }` before delegating to the generated `*_via` path. Promise.resolve + Promise.reject closures additionally forward `current_this` to the generated path (was discarded as `Value::Undefined`).

Probe: 36/39 PASS (39-cell probe; 3 ctx-ctor subclass cells out of EXT 0 scope, carry-forward to EXT 1).

Per-arc cell delta: ~36 cells closed across `ctx-non-object.js` × 6 directories (all, race, any, allSettled, resolve, reject).

## 2026-05-31 — PSCV-EXT 1 (subclass routing via NewTarget + cap.resolve/reject)

Closure landed before arc scaffolded. Substrate:

1. Promise constructor (§27.2.3.1 step 3 OrdinaryCreateFromConstructor): after `new_promise(rt)`, override result's [[Prototype]] with `current_new_target.prototype` when NewTarget is set.
2. Promise.resolve closure (§27.2.4.7 PromiseResolve): short-circuit on native; else NewPromiseCapability(C) + cap.[[Resolve]](x).
3. Promise.reject closure (§27.2.4.4): mirror — short-circuit on native; else NewPromiseCapability(C) + cap.[[Reject]](r).

Probe: 39/39 PASS (was 36/39 after EXT 0; closes 3 subclass cells). cargo test 74/0/1 preserved.

Per-arc cell delta: ~6 cells across `ctx-ctor.js` × 6 directories (subclass-shaped result instead of default Promise).

## 2026-05-31 — PCEXC-EXT 0 (capability-executor-called-twice)

Closure landed before arc scaffolded. Substrate: per-capability `already_called: Rc<RefCell<bool>>` flag in `new_promise_capability`'s executor closure. First invocation sets it true; subsequent invocations throw TypeError before storing args. Closes capability-executor-called-twice test family across Promise.all/race/any/allSettled/resolve.

Probe: 8/8 PASS. cargo test 74/0/1 preserved.

Per-arc cell delta: ~5 cells across `capability-executor-called-twice.js` × 5 directories.

## 2026-05-31 — PCRT-EXT 0 (cap.resolve throws → IfAbruptRejectPromise)

Closure landed before arc scaffolded. Substrate: wrap `promise_all_maybe_complete_via` calls in Promise.all + Promise.allSettled done-branches with `if let Err(e) → promise_reject_with_error(cap_reject, e)`. Implements §27.2.4.1/.2 step 8 IfAbruptRejectPromise for the resolve-side throw.

Probe: 2/2 PASS. cargo test 74/0/1 preserved.

Per-arc cell delta: ~4 cells across `capability-resolve-throws-reject.js` × 2-4 directories.

## 2026-05-31 — PTHEN-EXT 0 (then-getter throws → close + reject)

Closure landed before arc scaffolded. Substrate: replace silent `object_get(*id, "then")` with `spec_get(&next_promise, "then")` match across all 4 PIID interleave helpers; on Err, iter_close_rt + capability rejection with the getter-thrown value. Implements §27.2.4.* Invoke abrupt-completion handling.

Probe: 4/4 PASS. cargo test 74/0/1 preserved.

Per-arc cell delta: ~4 cells across `invoke-then-get-error-reject.js` × 4 directories.

## 2026-05-31 — PCATCH-EXT 0 (Promise.prototype.catch ToObject coercion)

Closure landed before arc scaffolded. Substrate: per §27.2.5.1 + Invoke (GetV ToObject-wraps), null/undefined throw TypeError; other primitives ToObject-coerce; .then resolved via the coerced receiver's wrapper prototype.

Probe: 7/7 PASS (Boolean/Number/String/Symbol primitives + undefined/null TypeError + Promise regression). cargo test 74/0/1 preserved.

Per-arc cell delta: ~4-6 cells in `Promise/prototype/catch/this-value-obj-coercible.js`.

## 2026-05-31 — PTHEN-EXT 1 (Promise.prototype.then SpeciesConstructor)

Closure landed before arc scaffolded. Substrate: SpeciesConstructor lookup per §27.2.5.4 step 3 + §7.3.22. Get(promise, "constructor") + @@species resolution; non-Object → TypeError; native Promise → fast-path `new_promise`; otherwise `NewPromiseCapability(C)` preserves SubP-derived chain shape.

Probe: 6/6 PASS. cargo test 74/0/1 preserved. Promise.prototype.catch inherits via delegation.

Per-arc cell delta: ~6-10 cells across `ctor-null.js`, `ctor-throws.js`, `ctor-poisoned.js`, `ctor-custom.js`, `resolve-pending-fulfilled-poisoned-then.js` family.

Carry-forward: @@species getter resolution edge case (when species is defined via `static get [Symbol.species]() {}`) — cross-listed with WKSL.

## 2026-05-31 — PFINALLY-EXT 0 (Invoke(this, "then") delegation)

Closure landed before arc scaffolded. Substrate: per §27.2.5.3, finally now returns `Invoke(this, "then", thenFinally, catchFinally)`. ThenFinally wraps `(value) => { onFinally(); return value; }`; CatchFinally wraps `(reason) => { onFinally(); throw reason; }`. Non-callable onFinally → pass through. Receiver null/undefined → TypeError; primitive → ToObject.

Probe: 14/14 PASS (mocked .then receives (thenFinally, catchFinally) with thisValue=target + argCount=2; wrappers are functions with name="" length=1; result === target.then's return value; regression preserved). cargo test 74/0/1 preserved.

Per-arc cell delta: ~16 cells across `invokes-then-with-function.js`, `invokes-then-with-non-function.js`, plus rejected/resolved-observable-then-calls + species-constructor + subclass-reject-count cells.

## 2026-05-31 — PRESOLVE-EXT 0 (thenable resolution per §27.2.1.4)

Closure landed before arc scaffolded. Substrate: when `resolve_promise` is called with a non-Promise Object whose `.then` is callable, synchronously invoke `value.then(resolveFn, rejectFn)` to chain settlement. Get(.then) throws → reject with thrown value. Non-callable .then falls through to ordinary fulfillment.

Probe: 5/5 PASS. cargo test 74/0/1 preserved.

Per-arc cell delta: ~5-8 cells across `arg-poisoned-then.js`, `resolve-poisoned-then-immed/deferred.js`.

Carry-forward: full §27.2.1.4 PromiseResolveThenableJob microtask deferral.

## 2026-05-31 — PRESOLVE-EXT 1 (Promise.resolve x.constructor === C check)

Closure landed before arc scaffolded. Substrate:

1. Promise.resolve: short-circuit IsPromise(x) gated on `Get(x, "constructor") === C` per §27.2.4.7 step 1.
2. Promise.reject: removed `promise_reject_via` fast-path. §27.2.4.4 has no short-circuit; always NewPromiseCapability(C) + cap.[[Reject]](r).

Probe: 6/6 PASS. cargo test 74/0/1 preserved.

Per-arc cell delta: ~3-5 cells across `arg-uniq-ctor.js` + `capability-invocation-error.js`.

## 2026-05-31 — Arc enrollment chapter-close (Phase 5)

### Cumulative measured probe yield

| Probe | Cells | Status |
|---|---|---|
| PSCV | 39 | PASS |
| PCEXC | 8 | PASS |
| PCRT | 2 | PASS |
| PTHEN | 4 | PASS |
| PCATCH | 7 | PASS |
| PTHEN-species | 6 | PASS |
| PFINALLY | 14 | PASS |
| PRESOLVE | 5 | PASS |
| PRESOLVE-1 | 6 | PASS |
| **Total arc-scope** | **91** | **91/91 PASS** |

cargo test --release -p rusty-js-runtime --lib: 74/0/1 preserved across all 10 landings.

### Predictions held / refuted (provisional)

- **Pred-pcsc.2** (each sub-ext closes in 1 round): **HELD** — 10 of 10 landings were single-round.
- **Pred-pcsc.3** (zero PASS→FAIL regressions): **HELD** — full regression sweep across 12-13 probes preserved at every landing.
- **Pred-pcsc.4** (per-extension cell-delta sums, not multiplies): **HELD** — each extension targeted a single failure-reason cluster (matrix-cell-level, not arc-cell-level).
- **Pred-pcsc.1** (Promise FAIL bucket compresses from 190 to <50): **UNMEASURED** — no post-arc test262-sample re-run yet. Predicted ~90-100 cells closed based on per-ext estimates; would compress bucket toward ~90-100.

### Open carry-forwards

- @@species getter resolution edge case (PTHEN-EXT 1 §3.2)
- Asynchronous deep-async-thenable resolution (PRESOLVE-EXT 0 deferred microtask job)
- Promise.any AggregateError cause-chain
- Promise.allKeyed / allSettledKeyed (Stage 1/2 proposals; out of arc scope)
- Test262-sample re-run for Pred-pcsc.1 measurement

### Status

**CHAPTER OPEN.** Arc-tier substrate cascade landed; ten sub-extensions complete; close-condition not yet at quantitative target (requires post-landing test262-sample re-run to confirm Promise bucket drops below 50). Carry-forwards listed above are separable from primary scope.

Next arc-tier inspection: at next test262-sample re-run.
