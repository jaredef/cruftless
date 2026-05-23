# rusty-js-jit/value-domain — Trajectory

Per-VD-EXT log for the Φ-encoding extension pilot (closes the value-domain coverage tier per Doc 740 §II.2 + Finding VII.3).

---

## VD-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 21:51-local as the (α) Φ-encoding extension pivot from TL locale's (b-narrow) chapter close. Nested under LeJIT per Doc 737 §IV.

### Trigger

- TL findings.md Finding TL.2 (engagement-promoted as VII.3 at findings.md Addendum V): Φ calling convention encodes only Number + Object; non-Number/Object Values degrade to 0.0.
- TL pilot's (b-narrow) Moves 3+4 structurally blocked at the encoding tier. Pivot to (b-architectural).
- Two co-equal architectural targets surfaced; keeper selected (α) Φ-encoding extension as the load-bearing prerequisite tier for any future Value-non-Number JIT-IC work.

### Substrate delivered

- `seed.md` (~120 lines): telos, 8 constraints C1-C8, 5 falsifiers Pred-vd.1-.5, methodology VD-EXT 0-7, carve-outs.
- `trajectory.md` (this file).
- `docs/` + `fixtures/` scaffolds.

### Locale registration

Locale count: 21 → 22 after this spawn (13 top-level unchanged; 8 → 9 nested under LeJIT). Manifest refresh queued at end of VD-EXT 0.

### Open scope at VD-EXT 0 close

1. **VD-EXT 1** — encoding design doc (NaN-boxing scheme; bit layout; tag values; encoder + decoder reference)
2. **VD-EXT 2** — encoding implementation (extend unbox_arg_f64 + add box_to_value)
3. **VD-EXT 3** — composition probe + fuzz + diff-prod gate
4. **VD-EXT 4-7** — follow-on Value variants + default-on confirmation

### Cumulative status

LOC delta: 0 (apparatus round only).

---

*VD-EXT 0 closes. Pilot founded as the (α) Φ-encoding extension. VD-EXT 1 designs the NaN-boxing scheme.*

---

## VD-EXT 1 — 2026-05-23 (NaN-boxing scheme design)

### Headline

Design-tier round. `docs/design.md` (~250 lines) specifies the NaN-boxing scheme using sign-bit-set distinguishing pattern: mask `0xFFF0_0000_0000_0000`, 4-bit tag at bits 51-48, 48-bit payload below. String encoding via VD_TAG_STRING=2 + Rc<String> raw pointer. Number + Object encodings preserved byte-identical per C2/C3. Encoder + decoder + round-trip tests + 6 named risks.

### The encoding (one-line summary)

```
encoded = 0xFFF0_0000_0000_0000 | (tag << 48) | (payload & 0xFFFF_FFFF_FFFF)
```

- Sign bit = 1 distinguishes boxed values from real Numbers (real arithmetic NaNs have sign=0)
- 4-bit tag (16 possible types; 7 used at first cut)
- 48-bit payload (Rc<String> raw pointer on aarch64; or 0/1 for Boolean; or 0 for Null/Undefined)

Number stays unboxed (any f64 with sign=0 OR an arithmetic NaN). NaN canonicalization at unbox closes the edge case of hardware-produced sign=1 NaNs.

### Backwards compat preservation

Object encoding (`f64::from_bits(id.0 as u64)`) UNCHANGED at first cut per C3. The latent unsoundness (ObjectId.0 ≥ 2^52 would alias boxed-NaN range) documented as R1; out of scope per Pred-vd.4.

The first-cut decoder is implicit at the consumer site: a consumer expecting String bitcasts f64→I64 + masks high 16 bits → `*const String`. A consumer expecting Object bitcasts directly (unchanged). General `box_to_value` deferred — not needed at first cut.

### 6 named risks

R1 Object encoding shadowing at high ObjectId.0; R2 Rc pointer stability (mitigated by caller-frame lifetime); R3 Rc strong-count semantics at decode (use `&*ptr` not `Rc::from_raw`); R4 endianness (aarch64 little-endian only); R5 Cranelift bitcast semantics (verified via fuzz); R6 NaN canonicalization performance (Pred-vd.5 measures; switch to assume-no-sign-NaN if regression > 5%).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier).
Per Doc 734 §V: growth (c) preparatory — design's encoding scheme anchors VD-EXT 2 implementation.
Per Doc 735 §X.h.c: three-probe-levels applied at VD-EXT 2.

### Composition with prior corpus / engagement work

- **Doc 740 §II.2**: this pilot closes the value-domain coverage tier in R for any non-Number-receiver JIT-IC pilot.
- **Finding VII.3 (Addendum V)**: this pilot's design is the apparatus implementation of the value-domain-coverage check rule 11 extension.
- **Φ §I.2 constraint enumeration**: discipline reapplied here at the encoding tier.
- **Doc 731 §XIV.d alphabet purity**: NaN-boxing extends the "alphabet" the JIT's calling convention can carry; consistent with Doc 731's framing that "the calling convention IS the alphabet."

### Open scope at VD-EXT 1 close

1. **VD-EXT 2** — encoding implementation (extend unbox_arg_f64 + helpers + unit tests)
2. **VD-EXT 3** — composition probe + fuzz + diff-prod gate
3. **VD-EXT 4-7** — follow-on Value variants (BigInt, Boolean, Null, Undefined, Symbol) + default-on confirmation

### Cumulative status at VD-EXT 1 close

LOC delta: ~250 (design doc). Encoding scheme + helpers + tests fully specified; risks named with mitigations.

---

*VD-EXT 1 closes. NaN-boxing design specified. VD-EXT 2 implements unbox_arg_f64 extension + helpers + unit tests.*
