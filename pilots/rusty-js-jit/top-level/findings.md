# rusty-js-jit/top-level — Local Findings

Per Doc 737 §IV nested locale convention. Locale-scoped findings; promotions to parent (LeJIT) findings.md or to engagement-wide findings (corpus Doc 740+) noted explicitly when they occur.

---

## Finding TL.1 (Whole-body-or-nothing JIT discipline bounds the (b-narrow) reclaim ceiling for top-level fixtures with mixed alphabet) *[new, 2026-05-23 via TL-EXT 3 empirical readout]*

**Anchor**: TL-EXT 3 landed the module-body JIT entry wrapper. Empirical readout on json_parse_transform: the wrapper attempts compile, bails at parse-time on the top-level body's MakeClosure / Op::Call / Op::CallMethod / LoadGlobal ops per C8 (whole-body-or-nothing bail discipline). Falls through to interp cleanly. A/B probe flat as predicted.

The structural implication: the JIT's current bail discipline is whole-function (or whole-module after Move 2). If ANY op in the body falls outside the alphabet, the JIT cannot compile the body at all, regardless of how hot the inner-loop sub-region is. For json_parse_transform's top-level body (which has many non-loop ops: `const payload = makePayload(N)`, `const text = JSON.stringify(payload)`, the `for (let iter = 0; iter < ITER; iter++) { ... }` outer iter, etc.), the inner charCodeAt loop's eligibility is gated on EVERY other op in the body also being JIT-eligible.

**The (b-narrow) plan's reclaim ceiling**:
- (b-narrow) closes 3 alphabet gaps (PushConst Number, GetProp length-IC, CallMethod charCodeAt-IC) per TL-EXT 1 design.
- json_parse_transform top-level body uses ≥10 other ops outside (b-narrow)'s scope (MakeClosure for makePayload; LoadGlobal for JSON; Op::Call for JSON.parse / JSON.stringify; etc.).
- Cumulative reclaim ceiling on json_parse_transform: ~0% via (b-narrow) alone. The pipeline DOES NOT connect.

**Substrate implication**: the (b-narrow) plan's pre-implementation reclaim projection (40-60% at TL-EXT 5) assumed that closing the inner-loop alphabet would let the loop fire. The whole-body bail discipline falsifies that assumption for any fixture whose top-level body has alphabet gaps beyond the inner loop. **(b-narrow) is structurally insufficient for json_parse_transform**.

The reclaim is bounded above by 0% on this fixture unless:
- (b-medium): close MORE alphabet (LoadGlobal/StoreGlobal/Op::Call/MakeClosure/etc.) so the whole top-level body becomes JIT-eligible. ~10+ rounds; very large scope.
- (b-architectural): OSR or loop-extraction so the inner loop can JIT independently of the surrounding body's alphabet coverage. Architecture-tier change; ~10+ rounds.
- (b-different-fixture): pick a fixture whose top-level body IS in the (b-narrow) alphabet (e.g., a synthetic tight-loop benchmark). Closes (b-narrow) on the synthetic; doesn't close json_parse_transform.

**How to apply**: at any pilot whose telos is "extend JIT alphabet to close a measured bottleneck," the source-read enumeration MUST include the FULL top-level body's op set (not just the inner-loop ops). The bail discipline is whole-body, so the alphabet coverage requirement is whole-body. If the alphabet coverage required is larger than the pilot's scope, the pilot's reclaim ceiling on that fixture is 0% via that pilot alone.

**Generalization to engagement-tier reading**: per Doc 740 §II.4 component A/B probe (standing rule 11), the discipline catches component mis-attribution. **This finding extends rule 11 with an op-set-coverage check**: before spawning a JIT-alphabet pilot, source-read the FULL bytecode of the target fixture's hot-path enclosing scope (whole module for top-level; whole function for function-body); enumerate the op set; verify the pilot's alphabet additions cover ALL ops in scope. If not, the pilot's reclaim ceiling on that fixture is 0% via that pilot alone.

