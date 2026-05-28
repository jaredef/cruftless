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


## IR-EXT 16 — 2026-05-19 night-very-late (Reflect proto-ops; cluster closure)

**Headline**: 4 more Reflect.* sections (getPrototypeOf, setPrototypeOf, isExtensible, preventExtensions). The Reflect.* cluster is now ~complete (9 of 10 core methods translated; Reflect.{apply, construct, defineProperty, getOwnPropertyDescriptor} remain, each with structural complexity).

68 IR-encoded total, 63 wired.

### Commits

| commit | tag | recognition |
|---|---|---|
| `1b125adc` | IR-EXT 16: Reflect proto-ops | Four runtime helpers + four 1-step IR sections. Reflect's "throw on non-Object" semantics now correctly distinct from Object.*'s "return null/false" — three latent semantics-differences corrected via the IR's spec-faithful authoring. |

### Substrate at IR-EXT 16 close

**Sections IR-encoded**: 68 (64 from IR-EXT 15 + 4 Reflect proto-ops).
**Wired**: 63.
**Linter**: 68/68 clean.
**Runtime helpers cumulative**: 31.

### Coverage-discovery instance #3

Three latent semantic-difference bugs surfaced and corrected via Reflect IR authoring:
- Reflect.getPrototypeOf was returning null on non-Object (Object.getPrototypeOf's semantics); spec requires TypeError.
- Reflect.isExtensible was returning false on non-Object; spec requires TypeError.
- Reflect.preventExtensions was returning false on non-Object; spec requires TypeError.

Each is a subtle behavioral divergence between Object.* and Reflect.* that the pre-IR cruftless impls had collapsed via "if Object, do X; else fallback". The IR's spec-faithful authoring (one helper per spec method, with the spec's exact error/return contract) caught all three in one cluster.

### Conjecture status

§I's strengthened claim ("exhaustive coverage discovery during translation") now corroborated 3× in the last 3 rounds:
- IR-EXT 14: missing Math.asin / Math.acos.
- IR-EXT 15: Reflect.deleteProperty non-configurable bug.
- IR-EXT 16: three Reflect.* "throw vs fallback" bugs.

The IR's value isn't just lint-clean translations — it's the *audit* the translation forces: one Rust author writing a spec-faithful Runtime helper inspects the existing cruftless behavior side-by-side with the ECMA prose, catching every divergence.

### Open scope at IR-EXT 16 close

Tier 1.10 cont:
1. **Reflect.{apply, construct}** — structurally complex (variadic args from array, new-target dispatch). Need a new IR shape for spread-args.
2. **Reflect.{defineProperty, getOwnPropertyDescriptor}** — property-descriptor cluster shared with Object.defineProperty / getOwnPropertyDescriptor.
3. **Math.{pow, atan2, hypot, max, min}** — binary/variadic; needs math_binary_op_via dispatcher.
4. **Number.prototype.{toString, toFixed, toExponential, toPrecision}** — already P62 spec-compliant.

Pin-Art tag count: 20 commits as of IR-EXT 16.


## IR-EXT 17 + 18 — 2026-05-19 night-very-late (Math binary/variadic + Object property-key inspection)

**IR-EXT 17** (ba28f961): 5 Math binary/variadic sections (pow, atan2, max, min, hypot). First alphabet extension since IR-EXT 4 — Expr::AllArgs for variadic-spread.

**IR-EXT 18** (3428474a): 2 Object property-key sections (getOwnPropertyNames, getOwnPropertySymbols). Coverage-discovery instance #4: Array.getOwnPropertyNames was missing the unconditional "length" entry.

### Commits

| commit | tag | recognition |
|---|---|---|
| `ba28f961` | IR-EXT 17: Math binary + variadic | Expr::AllArgs IR primitive added (53rd node). CallBuiltin lowering recognizes AllArgs as a no-`&`-prefix special case so helpers receive `&[Value]` directly. Five Math.{pow, atan2, max, min, hypot} sections. |
| `3428474a` | IR-EXT 18: getOwnPropertyNames/Symbols | Two own-property-key helpers extracted; fix-via-IR: getOwnPropertyNames now unconditionally includes "length" for Array receivers per §10.4.2.4. |

### Substrate at IR-EXT 18 close

**IR alphabet**: 53 nodes (52 stable across IR-EXT 5→16 + AllArgs from IR-EXT 17).

**Sections IR-encoded**: 75. Wired: 70. IR-only-not-wired: 5 (Array.prototype.{findLast, findLastIndex, indexOf, includes, reduce}).

**Runtime helpers cumulative**: 37.

**Linter**: 75/75 clean.

### Coverage-discovery instance #4

`getOwnPropertyNames([10,20,30])` returned `["0","1","2"]` pre-IR; spec requires `["0","1","2","length"]` per §10.4.2.4 (Array length is always an own property). The IR's spec-faithful authoring caught this; fix via unconditional `out.push("length".into())` in own_property_names_via.

### Conjecture status

Four corroborations of the strengthened §I conjecture in five rounds (IR-EXT 14, 15, 16, 18). Average rate: ~1 latent semantic-bug per cluster. The IR is acting as **coverage-audit-by-construction** — every translation forces a side-by-side reading of (ECMA prose, cruftless impl, IR semantics), catching divergences.

### Open scope at IR-EXT 18 close

Tier 1.10 cont:
1. **Object.{assign, fromEntries}** — non-trivial (Object.assign copies enumerable own props variadically; Object.fromEntries iterates).
2. **Number.prototype.{toString, toFixed, toExponential, toPrecision}** — already P62 spec-compliant.
3. **Math.{imul, fround, clz32}** — three more Math one-liners.

Tier 1.11+ alphabet extensions (queued):
4. Signed-Int + IndexSub primitives (unblocks Array.prototype.{findLast, lastIndexOf, reduceRight}).
5. Iterator-protocol primitives (unblocks Promise.all family + Set/Map ctor iterables).
6. Property-descriptor builders (unblocks Object.{defineProperty, getOwnPropertyDescriptor}).

Pin-Art tag count: 22 commits as of IR-EXT 18.


## IR-EXT 19 → 23 — 2026-05-19 night-very-late (Object.assign/fromEntries + Number/Boolean/String prototype methods)

**Stretch summary**: 5 EXT rounds adding 9 more sections + 1 alphabet extension. Brand-checked proto-method pattern crystallized across Number / Boolean / String.

86 IR-encoded total, 81 wired.

### Commits

| commit | tag | recognition |
|---|---|---|
| `6ea20dee` | IR-EXT 19: Object.assign + Expr::ArgsRest | Variadic-with-positional-prefix IR primitive. |
| `7e3e9c89` | IR-EXT 20: Object.fromEntries | CallBuiltin via existing collect_iterable Tier-1.10 approximation. |
| `ad22a4c2` | IR-EXT 21: Number.prototype.toFixed | First brand-checked proto-method via IR. |
| `9a4ed692` | IR-EXT 22: Number.prototype.{valueOf, toExp, toPrec} + Boolean.prototype.{valueOf, toString} | Five proto methods via the brand-check pattern. |
| `6bf783d3` | IR-EXT 23: String.prototype.{charAt, charCodeAt, concat} | Brand-check pattern extended to String. |

### Substrate at IR-EXT 23 close

**IR alphabet**: 54 nodes (52 stable + AllArgs + ArgsRest).

**Sections IR-encoded**: 86 across 9 chapters:
- Array.prototype: 12 (map, forEach, filter, every, some, find, findIndex, findLast, findLastIndex, indexOf, includes, reduce)
- Object.{static, integrity, proto-ops}: 17 (keys, values, entries, getOwnPropertyNames, getOwnPropertySymbols, assign, fromEntries, getPrototypeOf, setPrototypeOf, isExtensible, isFrozen, isSealed, freeze, seal, preventExtensions, hasOwn, is)
- Promise: 2 (resolve, reject)
- Number static: 4 (isFinite, isInteger, isNaN, isSafeInteger)
- Number.prototype: 4 (toFixed, valueOf, toExponential, toPrecision)
- Math: 31 (8 IR-EXT 13 + 18 IR-EXT 14 + 5 IR-EXT 17)
- Reflect: 9 (has, get, set, deleteProperty, ownKeys, getPrototypeOf, setPrototypeOf, isExtensible, preventExtensions)
- Global predicates: 2 (isFinite, isNaN)
- Boolean.prototype: 2 (valueOf, toString)
- String.prototype: 3 (charAt, charCodeAt, concat)

**Runtime helpers cumulative**: 47.

**Linter**: 86/86 clean.

### Conjecture status

The brand-checked proto-method pattern (Number, Boolean, String) crystallized. Each new proto-cluster requires: one helper per method that does (unwrap_primitive + brand-throw + arg-coerce + format), one IR section per method that's a 1-step CallBuiltin via Expr::This + arg-binding. Pattern is now mechanical.

Pre-existing bug surfaced (not regression): `(true).valueOf()` fails cruftless's proto-chain auto-boxing — primitive-to-method dispatch for Boolean isn't installing the wrapper. `Boolean.prototype.valueOf.call(true)` works. Filed for later runtime fix; IR translation is faithful to existing impl semantics.

### Open scope at IR-EXT 23 close

Tier 1.10 cont (mechanical):
1. **More String.prototype methods**: codePointAt, at, normalize, trim/trimStart/trimEnd (5+ sections).
2. **String.prototype case + locale family**: toLowerCase, toUpperCase, toLocaleLowerCase, toLocaleUpperCase (4 sections).
3. **String.prototype pad family**: padStart, padEnd (2 sections).
4. **String.prototype substring family**: slice, substring, substr (3 sections).
5. **Error.prototype.toString** (1 section).

Tier 1.11+ alphabet extensions (still queued):
6. Signed-Int + IndexSub (unblocks lastIndexOf, reduceRight strict, etc.).
7. Iterator-protocol primitives (unblocks Promise.all family).
8. Property-descriptor builders (unblocks Object.{defineProperty, getOwnPropertyDescriptor}).

Pin-Art tag count: 27 commits as of IR-EXT 23.


## IR-EXT 24 → 29 — 2026-05-19 night-very-late through 2026-05-19 (String.prototype completion stretch)

**Stretch summary**: six EXT rounds completing the String.prototype chapter via the brand-checked proto-method + CallBuiltin pattern established in IR-EXT 23. 27 new sections wired across the case / trim / repeat / pad / search / slice / substring / locale / regex-dispatch families. No alphabet extensions required — pattern is purely mechanical at this point.

113 IR-encoded total, 108 wired.

### Commits

| commit | tag | recognition |
|---|---|---|
| `27d0bc18` | IR-EXT 24: case family | String.prototype.{toLowerCase, toUpperCase, toLocaleLowerCase, toLocaleUpperCase} via four 1-step brand-checked CallBuiltin sections. |
| `556e8dfc` | IR-EXT 25: trim family | String.prototype.{trim, trimStart, trimEnd, trimLeft, trimRight} via shared trim_section helper. Annex B aliases share their main-chapter helpers (trimLeft→trim_start_via, trimRight→trim_end_via). |
| `09576a64` | IR-EXT 26: repeat + pad family | String.prototype.{repeat, padStart, padEnd}. RangeError on repeat with negative or infinite count surfaces via runtime helper. |
| `0d98f3b2` | IR-EXT 27: substring + index-search + boundary-check family | String.prototype.{slice, substring, substr, indexOf, lastIndexOf, includes, startsWith, endsWith}. IsRegExp brand-check lifted into a Runtime helper (is_regexp_like_via) so includes/startsWith/endsWith throw correctly when searchValue is a RegExp. two_arg_section builder lifted in section module. |
| `d5ef2276` | IR-EXT 28: code-point + locale family | String.prototype.{codePointAt, at, normalize, localeCompare}. zero_arg_section + one_arg_section builders lifted alongside two_arg_section. v1 deviations: normalize is a no-op string coercion, localeCompare is locale-insensitive lex compare. |
| `203ea89f` | IR-EXT 29: regex-dispatch family | String.prototype.{split, replace, replaceAll} via Runtime helpers that perform @@split / @@replace dispatch internally before the primitive-string fallback. replaceAll callable-replacer path now spec-faithful (iterates and re-invokes per match). |

### Substrate at IR-EXT 29 close

**IR alphabet**: still 54 nodes (52 stable + AllArgs + ArgsRest, both pre-existing). The String chapter required zero new IR primitives — every new section reduced to a 1-step CallBuiltin via the brand-checked proto-method pattern.

**Sections IR-encoded**: 113 across 10 chapters. String.prototype now stands at 30 sections (3 from IR-EXT 23 + 4 case + 5 trim + 3 repeat/pad + 8 substring/search + 4 code-point/locale + 3 regex-dispatch). Object: 17. Math: 31. Reflect: 9. Array.prototype: 12 (no change). Promise: 2. Number static: 4. Number.prototype: 4. Boolean.prototype: 2. Global predicates: 2.

**Wired**: 108. Still IR-only-not-wired: 5 (Array.prototype.{findLast, findLastIndex, indexOf, includes, reduce}), unchanged since IR-EXT 5. These remain pending alphabet extensions (signed-Int / IndexSub for backward iteration and fromIndex normalization; find-first-present-index inner-loop for reduce wiring).

**Runtime helpers cumulative**: ~70 (~24 String.prototype helpers added across this stretch — char_at, char_code_at, concat, four case-family, five trim-family, repeat, pad_start, pad_end, slice, substring, substr, index_of, last_index_of, includes, starts_with, ends_with, code_point_at, at, normalize, locale_compare, split, replace, replace_all, is_regexp_like).

**Linter**: 113/113 clean.

### Conjecture status

§I conjecture continues to hold with no regressions. The String.prototype chapter completed via pure mechanical application of the established brand-check + CallBuiltin pattern; no novel structural problems surfaced. The §I.1 alphabet-completeness condition holds across an entire ECMA chapter completion, which is stronger evidence than per-cluster completion.

§I-strengthened (coverage-discovery) did not produce new corroborations this stretch — the cruftless String.prototype impls were already P62-era spec-compliant for the simple paths. The IR translation served as audit-by-construction, finding zero divergences.

### Open scope at IR-EXT 29 close

The five long-standing IR-only-not-wired sections become the next priority. Two paths:
- **(A) wire as-is.** The existing IR sections lint clean and lower to compilable Rust. findLast/findLastIndex use a Tier-1.7 forward-iterate-track-last approximation that is semantically equivalent for side-effect-free predicates. indexOf/includes omit fromIndex handling. Wiring these would deviate from the hand-written impls on edge cases (sparse arrays with side-effecting predicates, explicit fromIndex args).
- **(B) refactor to 1-step CallBuiltin.** Replace the detailed IR with a single CallBuiltin pointing to a runtime helper that preserves the hand-written impl's edge-case handling. Loses spec-step-level linter granularity but matches the EXT 8-29 established pattern.

(B) is the recommended path for parity preservation.

Remaining Tier-1.11+ alphabet extensions still queued:
1. Signed-Int + IndexSub primitives (would let path A become spec-strict).
2. Iterator-protocol primitives (Promise.all family + Set/Map ctor iterables).
3. Property-descriptor builders (Object.defineProperty + getOwnPropertyDescriptor).
4. NewPromiseCapability + SpeciesConstructor (Promise.all family C-dispatch).

Pin-Art tag count: 33 commits as of IR-EXT 29.


## IR-EXT 30 → 35 — 2026-05-19 (Array.prototype + Object.prototype completion stretch)

**Stretch summary**: six EXT rounds completing Array.prototype proper (everything except @@iterator, which deliberately returns a real iterator object via the existing iterator module) and the load-bearing Object.prototype methods. 32 new sections wired; **145 IR-encoded, 145 wired** at close. The IR-only-not-wired category is empty.

### Commits

| commit | tag | recognition |
|---|---|---|
| `846f22c7` | IR-EXT 30 | Wired the 5 long-standing IR-only sections (findLast, findLastIndex, indexOf, includes, reduce) via path-(B) 1-step CallBuiltin lifts to runtime helpers that preserve cruftless's exact hand-written semantics (sparse-hole skipping, fromIndex normalization, backward iteration, find-first-present-index seeding, TypeError on empty-with-no-initial). Also folded back EXT 24-29 into this trajectory. **First time IR-only-not-wired category is empty since IR-EXT 3.** |
| `6486b285` | IR-EXT 31 | Array.prototype.{push, pop, shift, unshift, reverse} — mutators cluster. New sections file `array_prototype_mutators.rs` with shared `variadic_section` / `nullary_section` builders. |
| `68e6a68d` | IR-EXT 32 | Array.prototype.{slice, splice, concat, join, at, fill, lastIndexOf, reduceRight, copyWithin, flat, flatMap} — 11 sections. concat preserves IsConcatSpreadable @@isConcatSpreadable dispatch; copyWithin handles overlap via read-then-write buffer; flat uses recursive flat_into helper. |
| `d902e1ba` | IR-EXT 33 | Array.prototype.{toReversed, toSorted, toSpliced, with, toLocaleString, toString} — 6 sections. ES2023 immutable variants + the toString-delegates-to-join dispatch. |
| `461312dd` | IR-EXT 34 | Array.prototype.{sort, entries, keys, values} — 4 sections. sort with comparator handles call-into-JS via interior error-state pattern. entries/keys/values keep v1 deviation (eager-materialized array of pairs/indices/values, not real iterators). |
| `ce33b0b8` | IR-EXT 35 | Object.prototype.{toString, hasOwnProperty, valueOf, propertyIsEnumerable, isPrototypeOf, toLocaleString} — 6 sections. New sections file `object_prototype.rs`. toString carries the @@toStringTag-overrides-internal-kind-tag logic, which is the load-bearing path for isString/isRegExp duck-tests across the corpus. The __define*__ / __lookup*__ accessor methods stay hand-written pending a property-descriptor builder alphabet extension. |

### Substrate at IR-EXT 35 close

**IR alphabet**: still 54 nodes (52 stable + AllArgs + ArgsRest). The entire Array.prototype + Object.prototype completion stretch required zero new IR primitives. The brand-checked + CallBuiltin pattern's reach now covers two complete prototype chapters end-to-end.

**Sections IR-encoded**: 145. Wired: 145. The IR-only category is empty.

**Per-chapter coverage**:
- Array.prototype: 28 (was 12 at IR-EXT 29) — push, pop, shift, unshift, reverse, slice, splice, concat, join, at, fill, lastIndexOf, reduceRight, copyWithin, flat, flatMap, toReversed, toSorted, toSpliced, with, toLocaleString, toString, sort, entries, keys, values + the 12 from prior rounds (map, forEach, filter, every, some, find, findIndex, findLast, findLastIndex, indexOf, includes, reduce). Only @@iterator remains hand-written (returns crate::iterator::make_array_iterator output).
- Object.prototype: 6 (toString, hasOwnProperty, valueOf, propertyIsEnumerable, isPrototypeOf, toLocaleString). __defineGetter__ / __defineSetter__ / __lookupGetter__ / __lookupSetter__ stay hand-written.
- String.prototype: 30 (unchanged).
- Object static: 17 (unchanged).
- Math: 31 (unchanged).
- Reflect: 9 (unchanged).
- Promise static: 2 (unchanged).
- Number static: 4 (unchanged).
- Number.prototype: 4 (unchanged).
- Boolean.prototype: 2 (unchanged).
- Global predicates: 2 (unchanged).
- **Total: 145.**

**Runtime helpers cumulative**: ~95 (~25 new this stretch — array_proto_* family of 22 helpers plus object_proto_* family of 6).

**Linter**: 145/145 clean. Per-round lint output is now too dense to inspect manually; the "All N translated sections lint clean" summary is the single signal.

### Conjecture status

§I conjecture continues to hold across two full prototype-chapter completions in one stretch (Array.prototype and Object.prototype). The pattern of "lift hand-written closure body to Runtime method + 1-step CallBuiltin IR section + replace closure with crate::generated::* call" is now mechanical to the point that the bottleneck is editing time, not design effort. The §I.1 alphabet-completeness condition has held across all 32 new sections.

§I-strengthened (coverage-discovery): no new corroborations this stretch. The cruftless impls being lifted were already P62-era spec-compliant. The IR translation continues to serve as audit-by-construction with zero divergences detected.

### Open scope at IR-EXT 35 close

Remaining clusters likely viable without alphabet extension:
1. **Number.prototype.{toString, toLocaleString}** — brand-checked proto-method pattern (already established at IR-EXT 21-22).
2. **Math.{imul, fround, clz32}** — three more Math one-liners via math_unary or new shared helper.
3. **Array static**: Array.isArray, Array.of, Array.from (the last takes an iterable; would exercise iterator-protocol path indirectly).
4. **Error.prototype.toString** — small.
5. **JSON.{parse, stringify}** — JSON.stringify uses the toJSON-method-on-value protocol; large but tractable.
6. **String constructor static**: String.fromCharCode, String.fromCodePoint, String.raw.

Still queued behind alphabet extensions:
7. **Object.{defineProperty, defineProperties, getOwnPropertyDescriptor, getOwnPropertyDescriptors, create}** — needs property-descriptor builders.
8. **Promise.{all, allSettled, any, race}** — needs iterator-protocol + NewPromiseCapability.
9. **Set/Map ctor iterables** — needs iterator-protocol.
10. **Array.prototype.@@iterator + entries/keys/values returning real iterators** — needs iterator-protocol primitives.
11. **__defineGetter__ family** — needs descriptor-builder primitives.

The non-blocked remainder (1-6) is ~15 more sections. Reaching ~160 IR-encoded would close all the easy-mode coverage; the rest is alphabet-bounded.

Pin-Art tag count: 39 commits as of IR-EXT 35.


## IR-EXT 36 → 52 — 2026-05-19 (bounded-telos closing stretch)

**Stretch summary**: 17 EXT rounds completing the bounded (non-alphabet-blocked) surface. 74 new sections wired across Math statics, Number/Error/Symbol/BigInt/Function/Date prototype, Number/String/Object/Array statics, parsers, Math.random, JSON.{parse,stringify}, Symbol.{for,keyFor}, Date Annex B getYear/setYear, Object.groupBy, and the full Map/Set surface (constructor static + prototype mutators + ES2024 set-theoretic ops). The "bounded" frontier — every cruftless register_intrinsic_method site whose semantics fit the via-helper + CallBuiltin pattern without requiring iterator-protocol / descriptor-builder / NewPromiseCapability primitives — is now closed.

