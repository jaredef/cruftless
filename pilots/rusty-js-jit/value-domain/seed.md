# rusty-js-jit/value-domain — Resume Vector / Seed

**Locale tag**: `L.rusty-js-jit.value-domain` (nested under LeJIT per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (VD-EXT 0)**. Spawned per keeper directive 2026-05-23 21:51-local as the (α) Φ-encoding extension pivot from TL locale's chapter-close (Findings VII.2 + VII.3 + standing rule 11 multi-axis extension at engagement findings doc Addendum V). Architectural-tier pilot.

**Workstream**: extend Φ-EXT's f64-default calling convention to encode non-Number / non-Object Value variants. Per Finding VII.3: currently the convention encodes Number → f64 payload, Object → f64::from_bits(id.0 as u64), all others → 0.0. The 0.0 collapse is a structural blocker for any JIT-IC pilot whose target receiver is String / BigInt / Boolean / Symbol / Null / Undefined.

This pilot closes the value-domain coverage tier per Doc 740 §II.2's relevant-tier set R. It is the prerequisite tier for (b-architectural) Moves 3+4 (charCodeAt-IC at JIT tier) and for any future hot-intrinsic-IC table generalization at the JIT tier.

**Author**: 2026-05-23 session.
**Parent**: LeJIT (`pilots/rusty-js-jit/`).
**Sibling**: `top-level/` (TL pilot; closed (b-narrow) chapter; this pilot is its (b-architectural) prerequisite tier).
**Composes with**:
- [Findings doc Addendum V](../findings.md) — Finding VII.3 + standing rule 11 value-domain-coverage axis (the apparatus that named this pilot's scope)
- [Doc 740](../../../../corpus-master/corpus/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) — multi-tier reading; this pilot closes one of four tiers in R for json_parse_transform
- [Doc 731 §XIV.d](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — alphabet purity / "calling convention IS the alphabet"; encoding extension widens the alphabet
- [Φ seed §I.2](../f64-calling-convention/seed.md) — constraint enumeration discipline for f64-default; reused here to constrain the encoding extension
- [TL findings.md TL.2](../top-level/findings.md) — local empirical anchor for the encoding gap
- [TL trajectory chapter close](../top-level/trajectory.md) — substrate-introduction precedent at TL-EXT 3

## I. Telos

**Empirical answer to**: can the Φ calling convention be extended to carry String (and ideally all Value variants) into the JIT body without breaking the existing Σ/Τ/Ψ/Φ default-on pipeline?

The bench-anchored target: post-implementation, json_parse_transform's checksum loop can be (re-attempted as a follow-on TL pilot, post-value-domain) JIT'd with the String receiver intact. This pilot itself produces no direct CRB reclaim — it's prerequisite-tier substrate-introduction (per Doc 740 §II.2 + Finding II.2-bis). Downstream consumer-pilots (TL Moves 3+4 re-attempted; engagement-wide hot-intrinsic-IC table) deliver the cumulative reclaim that closes Pred-jsf.1 / Pred-tl.1.

### I.1 First-cut scope

- **String encoding only at first cut** (Pred-vd.4 scope discipline). String is the load-bearing Value variant for the json_parse_transform fixture's bottleneck; other non-Number/Object variants (BigInt, Boolean, Symbol, Null, Undefined) are deferred to follow-on rounds.
- **NaN-boxing scheme** as the candidate encoding (vs sentinel-bit / vs tagged-union). NaN-boxing is the conventional scheme in production JS engines (V8 SMI tagging; SpiderMonkey NaN-boxing); empirically validated; bit-pattern designable to coexist with existing Number+Object encoding.
- **Backwards-compat preservation**: existing Number / Object encoding bits must remain unchanged under the extended scheme. The Σ/Τ/Ψ/Φ default-on flags continue producing identical bench numbers post-VD.
- **Unbox/box symmetry**: extend unbox_arg_f64 + add a box_to_value helper for use at JIT body return + extern-call-site marshaling.

Out of scope (deferred):
- BigInt encoding (defer to VD-EXT N+1)
- Boolean / Symbol / Null / Undefined encoding (defer; Number-tag covers Boolean indirectly via 0.0/1.0 but explicit encoding adds robustness)
- IC fast-paths at JIT tier consuming String identity (that's TL pilot revival; this pilot delivers only the encoding)
- Performance optimization of NaN-boxing unbox cost (first-cut: correctness + zero-impact-on-Number path)

### I.2 Constraints (Pin-Art enumeration per Φ §I.2 discipline)

```
C1. Existing Σ/Τ/Ψ/Φ default-on paths produce byte-identical bench
    numbers post-VD (Pred-vd.5 composition gate).
C2. unbox_arg_f64(Value::Number(n)) === n preserved exactly (no
    NaN-boxing escape for non-NaN Numbers).
C3. unbox_arg_f64(Value::Object(id)) === f64::from_bits(id.0 as u64)
    preserved exactly (existing dispatch path).
C4. String encoding via NaN-boxing pattern that doesn't collide with
    any IEEE 754 valid double (signaling NaN payload space; ~52 bits
    available for pointer + 4-bit type tag).
C5. box_to_value(unbox_arg_f64(v)) === v for any v ∈ {Number, Object,
    String, Boolean, Null, Undefined} (round-trip identity).
C6. Canonical fuzz acc=-932188103 byte-identical throughout.
C7. diff-prod 42/42 throughout.
C8. JIT lib tests 38/38 throughout (existing 9 ignored remain ignored).

Architecture induced by C1-C8: NaN-boxing scheme with 4-bit type tag
in the top of the mantissa, 48-bit payload below. Number path
unchanged (real doubles + their NaN representations are distinct from
the boxed-NaN tag). Object path migrates to the boxed-NaN scheme
(adds tag verification at the receiver-extract site; old code
continues to decode correctly because Object's f64::from_bits encoding
happens to land in the boxed-NaN space already).
```

### I.3 Falsifiers

**Pred-vd.1**: post-implementation, `unbox_arg_f64(Value::String(Rc::new("abc")))` returns a non-zero f64 whose bits decode back to the same Rc<String> via `box_to_value`. Falsifier: 0.0 return → encoding didn't land.

**Pred-vd.2**: canonical fuzz (CMig-EXT 17 fuzz-canonical.mjs) remains byte-identical post-implementation (acc=-932188103 vs node). Falsifier: divergence → (P2.c) illegal-speed bug at encoding boundary.

**Pred-vd.3**: diff-prod 42/42 holds. Falsifier: any regression.

**Pred-vd.4**: scope discipline — only String encoding lands at first cut. BigInt / Boolean / Symbol / Null / Undefined encodings are deferred. Falsifier: any other Value variant added beyond String in this pilot's rounds.

**Pred-vd.5**: composition with Σ/Τ/Ψ/Φ default-on holds. bench_call_overhead + bench_ic per-iter latencies stay within ±5% of post-Φ baselines. Falsifier: regression → encoding extension broke existing JIT paths.

## II. Apparatus

- **Encoding design** (`docs/design.md`): VD-EXT 1 enumerates the NaN-boxing scheme; bit-layout; tag values; encoder + decoder reference implementations.
- **Runtime** (`pilots/rusty-js-runtime/derived/src/interp.rs`): extend unbox_arg_f64 + add box_to_value helper.
- **GC** (`pilots/rusty-js-gc/`): Strings are GC roots via Rc; the encoding holds raw Rc pointer bits. Need to verify the Rc isn't dropped while a JIT body holds the encoded f64. (Likely safe because the source Value::String lives in the caller's stack frame for the JIT call's duration; same shape as Object's id-encoding which is GC-stable by ID.)
- **JIT** (`pilots/rusty-js-jit/derived/src/translator.rs`): no immediate change required for encoding-only pilot. Downstream pilots that consume String identity at the JIT body call site will add the decode-receiver-tag logic.
- **Correctness instruments** (rule 5 + rule 10): canonical fuzz + diff-prod + JIT lib tests at each round.

## III. Methodology

1. **VD-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **VD-EXT 1** — encoding design doc: NaN-boxing bit layout; tag values; encoder + decoder reference. Source-read Value enum + existing unbox_arg_f64. Output: `docs/design.md`.
3. **VD-EXT 2** — encoding implementation: extend unbox_arg_f64 for String → boxed-NaN; add box_to_value; preserve Number + Object paths byte-identically.
4. **VD-EXT 3** — composition probe + fuzz + diff-prod gate (substrate-introduction signature expected).
5. **VD-EXT 4** — BigInt encoding extension (conditional on keeper signal; otherwise deferred to follow-on locale).
6. **VD-EXT 5** — Boolean / Null / Undefined encoding extension (conditional).
7. **VD-EXT 6** — Symbol encoding extension (conditional).
8. **VD-EXT 7** — default-on confirmation (the encoding extension is structural; no flag-flip needed — but the round verifies the encoding's coexistence with all Σ/Τ/Ψ/Φ defaults).

## IV. Carve-outs and bounded scope

- String encoding only at first cut (VD-EXT 1-3).
- NaN-boxing scheme (vs sentinel-bit / tagged-union — design-doc decision at VD-EXT 1).
- No JIT-tier consumer (downstream pilots — TL Moves 3+4 revival; hot-intrinsic-IC table — consume the encoding).
- No GC change (relies on caller-side Value lifetime + Rc stability).
- Aarch64 only (engagement reference).

## V. Standing artefacts

- `pilots/rusty-js-jit/value-domain/seed.md`, `trajectory.md`
- `pilots/rusty-js-jit/value-domain/docs/design.md` (VD-EXT 1)
- `pilots/rusty-js-jit/value-domain/fixtures/` for pilot-specific encoding tests
- Implementation lands in `pilots/rusty-js-runtime/derived/src/interp.rs`

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read Doc 740 §II.4 + findings.md Addendum V (Findings VII.2 + VII.3 + standing rule 11 extension) for the apparatus that named this pilot's scope. Read TL findings.md TL.2 + TL trajectory chapter-close for empirical anchor. Read Φ seed §I.2 for constraint enumeration discipline reused here.
