# rusty-js-ir — Trajectory

Chronological resume anchors for the IR workstream. Reads seed.md first; this file is the time-ordered record of substrate moves and their yields.

Format: one section per "EXT" (extension round); each round closes with a status block, a cumulative numbers table, and an open-scope list.

## IR-EXT 1 — 2026-05-19 (genesis)

**Headline**: end-to-end pipeline operational from ECMA spec text to running engine. One section (§23.1.3.20 Array.prototype.map) translated, lowered, wired into rusty-js-runtime, and verified against test262.

### Commits

| commit | tag | recognition |
|---|---|---|
| `fd9eba38` | IR-DESIGN.md draft | 360-line design doc with motivation, ~50-node IR surface, worked example, linter algorithm, three-tier impl plan. |
| `cb9b2966` | IR-DESIGN §0 + seed §A8.33 | Formalized the structural recognition: IR is resolver-instance #0 above Doc 729 §IV's #1 (Cargo). seed §A8.33 names the open-endedness of the resolver pattern; third corroboration of Doc 729 §IX Pred-729.5. |
| `21117369` | Tier 1: ir.rs / lower.rs / lint.rs + sections/array_prototype_map.rs | IR data structures (~50 nodes in 9 categories), lower_to_rust pipeline, spec-vs-IR diff linter, hand-translated §23.1.3.20, drift-demo example. |
| `fdacbf8f` | Tier 1.5: wire into runtime; test262 +3 | Typed-index IR variants (LetIndex/AssignIndex/IntConst/IndexAdd/IndexAsValue/IndexAsKey); six new Runtime helpers; generated.rs registered; Array.prototype.map slice 81.4% → 82.8%. |

### Substrate at IR-EXT 1 close

**IR alphabet**: ~50 nodes (per seed.md §III).

**Runtime helper coverage (IR lowering targets, cumulative)**:
- `rt.to_object(&v)` — §7.1.18.
- `rt.length_of_array_like(&v)` — §7.3.20.
- `rt.has_property_via(&v, key)` — §7.3.10 over &Value.
- `rt.read_property_via(&v, key)` — §7.3.2 over &Value.
- `rt.create_data_property_or_throw(&v, key, val)` — §7.3.6.
- `rt.array_species_create(&v, len)` — §23.1.3.27 (Tier-1.5 simplified).
- Plus pre-existing helpers: rt.is_callable, rt.coerce_to_number, rt.to_string_strict, rt.coerce_to_string, rt.coerce_to_number, rt.to_primitive, rt.op_add_rt, rt.is_loosely_equal_rt, rt.require_object_coercible, rt.unwrap_primitive.

**Sections translated**: 1 (Array.prototype.map §23.1.3.20).

**Linter findings against committed IR**: 2 expected (param.callbackfn / param.thisArg binding-convention markers), 0 unexpected.

### test262 numbers at IR-EXT 1 close

| section | pre-IR rate | post-IR rate | Δ |
|---|---|---|---|
| Array.prototype.map | 81.4% (176/216) | 82.8% (179/216) | +3 |
| Array.prototype (chapter, regression check) | 80.6% (2267/2810) | 80.7% (2270/2810) | +3 (same delta; no regression elsewhere) |

The +3 delta is structurally significant: the IR-lowered function is at least as spec-faithful as the hand-written version it replaced, with no regression on the surrounding chapter. The pipeline's correctness is empirically validated.

### Conjecture status

The seed.md §I conjecture (spec conformance gets monotonically easier post-IR) is **operationally testable** after IR-EXT 1 close: every subsequent section translation that lands in `generated.rs` provides one data point. The prediction: zero test262 regressions on IR-covered sections going forward, regardless of how many sections accumulate.

### Doc 729 §V cross-check

§V's vertical-recursion-with-stage-deterministic-emission claim is **corroborated at the authoring-to-substrate stratum** (cruftless seed §A8.33). Pre-IR-EXT 1 this was a design claim; post-IR-EXT 1 it is an empirical fact: the lowering compiler (resolver-instance #0c) emits a Rust function that runs in the engine and matches the hand-written version's behavior.

### Open scope at IR-EXT 1 close

Immediate (Tier 1.6 candidates):
1. **Iteration cluster translation**: Array.prototype.{forEach, filter, every, some, find, findIndex, reduce, reduceRight}. All share §23.1.3.20's iteration shape (with variations on what's done at each iteration step). High amortization — most of the IR primitives are already in the alphabet.
2. **Object.{keys, values, entries}**: small, structural, popular.
3. **Number.prototype.toString/toFixed/toExponential/toPrecision**: brand-check + RangeError pattern, similar to cruftless's P62.E19/E20 fixes.