**Candidate generalization**: extend Doc 740's §II.4 + standing rule 11 with "op-set-coverage as fundamental scope check for whole-body-bail-discipline pilots." Promotion to engagement-wide findings doc Addendum V queued.

### Composition with prior work

- **Doc 740 §II.4 component A/B probe**: identified the dominator (charCodeAt loop) at the call-site granularity but did NOT identify the enclosing scope's alphabet coverage requirement.
- **Standing rule 11 (Findings doc Addendum IV)**: prevents component mis-attribution; does NOT prevent op-set-coverage mis-attribution.
- **Finding II.2-bis substrate-introduction signature**: applies (M1+M2 flat A/B is expected); orthogonal to this finding.
- **Finding II.3 multi-tier cascade-revival**: applies (M3+M4 cascade-revival predicted at the alphabet+IC tier); but the cumulative-connection point (TL-EXT 5) is itself bounded by the op-set-coverage gap.

### Forward implication for the TL pilot

Three pivot options at this finding's recognition point (before TL-EXT 4 implementation):

1. **(b-narrow) continue**: implement Moves 3+4 anyway. The (b-narrow) substrate is real (alphabet extensions land; IC pattern at JIT tier validated) but cumulative CRB reclaim on json_parse_transform ceiling is 0%. Treat the pilot as substrate-introduction value (engagement-tier alphabet groundwork) not CRB-reclaim value.

2. **(b-architectural) pivot**: spawn loop-extraction or OSR work. Larger scope; closes the structural gap that Finding TL.1 names. CRB reclaim materialization possible.

3. **(b-different-fixture)**: rewrite Pred-tl.1 around a fixture whose top-level body fits (b-narrow)'s alphabet. Synthesizes a benchmark that demonstrates the pattern; doesn't address json_parse_transform.

4. **(b-stop)**: book Finding TL.1; close the TL locale at substrate-introduction value; pivot the engagement to a different bottleneck (Array.map at 55× per-op; JSON.parse at 3.3× per-op; or one of the four queued pilots RXF/SW/HS/web-crypto).

---

*This findings.md grows as TL-locale-specific findings surface. Promotions to LeJIT parent findings.md (engagement-wide JIT discipline) or to engagement findings doc Addendum V (corpus-level) noted explicitly.*

---

## Finding TL.2 (Non-Number, non-Object Values degrade to 0.0 at the JIT calling convention — (b-narrow) Moves 3+4 fast-paths structurally cannot deliver within Φ encoding) *[new, 2026-05-23 via TL-EXT 4 pre-implementation source-read]*

**Anchor**: pre-implementation source-read of `unbox_arg_f64` (interp.rs:9200-9206) reveals the JIT's f64-default calling convention encodes only:
- Value::Number → its f64 payload
- Value::Object(id) → `f64::from_bits(id.0 as u64)`
- **all other Value variants (String, BigInt, Boolean, Symbol, Null, Undefined) → 0.0**

Strings (and any non-Number / non-Object value) lose identity at the JIT/interp boundary. A `Value::String(Rc::new("abc..."))` becomes the f64 0.0 when passed to a JIT function or stored in a JIT local via LoadLocal → StoreLocal flow.

**Substrate implication for (b-narrow) Moves 3+4**: the design's Move 3 (GetProp+length-IC for String receivers) and Move 4 (CallMethod+charCodeAt-IC for String receivers) assumed the receiver f64 could be bitcast back to a String pointer or otherwise carry String identity. **No such encoding exists.** The f64 the JIT body holds for a String receiver is 0.0 — no String pointer recoverable.

TL-EXT 1 design's R1 ("String encoding bit-layout discovery") was named as a risk to resolve at TL-EXT 4 implementation time. The empirical resolution: **there is no encoding to discover; it doesn't exist.** Moves 3+4's IC fast-path bodies CANNOT be emitted within Φ's calling convention.

