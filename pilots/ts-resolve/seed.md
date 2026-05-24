# ts-resolve — Resume Vector / Seed

**Locale tag**: `L.ts-resolve` (top-level per Doc 737 §IV)

**Status as of 2026-05-24 (post-100% parse-parity milestone)**: **🎯 PARSE-PARITY 100.0% on real npm corpus (374/374)**. Full TS-parity arc complete across 11 sub-locales (TRSLS / TRCAPS / TRGC×6 follow-ons / TRMLE / TRE / TROI + long-tail closures) — see `pilots/ts-resolve-*/` siblings + findings.md Addenda X-XI. Execute-parity at 70.9% (TXC: 265 MATCH / 374 - 108 BUN_FAIL = 265/266 = 99.6% of runnable files). TSR-tier work is COMPLETE; remaining execute-parity gap is genuine runtime-substrate territory per Finding IX.9 (parse-execute-separability).

**Status as of 2026-05-24 (prior — TSR-EXT 5 close)**: **CHAPTER CLOSED at TSR-EXT 5 (4 implementation rounds; Pred-tsr.6 HELD)**. Five of six predicates HELD; Pred-tsr.5 (≥10% reclaim from annotation-driven IPBR shape skip) FALSIFIED at sub-noise effect — a high-information null result that materially refines the cruftscript-spec design space (load-bearing claim shifts from iter-protocol-shape-skip to JIT IC specialization + VD tag preservation). Native `.ts` execution by `cruft` operational for the high-frequency real-world TS surface. See trajectory.md TSR-EXT 5 entry for the empirical disposition + Finding TSR.1.

**Historical status (founding)**: WORKSTREAM FOUNDED (TSR-EXT 0). Spawned per keeper directive on the source-language-resolver pivot. First step in a two-locale arc: this locale (TS as a source-language resolver, type-erasure path, no soundness claim) is the empirical-first stage. The follow-on locale (`cruftscript-spec/`, deferred) is the design-first stage for sound static types as substrate input.

