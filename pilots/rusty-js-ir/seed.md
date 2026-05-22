# rusty-js-ir — Resume Vector / Seed

**Locale tag**: `L.rusty-js-ir` (per [Doc 737](../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md))

**Workstream**: spec-as-source-of-truth IR between ECMA-262 and rusty-js-runtime.
**Author**: 2026-05-19 session (post-cruftless P62.E25).
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Composes with**: `IR-DESIGN.md` (architecture); cruftless seed §A8.27–§A8.33; Doc 729 §V.

## I. Telos

Replace the human-transcription stage in cruftless's resolver chain (TC39 → ECMA-262 → Rust source → Cargo → ...) with a stage-deterministic compilation stage. Each ECMA-262 algorithm section becomes one `IRFunction`; the lowering compiler emits Rust that targets the existing rusty-js-runtime Runtime helper surface; the linter enforces 1:1 IR-vs-spec correspondence.

The conjecture (cruftless seed §A8.33 + IR-DESIGN.md §9): **spec conformance gets monotonically easier post-IR**. Each new built-in method translation goes through the linter once, never drifts again. The 30+ P62 substrate fixes per session shape should not recur for IR-covered sections.

### I.1 Refined telos (post-IR-EXT 11)

Two empirical refinements after the first 33 sections:

**(a) Telos is bounded coverage, not perfect coverage.** "Full representation" means *every section in cruftless's `register_intrinsic_method` registration table is either IR-encoded or explicitly carved out*. Carve-outs are legitimate when:
   - The section's hand-written impl is a perf-critical one-liner (Math.abs, Math.floor) and the IR overhead would dominate.
   - The section is a Bun-specific or Node-compat surface not in ECMA-262 (Bun.serve, fs.readFile).
   - The section's spec is unstable (TC39 stage-2/3 proposals).
   Each carve-out is recorded in trajectory.md with a one-line reason.

**(b) Telos is also alphabet completeness.** A section is *not blocked* on the alphabet if and only if its spec can be expressed in the existing IR nodes plus existing runtime helpers. Sections that need novel patterns drive alphabet extensions (HasArg, CallBuiltin, signed-Int, iterator-protocol, descriptor-builders). The alphabet is "complete" when no ECMA-262 algorithm shape from the cruftless coverage set requires a new IR primitive.

The conjunction of (a) + (b) gives a falsifiable termination condition: telos reached when *(i)* every non-carved-out cruftless section is in `sections/`, *(ii)* `cargo run --example lint_all` is ✓ across all entries, *(iii)* no section's translation triggered an alphabet extension in the last N rounds.

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

~52 nodes in 9 categories per IR-DESIGN.md §3. Cumulative at IR-EXT 11 close:

- **Coercion / type-check**: RequireObjectCoercible, ToObject, ToPrimitive, ToString, ToNumber, ToInteger, ToLength, ToUint32, ToBoolean, ToPropertyKey, IsCallable, IsConstructor, IsArray, IsRegExp, SameValue, SameValueZero.
- **Slot / property**: HasSlot, GetSlot, Get, HasProperty, HasOwnProperty, OrdinaryObjectCreate, ArraySpeciesCreate, CreateDataPropertyOrThrow, LengthOfArrayLike.
- **Calls**: Call, Construct, Invoke, **CallBuiltin** (added IR-EXT 4 — invoke a Runtime helper that isn't a JS-side method dispatch).
- **Control flow**: Throw (typed by ErrorClass), Return, If, While, Let, Assign, LetIndex, AssignIndex, Expr.
- **Constants**: Undefined, Null, Bool, Number, Str, IntConst, This, Arg, **HasArg** (added IR-EXT 4 — distinguishes "arg passed undefined" from "arg not passed").
- **Operators**: OpAdd, OpSub, OpMul, LooseEq, StrictEq, Lt, Le, Not, IndexAdd, IndexAsValue, IndexAsKey, AsIndex.

The alphabet has been **stable across IR-EXT 5 → IR-EXT 11** (5 chapters × 10 clusters × 33 sections without alphabet extensions). The two IR-EXT 4 additions (HasArg + CallBuiltin) covered the patterns that emerged through IR-EXT 11.

**Alphabet-extension triggers identified for future tiers**:
- Signed-Int + IndexSubtract + IndexLt: backward iteration (findLast spec-strict, reduceRight, lastIndexOf, ToInteger fromIndex normalization).
- IteratorOpen / IteratorNext / IteratorClose: Promise.all/allSettled/any/race + Set/Map ctor iterables.
- DescriptorBuild / DescriptorMerge: Object.defineProperty/defineProperties/getOwnPropertyDescriptor.
- NewPromiseCapability / SpeciesConstructor: Promise.all-style C-dispatch on `this`.

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

*(IR-EXT 6 status: parser landed at 12 sections. Currently operates on embedded fixtures; a tc39/ecma262 checkout + build.rs glue would close the live-source loop.)*

**M6. CallBuiltin preferred for non-JS-method abstract ops.** When a spec section's algorithm calls an abstract op that *isn't* a JS-side method dispatch (e.g., EnumerableOwnPropertyNames, SameValue, IsExtensible, ArraySpeciesCreate), prefer the CallBuiltin pattern: extract a Runtime helper `rt.<name>_via(...)`, then have the IR section call it through `Expr::CallBuiltin { name, args }`. This keeps the IR section thin and spec-traceable while leaving the implementation surface in idiomatic Rust.

Counter-rule: when the abstract op *is* a JS-side method dispatch (Call, Get, HasProperty), use the canonical IR primitive (Expr::Call, Expr::Get, Expr::HasProperty) — not CallBuiltin. These dispatch through the bytecode VM and respect JS semantics (accessor getters, proxy traps, etc.).

**M7. one_step_builtin builder for the 1-step canonical shape.** When a spec section reduces to "Return ? builtin(...)" — particularly common for Number static methods, global predicates, and Object integrity ops — use the `one_step_builtin` builder pattern (see `sections/number_static.rs`). Each section's IR builder collapses to ~3 lines of construction. The builder pattern is replicable across clusters; lift to a shared helper in `sections/mod.rs` when ≥3 clusters use it.

## VI. Termination conditions for the workstream

The IR reaches "full representation" when all four conditions hold:

1. **Every non-carved-out cruftless section is IR-encoded.** Concretely: every `register_intrinsic_method` site in `pilots/rusty-js-runtime/derived/src/{intrinsics, prototype, promise, regexp, ...}.rs` is either (a) replaced by a `generated.rs` invocation, or (b) recorded as a carve-out in trajectory.md with a one-line reason (see §I.1.a — perf-critical one-liners, Bun-specific, TC39 stage-2/3, etc.).
2. **`cargo run --example lint_all -p rusty-js-ir` exits ✓ across every entry.** Hand-authored or XML-parsed records, the diff is zero per spec section.
3. **`cargo run --example lint_from_spec -p rusty-js-ir` produces matching SpecStepRecord lists for every section that has a corresponding emu-alg block in the parser fixture set.** Resolver-instance #0b is fully closed.
4. **The test262 slice for each translated section holds at-or-above its pre-IR rate** (when the keeper authorizes a test262 run). No conformance regression from the swap.

At that point the IR is the canonical authoring surface; the hand-written registration files become thin shims that point to `generated.rs`.

### VI.1 Progress at IR-EXT 11 close

- 33 sections IR-encoded (28 wired through generated.rs, 5 IR-only).
- Linter ✓ 33/33.
- lint_from_spec ✓ on §23.1.3.20 (1 fixture; mass-fixture pending).
- test262: not measured since keeper directive (no sweeps without authorization). Smoke parity with Bun confirmed for each wired cluster.
- Estimated remaining coverage: ~50–80 more sections to reach full representation, distributed across String.prototype.*, Array.prototype mutators, RegExp.prototype.*, Date, Math, Reflect, the property-descriptor cluster, and Promise.all-family.

## VII. Out-of-scope (honestly delimited)

- **User JS bytecode.** The IR is for spec-built-in operations only. User-authored JS still goes through parse → bytecode-compile → bytecode-VM unchanged.
- **Non-ECMA APIs.** Bun-specific surface (Bun.serve, Bun.file, etc.), Node-compat (fs, http, ...), and host-v2 native code are not in scope. Those remain hand-authored.
- **Runtime performance.** The IR is a *correctness* artifact, not a speed artifact. The lowering can substitute pure-primitive forms when receiver type is statically provable, but no aggressive optimization passes in Tiers 1–3.
- **AsyncFunction / Generator semantics.** Async/generator algorithms in ECMA-262 reference suspendable execution contexts. The IR can encode the algorithm structure, but the lowering would need cruftless's existing async/generator machinery as targets; deferred to Tier 4.

## VIII. Resume protocol

Read `IR-DESIGN.md` + this seed + `trajectory.md`. Latest committed state is at `pilots/rusty-js-ir/derived/`; build with `cargo build -p rusty-js-ir`; lower a section with `cargo run --example lower_array_map -p rusty-js-ir`; lint with `cargo run --example lint_array_map -p rusty-js-ir`; drift-demo with `cargo run --example lint_drift_demo -p rusty-js-ir`.

After Tier 1.5, the working surface is `pilots/rusty-js-ir/derived/src/sections/`. Each new spec-section translation is one file in that directory, plus one registration edit in `pilots/rusty-js-runtime/derived/src/prototype.rs` (or `intrinsics.rs`) to route the corresponding method through `crate::generated::<rust_name>`.

The lower_to_rust output for *all* translated sections currently lives in `pilots/rusty-js-runtime/derived/src/generated.rs` (one file holds them all). When the count grows, refactor to one `generated/<section>.rs` per section.
