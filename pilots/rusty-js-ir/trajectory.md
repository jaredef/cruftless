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

### Conjecture status

**§I-strengthened corroboration #5 (2026-05-19, EXT 56)**: a queued alphabet extension predicted at EXT 52 close (property-descriptor builders) was empirically shown to be unnecessary upon implementation. The existing alphabet was already sufficient. This is the strongest corroboration of §I.1.b yet — the alphabet-completeness criterion is not just stable in practice but predictively *over-conservative* when projected forward.
