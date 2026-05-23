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

---

## VD-EXT 2 — 2026-05-23 (NaN-boxing implementation + design correction)

### Headline

NaN-boxing implementation landed in interp.rs (~140 LOC including 4 unit tests). String encoding via `unbox_arg_f64(Value::String(s)) = NaN-boxed (Rc::as_ptr(s) as u64) with VD_TAG_STRING=2`. Helpers: `is_boxed_value`, `extract_boxed_tag`, `extract_boxed_payload`, `decode_string_ptr`. All four probes GREEN.

**Design correction surfaced via unit-test failure**: `f64::NEG_INFINITY` has bits exactly `0xFFF0_0000_0000_0000` — collides with the boxed-NaN mask + tag=0 + payload=0. The VD-EXT 1 design's `is_boxed_value` would have mis-detected -∞ as boxed.

**Fix**: tag=0 reserved as "Number escape." is_boxed requires both (a) high-12-bits match mask AND (b) tag ≠ 0. Effective tag space shrinks from 16 to 15 (tags 1-15); no information loss (first-cut uses tag=2 only; 6 more variants queued for VD-EXT 4+).

### Four-probe results

| probe | result |
|---|---|
| Pred-vd.1 String round-trip | ✅ GREEN (4/4 unit tests pass) |
| Pred-vd.2 canonical fuzz (acc=-932188103) | ✅ GREEN |
| Pred-vd.3 diff-prod 42/42 | ✅ GREEN |
| Pred-vd.5 composition (Σ/Τ/Ψ/Φ defaults) | ✅ GREEN — A/B probe 1515-1526 median vs baseline 1480-1507; within ±2% noise |
| JIT lib tests | ✅ 38/38 (9 pre-existing ignored) |

### Substrate moves landed

1. Added constants: VD_BOXED_MASK (0xFFF0_..), VD_TAG_SHIFT (48), VD_PAYLOAD_MASK (0x..FFFF), VD_TAG_STRING (2).
2. Extended unbox_arg_f64 with NaN canonicalization + String NaN-box encoding; preserved Number + Object paths byte-identical.
3. Added 4 decoder helpers per design §3.3.
4. Added 4 unit tests covering Number-preserve / Object-preserve / String round-trip / collision-free.
5. **Design correction (in-round)**: is_boxed_value gated on tag ≠ 0 after -∞ collision surfaced.

### Lesson generated (candidate Finding TL.3 / VD.1)

**Finding VD.1 (NaN-boxing schemes that use sign=1 alone collide with -∞)**: any NaN-boxing scheme using mask `0xFFF0_0000_0000_0000` (sign=1 + exp=all-1) MUST exclude tag=0 to preserve -∞ as a Number. The design's "sign=1 distinguishes boxed from real Numbers" framing was incomplete; -∞ has sign=1 and the same exp pattern but is a valid Number. Tag=0 must be reserved as the "Number escape" value.

**How to apply**: any future NaN-boxing extension (BigInt, Boolean, etc. at VD-EXT 4+) MUST use tags ≥ 1; tag=0 stays reserved for -∞ preservation. Documented in interp.rs near the constants.

**Process lesson (engagement-tier)**: unit tests at substrate-introduction rounds catch design errors that the design-doc reasoning misses. The VD-EXT 1 design enumerated 6 risks; -∞ collision was NOT named (R5 came closest, on Cranelift bitcast semantics, but the -∞ shape is more fundamental). **Standing rule candidate**: NaN-boxing or similar bit-pattern schemes require an adversarial unit-test pass covering all IEEE 754 special values (±0, ±∞, ±NaN, subnormals, MIN_POSITIVE) before design closure. Promotion candidate at VD-EXT 3.

### Composition with prior corpus / engagement work

- **Doc 740 §II.2 value-domain coverage tier**: this round delivers the substrate at that tier.
- **Finding VII.3 (Addendum V)**: this implementation is the apparatus closure for the value-domain-coverage check.
- **Finding II.2-bis substrate-introduction signature**: this round's bench is flat as expected; downstream consumer-pilots deliver the cumulative reclaim.
- **Φ §I.2 constraint enumeration discipline**: reused here; the in-round design correction (tag=0 escape) extends C2's "Number byte-identical" to include -∞.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (NaN-boxing landing) + growth (b) negative-finding-catalyzes-refinement (-∞ correction + new Finding VD.1).
Per Doc 735 §X.h.b: **(P2.d) bench at substrate-introduction round, expected per Finding II.2-bis. The encoding is now available to consumer pilots; pipeline-connection point is downstream.**

