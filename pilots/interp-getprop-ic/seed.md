# interp-getprop-ic — Resume Vector / Seed

**Locale tag**: `L.interp-getprop-ic` (top-level per Doc 737 §IV)

**Status as of 2026-05-24**: **WORKSTREAM FOUNDED (GPI-EXT 0)**. Cross-locale follow-on from IHI's chapter close (IHI-EXT 11; CRB string_url_sweep -3.6%). Per the IHI-EXT 10/11 cost-analysis: the reclaim ceiling on string_url_sweep is structurally bounded by per-call cost components OUTSIDE IHI's scope — primarily Op::GetProp's descriptor-walk dispatch (~200-500ns per resolve; called once per method-call inner-iter). This pilot closes the GetProp interp-tier dispatch tier.

**Workstream**: build a per-call-site interp-tier IC for Op::GetProp method-resolve sites. Pattern mirrors IHI-EXT 11's bytecode-rewrite approach: on first successful resolve at a pc, rewrite Op::GetProp to a cached variant that skips the descriptor walk + cached the resolved Value object directly. Per Doc 740 §IV.2 + standing rule 13 (revert-then-deeper-layer-closure): design from the deeper-layer first to avoid IHI-EXT 7's wrong-cache-lifetime mis-design.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: `interp-hot-intrinsics/` (cross-tier pattern; closes Op::CallMethod tier; this pilot closes Op::GetProp tier).
**Composes with**:
- [Doc 741 §V.1](../../../corpus-master/corpus/741-the-multi-tier-cascade-pipeline-connects-an-empirical-materialization-of-doc-740-across-four-sibling-pilots-on-a-cruftless-cross-runtime-bench-fixture.md) — multi-tier-cascade-revival materialized at interp tier
- [findings.md Addendum IX](../rusty-js-jit/findings.md) — standing rule 13 + Findings VIII.4-6 (this pilot designs from the deeper-layer per the discipline)
- [IHI-EXT 11 trajectory entry](../interp-hot-intrinsics/trajectory.md) — empirical precedent; bytecode-rewrite pattern proven
- [Op::GetProp handler in interp.rs](../rusty-js-runtime/derived/src/interp.rs) — target site for the IC
- [string_url_sweep component A/B probe](../cross-runtime-bench/fixtures/string_url_sweep/component-ab-probe.mjs) — empirical anchor

## I. Telos

**Empirical answer to**: can a per-call-site IC at Op::GetProp's interp dispatcher close the ~200-500ns/resolve cost surface for method-call resolve patterns (Dup; GetProp "key"; ...; CallMethod) in interp-tier-bound hot loops?

The bench-anchored target: post-implementation, string_url_sweep header_loop drops an additional 50-150 ms (from current 284.5 ms toward 130-235 ms = -10-25% additional) via Op::GetProp dispatch elimination at hot method-resolve sites. Combined with IHI's -14% header_loop reclaim, would bring header_loop toward -25-40% vs original 332 ms — approaching Pred-ihi.5's ≥30% target via cross-locale composition (Doc 740 §II.2 P4).

### I.1 First-cut scope

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. Skip the Frame-cache mis-design that IHI-EXT 7 paid for; go directly to bytecode rewrite.

- **Op::GetPropOnString** (new opcode 0xFD) — bytecode-rewrite target for GetProp on String receivers
- **Op::GetPropMethodCached** (new opcode 0xFE; index-rewrite for method resolves) — for the Dup;GetProp;CallMethod pattern specifically
- **Substrate-introduction first cut**: focus on **method-resolve** pattern (Dup;GetProp;CallMethod-or-CallMethodIcCached) since that's the IHI-EXT 11 hot-path consumer
- **Cache shape**: same as IHI's CachedDispatch but for GetProp resolution; on first resolve at pc, write the resolved Value's discriminator + pointer into the bytecode

Out of scope (deferred):
- Op::GetProp on non-String receivers (Object property access; broader; needs more careful design)
- Op::GetProp standalone (without following CallMethod; e.g., `s.length` standalone access)
- Object-shape-aware IC (Σ stub-emitter at JIT tier handles this for JIT eligible code; interp tier deferred)

### I.2 Constraints (Pin-Art enumeration)

