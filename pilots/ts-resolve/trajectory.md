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

---

## TSR-EXT 2 — lexer + crate scaffolding (2026-05-24)

**Empirical finding at the lexer tier**: TypeScript adds **zero new token kinds** vs ECMAScript. All TS contextual keywords (`type`, `interface`, `keyof`, `as`, `is`, `readonly`, `unique`, `infer`, `satisfies`, `namespace`, `declare`, `abstract`, `override`, `public/private/protected`, `implements`, `out`, `asserts`, `global`) are valid identifier names at value position and are reserved only at type position — disambiguation belongs to the parser, not the lexer. The TS-only punctuation TSR cares about (`!` non-null postfix, `?` optional postfix, `<>` generic angle brackets, `=>` arrow-in-function-type) all exist in ECMAScript already.

This means the TSR lexer is a thin re-export of `rusty_js_parser::Lexer` + a small contextual-keyword table. Pin-Art-consistent (no derivation work needed for tokens), and per Doc 731's alphabet-purity claim, keeping the lexer's alphabet identical between JS and TS preserves the substrate-tier alphabet boundary cleanly.

**Crate scaffolding**:
- `pilots/ts-resolve/derived/Cargo.toml` — depends on rusty-js-ast + rusty-js-parser
- `pilots/ts-resolve/derived/src/lib.rs` — public API: `parse_and_erase` + `parse_with_witnesses`
- `pilots/ts-resolve/derived/src/lexer.rs` — re-exports + `TS_CONTEXTUAL_KEYWORDS` + `is_ts_contextual_keyword`
- `pilots/ts-resolve/derived/src/ts_ast.rs` — `TsTypeRef`, `TsObjectMember`, `TsFnParam`, `TsLiteralVal`, `TsAnnotation`, `TypeWitness`, `TypeWitnessKind`
- `pilots/ts-resolve/derived/src/parser.rs` — `TsParser` scaffold (delegates to rusty-js-parser at this round; TSR-EXT 3 replaces with real type-position consumer)
- `pilots/ts-resolve/derived/src/erase.rs` — `erase_module` (identity at this round; TSR-EXT 3+ adds real erasure)
- `pilots/ts-resolve/derived/tests/passthrough.rs` — 3 smoke tests (all PASS)
- `pilots/ts-resolve/fixtures/00-passthrough-valid-js.ts` — pure-JS-in-.ts smoke fixture
- `Cargo.toml` workspace member registration

**Round LOC**: ~210 (lib 38 + lexer 40 + ts_ast 92 + parser 50 + erase 18 + tests 30). Well under TSR-EXT 2's ~150 estimate when normalized to "lexer-tier work proper" (the ts_ast + parser/erase scaffolding is forward-loading TSR-EXT 3+ infrastructure).

**Gates**:
- `cargo build --release -p ts-resolve`: ✅ clean
- `cargo test --release -p ts-resolve`: ✅ 3/3 PASS
- diff-prod 42/42 PASS ✅ (Pred-tsr.3 still HELD; .js paths unaffected since cruftless doesn't yet depend on ts-resolve)

**Capabilities post-TSR-EXT 2**:
- A `.ts` file that uses ZERO TypeScript-specific syntax (i.e., is also valid JavaScript) round-trips through `ts_resolve::parse_and_erase` and yields the same `rusty_js_ast::Module` that `rusty_js_parser::parse_module` produces. Useful as a baseline + as the bail-safety fallthrough at TSR-EXT 3+.
- Contextual-keyword detection helper ready for TSR-EXT 3's type-position consumer.
- `TypeWitness` sidecar shape designed + types in place (empty vector emitted at this round; populated at TSR-EXT 5).

**Next round**: TSR-EXT 3 — real TS parser. Tier-A features end-to-end (type annotations on let/const/function param/return, interface, type alias, `as` cast, `!` non-null, generics, enums). Estimated ~600-1080 LOC across one or two implementation rounds. The real Pin-Art derivation work begins here.

**Status**: TSR-EXT 2 LANDED. Crate operational; passthrough verified; sidecar shape designed. TSR-EXT 3 next.