### Open scope at VD-EXT 2 close

1. **VD-EXT 3** — final composition probe + formal close of first-cut encoding work.
2. **VD-EXT 4-7** — follow-on Value variants (BigInt, Boolean, Null, Undefined, Symbol) per scope discipline.
3. **TL pilot revival** (post-VD substrate): TL Moves 3+4 can now structurally deliver fast paths consuming the String encoding.
4. **Engagement-wide hot-intrinsic-IC table** at JIT tier: generalization candidate consuming the encoding.
5. **Findings doc Addendum VI candidate**: Finding VD.1 (NaN-boxing tag=0 reservation) + standing rule for adversarial IEEE 754 special-value testing.

### Cumulative status at VD-EXT 2 close

LOC delta: ~140 (interp.rs: 4 constants + extended unbox_arg_f64 + 4 helpers + 4 unit tests + comment block). VD-EXT 0+1+2 cumulative: ~410 across the locale (seed + design + impl). All four probes GREEN.

---

*VD-EXT 2 closes. NaN-boxing implementation landed; -∞ design correction in-round; Finding VD.1 generated. Substrate-introduction value at the value-domain tier delivered. Consumer-pilot revival becomes structurally possible. VD-EXT 3 formal close + potentially Findings Addendum VI.*

---

## VD-EXT 3 — 2026-05-23 (formal close + Findings Addendum VI codification)

### Headline

Final round of the VD pilot's first cut. Findings doc Addendum VI promotes Finding VD.1 to engagement-scope as Finding VIII.1 (NaN-boxing tag=0 reservation) + introduces standing rule 12 (adversarial special-value test discipline for bit-pattern schemes). No source changes this round; documentation + categorization only.

### Engagement findings codified at Addendum VI

- **Finding VIII.1**: NaN-boxing schemes using sign=1 + exp=all-1 mask MUST reserve tag=0 to preserve -∞ as Number. Future tag assignments (BigInt, Boolean, Symbol at VD-EXT 4+) MUST use tags ≥ 1.
- **Standing rule 12**: any pilot introducing a bit-pattern-tagging scheme over a special-value-bearing type MUST include an adversarial unit-test pass covering ALL special values before design closure. For IEEE 754 doubles: ±0, ±∞, ±NaN (canonical + signaling + sign=1), MIN_POSITIVE, EPSILON, subnormals, MAX, π, e, common values.

### VD locale disposition

**VD first cut closed at (P2.a) (encoding substrate-introduction successful).** Pilot delivered:

1. **String encoding substrate**: VD_BOXED_MASK + VD_TAG_STRING constants + extended unbox_arg_f64 + 4 decoder helpers. Consumable by any future JIT-tier pilot needing String-receiver identity.
2. **Backwards compatibility**: Number + Object encoding preserved byte-identical (Pred-vd.2 + Pred-vd.5 GREEN; A/B probe 1515-1526 vs baseline 1480-1507, within ±2% noise).
3. **Finding VIII.1**: engagement-wide structural property captured.
4. **Standing rule 12**: engagement-wide process discipline added.
5. **-∞ correction precedent**: documented in interp.rs near constants + in VD-EXT 2 trajectory entry; any sibling pilot considering NaN-boxing extension sees the precedent first.

### Pred-vd.* falsifier disposition

| falsifier | disposition |
|---|---|
| Pred-vd.1 String round-trip | ✅ HELD (4/4 unit tests) |
| Pred-vd.2 canonical fuzz | ✅ HELD (acc=-932188103) |
| Pred-vd.3 diff-prod | ✅ HELD (42/42) |
| Pred-vd.4 scope discipline | ✅ HELD (String only at first cut; BigInt/Boolean/etc. deferred) |
| Pred-vd.5 composition with defaults | ✅ HELD (within ±2% noise) |

All 5 falsifiers hold. VD first cut is at (P2.a).

### Open scope at VD locale close (first cut)

1. **VD-EXT 4** — BigInt encoding extension (conditional; only land if a downstream consumer needs it; out of scope for first cut per Pred-vd.4)
2. **VD-EXT 5** — Boolean / Null / Undefined encoding extension (conditional)
3. **VD-EXT 6** — Symbol encoding extension (conditional)
4. **VD-EXT 7** — default-on confirmation (the encoding is structural; the Pred-vd.5 composition check at VD-EXT 2 already validated coexistence with Σ/Τ/Ψ/Φ defaults; formal default-on round may be unnecessary)

