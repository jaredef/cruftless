# LeJIT-Σ — Resume Vector / Seed

*(Internal name LeJIT-Σ per the LeJIT seed §I.2 pre-file. On-disk source code lives at `pilots/rusty-js-jit/derived/src/stub_*.rs` within the parent LeJIT crate; the seed + trajectory at this coordinate hold the Pin-Art record.)*

**Locale tag**: `L.rusty-js-jit/stub-emitter` (nested per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (StubE-EXT 0)**. No code yet. Pre-filed at LeJIT seed §I.2 (JIT-EXT 25) as the closure round to the hidden-classes substrate (`pilots/rusty-js-shapes/`). Spawn triggered now to begin scaffolding against the stable IC consumer API contract `Object::shape_ptr_and_slot_for(name) -> Option<(*const Shape, u32)>` — the API is in place since Shape-EXT 4 and returns None for every Object pre-enrollment; the stub emitter can be designed, implemented, and tested against synthetic shape pointers in advance of the enrollment flip.

**Workstream**: hand-rolled inline-cache stub emitter for the property-access fast path. Per LeJIT seed §I.2's hybrid-stance recognition: Cranelift cannot patch call targets in place; mainstream JITs all use bespoke 2-3-instruction shape-check + slot-load stubs that self-modify on miss. LeJIT-Σ is cruftless's instance of that pattern.

**Author**: 2026-05-23 session.
**Parent**: `pilots/rusty-js-jit/` (LeJIT).
**Sibling**: `pilots/rusty-js-shapes/` (hidden-classes substrate this pilot consumes).
**Composes with**:
- [LeJIT seed](../seed.md) §I.2 — the hybrid telos that names this pilot as one of four hand-rolled regions.
- [shapes pilot seed](../../rusty-js-shapes/seed.md) §I + [docs/shape-design.md §11](../../rusty-js-shapes/docs/shape-design.md) — the IC consumer API contract and stable-pointer safety story.
- [Doc 731](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) §VII R1–R8 — single-tier baseline JIT shape that LeJIT-Σ preserves.
- [Doc 735 §X.h](../../../../corpus-master/corpus/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — three-probe-levels discipline (bench + consumer-route + fuzz) that gates (P2.a) strict-win claims for this pilot's perf substrate moves.
- [Doc 738 §II](../../../../corpus-master/corpus/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md) — source-tier convention space mapped for stub-emitter identifiers.

## I. Telos

Land a hand-rolled aarch64 IC stub emitter for the property-access fast path that:

1. **Caches `(shape_ptr, slot_offset)` per IC site** at first hit. The cache lives in JIT-emitted code (inline literal address + literal offset) or in a side-table indexed by call-site id; the choice is a §III design decision.

2. **Emits a 2-3-instruction fast path** at each IC site: shape-pointer compare against the cached value, branch on miss, slot load on hit, return. Compiles to ~10-15 bytes of aarch64 machine code.

3. **Patches itself on miss** by overwriting the cached shape pointer (and optionally promoting from monomorphic to polymorphic-IC if the round chooses; first cut is monomorphic). The patching uses Cranelift's `JITModule::finalize_definitions`-friendly memory regions plus aarch64 instruction-cache invalidation (`__builtin___clear_cache` / `dc cvau` + `ic ivau` + `dsb ish` per ARMv8.0).

4. **Falls through to the slow path** (the existing extern `runtime_getprop_on_object` runtime helper from JIT-EXT 22) on stub-cache miss after the patch fails to converge (e.g., after N transitions through distinct shapes — the polymorphic-IC degradation threshold).

5. **Achieves at least 3× per-hit speedup** over the current extern-call IC dispatch on a representative property-access hot loop, per LeJIT seed §I.2's falsifier threshold. Below 3×, re-categorize as (P2.d) correct-but-losing per Doc 735 §X.h.b and revert to extern-call dispatch documenting the boundary.

The success criterion is the conjunction of (1)-(5), gated through the three-probe-levels discipline of Doc 735 §X.h.c: bench probe (synthetic property-access loop), consumer-route probe (diff-prod 42/42 + a JIT-on hot-loop fixture), fuzz probe (shape-transition-history fuzz over the IC dispatch).

### I.1 Bounded first-cut telos

The first cut covers:
- **aarch64 only** (the engagement's reference hardware, Pi). x86_64 stubs ship as the closure round LeJIT-Σ'.
- **Monomorphic IC only** (one cached shape per site). Polymorphic-IC (linear scan of N cached shapes per site) ships as closure round LeJIT-Σ.poly.
- **GetProp only** (data-property reads). SetProp + Call/CallMethod ship as closure rounds LeJIT-Σ.set / LeJIT-Σ.call.
- **Shape-cache only** (no value-tag inline checks; that's the sibling Value-tag inline emitter pre-filed at seed §I.2 item 4). Routing for non-Object receivers goes through the extern slow path as today.

The first-cut closure criterion: a JIT-emitted GetPropOnObject sequence on aarch64 inlines a shape-check + slot-load when the receiver is Shaped and the cached shape matches; on miss, patches the cache once and falls through to the extern helper. Bench probe shows ≥3× speedup vs the current extern-call dispatch; diff-prod 42/42 held; the new stub passes fuzz over the property-addition-history space.

## II. Apparatus

LeJIT-Σ is **a hand-rolled aarch64 instruction-emission tier alongside Cranelift** within the LeJIT crate. It composes with:

- **Cranelift** (resolver-instance #N below LeJIT) owns the function body's lowering for everything except IC sites. At each IC site (Op::GetPropOnObject), Cranelift emits a `call` to a known-address trampoline that LeJIT-Σ owns.
- **The hidden-classes substrate** (`pilots/rusty-js-shapes/` + Object::shape_ptr_and_slot_for) supplies the cache key. The cache is `(*const Shape, u32)` per the API contract. Stability is the design §11 safety story: IC stub holds a `Rc<Shape>` alongside the cached pointer to guarantee the allocation outlives the stub.
- **The LeJIT deopt machinery** (JIT-EXT 10-17) is the fallback for hard failures (e.g., the receiver isn't even an Object — shape_ptr_and_slot_for would return None and the stub takes the slow path; if the slow path itself throws, the existing deopt thunk routes back to the interpreter).

Per Doc 730 §XII–§XVI, the bidirectional engine-diff oracle gates each substrate move under diff-prod 42/42 + test262-sample 77.6%. The new probe class is the **bench-probe inline-IC speedup test** at `pilots/rusty-js-jit/derived/tests/bench_ic.rs` measuring extern-call vs stub-emitted dispatch on a synthetic 1M-iteration property-access loop. Pre-LeJIT-Σ baseline establishes the comparison point at StubE-EXT 1.

## III. Methodology

Each StubE-EXT is a substrate move per Doc 581 + Doc 729 §A8.13:

1. **StubE-EXT 0 (this round)** — workstream founding. seed.md + trajectory.md + docs/ scaffold. No code.
2. **StubE-EXT 1** — Pre-stub bench probe. Establish baseline measurement: `bench_ic.rs` running 1M iterations of `obj.x` reads through the current extern-call dispatch. Output: docs/bench-baseline.md with the per-iter cost in nanoseconds on the engagement's Pi target.
3. **StubE-EXT 2** — Stub emitter design. Cache layout (inline literal vs side-table), patching mechanism (aarch64 instruction-cache flush sequence), monomorphic-IC state machine, deopt-on-miss handoff. Output: docs/stub-design.md.
4. **StubE-EXT 3** — Stub emitter crate scaffold. `stub_aarch64.rs` module in the LeJIT crate. `StubAarch64` type + `emit_getprop_stub` function emitting raw aarch64 bytes into Cranelift-allocated executable memory. Test-only; not wired into the JIT translator.
5. **StubE-EXT 4** — Synthetic shape-pointer integration test. The test allocates a real Object via the shapes crate API (constructing it manually since enrollment hasn't flipped — call `Shape::root().transition_to("x")` directly, push a value into `shape_values`, set `shape` on a test Object), points the stub at the resulting `(shape_ptr, 0)` cache key, and verifies the stub returns the expected value via aarch64 emulation in a test harness OR direct execution if Pi-deployed.
6. **StubE-EXT 5** — Wire the stub into the JIT translator. Op::GetPropOnObject codegen path routes through stub-call rather than extern-call when LeJIT-Σ is enabled (env flag `CRUFTLESS_LEJIT_STUB=1` per the survey R2 mitigation pattern).
7. **StubE-EXT 6** — Bench measurement under env flag on the post-enrollment shapes substrate. Measure 3× threshold; categorize per Doc 735 §X.h.b.
8. **StubE-EXT 7** — Fuzz probe: property-addition-history fuzz over the IC dispatch space. Verify no (P2.c) illegal-speed via stub-cache stale-after-shape-transition bugs.
9. **StubE-EXT 8** — Promote `CRUFTLESS_LEJIT_STUB` from opt-in to default-on when all gates green.

StubE-EXTs 4-8 depend on the hidden-classes substrate's CMig-EXT 8 enrollment flip. EXTs 0-3 can land before that — they design and scaffold against the stable API contract.

## IV. Carve-outs and bounded scope

- **No x86_64 in this pilot.** LeJIT-Σ' is its own future closure round.
- **No polymorphic-IC in this pilot.** Monomorphic only; LeJIT-Σ.poly closes that gap.
- **No SetProp / Call / CallMethod stub.** GetProp only.
- **No Value-tag inline checks.** Sibling pilot `value-tag-inline` (per LeJIT seed §I.2 item 4) handles that.
- **No tiny-fn fast-baseline.** Sibling pilot `tiny-baseline` (per LeJIT seed §I.2 item 5) handles that.

These carve-outs preserve the substrate-introduction-round discipline. Each carve-out is a candidate closure round when its surface becomes load-bearing.

## V. Standing artefacts

- `pilots/rusty-js-jit/stub-emitter/seed.md` (this file).
- `pilots/rusty-js-jit/stub-emitter/trajectory.md` — per-StubE-EXT log.
- `pilots/rusty-js-jit/stub-emitter/docs/bench-baseline.md` — StubE-EXT 1 output.
- `pilots/rusty-js-jit/stub-emitter/docs/stub-design.md` — StubE-EXT 2 output.
- `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` — StubE-EXT 3 onward; lives in the LeJIT crate because the stub emitter is a module of the LeJIT codegen path.
- `pilots/rusty-js-jit/derived/tests/bench_ic.rs` — StubE-EXT 1 + StubE-EXT 6.

## VI. Resume protocol

Read [LeJIT seed §I.2](../seed.md), [shapes pilot seed](../../rusty-js-shapes/seed.md) + [docs/shape-design.md §11](../../rusty-js-shapes/docs/shape-design.md), [Doc 731 §VII](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md), then this seed, then trajectory.md. The next substrate move is StubE-EXT 1 (bench baseline measurement) when the cruftless Pi target is available for measurement; the design (StubE-EXT 2) can begin in parallel since it's reading + documenting.

Pin-Art tag prefix: `Ω.5.P03.E2.stub-*` per `host/tools/tag-grammar.md` (the stub emitter is a compile-tier substrate move at the engine-internal codegen tier).

## VII. Composition with parent (LeJIT) and sibling (shapes)

Per Doc 729 §A8.13 substrate-amortization, the staging across the three locales:

| Locale | Role | Status |
|---|---|---|
| `pilots/rusty-js-shapes/` | Substrate-introduction (hidden classes) | Shape-EXT 4 closed; CMig-EXT 0-3 closed; CMig-EXT 4-9 remaining |
| `pilots/rusty-js-shapes/consumer-migration/` | Consumer fanout for the substrate | CMig-EXT 3 closed; CMig-EXT 4-9 in progress |
| `pilots/rusty-js-jit/stub-emitter/` (this) | Closure round consuming the substrate | StubE-EXT 0 (founding) |

LeJIT-Σ scaffolding (StubE-EXTs 0-3) runs concurrently with the shapes work since it operates against the stable API contract without needing actual enrolled Shaped objects. StubE-EXTs 4-8 gate on CMig-EXT 8's enrollment flip. The two pilots compose at the `Object::shape_ptr_and_slot_for` API boundary.

## VIII. Falsifiers

**Pred-stub.1.** The hand-rolled aarch64 stub emitter achieves ≥3× per-hit speedup over the extern-call IC dispatch on a 1M-iteration property-access hot loop on the engagement's Pi. Falsifier: a measured speedup below 3× — re-categorize as (P2.d) per Doc 735 §X.h.b and revert.

**Pred-stub.2.** The stub's cached `*const Shape` remains valid (no use-after-free) across the stub's lifetime, even when the receiver Object transitions to a new shape (which decrements the cached shape's reference count via Rc dropping from the Object's `shape` field). Falsifier: a crash or undefined behavior in the fuzz probe (StubE-EXT 7) under high shape-transition workload.

**Pred-stub.3.** The IC stub's miss-and-patch sequence converges to a stable cached shape under monomorphic-call-site workload (one shape touches the site over its lifetime). Falsifier: a workload where the stub repeatedly misses-and-patches against the same shape (cache-key extraction bug) or fails to patch despite stable shape (instruction-cache-flush bug on aarch64).

**Pred-stub.4.** Per Doc 738 §II convention conformance: stub-emitter identifiers fit the five-axis source-tier coordinate space. `__ic_*` prefix for stub-internal sentinels per §II.a; `emit_getprop_stub` snake_case per §II.b; pillar-path `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` per §II.e. Falsifier: a stub-emitter identifier violating one axis without explicit corpus-articulated rationale.

**Pred-stub.5.** Doc 731 §VII R1 (single tier) preserves under the hybrid Cranelift + LeJIT-Σ codegen. The stub emitter is a sub-substrate of the same JIT tier, not a second tier. Falsifier: a substrate move that requires a meta-tier above LeJIT-Σ to coordinate between Cranelift and the stub emitter (would indicate R1 was misclassified for the hybrid stance).

## IX. Hypostatic boundary

Per Doc 372, this seed operates at the functional layer. The aarch64 instruction emission discipline is one realization of the inline-IC pattern that V8 / JSC / SpiderMonkey have all implemented at scale; the hand-rolled aarch64 stub is corpus-original in its composition with cruftless's narrow-alphabet Doc 731 stance, not in the per-instruction emission patterns themselves (which are published codegen-literature substrate per Doc 735 §X.h.b).

The corpus-original contribution is: a hand-rolled IC stub emitter alongside a generic codegen substrate (Cranelift) preserves Doc 731 §VII R1's single-tier baseline shape, contradicting the V8/SpiderMonkey assumption that IC fast-paths require their own tier. The pilot's three-probe-levels-gated result will corroborate or falsify that corpus claim.

---

*Doc 581 standing instrument: this seed is the workstream's stable kernel. Changes to telos / apparatus / methodology / carve-outs land here; per-EXT substrate moves land in trajectory.md.*