**Workstream**: introduce a TypeScript source-language resolver upstream of `rusty-js-ir` that parses `.ts` files, performs type-erasure, and emits the same IR shape that `rusty-js-parser` produces for `.js`. Goal at this locale: native `.ts` execution by `cruft` without an external transpilation step. The probe question is whether erased type annotations can still feed downstream substrate tiers (JIT IC specialization, IHI/GPI/IPBR shape probes, VD's NaN-boxed tag schema) as profile-equivalent hints — i.e., whether **annotations carry substrate-actionable signal even after erasure**.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**:
- `rusty-js-parser/` — JS source-language resolver (existing; the dual)
- `rusty-js-ast/` — AST representation shared with this locale's output
- `rusty-js-ir/` — IR tier (this locale's output target)
- `rusty-js-jit/` — JIT tier (downstream consumer; profile-vs-annotation probe target)
- `interp-hot-intrinsics/`, `interp-getprop-ic/`, `iter-protocol-bytecode-rewrite/` — IC layers (downstream consumers)
**Composes with**:
- [Doc 729](../../docs/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — resolver-instance pattern; this locale adds a parallel source-language resolver feeding the same IR
- [Doc 730](../../docs/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) — lowering-compiler tier recurrence; TS-to-IR is one more recurrence
- [Doc 731](../../docs/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — alphabet purity upstream as bound on JIT complexity; erased TS annotations may **narrow the alphabet**, lowering JIT cost
- [docs/standing-rule-13-prospective-application.md](../../docs/standing-rule-13-prospective-application.md) — discipline anchor; TSR is the third prospective test of the thesis
- Planned sibling: `pilots/cruftscript-spec/` (not yet spawned) — sound-typed sibling language; TSR data informs the design

## I. Telos

**Two empirical answers**:

1. **(Substrate question)** Can a TS source-language resolver be inserted upstream of rusty-js-ir without altering the IR's contract, such that `.ts` files execute end-to-end through cruft with the same correctness gates (diff-prod 42/42, canonical fuzz byte-identical) that `.js` files hold today?

2. **(Research question, the load-bearing one)** After type-erasure, do the parsed-but-now-discarded TS annotations carry **substrate-actionable signal** that downstream tiers (JIT IC, IHI/GPI/IPBR shape probes, VD) can consume as profile-equivalent hints — i.e., does TS annotation density correlate with substrate-specialization opportunity in a way that JIT profiling alone misses?

Outcome (2) materially shapes the case for a follow-on `cruftscript-spec/` locale. Positive signal: sound types become substrate input rather than substrate hint, with closure of an entire JIT specialization tier. Null result: CruftScript would be valued for soundness alone, not for substrate leverage; the multi-quarter undertaking becomes harder to justify.

### I.1 First-cut scope

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. Skip any intermediate "TS as preprocessor" rounds; integrate the TS resolver as a peer of rusty-js-parser, both producing the same AST/IR.

- **TS lexer + parser** — minimum viable: TypeScript syntactic surface that real consumer code actually uses (let/const + type annotations, interfaces, type aliases, generics, `as` casts, `!` non-null assertions, enums, namespaces if low cost). Decision at TSR-EXT 1 design: write from scratch under Pin-Art discipline (consistent with `rusty-js-parser`), or vendor in a Rust TS parser (swc-style; mass of constraint complexity not derived from spec — collides with Pin-Art).
- **Type-erasure pass** — drop type annotations + interfaces + type aliases (no runtime presence); preserve enums (do have runtime presence); preserve namespaces if implemented (runtime presence).
- **AST output** — same `rusty_js_ast` shape produced by rusty-js-parser, so rusty-js-ir's lowering is reused verbatim.
- **CLI integration** — `cruft foo.ts` detects extension + routes to TS parser; `cruft foo.js` continues to use JS parser. Mixed-import modules (`.ts` importing `.js` and vice versa) at TSR-EXT 3.
- **Annotation-as-hint emission** — TSR-EXT 4 (deferred, research question (2)): a sidecar metadata channel from TS parser → IR carrying erased annotations as type-witness records. Downstream tiers can opt in.

### I.2 Constraints (Pin-Art enumeration)

```
C1. Existing .js execution paths byte-identical post-TSR (TS resolver
    is additive; .js continues to use rusty-js-parser unchanged).
C2. .ts execution end-to-end: parse → erase → AST → IR → bytecode →
    execute, with same correctness gates as .js. Diff-prod for .ts
    requires .ts versions of fixtures OR the assertion that erased
    .ts and equivalent .js produce identical output.
C3. Per Doc 729's resolver-instance pattern: the TS resolver does NOT
    carry over to the IR tier any TS-specific residue. The IR sees
    pure ECMAScript semantics. Type information lives in a sidecar
    channel (TSR-EXT 4), not in the IR proper.
C4. Pin-Art discipline at the TS lexer/parser: each TS surface
    feature is derived from a constraint (TC39 + microsoft/TypeScript
    spec excerpts), not copied from an existing implementation. This
    is the source of TS as a Pin-Art workstream — many existing TS
    parsers conflate parser + checker + transformer.
C5. TS unsoundness is preserved at this locale. `any`-typed code runs
    as plain JS. The locale makes NO soundness claim. Soundness is the
    cruftscript-spec/ locale's concern.
C6. Rule 11 5-axis pre-spawn check:
    (A1) component A/B: NOT YET DONE — no fixture corpus comparing
         .ts vs .js bytecode emission shape; built at TSR-EXT 1
    (A2) op-set coverage: AST/IR ops unchanged; TS adds no new ops
    (A3) value-domain coverage: same as .js (erasure)
    (A4) locals-marshaling: same as .js
    (A5) emission-shape coverage: same as .js post-erasure
C7. Per docs/standing-rule-13-prospective-application.md §3 conditions:
    (C1.sibling-anchor) rusty-js-parser is the empirical anchor for
                        the "source-language → AST/IR" closure shape
    (C2.shape-compat)  TS parser output reuses rusty_js_ast types
    (C3.cost-positive) TBD until TSR-EXT 1 design + LOC budget
    (C4.bail-safe)     .js path is unmodified; .ts failures don't
                        regress .js correctness
```

### I.3 Falsifiers

**Pred-tsr.1**: TS lexer + parser MVP under 2000 LOC. (Larger than IC pilots because parsing is non-trivial; TS adds ~50 surface forms over JS.)

**Pred-tsr.2**: canonical fuzz on `.js` corpus (acc=-932188103) byte-identical post-implementation. Falsifier: TSR's additive integration broke a `.js` path.

**Pred-tsr.3**: diff-prod 42/42 on `.js` fixtures holds. As Pred-tsr.2.

**Pred-tsr.4** (the new gate): create `.ts` versions of a subset of diff-prod fixtures with type annotations; assert each `.ts` fixture produces byte-identical stdout to its `.js` twin under `cruft`. Falsifier: type-erasure is leaking observable semantics.

**Pred-tsr.5** (the load-bearing research falsifier): on a benchmark fixture set written in both .ts (annotated) and .js (equivalent), the bytecode emitted is **identical at the IR level** modulo type-only constructs (enum lowering being the main expected difference). Falsifier: TS annotations are introducing IR shape that doesn't appear in JS equivalents — would indicate impurity in the resolver-instance design.

**Pred-tsr.6 (DISCIPLINE FALSIFIER per docs/standing-rule-13-prospective-application.md §5)**: locale closes in ≤6 implementation rounds. (Higher than GPI's/IPBR's ≤3 because the surface area is genuinely larger; TS has ~50 syntactic forms beyond JS. If it takes >6 rounds, the standing-rule-13 thesis remains intact — the discipline scales with surface-area complexity — but C1-C4's predictive power weakens at larger surfaces; revise as Finding TSR.X.)

## II. Apparatus

- **TS parser location**: new `pilots/ts-resolve/derived/src/` with `lexer.rs`, `parser.rs`, `erase.rs` (the lowering-to-rusty-js-ast pass).
- **Crate**: `ts-resolve` (or `rusty-ts-resolve` for naming parity with rusty-js-*). Decision at TSR-EXT 1.
- **AST target**: `rusty_js_ast::*` types reused verbatim.
- **CLI routing**: `cruftless/src/main.rs` (or wherever the script-load dispatch lives) detects file extension + routes accordingly.
- **Annotation sidecar** (TSR-EXT 4): a parallel `Vec<TypeWitness>` returned from the TS parser alongside the AST; downstream tiers consume opt-in via a Runtime config flag.
- **Bench instruments**: `.ts` fixture corpus at `pilots/ts-resolve/fixtures/`; pairwise `.ts` ↔ `.js` byte-identity comparison; CRB + diff-prod regression.
- **Correctness instruments**: canonical fuzz (untouched), diff-prod (untouched on .js; new on .ts), `.ts`-fixture-suite.

## III. Methodology

1. **TSR-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **TSR-EXT 1** — design doc: parser architecture (from-scratch vs. vendored decision), AST integration approach, erasure semantics, CLI routing, sidecar channel design, per-feature LOC budget. Includes a TS-surface-features-by-frequency table from a consumer-app corpus scan to bound scope.
3. **TSR-EXT 2** — TS lexer (subset: ASCII tokens + type-annotation tokens + arrow types). Pin-Art derivation per TS spec excerpts.
4. **TSR-EXT 3** — TS parser (subset: type annotations on let/const/function-param/return; interface; type alias; `as` cast; `!` postfix). Reuses rusty-js-parser's expression/statement layer where shapes overlap.
5. **TSR-EXT 4** — erasure pass + AST integration + CLI extension dispatch. End-to-end `cruft foo.ts` execution for the MVP surface.
6. **TSR-EXT 5** — annotation-sidecar channel + first downstream consumer probe (IHI/GPI: do erased Array<string> annotations short-circuit the IPBR shape probe?). LOAD-BEARING for research question (2).
7. **TSR-EXT 6** — composition probe + Pred-tsr.* booking + chapter close.

(**Discipline target**: ≤6 implementation rounds per Pred-tsr.6.)

## IV. Carve-outs and bounded scope

- TS subset only at first cut. Out of scope at this locale (deferred or rejected): decorators, JSX/TSX, namespaces-with-modules-interop, declaration-merging, conditional types, mapped types, template literal types, full generic instantiation. The subset target: ~80% of `.ts` files in popular consumer libs (lodash typings, express typings, simple application code) parse successfully.
- No type checking. TS unsoundness preserved. Errors at runtime, not compile time.
- Tooling integrations (LSP, source maps for TS) out of scope.
- CruftScript out of scope (separate locale).
- Aarch64 only.

## V. Standing artefacts

- `pilots/ts-resolve/seed.md`, `trajectory.md`
- `pilots/ts-resolve/docs/design.md` (TSR-EXT 1)
- `pilots/ts-resolve/docs/ts-surface-feature-budget.md` (TSR-EXT 1) — derived feature list with per-feature consumer-app frequency
- `pilots/ts-resolve/fixtures/` — pairwise `.ts` / `.js` test pairs
- `pilots/ts-resolve/derived/src/{lexer.rs,parser.rs,erase.rs}` (TSR-EXT 2-4)
- `cruftless/src/main.rs` adjustment for extension dispatch (TSR-EXT 4)
- Annotation sidecar channel touching `pilots/rusty-js-ir/derived/src/` (TSR-EXT 5)

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read Doc 729 + Doc 730 for the resolver-instance pattern that TSR materializes at the source-language tier. Read `docs/standing-rule-13-prospective-application.md` for the discipline this locale tests at larger surface-area scale. Inspect `pilots/rusty-js-parser/derived/src/parser.rs` for the dual JS parser's structure (TSR mirrors its shape; reuses its expression/statement code where surfaces overlap). The follow-on locale `pilots/cruftscript-spec/` is not yet spawned; TSR-EXT 5's outcome on research question (2) drives whether it gets spawned.

## VII. Strategic framing

This locale is the **empirical-first stage** of a two-locale arc. The keeper's framing question (TS unsoundness is a feature; CruftScript is the next-generation sound-typed sibling) is the research telos. TSR alone delivers:
- Native `.ts` execution by cruft (consumer value)
- Empirical data on whether erased annotations carry substrate-actionable signal (research value)
- Validation that the resolver-instance pattern admits parallel source-language resolvers (architecture value)

If TSR-EXT 5's annotation-sidecar probe shows substrate leverage (Pred-tsr.5 holds + downstream IC layers can act on erased annotations), the cruftscript-spec/ locale's design has empirical grounding: sound types become first-class substrate input, not erased before substrate sees them. If null result, cruftscript-spec/ proceeds on soundness-alone grounds with a smaller substrate-claim footprint.

**This is also a candidate Doc 7xx corpus publication**: cruftless as the first runtime where the source-language resolver tier is opened to parallel languages, and where typed-source-language annotations can be substrate input rather than erased upstream.
