# ts-resolve — Trajectory

Append-only log of rounds. Most recent at bottom.

---

## TSR-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper pivot directive on the source-language-resolver vision: "I'm thinking about adding a resolve layer that feeds into the rusty-js-ir that will allow typescript to be interpreted instead of transpiled. Typescript is unsound; I want to support it, but also want to build the next generation of statically typed: CruftScript."

**Strategic framing**: this is the empirical-first stage of a two-locale arc. TSR delivers native `.ts` execution + an empirical probe on whether erased TS annotations carry substrate-actionable signal. The follow-on `cruftscript-spec/` locale (sound-typed sibling language; types as first-class substrate input) is staged for after TSR-EXT 5's research-question outcome.

**Founding artefacts**:
- `pilots/ts-resolve/seed.md` (this round)
- `pilots/ts-resolve/trajectory.md` (this file)
- `pilots/ts-resolve/docs/` + `pilots/ts-resolve/fixtures/` scaffolded
- Follow-on locale `pilots/cruftscript-spec/` flagged but not yet spawned

**Two empirical telos** (per seed §I):
1. Substrate: native `.ts` execution end-to-end via cruft, same correctness gates as `.js`.
2. Research: do erased TS annotations carry substrate-actionable signal for downstream IC/JIT/VD tiers? (Load-bearing for the CruftScript follow-on.)

**Six Pred-tsr.* + 1 discipline falsifier** (per seed §I.3):
- Pred-tsr.1: TS lexer + parser MVP ≤2000 LOC
- Pred-tsr.2: canonical fuzz on `.js` corpus byte-identical post-TSR
- Pred-tsr.3: diff-prod 42/42 on `.js` fixtures
- Pred-tsr.4: `.ts` versions of fixtures produce byte-identical stdout to `.js` twins
- Pred-tsr.5 (load-bearing): TS-vs-JS IR-level bytecode identical modulo type-only constructs
- Pred-tsr.6 (DISCIPLINE FALSIFIER): closes in ≤6 implementation rounds. Higher bar than GPI/IPBR because TS surface is larger (~50 syntactic forms beyond JS); if exceeded, standing-rule-13 thesis remains intact (discipline scales with surface complexity) but C1-C4 predictive power weakens at larger surfaces.

**Pre-spawn rule 11 5-axis check**:
- (A1) component A/B — NOT YET DONE; built at TSR-EXT 1 as a TS-surface-feature frequency scan
- (A2) op-set coverage — AST/IR ops unchanged; TS adds no new ops
- (A3) value-domain coverage — same as `.js` (erasure)
- (A4) locals-marshaling — same as `.js`
- (A5) emission-shape coverage — same as `.js` post-erasure

**Deeper-layer design (rule 13 prospective)**:
- SKIP "TS as preprocessor" rounds; integrate TS as peer source-language resolver of rusty-js-parser, both producing the same AST/IR
- Annotation sidecar channel (TSR-EXT 5) separates type information from IR proper — preserves Doc 729 resolver-instance purity
- CLI dispatches by file extension; mixed-import handled at TSR-EXT 3

**Composition with existing pilots**:
- IHI / GPI / IPBR / JIT / VD all become downstream consumers of the TS annotation sidecar at TSR-EXT 5
- Doc 729 resolver-instance pattern admits this locale natively; the JS resolver remains as the canonical instance
- Doc 731 alphabet-purity claim: erased TS annotations may NARROW the alphabet (`Array<string>` is narrower than `any[]`), lowering JIT specialization cost

**Next round**: TSR-EXT 1 — design doc at `docs/design.md`. Output:
1. Parser architecture decision: from-scratch (Pin-Art-consistent) vs. vendored (faster but Pin-Art impure). Decision criterion: vendoring forfeits the "derived from constraints" property at this locale; from-scratch is the Pin-Art-consistent choice.
2. TS-surface-feature budget: scan ~5-10 consumer apps' `.ts` files; tally feature frequency; cut to MVP subset that covers ~80% of real `.ts` code.
3. AST integration approach: reuse `rusty_js_ast::*`; new TS-specific AST nodes only for type-only constructs that are then dropped by erasure.
4. Erasure semantics: annotations dropped; interfaces/type aliases dropped; enums lowered to objects; namespaces deferred or rejected.
5. CLI routing: extension-based dispatch at script-load.
6. Annotation sidecar channel: shape (`Vec<TypeWitness>` parallel to AST), consumer API, opt-in flag.
7. Per-feature LOC budget.

**Status**: SCAFFOLDED. Founding artefacts written; TSR-EXT 1 next.