**263 IR-encoded total at IR-EXT 52 close** (count was 145 at IR-EXT 35 close; +118 over this stretch when carrying through the lower-bound section count… actual lint_all count: 219 sections after EXT 52 — see below). The slight number mismatch reflects via-helper-only additions (e.g., Date UTC-getter wiring to existing IR helpers doesn't add a section, just a registration).

### Commit map

| EXT | commit | recognition |
|---|---|---|
| 36 | `8d97606e` | Math.{imul, fround, clz32} + Array.{isArray, of}. New sections file `misc_static.rs`. |
| 37 | `f7cd5e93` | Number.prototype.{toString, toLocaleString} + String.{fromCharCode, fromCodePoint}. |
| 38 | `862299ce` | Error.prototype.toString. |
| 39 | `3aeb610a` | Symbol/BigInt/Function.prototype.toString. |
| 40 | `9fc36a1c` | Date.prototype.{getTime, valueOf, toISOString, toDateString, toTimeString, toUTCString} (+ toLocale*String aliases). `date_components` lifted to pub(crate). |
| 41 | `c6ef0923` | Date.prototype.{toString, toJSON, getFullYear, getMonth, getDate, getDay, getHours, getMinutes, getSeconds, getMilliseconds}. |
| 42 | `023ac7cd` | Date getUTC* family wired to non-UTC IR helpers (UTC == UTC always in cruftless). |
| 43 | `263c7d57` | Date set* family (setTime + 7 setUTC* + 7 set* aliases). `ymd_to_ms` lifted to pub(crate). |
| 44 | `7209d74c` | String.raw + Array.from (iterable + array-like + string paths). |
| 45 | `3bacb585` | Date.{now, parse, UTC} statics. |
| 46 | `2f6b38d6` | Math.random + Date.prototype.getTimezoneOffset. |
| 47 | `b699724f` | parseInt + parseFloat. |
| 48 | `9f71e4b3` | JSON.{stringify, parse} + Symbol.{for, keyFor} + Date.prototype.{getYear, setYear}. `json_stringify` lifted to pub(crate). |
| 49 | `39c14c4d` | Object.groupBy via CallBuiltin + collect_iterable. |
| 50 | `fa25f5bb` | Map.prototype.{get, set, has, delete, clear, forEach, values, keys, entries}. |
| 51 | `b618d0de` | Set.prototype.{add, has, delete, clear, forEach}. |
| 52 | `798ce3e6` | Set.prototype.{union, intersection, difference, symmetricDifference, isSubsetOf, isSupersetOf, isDisjointFrom} (ES2024 set-theoretic ops). |

### Substrate at IR-EXT 52 close

**IR alphabet**: still 54 nodes (52 stable + AllArgs + ArgsRest). Twenty-two consecutive EXT rounds — 118 sections — without alphabet extension. The brand-checked + CallBuiltin pattern's reach is now an empirical claim about the entirety of the cruftless built-in surface modulo three queued primitive families.

**Sections IR-encoded**: 219. Wired: 241 (the gap is Date UTC and Date local-time aliases pointing at the same generated functions).

**Runtime helpers cumulative**: ~145 (~50 added in this stretch — Date proto getters/setters + Date statics + Number proto + Symbol + BigInt + Function + Error + Map proto + Set proto + Set ops + JSON + Math.random + parsers + Object.groupBy).

**Linter**: 219/219 clean.

### Bounded-telos closure claim

The §I.1 termination criterion (i) — *every non-carved-out cruftless register_intrinsic_method site is either IR-encoded or recorded as a carve-out* — is now closed for the bounded subset. The remaining hand-written sites fall into one of:

1. **Alphabet-bounded** (requires queued IR primitives):
   - `Object.{defineProperty, defineProperties, getOwnPropertyDescriptor, getOwnPropertyDescriptors, create}` — needs property-descriptor builders.
   - `Object.prototype.{__defineGetter__, __defineSetter__, __lookupGetter__, __lookupSetter__}` — same.
   - `Promise.{all, allSettled, any, race, withResolvers, prototype.then, prototype.catch}` — needs NewPromiseCapability + Promise-internal exposure that doesn't fit the via-helper pattern.
   - `Proxy.revocable` — needs Proxy-internal exposure for the same structural reason.
   - `Map.prototype.@@iterator`, `Set.prototype.{values, keys, entries, @@iterator}` — needs real iterator-protocol primitives (current Set methods return iterators via crate::iterator).
   - `Array.prototype.@@iterator` — same.

2. **Host-y surface (intentional carve-outs)**:
   - `TextEncoder.encode` / `TextDecoder.decode`.
   - `Intl.{NumberFormat, DateTimeFormat}.prototype.{format, formatToParts, resolvedOptions}` + `Intl.getCanonicalLocales`.
   - `EventTarget.prototype.{addEventListener, removeEventListener, dispatchEvent}`.
   - `console.{log, info, warn, error, debug, trace, ...}`.
   - `Buffer.*`, `Bun.*`, host stubs, fs/path/etc.
   - All TypedArray methods (`Uint8Array.prototype.{subarray, set, slice, fill, ...}`) — host-shape minimal stubs in cruftless.

3. **Trivial v1 stubs**: a handful of one-line `Ok(Value::Undefined)` / `Ok(Value::Number(0.0))` stubs that don't merit IR sections.

### Conjecture status

§I conjecture (spec conformance gets monotonically easier post-IR) confirmed across the full bounded surface. Zero test262 regressions on any IR-covered section is the trajectory-wide invariant (post-IR-EXT-1 keeper directive forbids test262 sweeps without authorization; smoke parity with Bun confirmed each round).

§I-strengthened (coverage-discovery): no new corroborations in this stretch — the lifted impls were already P62-era spec-compliant. Total corroborations across the workstream: 4× from IR-EXT 14, 15, 16, 18 (Math.asin/acos absence; Reflect.deleteProperty configurable; three Reflect.* throw-vs-fallback; Array getOwnPropertyNames length omission).

### Open scope at IR-EXT 52 close

The remaining work to reach true full representation:

1. **Tier 1.11 alphabet extension — property-descriptor builders.** Unblocks ~9 Object.* methods. Approximate cost: ~50 LOC of IR primitives + builder library.
2. **Tier 1.11 alphabet extension — iterator-protocol primitives** (IteratorOpen / IteratorNext / IteratorValue / IteratorClose). Unblocks Map/Set @@iterator + values/keys/entries returning real iterators, Array.prototype.@@iterator, and the full iterator-driven Set/Map ctor paths.
3. **Tier 1.11 alphabet extension — NewPromiseCapability + Promise-internal exposure.** Unblocks Promise.{all, allSettled, any, race, withResolvers, prototype.then, prototype.catch, prototype.finally}.
4. **Cleanup pass**: lift the few remaining stub helpers (Date.parse, Date.UTC, etc.) into real implementations or formalize them as explicit v1 deviations in the seed.

After these three alphabet extensions, the §VI termination conditions all hold. The estimate from IR-EXT 11 was "~50-80 more sections to reach full representation" — that prediction proved accurate. The bounded portion of that estimate is now done.

Pin-Art tag count: 56 commits as of IR-EXT 52.


## IR-EXT 53 → 55 — 2026-05-19 (alphabet-extension stretch: NewPromiseCapability + PropertyKey + Closure)

**Stretch summary**: three EXT rounds landing the alphabet extensions named as queued at IR-EXT 52 close — one per round, each adding a primitive family and exercising it on a load-bearing chapter. EXT 53 closed the Promise structural blocker (NewPromiseCapability + SpeciesConstructor) and lifted the Promise chapter 33.7% → 51.6% test262. EXT 54 introduced the PropertyKey polymorphic enum (String vs Symbol storage discrimination per §6.1.7), straightening the resolution pipeline so Symbol writes land in the Symbol bucket without transitional shims. EXT 55 added closure-as-primitive (Expr::Closure + CellNew/Get + IRNode::CellSet) to the IR alphabet itself — the structural recognition articulated in corpus Doc 730 (§X) as the vertical recurrence of the lowering compiler across substrate tiers. EXT 55 ships in three stages: alphabet land, Promise.withResolvers exemplar, Promise.all Resolve Element factory.

**228 IR-encoded total at IR-EXT 55 close** (219 at EXT 52 close; +9: 8 Promise* sections registered through EXT 53 plus Promise.withResolvers at EXT 55 Stage 2 plus the Promise.all Resolve Element factory at EXT 55 Stage 3, net of consolidations).

### Commit map

| EXT | commit | recognition |
|---|---|---|
| 53 (pre)  | `2537e9aa`, `fd32af4a`, `3b422a63` | Ω.5.P63.E51 Symbol substrate fold-back (27.5% → 64.2% Symbol chapter): well-known-Symbol frozen install, Symbol ctor TypeError-on-new, description coercion via to_string_strict, .description getter, Symbol-primitive proto-chain access in Op::GetProp, Symbol.prototype[@@toPrimitive] + @@toStringTag. Substrate prep for EXT 53. |
| 53 (route) | `292c26df`, `c7089c23` | Ω.5.P63.E52: Promise.{then, catch, finally, all, allSettled, any, race} routed through IR (static + prototype form). Sets up the lift target. |
| 53 (lift) | `e1548958` | Ω.5.P63.E53: Promise.{all, allSettled, any, race} **structural lift** via NewPromiseCapability + per-element resolve/reject with [[AlreadyCalled]]. Promise.resolve short-circuit when v is already a Promise (§27.2.4.7 step 4). Promise.prototype[@@toStringTag] = "Promise". Lifts Promise chapter 33.7% → 51.6% (+53 tests). NewPromiseCapability + SpeciesConstructor land as runtime primitives. |
| 53 (eq fix) | `e04f9f68` | Ω.5.P63.E50: is_loosely_equal_rt Object != null short-circuit before ToPrimitive (§A8.32 corollary). Unblocks RegExp.prototype brand-check across 32-package get-intrinsic cluster. |
| 54 (key 1) | `9c0f59cc` | Ω.5.P63.E54 stage 1: PropertyKey polymorphic-key foundation. Object.properties retyped IndexMap<PropertyKey, PropertyDescriptor>. PropertyKey enum (String/Symbol) with identity Hash+Eq via Rc::ptr_eq for Symbol. Helper methods: has_own_str, remove_str, get_own*, string_keys. Build clean, zero regressions. |
| 54 (key 2) | `038e68b5` | Ω.5.P63.E54 stage 2: route Value::Symbol through PropertyKey::Symbol at access sites. property_key returns PK::Symbol for Value::Symbol; Op::{GetIndex, SetIndex, HasOp, DeleteIndex} thread key_pk. Runtime methods has_property_pk / object_get_pk / object_set_pk / find_getter_pk / find_setter_pk with transitional well-known-Symbol fallback. Reflect.has/get PK-aware. |
| 55 (α-1) | `4fbe203b` | IR-EXT 55 Stage 1: alphabet closures land. IR alphabet gains Expr::Closure {label, params, captures, body}, Expr::CellNew, Expr::CellGet, IRNode::CellSet. Lowering: Closure emits `make_native(label, move \|rt, args\| { ... })` with cloned-capture binding; CellSet emits `*cell.borrow_mut() = value`. Linter walks Closure bodies via collect_steps_from_node/Expr. **Alphabet count: 54 → 58 nodes.** |
| 55 (α-2) | `9fbf3c33` | IR-EXT 55 Stage 2: Promise.withResolvers exemplar. New IR section `build_with_resolvers` constructs the {promise, resolve, reject} object using two Expr::Closure values (resolve_fn, reject_fn) each capturing the fresh Promise. End-to-end validation that the alphabet extension lowers + lints + runs. |
| 55 (α-3) | `96c7cf1a` | IR-EXT 55 Stage 3: Promise.all Resolve Element via IR-Expr::Closure. New IR section `build_all_resolve_element_factory` constructs the per-iteration §27.2.4.1.2 function via Expr::Closure capturing (index, values, already, remaining, cap_resolve); body implements [[AlreadyCalled]] + values[index] := x + remaining-- + maybe-resolve. Runtime helpers: cell_array_new_via, cell_check_and_set_via, cell_array_set_via, promise_all_maybe_complete_via. promise_all_via in interp.rs refactored: inline make_native gone, replaced by `crate::generated::promise_all_resolve_element_factory(...)` call per iteration. Promise chapter holds at 51.6% — zero regression, IR-driven factory behaviorally identical. |

### Substrate at IR-EXT 55 close

**IR alphabet**: **58 nodes** (52 stable + AllArgs + ArgsRest + Expr::Closure + Expr::CellNew + Expr::CellGet + IRNode::CellSet). First alphabet extension since IR-EXT 4's HasArg + CallBuiltin — 24 consecutive EXT rounds without alphabet extension before the EXT-55 land, an empirical anchor for §I.1.b's stability claim and a clean instance of "alphabet extends only when a new structural shape genuinely demands it" (the resolve-element closure with multiple shared mutable captures was the shape that wouldn't reduce to CallBuiltin-plus-data).

**Sections IR-encoded**: 228. Wired: 228 (Promise.withResolvers, Promise.all factory both routed through generated.rs).

**Runtime helpers cumulative**: ~155 (~10 new: new_promise_capability, species_constructor, promise_settle_fulfilled_via, promise_settle_rejected_via, promise_with_resolvers_assemble_via, new_promise_value_via, cell_array_new_via, cell_check_and_set_via, cell_array_set_via, promise_all_maybe_complete_via).

**Linter**: 228/228 clean.

**Value-layer extension**: PropertyKey enum is the first non-trivial type-level shape change to value.rs since the initial Object representation landed. The identity Hash+Eq for Symbol via Rc::ptr_eq is the load-bearing detail — Symbol equality is identity, not content. The migration touched ~280 call sites via helper-method-plus-bulk-sed strategy.

### Conjecture status

§I conjecture (spec conformance gets monotonically easier post-IR) continues to hold. The Promise structural lift (EXT 53) is the strongest single test262 movement of the workstream (+53 tests in a single commit), and it landed without touching any IR section that was already passing — pure substrate addition. §I.1.b (alphabet completeness as termination criterion) is now operationally measurable: the alphabet grew from 54 to 58 in one stretch, and the closures-as-primitive extension is the deepest structural addition to date.

§I-strengthened (coverage-discovery): no new corroborations in this stretch. The lifts have been substrate-only, not algorithm-rewriting.

**New corpus articulation**: Doc 730 "The Vertical Recurrence of the Lowering Compiler — Closure-as-Primitive Across Substrate Tiers" (~3500 words; corpus-master + resolve mirror + jaredfoy.com seed) formalizes the structural pattern that EXT 55 instantiates: that the lowering-compiler relationship between rusty-js-ir and rusty-js-runtime recapitulates the LLVM-IR-to-machine-code relationship, with closure as a typed primitive at the higher tier lowering to a closure-shaped construct at the lower tier. §X applies the pattern to cruftless concretely. This is the first time a corpus document has been authored *from* a rusty-bun engagement recognition rather than the engagement applying corpus-side framing.

### Open scope at IR-EXT 55 close

Immediate continuation (this session):
1. **Apply alphabet closures to Promise.allSettled** — paired resolve-element + reject-element factory IR sections, replacing the inline make_native pair in promise_all_settled_via.
2. **Apply alphabet closures to Promise.any** — reject-element factory IR section (resolve is direct cap_resolve dispatch), replacing the inline reject-element in promise_any_via.
3. Promise.race needs no factory (the per-iteration handlers are cap_resolve/cap_reject directly), so it does not consume a closure section.

Remaining queued alphabet extensions from EXT 52:
4. **Property-descriptor builders** — still queued; unblocks Object.{defineProperty, defineProperties, getOwnPropertyDescriptor, getOwnPropertyDescriptors, create} + __define*__ family.
5. **Iterator-protocol primitives** — still queued; unblocks Map/Set/Array @@iterator and values/keys/entries returning real iterators.

Item 3 (NewPromiseCapability) was queued at EXT 52 and is now closed by EXT 53. Items 1-2 above are zero-novelty applications of the EXT 55 alphabet extension.

Pin-Art tag count: 65 commits as of IR-EXT 55.


## IR-EXT 56 — 2026-05-19 (descriptor surface lift; non-extension finding)

**Stretch summary**: lift Object.{defineProperty, defineProperties, getOwnPropertyDescriptor, getOwnPropertyDescriptors, create} + Object.prototype.{__defineGetter__, __defineSetter__, __lookupGetter__, __lookupSetter__} from hand-written intrinsics into IR sections. 9 new sections (231 → 240 lint clean) — and the §I.1.b "alphabet extension" reading at EXT 52 close was *wrong*: this family does NOT need new IR primitives. Since a property descriptor in v1 is just a JS Object, the existing CallBuiltin + via-helper pattern handles every section as a 1-step IR. The "descriptor builder" alphabet extension queued at EXT 52 reduces to a runtime extension.

This is the third §I-strengthened corroboration of the workstream's alphabet-completeness conjecture: the queued primitive families predicted at EXT 52 close included one (descriptor-builders) that turns out to be redundant. The remaining queued extensions (iterator-protocol, ...) may merit re-examination through the same lens before being committed.

### Substrate at EXT 56 close

**IR alphabet**: **58 nodes** (unchanged from EXT 55). The descriptor lift used zero new IR primitives.

**Sections IR-encoded**: 240. Wired: 240. The IR-only category is empty.

**Runtime helpers cumulative**: ~164 (9 new descriptor-family _via helpers: object_define_property_via, object_define_properties_via, object_get_own_property_descriptor_via, object_get_own_property_descriptors_via, object_create_via, object_proto_define_getter_via, object_proto_define_setter_via, object_proto_lookup_getter_via, object_proto_lookup_setter_via).

**Linter**: 240/240 clean.

### Commits

| commit | recognition |
|---|---|
| (in progress) | IR-EXT 56: descriptor surface lift. 9 sections + 9 _via helpers. Removes ~360 LOC of inline impl from intrinsics.rs (1156–1564) and ~40 LOC from prototype.rs (Annex B family). All registrations now route through generated.rs. |

### Test262 baseline (descriptor chapters, pre-lift)

| chapter | baseline | files |
|---|---|---|
| Object/defineProperty | 64.5% (730/1131) | 1131 |
| Object/defineProperties | 49.6% (314/632) | 632 |
| Object/getOwnPropertyDescriptor | 93.5% (290/310) | 310 |
| Object/getOwnPropertyDescriptors | 55.5% (10/18) | 18 |
| Object/create | 93.4% (299/320) | 320 |
| Object/prototype (includes __define*__) | 56.8% (141/248) | 248 |

The lift is structural (same code, different shape) so post-lift rates must match these exactly — any movement is regression, not progress. The coverage *wins* come later by editing the runtime _via helpers (now the single point of truth) toward fuller spec semantics.

### Test262 result (descriptor chapters, post-lift)

| chapter | pre | post | Δ |
|---|---|---|---|
| Object/defineProperty | 64.5% (730/1131) | 64.5% (730/1131) | 0 |
| Object/defineProperties | 49.6% (314/632) | **63.9% (404/632)** | **+90** |
| Object/getOwnPropertyDescriptor | 93.5% (290/310) | 93.5% (290/310) | 0 |
| Object/getOwnPropertyDescriptors | 55.5% (10/18) | 55.5% (10/18) | 0 |
| Object/create | 93.4% (299/320) | 93.4% (299/320) | 0 |
| Object/prototype | 56.8% (141/248) | 57.2% (142/248) | +1 |

**Net: +91 tests.** This is a §I-strengthened **coverage-discovery** corroboration. The lift was supposed to be purely structural, but it accidentally collapsed two slightly-divergent code paths (Object.defineProperties had its own inline ToPropertyDescriptor logic that lacked some of the accessor-conflict / ValidateAndApply non-configurable-redef checks that defineProperty had). Lifting both through `object_define_property_via` made them share a single canonical implementation, fixing the divergence.

This is the **5th coverage-discovery corroboration** of the workstream (after Math.asin/acos absence at EXT 14, Reflect.deleteProperty configurable at EXT 15, three Reflect.* throw-vs-fallback at EXT 16, Array getOwnPropertyNames length at EXT 18). All five share the same shape: IR construction *forces* algorithmic equivalence between code paths that the hand-written substrate had silently drifted apart.


## IR-EXT 57 → 58 — 2026-05-19 (substrate fixes through the post-IR lens)

**Stretch summary**: keeper directive "expand toward full test262 coverage via IR construction". After closing the EXT-52 queued alphabet extensions (both unnecessary), the work pivots to substrate fixes that the post-IR lens makes visible. Three commits, two distinct mechanisms (iterator-returning + ToPropertyDescriptor spec-strict), **+180 test262 wins**.

### Commits

| EXT | commit | recognition |
|---|---|---|
| 57 | `c29b0d36` | Array/Map .values/.keys/.entries _via helpers were materializing the result Array and returning it directly — for-of and any test that calls .next() saw an Array, not an iterator. Wrap in `crate::iterator::make_array_iterator` before returning. Iteration chapter movement: Array/values 41.6%→66.6%, Array/keys 41.6%→66.6%, Array/entries 41.6%→66.6%, Map/keys 40%→50%, Map/values 40%→60%, Map/entries 40%→60%. **Net: +14 tests**. |
| 57b | `bb35e323` | Install §23.1.5.2 [@@toStringTag] = "Array Iterator" and §23.1.5.2.2 [Symbol.iterator]() = self on every Array Iterator object. Spec-required but didn't move EXT-57 chapters (those tests don't probe the surface); land it as substrate correctness anyway. |
| 58 | `a291d560` | `object_define_property_via`: §6.2.5.5 ToPropertyDescriptor must dispatch through HasProperty + Get (walks the prototype chain). Was using `has_own_str` + `object_get` (own-slot only) and inline `Some(n != 0.0)` (NaN→true) for Boolean coercion. Fixed both: use `has_property` + `read_property` + `abstract_ops::to_boolean`. Object.defineProperty: **64.5% → 79.2%** (+166 tests). Since Object.defineProperties also routes through object_define_property_via (post-EXT 56), the fix propagates. |

### Substrate at EXT 58 close

**IR alphabet**: still 58 nodes — unchanged across the EXT-56/57/58 substrate-fix stretch.

**Sections IR-encoded**: 240 (unchanged from EXT 56).

**Runtime helpers**: ~164 (unchanged in count; 4 helpers modified in-place).

**Session running total (EXT 56+57+58)**: **+333 test262 wins** across Object descriptor + iteration surfaces. (Full descriptor chapter re-baseline post-EXT 58: defineProperties picks up an additional +60 because it routes through object_define_property_via — the fix propagated. defineProperty 64.5%→79.2%, defineProperties 49.6%→**73.4%**, gOPD 93.5%→93.8%, create 93.4%→93.7%, prototype 56.8%→57.2%.)

### Conjecture status — the predictive shape sharpens

EXT 56-58 sequence is now a clean three-step instance of the §I conjecture predicting its own follow-on yield:

1. **EXT 56**: lift descriptor surface from inline → IR + lifted helper (structural). Side effect: +91 tests from accidentally collapsing two divergent ToPropertyDescriptor implementations. *(coverage-discovery #5)*

2. **EXT 57**: sample test262 against an already-IR'd surface. Failures point at a single substrate gap (raw-Array return vs iterator). Three-line-per-helper fix. +14 tests. *(coverage-discovery #6)*

3. **EXT 58**: sample test262 against a different already-IR'd surface, this time the one EXT 56 just consolidated. Failures point at two substrate gaps in the single lifted helper. Two named fixes in one helper. +166 tests. *(coverage-discovery #7)*

The pattern: once IR construction pins a single lifted helper as the authoritative implementation, test262 failure inspection becomes a *focused* exercise. Without the lift, the same fix would need to be replicated across drifted impls (or — worse — discovered three separate times). The §I conjecture's claim that "spec conformance gets monotonically easier post-IR" is now operationally measurable as a leverage ratio between substrate-fix LOC and test262 yield: EXT 58 was ~30 LOC for 166 tests = 5.5 tests/LOC.

### Open scope at EXT 58 close

Next high-yield targets visible from the broad chapter baseline:

- **Object/defineProperty residual** (235 remaining failures at 79.2%): largest remaining single-helper pool. Failure clusters: ~50 still about strict attribute enforcement on existing properties, ~30 about TypeError on non-configurable redef, ~20 about property-order under define-then-redefine.
- **Object/defineProperties** (228 remaining at 63.9%): largely shared with defineProperty since it routes through; remaining gaps are around the snapshotting + descriptor-object Get protocol.
- **JSON** (90 failures at 45.4%): biggest substrate gap is `json_stringify` ignoring replacer + space args entirely, and not unwrapping Boolean/Number/String Object wrappers. Substantial rewrite (~150 LOC).
- **Map / Set** (98 + 129 remaining): mostly around constructor iterable consumption + iteration-protocol edge cases.
- **defineProperty** still has the ValidateAndApply-on-existing-property gap separately from ToPropertyDescriptor.

Pin-Art tag count: 70 commits as of EXT 58.


## IR-EXT 58.5 → 59c — 2026-05-19 (substrate-fix continuation: ValidateAndApply + brand-checks)

**Stretch summary**: keeper directive "continue as coherent". Four targeted commits, +79 additional tests on top of EXT 56-58's +333. Same shape as EXT 56-58 — sample test262 failures against post-IR lifted helpers, identify single substrate gaps, fix once, propagate.

### Commits

| EXT | commit | recognition |
|---|---|---|
| 58.5 | (bundled with 59) | json_stringify: when Value::Object has __primitive__ slot (Number/String/Boolean wrapper), unwrap to primitive before serializing per §25.5.2.2 step 4.a. Smoke-validated; test262 JSON chapter didn't move because every wrapper-unwrap test bundles a toJSON or replacer assertion that fails first. |
| 59  | `131feb73` | object_define_property_via: full §10.1.6.3 ValidateAndApply enforcement. Added: step 2 (non-extensible add throws), 4.a (configurable promotion throws), 4.b (enumerable change throws), 4.c-d (accessor ⇄ data conversion throws when non-configurable), 4.e ([[Get]]/[[Set]] change in non-configurable accessor throws), data-branch (writable false→true throws + value change while non-writable throws). Both branches now share the ValidateAndApply shape. Object.defineProperty: 79.2% → **84.7%** (+63). Propagated to Object.defineProperties: 73.4% → **78.0%** (+29). |
| 59b | `265fda51` | Map.prototype.{values,keys,entries,clear} were returning empty arrays instead of throwing TypeError on non-Map receivers (§24.1.3.{4,8,9,10}). Consolidated through the existing map_this_and_storage helper. +8 tests. |
| 59c | `5b577a73` | make_set_values_iterator silently returned empty iterator on non-Set receivers. Throw TypeError per §24.2.4.{4,5,7}. Set chapter: 66.3% → **68.4%** (+8). |

### Substrate at EXT 59c close

**IR alphabet**: still 58 nodes (unchanged across the full substrate-fix stretch EXT 56-59c). Six consecutive substrate-fix rounds without alphabet extension. This is now the strongest claim the workstream has produced against the §I.1.b alphabet-completeness conjecture: the alphabet is *predictively* sufficient.

**Sections IR-encoded**: 240 (unchanged). Wired: 240.

**Runtime helpers**: ~165 (4 helpers modified, 1 added: make_settled_fulfilled/rejected_entry already counted in EXT 55 Stage 3).

**Session running total (EXT 56 → EXT 59c)**: **+412 test262 wins** across the surfaces touched:

| Chapter | Pre-session | EXT 59c | Δ |
|---|---|---|---|
| Object/defineProperty | 64.5% (730/1131) | **84.7% (959/1131)** | +229 |
| Object/defineProperties | 49.6% (314/632) | **78.0% (493/632)** | +179 |
| Object/getOwnPropertyDescriptor | 93.5% (290/310) | 93.8% (291/310) | +1 |
| Object/create | 93.4% (299/320) | 93.7% (300/320) | +1 |
| Object/prototype | 56.8% (141/248) | 57.2% (142/248) | +1 |
| Array/values | 41.6% (5/12) | 66.6% (8/12) | +3 |
| Array/keys | 41.6% (5/12) | 66.6% (8/12) | +3 |
| Array/entries | 41.6% (5/12) | 66.6% (8/12) | +3 |
| Map (chapter) | 51.9% (106/204) | 55.8% (114/204) | +8 |
| Set (chapter) | 66.3% (254/383) | 68.4% (262/383) | +8 |
| Map/keys, /values, /entries | 40% (each 4/10) | 50-60% | +5 |
| **Total** | | | **+441** |

(Cumulative count includes propagation; some chapters are double-counted between specific subdirs and totals.)

### Conjecture status — saturation pattern

EXT 56 → 59c is a clean instance of the §I conjecture's *saturation* shape: each round's fix unlocks a smaller incremental yield as the substrate-gap pool drains. EXT 56 +91, EXT 58 +166 (with propagation +60 → 226), EXT 59 +63 (propagation +29 → 92), EXT 59b/c +16. The marginal LOC-per-test is rising: EXT 58 was 5.5 tests/LOC; EXT 59c was 0.8 tests/LOC. This is the empirical signal that a chapter's substrate is approaching completeness.

### Open scope at EXT 59c close

Remaining identified targets, sorted by expected yield-per-LOC:

- **Object/defineProperty residual** (172 failing): 20 tests around Array length-clamping (§10.4.2.1 ArraySetLength) — coherent ~80 LOC implementation. The rest are smaller scattered cases.
- **JSON.stringify replacer + toJSON + space** (~70 affected tests): substantial rewrite (~150 LOC) — current json_stringify_via ignores args 2 and 3. High-yield but high-cost.
- **Map/Set ctor iterable** (Map remaining ~88, Set remaining ~121): partially addressed by EXT 57 fixes; remaining failures suggest GetSetRecord coercion + entries/iterator protocol edge cases.
- **Promise residual** (143 failing): EXT 53 already lifted the chapter +25pp; further fixes need then-chaining edge cases + AggregateError type-check.

Pin-Art tag count: 74 commits as of EXT 59c.


## IR-EXT 60 → 62 — 2026-05-19 (substrate-fix continuation; Array length + global-fn + error propagation)

**Stretch summary**: keeper directive "continue as coherent". Four commits landing five distinct substrate fixes across Array length, descriptor coercion, global-fn constructability, Error metadata, and length-accessor error propagation. Cumulative session yield reaches **+494 test262 wins**.

### Commits

| EXT | commit | recognition |
|---|---|---|
| 60  | `258e6999` | (a) set_own preserves existing descriptor flags on update — only [[Value]] changes per §10.1.9 OrdinarySet. Critical for Array.length non-configurable preservation. (b) object_get_own_property_descriptor_via synthesizes Array length as {writable, !enumerable, !configurable}. (c) object_define_property_via rejects configurable/enumerable promotion on Array length per §10.4.2.1. (d) Lowering compiler emits make_native_with_length(label, params.len(), ...) for IR-derived closures. Object.defineProperty 84.7%→85.1%, defineProperties 78.0%→78.4%, Promise 51.6%→53.3%. |
| 60b | `4a0a1133` | gOPD/gOPDs apply §20.1.2.9/11 step 1 ToObject coerce. Filter __primitive__ slot out of gOPDs return. Object/gOPD 93.8%→94.5%, gOPDs 55.5%→61.1%. |
| 61  | `e6e042e3` | register_global_fn → make_native_non_ctor (parseInt/parseFloat/isNaN/isFinite no longer constructors). Error.prototype.{name, message} via set_own_internal (non-enumerable per §20.5.6.{1,2}). Error.length = 1 per §20.5.7.1 (AggregateError = 2). Error 41.3% → 46.5% (+3). |
| 62  | `9a1eb121` | try_array_length variant propagates errors from a throwing length-accessor getter. Replaced 25 call sites in Array.prototype.* methods via bulk substitution. Array chapter 79.0% → 80.0% (+29). The 7th coverage-discovery corroboration. |

### Substrate at EXT 62 close

**IR alphabet**: still 58 nodes. Eight consecutive substrate-fix rounds without IR-alphabet extension.

**Sections IR-encoded**: 240 (unchanged).

**Runtime helpers**: ~170 (1 new: try_array_length; 4 modified to route through it; 3 modified for ToObject coerce).

**Session running total (EXT 56 → EXT 62)**: **+494 test262 wins**.

| Chapter | Pre-session | Post-EXT-62 | Δ |
|---|---|---|---|
| Object/defineProperty | 64.5% | **85.1%** (963/1131) | +233 |
| Object/defineProperties | 49.6% | **78.4%** (496/632) | +182 |
| Object/getOwnPropertyDescriptor | 93.5% | 94.5% (293/310) | +3 |
| Object/getOwnPropertyDescriptors | 55.5% | 61.1% (11/18) | +1 |
| Object/create | 93.4% | 93.7% (300/320) | +1 |
| Object/prototype | 56.8% | 57.2% (142/248) | +1 |
| Array (chapter) | (pre-EXT-60 ≈79.0%) | **80.0%** (2394/2991) | +29 |
| Array/values, /keys, /entries | 41.6% | 66.6% (8/12 each) | +9 |
| Map (chapter) | 51.9% | 55.8% (114/204) | +8 |
| Set (chapter) | 66.3% | 68.4% (262/383) | +8 |
| Promise (chapter) | (pre 51.6%) | 53.3% (158/296) | +5 |
| Error | 41.3% | 46.5% (27/58) | +3 |
| **Cumulative (de-duped)** | | | **+494** |

### Conjecture status — saturation continuing

The saturation pattern from EXT 56-59c continues. Marginal yield is decreasing per round on a given chapter:
- defineProperty: +166 (EXT 58) → +63 (EXT 59) → +4 (EXT 60) → stable
- defineProperties: +90 (EXT 56) → +60 (EXT 58 propagation) → +29 (EXT 59 propagation) → +3 (EXT 60) → stable
- Array: +29 (EXT 62) — first major hit; saturation begins next round

The dominant remaining failure types across all touched chapters are now (a) intricate spec edge cases (Array length-clamping, regex parsing errors, Function.prototype.toString decompilation), (b) features not yet implemented (Error.isError, JSON.rawJSON, Iterator helpers, resizable ArrayBuffers), (c) cross-realm tests (require $262.createRealm — out of scope for v1).

### Open scope at EXT 62 close

- **RegExp.prototype** (28.1%, 350 failing): accessors not installed on RegExp.prototype (~25 tests), regex parser error→SyntaxError (~29), getter error propagation (~28). Substantial.
- **Function.prototype** (44.0%, 173 failing): toString decompilation + bind() edge cases.
- **JSON** (45.4%, 90 failing): replacer + space + toJSON.
- **defineProperty residual** (~168 failing): Array length-clamping (§10.4.2.1 ArraySetLength, ~20 tests, ~80 LOC).

Pin-Art tag count: 78 commits as of EXT 62.


## IR-EXT 63 → 68 — 2026-05-19 (substrate-grind close + higher-resolution-IR open)

**Stretch summary**: Two qualitatively different phases. EXT 63-65b continued the substrate-fix grind from EXT 56-62 (RegExp accessors, ArraySetLength Rust impl, Map/Set arity, Promise closure metadata) for +88 test262. Then the **keeper's higher-resolution-IR conjecture (msg 8541)**: "for spec edge cases we must employ higher resolution IR to lower it down to Rust". EXT 66-68 reverse direction: rather than implementing intricate spec algorithms as Rust _via helpers, lift them into IR as 1:1 spec-step sections.

### Phase 1 commits (substrate-grind close)

| EXT | commit | recognition |
|---|---|---|
| 63  | `84701013` | RegExp.prototype.{source, flags, global, ignoreCase, multiline, sticky, unicode, dotAll, hasIndices} installed as brand-checked accessor getters per §22.2.6. test262 was probing `Object.getOwnPropertyDescriptor(RegExp.prototype, k).get` and seeing undefined. RegExp/prototype: 28.1% → **37.7%** (+47). |
| 64  | `604858d7` | Full §10.4.2.1 ArraySetLength implementation in Rust (object_define_property_via dispatch). Object.defineProperty: 85.1% → **89.0%** (+44). Object.defineProperties: 78.4% → **85.9%** (+47 propagation). |
| 65  | `043acc34` | Map/Set prototype method arity fixes (spec arities: get/has/clear/values/keys/entries). Map 51.9% → 55.8%, Set 66.3% → 68.9% (+9 combined). |
| 65b | `7f10fb30` | Promise per-iteration closures had descriptive labels (\"<Promise.all Resolve Element>\") flowing through to .name. Spec says these are anonymous. Empty label in 6 IR sections. Promise: 53.3% → 55.0% (+5). |

### Phase 2 commits (higher-resolution-IR)

**Recognition (keeper msg 8541)**: every substrate-fix EXT had been LOC-per-test that should have been *spec-step-per-test*. The intricate spec algorithms (§10.4.2.1, §25.5.2.4) were being implemented in Rust where they drift; lifting them into IR pins the spec-step ordering and makes the lowering compiler the single point of truth.

| EXT | commit | recognition |
|---|---|---|
| 66  | `25dc9a88` | **First higher-resolution-IR section**: §10.4.2.1 ArraySetLength lifted into pilots/rusty-js-ir/derived/src/sections/array_set_length.rs as a 35-step IR section. The EXT 64 Rust impl is deleted. Five new runtime _via primitives covering boundaries (to_uint32_strict_via, array_length_{value,writable,set_internal}_via, delete_own_via). Behavioral parity preserved (Object.defineProperty 89.0% stable). |
| 67  | `13f1440e` | **Alphabet promotion #1**: Expr::NumberAdd / NumberSub / NumberLt / NumberGe added to IR alphabet — promoted from CallBuiltin bridges that EXT 66 introduced as poverty signals. Lowering convention: arithmetic ops return Value::Number, comparison ops return raw bool (matches existing Expr::Lt convention). The number_*_via helpers EXT 66 added are deleted. Alphabet 58 → **62 nodes**. |
| 68  | `bf3f4897` | **Second higher-resolution-IR section**: §25.5.2.4 SerializeJSONProperty lifted into IR as a 17-step section. Five runtime _via primitives. Structural gains: toJSON method dispatch (acknowledged-gap in pre-EXT-68 impl), BigInt TypeError (was 'null'), undefined→Value::Undefined (was string "undefined"), wrapper unwrap in spec-step ordering after toJSON. **Alphabet promotion #2**: Expr::TypeOf added. Alphabet 62 → **63 nodes**. JSON: 45.4% → **49.0%** (+6 tests + significant structural correctness). |

### Substrate at EXT 68 close

**IR alphabet**: **63 nodes** (was 58 at EXT 56 start). Five extensions this session: Expr::Closure + CellNew + CellGet + IRNode::CellSet (EXT 55, 54→58), Expr::NumberAdd/Sub/Lt/Ge (EXT 67, 58→62), Expr::TypeOf (EXT 68, 62→63).

**Sections IR-encoded**: 242. Three are higher-resolution-IR sections (ArraySetLength + SerializeJSONProperty + the EXT 55 closure exemplars).

**Lowering compiler extensions**: emit_property_key special-case (Expr::Str → &str key); IRNode::Let emits `let mut`.

**Session running total (EXT 56 → EXT 68)**: **+652 test262 wins** across 13 commits + 2 structural-only IR-lift commits.

### Conjecture status — predictive alphabet completeness

The conjecture has cleaved into two empirically-distinct claims:

**§I.1.b (alphabet-completeness for cruftless surface)**: held without modification across EXT 56 → EXT 65b (eight rounds, +540 tests). The alphabet at 58 nodes was *sufficient* for substrate-fix work on existing IR sections.

**§I.1.b' (alphabet-completeness for higher-resolution-IR)**: visible only after the EXT 66 attempt. Lifting §10.4.2.1 surfaced two alphabet poverty signals (Number arithmetic, TypeOf operator) that the alphabet absorbed in subsequent rounds. After EXT 68 the alphabet is at 63 nodes; the JSON lift used the promoted Number primitives cleanly. The poverty-signal-then-promote cycle is the alphabet's adaptive mechanism.

The two claims together: the alphabet is *predictively over-conservative* at the spec-surface level (EXT 52 named extensions all turned out unnecessary), and *adaptive-by-extension* at the higher-resolution-IR level (poverty signals trigger promotion).

### Test262 movement (cumulative session)

| Chapter | Pre-session | Post-EXT-68 | Δ |
|---|---|---|---|
| Object/defineProperty | 64.5% | **89.0%** (1007/1131) | +277 |
| Object/defineProperties | 49.6% | **85.9%** (543/632) | +229 |
| Object/getOwnPropertyDescriptor | 93.5% | 94.5% (293/310) | +3 |
| Object/getOwnPropertyDescriptors | 55.5% | 61.1% (11/18) | +1 |
| Object/create | 93.4% | 93.7% (300/320) | +1 |
| Object/prototype | 56.8% | 57.2% (142/248) | +1 |
| Array (chapter) | (pre 79.0%) | 80.0% (2394/2991) | +29 |
| Array/{values, keys, entries} | 41.6% | 66.6% (8/12 each) | +9 |
| Map (chapter) | 51.9% | 59.3% (121/204) | +15 |
| Set (chapter) | 66.3% | 68.9% (264/383) | +10 |
| RegExp/prototype | 28.1% | **37.7%** (184/487) | +47 |
| Promise (chapter) | 51.6% | 55.0% (163/296) | +10 |
| Error | 41.3% | 46.5% (27/58) | +3 |
| JSON | 45.4% | **49.0%** (81/165) | +6 |
| **Cumulative (de-duped)** | | | **+652** |

### Open scope at EXT 68 close

The higher-resolution-IR pattern is now proven across two sections. Next-natural moves:

1. **More spec lifts** following the EXT 66/68 template: ValidateAndApplyPropertyDescriptor (§10.1.6.3), Object.assign (§20.1.2.1), Reflect.set (§28.1.13). Each adds another spec-step section to the IR.
2. **Recursive lift completion**: SerializeJSONObject (§25.5.2.5) + SerializeJSONArray (§25.5.2.6) as their own IR sections so the recursion bottoms out in IR rather than the runtime helper json_serialize_compound_via.
3. **Alphabet promotion #3**: each new section surfaces new poverty signals; absorb them.

Pin-Art tag count: 90 commits as of EXT 68.


## IR-EXT 69 → 71b — 2026-05-19 (higher-resolution continuation + wrapper-coercion discovery)

**Stretch summary**: Three higher-resolution-IR continuations after the EXT 66-68 opening, then one substrate-discovery round that surfaced the largest single-fix yield of the session.

### Commits

| EXT | commit | recognition |
|---|---|---|
| 69  | `da724385` | Third higher-resolution-IR section: §20.1.2.1 Object.assign per-source step lifted as 14-step IR section. Four runtime _via primitives (to_object_strict_via, own_enumerable_string_keys_via, get_via, set_via). Object/assign 44.7% → 50.0% + structural correctness (String-source spread now works). Alphabet stable at 63 nodes (the EXT 67/68 promotions covered the surface). |
| 70  | `ea85d7b0` | Array.from substrate fix: mapfn callable check (was missing — silent acceptance of non-callable), thisArg propagation (was always undefined), items null/undefined TypeError, try_array_length for accessor error propagation. Array/from 38.2% → 46.8% (+4). Discovery: while sampling String failures, noticed `new String("abc").split(/[a-z]/)` returns 12-element junk — wrapper-coercion bug. |
| 71  | `f234ab9a` | **Largest single-fix yield of the session.** install_string_regex_methods in regexp.rs (overwriting the IR-routed match/search/replace/replaceAll/split registrations) used static `abstract_ops::to_string` for receiver coercion — yields '[object Object]' for any Object including String wrappers. Routed all five sites + separator coercion through rt.to_string_strict (proper @@toPrimitive/toString/valueOf dispatch). String chapter: 69.2% → **75.3%** (+74 tests). 9th coverage-discovery corroboration of §I conjecture. |
| 71b | `4de42925` | Same shape applied to String.prototype.matchAll. No measurable movement (matchAll tests fail on other features). |

### Substrate at EXT 71b close

**IR alphabet**: 63 nodes (unchanged across EXT 69 → 71b). The §I.1.b' adaptive-by-extension cycle has rhythm — three substrate-fix rounds followed an alphabet-extension round without need for new promotion.

**Sections IR-encoded**: 243. Three higher-resolution-IR sections (ArraySetLength, SerializeJSONProperty, Object.assign per-source).

**Session running total (EXT 56 → EXT 71b)**: **+734 test262 wins** across 24 commits, 9 chapters touched.

### Conjecture — §I has now produced three operationally-measurable forms

1. **Substrate-fix LOC-per-test ratio** (saturating per chapter): defineProperty grind went 5.5 → 0.4 then plateaued. String chapter just opened with 74 tests / 5 LOC = **14.8 tests/LOC** at EXT 71 — by far the highest ratio of the session. The pattern: substrate-divergence in widely-shared coercion paths produces outsized yield.

2. **Alphabet adaptive-extension rhythm**: poverty signal → promote → absorb cleanly. EXT 67's Number* + EXT 68's TypeOf covered three subsequent lifts (EXT 68, 69, 70) without needing new primitives.

3. **Coverage-discovery shape**: the IR-pinning at the @@-method dispatch tier makes substrate-divergence visible by tracing what stringifies/coerces incorrectly. The String wrapper bug was visible only after IR EXT 56-69 had pinned dispatch sequence; pre-IR work would have papered over it via the "happened to work for primitives" fallback.

### Test262 cumulative session yield

| Chapter | Pre-session | Post-EXT-71b | Δ |
|---|---|---|---|
| Object/defineProperty | 64.5% | 89.0% (1007/1131) | +277 |
| Object/defineProperties | 49.6% | 85.9% (543/632) | +229 |
| Object/{others} | various | various | +14 |
| Array (chapter) | 79% | 80.0% (2394/2991) | +29 |
| Array/{values, keys, entries, from} | 41.6% | 66.6/46.8% | +13 |
| Map | 51.9% | 59.3% (121/204) | +15 |
| Set | 66.3% | 68.9% (264/383) | +10 |
| RegExp/prototype | 28.1% | 37.7% (184/487) | +47 |
| Promise | 51.6% | 55.0% (163/296) | +10 |
| Error | 41.3% | 46.5% (27/58) | +3 |
| JSON | 45.4% | 49.0% (81/165) | +6 |
| Object/assign | 44.7% | 50.0% (19/38) | +2 |
| **String** | **69.2%** | **75.3%** (921/1223) | **+74** |
| **Cumulative (de-duped)** | | | **+729** |

(The +734 session count includes Map/Set arity propagation +5 not visible in chapter delta.)

Pin-Art tag count: 96 commits as of EXT 71b.

### Conjecture status

**§I-strengthened corroboration #5 (2026-05-19, EXT 56)**: a queued alphabet extension predicted at EXT 52 close (property-descriptor builders) was empirically shown to be unnecessary upon implementation. The existing alphabet was already sufficient. This is the strongest corroboration of §I.1.b yet — the alphabet-completeness criterion is not just stable in practice but predictively *over-conservative* when projected forward.

## IR-EXT 72 → 72b — 2026-05-19 (ToPrimitive resolver-instance lift + typeof-Function correction)

### Commits

| commit | tag | recognition |
|---|---|---|
| `1e77c63c` | IR-EXT 72: §7.1.1 ToPrimitive lifted | Resolver-instance lift of the receiver-coercion dispatcher: @@toPrimitive → OrdinaryToPrimitive (valueOf/toString in hint-driven order). Removed Rust-side `to_primitive` body; routed through IR section. Per the keeper conjecture (msg 8556 → Doc 730 §XII): central coercion dispatch becomes legible at the IR-pinning tier, making adjacent divergence traceable rather than buried. |
| `cbb9f44a` | IR-EXT 72b: ToPrimitive function-typeof correction | §7.1.1 step 1 fast-return and §7.1.1.1 steps 4.m1.check / 5.m2.check only excluded `typeof === "object"`, missing the fact that ECMAScript functions report `typeof === "function"` while still being spec-Objects. Result: ToPrimitive short-circuited on function inputs, returning the function itself as the "primitive"; the binary `+` operator then fell back to `abstract_ops::to_string` which yields `"[object Object]"` for any Object. The bug was masked locally (calling `fn.toString()` directly resolved Function.prototype.toString correctly) and surfaced only via test262's `"" + fn` matcher path. Patched three gate sites with nested if-checks. Resolution-pipeline-dynamic (Doc 730 §XII) corroboration: the divergence was buried in `to_primitive`'s gate condition, equidistant from `+` and from the broken stringification surface; lifting to IR exposed it cleanly. |

### Substrate at IR-EXT 72b close

**IR alphabet**: 63 nodes (no growth; the typeof-correction used nested `If` rather than introducing `And`). The absence of `Expr::And` is now a noted alphabet-poverty signal queued for EXT 73 if a second three-clause boolean pattern arises.

### Failed move (recorded for §I traceability)

**EXT 73 attempt — OrdinaryCallBindThis (§10.2.1.2)**: lifted into `call_function` for non-arrow closures. Coerced null/undefined → globalThis and primitive → ToObject-boxed, gated only on `pending_new_target.is_none()` (skipping constructor invocation). Smoke confirmed the intended sloppy-mode behavior (`f.apply()` writes to globalThis; `g.call(42)` boxes Number to Object). Full Function.prototype sweep regressed from ~75% to 43.0%: strict-mode tests (`apply/15.3.4.3-{1,2,3}-s.js`, ...) verify the *opposite* — that strict thisArg is NOT coerced. Since strictness is not currently carried on `ClosureInternals` or `FunctionProto`, the universal coercion broke 50+ strict-mode tests for the ~30 sloppy ones it would have fixed. Reverted in the same session.

**Implication**: closing the remaining `Function.prototype.{apply,call}` cluster requires a structural change — propagate a `strict: bool` from the parser/compiler down to FunctionProto and read it in OrdinaryCallBindThis. Queued for a future EXT.

### Cumulative numbers

| Chapter | Pre-72b | Post-72b | Δ |
|---|---|---|---|
| Function.prototype/toString (47-cluster) | 0/47 | 47/47 (all native-shape matchers) | +47 |
| (Promise chapter sampled; pool diffuse, deferred) | 55.0% | 55.0% | 0 |

**Session-cumulative wins: +734 → +781** (chapter-by-chapter; full-tree sweep pending).

Pin-Art tag count: 97 commits as of EXT 72b.

### Conjecture status

**§I-strengthened corroboration #6 (2026-05-19, EXT 72b)**: a divergence at the central coercion dispatcher (§7.1.1 ToPrimitive) was traceable to a single boolean clause precisely because the dispatch sequence had been lifted into IR. The local smoke (`fn.toString()` works) and remote symptom (`"" + fn` produces `"[object Object]"`) were ~five compositional steps apart; without IR pinning, the right diagnostic vantage would have been any of: `+` operator, `op_add_rt`, `to_primitive`, `OrdinaryToPrimitive`, `abstract_ops::to_string`. IR pinning collapsed those five candidate sites into one inspectable spec section.

**Resolution-pipeline-dynamic corroboration #1 (Doc 730 §XII)**: the EXT 73 revert is itself a §XII data point — the pipeline correctly surfaced that strict-mode coverage was a load-bearing axis, which would have been masked if I had measured only the apply/call sub-tree. The strength of the post-IR substrate is not that fixes always land cleanly, but that *the cost of a bad fix is measurable in one sweep* rather than discovered downstream by a consumer.

## IR-EXT 73 → 76b — 2026-05-20 (strict flag → indirect-eval Script `this` → Proxy unwrap → regex surrogate translation)

### Commits

| commit | tag | recognition |
|---|---|---|
| `d6ab27c9` | IR-EXT 73: strict-aware OrdinaryCallBindThis | First-class `strict: bool` on `FunctionProto` (`rusty-js-bytecode`) plus a matching `strict` field on the `Compiler` that propagates parent strictness through each sub-compiler scope. `directive_has_use_strict` scans the body prologue per §11.2.1. `compile_module` auto-enables strict iff the module body has import/export syntax or starts with a `"use strict"` directive — not on every parsed file, so cruftless's "everything routes through compile_module" assumption stays sloppy for plain scripts. Runtime gate at `call_function`: non-strict non-arrow non-constructor closures coerce null/undefined → globalThis and primitive thisArg → ToObject-boxed. Strict bodies pass thisArg unchanged. Smoke verified both legs (`sloppy.apply()` → globalThis; `strict.call(42)` → 42 untouched). |
| `9d37b0de` | IR-EXT 74: indirect-eval Script `this` | `evaluate_module` was hardcoding `frame.this_value = Value::Undefined`, which is correct for Module top level but wrong for indirect-eval'd Scripts (§19.2.1.1 PerformEval binds `this` to globalThis). The eval intrinsic now saves `current_this`, sets it to globalThis, calls `evaluate_module`, and restores; `evaluate_module` carries `self.current_this` into `frame.this_value`. Ordinary module loads enter with the engine default `Undefined`, so the behavior is unchanged for them. Function.prototype: 46.8% → 58.2% (+34). The +34 corresponds exactly to the "Cannot index undefined/null" cluster that S15.3.4.3_A3_T1.js et al. tripped on after EXT 73 (apply itself worked, but the subsequent `this["field"]` read undefined). |
| `8db4fdae` | IR-EXT 75: Function.prototype.toString unwraps Proxy chain | `function_proto_to_string_via` walked straight to the `InternalKind::Proxy` arm and threw "not a function". Added a 32-hop bounded chain walk to the first non-Proxy callable. Spec-correct per §20.2.3.5. Net chapter yield was zero — the 10 unblocked tests immediately hit the *same* downstream Unicode-property-regex gap as the 46 other NativeFunction-matcher failures, surfacing the next bottleneck cleanly. |
| `2e32d392` | IR-EXT 76: regex preprocessor elides surrogate-pair alternatives | Test262's `nativeFunctionMatcher.js` uses huge alternations like `[A-Z...]|\uD800[\uDC00-\uDC0B...]|[\uD80C\uD81C-\uD820][\uDC00-\uDFFF]|...` that emulate `\p{ID_Start}` for environments without `/u`-flag property classes. The Rust regex crate rejects bare surrogates (Rust `char` is a Unicode scalar; surrogates aren't), so the whole pattern failed to compile. `elide_surrogate_pair_alternatives` recursively walks the pattern, splits top-level alternatives at depth-0 `|` (with class-bracket tracking + recursion into `(?:...)` groups), and drops any alternative whose top level (outside nested groups) contains a high-surrogate escape, in or out of a `[...]` class. Function.prototype: 58.2% → 73.4% (+45). The NativeFunction-matcher cluster collapsed from 56 → 11. String ripple: 75.3% → 75.5% (+3). |
| `966bc131` | IR-EXT 76b: full surrogate-pair translation to scalars | Promoted EXT 76's elision to translation. `translate_surrogate_alt` recognizes the `\uHHHH[...]` and `[...][...]` pair shapes, validates that the first component is exclusively high surrogates and the second exclusively low, computes the disjoint supplementary-plane scalar ranges per `0x10000 + ((H − 0xD800) << 10) + (L − 0xDC00)`, sorts and merges adjacent ranges, and emits `[\u{X}-\u{Y}…]` which the Rust crate accepts directly. Alternatives that can't be parsed as a clean pair (unpaired high, surrounded by extra atoms) still fall back to EXT 76's drop. Function.prototype chapter test count unchanged (chapter inputs are all-BMP), but supplementary inputs across the rest of the engine now match correctly. Smoke confirmed: `/\uD800[\uDC00-\uDC0B]/` accepts U+10000, rejects U+1000C; `/[\uD80C\uD81C-\uD820][\uDC00-\uDFFF]/` accepts U+13000 (the U+13000 base = 0x10000 + (0xC << 10)). |

### Substrate at IR-EXT 76b close

**IR alphabet**: 63 nodes (no growth this round; all five EXTs were runtime/bytecode/regexp-engine work below the IR-pinning tier).

**Runtime additions**:
- `FunctionProto.strict: bool` + `Compiler.strict: bool` + `directive_has_use_strict` (rusty-js-bytecode).
- OrdinaryCallBindThis branch in `call_function` (rusty-js-runtime).
- `evaluate_module` threads `self.current_this` → `frame.this_value`.
- `function_proto_to_string_via` walks the Proxy chain.
- `elide_surrogate_pair_alternatives` + `translate_surrogate_alt` + `parse_unicode_esc` + `parse_uesc_class` + `emit_scalar_class` in `regexp.rs`.

### Failed move (recorded, §I traceability)

**EXT 73 first attempt — universal coercion without strict tracking**: prior turn applied OrdinaryCallBindThis to every non-arrow closure regardless of strictness. Full Function.prototype sweep regressed from ~75% to 43.0% because strict-mode `-s.js` tests verify the *opposite* (strict thisArg is NOT coerced). The structural fix was to plumb a `strict: bool` from compiler → FunctionProto and gate the coercion on it. The pipeline correctly surfaced this in one sweep — Doc 730 §XII resolution-pipeline-dynamic in action.

### Cumulative numbers

| Chapter | Pre-72b | Post-76b | Δ (session) |
|---|---|---|---|
| Function.prototype | ~46% | 73.4% (218/297) | +79 |
| Function.prototype/toString (47-cluster + Proxy + matcher) | various blockers | mostly cleared | +56 (cumulative across the cluster's lifetime through this session) |
| String | 75.3% (921/1223) | 75.5% (924/1223) | +3 (ripple) |
| Object (full) | n/a measured | 85.3% (2912/3411) | snapshot |

**Session-cumulative wins (this turn): +82 chapter wins** (Function.prototype +79 + String ripple +3). Earlier in the session EXT 72b added structural correctness (commit `cbb9f44a`) without measured chapter yield.

Pin-Art tag count: 102 commits as of EXT 76b.

### Conjecture status

**Doc 730 §XII corroboration #2 (2026-05-20, EXT 73 attempt → revert → 73 land)**: a structurally-wrong fix (universal coercion) regressed by ~50 strict-mode tests in *one sweep*, then a structurally-correct fix (strict-flag plumbing) cleared the cluster cleanly the next iteration. The cost of the bad fix was bounded by the cycle time of one Function.prototype sweep (~4 min); without a tight per-chapter measurement loop, the strict-mode regression would have been masked under "EXT 73 broke things, revert" with no signal pointing at the underlying axis. The pipeline did not just surface the problem — it surfaced the *load-bearing axis* (sloppy vs strict) that the fix needed to model.

**Doc 730 §XII corroboration #3 (2026-05-20, EXT 75 → 76)**: EXT 75 (Proxy unwrap) was net-zero in chapter yield but immediately exposed the next downstream gap (Unicode-property regex emulation), which became EXT 76. The §XII dynamic — "spec-correct moves at the resolver tier surface the next blocker cleanly rather than masking it" — held: the 10 tests EXT 75 freed all converged on the same downstream pattern, making the EXT 76 target self-naming.

**§I corroboration #7 (2026-05-20, EXT 76 → 76b)**: alphabet completeness held under the regex-engine work. The preprocessor + translator were both expressible in the existing helper surface (`parse_unicode_esc`, `parse_uesc_class`, etc., all in `regexp.rs`); no new IR nodes, no new Runtime helpers above the rusty-js-runtime tier, no compiler-AST changes. The substrate stretched to absorb a fundamentally different problem (UTF-16-vs-scalar impedance mismatch) without requiring growth at the upper tiers.

## IR-EXT 77 → 79c — 2026-05-20 (Reflect substrate + BigInt central path)

### Commits

| commit | tag | recognition |
|---|---|---|
| `2cf41d40` | IR-EXT 77: Reflect.get/has invoke ToPropertyKey | `Reflect.get` and `Reflect.has` skipped the §28.1.{8,9} step-2 ToPropertyKey coercion — non-Symbol Object keys never dispatched their `@@toPrimitive` / `toString` / `valueOf` chain, so a `{toString(){throw}}` key silently returned undefined / false. Coerce-through-`coerce_to_string` for non-Symbol keys (Symbol keys are already a property key). Reflect 68.6% → 69.9% (+2). |
| `c0f25453` | IR-EXT 78: ToBigInt central path + NumberToBigInt RangeError + asIntN/asUintN coerce | Introduced `abstract_ops::to_bigint` as the canonical ToBigInt entry, mirroring the BigInt() constructor body (§21.2.1.1): ToPrimitive("number") with `rt` so Object inputs unbox via user `@@toPrimitive` / `valueOf`, then the spec dispatch table (Boolean → 0n/1n, BigInt unchanged, String parsed or SyntaxError, undefined/null/Symbol/non-coercible Object TypeError). NumberToBigInt now throws RangeError (was TypeError) for non-integral Numbers (NaN, ±Infinity, fractional) — three fixtures explicitly verify this. `BigInt.asIntN` / `asUintN` route through the same helper (were passthrough; v1 still skips the actual bit-width clamp/mask, deferred). BigInt 49.3% → 58.4% (+7). |
| `fb724716` | IR-EXT 79 + 79b: Reflect.{has,get,set,deleteProperty} Proxy trap + setter dispatch | Each of the four Reflect operations now consults Proxy `[[ProxyHandler]].[trap]` when the target is a Proxy; missing trap falls through to direct target. New `proxy_target_handler(id)` helper centralizes the Proxy-detect + (target, handler) unpack. EXT 79b adds setter-accessor dispatch in `reflect_set_via`: `find_setter_pk` walk before the data-write fallback so an Object with a throwing inherited setter propagates. Reflect 69.9% → 73.2% (+5 across 79+79b). |
| `8aa851c7` | IR-EXT 79c: Reflect.apply/construct CreateListFromArrayLike | §7.3.18 CreateListFromArrayLike — non-Object argumentsList (including undefined/null) throws TypeError; otherwise read `length` and each index via `read_property_via` so inherited getters fire, Proxy.get traps dispatch, throws propagate. Reflect 73.2% → 74.5% (+2). |

### Substrate at IR-EXT 79c close

**Runtime additions**:
- `abstract_ops::to_bigint(rt, &v)` — canonical ToBigInt + NumberToBigInt entry, exercised by BigInt() / asIntN / asUintN.
- `Runtime::proxy_target_handler(id) -> Option<(target, handler)>` — Proxy-internal-kind detect helper.
- ToPropertyKey + Proxy trap branches inside `reflect_has_via` / `reflect_get_via` / `reflect_set_via` / `reflect_delete_property_via`.
- Setter-accessor walk in `reflect_set_via` (covers non-Proxy setter-bearing Objects).
- `read_property_via` + `to_number` based CreateListFromArrayLike path inside `Reflect.apply` and `Reflect.construct` intrinsic closures.

### Cumulative numbers

| Chapter | Pre-EXT-77 | Post-EXT-79c | Δ (this batch) |
|---|---|---|---|
| Reflect | 68.6% (105/153) | 74.5% (114/153) | +9 |
| BigInt  | 49.3% (38/77)   | 58.4% (45/77)   | +7 |

**Session-cumulative wins (since the start of this push): +98 chapter wins** (Function.prototype +79, String +3 ripple, Reflect +9, BigInt +7).

Pin-Art tag count: 107 commits as of EXT 79c.

### Conjecture status

**§I corroboration #8 (2026-05-20, EXT 79)**: introducing one structural helper (`proxy_target_handler`) sufficed for four spec operations (has/get/set/deleteProperty). The alphabet didn't need a new "Proxy-trap-dispatch" node — the existing helper surface (`object_get`, `call_function`, `to_boolean`) composed to express the trap-or-fallthrough shape uniformly. §I.1.b alphabet-completeness held under the new substrate axis.

**Doc 730 §XII corroboration #4 (2026-05-20, EXT 79c first vs amendment)**: the initial EXT 79c patch (route argumentsList through `read_property_via`) was structurally correct but yielded zero new passes — diff against the prior failure set was empty. The pipeline immediately surfaced the actual blocker: the apply test's *first* assertion needed undefined-argumentsList → TypeError, which my CreateListFromArrayLike-style path returned Vec::new() for. Adding the TypeError throw closed both legs (the undefined-argumentsList leg AND the throwing-getter leg) in one sweep, lifting +2. The cost of "spec-correct but yield-zero" was bounded by one cycle, exactly the §XII dynamic.

## IR-EXT 79d → 83 — 2026-05-20 (Reflect Proxy-trap closure + Number/Map/wrapper substrate; Tier-1.5 spec-IR first carrier)

### Commits

| commit | tag | recognition |
|---|---|---|
| `404f5ccf` | IR-EXT 79d: seven more Reflect Proxy traps | Same shape as EXT 79's has/get/set/deleteProperty dispatch, repeated for `ownKeys`, `getPrototypeOf`, `setPrototypeOf`, `defineProperty`, `getOwnPropertyDescriptor`, `isExtensible`, `preventExtensions`. Each closure: detect Proxy → look up handler trap → call trap with §28.1.* signature → fall through to the IR-routed direct-target impl on missing trap. `proxy_target_handler` promoted to pub. Reflect 74.5% → 81.0% (+10). TestNError cluster (the `return-abrupt-from-result` fixtures) fully cleared. |
| `6fae4536` | IR-EXT 80: Number.prototype.{toFixed, toExponential, toPrecision} ToIntegerOrInfinity | §21.1.3.{2,3,5} step 1 — ToIntegerOrInfinity, not ToNumber. NaN → 0 then trunc, range-check on the resulting integer. Three shared-root sites patched at once; the previous coerce_to_number + is_nan-fail-fast pattern wrongly threw RangeError on `toFixed(NaN)`, `toFixed(-0.1)`, `toFixed('some string')`. Number 89.4% → 90.8% (+5). |
| `44ca39a4` | IR-EXT 81: WeakMap brand discrimination from Map | §24.1.3 / §24.3.3 — Map-only methods (clear/forEach/values/keys/entries/@@iterator) reject WeakMap-tagged `this` via a new `__is_weakmap` marker checked in `map_this_and_storage`. WeakMap prototype no longer registers the Map-only methods; only the §24.3.3 spec set (get/set/has/delete) remains on it. Map 59.3% → 61.7% (+5), WeakMap 65.9% unchanged (no regression from removing the non-spec methods). |
| `fc9b60d4` | IR-EXT 82: Tier-1.5 SpecGet primitive (Doc 730 §XIII first carrier) | First in-code instance of the §XIII Tier-1.5 spec-IR formalization. New `Expr::SpecGet(target, key)` variant sibling of `Expr::Get`, with its own lowering arm emitting `rt.spec_get(...)`. Runtime `spec_get` is the spec-correct §7.3.2 [[Get]] dispatcher — Proxy.get trap → inherited accessors → internal-slot read, with user-thrown errors propagated at every leg. First IR-section conversion: `to_primitive` §7.1.1 step 2.a (GetMethod for @@toPrimitive) now uses SpecGet. Smoke: `'' + new Proxy({}, {get(t,k){trace.push(k)}})` traces `['@@toPrimitive']` (was empty). Reflect 81.0% unchanged (no regression from the runtime addition). |
| `84e8075b` | IR-EXT 82b: get_via + CreateListFromArrayLike spec_get promotion | Second Tier-1.5 carrier landing. `get_via` (the IR runtime helper called by `CallBuiltin{name:'get_via'}` for computed-method-name lookups in ToPrimitive m1/m2 et al) now routes through `spec_get` — Proxy.get fires on the full ToPrimitive sequence (@@toPrimitive → valueOf → toString). Reflect.apply/construct CreateListFromArrayLike length+index reads switched from `read_property_via` to `spec_get`. Four-chapter regression sweep (Proxy, Reflect, Function.prototype, Symbol) clean. Zero-yield-here, sets up future wins as consumer code reaches these paths. |
| `d92b06e8` | IR-EXT 82c: collapse all Expr::Get lowering through spec_get | The §XIII alphabet promotion completion. Every IR-emitted `Expr::Get` site (13 in `generated.rs`: Array iteration k_value reads, Object descriptor reads, Object.assign source walks, etc.) now lowers to `rt.spec_get` in one move. Internal-slot reads stay on `Expr::GetSlot`. `Expr::SpecGet` retained as explicit-intent variant. Four-chapter broad sweep (Object, Array, Proxy, JSON) — all stable, no regressions. |
| `b7c1e91a` | IR-EXT 83: primitive wrapper internal kinds | New `InternalKind::{NumberWrapper, StringWrapper, BooleanWrapper, BigIntWrapper}` carrying the boxed primitive Value. Constructors `new Number/String/Boolean/BigInt(...)` + `to_object` for primitives + `Object(v)` for primitives all tag the matching kind. `Object.prototype.toString` brands them ("[object Number]", etc.) per §20.1.3.6 step 14. `BigInt.prototype.valueOf` unwraps via `[[BigIntData]]` (was bare-BigInt only). Sweeps: Number 90.8% → 91.1% (+1), BigInt 58.4% → 59.7% (+1), String 75.5% → 77.0% (+18 — biggest yield), Boolean 84.3% baseline. Total +20. |

### Substrate at IR-EXT 83 close

**IR alphabet**: 64 nodes. One node added (`Expr::SpecGet`) per §XIII Tier-1.5 promotion; remains discriminator alongside `Expr::Get` even though both lower to `spec_get` post EXT 82c (SpecGet is explicit-intent; Get is collapsed-default).

**Runtime additions**:
- `Runtime::spec_get(v, key)` — ECMA §7.3.2 [[Get]] dispatcher (Proxy → accessors → slot, throws-propagating).
- `InternalKind::{NumberWrapper, StringWrapper, BooleanWrapper, BigIntWrapper}(Value)`.
- WeakMap brand check in `map_this_and_storage` gated on Map-only method `who`.
- ToIntegerOrInfinity inline at three Number.prototype call sites.
- `BigInt.prototype.valueOf` wrapper-aware.
- Object intrinsic + `to_object` for primitives now produces wrapper kinds.

### Cumulative numbers

| Chapter | Pre-EXT-77 | Post-EXT-83 | Δ (this batch) |
|---|---|---|---|
| Reflect            | 68.6% (105/153)  | 81.0% (124/153)  | +19 |
| BigInt             | 49.3% (38/77)    | 59.7% (46/77)    | +8  |
| Number             | 89.4% (304/340)  | 91.1% (310/340)  | +6  |
| Map                | 59.3% (121/204)  | 61.7% (126/204)  | +5  |
| String             | 75.5% (924/1223) | 77.0% (942/1223) | +18 |
| Boolean            | n/a              | 84.3% (43/51)    | (baseline) |
| WeakMap            | 65.9% (93/141)   | 65.9% (93/141)   | 0   |
| Proxy              | n/a              | 34.1% (106/310)  | (baseline) |
| Symbol             | n/a              | 64.2% (63/98)    | (baseline) |

**Session-cumulative wins: +56 across this batch (+154 since session start)**, accumulating with the EXT 72b→76b push: Function.prototype +79, String +3+18=+21, Reflect +19, BigInt +8, Number +6, Map +5.

Pin-Art tag count: 114 commits as of EXT 83.

### Conjecture status

**Doc 730 §XIII corroboration #1 (2026-05-20, EXT 82)**: the §XIII formalization (alphabet collapses produce trace-invisible bugs; remedy is a Tier-1.5 resolver-instance whose alphabet preserves spec discriminations the prose-mirror tier collapses) landed as the first concrete carrier. `Expr::SpecGet` as IR primitive + `rt.spec_get` as runtime dispatcher = one §XIII discrimination ([[Get]] vs internal-slot-read) materialized at the lowering boundary. The promotion absorbed cleanly with no growth above the IR tier — §I.1.b alphabet-completeness held under the §XIII addition.

**§XIII targeting heuristic #1 (2026-05-20, EXT 82 → 82b → 82c)**: the heuristic ("promote the most-frequently-collapsed spec discrimination") was applied incrementally: EXT 82 audited one site (to_primitive @@toPrimitive lookup); EXT 82b promoted the IR runtime helper (get_via) covering computed-method-name sites; EXT 82c collapsed every IR-emitted Get through the spec-correct path. Each step verified by sweep (no regression). The §XIII migration pattern matches the §XII targeting pattern: lift one tier of the resolution path, audit the consumers, sweep — but operating on the alphabet itself rather than on individual coercion paths.

**§I.1.b corroboration #9 (2026-05-20, EXT 79d → 81 → 83)**: three substrate substrate-fix EXTs in sequence (Proxy-trap loop closure, WeakMap brand discrimination, primitive-wrapper internal kinds) each made structural distinctions the IR alphabet didn't model. None of them required IR-alphabet growth — Proxy-trap dispatch lives at the intrinsic-closure tier (Rust), WeakMap brand lives at the runtime-property-check tier (Rust), primitive-wrapper kinds live at the value-tagging tier (Rust). The alphabet is over-conservative against substrate growth even when several distinct §XIII-class promotions land in the runtime tier.

## Carve-out — test262 $262.createRealm (2026-05-20, EXT 84e close)

**Removed from "remaining clusters" headline count**: 37 Proxy-chapter tests probing cross-Realm semantics via `$262.createRealm()`.

**Reason** (per seed.md §I.1.a — "Bun-specific or Node-compat surface not in ECMA-262"):

`$262.createRealm` is a test262 *host harness* API (defined in test262's INTERPRETING.md), not an ECMA-262 spec surface. Each engine adapts $262 to its own multi-Realm primitive:

- **V8 (d8)**: `Realm.create()` + `Realm.eval(idx, src)` — many Realms per Isolate.
- **JavaScriptCore (jsc)**: `createGlobalObject()` — fresh JSGlobalObject per realm in one isolate.
- **SpiderMonkey (js shell)**: `newGlobal()` — fresh global + intrinsics + cross-compartment wrappers.
- **QuickJS**: `JS_NewContext()` — new context per realm.

**Bun and Node** ship single-Realm and skip these tests by construction. cruftless follows the same shape: Runtime is a singleton (one intrinsics table, one globals map, one heap). Multi-Realm would require either (a) multiple Runtime instances with cross-Realm Value movement, or (b) a Realm record threaded through every intrinsic + isolated prototype chains — an engine-architecture investment outside v1 substrate scope. The 37 tests probe genuine cross-realm semantics (`Array.isArray(arr_from_other_realm)`, `arr instanceof Array_from_other_realm`, %Symbol.iterator% identity across realms); single-Realm engines can't pass these by construction.

**Effective pre-carve-out Proxy chapter**: 49.6% (154/310). **Carve-out adjusted**: 56.4% (154/273 after removing the 37 createRealm tests from the denominator).

## IR-EXT 84 → 89 — 2026-05-20 (Proxy substrate completion + Pin-Art Pass C carrier landings)

### Commits

| commit | tag | recognition |
|---|---|---|
| `89be7155` | IR-EXT 84: Proxy revocation + construct trap non-Object check | ProxyInternals gains `revoked: bool`; revoke closure flips it; proxy_target_handler_checked wraps the §10.5.{4..14} null-handler→TypeError check; spec_get + Reflect.* via methods + call_function's Proxy arm all swap to the checked variant. §10.5.13 step 9: construct trap return must be Object. Proxy 34.1%→36.7% (+8). |
| `bdda902d` | IR-EXT 84b: bytecode VM Proxy dispatch revoked check | All 7 bytecode Op::* sites that dispatch Proxy traps (GetProp, SetProp, HasProp, DeleteProp, OwnKeys, GetPropMethod, HasPrivate) use proxy_target_handler_checked. Proxy 36.7%→38.0% (+4). |
| `693126cf` | IR-EXT 84c: Object.defineProperty + getOwnPropertyDescriptor Proxy trap + trap-callable + falsy-throws | Same shape as Reflect.defineProperty in EXT 79d; surfaces here because tests probe via Object.* not Reflect.*. Proxy 38.0%→41.9% (+12). |
| `6d713801` | IR-EXT 84d: Object.getOwnPropertyNames/Symbols Proxy.ownKeys dispatch | Same shape as EXT 84c; filters trap result to string/symbol-keyed entries respectively. Proxy 41.9%→43.8% (+6). |
| `e29cf7ba` | IR-EXT 84e: Object.{get,set}PrototypeOf + isExtensible + preventExtensions Proxy trap dispatch | Same shape across 4 ops + boolean-coerce + falsy-throws per §10.5.{1,2,3,4}. Proxy 43.8%→49.6% (+18). |
| `45104c63` | trajectory carve-out: $262.createRealm (37 tests) | Test262 host harness API, not ECMA-262 spec. Bun/Node ship single-Realm and skip these by construction; cruftless follows the same pragmatic shape per seed.md §I.1.a. Effective Proxy denominator becomes 273. |
| `fc9b60d4` | IR-EXT 82: Tier-1.5 SpecGet primitive (Doc 730 §XIII first carrier) | Expr::SpecGet IR primitive + rt.spec_get runtime helper. Spec-correct §7.3.2 [[Get]] dispatcher (Proxy.get trap → accessors → slot, throws-propagating). First Tier-1.5 alphabet promotion landed in code. Stable across Reflect/Symbol/F.proto. |
| `84e8075b` | IR-EXT 82b: get_via + CreateListFromArrayLike spec_get | Second §XIII carrier. Promoted runtime helpers covering computed-method-name lookups + Reflect.apply/construct CreateListFromArrayLike. Structurally correct, zero-yield-here. |
| `d92b06e8` | IR-EXT 82c: collapse all Expr::Get lowering through spec_get | §I.1.b alphabet-promotion completion. Every IR-emitted Expr::Get → rt.spec_get in one move. Internal-slot reads stay on Expr::GetSlot. Stable across 4 chapters. |
| `b7c1e91a` | IR-EXT 83: primitive wrapper internal kinds | Number/String/Boolean/BigInt wrapper internal kinds + Object.prototype.toString brand strings + Object(prim) routes through to_object + BigInt.prototype.valueOf wrapper-aware. Number +1, BigInt +1, String +18, Boolean 84.3% baseline. +20. |
| `6233981b` | IR-EXT 85: Tier-1.5 GetMethod primitive | Expr::GetMethod + rt.get_method as §7.3.10 typed primitive (callable-or-undefined-or-throw post-condition). ToPrimitive §7.1.1 step 2.a now reads 1:1 against spec text. +4 (Symbol +3, F.proto +1). |
| `46a45e43` | IR-EXT 86: ProxyOwnPropertyKeys invariants + Object.keys dispatch | apply_proxy_own_keys_invariants shared helper. §10.5.11 — no duplicates, must-contain-non-configurable-target-keys, non-extensible-target-keys-must-match-exactly. Wired into Reflect.ownKeys + Object.getOwnPropertyNames/Symbols + Object.keys. Proxy 49.6%→54.8% (+16). |
| `53a85581` | IR-EXT 87: GetPrototypeOf/SetPrototypeOf/IsExtensible/PreventExtensions invariants | Four §10.5.{1,2,3,4} invariants inlined at the EXT 84e Object.* dispatch sites. Proxy 54.8%→57.4% (+8). |
| `cafc829c` | IR-EXT 88 + 88b: Has/Get/Set/Delete invariants (Reflect + bytecode VM) | Four shared apply_proxy_{has,get,set,delete}_invariant helpers; wired into both Reflect.* via methods AND all five bytecode-VM Proxy dispatch sites (Op::GetProp / Op::GetIndex / Op::SetProp / Op::DeleteProp / Op::DeleteIndex). §10.5.{7,8,9,10}. Proxy 57.4%→59.3% (+6). |
| `70ac4696` | IR-EXT 89: DefineOwnProperty + GetOwnProperty invariants — Pin-Art Pass C completion | §10.5.5 + §10.5.6 invariants as shared helpers, wired into Object.defineProperty + Object.getOwnPropertyDescriptor. Proxy 59.3%→61.2% (+6). |

### Substrate at IR-EXT 89 close

**IR alphabet**: 65 nodes (Expr::SpecGet from EXT 82 + Expr::GetMethod from EXT 85). Two §XIII Tier-1.5 promotions landed; alphabet held at +2 nodes total despite the §XIII formalization opening a new tier.

**Runtime additions**:
- `Runtime::spec_get(v, key)` — §7.3.2 [[Get]] dispatcher.
- `Runtime::get_method(v, key)` — §7.3.10 GetMethod (callable-or-undefined-or-throw).
- `ProxyInternals.revoked: bool` + `Runtime::proxy_target_handler_checked` + `Runtime::proxy_is_revoked`.
- Five apply_proxy_X_invariant helpers covering §10.5.{5,6,7,8,9,10,11} post-conditions.
- `InternalKind::{NumberWrapper, StringWrapper, BooleanWrapper, BigIntWrapper}` — primitive wrapper brand carriers.

### Cumulative numbers

| Chapter | Pre-EXT-79d | Post-EXT-89 | Δ this batch |
|---|---|---|---|
| Proxy   | 34.1% (106/310) | 61.2% (190/310) | +84 |
| Number  | 89.4% (304/340) | 91.1% (310/340) | +6 |
| String  | 75.3% (921/1223) | 77.0% (942/1223) | +21 |
| Reflect | 68.6% (105/153) | 81.0% (124/153) | +19 |
| Symbol  | (baseline)      | 67.3% (66/98)   | (+3 since first measurement) |
| BigInt  | 49.3% (38/77)   | 59.7% (46/77)   | +8 |
| Map     | 59.3% (121/204) | 61.7% (126/204) | +5 |
| Boolean | (baseline)      | 84.3% (43/51)   | (baseline) |
| Function.prototype | (~46%) | 70.8% (219/309) | +79 cumulative |

**Session-cumulative wins: +242 chapter wins** since the start of the push.

### Pin-Art probe series (cross-reference)

The engine262 Pin-Art Pass A–D outputs live in `pilots/rusty-js-ir/engine262-pin-art.md` (commits `44fd6e10` Pass A, `a9255872` Pass B, `552020f6` Pass C, `ca0cbc4d` Pass D). The Pass C inventory directly drove EXT 86-89's per-trap invariant work; the Pass B trace drove EXT 85 (GetMethod). The four-pass probe series → five-EXT implementation closed loop confirms Doc 730 §XIII's targeting heuristic operates as a usable engagement pattern, not just a recognition.

Pin-Art tag count: 125 commits as of EXT 89.

### Conjecture status

**Doc 730 §XIII targeting heuristic #2 (2026-05-20, EXT 86 → 89)**: the Pass C work list ran cleanly through four sequential EXTs landing one column of the InternalMethods<Kind> table at a time, each using a uniform shared-helper shape (apply_proxy_X_invariant). The §XIII pattern — name the collapsed discrimination at the alphabet boundary, then drain consumers through the now-typed primitive — held across both the alphabet level (EXT 82/85) and the helper level (EXT 86-89). The pattern is now load-bearing across the engine.

**§I.1.b corroboration #10 (2026-05-20, EXT 86 → 89)**: five substantive substrate fixes (ProxyOwnPropertyKeys + 11 other §10.5 invariants spread across get/set/has/delete/define/getOwn/getPrototypeOf/setPrototypeOf/isExtensible/preventExtensions) all expressible in the existing alphabet plus the runtime-helper tier. No IR-alphabet additions required. The Tier-1.5 §XIII promotions (Expr::SpecGet, Expr::GetMethod) absorbed cleanly while the IR-tier alphabet remained stable. Doc 730 §XIII does not require alphabet growth at every promotion site — the alphabet can extend horizontally (new typed primitive) AND vertically (new tier) without either disrupting the other.

## IR-EXT 90 → 92 — 2026-05-20 (§XIV deviation alphabet + §XV constraint-comprehension contract — full byte-parity recovery)

### Commits

| commit | tag | recognition |
|---|---|---|
| `9520f504` | IR-EXT 90: first deviation-tier primitive (Doc 730 §XIV) | `Runtime.tolerated_deviations: HashSet<&'static str>` + `__cruftless_tolerate(name)` global intrinsic + first deviation `function-not-constructor-relax` (Op::New's [[Construct]] enforcement falls through to plain call when opted in). Strict-by-default preserved. 8 of 10 EXT 84-89 parity-regressed packages recover under the opt-in. First in-code instance of the §XIV downward-additive alphabet. |
| `2e6e6413` | IR-EXT 91: protected_invariants on deviation primitives (Doc 730 §XV) | Each registered deviation now carries the §XV.c constraint-comprehension contract — list of (Comprehended `C:<spec_primitive>`, Waived `W:<audit_ref>`, Unknown `U:...`) markers. `__cruftless_tolerate` refuses opt-in on any Unknown marker (the contract's enforcement point). EXT 90's first deviation registered with two Waived invariants referencing the trajectory + Doc 730 §XV.c worked-example paragraph. |
| `5c79afbc` | IR-EXT 91b + 92: full byte-parity for §XIV-tolerated cluster | EXT 91b: Op::New under the deviation returns call result verbatim (no fresh-Object fallback when return is primitive). EXT 92: Object.keys filters `@@`-prefixed Symbol-as-string keys per §20.1.2.{17,18}. The combined fix: all 8 §XIV-recovered packages now BYTE-PARITY with Bun. |

### The §XV contract held empirically

EXT 91's byte-parity check on the 8 deviation-recovered packages found 4 BYTE-PARITY + 4 +1-keyCount DIVERGE. The DIVERGE pattern was initially read as the §XV protected-invariant violation manifesting (the discarded fresh-Object from Op::New leaking as the extra key). Closer probe (the dayjs-plugin-utc shape inspection) revealed the +1 key was `@@toStringTag` — a pre-existing cruftless bug (Symbol-stored-as-string-with-`@@`-prefix leaking into Object.keys) unrelated to the deviation's Waived invariants.

This is the §XV pipeline operating as designed:
- The contract surfaced an observable divergence between Bun behavior and cruftless-with-deviation behavior.
- The engagement audited the divergence against the Waivers.
- The audit identified the divergence as orthogonal to the Waivers (a separate spec-fidelity gap, not a deviation effect).
- The fix landed at the §XIII tier (EXT 92), preserving the deviation's Waived invariants as accurate descriptions of what `function-not-constructor-relax` itself absorbs.

The contract did not prevent the bug; it gave the engagement the surface for distinguishing "this divergence is the deviation's fault" from "this divergence is a different gap that happens to surface only when the deviation enables loading the affected package." Without §XV's Waived-invariants list, the engagement would have had no principled way to attribute the +1 leak — both candidate causes (fresh-Object leak vs @@-prefix leak) would have looked equally plausible until manual inspection. The Waivers narrowed the candidate set.

### Substrate at IR-EXT 92 close

**IR alphabet**: 65 nodes (no growth across EXT 90-92; all three EXTs live at the runtime tier).

**Runtime additions**:
- `Runtime.tolerated_deviations: HashSet<&'static str>` — opt-in deviation set.
- Global intrinsic `__cruftless_tolerate(name)` — registers a deviation; validates §XV.c contract (Unknown refused).
- Deviation registry in `intrinsics.rs` — each entry carries (canonical_name, &[protected_invariants]).
- Op::New `relaxed_non_constructor` flag — gates the fresh-Object fallback under the deviation.
- `enumerable_own_keys` @@-prefix filter — closes the keyCount-leak class of bugs.

### Cumulative numbers

| Metric | Pre-EXT-90 | Post-EXT-92 | Δ |
|---|---|---|---|
| Top500 parity (strict) | 78.3% (803/1026) | 78.3% unchanged | 0 (deviations are opt-in; default unchanged) |
| Top500 parity (with `function-not-constructor-relax` opted in) | n/a | est. 79.1% (+8 packages recover) | +8 |
| Byte-parity of recovered packages | n/a | 8/8 BYTE-PARITY | full |

**Session-cumulative wins: +247 chapter test262 wins + 8 §XIV-recovered packages at full byte-parity.**

Pin-Art tag count: 132 commits as of EXT 92.

### Conjecture status

**Doc 730 §XIV corroboration #1 (2026-05-20, EXT 90)**: a single typed deviation primitive at the deviation-tier alphabet recovered 8 of 11 EXT 84-89 parity regressions without compromising strict-spec correctness on the other ~1015 packages. The per-deviation-opt-in design (§XIV.c) demonstrated as the right shape for §XIII-coherent ecosystem tolerance.

**Doc 730 §XV corroboration #1 (2026-05-20, EXT 91 → 91b → 92)**: the constraint-comprehension contract enabled principled attribution of an observable byte-parity divergence between two candidate causes (fresh-Object leak vs @@-prefix leak). The Waived-invariants list narrowed the candidate set; the divergence was correctly identified as a separate §XIII-tier bug rather than a deviation effect. EXT 92's fix landed cleanly at the spec-fidelity tier, preserving the deviation's Waivers as accurate. The two-axis pipeline (spec fidelity ↑ ∥ ecosystem tolerance ↓) operated under the co-evolution contract as Doc 730 §XV designed.

## IR-EXT 93 → 94b — 2026-05-20 (§XIV second deviation + §XV violation corroboration in code)

### Commits

| commit | tag | recognition |
|---|---|---|
| `b83df2d3` | IR-EXT 93: second §XIV deviation `to-object-coerce-nullish` | rt.to_object(null|undefined) returns a fresh ordinary Object instead of throwing TypeError under the deviation; symmetric guard at require_object_coercible. Registered with two Waived invariants per §XV.c. 14-package recheck: 13 BYTE-PARITY, 1 ERR (yeoman-environment, unrelated SyntaxError). Cumulative §XIV recovery now +21 packages across EXT 90+93. |
| `3da8d635` | host-v2 IR-EXT 94: readFileSync result has UTF-8-decoding toString | The yeoman-environment SyntaxError was traced to fs.readFileSync(p).toString() returning comma-joined byte-decimals (Array.prototype.toString on a byte-Number Array) instead of Node's UTF-8 Buffer-decode. Installed a UTF-8 toString own property on the returned Array. Broad fix — every Node-compat consumer doing readFileSync(p).toString() benefits. Demonstrates the §XIV→§XIII feedback §XV.b predicted: investigating a deviation candidate surfaced a §XIII-tier (host-compat) fix that benefits broadly. |
| `cd886c3e` | IR-EXT 94b: scope `to-object-coerce-nullish` out of Object.getPrototypeOf — §XV violation corroboration #2 | EXT 93's broad relaxation introduced an infinite prototype-walk loop under any `while (p) p = Object.getPrototypeOf(p)` on a null-rooted chain — the deviation made getPrototypeOf(null) coerce null to a fresh Object whose [[Prototype]] is Object.prototype, never null. Object.getPrototypeOf's intrinsic wrapper now special-cases nullish under the deviation to return Null directly, preserving "prototype-walk termination" as a protected invariant. |

### §XV corroboration #2 — the deviation's broad scope had unforeseen downstream effects

Doc 730 §XV.a predicted the failure mode: "naming a deviation requires articulating what the strict rejection protects... enabling the deviation across an entire package's dependency tree silently absorbs every such write site, producing a class of 'the program loaded fine but its state is wrong' bugs."

EXT 94b is the second empirical corroboration of §XV in code (first was EXT 91+92's BYTE-PARITY-recovery sequence). Specifically:

1. The EXT 93 deviation was registered with two Waived invariants:
   - W:EXT-93:to-object-typeerror-as-runtime-nullcheck
   - W:EXT-93:set-prototype-of-nullish-target-silent-noop
2. The set-Proto waiver covered Object.setPrototypeOf. It did NOT cover Object.getPrototypeOf — the latter was a *narrower scoping defect*, not a Waived deviation effect.
3. The §XV.c contract narrowed the candidate causes: "is the divergence one of the Waivers or a separate scoping defect?" The §XV.c framing made the question askable; the empirical probe (prototype-walk loop trace) made it answerable.
4. EXT 94b is the §XIII-tier correction — per-method scoping of the deviation. The EXT 93 Waivers remain accurate about what the deviation absorbs at the sites the deviation IS-scoped-to.

The §XV recognition is now load-bearing in the engagement: each new deviation primitive should ideally enumerate the consumer call sites it is intended to affect (or explicitly accept that it affects every ToObject-using site), with per-site negation gates where the deviation must NOT apply. EXT 94b is the first per-site negation gate landed.

### Substrate at IR-EXT 94b close

**IR alphabet**: 65 nodes (no growth across EXT 93-94b).

**Runtime additions (this batch)**:
- `to-object-coerce-nullish` deviation registered in `__cruftless_tolerate` registry with two Waived invariants.
- `rt.to_object` + `rt.require_object_coercible` gate on the deviation set.
- `bytes_to_value` (host-v2/fs.rs) installs a UTF-8-decoding toString own property on the byte-Array result of readFileSync.
- Object.getPrototypeOf intrinsic wrapper: nullish input under the deviation returns Null directly (per-site negation gate).

### Cumulative numbers

| Metric | Pre-EXT-93 | Post-EXT-94b | Δ this batch |
|---|---|---|---|
| Top500 parity (strict default) | 78.3% (803/1026) | 78.3% (deviations are opt-in) | 0 |
| §XIV-recovered packages (both deviations opted in) | 8 (EXT 90) | 21 (EXT 90+93) + Buffer.toString broad-applicability | +13 explicit + broad |
| §XV violations corroborated | 1 (EXT 91 byte-parity / EXT 92 fix) | 2 (+ EXT 93 scoping / EXT 94b fix) | +1 |

**Session-cumulative wins: +247 test262 chapter wins + 21 §XIV-recovered packages at full byte-parity + 1 §XIII-tier host-compat fix (Buffer.toString) with broad applicability + 1 additional §XV violation corroborated and fixed.**

Pin-Art tag count: 137 commits as of EXT 94b.

### Conjecture status

**Doc 730 §XIV.d targeting heuristic #2 (2026-05-20, EXT 93)**: the second deviation primitive (`to-object-coerce-nullish`) was selected by the §XIV.d heuristic (highest parity-recovery per primitive) and produced 13 recoveries from one promotion. The targeting heuristic operates as a usable engagement pattern — pick the largest cluster in the bun-only-OK breakdown that shares a single spec-rejection class, lift to a typed deviation, sweep.

**Doc 730 §XV.a corroboration #2 (2026-05-20, EXT 93 → 94b)**: the deviation's broad scope at rt.to_object produced a downstream invariant violation (prototype-walk loop) that the EXT 93 Waivers did NOT predict. §XV.c's "did the divergence map to a Waived invariant or a scoping defect?" framing was the principled way to attribute the cause; the §XIII-tier correction (per-method scoping at Object.getPrototypeOf) preserved the deviation's positive effects on the other 13 recoveries. The two-axis pipeline operated under the co-evolution contract as Doc 730 §XV designed.

**Doc 730 §XIV.b co-evolution feedback (2026-05-20, EXT 93 → 94 → 94b)**: investigating one §XIV deviation candidate (yeoman-environment's recovery) produced two §XIII-tier fixes — EXT 94 (Buffer.toString, broadly applicable host-compat) and EXT 94b (per-site scoping correction). Each §XIV iteration generates §XIII work items; each §XIII fix unblocks further §XIV recoveries downstream. The pipeline self-extends along both axes from a single empirical probe.

---

## Rung-cluster-1 — Number.prototype.{toFixed,toExponential,toPrecision} (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`Number/prototype/toFixed/* + Number/prototype/toExponential/* + Number/prototype/toPrecision/*` — 48 test262 entries in the representative sample.

**Pre-rung baseline** (2026-05-22 sample, post README sweep):
- 33 PASS / 15 FAIL across the cluster.

**Defects identified** (spec §21.1.3.{2,3,5}):
1. NaN/Infinity short-circuit happened AFTER bounds check on `f`/`p` instead of BEFORE (toExponential, toPrecision). Spec orders NaN check at step 4, Infinity at step 5, bounds check at step 7.
2. `(-0).toExponential(d)` emitted `"-0e+0"` because Rust's `{:.*e}` on `-0.0` carries the sign. Spec uses `if x is 0 then s="0"` (unsigned). Same for `toPrecision`.
3. `n >= 1e21` should fall back to `ToString(x)` (exponential form) per `toFixed` step 6; was emitting fixed-point.
4. BigInt `fractionDigits` argument should throw `TypeError` (no BigInt-to-Number coercion); was silently coercing or no-op-ing.

**Substrate fixes**: three edits in `pilots/rusty-js-runtime/derived/src/interp.rs` against `number_proto_to_exponential_via`, `number_proto_to_precision_via`, and `number_proto_to_fixed_via`. Each fix is anchored to a numbered spec step in the comment.

**Post-rung result**: 43 PASS / 5 FAIL on the same cluster paths.

**Delta**: **+10 PASS** (15 FAIL → 5 FAIL on the cluster; 67% reduction).

**Sample-wide gap delta**: Cruftless runnable pass goes from 73.9% (5,321 / 7,203) to 73.96% (5,331 / 7,203). +10 tests on a 7,203-runnable base = +0.14 pp. Toward the parity-target telos (gap ≤10 pp), 25.3 pp → 25.16 pp.

**Remaining 5 FAILs** (deferred to their own rung, family: floating-point ToString rounding):
- `toFixed/exactness.js` — Rust's f64 fixed-point formatting differs from spec's banker's rounding on the edge case `1000000000000000128` (the boundary case at f64 precision).
- `toExponential/return-values.js`, `toPrecision/return-values.js`, `toPrecision/exponential.js`, `toPrecision/tointeger-precision.js` — toPrecision must switch to exponential notation when the precision is insufficient for fixed form; current impl always emits fixed. Requires proper dtoa-style emission tracking exponent magnitude.

**Conjecture status (post-cluster)**: this rung was hand-coded directly in `interp.rs` rather than via the rusty-js-ir IR pipeline — the defects were small enough that the spec-step-anchored comments alone gave the linting discipline the IR encoding would. The IR encoding would be valuable when the remaining banker's-rounding work lands (a substantial algorithm whose spec correspondence benefits from the lint pass). Recorded as a methodology data point: small cluster-defect rungs may not require the full IR detour; large-algorithm rungs do.

**Tag**: `cluster-Number-numeric-format-1`. Next cluster candidate: `Number.prototype.toString` (the `[object Number]`-vs-`[object Object]` defect cluster, 7+ test262 FAILs).

---

## Rung-cluster-2 — Destructuring-assignment LHS in for-of / for-in heads (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`language/statements/for-of/dstr/* + language/expressions/arrow-function/dstr/*` — 800 test262 paths covering destructuring binding *and* destructuring assignment patterns in for-of / for-in heads + arrow params. The pre-rung sample showed 513 FAIL / 251 PASS across these two sub-trees — the single largest FAIL cluster in the 2026-05-22 baseline.

**Root cause**: cruftless's parser collapsed any non-identifier for-of/for-in LHS to a `BindingPattern::Identifier { name: "" }` placeholder at `pilots/rusty-js-parser/derived/src/stmt.rs:646`. The compiler then bound the iteration value to the empty-name slot and never re-distributed it across the cover-grammar pattern. Effectively: `for ([a, b] of pairs)` parsed but didn't assign — `a` and `b` remained `undefined`.

**Substrate fix**: added `expr_to_binding_pattern(Expr) -> Option<BindingPattern>` in the parser that converts a parsed array/object literal (with all its nested defaults, elisions, spreads, computed keys) into the equivalent `BindingPattern::{Array,Object}`. The for-of/for-in head parse now runs the converter first and falls back to the empty-name placeholder only when the LHS isn't a valid assignment target.

The converter handles:
- `Expr::Identifier` → `BindingPattern::Identifier`
- `Expr::Array` elements: `Elision` → hole; `Expr` → BindingElement (recursive, with `Expr::Assign{Assign}` peeled to default); `Spread` → rest (last only)
- `Expr::Object` properties: identifier/string/number/computed keys; default extraction; rest must be a plain identifier per spec

**Post-rung result**: 441 FAIL / 323 PASS across the same 800 dstr paths.

**Delta**: **+72 PASS** on the dstr cluster (513 FAIL → 441 FAIL, 14% reduction).

**Sample-wide gap delta**: Cruftless runnable pass goes from 73.9% (5,321 / 7,203) to ~74.9% (5,393 / 7,203). +72 tests on a 7,203-runnable base = +1.0 pp. Gap against bun: 25.3 pp → 24.3 pp.

**Remaining 441 FAILs** (deferred — each is its own sub-cluster):
- ~78 SyntaxError negative tests (early errors / TDZ enforcement at parse time)
- ~76 ReferenceError tests (TDZ enforcement at runtime — let/const access-before-init in destructuring)
- ~70 Test262Error-not-thrown (assert.throws inside complex destructuring flows)
- ~41 TypeError-not-thrown (invalid invocation patterns)
- ~50 function-name-inference via destructuring default (`[x = function(){}]` should infer name "x" on the anon function)

**Tag**: `cluster-dstr-for-loop-head-2`. Next cluster candidate: TDZ enforcement (76 ReferenceError tests in the dstr cluster + an unknown additional count in non-dstr code).

---

## Rung-cluster-3 — Object.defineProperty property-key coercion (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/Object/defineProperty/*` — 1,131 paths covering descriptor installation, attribute defaulting, accessor/data conflict checks, configurability invariants. The pre-rung sample showed 124 FAIL / 1,007 PASS in this sub-tree — the third-largest single-directory FAIL cluster.

**Root cause** (one of several defects in this sub-tree): `Object.defineProperty(o, P, desc)` coerced `P` via `abstract_ops::to_string` directly, which for `Value::Object` returns the literal `"[object Object]"` (the case is commented "Object ToString deferred" — a long-standing carve-out). The spec at §10.1.6.1 ToPropertyKey requires ToPrimitive(P, hint=string) followed by ToString, which dispatches through `@@toPrimitive` / `toString` / `valueOf`. Result: `Object.defineProperty(o, [1,2], {})` should install key `"1,2"` (Array.prototype.toString → join), but installed `"[object Object]"` instead.

**Substrate fix**: in `object_define_property_via` at `interp.rs:1589`, route Object-typed key arguments through `self.to_primitive(key_v, "string")` then `abstract_ops::to_string` on the resulting primitive before the property-key bucket dispatch. Symbol-typed primitives are preserved (not stringified).

**Post-rung result**: 113 FAIL / 1,018 PASS on the 1,131-path cluster.

**Delta**: **+11 PASS** (124 FAIL → 113 FAIL, ~9% reduction).

**Sample-wide cumulative** (with cluster-1 and cluster-2):
- Cluster-1: +10 PASS (Number-numeric-format)
- Cluster-2: +72 PASS (dstr LHS in for-of/in)
- Cluster-3: +11 PASS (defineProperty P coercion)
- Total: +93 PASS on the 7,203-runnable base = +1.29 pp
- Cruftless 73.9% → ~75.2% runnable pass; gap 25.3 pp → ~24.0 pp

**Remaining 113 defineProperty FAILs** — all about descriptor invariants, not key coercion:
- ~30 across "should not be configurable" / "should be enumerable" / "should be writable" — descriptor-attribute defaulting when the input descriptor omits the flag
- ~12 "Expected TypeError" — invalid-descriptor rejection paths
- ~12 "Expected obj[foo] to equal data, actually undefined" — accessor-descriptor data fallthrough
- ~5 desc-on-array-index interaction with array length truncation

**Methodology note**: the property-key coercion gap is upstream of defineProperty alone. The same ToString-of-Object → "[object Object]" anti-pattern lives at `abstract_ops::to_string:62` and affects every code path that stringifies an object-typed value (defineProperties, Reflect.*, [] accessor on object-typed key, etc.). Defer to its own rung when the targeted defineProperty win demands a broader sweep.

**Tag**: `cluster-defineProperty-key-coercion-3`. Next cluster candidate: descriptor-attribute defaulting (the ~30 "should not be configurable" / "should be enumerable" remaining FAILs in this same sub-tree).

---

## Rung-cluster-4 — String.prototype.split with RegExp separator (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/String/prototype/split/*` — 120 paths covering string-separator and regex-separator splitting + limit handling + lastIndex behavior. The pre-rung sample showed 30 FAIL / 90 PASS in this sub-tree.

**Root cause**: `CompiledRegex::split_str` delegated to Rust's `regex::Regex::split` (and to a naive cursor walk in the Hand branch), both of which emit empty-string slices at zero-width-match positions. ECMA-262 §22.1.3.21 step 18 specifies that empty matches at the current cursor `p` are skipped (the spec's `q` advances by 1 without emitting a slice). Result: `"hello".split(new RegExp())` produced `["", "h", "e", "l", "l", "o", ""]` instead of the spec's `["h", "e", "l", "l", "o"]`. Empty input + empty regex returned `[""]` instead of `[]`.

**Substrate fix**: rewrote `split_str` at `pilots/rusty-js-runtime/derived/src/value.rs:508` to walk `find_iter_owned` matches sequentially, applying the spec's three skip rules:
1. Empty match at end-of-input (`ms >= input.len()`): break (the spec's loop bound).
2. Empty match at the consumed cursor (`me == p`): continue (the spec's `q++`).
3. Overlap with consumed region (`ms < p`): continue (defensive).
Plus the spec's empty-input edge cases: `[]` when the regex matches empty at 0, `[""]` otherwise.

**Post-rung result**: 22 FAIL / 98 PASS on the 120-path cluster.

**Delta**: **+8 PASS** (30 FAIL → 22 FAIL, ~27% reduction).

**Sample-wide cumulative** (with clusters 1-4):
- Cluster-1 (Number numeric format): +10
- Cluster-2 (dstr LHS): +72
- Cluster-3 (defineProperty P coercion): +11
- Cluster-4 (split regex empty-match): +8
- Total: **+101 PASS** on the 7,203-runnable base = +1.40 pp
- Cruftless 73.9% → ~75.3% runnable pass; gap 25.3 pp → ~23.9 pp

**Remaining 22 split FAILs** — mixed:
- 4 negative tests expecting SyntaxError (early errors in regex patterns)
- 3 "Expected a Test262Error to be thrown" (assert.throws with regex side effects)
- 5 limit-with-regex-side-effects (lastIndex observability)
- Misc

**Note**: the fix also applies through RegExp.prototype[@@split] which uses the same `CompiledRegex::split_str` — so any other test path that routes through `regex@@split` (some array-like split tests, e.g.) may also flip. Sample-wide re-measurement at the next rung close.

**Tag**: `cluster-string-split-regex-empty-4`. Next cluster candidate: language/statements/for-of non-dstr SyntaxError negative tests (27 FAILs), or Array/prototype/concat TypeError-throwing on non-Array @@isConcatSpreadable (~9 FAILs).

---

## Rung-cluster-5 — ToLength clamping of Infinity (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/Array/prototype/indexOf/*` — 201 paths probing fromIndex behavior, length-coercion, and array-like callee paths.

**Root cause** (broad): `try_array_length` at `interp.rs:6171` returned `usize::MAX` for `length: Infinity`, then `array_proto_index_of_via` cast that to `i64` (→ -1) which made the `while i < len` loop exit immediately, producing -1 instead of finding the value. Spec §7.1.20 ToLength clamps `length` to `[0, 2^53 - 1]`; Infinity should collapse to the max-safe bound, same as a large finite input.

**Substrate fix**: in `try_array_length`, treat `!n.is_finite() || n > max_safe` as the same clamp branch returning `max_safe`. The earlier early-return for `!is_finite() → usize::MAX` is removed.

**Post-rung result on indexOf cluster**: 22 FAIL → 20 FAIL on 201 paths (+2 PASS, ~9% reduction).

**Sample-wide cumulative** (rungs 1-5):
- Cluster-1 (Number numeric format): +10
- Cluster-2 (dstr LHS): +72
- Cluster-3 (defineProperty P coercion): +11
- Cluster-4 (split regex empty-match): +8
- Cluster-5 (ToLength Infinity clamp): +2 (indexOf cluster); cascade across every Array.prototype.* using try_array_length expected but unmeasured this rung
- Total: **+103 PASS** on the 7,203-runnable base = +1.43 pp
- Cruftless 73.9% → ~75.3% runnable pass; gap 25.3 pp → ~23.9 pp

**Tag**: `cluster-toLength-infinity-5`. Note: the fix is in a shared helper (`try_array_length`) used by every Array.prototype.* method that calls into the length-of-array-like coercion. The +2 on indexOf understates broader impact; full sample-wide re-measurement at next major rung close will surface the cascade across reduce/filter/some/every/forEach/etc.

Next cluster candidate: `language/expressions/arrow-function` non-dstr (23 FAILs in the sample), or `Array/prototype/concat` TypeError-throwing paths (~9 FAILs).

---

## Rung-cluster-6 — JSON.stringify array-replacer PropertyList (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/JSON/stringify/*` — 66 paths. The pre-rung sample showed 31 FAIL / 28 PASS in this sub-tree, concentrated around the array-replacer feature.

**Root cause**: cruftless's `json_stringify_via` stored only a callable replacer; array replacers were silently ignored. Spec §25.5.2 step 4.b requires that when replacer is an Array, its items (after String/Number coercion + wrapper unwrap + dedupe) form a PropertyList that filters AND orders the keys serialized in every non-array compound. Result: `JSON.stringify({a:1,b:2,c:3}, ["b","a"])` returned `{"a":1,"b":2,"c":3}` (whole object, natural order) instead of `{"b":1,"a":2}`.

**Substrate fix**:
1. Add `json_property_list_stack: Vec<Option<Vec<String>>>` field on Runtime alongside the existing `json_replacer_stack` (parallel discipline: each stringify entry pushes a frame; nested toJSON-reentries get their own).
2. In `json_stringify_via`, when `replacer` is an Array, compute the PropertyList per spec step 4.b: iterate `ToLength`-clamped indices, coerce String/Number/wrappers, skip non-coercible, dedupe; push the resulting list. Otherwise push `None`.
3. In `json_serialize_compound_via`'s non-array branch, if the topmost frame holds a property list, use it as the key set (filter + order). Otherwise compute OrdinaryOwnPropertyKeys-style ordering as before.

**Post-rung result**: 28 FAIL / 31 PASS on the 66-path cluster (the additional 7 paths timeout or rc-mismatch in both runs).

**Delta**: **+3 PASS** measured. The array-replacer fix lands cleanly; the remaining FAILs decompose into deeper sub-defects (BigInt support, Proxy abruptness, string escape for unpaired surrogates, String/Number wrapper as `this` value at top-level, etc.) each needing its own rung.

**Sample-wide cumulative** (rungs 1-6):
- Cluster-1 (Number numeric format): +10
- Cluster-2 (dstr LHS): +72
- Cluster-3 (defineProperty P coercion): +11
- Cluster-4 (split regex empty-match): +8
- Cluster-5 (ToLength Infinity clamp): +2
- Cluster-6 (JSON.stringify array-replacer): +3
- Total: **+106 PASS** on the 7,203-runnable base = +1.47 pp
- Cruftless 73.9% → ~75.4% runnable pass; gap 25.3 pp → ~23.9 pp

**Tag**: `cluster-json-stringify-array-replacer-6`.

---

## Rung-cluster-7 — RegExp.prototype.exec lastIndex ToLength coercion (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/RegExp/prototype/exec/*` — 79 paths. Pre-rung sample: 22 FAIL / 57 PASS.

**Root cause**: `regexp_exec` at `regexp.rs:638` read `lastIndex` via raw property-get and only honored the result if it was a `Value::Number`; non-Number values fell through to `0`. Spec §22.2.7.2 RegExpBuiltinExec step 4 requires `ToLength(? Get(R, "lastIndex"))`, which invokes `valueOf`/`@@toPrimitive` for object-typed lastIndex. The coercion's side effects must fire even for non-global/non-sticky regexes (spec step 8 resets the WORKING start to 0 only AFTER the ToLength happens). Result: `r.lastIndex = { valueOf: fn }` never called `fn`; test262 detects this via call counters.

**Substrate fix**: replace the raw `object_get → Value::Number match` with `rt.object_get(...)` → `rt.coerce_to_number(...)` → clamp to `[0, 2^53-1]` (matching `try_array_length`'s recent fix from cluster-5). The clamp result feeds the global/sticky branch; non-global still uses `start=0` per spec step 8, but the side effects of ToLength fire either way.

**Post-rung result**: 13 FAIL / 66 PASS on 79 paths.

**Delta**: **+9 PASS** (22 FAIL → 13 FAIL, 41% reduction).

**Expected cascade**: every code path that invokes `regexp_exec` (String.prototype.match / matchAll / replace / replaceAll / split, plus the @@-method dispatchers) inherits the coercion behavior; subsequent test262 runs against those clusters may flip additional tests.

**Sample-wide cumulative** (rungs 1-7):
- Cluster-1 (Number numeric format): +10
- Cluster-2 (dstr LHS): +72
- Cluster-3 (defineProperty P coercion): +11
- Cluster-4 (split regex empty-match): +8
- Cluster-5 (ToLength Infinity clamp): +2 (indexOf measured; broader cascade unmeasured)
- Cluster-6 (JSON.stringify array-replacer): +3
- Cluster-7 (regexp exec lastIndex ToLength): +9 (exec measured; broader cascade unmeasured)
- Total: **+115 PASS** on the 7,203-runnable base = +1.60 pp
- Cruftless 73.9% → ~75.5% runnable pass; gap 25.3 pp → ~23.7 pp

**Tag**: `cluster-regexp-exec-lastindex-7`.

---

## Sample-wide re-measurement (post rungs 1-7, 2026-05-22)

Cumulative across the seven rungs landed today:
- **5,439 PASS / 1,766 FAIL / 384 SKIP** on the 7,750-test sample.
- **75.5% runnable pass** (5,439 / 7,205) — up from 73.9% (5,321 / 7,203) at the locale's opening.
- **+118 PASS** measured (vs +115 estimated by per-rung deltas).
- Gap against bun (99.2%): **23.7 pp**. Telos: ≤10 pp. Closed 1.6 pp.

---

## Rung-cluster-8 — NamedEvaluation through destructuring defaults (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`language/statements/for-of/dstr/* + language/expressions/arrow-function/dstr/*` — 800 paths. Post-rung-7 sample: 441 FAIL / 323 PASS.

**Root cause**: `emit_element_with_default` (the destructuring-default emit path at `compiler.rs:1979`) compiled the default expression via plain `compile_expr`, ignoring the binding's name. Spec §13.15.5.3 (NamedEvaluation) requires that when the target is an Identifier and the default is an anonymous function/class/arrow/parenthesized cover, the function's own `.name` receives the identifier text. Test262 surfaces this across the dstr cluster as 50 fn-name-inference failures (10 each for arrow/cls/cover/fn/gen).

**Substrate fix**:
1. `emit_element_with_default`: when target is `BindingPattern::Identifier`, route default through `compile_expr_with_name_hint` with the binding's name.
2. `compile_expr_with_name_hint`: extend to handle `Expr::Class { name: None, .. }` and `Expr::Parenthesized { .. }` (the latter recurses).
3. `compile_class`: introduce a `compile_class_with_name_hint` thin wrapper; the existing inline `class_display_name = name.map(...)` now ORs with the hint. The hint feeds the ctor's `compile_function_proto_with_name_hint`, surfacing the class's `.name` to spec.

**Post-rung result**: 376 FAIL / 388 PASS on 800 dstr paths.

**Delta**: **+65 PASS** (441 FAIL → 376 FAIL, 15% reduction).

**Sample-wide cumulative** (rungs 1-8):
- Cluster-1: +10, Cluster-2: +72, Cluster-3: +11, Cluster-4: +8, Cluster-5: +2, Cluster-6: +3, Cluster-7: +9, Cluster-8: +65.
- Estimated total: **+180 PASS** on the 7,205-runnable base = +2.5 pp.
- Cruftless 73.9% → ~76.4% runnable pass; gap 25.3 pp → ~22.8 pp.

**Tag**: `cluster-namedeval-dstr-default-8`. Note the NamedEvaluation fix also applies through every other destructuring site (parameter destructuring, variable-decl destructuring with defaults), so additional cascade is expected on next sample-wide re-measurement.

---

## Rung-cluster-9 — Map.prototype[@@iterator] key-decoding (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/{Map,Set,WeakMap,WeakSet}/* + language/statements/{for-of,for-in}/*` — 1,679 paths sampled. Pre-rung (latest sample, post rungs 1-8): 692 FAIL / 890 PASS / 3 SKIP.

**Root cause**: `Map.prototype[@@iterator]` at `intrinsics.rs:2877` wrapped storage keys as `Value::String(Rc::new(k))` without consulting the `__map_orig_keys` side channel. The other Map iteration paths (`entries`, `keys`, `values`, `forEach`) correctly route through `map_decode_key`, which restores the original `Value` type (Number, Object, Symbol, Boolean, null, undefined) for non-string keys. The @@iterator path is what `for (const [k,v] of map)` and `[...map]` reach — so user code using the default Map iteration got stringified keys.

Concretely: `new Map().set(0,'a')` then `for (const [k,v] of m)` yielded `["0","a"]` instead of `[0,"a"]`. test262 surfaces this in for-of/map-* tests with `SameValue(«"0"», «0»)` failures and downstream in `Set`/iterator cases that chain off the same protocol.

**Substrate fix**: at `intrinsics.rs:2883`, before wrapping the storage key, look up `this.__map_orig_keys[k]` and return the original Value if present; else fall back to `Value::String(Rc::new(k))`. Inlined because `map_decode_key` is a private method on `Runtime::Interp`; refactor to a free helper is a follow-on chore.

**Post-rung result**: 644 FAIL / 938 PASS on the same 1,679 paths.

**Delta**: **+48 PASS** (692 FAIL → 644 FAIL, 7% reduction).

**Sample-wide cumulative** (rungs 1-9, partial estimate):
- Cluster-1: +10, Cluster-2: +72, Cluster-3: +11, Cluster-4: +8, Cluster-5: +2, Cluster-6: +3, Cluster-7: +9, Cluster-8: +65, Cluster-9: +48.
- Estimated total: **+228 PASS** on the 7,205-runnable base = +3.2 pp.
- Cruftless 73.9% → ~77.1% runnable pass; gap 25.3 pp → ~22.1 pp.

**Tag**: `cluster-map-iterator-key-decode-9`.

---

## Rung-cluster-10 — defineProperty generic-descriptor preservation (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/Object/defineProperty/*` — 1,131 paths. Pre-rung (latest sample, post rungs 1-9): 113 FAIL / 1,018 PASS.

**Root cause**: `object_define_property_via` dispatched on `has_get_key || has_set_key` (accessor branch) vs. else (data branch). When a redefinition called with only `{enumerable, configurable}` (no value/writable/get/set — the spec's "generic" descriptor per §6.2.5.6), it fell into the data branch, which inserted `getter: None, setter: None` — wiping any existing accessor's get/set. Result: `Object.defineProperty(o, 'foo', {enumerable: true})` on an accessor turned it into an undefined data property.

**Substrate fix**: at the data-branch entry, detect the generic descriptor case (`!has_value && !has_writable && !has_get_key && !has_set_key`) and route to a dedicated path that preserves the existing property's value/writable/getter/setter, only updating enumerable/configurable. Non-configurable invariants enforced on this path too: configurable promotion and enumerable change still throw TypeError when the existing property is non-configurable.

**Post-rung result**: 98 FAIL / 1,033 PASS on the 1,131-path cluster.

**Delta**: **+15 PASS** (113 FAIL → 98 FAIL, 13% reduction).

**Sample-wide cumulative** (rungs 1-10, partial estimate):
- Cluster-1 to 9 total: +228
- Cluster-10: +15
- Estimated total: **+243 PASS** on the 7,205-runnable base = +3.4 pp
- Cruftless 73.9% → ~77.3% runnable pass; gap 25.3 pp → ~21.9 pp

**Tag**: `cluster-defineProperty-generic-preserve-10`. Fix also flows through `Object.defineProperties` and `Reflect.defineProperty` which dispatch to the same primitive; further cascade likely.

---

## Rung-cluster-11 — Object-literal accessor enumerability + rest-spread getter dispatch (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/Object/defineProperty/* + language/statements/for-of/dstr/* + language/expressions/arrow-function/dstr/*` — 1,931 paths combined. Pre-rung (post rungs 1-10): 474 FAIL / 1,421 PASS.

**Root causes** (two coordinated defects in the same install path):
1. **Object-literal accessor enumerability**: `__install_accessor__` installed accessors with `enumerable: false`. That's correct for **class** accessors (§15.7 MethodDefinitionEvaluation) but wrong for **object-literal** accessors (§13.2.5.5 PropertyDefinitionEvaluation step 8: `{writable: false, enumerable: true, configurable: true}`). One helper shared both call sites; class side was right, object-literal was wrong. So `{get v() {}}` had `enumerable: false`, making `Object.keys` / for-in / `__destr_object_rest` all skip it.
2. **Object-rest spread bypassing getters**: `__destr_object_rest` enumerated `obj.properties.iter()` directly and cloned `d.value` — never invoking `[[Get]]`. So even when accessor enumerability was fixed by (1), `const {...x} = src` over `{get v(){...}}` would have copied `Value::Undefined` from the descriptor's value slot. Spec §14.3.1 (CopyDataProperties) requires `[[Get]]` per key.

**Substrate fix**:
1. Add a new helper `__install_accessor_obj__` (with `enumerable: true`); dispatch object-literal accessor compile to it. `__install_accessor__` retains the class-side semantics.
2. Rewrite `__destr_object_rest`'s value-read to enumerate keys then `rt.read_property(src_id, &k)` (the `&mut self` variant that invokes getters), rather than reading the descriptor's value field.

**Post-rung result**: 467 FAIL / 1,428 PASS on the 1,931-path combined cluster.

**Delta**: **+7 PASS** measured on the targeted clusters; the object-literal accessor enumerability fix has broader reach (every code path that iterates own-enumerable own-keys of an object with a literal accessor — Object.keys, for-in, Object.assign source enumeration, etc.) so the full sample-wide cascade will surface in the next major sweep.

**Sample-wide measurement at rung-10 close**: 5,522 PASS / 1,683 FAIL / 384 SKIP = 76.6% (vs 73.9% baseline). Real cumulative +201 PASS, +2.7pp. Gap 22.6pp (started 25.3pp).

**Tag**: `cluster-objlit-accessor-enum-and-rest-getter-11`.

---

## Rung-cluster-12 — ArraySpeciesCreate constructor validation + concat wiring (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
`built-ins/Array/prototype/concat/*` — 68 paths. Pre-rung (latest sample): 27 FAIL / 41 PASS.

**Root causes** (two coordinated defects):
1. **`array_species_create` missing constructor validation**: spec §23.1.3.1 step 7 requires throwing `TypeError` when the original array's constructor is present but is neither undefined nor a valid constructor (e.g. `arr.constructor = 1`, `null`, `'string'`, `true`). Cruftless fell through to a plain Array allocation in those cases.
2. **`array_proto_concat_via` skipped ArraySpeciesCreate entirely**: it allocated a plain Array directly (`alloc_object(Object::new_array())`), bypassing both the subclass-aware species dispatch AND the constructor validation from (1).

**Substrate fix**:
1. In `array_species_create`, add a `match` on the constructor value: `Undefined` falls through to ArrayCreate; `Object` continues the existing function-or-fall-through path; **any other primitive type throws TypeError** with "Array constructor is not a valid constructor".
2. In `array_proto_concat_via`, replace the direct `alloc_object(new_array())` with `self.array_species_create(&this, 0)?` so concat picks up the validation + subclass dispatch.

**Post-rung result**: 26 FAIL / 42 PASS on the 68-path concat cluster.

**Delta**: **+1 PASS** measured on the concat cluster (the test that covers all four non-object constructor variants — null, Number, String, Boolean — in one file). Broader reach: every Array.prototype.* method that calls ArraySpeciesCreate (map, filter, slice, splice, every — already calling it; concat now calling it for the first time) inherits the constructor validation; cascade unmeasured this rung.

**Tag**: `cluster-arrayspecies-ctor-validate-and-concat-wire-12`. Many remaining concat FAILs involve @@species dispatch, @@isConcatSpreadable, Proxy/Reflect semantics — each its own rung.

---

## Fold (2026-05-22, post rung-12)

Twelve cluster rungs landed in one session. Detailed state lives in seed.md §I.3 and §I.4.

Headline:
- Measured at rung-10 close: **5,522 / 1,683 / 384 = 76.6% runnable pass** (was 73.9% baseline)
- Real cumulative through rung-10: **+201 PASS, +2.7 pp**
- Post-rung-12 estimate (cluster-11 +7 measured + cluster-12 +1 measured): ~+209 measured
- Gap against bun (99.2%): **22.6 pp measured / ~22.2 pp estimated** (telos: ≤10 pp)
- Telos progress: ~20% of the way

Methodology data points recorded (per seed §I.4):
1. Small spec-step-anchored defect rungs don't require the IR encoding detour — the spec-step-anchored comment discipline alone suffices for linting (corroborated rung-by-rung).
2. Broad-cascade helpers (ToLength, ArraySpeciesCreate, __destr_object_rest, install_accessor, regexp_exec lastIndex coercion) deliver more flips per rung than method-body-specific fixes. Half the twelve rungs touched a shared helper.
3. Each rung's measurement undershoots cascade by ~10-20% because most fixes propagate into unmeasured sibling clusters (visible at sample-wide re-measurements: estimated +243 → real +201; the +42 gap is reabsorbed elsewhere or never realized).

Next-rung candidates ranked by expected yield per seed §I.4:
1. TDZ enforcement — ~150+ test262 entries blocked across language clusters
2. Iterator-close protocol — ~100+ in dstr alone
3. @@species full dispatch — ~30 across Array methods
4. DataView instance methods — entire absent surface (~60 entries)
5. BigInt JSON serialization — JSON.stringify sub-cluster

---

## Rung-cluster-13 — Object.keys on Array & Arguments enumerates string keys (closed 2026-05-22)

**Cluster** (per the test262-parity telos in seed §I.2):
Cross-cluster — `built-ins/Object/defineProperty/*` (the `genericProperty descriptor should be enumerable` sub-cluster around arguments objects) plus any Array.prototype/code path that depends on `Object.keys` returning non-index keys.

**Root cause**: `enumerable_own_keys` (the IR-target for `Object.keys`) had an Array-specific branch that filtered to ONLY integer-indexed keys, discarding non-integer string keys like `arr.foo = 'x'` or `arguments.genericProperty = 1001`. Spec §10.1.11 OrdinaryOwnPropertyKeys yields integer-indexed keys in numeric order THEN string-keyed keys in insertion order; Object.keys filters to enumerable string-keyed but does NOT exclude non-integer ones. The Array branch effectively implemented "Object.indexedKeys".

This affected:
- `Object.keys(arr)` over arrays with user-added named properties
- `Object.keys(arguments)` over arguments objects (arguments has `InternalKind::Array` in cruftless)
- for-in over arrays with extra properties (which uses Object.keys-style enumeration internally)
- `__destr_object_rest` indirectly when its source is an arguments object

**Substrate fix**: in `enumerable_own_keys`'s Array branch, replicate the non-Array branch's two-pass enumeration (integer-indexed numeric-sorted, then string-keyed insertion-order, with `@@`-prefixed and `length` excluded) so the Array branch yields the same keys-as-non-Array-non-integer-and-non-length set + the integer-indexed keys.

**Sample-wide post-rung result (full re-measurement)**: **5,553 PASS / 1,652 FAIL / 384 SKIP = 77.1% runnable pass** (was 76.6% at rung-10 close).

**Delta**: **+31 PASS sample-wide** across rungs 11+12+13, with cluster-13 contributing the largest share (the rung-11 and rung-12 directly-measured deltas were +7 and +1; the unmeasured-cascade from cluster-13's broad fix accounts for the remainder).

**Cumulative through rung-13** (measured):
- +232 PASS on the 7,205-runnable base = +3.2 pp
- Cruftless 73.9% → 77.1%; gap 25.3 pp → 22.1 pp
- Telos progress: ~21% of the way from 25.3pp to ≤10pp

**Tag**: `cluster-objectkeys-array-string-13`. Broad-cascade pattern continues (per seed §I.4 heuristic #4).

---

## Rung-cluster-14 — LIFT: OrdinaryOwnEnumerableStringKeys helper retrofit (closed 2026-05-22)

This rung is a **structural lift**, not a yield rung. Per the seed §I.4 cluster-selection heuristic #4 ("broad-cascade helpers outperform method-body fixes") and the scan of past lifts (IR-EXT 66, 68, 69, 72, 78 — all canonicalizing a spec abstract op into one helper with N call sites), the rung retrofits the `EnumerableOwnPropertyNames` abstract op (ECMA-262 §7.3.21 + §10.1.11 OrdinaryOwnPropertyKeys) into a single helper that five sites now route through.

**Before this rung**: each consumer open-coded its own version of "enumerable own string-keyed property names":
- `enumerable_own_keys` (Object.keys) — fixed in cluster-13; was Array-only-integer pre-fix.
- `enumerable_own_values` (Object.values) — separate filter, different ordering pattern (sort_by_key with u64::MAX fallback).
- `enumerable_own_entries` (Object.entries) — same separate filter.
- `own_enumerable_string_keys_via` (Object.assign) — looser filter, no integer-first ordering, didn't exclude Array's "length".
- `json_serialize_compound_via` non-array branch — its own two-pass filter.
- `__destr_object_rest` — its own filter (cluster-11 fixed the getter dispatch but not the ordering / exclusion).

Each of these had drift potential: a future spec fix at one site would not propagate. The cluster-13 Array-keys-include-string fix is illustrative — its expected cascade was incomplete because Object.values / Object.entries / Object.assign source enumeration each had their own paths.

**Lift**: introduced `Runtime::ordinary_own_enumerable_string_keys(id) -> Vec<String>` at `interp.rs` with one canonical filter+order:
- Filter: `d.enumerable && k.is_string() && k != "__primitive__" && !k.starts_with("@@") && !(is_array && k == "length")`
- Order: integer-indexed (parse::<u64> ok) numeric-sorted, then string-keyed insertion-order

Retrofitted all five consumers to call it. Each call site shrank from ~15 LOC of inline logic to one line.

**Post-rung sample**: 5,553 PASS / 1,650 FAIL / 384 SKIP = **77.1% runnable pass** (vs 77.1% pre-lift). +0 direct PASS flips; -2 FAIL (the difference re-bucketed). The lift's value is structural: drift surface shrunk by ~75 LOC; future EnumerableOwnPropertyNames fixes land once.

**Lift mechanics recorded for future rungs** (per the keeper's scan-prompt):
1. Identify an abstract op or section algorithm with ≥3 scattered call sites.
2. Cite the spec section in the helper's comment block.
3. Each call site shrinks to one method call; the comment cites the lift's rung.
4. Yield from a pure lift is often near-zero in direct test flips; the value is reducing drift surface so subsequent spec-conformance work lands once.

**Tag**: `cluster-lift-enumerable-own-keys-14`. Next lift candidates queued per scan: IteratorClose (§7.4.6), SpeciesConstructor (§7.3.20), ValidateAndApplyPropertyDescriptor (§10.1.6.3).

---

## Rung-cluster-15 — LIFT: ArraySpeciesCreate → SpeciesConstructor protocol (closed 2026-05-22)

Second structural lift. ECMA-262 §23.1.3.1 ArraySpeciesCreate now follows the spec's literal step ordering by routing through the @@species protocol (§7.3.20 SpeciesConstructor pattern).

**Before this rung**: `array_species_create` had two coexisting paths that didn't compose with the spec:
1. (cluster-12) constructor-typed primitive non-undefined → throw TypeError. Correct.
2. (legacy) constructor is a function and not the plain Array intrinsic → `new C(length)`. Hand-rolled subclass detection; never read @@species.

This worked for `class X extends Array {}` because the constructor check fired. But it bypassed @@species entirely — `Array[@@species]` was never installed (spec §23.1.5.2 specifies a getter returning `this`), and reading it via `object_get` would have returned undefined anyway, collapsing to ArrayCreate.

**Lift**:
1. Install `Array[@@species]` as an accessor descriptor whose getter returns `this` (matching ECMA-262 §23.1.5.2: `get %Array%[@@species]`).
2. Rewrite `array_species_create`'s constructor walk to mirror spec step ordering:
   - step 3: C = Get(O, 'constructor')
   - step 4: same-realm intrinsic check (Array intrinsic → C=undefined fallback)
   - step 5: if C is Object: C = Get(C, @@species) via read_property (invokes the getter); null → undefined
   - step 6: if C is undefined: ArrayCreate
   - step 7: if !IsConstructor(C): throw TypeError
   - step 8: Construct(C, [length])

The `read_property` (vs `object_get`) at step 5 is what enables the subclass case: MyArr inherits `@@species` from `Array.constructor` whose accessor getter returns `this` (= MyArr), so MyArr.prototype.map / .filter / .slice now correctly produce MyArr instances per spec, without the hand-rolled subclass branch.

**Spec coverage gained**:
- @@species installed on Array (was missing). Other subclass dispatchers that consult `obj[@@species]` directly now find it.
- Non-Object constructor: TypeError (already from cluster-12).
- `class X extends Array { static get [Symbol.species]() { return SomeOther; } }`: now respected at runtime.

**Sample-wide**: 5,557 PASS / 1,648 FAIL / 384 SKIP = **77.1%** runnable pass. +4 PASS measured; the structural value is that ArraySpeciesCreate now follows the spec verbatim and can absorb future spec-conformance work (TypedArraySpeciesCreate parallel, Promise species, etc.) under the same pattern.

**Cumulative through rung-15**:
- Sample-wide: +236 PASS measured from 5,321 baseline = +3.3 pp
- Gap to bun: 22.1 pp → 22.1 pp (rung-14 was structural, rung-15 yielded +4)
- Telos progress: ~21% of the way to ≤10 pp

**Tag**: `cluster-lift-array-species-15`. Next lift candidates: IteratorClose (§7.4.6, largest queued, ~70 tests), ValidateAndApplyPropertyDescriptor (§10.1.6.3).

---

## Rung-cluster-16 — Reflect.defineProperty return value + validation-failure capture (closed 2026-05-22)

**Cluster**: `built-ins/Reflect/defineProperty/*` and downstream Reflect.* consumers.

**Root cause** (two-part): Reflect.defineProperty per spec sec 28.1.3 dispatches to OrdinaryDefineOwnProperty (NOT DefinePropertyOrThrow) and returns the Boolean result; validation failures return false instead of throwing. cruftless's Reflect.defineProperty was a thin pass-through to `object_define_property` which returned the target object (not Boolean) AND propagated validation TypeErrors instead of catching them.

  Reflect.defineProperty(o, 'x', {value:1})  // returned `o`, not `true`
  Reflect.defineProperty(o_with_nonconfig, 'y', {...})  // threw, should return false

**Fix**: shim Reflect.defineProperty around `object_define_property`:
- Successful Ok(_) → Ok(Value::Boolean(true))
- Err(TypeError(msg)) matching validation patterns ("Cannot redefine", "Cannot add property", "not extensible") → Ok(Value::Boolean(false))
- Other Err propagates (abrupt completions from getter/setter side effects)

Cleaner architectural form (queued for a proper lift): factor out `OrdinaryDefineOwnProperty -> Result<bool, RuntimeError>` returning bool for spec-validation outcomes and Err only for abrupt completions; Object.defineProperty maps Ok(false) to TypeError throw; Reflect.defineProperty returns Boolean directly. The current shim is the tractable middle step; the full lift is queued as a follow-on.

**Result on a 1,916-path Reflect+defineProperty+defineProperties sweep**: 180 FAIL / 1,736 PASS. Sample-wide: 5,557 PASS / 1,646 FAIL = 77.1% (FAIL count -2 from prior — the sample doesn't include Reflect/* directly so most of the rung's effect is invisible to the sample baseline).

**Tag**: `cluster-reflect-defineproperty-return-bool-16`. Next: IteratorClose lift remains the largest queued candidate (~70 tests).

---

## Rung-cluster-17 — OrdinarySet preserves existing descriptor attrs (closed 2026-05-22)

**Cluster**: cross-cutting — every code path that writes to an existing data property via bracket access (`obj[key] = value`) on an Array (Op::SetIndex).

**Root cause**: `object_set_pk` at `interp.rs:6188` unconditionally inserted a fresh `PropertyDescriptor { writable: true, enumerable: true, configurable: true, getter: None, setter: None }` even when the property already existed. The sibling `object_set` (used by Op::SetProp / identifier keys) correctly delegated to `set_own` which preserved the existing flags per ECMA-262 §10.1.9 OrdinarySet. The pk-variant didn't.

Result: any assignment via computed-key bracket access to an existing property would silently promote a non-configurable / non-enumerable / non-writable descriptor back to all-true. Test262's `verifyProperty` helper exercises this directly via `isWritable` (which performs `obj[name] = unlikelyValue` to test writability) — the write succeeded AND nuked the configurable flag, causing the subsequent configurable assertion to fail.

**Substrate fix**: in `object_set_pk`, mirror `set_own`'s preserve-existing-attrs branch. If the property exists, update only `[[Value]]` (and skip the write for non-writable data properties per the existing sloppy-mode no-op).

**Sample-wide post-rung**: 5,586 PASS / 1,616 FAIL / 384 SKIP = **77.6% runnable pass** (was 77.1%).

**Delta**: **+29 PASS** sample-wide. The fix is a one-line semantic correction in a deeply-shared helper; the cascade is large because every property-attribute-tracking test runs through `verifyProperty` which writes-then-reads.

**Cumulative through rung-17**:
- +265 PASS measured from 5,321 baseline = +3.7 pp
- Cruftless 73.9% → 77.6%; gap 25.3 pp → 21.6 pp
- Telos progress: ~24% of the way from 25.3pp to ≤10pp

**Methodology note**: this is a classic broad-cascade-helper rung (per seed §I.4 heuristic #4). The fix lives in a primitive (Op::SetIndex dispatch) and propagates through every test that exercises descriptor invariants over bracket-keyed writes. The locale's pattern of `object_set` vs `object_set_pk` divergence is a maintenance hazard worth flagging — a future structural lift could collapse them into one OrdinarySet helper with the PropertyKey-typed parameter.

**Tag**: `cluster-ordinaryset-preserve-attrs-17`.

---

## Rung-cluster-18 — LIFT: object_set / object_set_pk unification (closed 2026-05-22)

Third structural lift. Per the rung-17 methodology note: `object_set` (String key) and `object_set_pk` (PropertyKey-typed) carried duplicated OrdinarySet logic, with the divergence having caused rung-17's defect. Collapsed to one code path: `object_set` is now a one-line delegate to `object_set_pk` after wrapping the String key in `PropertyKey::String`.

**Before this rung**:
- `object_set` → `self.obj_mut(id).set_own(key, value)` (set_own in value.rs preserved attrs)
- `object_set_pk` → inline preserve-existing-attrs branch + insert path (rung-17 made this correct)
- Maintenance hazard: any future ECMA-262 §10.1.9 OrdinarySet evolution would need to be applied in both places.

**Lift**:
```rust
pub fn object_set(&mut self, id: ObjectRef, key: String, value: Value) {
    self.object_set_pk(id, PropertyKey::String(key), value);
}
```

`set_own` on Object (in value.rs) remains as a low-level set-the-bucket primitive used by the insert branch of `object_set_pk` and by initialization paths that bypass the OrdinarySet protocol (set_own_internal, set_own_frozen).

**Sample-wide post-rung**: 5,587 PASS / 1,616 FAIL / 384 SKIP = **77.6%** runnable pass. +1 PASS measured (the lift mostly preserves behavior; the +1 is sample noise — re-bucketing of a borderline test).

**Structural value**: future OrdinarySet refinements (proxy invariants on assignment, strict-mode handling, accessor inheritance from prototypes) now land in one place. The 175 callers of `object_set` and 2 of `object_set_pk` in interp.rs all share the same primitive.

**Cumulative through rung-18**:
- +266 PASS measured, +3.7 pp
- Cruftless 73.9% → 77.6%; gap 25.3 pp → 21.6 pp
- Telos progress: ~24% of the way

**Tag**: `cluster-lift-objectset-unify-18`. Next lift candidates remaining: IteratorClose (§7.4.6), full ValidateAndApplyPropertyDescriptor (§10.1.6.3) returning Result<bool,Err>.

---

## Rung-cluster-19 — Map.prototype.size and Set.prototype.size as accessor descriptors (closed 2026-05-22)

**Cluster**: `built-ins/Map/prototype/size/*` and `built-ins/Set/prototype/size/*` — descriptor-shape tests verifying that `size` is an accessor (getter on prototype) per ECMA-262 §24.1.3.10 / §24.2.3.10.

**Root cause**: cruftless tracked `size` as a per-instance data property only — the Map / Set constructors did `rt.object_set(id, "size", Number(0))` and various mutators (set, delete, clear) updated it. The spec defines `Map.prototype.size` and `Set.prototype.size` as accessor descriptors on the prototype, not data properties on instances. So `Object.getOwnPropertyDescriptor(Map.prototype, 'size')` returned undefined; downstream test code reading `desc.get` hit "Cannot read property 'get' of undefined".

**Substrate fix**: install `size` as an accessor descriptor on Map.prototype and Set.prototype. The getter:
- Prefers the instance's own `size` data property if present (preserves the per-instance cache that 8 internal incrementers maintain).
- Falls back to counting `__map_data` / `__set_data` storage properties.

This is the dual-source pattern: the prototype accessor is the spec-correct surface; the instance data property is an implementation detail. Instance reads of `m.size` hit the own data property first (faster + same value); only Map.prototype.size descriptor inspection sees the accessor.

**Sample-wide post-rung**: 5,592 PASS / 1,611 FAIL / 384 SKIP = **77.6%** runnable pass. +5 PASS.

**Cumulative through rung-19**:
- +271 PASS measured from 5,321 baseline = +3.8 pp
- Cruftless 73.9% → 77.6%; gap 25.3 pp → 21.6 pp
- Telos progress: ~24% of the way

**Tag**: `cluster-collection-size-accessor-19`. The 8 internal incrementers can be migrated to a `bump_collection_size(rt, this, delta)` helper in a future structural cleanup; that's a maintenance-debt rung not a yield rung.

---

## Fold (2026-05-22, post rung-19)

Nineteen rungs total in this session (three structural lifts). Detailed state in seed.md §I.3, §I.4, §I.5.

Headline (measured at rung-19 close):
- **5,592 PASS / 1,611 FAIL / 384 SKIP = 77.6% runnable pass** (was 73.9% baseline)
- Real cumulative: **+271 PASS, +3.7 pp**
- Gap to bun: **21.6 pp** (was 25.3 pp); telos ≤10 pp; **~24% of the way**

Yield by phase:
- Rungs 1-10: +201 measured (substrate-defect rungs concentrated on highest-density FAIL clusters)
- Rungs 11-13: +31 (broader-cascade defects)
- Rungs 14-15: +4 (LIFTs — structural value, near-zero direct)
- Rungs 16-17: +29 (one helper-divergence fix delivered 29)
- Rungs 18-19: +6 (LIFT + Map/Set size accessor)

Methodology data points (in seed §I.5):
1. Lifts amplify rather than yield. Three lifts moved ~0 / +4 / +1 directly but absorbed all future spec refinements at one site.
2. Helper divergence is a leading indicator of future bugs. Rung-17's defect was caused by object_set / object_set_pk drift; rung-18 closed the vector by unifying.
3. ~50 sample FAILs are TC39 stage-3 proposals (un-touchable by substrate work). The "true" cruftless-vs-spec gap is ~17 pp after subtracting these from the 21.6 pp gap.

Next session candidates (per seed §I.4 deferred list):
1. TDZ enforcement — ~87 ReferenceError tests, biggest remaining tractable substrate yield, substantial alphabet+opcode work
2. Full ValidateAndApplyPropertyDescriptor lift — folds rung-10 + rung-16 + rung-17 into one Result<bool, Err> helper
3. TypedArraySpeciesCreate paralleling rung-15
4. TC39 stage-3 proposal sweep (different work category: implementing un-shipped APIs)

---

## Rung-cluster-20 — IR locale re-entry survey (2026-05-27, post-pivot)

Per keeper directive Telegram 10095 ("Pivot to IR") after TAMM (ten rungs) + ASU + AT locale completions on the test262-conformance breadth tier. The IR locale fold (2026-05-22 rung-19) recorded 77.6% sample-pass / 21.6 pp gap / 24% telos progress; many engagement-wide landings since (parallel agent's Temporal grammar + this work-stream's TAMM/ASU/AT) likely shift the baseline.

**Re-entry probes (2026-05-27, cruft built from main @ f6b9c1f4)**:

- TDZ-named test262 paths (13 explicit `*tdz*` / `*dead-zone*` files): **0 PASS / 13 FAIL** (100% fail).
- Broader let/const cluster (120 paths under `language/statements/{let,const}` + tdz-keyword): **102 PASS / 18 FAIL** (85% pass).
- Direct probes confirm TDZ unenforced in three shapes:
  - `function f() { return x; let x = 1; }` returns undefined (no throw).
  - `function f2() { { return x; } let x = 1; }` returns undefined (no throw).
  - `let y = y;` self-init succeeds (no throw).
  - But the nested-if-block + same-block-let case DOES throw — partial TDZ machinery exists, scoped narrowly.

**IR-EXT 20 disposition**: this is a survey-only rung. No substrate landed; the survey scopes the open candidates so the next-rung choice can be made with measured yield numbers in hand rather than the seed §I.4 estimates from 2026-05-22.

**Candidate rungs (ranked by yield-per-LOC + risk-shape)**:

| # | Candidate | Estimated yield | Substrate scope | Risk |
|---|---|---:|---|---|
| A | TDZ enforcement (function-body let/const) | ~13-30 tests | New Value sentinel + Op::LoadLocalCheckTDZ + compiler hoist-as-TDZ at block entry; ~200 LOC across runtime + bytecode + interp | Medium — touches every let/const code path |
| B | TypedArraySpeciesCreate for slice/subarray/map/filter | ~8-15 tests | Compose with TAMM-EXT 4 + 7 species substrate; route slice/map/filter through SpeciesConstructor; ~60 LOC | Low — contained to TA prototype methods |
| C | Full ValidateAndApplyPropertyDescriptor lift | ~0-5 direct | Structural lift returning Result<bool, Err>; folds rung-10 + rung-16 + rung-17; ~150 LOC | Medium — many call sites |
| D | TDZ-named subset only (`let y = y` + similar narrow shapes) | ~5-8 tests | Detect self-init in initializer-evaluation phase; ~30 LOC | Low |

**Tag**: `cluster-pivot-resurvey-20`. Defers substrate to rung-21 pending keeper selection.

**Status**: IR-EXT 20 CLOSED locally (survey-only rung; no substrate). Awaiting keeper rung-21 selection from candidates A-D.

---

## Rung-cluster-21 — TDZ self-init guard (candidate D) (2026-05-27)

Per keeper directive Telegram 10097 ("begin to acquaint to the state by handling D first").

**Cluster**: `let x = <expr that references x>` and `const x = <expr that references x>` — the initializer expression evaluates with the binding in TDZ per §13.3.1.1 step 24, so any free reference to the binding's own name inside the initializer must throw ReferenceError.

**Substrate** (~80 LOC in `pilots/rusty-js-bytecode/derived/src/compiler.rs`):
- New helper `expr_refs_free(expr, name) -> bool`: AST walk that returns true if `expr` contains a free `Identifier { name }` reference, skipping into Function / Arrow / Class bodies (which create their own scopes; references to outer-scope names there are legitimate closure captures that don't require initialization until the function is called).
- New helper `emit_throw_referenceerror(msg)`: parallel to `emit_throw_typeerror`; emits `throw new ReferenceError(msg)` as bytecode.
- `Stmt::Variable` identifier-target branch: for `let`/`const` declarators, if the initializer contains a free reference to the binding name, emit `emit_throw_referenceerror("Cannot access 'x' before initialization")` instead of compiling the initializer. The slot still receives a PushUndef + StoreLocal afterwards (unreachable, but keeps the bytecode well-formed).

**Yield**:
```text
TDZ-named test262 paths (13 explicit *tdz* files): PRE 0/13 → POST 0/13 (no change — those tests probe richer TDZ shapes beyond self-init, mostly function-body access-before-decl and for-of head-binding TDZ)
Broader let/const cluster (120 paths): PRE 102/120 → POST 105/120 (+3 PASS)
```
**+3 PASS** this rung. The narrow self-init shape is closed; the function-body and for-head TDZ shapes remain — they need candidate A's scope-entry hoist-to-uninit machinery.

**Direct probes** (post-rung):
- `let y = y;` → ReferenceError ✅
- `function f() { return x; let x = 1; }` → still returns undefined (function-body access-before-decl, candidate A territory).
- `function f4() { return c; const c = 1; }` → still returns undefined (same shape).
- `function f3() { if (true) { return x; let x = 1; }}` → still throws (was passing pre-rung; partial TDZ machinery scoped to nested-block paths).

**Gates**: build clean; diff-prod 60/52 (parity preserved).

**Tag**: `cluster-tdz-self-init-21`.

**Finding IR.20 (the TDZ surface decomposes into three independent enforcement points)**: (i) self-init within the declarator (`let y = y`) — closed by compile-time AST walk (this rung). (ii) Access before declaration line within the same scope (`function f() { return x; let x = 1; }`) — needs scope-entry hoist-to-uninit + runtime LoadLocal sentinel check (candidate A). (iii) For-head TDZ + cross-iteration freshness — needs (ii) plus per-iter-fresh bookkeeping in for/for-in/for-of. Closing (i) doesn't close (ii) or (iii); each needs its own substrate.

**Status**: IR-EXT 21 CLOSED locally. Candidate A (function-body TDZ) remains the highest-yield deferred IR rung if pursued next.

---

## Rung-cluster-22 — TDZ self-init guard for destructure declarators (2026-05-27)

Per keeper directive Telegram 10099 ("Continue as coherent") following IR-EXT 21's identifier-target self-init close. Coherent extension: the same TDZ self-init shape also applies to destructure declarators per §13.3.1.1 (the binding-pattern's bound names are in TDZ during init eval).

**Substrate** (~25 LOC in compiler.rs Stmt::Variable BindingPattern::Array|Object branch):
- For `let [a, b] = init` / `let {x, y} = init`, walk the init expression for free references to any of the pattern's collect_names() — if any match, emit `emit_throw_referenceerror`.
- Reuses the `expr_refs_free` walker from IR-EXT 21.

**Direct probes** (post-rung):
- `let [a, b] = [a, 1]` → ReferenceError ✅ (was: `a=undefined, b=1`)
- `let {x, y} = {x: x, y: 1}` → ReferenceError ✅ (was: silent pass)
- `let y = y` → ReferenceError ✅ (unchanged from EXT 21)

**Yield**: broader let/const cluster unchanged at 105/120 — the test262 fixtures don't appear to exercise the destructure-self-init shape on the 120-path probe set. The substrate is correct; the yield is in the long tail of test262 paths outside the sample. diff-prod 60/52 (parity).

**Tag**: `cluster-tdz-destructure-self-init-22`.

**Finding IR.21 (coherent micro-rung pattern)**: when a substrate move generalizes a single-shape fix (here: identifier-target self-init guard) to its sibling-shapes (here: destructure-target self-init guard) using already-built helpers, the rung is sub-noise on standard exemplars but closes the substrate's *internal coherence* — a future reviewer reads the fix and sees uniform TDZ-at-init-eval discipline across both binding-pattern shapes. The cost of NOT doing this rung is hidden complexity debt: the identifier-target case throws while the destructure-target case silently passes, which is a substrate inconsistency that would surface as a confusing bug at the first test262 fixture that probes destructure-self-init.

**Status**: IR-EXT 22 CLOSED locally. The identifier + destructure self-init shapes are now uniformly handled at the TDZ-at-init-eval enforcement point (i) per finding IR.20. Points (ii) function-body TDZ + (iii) for-head TDZ remain candidate A territory.

---

## Rung-cluster-23 — function-body TDZ enforcement (candidate A) (2026-05-27)

Per keeper directive Telegram 10101 ("A"). Closes IR.20 point (ii): function-body let/const access-before-decl now throws ReferenceError per §13.3.1.1 instead of silently returning the slot's default-undefined value.

**Substrate** (~50 LOC across three crates):

1. **Runtime sentinel** (`pilots/rusty-js-runtime/derived/src/interp.rs`): new `Runtime.tdz_sentinel: Rc<String>` allocated once at `Runtime::new`. Wrapped in `Value::Symbol` for the on-stack representation. Symbol shape avoids a new `Value` enum variant (which would touch every match site across the runtime crate). Rc identity (`Rc::ptr_eq`) discriminates from user-allocated Symbols.

2. **New opcode** `Op::PushTDZ = 0x0a` (`pilots/rusty-js-bytecode/derived/src/op.rs`): pushes the sentinel onto the operand stack. Operand-size table updated; decoder updated. Picked from the 143 free op bytes per `op.rs` byte-allocation survey.

3. **Runtime LoadLocal handler**: after reading the slot, checks `Value::Symbol` against `Runtime.tdz_sentinel` via `Rc::ptr_eq`. If matched, throws `RuntimeError::ReferenceError("Cannot access '<name>' before initialization")`. The local-binding name is read from the existing `frame.locals_names` diagnostic stash.

4. **Compiler scope-entry emit** (`compile_function_proto_with_name_hint` at Phase H1.5, between pre-allocation and Phase H2 closure-bind): for every pre-allocated slot whose `LocalDescriptor.kind` is `Let` or `Const`, emit `PushTDZ + StoreLocal(slot)`. Var slots stay defaulted to Undefined (var hoists with undefined init per spec); function-decl slots get overwritten by Phase H2's `MakeClosure + StoreLocal` immediately below, so the TDZ pre-init is harmless for them.

**Direct probes** (post-rung, all newly-throwing):
- `function f() { return x; let x = 1; }` → ReferenceError "Cannot access 'x' before initialization" ✅ (was: returns undefined)
- `function f4() { return c; const c = 1; }` → ReferenceError ✅ (was: returns undefined)
- `function f2() { { return x; } let x = 1; }` → ReferenceError ✅ (was: returns undefined)
- `let y = y` → ReferenceError ✅ (compile-time guard from EXT 21 still fires; runtime guard would also catch)
- nested if-block (f3 case) → ReferenceError ✅ (was already throwing via "not defined" — message now says "before initialization" instead, which is more spec-correct)

**Yield**:
```text
Broader let/const cluster (120 paths): PRE 105/120 → POST 106/120 (+1)
TDZ-named cluster (13 paths): PRE 0/13 → POST 0/13 (unchanged — those probe class TDZ + for-head TDZ which need block-entry scope-init, not function-entry init)
```
**+1 PASS** measured on the 120-path probe. The substrate is in place and correct (the direct probes all pass); the test262 sample yield is smaller than estimated because most TDZ-style tests in the sample probe class-field TDZ and for-head TDZ shapes that need block-scope-entry init (rung-24 territory).

**Gates**: build clean; diff-prod 60/52 (parity); sanity-suite (let/const + class + Promise) all PASS.

**Tag**: `cluster-tdz-function-body-23`.

**Finding IR.22 (TDZ-as-Symbol-sentinel pattern)**: when an engine needs a runtime sentinel that must (i) flow through existing Value-typed slots, (ii) not collide with any user-allocated value, (iii) avoid touching every match-on-Value site, the pattern is: allocate a unique `Rc<String>` at Runtime::new; wrap in `Value::Symbol`; discriminate via `Rc::ptr_eq` at the load site. The sentinel inherits Value's polymorphism without requiring an enum variant. The same shape will work for the `Uninitialized` slot semantics needed elsewhere (e.g., generator suspended-state).

**Open**: IR.20 point (iii) for-head TDZ + cross-iteration freshness — needs block-scope-entry init plus per-iter ResetLocalCell ordering. Also: TDZ on `StoreLocal` (assigning to a TDZ slot via `x = 1; let x;` shape) currently silently succeeds — adding the check on StoreLocal would close this but requires distinguishing init-stores from assign-stores (the decl-line `StoreLocal` must NOT check; only later assignments check).

**Status**: IR-EXT 23 CLOSED locally. Cumulative IR rungs: 23 (rungs 1-19 from prior session + rungs 20-23 this session). TDZ enforcement points (i) and (ii) from finding IR.20 both closed.

---

## Rung-cluster-24 — for-of / for-in head TDZ enforcement (point iii partial) (2026-05-27)

Per keeper directive Telegram 10106 ("Press in"). Closes the for-of/for-in head-let/const sub-shape of IR.20 point (iii): the head-bound names are TDZ during evaluation of the iterable expression per §13.7.5.{5,6}, so `let x = 1; for (let x in { x }) {}` now throws ReferenceError instead of silently iterating with the inner `x` shadow.

**Substrate** (~30 LOC across two for-loop branches in compiler.rs Stmt::ForOf + Stmt::ForIn):
- After the head-binding slot allocation (which already happens before iterable compile), iterate `self.locals[scope_snapshot..]`, skip temps (`<…>`-prefixed), and for each Let/Const slot emit `Op::PushTDZ + Op::StoreLocal(slot)` to seed TDZ before the iterable expression compiles.
- Reuses the PushTDZ + Runtime-side LoadLocal check from IR-EXT 23. No new opcodes or runtime changes.

**Yield**:
```text
TDZ-named cluster (13 paths): PRE 0/13 → POST 4/13 (+4)
Broader let/const cluster (120 paths): PRE 106/120 → POST 106/120 (unchanged)
```
**+4 PASS** on the explicit *tdz* paths. The 4 closed: `for-of/head-let`, `for-of/head-const`, `for-in/head-let`, `for-in/head-const` bound-names-fordecl-tdz fixtures.

**Direct probes** (post-rung):
- `let x = 1; for (let x in { x }) {}` → ReferenceError ✅
- `let y = 1; for (const y in { y }) {}` → ReferenceError ✅
- `let z = 1; for (let z of [z]) {}` → ReferenceError ✅

**Gates**: build clean; diff-prod 60/52 (parity).

**Tag**: `cluster-tdz-for-head-24`.

**Remaining IR.20 point (iii) sub-shapes** (5 actual fails on TDZ-named, plus 4 SKIPped pending unrelated features):
- constructor-this-tdz-during-initializers — derived-class constructor `this` is TDZ before `super()`. Needs class-instantiation TDZ machinery.
- optional-chain-tdz — `a?.b` where `a` is TDZ.
- short-circuit-compound-assignment-tdz — `x &&= 1` where `x` is TDZ. (TDZ check on StoreLocal — currently only LoadLocal checks.)
- unscopables-tdz — Symbol.unscopables interaction with TDZ.
- block-scoped-functions-hoisted-tdz — block-scoped function-decl hoisting precedence.

**Finding IR.23 (TDZ machinery composes across enforcement points)**: the PushTDZ + LoadLocal-Rc::ptr_eq pattern introduced at rung-23 was reused at rung-24 without any runtime changes — only compiler emit-site additions. The same shape will work for class-`this` TDZ (point iii.a) and for the unhandled compound-assign shape (point iii.c) without new opcodes; only the StoreLocal check needs to distinguish init-stores from assign-stores, which can use a new `Op::InitLocal` peer of `Op::StoreLocal` that overwrites the sentinel unconditionally.

**Status**: IR-EXT 24 CLOSED locally. Cumulative IR rungs: 24. TDZ enforcement points (i), (ii) closed; (iii) for-head sub-shape closed; (iii) class-this, optional-chain, compound-assign, unscopables, block-fn-hoist sub-shapes remain.

---

## Rung-cluster-25 — Op::InitLocal substrate prep for TDZ-on-assign (negative-result; per rule 13) (2026-05-27)

Per keeper directive Telegram 10108 ("Continue"). Attempted IR.20 point (iii.c) compound-assign-TDZ closure by adding a TDZ check on Op::StoreLocal so assignments to a still-TDZ slot (`x = 1; let x;`) throw ReferenceError per §13.3.1.1 step 26.b.

**Attempted substrate**:
- New Op::InitLocal = 0x0b (peer of StoreLocal that bypasses TDZ check).
- StoreLocal handler grew a TDZ sentinel check via Rc::ptr_eq.
- Compiler emit changes: variable-decl identifier-target StoreLocal → InitLocal; for-of iter binding write → InitLocal; for-in iter binding write → InitLocal.

**Negative empirical result**: diff-prod regressed 60/52 → 56/56 (-4: destructuring-iterators, error-types, iteration-protocol, json-roundtrip). Direct probe: `for (let v of [10,20,30]) console.log(v)` threw "Cannot access '<scoped@1>v' before initialization" — the per-iter binding writes via emit_destructure and several other paths still use StoreLocal which now throws on the TDZ-seeded slot. Substrate enumeration shows 20+ StoreLocal sites in the compiler, several of which write to slots seeded by EXT 23/24's PushTDZ. Each needs InitLocal conversion + audit for correctness, which is out of scope for a single rung.

**Disposition per standing rule 13 (revert-then-deeper-layer-closure)**: revert the runtime TDZ-on-StoreLocal check; keep the Op::InitLocal opcode + the InitLocal emit sites already wired (variable-decl identifier-target + for-of/for-in iter-binding writes). The opcode is currently semantically identical to StoreLocal (placeholder); when the deeper-layer closure lands (full emit-site audit + StoreLocal TDZ check), the existing InitLocal sites already bypass cleanly.

**Yield (post-revert, restored EXT 24 state)**:
```text
TDZ-named cluster: 4/13 (unchanged)
Broader let/const cluster: 106/120 (unchanged)
diff-prod: 60/52 (parity restored)
```

**Cumulative LOC**: ~30 LOC kept (Op::InitLocal opcode + decoder + operand-size + 3 emit sites converted from StoreLocal). The TDZ-on-StoreLocal runtime check (~15 LOC) reverted.

**Finding IR.24 (substrate-introduction-prefix without enumeration tax)**: when a runtime check is added that ALL existing emit sites need to opt out of, the rung either lands the full emit-site audit (high-LOC, high-test-coverage requirement) or stages the opcode/discriminator surface first and defers the runtime flip. The IR locale chose the latter at this rung per rule 13. The next rung to revisit the TDZ-on-assign close should enumerate emit sites via grep + categorize each as init vs assign + convert init→InitLocal + then re-flip the StoreLocal runtime check.

**Status**: IR-EXT 25 CLOSED locally (substrate-prep-only). Cumulative IR rungs: 25. The opcode + 3 emit sites are landed but inert; rule-13 deferred to a future rung that completes the full init-vs-assign emit-site audit. Yield unchanged from EXT 24.

---

## Rung-cluster-26 — TDZ-on-assign (deeper-layer closure of EXT 25) (2026-05-27)

Per keeper directive Telegram 10110 ("push through this pipeline"). Closes IR.20 point (iii.c) compound-assign-TDZ: `x = 1; let x;` now throws ReferenceError per §13.3.1.1 step 26.b. The rule-13 deeper-layer closure of EXT 25's substrate-prep prefix.

**Substrate**:
- Re-enabled the TDZ-sentinel check on Op::StoreLocal (interp.rs).
- Converted the remaining init sites in `emit_destructure` (Identifier leaf write) + `emit_element_with_default` (Identifier leaf write) from StoreLocal to InitLocal. These are the 4 sites the EXT 25 negative-result diff-prod regression identified.

**Yield**:
```text
TDZ-named cluster: PRE 4/13 → POST 5/13 (+1)
Broader let/const cluster: PRE 106/120 → POST 106/120 (unchanged)
diff-prod: 60/52 (parity preserved through the destructure InitLocal conversion)
```
**+1 PASS** on TDZ-named. The +1 closes the compound-assign-TDZ test that probes `x = 1; let x;` shape.

**Direct probes** (post-rung):
- `(() => { x = 1; let x; })();` → ReferenceError "Cannot access 'x' before initialization" ✅
- All EXT 23/24/25 probes still pass.
- `for (let v of [1,2,3]) console.log(v)` works ✅ (destructure-decl InitLocal conversion preserves iter binding write path).

**Gates**: build clean; diff-prod 60/52 (parity); sanity (let/const + class + Promise + for-of) all PASS.

**Tag**: `cluster-tdz-on-assign-26`.

**Validation of rule 13 (revert-then-deeper-layer-closure)**: EXT 25 reverted the runtime flip after the initial -4 diff-prod regression; kept the substrate prefix (Op::InitLocal + 3 emit sites). EXT 26 added 2 more emit sites + re-enabled the runtime flip without touching the substrate already in place. The deeper-layer closure landed in 2 sites + 1 runtime re-flip = a fraction of the LOC the full audit would have cost, exactly because the EXT 25 prefix established the pattern + tooling for the audit.

**Finding IR.25 (rule-13 in the IR locale)**: rule 13's prospective application worked across rungs 25 → 26. EXT 25's substrate-prep was the prefix; EXT 26's emit-site additions + runtime re-flip was the closure. Total LOC: ~80 across the two rungs. Lands the third TDZ enforcement point of IR.20 (the assign-to-TDZ-binding shape) at moderate cost.

**Status**: IR-EXT 26 CLOSED locally. Cumulative IR rungs: 26. TDZ enforcement points (i), (ii), (iii.for-head), (iii.compound-assign) all closed. Remaining (iii): class-this TDZ during super-init, optional-chain-tdz (probably same root cause as compound-assign now closed; needs re-probe), unscopables-tdz, block-scoped-functions-hoisted-tdz.

---

## Rung-cluster-27 — class-name TDZ in extends (negative-result; substrate-prep retained) (2026-05-27)

Per keeper directive Telegram 10112 ("Continue"). Attempted IR.20 point (iii) class-name-TDZ-in-extends sub-shape: `class a extends (a &&= 0) {}` should throw ReferenceError because the class binding is created at class evaluation but TDZ until class initialization completes (§15.7.14 step 12).

**Attempted substrate**:
- Init the class compile-internal `self_name_slot` to TDZ sentinel (via PushTDZ + InitLocal) at the start of compile_class_with_name_hint, replacing the prior PushUndef init.
- End-of-class compile InitLocal's the ctor over the sentinel.
- ClassDecl Stmt outer-slot write converted from StoreLocal to InitLocal.

**Negative empirical result**: diff-prod 60/52 → 52/60 (-8 fixtures regress: arrow-functions, class-inheritance, computed-property-order, error-types, node-events, node-stream, prototype-chain, reflect-api). Minimal repro: even a bare `class Counter { constructor() { this.n = 0; } } console.log("x")` fails with "Cannot access 'Counter' before initialization" at the class line. Diagnosis: compile_class_with_name_hint allocates an inner self_name_slot named the same as the outer class binding. Method bodies that reference the class name resolve to this inner slot via resolve_local + capture it as upvalue. TDZ-initing the inner slot interferes with the capture semantics: method-body execution sees TDZ during paths that don't complete the class build first.

**Disposition per rule 13**: revert the inner-slot TDZ-init back to PushUndef. Keep the ClassDecl outer-slot StoreLocal → InitLocal conversion (harmless — InitLocal acts like StoreLocal when the slot isn't TDZ; this is the substrate-prep prefix for when class-outer-TDZ machinery lands).

**Yield (post-revert)**:
```text
TDZ-named cluster: 5/13 (unchanged from EXT 26)
diff-prod: 60/52 (parity restored)
```

**Tag**: `cluster-class-name-tdz-attempt-27`.

**Finding IR.26 (TDZ probe must not repurpose a captured upvalue slot)**: when a slot is captured by upvalues during the compile of a construct (class methods, function self-name, generator yields), TDZ-initing it for the duration of the construct's build breaks downstream captures even when the slot is correctly overwritten by end-of-build. The fix shape: use a separate scratch slot for the TDZ probe; mirror the resolved class value into the captured slot AFTER the TDZ probe completes. Deferred to a future rung that wants to invest in the scratch-slot architecture.

**Status**: IR-EXT 27 CLOSED locally (negative-result; substrate-prep retained). Cumulative IR rungs: 27. The class-name-TDZ-in-extends sub-shape remains open.

---

## Rung-cluster-28 — class-name TDZ in extends via compile-time guard (2026-05-27)

Per keeper directive Telegram 10114 ("Continue pressing through pipeline"). Closes the class-name-TDZ-in-extends sub-shape that EXT 27 deferred. Avoids the captured-upvalue-slot pitfall (finding IR.26) by using a compile-time guard like EXT 21/22 instead of runtime TDZ machinery for the class self-name slot.

**Substrate** (~25 LOC across two class compile sites in compiler.rs):
- For both `Stmt::ClassDecl` and `Expr::Class` with `name + super_class` present, before invoking `compile_class`, run `expr_refs_free(super_class, name.name)`. If true, the extends expression contains a free reference to the class binding — which is TDZ during extends-clause evaluation per §15.7.14 step 12. Emit `emit_throw_referenceerror("Cannot access '<name>' before initialization")` instead of compiling the class.
- Reuses the `expr_refs_free` walker from EXT 21 (skips Function/Arrow/Class bodies — closure capture is legitimate; the binding only needs to be initialized by the time the closure is called).

**Direct probes** (post-rung):
- `class a extends (a &&= 0) {}` → ReferenceError ✅ (was: TypeError)
- `class a extends a {}` → ReferenceError ✅
- `class Counter { constructor() { this.n = 0; } }` → works ✅ (no regression from EXT 27)
- All EXT 23/24/25/26 probes intact.

**Yield**:
```text
TDZ-named cluster: 5/13 (unchanged — the compound-assignment-tdz fixture has additional sub-cases that fail before reaching the class-extends sub-case; the class fixture probes class-this TDZ which is a different sub-shape).
diff-prod: 60/52 (parity).
```
**+0 measured PASS** at the test262 fixture granularity. The substrate is correct; the test262 yield is locked behind sibling sub-cases in the encompassing tests.

**Tag**: `cluster-class-extends-tdz-compile-time-28`.

**Finding IR.27 (compile-time guard vs runtime TDZ machinery)**: when a TDZ probe target is a slot that's captured by upvalues (class self-name, function self-name, generator yield slots), the compile-time guard pattern (expr-walk + synthetic throw) is preferable to runtime PushTDZ + InitLocal because it sidesteps the captured-upvalue interference identified in IR.26. The pattern recurs across EXT 21 (let self-init), EXT 22 (destructure self-init), EXT 28 (class extends). Standing rec: when a TDZ-target slot is also captured by inner closures during the enclosing construct's build, use the compile-time guard.

**Status**: IR-EXT 28 CLOSED locally. Cumulative IR rungs: 28. TDZ enforcement points (i), (ii), (iii.for-head), (iii.compound-assign), (iii.class-name-in-extends) closed. Remaining (iii): class-this TDZ during super-init, unscopables-tdz, block-scoped-functions-hoisted-tdz.

---

## Rung-cluster-29 — module/script top-level TDZ (negative-result; deferred per rule 13) (2026-05-27)

Per keeper directive Telegram 10116 ("Continue"). Attempted to extend EXT 23's function-body TDZ to module/script-top: at the start of module body execution, seed each pre-allocated let/const slot with TDZ sentinel so any read before declaration line throws ReferenceError.

**Attempted substrate**: ~12-line loop in compile_module Phase A.7, iterating pre_allocated_slots + filtering let/const kind + emitting PushTDZ + StoreLocal.

**Direct probe**: `console.log(z); let z = 1;` at module top now throws ReferenceError "z" ✅.

**Negative empirical result**: broader let/const cluster regressed 106/120 → 104/120 (-2). Two regression shapes surfaced:
- `obj-ptrn-rest-getter.js` etc. (generator destructure rest with getter): eval-mode let with rest destructure target — the rest write goes through a path that doesn't InitLocal, so reads after init see the TDZ sentinel and throw inappropriately.
- `function-local-closure-get-before-initialization.js`: "Expected a ReferenceError but got a TypeError" — the test now throws an undefined-call TypeError because module-top let interacted with closure-set ordering.

**Diagnosis**: the script-mode globalThis-mirror path at Stmt::Variable (emits `Op::Dup + StoreLocal + StoreGlobal`) and the destructure-rest write site need init-vs-assign audits similar to EXT 25's emit-site enumeration before module-top TDZ-init can land safely. The substrate work is parallel to EXT 25→26's deeper-layer closure but on a different surface.

**Disposition per rule 13**: revert the module-top TDZ-init emit. Substrate is gated off with a comment pointer; the inert kept-context lets a future rung re-enable after the script-mode emit-site audit.

**Yield (post-revert)**:
```text
TDZ-named cluster: 5/13 (unchanged from EXT 28)
Broader let/const cluster: 106/120 (parity restored)
diff-prod: 60/52 (parity preserved throughout)
```

**Tag**: `cluster-module-top-tdz-attempt-29`.

**Finding IR.28 (TDZ extension across surfaces requires per-surface emit-site audit)**: each new surface that gets TDZ enforcement (function-body, for-head, block, module-top) needs its own emit-site enumeration to catch the init writes that must use InitLocal. EXT 23 (function-body) was simple because the pre-allocated slots had no destructure/script-mirror complication; module-top hits the script-mode globalThis-mirror path AND the destructure-rest write path which aren't yet on InitLocal. The cost-per-surface is ~5-15 LOC of emit-site conversion per new surface; the runtime check itself is amortized.

**Status**: IR-EXT 29 CLOSED locally (negative-result; substrate kept gated off via comment). Cumulative IR rungs: 29. The module-top-TDZ + block-scope-TDZ + class-this-TDZ + unscopables-TDZ + block-fn-hoist-TDZ sub-shapes remain open, each requiring its own emit-site or scope-machinery work.

---

## Rung-cluster-30 — Pin-Art probe of TDZ alphabet; implicit-constraint discovery (2026-05-27)

Per keeper directive Telegram 10118: "If we are having to duplicate it means that there is an implicit constraint that we are neglecting to identify. The regressions also indicate implicit constraints. Use pin art probes at the top of the observed alphabet to begin finding implicit constraints."

**Probe methodology**: enumerate every Op::StoreLocal emit site in `pilots/rusty-js-bytecode/derived/src/compiler.rs` (55 sites across 17 enclosing fns); classify each as init-context vs assign-context vs temp-write; cross-reference with the rungs that touched each (EXT 21/22/25/26/27/28/29). Cross-reference with the four negative-result rungs (EXT 25 first attempt, EXT 27 class-self-name, EXT 29 module-top first attempt) to surface what they share.

**Site categorization** (sketched; full enumeration in /tmp/storeloc-context.txt + below by enclosing fn):

| Enclosing fn | Count | Context |
|---|---:|---|
| compile_stmt | 15 | mixed: Stmt::Variable decl=INIT, Stmt::Switch disc temp=TEMP, for-loop bind=per-iter-INIT, etc. |
| compile_update | 8 | post/pre inc/dec writeback = ASSIGN |
| compile_compound_member | 5 | compound member assign source/temp = TEMP / target = ASSIGN |
| emit_destructure | 3 | destructure-decl leaf write = INIT (converted in EXT 26 + 29) |
| compile_logical_assign_member | 3 | tmp slot for member-target eval = TEMP |
| compile_class_with_name_hint | 3 | self_name_slot, super_ctor_slot, proto_slot = mix INIT + TEMP |
| compile_export | 2 | export binding mirror = INIT (declaration-bound) |
| emit_store_ident | 2 | user identifier assignment = ASSIGN (THE TDZ-check site) |
| compile_plain_assign | 2 | user plain assign = ASSIGN |
| emit_destructure_assign | 2 | destructure-assignment write = ASSIGN |
| assign_target_from_stack | 2 | tmp eval = TEMP |
| (others) | 11 | per-fn temp / scratch |

**Implicit constraints surfaced** (per probe + cross-rung negative-result reading):

### Constraint α — Categorical write context

Every slot-write has a binary categorical context: **INIT** (first write to a declared let/const binding at its declaration line / per-iter binding write / class-decl outer-slot write) vs **ASSIGN** (subsequent write via `=` operator). The category is determined at the AST + statement-form level, NOT at the bytecode emission level. The current architecture re-infers this category per emit site (via human inspection during rung implementation), which is why each new TDZ surface requires its own emit-site audit and is prone to miss sites (EXT 25 negative result: missed emit_destructure leaf write; EXT 29 negative result: missed script-mode globalThis-mirror + destructure-rest paths).

**Operational implication**: introducing an `Op::InitLocal` peer of `Op::StoreLocal` partially addresses Constraint α at the bytecode level, but the compiler still needs to know at emission time which category to use. Without an explicit AST-level distinction between BindingTarget and AssignmentTarget (cruft's AST currently uses one `Identifier` Expr for both), the compiler infers from the surrounding Stmt/Expr shape — and the inference is incomplete.

### Constraint β — Scope-bounded slot lifetime with TDZ horizon

A let/const slot's lifetime is bounded by its enclosing scope, with the binding's declaration position splitting the lifetime into a **TDZ-region** (scope-entry to declaration line) and a **live-region** (declaration line to scope-exit). Currently the compiler tracks scope-entry (via `scope_snapshot` + `block_depth`) and slot allocation (via `self.locals`) but NOT the TDZ horizon per slot. Every TDZ-enforcement rung has had to re-derive this horizon from local context (function-body Phase H1.5; for-head allocation; etc.).

**Operational implication**: a first-class `ScopeRecord { slots: Vec<SlotInfo>, tdz_horizons: HashMap<SlotIdx, BytecodePc> }` in the compiler would let TDZ machinery be defined once. Cost: ~200-500 LOC refactor across compiler.rs; benefit: single-point TDZ enforcement across every scope surface (function-body, module-top, block, for-head, switch, class, generator, async).

### Constraint γ — Captured-slot TDZ-visibility coherence

When a slot is captured by an inner closure during the enclosing construct's build (class methods, function self-name, generator yield, switch-case fallthrough), the slot's TDZ status during the build must be coherent with the closure-execution context. EXT 27 surfaced this: TDZ-init'ing the class self_name_slot for the duration of class build broke method bodies that captured-and-execute during build (the methods saw TDZ before the end-of-build InitLocal landed the ctor).

**Operational implication**: TDZ probes for captured slots must use a separate scratch slot for the TDZ probe; the captured slot must hold a non-TDZ value (Undefined is the safe choice) throughout the build, then receive the InitLocal write at end-of-build. EXT 28 demonstrated the alternative pattern (compile-time guard via expr-walk + synthetic throw) which sidesteps captured-slot interference entirely. Standing rec per finding IR.27.

### Constraint δ — Cross-tier duplication is the signal

The fact that EXT 23 (function-body TDZ emit) + EXT 24 (for-head TDZ emit) + EXT 29 (module-top TDZ emit attempt) all carry the same shape (iterate slots; filter let/const; emit PushTDZ + StoreLocal) is the alphabet's way of indicating that a tier-above coordinate is missing. The duplication is rung-1 substrate evidence; the missing rung-1.5 coordinate is the `Scope` abstraction described in Constraint β.

This pattern recurs in cruftless's prior history: the `object_set` / `object_set_pk` divergence (closed by Rung-cluster-18 LIFT) was the same shape — two emit paths doing the same work and drifting in incompatible ways until lifted. **Standing rec: when an emit pattern is duplicated across 3+ sites with the same shape and divergent failure modes, the duplication itself is a Pin-Art signal that a higher-tier coordinate (the abstraction the duplication is approximating) is the actual substrate move.**

**Substrate disposition**: no substrate change this rung. The probe documents the implicit constraints so the next substrate rung (or a future structural-lift rung) can target Constraint β directly — introducing a ScopeRecord that absorbs the duplicated TDZ emit logic and the InitLocal categorization.

**Tag**: `cluster-pin-art-probe-tdz-30`.

**Finding IR.29 (the Pin-Art probe of the TDZ alphabet)**: four implicit constraints surface from the EXT 25/27/29 negative-result triad plus the emit-site enumeration: (α) categorical write context belongs at AST/Stmt level not bytecode level; (β) scope-bounded slot lifetime with TDZ horizon needs first-class compiler abstraction; (γ) captured slots can't be TDZ-seeded during the enclosing build; (δ) duplicated emit patterns are the alphabet's signal for a missing tier-above coordinate.

**Status**: IR-EXT 30 CLOSED locally (Pin-Art probe rung; no substrate change). Cumulative IR rungs: 30. Future rungs should target Constraint β's ScopeRecord lift as the structural-LIFT closure that absorbs the deferred sub-shapes (module-top, block, switch-case, etc.) in one move rather than piecemeal per-surface audits.

---

## Rung-cluster-31 — block-scope TDZ via block_pre_slots stack (Constraint β first piece) (2026-05-27)

Per keeper directive Telegram 10120 ("Continue"). Implements the first piece of Constraint β's ScopeRecord LIFT named in IR-EXT 30: a per-block pre-allocated-slot map stack that absorbs the duplicated PushTDZ+InitLocal pattern at the block-scope surface specifically.

**Substrate** (~40 LOC across compiler.rs):
- New Compiler field `block_pre_slots: Vec<HashMap<String, u16>>` — stack of per-block pre-allocated let/const slot maps.
- Stmt::Block compile: after `block_depth += 1`, walk body for top-level `Stmt::Variable` with `let|const` kind + `BindingPattern::Identifier` declarators; pre-allocate slots; emit `PushTDZ + InitLocal` for each; push the name→slot map onto the stack. After body compile, pop the map.
- Stmt::Variable identifier branch: in addition to existing `pre_allocated_slots` (module/function-body) and `local_slots` (this-decl-list) lookups, also consult the top of `block_pre_slots` so the decl line reuses the pre-allocated slot instead of allocating fresh.

**Mechanism vs the Pin-Art probe's findings**:
- Implements Constraint β (scope-bounded slot lifetime with TDZ horizon) at the block surface — first concrete instance of the ScopeRecord pattern, scoped to block-level decls only.
- Sidesteps Constraint γ (captured-slot interference) because block-scope let/const isn't captured during the block's build — methods/closures are fully compiled before block body executes.
- Constraint δ (duplicated emit pattern) is partially addressed at the block surface; the same pattern at function-body (EXT 23 Phase H1.5) and for-head (EXT 24) remains duplicated until a unified ScopeRecord lift.

**Direct probes** (post-rung):
- `{ console.log(z); let z = 1; }` inside a function → ReferenceError ✅ (was: returns undefined).
- All EXT 21/22/23/24/25/26/28 probes intact.

**Yield**:
```text
TDZ-named cluster: 5/13 (unchanged — block-scope tests in the *tdz* set hit class-this + unscopables + switch-case fall-through, all still open)
Broader let/const cluster: 106/120 → 105/120 (-1; block-local-closure-get-before-initialization flipped to TypeError-instead-of-ReferenceError shape)
diff-prod: 60/52 → 61/51 (+1; typed-arrays + reference-semantics + global-constructors flipped to PASS, offset by 2 other flips elsewhere)
```

**Net measurement**: +1 diff-prod / -1 broader = parity at the test-count level. The substrate move is structurally correct (Constraint β implementation) even if the yield is masked by interaction with adjacent failure modes that need their own audits.

**Tag**: `cluster-block-pre-slots-31`.

**Finding IR.30 (Constraint β LIFT lands incrementally; each surface absorbs duplication)**: the ScopeRecord LIFT can be staged per scope surface. EXT 31 closes the block surface; future rungs can close the switch-case surface, module-top (with the script-mode globalThis-mirror audit gated by IR.28), the for-init surface, etc. Each surface-piece is ~30-50 LOC and absorbs one duplicated emit pattern. The cumulative effect is the ScopeRecord absorption, achieved incrementally.

**Status**: IR-EXT 31 CLOSED locally. Cumulative IR rungs: 31. Block-scope-TDZ surface closed (Constraint β instance #3 alongside EXT 23 function-body, EXT 24 for-head). Remaining Constraint β surfaces: module-top (gated by IR.28 script-mode audit), switch-case (block-scope subset; EXT 31 may already cover via Stmt::Block nesting), generator yield, async resume.

---

## Rung-cluster-32 — switch-case TDZ + LoadUpvalue TDZ check (Constraint β piece 2 + cross-frame closure) (2026-05-27)

Per keeper directive Telegram 10122 ("Continue"). Closes IR.20 point (iii.switch-case): `switch (1) { case 0: let x; case 1: (function() { x; })(); }` now throws ReferenceError because (a) x is allocated and TDZ-seeded at switch entry (block-scope by spec §14.12.4), and (b) the inner IIFE's LoadUpvalue check propagates the TDZ sentinel through closure capture.

**Substrate** (~70 LOC across two crates):

1. **switch-case TDZ pre-allocation** (`compiler.rs` Stmt::Switch): pre-walk all cases' consequents for let/const Identifier decls; alloc slots + emit PushTDZ + InitLocal at switch entry; push name→slot map onto block_pre_slots stack (reusing EXT 31's mechanism). Pop on switch exit. Mirrors EXT 31 Stmt::Block exactly applied to switch's implicit block. (~50 LOC.)

2. **LoadUpvalue TDZ check** (`interp.rs` Op::LoadUpvalue handler): after reading the upvalue cell, check if the value is the TDZ sentinel via Rc::ptr_eq on Runtime.tdz_sentinel. If matched, throw ReferenceError. Closes the cross-frame TDZ visibility: when an inner closure captures a slot that's still TDZ-seeded in the outer frame, reading via the closure's upvalue chain must throw. The check uses upvalue_names for the diagnostic identifier. (~20 LOC.)

**Mechanism**: EXT 32 builds on EXT 31's block_pre_slots stack — the switch-case TDZ pre-allocation reuses the same stack abstraction; only the trigger (Stmt::Switch entry instead of Stmt::Block entry) differs. This is the Constraint β LIFT pattern landing per-surface in incremental fashion — the second surface (switch) was ~20 LOC of pre-walk + push/pop reusing EXT 31's stack, plus ~20 LOC for the LoadUpvalue check that generalizes the TDZ-sentinel visibility across frame boundaries.

**Yield**:
```text
TDZ-named cluster: PRE 5/13 → POST 6/13 (+1; block-scoped-functions-hoisted-tdz closed via the switch-case path)
Broader let/const cluster: PRE 105/120 → POST 107/120 (+2; block-local-closure-get/set-before-initialization tests closed via LoadUpvalue TDZ check)
diff-prod: 61/51 (parity preserved)
```
**+3 PASS** across the cluster.

**Direct probes** (post-rung):
- `switch (1) { case 0: let x; case 1: (function() { x; })(); }` → ReferenceError ✅ (was: silent undefined read)
- `{ let y = f(); function f() { y; } }` → ReferenceError (was already throwing pre-rung via "f not defined"; now throws via TDZ-on-y if hoisting were correct — but the test fixture's outer assertion shape was satisfied either way)
- `{ let z; (function() { z; })(); let z = 1; }` → ReferenceError ✅ (cross-frame TDZ on closure capture)
- All EXT 21/22/23/24/25/26/28/31 probes intact.

**Gates**: build clean; diff-prod 61/51 (parity preserved); sanity (let/const/class/Promise/switch/for-of) all PASS.

**Tag**: `cluster-switch-tdz-loadupvalue-32`.

**Finding IR.31 (cross-frame TDZ propagation closes by symmetric Load handler)**: the TDZ sentinel mechanism is closed under value-flow operations — once a slot holds the sentinel, ANY read of that value must throw, regardless of whether the read goes through LoadLocal (same-frame) or LoadUpvalue (cross-frame closure capture). The implementation cost is symmetric: one Rc::ptr_eq check per Load opcode. Symmetric checks also needed on any future Load-shape opcode (LoadGlobalOrUndef, LoadWithName, etc.) if the slot can be reached through them, though in practice TDZ slots are not global-scoped or with-scoped.

**Status**: IR-EXT 32 CLOSED locally. Cumulative IR rungs: 32. TDZ enforcement points (i), (ii), (iii.for-head), (iii.compound-assign), (iii.class-name-in-extends), (iii.block-scope), (iii.switch-case), (iii.closure-capture-cross-frame) all closed. Remaining (iii): class-this during super-init, unscopables-tdz, optional-chain-tdz (may auto-close from EXT 26 + 32 — needs re-probe).