These are conditional follow-ons. The first cut's substrate-introduction value (String encoding) is consumable now.

### Forward (post-VD locale close)

Per Doc 740 §II.2 P4 multi-tier reading, the next consumer pilot is the natural revival of TL Moves 3+4 — now structurally possible because:

- Value-domain coverage closed (this pilot, VD)
- Entry-mechanism tier closed (TL-EXT 3)
- Op-set coverage still open per Finding VII.2 (TL Move 3 + Move 4 add GetProp + CallMethod alphabet variants; the inner-loop alphabet closure)
- IC fast-path body tier still open per design (Moves 3+4 implementation)

The TL pilot revival is a sibling-pilot to VD; not part of this locale's scope. Spawn decision pending keeper signal.

Alternative consumer-pilot: engagement-wide hot-intrinsic-IC table at JIT tier (consumes VD encoding for hot intrinsic methods across String / Array / etc.). Larger scope; also pending keeper signal.

### Cumulative status at VD locale first-cut close

LOC delta (VD-EXT 0-3): ~410 across seed + design + impl + trajectory entries + findings Addendum VI.
Source LOC: ~140 in interp.rs (constants + extended unbox_arg_f64 + 4 helpers + 4 unit tests).
Engagement findings: Addendum VI codified (Finding VIII.1 + standing rule 12).
All 5 Pred-vd.* falsifiers held.

---

*VD-EXT 3 closes. VD first cut closed at (P2.a). String encoding substrate available for downstream consumer pilots. Findings Addendum VI codified. Pilot's load-bearing contributions: String-receiver JIT-tier substrate + engagement-wide bit-pattern-scheme discipline + -∞-collision precedent. Pivot decision (TL revival vs hot-intrinsic-IC table vs other) pending keeper signal.*

---

## Session chapter close — 2026-05-23 (architectural-pivot session arc summary)

Recorded here at the VD locale (the last active workstream of the session) per keeper directive 2026-05-23 22:31-local. Spans JSF chain + CharCode chain + TL pilot + VD pilot + 2 corpus docs (740 alongside the prior 739) + 3 findings addenda (IV, V, VI). The session's arc is the multi-tier-cascade-revival recognition applied iteratively across actual substrate work.

### Session arc (commit-tag coordinate sequence)

1. **JSF chain (M1-M4)**: substrate-introduction at JSON.stringify tier; flat per-shape bench at first cut; cumulative 1-8% per-shape micro-bench wins. Cumulative CRB: -1% (within noise). Pred-jsf.1 falsified at first cut. Commits db5c5833, 3a6fd70f, 33a86a5d, e1ce4a99.

