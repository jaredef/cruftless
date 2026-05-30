# ta-element-coercion-spec-faithful — Findings

Per-locale empirical findings extracted from `trajectory.md`. Cross-referenced in `apparatus/docs/findings-ledger.md`. Append-only per Doc 727 §X basin-stability discipline.

The locale was founded 2026-05-30 at TAECSF-EXT 0. First `findings.md` authored 2026-05-30 after TAECSF-EXT 1 NEGATIVE (Rule 13 revert) produced a second rung's worth of empirical material — per the per-locale convention (defer `findings.md` extraction until the second productive or instructive rung in the locale).

## Finding TAECSF.1 — narrow dispatcher beats wide signature lift for Result-threading

**Source**: TAECSF-EXT 0 (LANDED 2026-05-30).

**Class**: error-propagation.

**Statement**: when a coercion abstract op needs to propagate its error from a deep storage path through a non-Result-returning intermediate function with many callers (here `object_set_pk`, ~hundreds of callers), prefer a narrow new dispatcher that lives in the Result-returning caller's frame over lifting the intermediate's signature. The narrow dispatcher's blast radius is the named call-site; the wide lift's blast radius is every caller of the intermediate.

**Predicts**: substrate rungs that lift a non-Result function to Result without first attempting a narrow-dispatcher alternative will incur 3-10× larger blast radius than necessary, with proportional regression risk. Substrate rungs that adopt the narrow-dispatcher pattern land with bounded blast radius and gate-clean.

**Evidence**: TAECSF-EXT 0 selected option (ii) (narrow `Runtime::typed_array_set_index_checked` at `interp.rs:632`) over option (i) (wide `object_set_pk` → Result lift). Substrate at ~70 LOC single-file. Gates landed clean: TAWR 63 → 67 (+4 PASS), diff-prod 61/51 → 64/48 (+3 PASS), TAMM 82 → 86 (+4). No regression on any instrument. Direct probe `new BigInt64Array(1)[0] = "bad"` → SyntaxError per ECMA-262 §7.1.13.

**Composes with**: Rule 4 (never split a substrate move; bundle-vs-narrow scoping is the natural sibling decision), Rule 21 (probe-first scoping for substrate cost; the narrow dispatcher is the probe that defeats the architectural property at minimum cost).

**Promotion-readiness**: one-more-observation. Candidate apparatus standing-rule (Rule 27 or post-rules-16–22-consolidation #). Awaiting a second cross-locale instance of the dispatcher-over-lift heuristic. Candidate sites: template-literal ToNumber error-path (per MILF-EXT 8's BigInt-template-literal close), other Result-thread-through-non-Result-callsite work in the engine.

## Finding TAECSF.2 — direct-probe success can coexist with cluster regression via non-obvious shared state

**Source**: TAECSF-EXT 1 (NEGATIVE, Rule 13 revert, 2026-05-30).

**Class**: measurement-discipline.

**Statement**: when a substrate edit passes the direct probe assertions designed in the proposal but fails the cluster regression-gate, the mechanism is rarely in the substrate edit's named scope. The substrate edit incidentally touches shared state (prototype-chain reads, cache invalidation, refcount sensitivity, match-arm codegen, branch-prediction-sensitive ordering) at the dispatch site that surfaces only in the cluster's broader test coverage.

**Predicts**: rungs whose proposal lists direct-probe assertions as the load-bearing terminus will, when those probes PASS, sometimes regress cluster gates by a small number (1–5 cells) via the non-obvious-shared-state mechanism. The diagnostic discipline is Rule 22 (spot-read 2–3 residual tests) + Rule 13 (revert, identify deeper-layer closure). Forward-pursuit at the dispatch site without diagnosis re-regresses on the next attempt.

**Evidence**: TAECSF-EXT 1 (2026-05-30) passed 10/10 direct probe assertions including five spec cells (Uint8 wrap 300→44; Int8 wrap 130→-126; Uint8Clamped NaN→0; Uint8Clamped saturate 300→255; round-half-to-even 254.5→254) plus EXT 0 regression check (`BigInt64Array(1)[0] = "bad"` SyntaxError). TAWR remained 67/100; diff-prod remained 64/48. But TAMM regressed 86 → 83 (-3 PASS), with all 3 regressions in non-integer-kind cells:
1. `built-ins/TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js`
2. `built-ins/TypedArrayConstructors/internals/Set/conversion-operation-consistent-nan.js`
3. `built-ins/TypedArray/prototype/some/BigInt/values-are-not-cached.js`

The substrate edit's BigInt branch was byte-equivalent in semantics to EXT 0; the Float fall-through was also byte-equivalent. The regression mechanism is in the shared state the edit incidentally touched (the kind-detection path's `object_get` invocation pattern, the `kind` String allocation, the match-arm ordering, or NaN bit-pattern interaction). Hypotheses (a)/(b)/(c) named in trajectory.md.

