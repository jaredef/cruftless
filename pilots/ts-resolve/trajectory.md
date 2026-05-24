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

---

## TSR-EXT 1 — design doc (2026-05-24)

Output: `docs/design.md`. Key decisions:

- **Parser architecture**: from-scratch, Pin-Art-consistent. Vendoring forfeits the derived-from-constraints property at this locale. TSR wraps + extends rusty-js-parser rather than duplicating; tier-A+B subset estimated ~1630 LOC total (within Pred-tsr.1's ≤2000).
- **TS-surface-feature MVP**: 22 features across Tier-A (10) + Tier-B (12). Covers ~80% of consumer `.ts` files for application code. Conditional/mapped types, decorators, namespaces, JSX deferred or rejected.
- **AST integration**: reuse `rusty_js_ast::*` verbatim for runtime constructs; new `ts_ast::TsTypeRef` only for type-position constructs that erasure drops.
- **Erasure semantics**: annotations + `?` + `as` + `!` + interfaces + type aliases + generic args dropped; enums lowered to `Object.freeze({...})` with reverse-mapping; `public x: T` constructor params rewritten to ctor-body assignment.
- **CLI routing**: extension-based dispatch at script-load; import resolution tries `.ts` then `.js` then `.mjs`.
- **Annotation sidecar (TSR-EXT 5 preview)**: `(Module, Vec<TypeWitness>)` from TS parser; Runtime opts in; IPBR's ForOfFastNext eligibility check is the load-bearing probe — skip `_arr`/`_i` shape check when iter_slot's source is statically witnessed as Array. Per IPBR-EXT 2's ~50ns/iter shape-probe cost, ≥10% additional reclaim on TS-annotated string_url_sweep header_loop empirically vindicates research question (2).

Five open risks documented R1-R5 (contextual keywords; angle-bracket cast ambiguity; enum reverse-mapping; constructor-param rewriting; fixture-selection for Pred-tsr.5).

**Methodology**:
- TSR-EXT 2: lexer (~150 LOC)
- TSR-EXT 3: parser tier-A + tier-B (~1080 LOC)
- TSR-EXT 4: erasure + enum lowering + CLI dispatch (~250 LOC) — Pred-tsr.2/.3/.4 booking
- TSR-EXT 5: sidecar + IPBR consumer probe (~150 LOC) — Pred-tsr.5 booking + chapter close

**Status**: DESIGN COMPLETE. Multi-session implementation expected (TSR-EXT 2-5 ≈ 1500+ LOC across 4 rounds). Discipline target Pred-tsr.6: ≤6 implementation rounds.