2. **JSF-EXT 8 component A/B probe**: per Finding VII.1 (rule 11). Identified actual dominator on json_parse_transform: charCodeAt loop at 77% (vs CRB-EXT 9's "JSON.stringify ~5-10× contributor" estimate, off by ~20×). Commit 92dfc87b.

3. **CharCode-EXT 1+2**: substrate ASCII charCodeAt + interp-tier hot-intrinsic IC. Multi-tier cascade-revival empirically demonstrated: substrate-tier closure alone -3%; +interp IC closure -12% cumulative CRB; cruft/node 20.34× → 17.93×. **Pipeline connection point.** Commits b8560a89, 5fdc4998.

4. **Findings Addendum IV + Doc 740 corpus articulation**: Finding II.2-bis (substrate-introduction (P2.d) signature), Finding VII.1 (component A/B before pilot spawn), Finding II.3 (multi-tier cascade-revival), standing rule 11, 2 new engagement instruments. Doc 740 corpus-master + resolve mirror + jaredfoy.com seed pipeline complete. Commits dadac18 (corpus-master), becdb4e (resolve), b27443ec (rusty-bun findings).

5. **TL pilot (b-narrow)**: spawned for LeJIT top-level loop JIT (the 1480ms residual dispatch tier). Closed structurally at TL-EXT 3 (not TL-EXT 5 as designed). Findings TL.1 + TL.2 surfaced two new blockers (whole-body bail; value-domain encoding). Promoted to engagement-wide as Findings VII.2 + VII.3 + standing rule 11 multi-axis extension at Addendum V. Commits 4d0dc240... ecc12da0.

6. **VD pilot (α architectural pivot)**: spawned for Φ-encoding extension. NaN-boxing for String receivers. First cut closed at (P2.a) at VD-EXT 3. Finding VIII.1 (-∞ tag=0 reservation) + standing rule 12 (adversarial special-value tests for bit-pattern schemes) at Addendum VI. Commits 7a067512, e62a280c, 073d6ddc.

### Engagement-tier delta (cumulative across session)

| dimension | start of session | end of session | delta |
|---|---|---|---|
| CRB json_parse_transform | 2481 ms | 2188 ms | **-12%** |
| cruft/node ratio | 20.34× | 17.93× | **-12%** |
| Corpus docs | 739 | 740 | **+1** (multi-tier-cascade-revival) |
| Engagement findings | 9 in 4 addenda | 15 in 6 addenda | **+6** |
| Standing rules | 10 | 12 | **+2** (rule 11 multi-axis + rule 12 adversarial-IEEE-754) |
| Engagement instruments | 0 standing | 2 standing | **+2** (component-A/B probe + hot-intrinsic IC pattern) |
| Active locales | 16 | 22 | **+6** (JSF-chain 4 spawns + TL + VD) |

### Doc 740 multi-tier reading: empirical instantiation

Doc 740's abstract pattern (closing one tier alone is insufficient when the hot path traverses multiple tiers) materialized THREE times in this session at different scales:

- **JSF chain**: JSON.stringify substrate (M1-M4) at the wrong tier per Finding VII.1. R was {charCodeAt loop dispatch, charCodeAt algorithm}; JSON.stringify ∉ R.
- **CharCode chain**: substrate-tier alone (CC-1, -3%) + dispatch-tier alone (would be -15% if measured alone) ; both together -12% cumulative. Multi-tier pipeline-connection materialized.
- **TL + VD pilots**: R for the full TL-equivalent closure has 4 tiers — {value-domain coverage (VD closed), entry-mechanism (TL-EXT 3 closed), op-set coverage (open), IC fast-path body (open)}. Two of four closed in this session; full pipeline-connection requires the remaining two.

### Apparatus refinement

Standing rule 11 now multi-axis:
- A/B probe before pilot spawn (Addendum IV)
- Op-set coverage check before alphabet-pilot spawn (Addendum V Finding VII.2)
- Value-domain coverage check before IC-pilot spawn (Addendum V Finding VII.3)

Standing rule 12 added (Addendum VI): adversarial IEEE 754 special-value test gate for bit-pattern-tagging schemes.

The apparatus self-applied at least 4 times within the session:
- JSF-EXT 8 A/B probe (rule 11 retrospective application; would have prevented JSF mis-spawn)
- TL-EXT 3 source-read enumeration (rule 11 op-set axis prospective application; would have re-scoped (b-narrow))
- VD-EXT 1 design source-read (rule 11 value-domain axis prospective application; named scope correctly)
- VD-EXT 2 -∞ unit-test catch (rule 12 retrospective demonstration; would-have-prevented-blame)

### Session disposition + pivot

**Session closes here per keeper directive 2026-05-23 22:31-local.** Substantial substrate-introduction value delivered + apparatus refinement + corpus articulation. Context budget conserved by closing without immediately launching next consumer-pilot rounds.

**Pivot target identified**: (ii) OSR / loop-extraction pilot. Per Doc 740 §II.2 P4 multi-tier reading, OSR closes the op-set coverage tier (Finding VII.2) by reducing the enclosing scope from whole-module-body to the inner loop. Combined with VD's value-domain closure (this session) + TL's entry-mechanism closure (this session), the full multi-tier pipeline-connection on json_parse_transform becomes feasible at the OSR pilot's close.

OSR pilot to be spawned next per the directive's "report before pivot to ii". Spawn details + scope decisions pending next session's first round.

---

*Session chapter closes at VD-EXT 3. Substrate-introduction work landed at 4 tiers (substrate algorithm via CC-1; interp dispatch via CC-2; entry-mechanism via TL-EXT 3; value-domain via VD-EXT 2). Two tiers remain open (op-set coverage; IC fast-path bodies) — OSR pilot is the (β) pivot target named in TL findings.md TL.1 + Addendum V Finding VII.2 as the structural closure path. -12% CRB cumulative reclaim demonstrates the multi-tier reading empirically. Pre-session-resume protocol: read Doc 740 + findings Addendum V+VI + TL findings.md + this session-close entry for full context.*