The structural alternatives at the engagement-tier level:
1. **Extend Φ encoding**: add a String tag scheme. NaN-boxing or sentinel-bit pattern that holds Rc<String> pointer bits. Major Φ-tier work; ~10+ LOC across translator + unbox helpers + dispatcher; risks breaking existing Σ/Τ/Ψ default-on paths if encoding choice conflicts.
2. **Per-receiver-type deopt at JIT body**: any time the JIT encounters a non-Number, non-Object Value (from LoadLocal of a previously String-typed local), trip a deopt and fall through to interp. Doesn't deliver reclaim (deopt cost > interp cost on first trip; cache state machine doesn't help here); essentially equivalent to whole-body bail.
3. **Source-read receiver type before compile**: at parse-time, infer that a local has only Number/Object values throughout the function (a flow analysis). Bail at compile if any LoadLocal sees a String/etc. Conservative; would bail json_parse_transform's inner loop because `out` is a String. Same outcome as Finding TL.1.

None of these alternatives are (b-narrow) scope. The cleanest read: **(b-narrow) as written cannot deliver Moves 3+4 fast paths in any session.** TL-EXT 4 and TL-EXT 5 reduce to skeleton-only delivery (ParsedOp variants + parse-pass scope check + immediate bail-to-non-JIT for the body), which is degenerate substrate-introduction work that doesn't enable any future fast-path.

**The honest closure of (b-narrow)**: TL-EXT 3 closed the entry-mechanism tier. Moves 3+4 cannot close their tier without prior Φ-encoding extension. The (b-narrow) plan was structurally bounded at TL-EXT 3, not at TL-EXT 5.

**Substrate implication (candidate Addendum V refinement to engagement findings doc)**: extends Finding TL.1's op-set-coverage check with a **value-domain-coverage check**: before spawning a JIT-alphabet pilot whose telos requires non-Number / non-Object receivers, verify the calling convention's encoding supports those receivers. If not, name the calling-convention extension as a prerequisite tier in the multi-tier reading (per Doc 740 §II.2 P1 relevant-tier set R).

**Composition with prior findings**:
- **Finding TL.1**: whole-body bail is the SECONDARY blocker. Finding TL.2 is the PRIMARY blocker for Moves 3+4 (they cannot deliver fast paths even if alphabet covered the whole body).
- **Doc 740 multi-tier reading**: R for json_parse_transform's checksum loop = {substrate algorithm (closed by CharCode-EXT 1), interp dispatch (closed by CharCode-EXT 2), JIT-tier loop dispatch (open; gated on TL pilot)}. Finding TL.2 adds a fourth tier: **JIT-tier value-domain encoding** (currently Φ encodes only Number+Object). The relevant-tier set R for the full pipeline-connection has 4 members, not 3.
- **Finding II.2-bis substrate-introduction signature**: TL-EXT 3 delivered structural substrate-introduction value at the entry-mechanism tier; that value stands. The (b-narrow) plan's Moves 3+4 design was wrong about the available substrate at the next tier.

### Forward implication

Per keeper directive "continue with (b-narrow) then pivot to (b-architectural)": Finding TL.2 reframes "continue with (b-narrow)" as a structural impossibility for Moves 3+4 fast-path delivery. **Honest closure**: TL-EXT 4 lands the alphabet variant skeletons (ParsedOp::GetProp + ParsedOp::CallMethod with parse-pass scope discipline) but translates each to bail-to-interp via whole-function-bail; document as "skeleton-only landing pending Φ-encoding extension." TL-EXT 5 measurement confirms zero reclaim (Pred-tl.1 falsified by Finding TL.2 as the structural cause). Pivot to (b-architectural) immediately.

Or: skip TL-EXT 4+5 entirely; close the locale at TL-EXT 3 + Findings TL.1+TL.2; pivot now. Saves substrate-introduction-without-future-value rounds.

Recommendation pending keeper signal.
