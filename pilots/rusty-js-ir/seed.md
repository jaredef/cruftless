# rusty-js-ir — Resume Vector / Seed

**Workstream**: spec-as-source-of-truth IR between ECMA-262 and rusty-js-runtime.
**Author**: 2026-05-19 session (post-cruftless P62.E25).
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Composes with**: `IR-DESIGN.md` (architecture); cruftless seed §A8.27–§A8.33; Doc 729 §V.

## I. Telos

Replace the human-transcription stage in cruftless's resolver chain (TC39 → ECMA-262 → Rust source → Cargo → ...) with a stage-deterministic compilation stage. Each ECMA-262 algorithm section becomes one `IRFunction`; the lowering compiler emits Rust that targets the existing rusty-js-runtime Runtime helper surface; the linter enforces 1:1 IR-vs-spec correspondence.

The conjecture (cruftless seed §A8.33 + IR-DESIGN.md §9): **spec conformance gets monotonically easier post-IR**. Each new built-in method translation goes through the linter once, never drifts again. The 30+ P62 substrate fixes per session shape should not recur for IR-covered sections.

## II. Apparatus

The IR is **resolver-instance #0** per IR-DESIGN.md §0, decomposed into three sub-stages above Doc 729 §IV's resolver-instance #1 (Cargo):

| # | resolver-instance | input | output |
|---|---|---|---|
| 0a | editorial → emu-alg XML | TC39 normative intent | ECMA-262 emu-alg XML |
| 0b | spec parser + IR linter | ECMA-262 XML | validated IR functions |
| 0c | lowering compiler | IR functions | Rust source |

