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
