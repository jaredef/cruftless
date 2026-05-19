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
