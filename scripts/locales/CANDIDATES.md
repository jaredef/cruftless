# Locale candidates — next-spawn queue

Living document; append-only at the bottom (newest at top within a section). Each candidate is a prospective Pin-Art locale not yet founded as `pilots/<name>/seed.md`. Spawning protocol: per standing rule 11, do a 5-axis pre-spawn check (component A/B + op-set + value-domain + locals-marshaling + emission-shape) before founding. Per standing rule 13, design from the deeper-layer first; do not pay the cache-tier substrate-introduction tax if the closure tier is known.

**Status legend**: `🟢 RIPE` (rule 11 satisfied, ready to spawn) · `🟡 PROBED` (component A/B in progress) · `⚪ HYPOTHETICAL` (not yet probed).

---

## Tier A — empirically-anchored single-pilot yield targets

### (a) `iter-protocol-bytecode-rewrite` — **SPAWNED** 2026-05-24 as [`pilots/iter-protocol-bytecode-rewrite/`](../../pilots/iter-protocol-bytecode-rewrite/seed.md)
**Telos**: close the for-of protocol envelope dispatch in hot loops. Per GPI-EXT 3 cost analysis, after IHI+GPI's hot-method-call closure (~260 → ~15 ns/iter), the per-iter dominator on `string_url_sweep` header_loop is the IterInit/IterNext/IterClose dispatch + the synthetic iterator-result object allocation per `.next()`.
**Anchor**: `string_url_sweep` header_loop A/B probe at ~252-260 ms (post-GPI). Per the cost model, IterInit/IterNext is ≥50% of remaining per-iter cost.
**Deeper-layer design (rule 13 prospective)**: bytecode-rewrite the `IterInit; LabelTop; IterNext; JumpIfDone; ...body...; Jump LabelTop; IterClose` pattern into a single `IterFastLoop(local_idx)` for Array/String receivers where the iterator is the well-known intrinsic (not user-overridden). Eliminates per-`.next()` synthetic object allocation; index-based scan over the underlying String bytes / Array elements.
**LOC estimate**: ~60-100 (the rewrite pattern detection is non-trivial; consider a discovery-pass over FunctionProto on first invocation rather than per-dispatch).
**Cross-tier dual**: would also benefit OSR-eligible loops; consider promoting to JIT tier after interp-tier proves.

### (b) `jit-getprop-method-ic` — 🟡 PROBED
**Telos**: extend Σ stub-emitter's existing String-receiver property-get path to handle method-resolve composition with HI's CallMethodIcCached. Currently Σ handles standalone `s.length` and `s[i]`-style access; method dispatch falls through to a generic call path even when HI would fast-path the call.
**Anchor**: needs JIT-eligible fixture A/B probe; string_url_sweep's hot loop is interp-bound, so JIT GPI requires a different empirical anchor (json_parse_transform's `.charCodeAt` is OSR-eligible per HI close).
**Deeper-layer design**: stub emit `GetProp(method-name) + IcCached(idx)` as a fused JIT instruction sequence that directly invokes HI's IcEntry.fast without round-tripping through the interp dispatcher.
**LOC estimate**: ~80-120 (Cranelift stub assembly + IcEntry handoff).
**Risk**: composition with existing OSR boundary; the JIT-→interp re-entry on bail must handle the rewritten bytecode.

### (c) `ihi-array-entries` — 🟢 RIPE
**Telos**: extend IHI_TABLE with Array.prototype intrinsic entries: push/pop/shift/unshift, forEach (with callback), indexOf, includes, slice, concat. Pattern mirrors existing String entries.
**Anchor**: `json_parse_transform` is Array-method-heavy; pre-GPI median 1773 ms. Per-entry LOC budget per IHI's existing pattern: ~30-50 LOC each.
**Deeper-layer design (rule 13 prospective)**: skip cache-tier substrate work; reuse the existing CallMethodIcCached + GetPropSkipForMethod bytecode-rewrite infrastructure. Each Array entry costs the same shape as a String entry.
**Composition risk**: receiver_kind dispatch must distinguish Object-receivers-that-are-Arrays from generic Objects; the IhiReceiverKind::Array gate already exists per interp_ic_table.rs.
**Predicted yield**: json_parse_transform 1773 → ≤1600 ms (-10%); cruft/node 14.78x → ≤13.3x.

