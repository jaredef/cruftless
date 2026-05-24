# interp-hot-intrinsics — Resume Vector / Seed

**Locale tag**: `L.interp-hot-intrinsics` (top-level per Doc 737 §IV)

**Status as of 2026-05-24**: **WORKSTREAM FOUNDED (IHI-EXT 0)**. Cross-tier dual of `pilots/rusty-js-jit/hot-intrinsics/` (HI; closed at HI-EXT 5 with 4 entries). The interp-tier IC table bypasses call_function dispatch for hot intrinsic method calls regardless of JIT eligibility. Closes the structural gap that HI's JIT-tier table can't reach: interp-tier-bound hot loops (for-of bodies; non-OSR-eligible code; broad interp-tier dispatch sites).

Top-level locale (not nested under rusty-js-runtime/, which itself is not a Pin-Art locale). Spawned per keeper directive 2026-05-24 04:31-local as the (d) pivot from string_url_sweep's component A/B probe (header normalization loop = 77% of cruft's wall-clock; OSR doesn't fire on the for-of body's alphabet).

**Workstream**: build an interp-tier hot-intrinsic IC table consumed by Op::CallMethod's dispatcher in `pilots/rusty-js-runtime/derived/src/interp.rs`. Each table entry has the same shape as HI's JIT-tier IcEntry but the fast-path runs in interp (no JIT compile required). The CharCode-EXT 2 ad-hoc charCodeAt fast-path (interp.rs:8073-8125) is the precedent; this pilot materializes the precedent as a reusable table.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: `rusty-js-jit/hot-intrinsics/` (JIT-tier dual), `cross-runtime-bench/`, `diff-prod/`.
**Composes with**:
- [Findings Addendum IV](../rusty-js-jit/findings.md) — standing instrument list; this pilot materializes the cross-tier dual of HI
- [Doc 741 §V.1](../../../corpus-master/corpus/741-the-multi-tier-cascade-pipeline-connects-an-empirical-materialization-of-doc-740-across-four-sibling-pilots-on-a-cruftless-cross-runtime-bench-fixture.md) — the pattern generalizes; this is its second materialization (interp tier)
- [HI seed](../rusty-js-jit/hot-intrinsics/seed.md) — JIT-tier dual; entry-shape mirrored at the cross-tier
- [HI docs/design.md](../rusty-js-jit/hot-intrinsics/docs/design.md) — IcEntry struct + registration pattern; adapt to interp dispatch
- [CharCode-EXT 2](../rusty-js-runtime/derived/src/interp.rs) at lines ~8073-8125 — the ad-hoc precedent (charCodeAt interp IC); this pilot subsumes it
- [string_url_sweep component A/B probe](../cross-runtime-bench/fixtures/string_url_sweep/component-ab-probe.mjs) — empirical anchor; the header normalization loop's String intrinsics are the dominator

## I. Telos

**Empirical answer to**: can an interp-tier IC table close the dispatch-overhead surface for hot intrinsic method calls in non-JIT-eligible code paths (for-of bodies; non-OSR loops; module-level top-level loops with mixed alphabet)?

The bench-anchored target: post-implementation, string_url_sweep's header normalization loop drops by ≥30% (from 332 ms toward ~232 ms or below) via interp-tier IC fast-paths for toLowerCase + trim + indexOf + slice. Each entry bypasses call_function's frame setup + this-binding + descriptor walk + Value-boxing per method call.

### I.1 Starter set (per header-normalization-loop empirical dominators)

Priority order by string_url_sweep's per-component cost:

