# rusty-js-jit/hot-intrinsics — Resume Vector / Seed

**Locale tag**: `L.rusty-js-jit.hot-intrinsics` (nested under LeJIT per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (HI-EXT 0)**. Engagement-wide instrument spawned per keeper directive 2026-05-23 02:39-local (continuation after the architectural-pivot session's pipeline-connection at OSR-EXT 6b). The instrument generalizes the OSR-EXT 6b + CharCode-EXT 2 hot-intrinsic-IC pattern (already validated on charCodeAt) into a multi-intrinsic TABLE — a reusable apparatus that future pilots extend with new intrinsics.

**Workstream**: build a hot-intrinsic-IC table that JIT-tier IR can consume to inline-fast-path the most-frequently-called intrinsic methods. Each table entry names: (method-key, receiver-Value-kind, arity, fast-path-extern, IR-lowering-shape, correctness-gate). The table's first cut covers a starter set; future pilots add entries.

**Author**: 2026-05-23 session.
**Parent**: LeJIT (`pilots/rusty-js-jit/`).
**Siblings**:
- `osr/` (validated charCodeAt + length at OSR-EXT 6b; first instance of the pattern at JIT tier)
- `value-domain/` (VD encoding consumed by IC fast-paths for non-Number/Object receivers)
- `top-level/` (TL pattern reused for non-OSR JIT consumers)

**Composes with**:
- [Doc 741](../../../../corpus-master/corpus/741-the-multi-tier-cascade-pipeline-connects-an-empirical-materialization-of-doc-740-across-four-sibling-pilots-on-a-cruftless-cross-runtime-bench-fixture.md) §V.1 (the pattern generalizes; this pilot is the engagement-tier instrument)
- [Doc 740 §VIII](../../../../corpus-master/corpus/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) (coverage axes; this pilot operates within axes A2 + A3, validated at OSR-EXT 6b)
- [Findings Addendum VII + VIII](../findings.md) — standing rule 11 multi-axis form; hot-intrinsic IC pattern as standing instrument
- [OSR-EXT 6b](../osr/trajectory.md) — empirical anchor: charCodeAt-IC fired with -66% CRB reclaim cumulative
- [VD pilot](../value-domain/seed.md) — String NaN-boxing consumed in non-Number receiver paths
- [CharCode-EXT 2](../../rusty-js-runtime/derived/src/interp.rs) — interp-tier IC pattern (the cross-tier analog; this pilot is the JIT-tier table)

## I. Telos

**Empirical answer to**: can a generalized hot-intrinsic-IC table at JIT tier cover N intrinsic methods with a uniform apparatus — table-entry registration, extern declaration, IR lowering — so that adding a new intrinsic is bounded LOC per entry rather than full substrate work?

The bench-anchored target: post-implementation, additional intrinsics beyond charCodeAt (e.g., charAt, codePointAt, indexOf, slice, push, length-on-Array, etc.) JIT-compile in OSR loop bodies with ~50 LOC per entry (vs ~150 LOC per intrinsic ad-hoc per OSR-EXT 6b's first-cut shape).

### I.1 Starter set (per Doc 741 §IV + OSR-EXT 6b precedent)

The candidate first-cut starter set, in priority order by realistic workload frequency:

1. **String.prototype.charCodeAt** — already validated at OSR-EXT 6b; serves as the table's anchoring entry
2. **String.prototype.length** (property access) — already validated at OSR-EXT 6 as GetPropLength
3. **String.prototype.charAt** — similar shape to charCodeAt
4. **String.prototype.codePointAt** — handles surrogate pairs
5. **Array.prototype.length** (property access; high-frequency)
6. **Array.prototype.push** (single arg, mutating; common in loops)

Subsequent rounds add: Array.prototype.indexOf, Array.prototype.includes, Array.prototype.slice, String.prototype.slice, Object.prototype.hasOwnProperty, Number.prototype.toString, Math.floor (no-receiver call), etc.

### I.2 Constraints (Pin-Art enumeration)

```
C1. Existing Σ/Τ/Ψ/Φ/VD/TL/OSR default-on paths produce byte-identical
    bench numbers post-HI.
C2. Each table entry is correctness-preserving on its own (canonical
    fuzz + diff-prod + per-entry adversarial fixture).
C3. Table entries are independent: adding entry N+1 doesn't affect
    entries 1..N's behavior or bench.
C4. The apparatus is LOC-bounded per entry: ~30-50 LOC per entry after
    the table infrastructure lands.
C5. Pre-spawn check (rule 11): for each entry, verify (A2 op-set) +
    (A3 value-domain via VD) + (A4 if non-arg state) + (A5 if region-
    extraction) coverage before adding to the table.
C6. Override-safety: each entry has a runtime gate that bails to interp
    if user code overrode the intrinsic method (e.g., String.prototype.
    charCodeAt = ...). The gate's overhead is bounded.
C7. Composition with OSR-EXT 6b: the existing GetPropCharCodeAt +
    CallMethodCharCodeAt ParsedOps fold into the table apparatus
    OR remain as the anchoring instance + table covers the rest.

Architecture induced: a registration-based table where each entry
provides (key, arity, receiver-kind, fast-path-extern, IR-lowering-fn).
The translator iterates entries at parse-time + emits the matched
entry's IR at translate-time. Bounded per-entry LOC + uniform
correctness gate.
```

### I.3 Falsifiers (provisional; refined at HI-EXT 1)

**Pred-hi.1**: post-implementation, adding intrinsic N+1 to the table costs ≤50 LOC. Falsifier: any entry beyond the initial infrastructure round costs >50 LOC.

**Pred-hi.2**: canonical fuzz remains byte-identical (acc=-932188103) after each table entry lands. Falsifier: divergence → (P2.c) illegal-speed bug.

**Pred-hi.3**: diff-prod 42/42 holds after each entry lands. Falsifier: any regression.

**Pred-hi.4**: composition with existing defaults holds; bench_call_overhead + bench_ic + A/B probe + CRB stay within ±5% per entry-landing. Falsifier: regression → table entry broke existing paths.

**Pred-hi.5**: starter-set entries (per §I.1) deliver bench-measurable speedup on a per-entry synthetic fixture that exercises the intrinsic in a hot loop. Falsifier: <5% speedup on the synthetic fixture → the entry's IR is no faster than the existing path.

## II. Apparatus

- **Table registration**: a Rust data structure in JIT crate's translator.rs (or a new helpers/ic_table.rs) holding the entries. Per-entry struct: `IcEntry { key: &'static str, arity: u8, receiver_kind: ReceiverKind, lower: fn(...) }`.
- **Parse-time match**: parse_bytecode's Op::GetProp + Op::CallMethod arms consult the table for accepted (key, arity, receiver-kind) combos.
- **Extern registration**: at JIT module setup, iterate table entries; JITBuilder::symbol per entry's extern; declare_function per entry's signature.
- **IR emission**: at translate-time, lookup the entry by ParsedOp's encoded table-index; call the entry's lower fn.
- **Correctness instruments** (rule 5 + rule 10): canonical fuzz + diff-prod + per-entry synthetic fixture at each round.

## III. Methodology

1. **HI-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **HI-EXT 1** — design doc: enumerate the IcEntry struct + registration shape + parse-table-lookup + IR-lowering-dispatch + per-entry LOC estimates for the starter set. Output: `docs/design.md`.
3. **HI-EXT 2** — infrastructure round: IcEntry struct + table registry + parse-table dispatch + translator integration. Substrate-introduction; charCodeAt + length entries migrated from OSR-EXT 6/6b's ad-hoc form into the table.
4. **HI-EXT 3-N** — per-entry additions: each round adds 1-2 entries with three-probe gate.
5. **HI-EXT N+1** — composition probe + final disposition + Pred-hi.* booking.

## IV. Carve-outs and bounded scope

- JIT-tier IC only; interp-tier IC table (a separate engagement-wide instrument) is deferred.
- Per-entry override-safety gate uses a simple flag (e.g., per-entry AtomicBool set to false when an override is detected). Sophisticated invalidation strategies deferred.
- Starter set ≤6 entries; broader generalization is follow-on locales' scope.
- Aarch64 only.

## V. Standing artefacts

- `pilots/rusty-js-jit/hot-intrinsics/seed.md`, `trajectory.md`
- `pilots/rusty-js-jit/hot-intrinsics/docs/design.md` (HI-EXT 1)
- `pilots/rusty-js-jit/hot-intrinsics/fixtures/` for per-entry synthetic fixtures
- Implementation lands in `pilots/rusty-js-jit/derived/src/translator.rs` + possibly a new helpers/ic_table.rs

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read Doc 741 §IV-V for the apparatus context. Read OSR-EXT 6b trajectory for the empirical anchor (charCodeAt-IC works; pattern proven). Read VD-EXT 1 design.md for the NaN-boxing scheme that non-Number entries consume.
