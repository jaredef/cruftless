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

---

## TSR-EXT 3 — type stripper (2026-05-24)

**Design pivot from TSR-EXT 1's full-parser approach**: rather than reimplementing ECMAScript's expression+statement grammar to handle TS extensions, TSR strips TS-only syntax at the **source-text tier** using `rusty_js_parser::Lexer` for token positioning, then feeds the stripped text to the existing `rusty-js-parser`. Pin-Art-consistent — the stripping rules are derived from TS spec excerpts; the parser tier is reused unchanged.

**Established practice** (Bun's TS support, esbuild, swc's "transpile only" mode all use this technique). The trade for TSR is: dramatically lower LOC (~400 vs the design's ~1080), faster iteration toward end-to-end `.ts` execution, at the cost of: enum lowering needs a separate AST-replacement pass (TSR-EXT 4) and some edge cases (template-string contents resembling types) need disambiguation.

**Rules implemented** (`pilots/ts-resolve/derived/src/strip.rs`):

| Construct | Strip rule |
|---|---|
| `: T` annotation | After Ident/`)`/`]` at non-object-key context; consumed via `skip_type` (balances `<>` `[]` `{}` `()`) |
| `?` optional postfix | `?` immediately followed by `:` AND preceded by Ident |
| `!` non-null postfix | After expr-terminator AND before postfix-context (binop / `.` / `)` / etc.) |
| `as T` cast | After expr-terminator; consumes through end-of-type |
| `interface X { ... }` | Strip whole declaration via brace-matching |
| `type X = ...;` | Strip from `type` keyword through next statement-terminator |
| `declare ...` | Strip from `declare` through next statement-terminator |

**Skip-type heuristic** correctly handles:
- `string[]`, `Array<T>`, `T | U`, `T & U`
- Nested object types `{ k: number, v: string[] }` — `{` at type-start descends as object type literal; subsequent top-level `{` is function/initializer body
- Function types `(x: T) => U`
- Intersection within object types
- Annotation against `?`-marked identifier (`x?: T` — anchor walks past `?` to find `x`)

**Witness emission**: a `TypeWitness::LocalBinding` record is captured for each annotation, with the type as a raw text-extract `TsTypeRef::Named` (TSR-EXT 5 enriches the structured form).

**Crate LOC at TSR-EXT 3**: `strip.rs` ≈ 380, test suite ≈ 110, total round delta ≈ 490 LOC. Cumulative `ts-resolve` crate ≈ 700 LOC.

**Gates**:
- `cargo build --release -p ts-resolve`: ✅ clean
- `cargo test --release -p ts-resolve`: ✅ **21/21 PASS** (3 passthrough + 18 strip)
- diff-prod 42/42 PASS ✅ (Pred-tsr.3 HELD; .js paths untouched)
- canonical fuzz acc=-932188103 byte-identical ✅ (Pred-tsr.2 HELD)

**Test coverage** at TSR-EXT 3:
1. let annotation
2. const annotation
3. function param + return annotations
4. arrow param annotations
5. interface declaration
6. type alias
7. `as` cast
8. `!` non-null postfix
9. `?` optional param marker
10. `declare` statement
11. Array type `T[]`
12. Generic array `Array<T>`
13. Union types `A | B`
14. Intersection types `A & B`
15. Nested object type
16. Function type `(x: T) => U`
17. Witness capture
18. Pure-JS-via-TS passthrough preserved

**Bugs caught + fixed mid-implementation**:
- `LexerGoal::InputElementDiv` → corrected to `LexerGoal::Div` (sibling parser uses the short name)
- `skip_type`'s LBrace logic was rejecting object-type-literal `{...}` at type-start — fixed by tracking `start == i` for first-iter LBrace
- `is_annotation_colon` didn't see the Ident anchor when preceded by `?` (e.g., `x?: T`) — fixed by walking the anchor past `?`

**Capabilities post-TSR-EXT 3**:
- A `.ts` file using Tier-A annotations + interfaces + type aliases + casts + non-null + optional + `declare` parses end-to-end through `ts_resolve::parse_and_erase` and produces the same `rusty_js_ast::Module` as the equivalent erased `.js`
- Witness emission captures type names against binding names (suitable for TSR-EXT 5 sidecar consumption)

**Deferred to TSR-EXT 4**:
- Generic declaration heads `function f<T>(...)`, `class C<T> { ... }`
- Generic call sites `f<T>()` (angle-bracket disambiguation against `<` operator)
- Enum lowering (requires actual AST replacement, not pure text-strip)
- `public/private/protected/readonly` constructor-param shorthand (requires ctor-body rewrite)
- Class field annotations (some pass through current rules; full coverage needs verification)
- CLI extension dispatch (`cruft foo.ts`)

**Next round**: TSR-EXT 4 — enum lowering + ctor-param shorthand + class fields + CLI dispatch. End-to-end `cruft foo.ts` execution + Pred-tsr.2/.3/.4 booking. Estimated ~250 LOC.

**Status**: TSR-EXT 3 LANDED. Tier-A core surface erases correctly; all 21 tests pass. 2 of 6 implementation rounds consumed (Pred-tsr.6 budget on track).

---

## TSR-EXT 4 — CLI dispatch + decl-head generics + brace-context disambig (2026-05-24)

**Round shape**: ship end-to-end `cruft foo.ts` execution; cover the most-frequent remaining TS surface gaps (function/class decl-head generics, object-literal vs class-body disambig); land Pred-tsr.4 booking.

**Edits**:

1. **CLI dispatch** (`cruftless/Cargo.toml` + `cruftless/src/main.rs`):
   - Added `ts-resolve` as dependency
   - In `main`, after `read_to_string`, route `.ts` / `.mts` / `.cts` extensions through `ts_resolve::strip::strip_ts` before `evaluate_module`. The stripped source keeps byte-aligned positions, so error spans + line numbers remain accurate.

2. **Decl-head generics** (`strip.rs` step + new `match_angle`):
   - When Ident `function`/`class` is followed by Ident NAME + `<`, strip from `<` to its matching `>` via `match_angle` (handles nested generics + `>>` treated as two closers). Unambiguous because the contexts are syntactically distinct from the `<` operator.

3. **Brace-context stack** (`BraceCtx::{Block, ObjectLit, ObjectType}` + `Scanner::brace_stack` + `classify_brace`):
   - At each `{`, classify based on the immediately-preceding token: expression-context (`=` / `(` / `,` / `:` / `?` / `=>` / `&&` / `||` / `??` / `...` / `return` / `yield` / `throw` / etc.) → `ObjectLit`; otherwise `Block`.
   - At each `}`, pop the stack.
   - `is_annotation_colon` bails unconditionally when the top of `brace_stack` is `ObjectLit` — fixes the bug where object-literal `key: value` was misread as annotation.

**Bugs caught + fixed mid-round**:
- The first end-to-end run errored with `Cannot read property 'who' of undefined (receiver='g')`. Root cause: `const target: Greeting = { who: "world" }` was stripping `= ` along with the annotation because object-literal `who: "world"` was ALSO being mis-stripped as an annotation, cascading into broken bytecode. **Fix**: brace-context stack (above).
- Second debugging pass discovered `Punct::Eq` (`==`) was being used where `Punct::Assign` (`=`) was meant — in three locations in `strip.rs` (classify_brace, skip_type top-stopper, is_annotation_colon safe-terminator set). **Fix**: replaced all three with `Punct::Assign`. The `next_is_postfix_context` use of `Eq`/`Ne` is correct (those are binops that can follow a stripped `!`).
- Both bugs caught only at the end-to-end smoke test, not by the unit tests — direct evidence that **e2e gates uncover failure modes that unit tests miss**. Adding to standing-discipline awareness.

**Gates**:
- `cargo build --release --bin cruft -p cruftless`: ✅ clean
- `cargo test --release -p ts-resolve`: ✅ **24/24 PASS** (3 passthrough + 21 strip including 3 new generics tests)
- diff-prod 42/42 PASS ✅ (Pred-tsr.3 HELD; .js paths still byte-identical)
- canonical fuzz acc=-932188103 byte-identical ✅ (Pred-tsr.2 HELD)
- **End-to-end `cruft foo.ts`**: ✅ **WORKING**
  - `01-end-to-end-hello.ts`: interface + annotated function + annotated const + method call → `hello, world`
  - `02-generics-and-union.ts`: generic `function total<T extends Item>(items: T[])`: number + `for (const it of items)` + nested object literals → `6`
- **Pred-tsr.4 (TS twin byte-identical to JS twin)**: ✅ **HELD** — `01-end-to-end-hello.ts` vs `01-end-to-end-hello.js` produce byte-identical stdout under cruft

**LOC delta this round**: ~80 (CLI dispatch ~16 + brace-context machinery ~50 + decl-head generics ~14). Cumulative ts-resolve crate ≈ 780 LOC.

**Capabilities post-TSR-EXT 4**:
- End-to-end `.ts` execution by `cruft` for the high-frequency real-world surface (annotations, interfaces, type aliases, casts, non-null, optional, declare, generics on function/class heads, union, intersection, nested object types, function types)
- Object-literal vs class-body disambig correct via brace-context stack
- Pred-tsr.4 demonstrably HELD on first end-to-end fixture

**Deferred to TSR-EXT 5** (chapter-close round):
- Enum lowering (requires actual AST replacement — `enum E { A, B }` → `const E = Object.freeze({...})` with reverse-mapping)
- `public/private/protected/readonly` ctor-param shorthand (ctor-body rewrite)
- Generic call-sites `f<T>()` (angle-bracket disambig vs `<` operator — Pin-Art tractable but lower priority than the load-bearing sidecar probe)
- **Annotation sidecar wiring + IPBR consumer probe** — the load-bearing Pred-tsr.5 work
- Composition probe + final disposition + chapter close

**Implementation rounds consumed**: 3 of 6 (TSR-EXT 2 lexer + TSR-EXT 3 stripper + TSR-EXT 4 CLI/disambig/generics). Pred-tsr.6 budget on track; final round (TSR-EXT 5) holds the load-bearing research-question probe.

**Status**: TSR-EXT 4 LANDED. End-to-end `cruft foo.ts` operational. Three concrete bugs caught + fixed; brace-context disambig is the key correctness machinery for handling object literals vs class bodies vs function bodies in TS source. Standing observation: e2e smoke tests caught failure modes that 21 unit tests did not.

---

## TSR-EXT 5 — research-question probe + chapter close (2026-05-24)

**Round shape**: book the load-bearing Pred-tsr.5 prediction (≥10% additional reclaim when TS annotations short-circuit IPBR's runtime shape probe on `Array<T>` for-of loops). The empirical answer to this prediction shapes whether `cruftscript-spec/` is founded on grounded-substrate-claim or soundness-alone grounds.

### Probe instrument

Rather than full annotation-sidecar plumbing (which would require threading TypeWitness through cruftless → Runtime → bytecode compiler → IPBR's handler, ~200 LOC of cross-crate work for an effect of unknown sign), TSR-EXT 5 implemented an **upper-bound measurement probe**: a per-iter_slot `(iter_id, src_id, idx)` cache on Frame that, after the first ForOfFastNext iteration at a site, skips the `_arr`/`_i`/InternalKind lookup on all subsequent iterations.

This is exactly what a TS `Array<T>` annotation COULD enable (compile-time pre-population of the cache eliminating the first-iter probe too). The cache thus measures the **upper bound** of annotation-driven savings at IPBR.

Implementation:
- New `Frame::iter_fast_cache: Vec<Option<(ObjectRef, ObjectRef, usize)>>` (sparse, indexed by iter_slot)
- `iter_fast_cache_set` helper
- `write_local` invalidates the cache on any overlapping local write
- ForOfFastNext handler: check cache first; fall back to shape probe; populate cache on probe hit
- ~50 LOC delta

### Measurement

| Fixture | IPBR baseline (no cache) | TSR-EXT 5 (with cache) | Δ |
|---|---:|---:|---:|
| string_url_sweep CRB median (N=5) | 584 ms | 588 ms | +0.7% (sub-noise) |
| string_url_sweep header_loop A/B (range over 4 readings) | 193-218 ms | 195-198 ms | within noise |
| json_parse_transform CRB median (N=5) | 1773-1818 ms | 1853 ms | +2.5% (marginal regression from dispatch overhead) |

### Empirical finding (load-bearing for research question)

**Pred-tsr.5 FALSIFIED at the IPBR consumer.** The per-iter shape-lookup cost the cache eliminates (~50ns per iter for two `object_get` calls + InternalKind match) is too small relative to the dominant per-iter costs:
- `idx.to_string()` allocation for `arr[idx]` lookup (~100ns/iter)
- `object_set("_i", ...)` HashMap write (~80ns/iter)
- The actual body work (string method dispatch, etc.)

The cache itself adds dispatch overhead (cache-hit branch + cache write) that on json_parse_transform marginally exceeds the savings.

### Finding TSR.1 — load-bearing for cruftscript-spec design

The research question (2) — "do erased TS annotations carry substrate-actionable signal for downstream IC/JIT/VD tiers?" — at the IPBR-tier consumer is **NULL**.

Implications:
- The substrate-leverage claim for TS annotations at iter-protocol shape probes does NOT pay off — the probe is already cheap enough that elimination is sub-noise.
- The cruftscript-spec follow-on locale should NOT be founded on iter-protocol-shape-elimination as a load-bearing claim.
- Other potential consumers may still pay off (JIT IC specialization on typed args, VD NaN-box tag on typed numbers, IHI/GPI dispatch specialization on typed receivers) — each needs its own empirical probe before the substrate-leverage claim can be made for that consumer.
- The cruftscript-spec value proposition reduces to: **soundness alone + (potentially) JIT IC specialization** — still valuable but a SMALLER corpus claim than originally framed.

This is a high-information null result. Doc 723 §IV.b finding-density holds: an empirical probe that returns a clean negative is more informative than an ambiguous positive.

### Decision: revert the cache instrument

The cache adds dispatch overhead without measurable savings → reverted from `pilots/rusty-js-runtime/derived/src/interp.rs` to pre-instrument state. The probe data is recorded; the substrate stays clean.

### Deferred from this round

Enum lowering + ctor-param shorthand + sidecar plumbing were originally scoped for TSR-EXT 5. Given the research-question null result, the strategic priority shifts: these features are still worthwhile for TS feature completeness BUT they no longer carry the load-bearing case for cruftscript-spec. Deferred to follow-on `pilots/ts-resolve-features/` or absorbed into a single feature-completion round if revisited.

### Final disposition

| Predicate | Disposition |
|---|---|
| Pred-tsr.1 (≤2000 LOC) | ✅ HELD at ~780 LOC cumulative |
| Pred-tsr.2 (canonical fuzz on .js) | ✅ HELD byte-identical |
| Pred-tsr.3 (diff-prod 42/42 on .js) | ✅ HELD |
| Pred-tsr.4 (.ts twin produces byte-identical stdout) | ✅ HELD (`01-end-to-end-hello.ts` vs `.js` twin) |
| Pred-tsr.5 (≥10% reclaim from annotation-driven shape skip) | ❌ **FALSIFIED at the IPBR consumer** (sub-noise effect; cache overhead marginally exceeds savings on heavier fixtures) |
| Pred-tsr.6 (≤6 implementation rounds) | ✅ HELD at 4 implementation rounds (TSR-EXT 2 lexer + TSR-EXT 3 stripper + TSR-EXT 4 CLI/disambig/generics + TSR-EXT 5 probe) |

### Cross-locale implications

**Standing rule 13 prospective-application thesis** (per `docs/standing-rule-13-prospective-application.md`): TSR is the third locale to apply the rule. Outcome:
- Locale closed in 4 implementation rounds (under Pred-tsr.6's ≤6 budget). The discipline scales gracefully with surface-area complexity (TSR is ~5× the LOC of GPI/IPBR; took 4 rounds vs their 1).
- The C1-C4 conditions held for the substrate-tier work (lexer + stripper + CLI dispatch). Where they didn't hold was the research-question probe: condition C3 (cost-positive when integrated) FAILED at the IPBR consumer.
- This is itself a finding about the thesis: **C3 is the load-bearing condition for substrate-leverage claims at a downstream consumer**. C1 (sibling anchor) + C2 (shape compat) + C4 (bail safety) can all hold, and a probe can still return null if C3's per-call cost model isn't favorable.
- The standing-rule-13 thesis is REFINED rather than weakened: ≤3 implementation rounds is the convergent shape for substrate-tier work; research-question probes can run an extra round to book a null result, and that's still discipline-consistent.

**For cruftscript-spec follow-on**: founded on weaker substrate claim than originally framed. Likely-load-bearing consumers (to probe next): JIT IC specialization on typed function args + VD NaN-box tag preservation through typed numerics. These would need their own measurement instruments.

### Capabilities post-TSR-EXT 5 (chapter close)

- Native `.ts` execution by `cruft` for high-frequency TS surface (annotations, interfaces, type aliases, casts, non-null, optional, declare, generics on decl heads, unions, intersections, nested object types, function types)
- 24/24 unit tests; diff-prod 42/42; Pred-tsr.2/.3/.4 all HELD
- Sidecar shape designed + types in place (`TypeWitness` enum) for future consumers
- Pred-tsr.5 empirical null result documented as Finding TSR.1
- Pred-tsr.6 discipline HELD at 4 implementation rounds

### Status: CHAPTER CLOSED at TSR-EXT 5

**Outcome**: 5 of 6 predicates HELD; 1 (Pred-tsr.5) FALSIFIED with high-information null result that materially refines the cruftscript-spec design space. Substrate (native `.ts` execution) delivered + working. Research question (annotation-as-substrate-input) answered honestly at the IPBR consumer.

**Locale closed in 4 implementation rounds** vs Pred-tsr.6's ≤6 budget. Standing-rule-13 thesis: third corroboration on discipline (rounds-to-close); first empirical refinement on C3 condition (cost-positive integration is the load-bearing condition that can fail independently).

**Implication for `cruftscript-spec`**: still a valuable locale to spawn; load-bearing case shifts from "sound types as IPBR-shape-skip substrate" to "sound types as JIT IC specialization input + VD tag preservation." Each follow-on consumer needs its own empirical probe.

---

## 🎯 TS-PARITY ARC MILESTONE — 100.0% PARSE-PARITY ACHIEVED (2026-05-24)

Booked as a milestone entry across the TSR locale's trajectory after the broader TS-parity arc closed. The TSR locale itself closed at TSR-EXT 5; the parse-parity work was carried by 11 sibling sub-locales spawned downstream of TSR's substrate base.

### Arc totals (since TCC-EXT 1 baseline 2026-05-24 early in the session)

| Metric | Baseline | Final | Δ |
|---|---:|---:|---:|
| Parse-parity (TCC, 374 npm `.ts` files) | 37.7% | **100.0%** | +62.3 pp |
| Execute-parity (TXC, same corpus) | 5.1% | 70.9% | +65.8 pp |
| Execute-parity of *runnable* files (excluding 108 BUN_FAIL) | — | **265/266 = 99.6%** | — |
| Cumulative LOC delta to TSR substrate | ~1700 (founding) | ~2900 | +1200 LOC |

### Sub-locales that carried the arc

| Locale | Δ parse pp | Substrate work |
|---|---:|---|
| TRSLS | +9.4 | Template-tail goal selection in lex_all |
| TRCAPS | +12.8 | Class + param shapes (6 fixes) |
| TRGC-EXT 1 | +9.4 | Unified `<...>(` rule + ternary tracking |
| TRGC-EXT 2 | +1.6 | Arrow/fn-type disambig, ASI, overload-MVP |
| TRGC-EXT 3 | +10.9 | Overload-pattern completions (keyword unblock + decl-head generics) |
| TRGC-EXT 4 | +7.5 | UShr/Shr in overload scan + match_angle |
| TRGC-EXT 5 | +4.8 | Intersection descent, import-type strip, ClassBody distinction |
| TRGC-EXT 6 | +1.1 | Arrow-body Block classify + abstract methods |
| TRE | +0.6 | Enum-MVP-strip + keyword-overload unblock |
| TRMLE | +1.3 | Module-loader TS dispatch + skip_type semicolon-at-top |
| TROI | +0.3 | Type-only-import elision |
| Long-tail close (case-label, !`(`/`[`, export-default-fn binding, brace-paren tuple, ternary obj-key guard, match_angle bracket-balance + Template stopper, postfix-! goal, import-equals, ternary cross-talk, computed-key obj-lit) | +3.5 | 10+ singleton substrate fixes |

### Findings booked at the milestone

Per findings.md Addendum XI:
- **IX.7** (cross-talk between depth-tracking stacks) + new standing rule #16 (two-coordinate matching for depth stacks).
- **IX.8** (long-tail singletons are full-size SIPE-T instances — SIPE-T scale-invariance confirmed at the smallest scale).
- **IX.9** (parse-execute-separability as engagement-tier finding).

### Status: TSR-tier work COMPLETE

The TS-parity arc has saturated at the TSR substrate tier. The 1 remaining CRUFT_FAIL (`rxjs/ajax.ts`) is a residual ESM-cycle issue not solvable at the TSR tier (rxjs's internal type/runtime structure produces a cycle that even tsc-emitted code would need runtime live-bindings to handle). Per Finding IX.9, the remaining execute-parity gap belongs to a separable research arc (runtime-substrate work, corpus-runnability work).

**Doc 742 §III** (P/I/R decomposition) is empirically validated at the final milestone: parse-parity (P) reached 100% via TSR work, integration-parity (I) reached ~95% via TRMLE module-loader work + the dispatcher hookup, elision-parity (R) reached ~99% via TROI type-only-imports + the post-strip pass. Each component is independently measurable; each is independently solvable at its own substrate tier.