1. **String.prototype.toLowerCase** — ASCII fast-path (return-self if already lowercase; in-place lower otherwise). High frequency in header normalization.
2. **String.prototype.trim** — byte-scan for whitespace; return slice or self if no trim needed. High frequency in header normalization.
3. **String.prototype.indexOf** (arity 1 + 2) — already in JIT's HI but interp-tier IC bypasses for ALL call sites (not just OSR-eligible).
4. **String.prototype.slice** — substring extraction; needs allocation discipline (mirrors HI's Finding HI.2 charAt issue).
5. **String.prototype.charCodeAt** — already ad-hoc in interp.rs CharCode-EXT 2; **migrate into the table** as the anchoring entry (same shape as HI-EXT 2's behavior-neutral migration of OSR-EXT 6/6b's ad-hoc paths).

Subsequent entries (after first cut): String.startsWith, String.endsWith, String.includes, String.padStart, String.padEnd, String.split (1-arg form), Array.prototype.push/pop (high frequency in hot loops too).

### I.2 Constraints (Pin-Art enumeration; mirrors HI seed §I.2 with interp-tier adaptations)

```
C1. Existing Σ/Τ/Ψ/Φ/VD/TL/OSR/HI default-on paths byte-identical
    post-IHI (the IC fires only for the exact recognized shape; bails
    on any deviation).
C2. Each table entry correctness-preserving on its own (canonical
    fuzz + diff-prod + per-entry test).
C3. Table entries independent: adding entry N+1 doesn't affect 1..N.
C4. ≤30-50 LOC per entry after infrastructure (Pred-ihi.1; mirrors HI).
C5. Rule 11 5-axis pre-spawn check per entry (component A/B + op-set
    + value-domain + locals-marshaling + emission-shape; for interp-
    tier, locals-marshaling and emission-shape usually trivially pass).
C6. Override-safety gate per entry: cache the intrinsic ObjectId at
    Runtime init; check at IC fast-path; bail to slow path on user
    override. The CharCode-EXT 2 precedent already established the
    pattern via `intrinsic_string_charcodeat_id`.
C7. Composition with the existing CharCode-EXT 2 ad-hoc path:
    migrate charCodeAt into the table; remove the ad-hoc check.

Architecture induced: an IcEntry-style table in rusty-js-runtime
that Op::CallMethod's handler consults BEFORE call_function. The
table covers (method-name, receiver-Value-kind, arity, fast-path-fn,
intrinsic-ObjectId-cache).
```

### I.3 Falsifiers

**Pred-ihi.1**: per-entry LOC ≤50 (mirrors HI's Pred-hi.1). Falsifier: any entry costs >50 LOC.

**Pred-ihi.2**: canonical fuzz remains byte-identical (acc=-932188103) after each entry lands. Falsifier: divergence → (P2.c) illegal-speed bug.

**Pred-ihi.3**: diff-prod 42/42 holds after each entry lands. Falsifier: any regression.

**Pred-ihi.4**: composition with all existing defaults holds; CRB + A/B + JIT bench stay within ±5% per entry-landing. Falsifier: regression → entry broke existing paths.

**Pred-ihi.5**: header normalization loop (string_url_sweep A/B probe variant V2 - V1 delta) drops by ≥30% after toLowerCase + trim + indexOf entries land. Falsifier: <30% reclaim → entries' fast-paths aren't covering the actual hot sites (e.g., toLowerCase always allocates because input isn't already-lowercase; the ASCII fast-path's return-self optimization is rare).

## II. Apparatus

- **Table location**: new `pilots/rusty-js-runtime/derived/src/interp_ic_table.rs` (or inline in interp.rs near Op::CallMethod handler at line 8007+). Decision at IHI-EXT 1 design.
- **Per-entry shape**: IcEntry struct adapted from HI's JIT-tier shape; replace `extern_ptr` + `lower fn` with a direct Rust fn pointer that takes `(&mut Runtime, &Value receiver, &[Value] args) -> Option<Result<Value, RuntimeError>>` (None = bail to slow path).
- **Dispatch integration**: Op::CallMethod handler at interp.rs:8007 consults the table BEFORE the call_function slow path. Lookup by method_name (already captured at pending_method_name) + receiver kind + arity.
- **Override-safety**: per-entry `intrinsic_X_id: Option<ObjectId>` cache on Runtime; populated lazily at first eligible call; bail on mismatch.
- **Bench instruments**: `pilots/cross-runtime-bench/fixtures/string_url_sweep/component-ab-probe.mjs` (re-run each round); CRB string_url_sweep at IHI-EXT N close.
- **Correctness instruments** (rule 5 + rule 10 + rule 11): canonical fuzz + diff-prod + per-entry test at each round.

## III. Methodology

1. **IHI-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **IHI-EXT 1** — design doc: per-entry shape; dispatch integration in Op::CallMethod; override-safety gate; per-entry LOC estimates for the starter set. Output: `docs/design.md`.
3. **IHI-EXT 2** — infrastructure round: IcEntry types + IC_TABLE registry + Op::CallMethod dispatch integration + charCodeAt migration from CharCode-EXT 2 ad-hoc. Substrate-introduction.
4. **IHI-EXT 3-N** — per-entry rounds: toLowerCase, trim, indexOf (arity 1+2), slice, etc.
5. **IHI-EXT N+1** — composition probe + final disposition + Pred-ihi.* booking.

## IV. Carve-outs and bounded scope

- Interp-tier only; JIT-tier is HI's domain.
- Per-entry override-safety gate uses ObjectId cache (per CharCode-EXT 2's precedent).
- Starter set ≤5 entries first cut; broader generalization in follow-on rounds.
- Aarch64 only.
- No Array intrinsics in first cut (Array.push + Array.pop deferred; mirrors HI's Array entry deferral).

## V. Standing artefacts

- `pilots/interp-hot-intrinsics/seed.md`, `trajectory.md`
- `pilots/interp-hot-intrinsics/docs/design.md` (IHI-EXT 1)
- `pilots/interp-hot-intrinsics/fixtures/` for per-entry synthetic fixtures
- Implementation lands in `pilots/rusty-js-runtime/derived/src/interp.rs` + possibly a new `interp_ic_table.rs` module

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read HI's seed.md + docs/design.md for the JIT-tier dual's shape (this pilot mirrors structure with interp-tier adaptations). Read CharCode-EXT 2 (interp.rs ~8073) for the ad-hoc precedent that the table generalizes. Read the string_url_sweep component A/B probe + its readout for empirical context (header normalization = 332 ms, 77% of cruft).