```
C1. Existing default-on paths byte-identical post-GPI.
C2. Each rewrite-rules entry correctness-preserving on its own.
C3. Bytecode rewrite SAFE: cruft single-threaded; FunctionProto's
    bytecode is owned Vec<u8>; byte-aligned writes; idempotent.
C4. Override-safety: cached resolve invalidates if user code adds an
    own-property at the same key on the receiver type after cache
    population. First-cut: no invalidation (frozen-prototype assumption);
    document Finding GPI.1 candidate for hardening.
C5. Composition with IHI's Op::CallMethodIcCached: the rewritten
    GetProp must produce a Value the IHI cached entry's lookup logic
    accepts (or directly bind to IHI table entries at GetProp time,
    eliminating BOTH the GetProp + the CallMethod table-lookup).
C6. Per standing rule 13 + Doc 740 §IV.2: design from deeper-layer
    first; skip Frame-cache mis-design.
C7. Rule 11 5-axis pre-spawn check:
    (A1) component A/B: ALREADY DONE — string_url_sweep header_loop's
         GetProp is the ~200-500ns/iter component per IHI-EXT 10/11
         cost-analysis (Doc 741 instance precedent)
    (A2) op-set coverage: Op::GetProp is in interp's set already; just
         adding a fast-path arm
    (A3) value-domain coverage: receiver is String (via header value);
         no encoding change needed at interp tier
    (A4) locals-marshaling: N/A (no JIT body)
    (A5) emission-shape coverage: N/A (no region extraction)
```

### I.3 Falsifiers

**Pred-gpi.1**: per-rewrite-rule LOC ≤50. Mirrors Pred-ihi.1.

**Pred-gpi.2**: canonical fuzz (acc=-932188103) byte-identical post-implementation.

**Pred-gpi.3**: diff-prod 42/42 holds.

**Pred-gpi.4**: composition with all defaults ±5%.

**Pred-gpi.5**: string_url_sweep header_loop drops ≥10% additional beyond IHI-EXT 11's 284.5 ms (target ≤256 ms). Sub-target: CRB string_url_sweep cumulative reclaim crosses 5% threshold (target ≤706 ms from 743 baseline).

### I.4 Composition with IHI per Doc 740 §IV.2

The IHI chain demonstrated that cumulative reclaim materializes at the DEEPER-LAYER closure (IHI-EXT 11 bytecode rewrite) AFTER the cache-tier substrate-introduction rounds. GPI per standing rule 13: SKIP the cache-tier substrate-introduction; go directly to bytecode rewrite via deeper-layer design.

The combined `Dup; GetProp "X"; LoadConst arg; CallMethodIcCached(idx)` pattern (post-IHI) becomes `Dup; GetPropMethodCached(ihi_idx); LoadConst arg; CallMethodIcCached(idx)` post-GPI — two bytecode rewrites compose at the same source-line method-call. The cumulative dispatch cost drops further.

Even cleaner: at GetProp rewrite time, if the method resolves to an IHI_TABLE entry, encode that idx directly. Subsequent dispatch reads the GetProp's cached idx + skips the actual GetProp resolve entirely (it's NoOp since CallMethodIcCached doesn't actually need the method value).

## II. Apparatus

- **Bytecode**: new opcodes 0xFD (GetPropOnString) and/or 0xFE (GetPropMethodCached). Per IHI-EXT 11 precedent.
- **Dispatch**: rewrite Op::GetProp's handler to detect String-receiver pattern + cache result; on first hit, rewrite op + operand.
- **Cache key**: site_pc (per IHI-EXT 11 model). Single-byte operand encodes the IHI_TABLE entry idx for the method-call form OR a Runtime-tier secondary cache idx for the property-get form.
- **Override-safety**: Doc-deferred per C4; first-cut no invalidation.
- **Composition with IHI**: same standing rule 11 5-axis discipline; rule 13 (deeper-layer design) applied prospectively.

## III. Methodology

1. **GPI-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **GPI-EXT 1** — design doc: bytecode shape + dispatch shape + IHI-composition strategy + per-entry LOC budget.
3. **GPI-EXT 2** — infrastructure + first rewrite-target (Dup;GetProp(method-key);CallMethodIcCached pattern).
4. **GPI-EXT 3** — composition probe + Pred-gpi.* booking.

## IV. Carve-outs and bounded scope

- Interp-tier only; JIT-tier Σ stub-emitter handles JIT-eligible GetProp.
- String-receiver method-resolve only at first cut; Object/Array deferred.
- No override-invalidation at first cut; hardening tier work.
- Aarch64 only.
- Per standing rule 13: design from deeper-layer first; no cache-tier substrate-introduction mis-design.

## V. Standing artefacts

- `pilots/interp-getprop-ic/seed.md`, `trajectory.md`
- `pilots/interp-getprop-ic/docs/design.md` (GPI-EXT 1)
- `pilots/interp-getprop-ic/fixtures/` for any pilot-specific tests
- Implementation lands in `pilots/rusty-js-bytecode/derived/src/op.rs` (new opcodes) + `pilots/rusty-js-runtime/derived/src/interp.rs` (handler + rewrite)

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read IHI's trajectory chapter-close (IHI-EXT 11 + chapter close entries) for the bytecode-rewrite precedent + Doc 740 §IV.2 empirical materialization. Read findings.md Addendum IX standing rule 13 for the revert-then-deeper-layer-closure discipline this pilot applies prospectively. Read Doc 741 for the multi-tier cascade-revival apparatus.