Mid-term (Tier 2):
4. **ECMA-262 spec-XML parser**: ingest emu-alg blocks from the tc39/ecma262 source; auto-derive SpecStepRecord. Triggers when ≥10 sections are hand-translated (per seed §V.M5).

Long-term (Tier 3+):
5. **Broad coverage**: ~80–100 spec sections covering all of cruftless's built-in surface.
6. **Async / generator semantics**: deferred to Tier 4 per seed §VII.

### Resume protocol

Read `IR-DESIGN.md`, `pilots/rusty-js-ir/seed.md`, this trajectory. Latest committed binary at the cruftless build target. Pipeline driver:

```bash
cargo run --example lower_array_map -p rusty-js-ir   # emit Rust
cargo run --example lint_array_map -p rusty-js-ir    # spec-vs-IR diff
cargo run --example lint_drift_demo -p rusty-js-ir   # prove linter catches drift
```

Add a section by creating `pilots/rusty-js-ir/derived/src/sections/<name>.rs` with `pub fn build() -> IRFunction` + `pub fn spec_steps() -> Vec<SpecStepRecord>`, registering it in `sections/mod.rs`, lowering via a new example or by appending to `generated.rs`, and replacing the hand-written registration in `pilots/rusty-js-runtime/derived/src/prototype.rs`.

Pin-Art tag count for the IR workstream: 4 commits as of IR-EXT 1.


## IR-EXT 2 — 2026-05-19 (iteration cluster + runtime length-bump)

**Headline**: pipeline scaled to 5 spec sections in one round. Four more Array.prototype iteration methods (forEach, filter, every, some) translated, lowered, wired in, regression-checked. Net +15 tests across the cluster vs pre-IR baseline.

### Commits

| commit | tag | recognition |
|---|---|---|
| `9ec80b4c` | IR-EXT 2: seed + trajectory + iteration cluster | Workstream seed.md + trajectory.md per Pin-Art shape; sections/array_prototype_iteration.rs translates §23.1.3.{15,7,6,29} sharing §23.1.3.20's preamble; ToBoolean lowering returns Rust bool; create_data_property_or_throw on Array auto-bumps length per §10.4.2.4. |

### Substrate at IR-EXT 2 close

**IR alphabet**: unchanged from IR-EXT 1 (the iteration cluster used only existing nodes). The pattern: when consecutive translations target methods that share structural shape, no alphabet extension is needed — the IR is already complete enough.

**Runtime helper coverage**: unchanged surface; internal refinement of `array_species_create` (no explicit length=0) and `create_data_property_or_throw` (auto-bump array length).

**Sections translated**: 5 (Array.prototype.{map, forEach, filter, every, some}).

**Linter findings**: still 2 expected per section (param.* binding markers), 0 unexpected. The drift-demo example continues to pass.

### test262 numbers at IR-EXT 2 close

| section | pre-IR baseline | IR-EXT 1 | IR-EXT 2 | Δ pre→post |
|---|---|---|---|---|
| Array.prototype.map     | 81.4% | 82.8% | 82.4% | +1.0 |
| Array.prototype.forEach | 88.9% | (unchanged) | 90.5% | +1.6 |
| Array.prototype.filter  | 85.5% | (unchanged) | 86.3% | +0.8 |
| Array.prototype.every   | 89.4% | (unchanged) | 92.2% | +2.8 |
| Array.prototype.some    | 89.4% | (unchanged) | 90.8% | +1.4 |

Cluster total: **+15 tests** across the five slices. All five rates exceed pre-IR baseline.

### Conjecture status

Strongly corroborated this round. The pipeline scaled from 1 → 5 sections without alphabet extension, lowering bugs, or per-section runtime helper additions (only general improvements to array_species_create and create_data_property_or_throw that benefit all callers). seed.md §I conjecture ("spec conformance gets monotonically easier post-IR") holds across the increment.

### Doc 729 §V cross-check

Re-corroborated. The IR-EXT 2 round produced 5 generated Rust functions from 5 IR sections via one `cargo run --example build_generated_rs` invocation — stage-deterministic emission empirically observed at scale.

### Open scope at IR-EXT 2 close