Operational state at IR-EXT 1 close:
- **0a**: external (TC39's publication pipeline; we read the output).
- **0b**: linter operational; SpecStepRecord input is **hand-authored** in Tier 1, **spec-XML-parsed** in Tier 2.
- **0c**: lower_to_rust operational; emits compilable Rust against the runtime helper surface.

## III. Alphabet (the IR node set)

~50 nodes in 9 categories per IR-DESIGN.md §3. Cumulative at IR-EXT 1 close:

- **Coercion / type-check**: RequireObjectCoercible, ToObject, ToPrimitive, ToString, ToNumber, ToInteger, ToLength, ToUint32, ToBoolean, ToPropertyKey, IsCallable, IsConstructor, IsArray, IsRegExp, SameValue, SameValueZero.
- **Slot / property**: HasSlot, GetSlot, Get, HasProperty, HasOwnProperty, OrdinaryObjectCreate, ArraySpeciesCreate, CreateDataPropertyOrThrow, LengthOfArrayLike.
- **Calls**: Call, Construct, Invoke.
- **Control flow**: Throw (typed by ErrorClass), Return, If, While, Let, Assign, LetIndex, AssignIndex, Expr.
- **Constants**: Undefined, Null, Bool, Number, Str, IntConst, This, Arg.
- **Operators**: OpAdd, OpSub, OpMul, LooseEq, StrictEq, Lt, Le, Not, IndexAdd, IndexAsValue, IndexAsKey, AsIndex.

The alphabet is **closed for Tier 1.5**. Tier 1.6 (iteration cluster) may add `IteratorOpen`/`IteratorNext`/`IteratorClose`. Tier 2 (spec parser) adds nothing; it consumes the alphabet from XML.

## IV. Disciplines lifted to the IR boundary

Each cruftless seed §A8 discipline encodes a runtime-side invariant. The IR makes the invariant structurally inviolable at construction time, before reaching the runtime:

| cruftless §A8 | IR encoding | bug class eliminated |
|---|---|---|
| §A8.28 descriptor-shape | OrdinaryDefineOwnProperty(O, P, Desc) takes typed Desc record | leaked default `{w,e,c}=true` into spec-tight context |
| §A8.29 abstract-ops duality | IR exposes only dispatching form; pure form is lowering optimization | called pure form when receiver might be Object |
| §A8.30 brand-check | GetSlot(O, [[XData]]) auto-throws TypeError | silently fell through on missing internal slot |
| §A8.31 SyntaxError canonical | Throw(class) takes typed enum variant | misclassified TypeError where spec says SyntaxError |
| §A8.32 ToPrimitive at operator | OpAdd / LooseEq route through op_add_rt / is_loosely_equal_rt | called pure-primitive op when operand might be Object |

## V. Future-move discipline

**M1. Section selection.** The next IR translation is chosen by yield × structural similarity:
1. **High yield, low novelty**: methods that share most of an already-translated section's structure. Pattern: Array.prototype.{forEach, filter, every, some} all share §23.1.3.20's iteration shape. Translation cost is small per method; the IR linter pins each to its own spec section.
2. **High yield, high novelty**: methods with structural primitives the IR doesn't yet have (e.g., async generators, AbruptCompletion plumbing). Add the primitive once, drain the consumers.
3. **Low yield, high test-262 visibility**: chapters cruftless's hand-written impl already covers well (e.g. Number). Defer.

**M2. Runtime helper coverage.** The lowering compiler emits Rust against a fixed Runtime API. When a new IR node lowers to a helper that doesn't yet exist on Runtime, add the helper. Helpers added in Tier 1.5: `to_object`, `length_of_array_like`, `has_property_via`, `read_property_via`, `create_data_property_or_throw`, `array_species_create`. Tier 1.6 may add: `iterator_open`, `iterator_next`, etc.

**M3. Drift detection at every commit.** Every IR translation lands with its `spec_steps()` (the SpecStepRecord list). The drift-demo example pattern (`lint_drift_demo.rs`) proves the linter catches the intended bug class. Periodically run the linter against all translated sections; ensure zero findings outside the known `param.*` binding-convention markers.

**M4. Regression discipline.** When an IR-lowered function replaces a hand-written one in rusty-js-runtime, the corresponding test262 slice must pass at or above its prior rate. Regression below the prior rate is a Tier-1.5 incompleteness signal (the IR's runtime-helper surface needs expansion).

**M5. Spec-XML parser (Tier 2 trigger).** When the count of hand-translated SpecStepRecord lists exceeds ~10 sections, the parser-cost-vs-translation-cost cross-over justifies building it. Until then, hand-authoring the records is the cheaper move.

## VI. Termination conditions for the workstream

The IR reaches "full representation" when:

1. **All cruftless-implemented built-in surface is IR-encoded.** Concretely: every `register_intrinsic_method` site in `pilots/rusty-js-runtime/derived/src/{intrinsics, prototype, promise, regexp, ...}.rs` is replaced by an IR-derived `generated.rs` entry.
2. **The linter passes against every IR function.** Hand-authored or XML-parsed records, the diff is zero.
3. **The test262 slice for each translated section holds at-or-above its pre-IR rate.** No conformance regression from the swap.

At that point the IR is the canonical authoring surface; intrinsics.rs / prototype.rs become thin registration shims that point to `generated.rs`.

## VII. Out-of-scope (honestly delimited)

- **User JS bytecode.** The IR is for spec-built-in operations only. User-authored JS still goes through parse → bytecode-compile → bytecode-VM unchanged.
- **Non-ECMA APIs.** Bun-specific surface (Bun.serve, Bun.file, etc.), Node-compat (fs, http, ...), and host-v2 native code are not in scope. Those remain hand-authored.
- **Runtime performance.** The IR is a *correctness* artifact, not a speed artifact. The lowering can substitute pure-primitive forms when receiver type is statically provable, but no aggressive optimization passes in Tiers 1–3.
- **AsyncFunction / Generator semantics.** Async/generator algorithms in ECMA-262 reference suspendable execution contexts. The IR can encode the algorithm structure, but the lowering would need cruftless's existing async/generator machinery as targets; deferred to Tier 4.

## VIII. Resume protocol

Read `IR-DESIGN.md` + this seed + `trajectory.md`. Latest committed state is at `pilots/rusty-js-ir/derived/`; build with `cargo build -p rusty-js-ir`; lower a section with `cargo run --example lower_array_map -p rusty-js-ir`; lint with `cargo run --example lint_array_map -p rusty-js-ir`; drift-demo with `cargo run --example lint_drift_demo -p rusty-js-ir`.

After Tier 1.5, the working surface is `pilots/rusty-js-ir/derived/src/sections/`. Each new spec-section translation is one file in that directory, plus one registration edit in `pilots/rusty-js-runtime/derived/src/prototype.rs` (or `intrinsics.rs`) to route the corresponding method through `crate::generated::<rust_name>`.

The lower_to_rust output for *all* translated sections currently lives in `pilots/rusty-js-runtime/derived/src/generated.rs` (one file holds them all). When the count grows, refactor to one `generated/<section>.rs` per section.