**Composes with**: Rule 1 (report per-workload; cluster gates must be run before declaring closure), Rule 13 (revert-then-deeper-layer closure; this rung is a Rule-13 instance), Rule 15 (chapter-close-inspect; the cluster regression surfaces only at chapter-close, not at the direct probe site), Rule 22 (partial-exemplar-closure as substrate-axis discriminator; the cluster regression is itself a partial-axis surfacing that points at the dispatch site's shared state rather than the substrate scope).

**Promotion-readiness**: trajectory-embedded; one-more-observation. Candidate apparatus standing-rule (Rule 28 or post-consolidation #). Awaiting a second locale's instance of the direct-probe-vs-cluster-regression divergence pattern. Until promoted, TAECSF.2 is a locale-local measurement discipline that the EXT 2 rung must internalize before re-attempting the integer-kind substrate move.

**Standing carry-forward**: the `abstract_ops::convert_number_to_typed_array_element` helper remains on disk (added at EXT 1, retained post-revert) per Finding IR.33 cumulative substrate amortization. It is the substrate prefix that TAECSF-EXT 2's deeper-layer closure consumes.

## Finding TAECSF.3 — typed-array buffer storage is Value-cell-aliased, not byte-aliased; coercion at SetIndex breaks view-aliasing pass-through

**Source**: TAECSF-EXT 1 NEGATIVE convergent diagnosis (2026-05-30, under keeper Telegram 10580 "investigate").

**Class**: substrate-pattern (engine-architectural).

**Statement**: the engine's `ArrayBufferData.data` field is `Vec<Value>`, not `Vec<u8>`. Each typed-array view stores its element value at the byte-offset position as a `Value` of any kind (Number / BigInt / String / Object). Multiple views aliasing the same buffer share the same `Value` cells. The test262 resizable-buffer harness exploits this aliasing in `copyIntoArrayBuffer`: it copies between buffers byte-by-byte via `destView[i] = srcView[i]` on Uint8Array views regardless of the underlying buffer's actual element type. Pre-EXT 1, this round-tripped any Value (including Value::BigInt) through the Uint8Array storage cell unchanged. With EXT 1's spec-faithful coercion at the Uint8Array dispatch site, the Uint8Array write coerces the BigInt to a Number, breaking the aliasing pass-through and corrupting the subsequent BigInt64Array view of the same buffer.

**Predicts**: any substrate edit that adds spec-faithful coercion at the SetIndex dispatcher for typed-array views — without first migrating `ArrayBufferData.data` to actual byte storage (`Vec<u8>` with NumberToRawBytes encoding per ECMA-262 §6.1.6.1) — will regress the test262 `internals/Set/` + resizable-buffer cells that rely on cross-view aliasing pass-through. The regression is silent on direct probe assertions (which exercise a single view's coercion in isolation) and surfaces only at the cluster level (which includes the harness's view-aliasing setup).

**Evidence**: TAECSF-EXT 1 substrate edit re-applied under keeper "investigate" directive Telegram 10580. Test cell `built-ins/TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js` ran under EXT 1 substrate; output: `Expected SameValue(«42», «42n») to be true`. The constructor `new BigInt64Array(makeResizableArrayBuffer(BigInt64Array, [42n, 43n]))` invokes the harness's `copyIntoArrayBuffer`, which executes `destView[i] = srcView[i]` on Uint8Array views aliased to the buffer. Under EXT 1, the Uint8Array write calls `convert_number_to_typed_array_element(Value::BigInt(42n), "Uint8Array")` which calls `to_number(Value::BigInt(42n))` (yielding `42.0`) then `to_uint_n(42.0, 8)` (yielding `42`), storing `Value::Number(42)` at the cell. The subsequent BigInt64Array view reads `Value::Number(42)` at byte_index 0, fails the `=== 42n` assertion. Identical mechanism for the other two regressing cells (Set/conversion-operation-consistent-nan tests Float NaN bit-pattern aliasing; values-are-not-cached tests BigInt-TA cache invalidation through the harness's makeResizableArrayBuffer path).

**Deeper-layer closure (the TAECSF-EXT 2 design)**: integer-kind coercion at the SetIndex dispatcher CANNOT land correctly until one of three preconditions holds:
- (a) `ArrayBufferData.data` migrated from `Vec<Value>` to `Vec<u8>` with NumberToRawBytes per §6.1.6.1 + view-tier dispatch on read. Massive substrate move; engine-architectural; multi-rung; likely a new locale or arc rather than a continuation rung of TAECSF.
- (b) Heuristic guard at the integer-kind dispatch: skip coercion when the incoming `value` is `Value::BigInt(_)` or `Value::Number(n)` where `n` was just produced by a view-aliased read. Brittle; depends on the engine recognizing view-aliasing reads.
- (c) Move the coercion site from the bytecode `Op::SetIndex` to a different layer (e.g., the `Object.defineProperty` path at line 3848, which doesn't intersect the harness's `copyIntoArrayBuffer` aliasing pattern). Closes a smaller surface of user-facing assignments but preserves test cluster.

**Recommendation**: TAECSF-EXT 2 escalates to (a) as a precursor architectural rung (likely founded as a sibling locale `typed-array-byte-storage-conformance` since the substrate scope is larger than this locale's telos). Sub-substrate (a) integer-kind coercion within TAECSF remains DEFERRED until the precursor lands. Sub-substrate (b) Float32 canonical-NaN preservation is structurally subject to the same architectural constraint; deferred jointly with (a).

**Composes with**: Rule 13 (revert-then-deeper-layer-closure; this finding IS the deeper-layer closure pointing at engine architecture rather than dispatch routing), Rule 15 (chapter-close-inspect; the convergent diagnosis came from spot-reading ONE regressing test262 cell rather than chasing the cluster delta in aggregate), Rule 21 (probe-first scoping; the EXT 1 probe correctly answered the "can integer-kind coercion land at the SetIndex dispatch alone?" question — **no** — at bounded cost).

**Promotion-readiness**: trajectory-and-findings-embedded; one-more-observation. Candidate apparatus standing-rule: "engine architectural constraints can invalidate substrate-level fixes even when the fix is spec-faithful in isolation; the test cluster surfaces this when the harness exploits engine-internal invariants the fix would otherwise break." Awaiting a second locale's instance of substrate-spec-correctness-vs-engine-architecture conflict.