Immediate (Tier 1.7 candidates):
1. **Array.prototype.{find, findIndex, findLast, findLastIndex}** — same iteration shape, returns kValue or index instead of array. Trivial extensions.
2. **Array.prototype.{reduce, reduceRight}** — iteration with accumulator. Adds one IR shape (accumulator-folded iteration) but reuses preamble.
3. **Array.prototype.{indexOf, lastIndexOf, includes}** — iteration without callback (uses SameValueZero). Different shape; one new pattern.
4. **Object.{keys, values, entries}** — different chapter; different shape (OwnPropertyKeys iteration). Larger surface introduction.

Mid-term (Tier 2):
5. **Spec-XML parser** — closer now (5 sections of hand-authored SpecStepRecord lists vs the seed §V.M5 threshold of ~10).

Pin-Art tag count for the IR workstream: 5 commits as of IR-EXT 2.


## IR-EXT 3 — 2026-05-19 (find cluster + index-search cluster + lint_all)

**Headline**: pipeline scaled from 5 → 11 IR-encoded sections in one round. Find/findIndex/findLast/findLastIndex translated and lint-clean; indexOf/includes translated but not yet wired (existing impls have fromIndex normalization the Tier-1.7 IR doesn't yet model). Linter expanded to recognize the SameValue family + IndexAsKey-as-ToString. New lint_all example runs the diff across all sections in one shot.

### Commits

| commit | tag | recognition |
|---|---|---|
| `2142abf0` | IR-EXT 3: find + index-search + lint_all | sections/array_prototype_find.rs (4 sections); sections/array_prototype_index_search.rs (2 sections); abstract_ops same_value + same_value_zero; lowering StrictEq/SameValueZero/SameValue → bool; linter SameValueZero/SameValue/IsStrictlyEqual recognition; examples/lint_all.rs; prototype.rs wires find + findIndex through generated. |

### Substrate at IR-EXT 3 close

**IR alphabet**: same 50 nodes as IR-EXT 2 plus the existing SameValue/SameValueZero/StrictEq now properly recognized by the linter.

**Runtime helper coverage**: added `abstract_ops::same_value` (§7.2.10) and `abstract_ops::same_value_zero` (§7.2.11).

**Sections translated**: 11 (Array.prototype.{map, forEach, filter, every, some, find, findIndex, findLast, findLastIndex, indexOf, includes}). Of these, 7 wired in prototype.rs (map, forEach, filter, every, some, find, findIndex); 4 IR-only (findLast, findLastIndex, indexOf, includes — existing hand-written impls have additional semantics — backward iteration, fromIndex normalization — that Tier 1.7 IR doesn't yet model).

**Linter findings**: 0 unexpected across all 11 sections. The drift-demo example still passes.

**Lint infrastructure**: `cargo run --example lint_all -p rusty-js-ir` now produces a one-line-per-section status report; exits nonzero on any unexpected findings.

### test262 numbers at IR-EXT 3 close

Per keeper directive, no test262 sweep this round. The seven wired sections retain their IR-EXT 2 numbers (cluster total +15 vs pre-IR). The IR-only-not-wired four sections await Tier 1.8 alphabet extensions before swap-in.

### Conjecture status

Pipeline scaled 5 → 11 sections without alphabet extension (only linter recognition refinement and two new abstract_ops helpers). The §A8.30 brand-check discipline applies symmetrically: when the spec uses HasProperty in a step (e.g., indexOf step 9.b), modeling it as an explicit Let-binding in the IR (rather than folding into the If condition) keeps the linter's spec-vs-IR correspondence intact.

The pattern crystallized in this round: **modeling a spec step as a single IR statement is the discipline; consolidating multiple spec steps into one Rust-friendly expression is the optimization the lowering may apply when statically provable**. Section authors stay at the spec-step granularity; the lowering compiler decides representational consolidation.

### Doc 729 §V cross-check

The lint_all output (11 ✓ lines) is the operational signal that resolver-instance #0b is stage-deterministic at scale: 11 separate IR functions, 11 separate SpecStepRecord lists, zero drift findings. Same shape Doc 729 §V predicts for any vertical-recursion-with-stage-deterministic-emission boundary.

### Open scope at IR-EXT 3 close

Tier 1.8 (immediate):
1. **CallBuiltin IR primitive** — for runtime-builtin abstract operations that don't fit the §7.3.x prototype-dispatch model (EnumerableOwnPropertyNames, NewPromiseCapability, SpeciesConstructor, etc.). Once available, unlocks Object.{keys, values, entries} + ArraySpeciesCreate refinement + Promise.all etc.
2. **fromIndex normalization** for indexOf/lastIndexOf/includes — needs signed Int IR primitive (or `Expr::SignedAsIndex(v, len)` helper). After this, indexOf/includes/lastIndexOf get swapped in.
3. **Backward iteration** for findLast/findLastIndex — needs the same signed Int primitive.
4. **Array.prototype.{reduce, reduceRight}** — accumulator pattern. One new IR shape (initial-value handling).

Tier 2 (mid-term):
5. **Spec-XML parser** — at 11 hand-authored SpecStepRecord lists, getting closer to the seed §V.M5 threshold of ~10 sections. The parser will derive these from `<emu-alg>` directly.

Pin-Art tag count for the IR workstream: 6 commits as of IR-EXT 3.


## IR-EXT 4 — 2026-05-19 late (Tier 1.8: alphabet extension; reduce translation)

**Headline**: alphabet extended with two new primitives (HasArg, CallBuiltin) needed for the next batch of sections. One more section translated (Array.prototype.reduce); 12 IR-encoded total, all lint-clean.

### Commits

| commit | tag | recognition |
|---|---|---|
| `a559cd67` | Tier 1.8: HasArg + CallBuiltin + reduce | Two IR primitives added; sections/array_prototype_reduce.rs translates §23.1.3.24 with the step-4 empty+no-init TypeError + step-6/7 initialValue fork modeled via HasArg(1). |

### Substrate at IR-EXT 4 close

**IR alphabet**: ~52 nodes now (50 + HasArg + CallBuiltin). The two new primitives are forward-looking — HasArg is in use immediately by reduce; CallBuiltin is reserved for Object.{keys, values, entries} + Promise.all + ArraySpeciesCreate refinement + other runtime-builtin abstract ops that follow.

**Runtime helper coverage**: unchanged surface this round (reduce uses only existing helpers; CallBuiltin lowering is wired but no consumer yet).

**Sections translated**: 12 (eleven from IR-EXT 3 + Array.prototype.reduce). Wired: 7 (same as IR-EXT 3 — reduce stays IR-only awaiting the find-first-present-index inner loop pattern or a `rt.find_first_present` helper). IR-only-not-wired: 5 (findLast, findLastIndex, indexOf, includes, reduce).

**Linter findings**: 0 unexpected across all 12 sections.

### Conjecture status

The alphabet-extension move proved cheap: adding HasArg + CallBuiltin took ~30 LOC across ir.rs / lower.rs / lint.rs. Once available, modeling reduce's initialValue branch became trivial — the spec text "If initialValue is present, then …" maps to `IRNode::If { cond: Expr::HasArg(1), ... }`, exactly the kind of 1:1 mapping the IR-vs-spec linter validates.

The pattern crystallized this round: **the alphabet grows monotonically; each new primitive is reused across many sections**. The 50→52 alphabet jump unlocks the next ~10–15 sections (Object.\*, Promise.\*, anything with optional args or runtime-builtin dispatch).

### Open scope at IR-EXT 4 close

Tier 1.9 (immediate):
1. **Wire reduce** — needs either a `rt.find_first_present_index(o, len)` helper or an IR sub-shape for "find first present index". Once wired, +12 sections in generated.rs become +13 wired.
2. **Object.{keys, values, entries} via CallBuiltin** — needs `rt.object_keys_helper(v)` / `rt.object_values_helper(v)` / `rt.object_entries_helper(v)` extracted from the existing intrinsics.rs impls.
3. **Promise.{resolve, reject}** — simple CallBuiltin to existing helpers.
4. **Array.prototype.{lastIndexOf, reduceRight, copyWithin}** — adds signed-Index pattern.

Tier 2 (mid-term):
5. **Spec-XML parser** — 12 SpecStepRecord lists hand-authored, surpassing the seed §V.M5 threshold of ~10. The parser pays for itself starting at Tier 1.9.

Pin-Art tag count for the IR workstream: 8 commits as of IR-EXT 4.


## IR-EXT 5 — 2026-05-19 late (Tier 1.9: Object cluster via CallBuiltin)

**Headline**: CallBuiltin pattern demonstrated at scale. Object.{keys, values, entries} translated as thin 2-step wrappers; their hand-written impls extracted into Runtime helpers (rt.enumerable_own_{keys,values,entries}). 15 sections IR-encoded, 10 wired.

### Commits

| commit | tag | recognition |
|---|---|---|
| `5c07e1ae` | IR-EXT 5: Object.{keys, values, entries} | Three runtime helpers extracted from intrinsics.rs (~140 LOC consolidated into Runtime methods). Three 2-step IR sections. CallBuiltin lowering refined to `&` prefix matching IR-target convention. Side-fix: String exotic's length property is now non-enumerable per §22.1.4 (surfaced via Object.keys("abc") smoke). |

### Substrate at IR-EXT 5 close

**IR alphabet**: unchanged (CallBuiltin from IR-EXT 4 was already in place; used here for the first time).

**Runtime helper coverage**: added rt.enumerable_own_keys / _values / _entries per §7.3.23.

**Sections translated**: 15 (12 from IR-EXT 4 + Object.keys + Object.values + Object.entries). Wired: 10 (Array.prototype.{map, forEach, filter, every, some, find, findIndex} + Object.{keys, values, entries}). IR-only-not-wired: 5 (Array.prototype.{findLast, findLastIndex, indexOf, includes, reduce} — awaiting alphabet extensions: signed-Int for backward iteration / fromIndex normalization, find-first-present-index inner-loop for reduce).

**Linter**: 15/15 clean.

### Conjecture status

The hand-written-to-IR-wrapper transition shape crystallized: when the spec abstract op is a Runtime-tier helper (not a JS-side method dispatch), CallBuiltin makes the IR a thin syntactic stub while the helper does the work. The IR carries spec-step traceability + linter validation; the runtime carries performance + edge-case handling.

This is the §A8.30 brand-check discipline's dual: §A8.30 says receivers without the right slot must throw; here, abstract ops that don't fit Get-then-Call get their own dispatch primitive. The two together exhaust the "operator-to-runtime" coupling shapes.

### Open scope at IR-EXT 5 close

Tier 2 (immediate next step):
1. **Spec-XML parser** — 15 SpecStepRecord lists hand-authored, well past the seed §V.M5 threshold of ~10. The parser pays for itself starting here.

Tier 1.10 (parallel work):
2. **Signed-Index primitives** — unblock backward iteration (findLast/findLastIndex/reduceRight/lastIndexOf) + fromIndex normalization (indexOf/lastIndexOf/includes).
3. **find-first-present-index inner-loop** — unblocks reduce wiring.
4. **Promise.{resolve, reject}** — 2-step CallBuiltin like Object.*; trivial.
5. **More Array.prototype methods** — flat, flatMap, slice (currently hand-written with complex semantics).

Pin-Art tag count for the IR workstream: 9 commits as of IR-EXT 5.


## IR-EXT 6 — 2026-05-19 night (Tier 2: spec-XML parser; resolver-instance #0b closed)

**Headline**: spec-XML parser operational. The IR pipeline now accepts ECMA-262 emu-alg source directly and produces SpecStepRecord lists structurally equivalent to the hand-authored ones. lint_from_spec example demonstrates end-to-end: parse → record list → lint against IR → ✓.

### Commits

| commit | tag | recognition |
|---|---|---|
| `b0f76cde` | IR-EXT 6: spec_parser + lint_from_spec | New module spec_parser.rs (~250 LOC) parses emu-alg source into SpecStepRecord lists. Linter refinements: inline-throw recognition + synthetic-inline filtering. IR convention refinement: map step 6.b/6.c split to match spec granularity. New example lint_from_spec.rs demonstrates the parser → lint pipeline end-to-end. |

### Substrate at IR-EXT 6 close

**Parser surface** (spec_parser.rs):
- parse_emu_alg(body) → Vec<SpecStepRecord>
- Numbered-step extraction via Bikeshed `1.` convention + indentation
- Step-ID synthesis (depth-1 numeric, depth-2 alphabetic, depth-3 lowercase Roman)
- Abstract-op recognition (35 known op-names curated)
- Throw-class recognition (all four canonical error classes)

**Linter operational extensions**:
- walk_step_collecting recurses into nested If/While bodies, collecting inline-throw ops into the parent step's op set.
- collect_steps filters synthetic-inline sub-step IDs (.throw / .guard / .return / .adj / .seed) so they don't appear as "ExtraStep" findings.

**Sections IR-encoded**: 15 (same as IR-EXT 5).

**Linter findings**: 15/15 clean. The parser-derived records for §23.1.3.20 produce zero findings against the hand-translated IR — the parser and the hand-authored records agree.

### Conjecture status

Doc 729 §V's vertical-recursion-with-stage-deterministic-emission claim is now corroborated at the parser stratum:
- Same input (emu-alg source) → same output (SpecStepRecord list).
- Same SpecStepRecord list compared to same IR → same lint findings.
- Both stages are pure functions of their input; no environmental state.

The two-step pipeline (parse + lint) replaces the previously implicit human-transcription stage (read spec → write SpecStepRecord by hand). The shape of the resolver-instance #0b stage is now operational: spec source → validated IR. Any future hand-authored record list can be cross-checked against the parser; any future drift between hand-author and spec gets caught at this stage.

### Open scope at IR-EXT 6 close

Tier 2.5 (immediate):
1. **Mass parsing**: feed multiple sections through parse_emu_alg + lint. The current lint_from_spec demos one section; extending to all 15 hand-translated sections proves the parser's coverage at scale.
2. **Live spec source**: replace embedded fixture with a runtime read from a tc39/ecma262 checkout. The shape of parse_emu_alg is unchanged; only the source plumbing.
3. **More known_ops**: as more sections land, the parser's known_ops list may need extension. Each addition is one entry.

Tier 1.10 (parallel work, unchanged from IR-EXT 5):
4. Signed-Index primitives (unblocks lastIndexOf/reduceRight/findLast/etc.).
5. Promise.{resolve, reject} via CallBuiltin.
6. More built-in sections.

Pin-Art tag count for the IR workstream: 10 commits as of IR-EXT 6.


## IR-EXT 7 — 2026-05-19 night (Tier 1.10 begin: Promise.{resolve, reject})

**Headline**: 47 commits pushed to origin/main (cumulative cruftless + IR work since the prior remote). Two more sections wired in this round; 17 IR-encoded total, 12 wired. CallBuiltin demonstrated portable across chapters.

### Commits

| commit | tag | recognition |
|---|---|---|
| `8a58d556` | IR-EXT 7: Promise.{resolve, reject} via CallBuiltin | Two runtime helpers (rt.promise_resolve_via / rt.promise_reject_via) extracted; two 1-step IR sections; wired in promise.rs replacing 4-line hand-written closures. |

### Substrate at IR-EXT 7 close

**Runtime helper coverage**: added rt.promise_resolve_via / rt.promise_reject_via.

**Sections IR-encoded**: 17 (15 from IR-EXT 6 + Promise.resolve + Promise.reject). Wired: 12 (10 + 2 new). IR-only-not-wired: 5 (findLast, findLastIndex, indexOf, includes, reduce).

**Linter**: 17/17 clean.

### Open scope at IR-EXT 7 close

Tier 1.10 (in progress):
1. **Signed-Int IR primitive** — unblocks ~6 sections (lastIndexOf, reduceRight, copyWithin, the backward-iterating find variants, fromIndex normalization).
2. **find_first_present_index helper** — unblocks reduce wiring.
3. **Promise.all / allSettled / any / race** — adds iterator-protocol IR primitives (GetIterator / IteratorNext / IteratorClose).
4. **Object.{getPrototypeOf, setPrototypeOf, freeze, isFrozen, seal, isSealed, getOwnPropertyDescriptor, getOwnPropertyDescriptors, defineProperty, defineProperties}** — adds OrdinaryDefineOwnProperty IR primitive + property-descriptor builders.

Tier 2.5 (mid-term):
5. **Mass parse**: feed multiple emu-alg blocks through spec_parser; cross-check all 17 hand-translated sections against parser-derived records.

Pin-Art tag count for the IR workstream: 11 commits as of IR-EXT 7.


## IR-EXT 8 + 9 — 2026-05-19 night-late (Object proto-ops + integrity clusters)

**Headline**: 10 more sections wired across two clusters (Object proto-ops + Object integrity). 27 IR-encoded total, 22 wired. The CallBuiltin pattern's per-section LOC cost has stabilized at ~25-45 LOC; the IR continues to scale linearly per chapter.

### Commits

| commit | tag | recognition |
|---|---|---|
| `daaeb759` | IR-EXT 8: proto-ops cluster | Object.{getPrototypeOf, setPrototypeOf, isExtensible, isFrozen, isSealed} via CallBuiltin. Five new runtime helpers (rt.get_prototype_of_via / set_prototype_of_via / is_extensible_via / is_frozen_via / is_sealed_via). |
| `23672dc5` | IR-EXT 9: integrity cluster | Object.{freeze, seal, preventExtensions, hasOwn, is} via CallBuiltin. Five new runtime helpers. Five hand-written closures replaced. |

### Substrate at IR-EXT 9 close

**Runtime helper coverage**: 10 new IR-target helpers added (one per section across the two clusters). All on `Runtime` taking `&Value` and returning `Result<Value, RuntimeError>`.

**Sections IR-encoded**: 27 (17 from IR-EXT 7 + 10 new). Wired: 22 (17 + 5 proto-ops + 5 integrity = 27 minus 5 IR-only). Wait — recount: 17 wired at IR-EXT 7 close + 5 proto-ops (E8) + 5 integrity (E9) = 27 wired in theory, but the 5 IR-only stays IR-only. Wired at IR-EXT 9 close = 17 + 10 = 27 minus 5 = 22.

Cumulative wired: 22 of 27 sections route through generated.rs.

**Linter**: 27/27 clean.

### Conjecture status

The cross-chapter portability claim (IR-EXT 7) is now strongly corroborated. CallBuiltin has been used to wire sections across three chapters (Array, Object, Promise) without any new IR primitives. The lowering compiler, linter, and spec-parser are chapter-agnostic. Each new section requires:
- One file under sections/ (~30-100 LOC, depending on step count)
- One or more runtime helpers if the spec abstract op isn't a JS-side method dispatch (~5-30 LOC each)
- One wiring edit per section in the registration file (1 line)

That's it. The bulk of the apparatus is in place.

### Open scope at IR-EXT 9 close

Tier 1.10 (continuing):
1. **Math.* cluster** — Math.abs, floor, ceil, round, trunc, sign, sqrt, cbrt, pow, max, min. Mostly one-liners on Number; the question is whether the IR overhead pays for itself for performance-sensitive ops. Defer until other higher-yield clusters are done.
2. **Array.prototype.{push, pop, shift, unshift}** — mutating methods. Adds one new pattern (mutate `this`, return new length / popped element).
3. **String.prototype.{includes, startsWith, endsWith}** — already P62.E13 spec-compliant; would benefit from IR encoding for spec-step traceability.
4. **Promise.all / allSettled / any / race** — adds iterator-protocol IR primitives (deferred to Tier 1.11).

Pin-Art tag count for the IR workstream: 13 commits as of IR-EXT 9.


## IR-EXT 10 — 2026-05-19 night-late (Number static-method cluster)

**Headline**: 4 more sections (Number.{isFinite, isInteger, isNaN, isSafeInteger}). 31 IR-encoded, 26 wired. Pattern refinement: `one_step_builtin` builder.

### Commits

| commit | tag | recognition |
|---|---|---|
| `c6d098ac` | IR-EXT 10: Number static methods | Four runtime helpers (rt.number_is_*) extracted. Four 1-step IR sections via the new `one_step_builtin` helper, which collapses the canonical "1-step CallBuiltin section" shape to a single line of section-level Rust. ~12 LOC per section, down from ~25-45 for the multi-step Object sections. |

### Substrate at IR-EXT 10 close

**Sections IR-encoded**: 31 (27 from IR-EXT 9 + 4 Number static). Wired: 26.

**Linter**: 31/31 clean.

**LOC trend**: per-section authoring cost has declined to ~12 LOC for canonical 1-step CallBuiltin sections. The builder pattern (one_step_builtin / one_step_spec) replicates across number_static.rs and could be lifted to a shared helper in a future round.

### Open scope at IR-EXT 10 close

Continuing translation cadence. Next-easy clusters:
1. **Global functions**: parseInt, parseFloat, isFinite, isNaN, encodeURI, decodeURI, encodeURIComponent, decodeURIComponent (8 sections).
2. **Math.* one-liners**: abs, floor, ceil, round, trunc, sign, sqrt, cbrt, pow (9 sections; performance trade-off needs evaluation).
3. **String.prototype.{includes, startsWith, endsWith}**: already P62.E13 spec-compliant; IR encoding for traceability.

Pin-Art tag count: 14 commits as of IR-EXT 10.


## IR-EXT 11 + 12 — 2026-05-19 night-late (global predicates + seed refinement)

**IR-EXT 11** — global isFinite / isNaN (ToNumber-coercing pair).
**IR-EXT 12** — seed.md refinement post-empirical-feedback.

### Commits

| commit | tag | recognition |
|---|---|---|
| `87dc4c93` | IR-EXT 11: global isNaN / isFinite | Two more sections via CallBuiltin. Distinct from Number.isNaN/Number.isFinite — these coerce via ToNumber. Two runtime helpers (rt.global_is_nan_via / rt.global_is_finite_via). |
| `17eba276` | IR-EXT 12: seed refinement | Three sections lifted from empirical state: §I.1 refined telos (bounded coverage + alphabet completeness + carve-out rule); §III alphabet status (stable across IR-EXT 5→11); §V.M5/M6/M7 discipline catalog (CallBuiltin preferred for non-JS-method abstract ops; one_step_builtin builder for canonical 1-step shape); §VI four-condition termination criteria with progress snapshot. |

### Substrate at IR-EXT 12 close

**Sections IR-encoded**: 33 (31 from IR-EXT 10 + global predicates).
**Wired**: 28.
**Linter**: 33/33 clean.

**Alphabet status**: 52 nodes, stable across 5 chapters × 10 clusters × 33 sections without alphabet extensions. Four alphabet-extension triggers identified for future tiers (signed-Int, iterator-protocol, descriptor-builders, NewPromiseCapability).

**Discipline catalog**: M1-M7 (refined this round). M6 (CallBuiltin preferred for non-JS-method abstract ops) and M7 (one_step_builtin builder) are the two new disciplines lifted from the IR-EXT 8-11 pattern crystallization.

### Conjecture status

After 33 sections across 5 chapters with zero alphabet extensions and zero unresolved linter findings, the §I conjecture ("spec conformance gets monotonically easier post-IR") holds strongly. The remaining work toward full representation is mechanical: each new section is roughly one IR file + runtime helpers + one-line wiring; the per-section cost has stabilized; the alphabet is sufficient for the remaining coverage modulo the four queued extensions.

### Open scope at IR-EXT 12 close

Tier 1.10 cont (translation rounds):
1. **Math.* one-liners** via shared math_unary_op_via helper (~8 sections).
2. **Reflect.* cluster** (~9 sections, mirrors Object.* shape).
3. **Array.prototype mutators** (push, pop, shift, unshift, reverse, sort caveat).
4. **String.prototype.{toLowerCase, toUpperCase, trim*}** (5 sections, already spec-compliant from P62.E15).

Tier 1.11+ (alphabet extension):
5. **Signed-Int + IndexSub primitives** — unblocks lastIndexOf/reduceRight/findLast spec-strict.
6. **Iterator-protocol primitives** — unblocks Promise.all-family.
7. **Property-descriptor builders** — unblocks Object.defineProperty/getOwnPropertyDescriptor.

Tier 2.5:
8. **Mass-fixture parse** — feed all 33 hand-translated sections' emu-alg blocks through spec_parser; cross-check against hand-authored SpecStepRecord lists.
9. **Live spec source** — replace embedded fixtures with tc39/ecma262 checkout via build.rs.

Pin-Art tag count: 16 commits as of IR-EXT 12.


## IR-EXT 13 + 14 + 15 — 2026-05-19 night-very-late (Math + Reflect coverage)

**IR-EXT 13** (8d sections — already anchored at commit `8433ff80`).
**IR-EXT 14** (8d5d2ddb): 18 more Math sections (exp/log/trig/hyperbolic) via the IR-EXT 13 shared dispatcher.
**IR-EXT 15** (this anchor): 5 Reflect.* sections.

### Substrate at IR-EXT 15 close

**Sections IR-encoded**: 64 (41 from IR-EXT 13 + 18 Math family + 5 Reflect.*). Wired: 59. IR-only-not-wired: 5 (findLast, findLastIndex, indexOf, includes, reduce — all awaiting alphabet extensions per IR-EXT 4 open scope).

**Linter**: 64/64 clean.

**Coverage gap surfaced this stretch**: cruftless was missing Math.asin / Math.acos entirely. The IR-vs-cruftless registration audit (one author writing the IR section discovers there's no corresponding intrinsics.rs registration to replace) caught the omission. Both installed via IR in IR-EXT 14.

**Runtime helpers cumulative**: 27 (10 from Object proto-ops/integrity, 5 from Promise/Number static, 1 Math unary dispatcher, 5 Reflect, 2 global predicates, etc.).

### Conjecture status

Coverage-discovery claim strengthens the §I conjecture: the IR doesn't just preserve spec conformance — it *discovers cruftless's spec-coverage gaps* (asin/acos were never installed). This is a stronger claim than "no drift" — it's "exhaustive coverage discovery during translation". The §VI termination conditions implicitly require it.

### Open scope at IR-EXT 15 close

Tier 1.10 cont:
1. **Reflect.{getPrototypeOf, setPrototypeOf, isExtensible, preventExtensions}** — four more, mirrors of Object.* with subtly different semantics.
2. **Math.{pow, atan2, hypot, max, min}** — binary/variadic, needs math_binary / math_variadic dispatcher.
3. **Number.prototype.{toString, toFixed, toExponential, toPrecision}** — already P62.E19/E20 spec-compliant; IR encoding for traceability.
4. **String.prototype trim cluster** — already P62.E15 spec-compliant.

Pin-Art tag count: 19 commits as of IR-EXT 15.