### (d) `gpi-override-safety` — ⚪ HYPOTHETICAL
**Telos**: harden Op::GetPropSkipForMethod against user-installed own-property override of an intrinsic key on the receiver type. First-cut GPI assumes frozen-prototype semantics; a user adding `String.prototype.toLowerCase = function() { ... }` after GPI rewrite would not invalidate the cache.
**Anchor**: a synthetic correctness fixture, not a perf surface. Spawn only if a real-world consumer-app surfaces the divergence.
**Deeper-layer design**: per-IhiEntry override-version counter at Runtime; bump on String.prototype mutation; check at GetPropSkipForMethod dispatch.
**Cost**: adds a per-dispatch counter compare; may regress GPI's reclaim.
**Disposition**: deferred until consumer-app shows the need.

---

## Tier B — broader surface targets (rule 11 component A/B still pending)

### (e) `arith-tight-loop-closure` — ⚪ HYPOTHETICAL
**Telos**: arith_tight_loop @ 422 ms / cruft/node 2.10x is the best ratio on CRB. The remaining gap to node is a candidate for closure via I64 typed-op promotion in interp (analogous to JIT's AddI64/MulI64 closure).
**Anchor**: needs component A/B probe to identify the dominator (typed-op dispatch vs. f64 boxing per iter).
**Deeper-layer design**: TL's existing I64 unbox at JIT extends to interp by treating loops with all-I64-typed locals as I64-direct.

### (f) `module-loader-eager-cache` — ⚪ HYPOTHETICAL
**Telos**: ESM import resolution cost on cold-start. Surfaces in consumer-app tests, not CRB.
**Anchor**: needs a consumer-app cold-start timing instrument; CRB fixtures are single-module so no anchor.

### (g) `regex-jit-precompile` — ⚪ HYPOTHETICAL
**Telos**: regex compile-on-first-test → precompile-at-LoadConst. version_regex + id_regex deltas (14, 20 ms per string_url_sweep A/B) are small but compound across modules.
**Anchor**: A/B probe variants V3, V4 (already exist in component-ab-probe.mjs).
**Deeper-layer design**: precompile regex literals at FunctionProto load time (constant pool); cache the compiled matcher.

---

## Tier D — strategic / language-tier (new 2026-05-24)

### (l) `ts-consumer-corpus` — **SPAWNED** 2026-05-24 as [`pilots/ts-consumer-corpus/`](../../pilots/ts-consumer-corpus/seed.md)
**Telos**: empirical measurement instrument for TSR's coverage of real consumer `.ts` source on npm. Failure-table drives priority order for the downstream TSR sub-locale arc (enums, classes, generics-calls, decorators, namespaces, conditional-types, JSX).
**Status**: SPAWNED. TCC-EXT 1 (corpus assembly) is the next round.

### (m-s) `ts-resolve-*` sub-locales — ⚪ QUEUED PENDING TCC FAILURE-TABLE
- `ts-resolve-enums/` — runtime-bearing; enum reverse-mapping + lowering
- `ts-resolve-classes/` — ctor-param shorthand, abstract, accessor modifiers
- `ts-resolve-generics-calls/` — f<T>() angle-bracket disambig vs `<` operator
- `ts-resolve-decorators/` — Stage 3 decorators; runtime descriptors
- `ts-resolve-namespaces/` — legacy but persistent in tooling code
- `ts-resolve-conditional/` — cond + mapped + template-literal types (strip-only; combined)
- `ts-resolve-jsx/` — separate locale; JSX/TSX
**Disposition**: priority order will be set by TCC-EXT 2's failure-table. Each sub-locale targets ≤3 implementation rounds per standing-rule-13 thesis.



### (j) `ts-resolve` — **SPAWNED** 2026-05-24 as [`pilots/ts-resolve/`](../../pilots/ts-resolve/seed.md)
**Telos**: native `.ts` execution by cruft via a TS source-language resolver upstream of rusty-js-ir. Empirical-first stage of a two-locale arc with `cruftscript-spec/`. Load-bearing research question: do erased TS annotations carry substrate-actionable signal for downstream IC/JIT/VD tiers?
**Status**: SPAWNED. TSR-EXT 1 (design doc) is the next round.

### (k) `cruftscript-spec` — ⚪ DEFERRED; TSR-EXT 5 PROBE RETURNED NULL AT IPBR CONSUMER 2026-05-24
**Telos**: design and specify CruftScript — a sound statically-typed sibling language to TS, following the Typed Racket model (typed code internally sound; runtime contracts at typed/untyped FFI boundary). The key architectural lever: types as first-class substrate input (drives JIT IC specialization, IHI/GPI/IPBR shape probes, VD's NaN-boxed tag schema) rather than erased upstream as tsc and Typed Racket do.
**Disposition**: deferred until TSR-EXT 5's annotation-sidecar probe data lands. Positive signal → cruftscript-spec founded on grounded substrate claims. Null signal → cruftscript-spec proceeds on soundness-alone grounds (still valuable but smaller corpus claim).
**Anchor**: TSR's empirical data on annotation-as-substrate-hint will inform the language design's scope + grammar bounds.
**2026-05-24 UPDATE per TSR-EXT 5 Finding TSR.1**: probe returned NULL at the IPBR consumer (per-iter shape-lookup cost too small for annotation-driven elimination to surface above noise). Load-bearing claim for cruftscript-spec shifts from "iter-protocol-shape-skip substrate" to **"JIT IC specialization on typed function args + VD NaN-box tag preservation through typed numerics"**. Each follow-on consumer needs its own empirical probe before substrate-leverage claim can be made for that consumer. Locale remains worth spawning, but on weaker (still valuable) grounds.

---

## Tier C — discipline / methodology / corpus work

### (h) `standing-rules-codification-pass` — **FORMALIZED** 2026-05-24 as [`docs/standing-rule-13-prospective-application.md`](../../docs/standing-rule-13-prospective-application.md)
**Telos**: review findings.md (20 findings; 13 standing rules) for consolidation. Standing rule 13's prospective application across IHI → GPI is a candidate Doc 7xx corpus publication (multi-tier-cascade-revival applied PROSPECTIVELY, not retrospectively).
**Output**: 1 corpus doc (~150-200 lines); refresh findings.md Addendum X.
**Status**: working draft landed in `docs/`; candidate for promotion to corpus Doc 742 after one additional empirical corroboration (e.g., `ihi-array-entries`) or keeper review of thesis at current anchor.

### (i) `crypto-sha256-batch-investigation` — ⚪ HYPOTHETICAL
**Telos**: CRB crypto_sha256_batch FAIL is pre-existing (CRB-EXT 0-6 baseline bb212c3c). Investigate root cause; could be missing host stub or `Buffer` API gap.
**Anchor**: stderr from `cruft pilots/cross-runtime-bench/fixtures/crypto_sha256_batch/main.mjs`.

---

## Spawning protocol

1. Read this file + identify the candidate.
2. Run rule 11 5-axis pre-spawn check (component A/B is the load-bearing one).
3. If 🟢 RIPE: spawn `pilots/<name>/{seed.md,trajectory.md,docs/,fixtures/}` with the seed founding-pattern (telos, constraints, falsifiers, methodology, carve-outs, resume protocol).
4. Refresh `scripts/locales/manifest.json` via `scripts/locales/discover.sh`.
5. Commit founding + manifest in one change.
6. Per standing rule 13: design the deeper-layer closure from the founding round if known.

## Standing edits

- When a locale is founded, **move its entry from this file to its own `pilots/<name>/seed.md`**; leave a one-line "**SPAWNED** as `pilots/<name>/` at YYYY-MM-DD" stub here for the audit trail.
- When a candidate is empirically falsified (component A/B shows the dominator is not what was predicted), strike through + annotate why.
- When new candidates surface from chapter-close disposition sections of other locales, append them under the appropriate tier.
