# Resolution-Layer Locale Snapshot — LPA-EXT 4 output

Current snapshot of the locale graph stratified by resolver layer and composition state.

This file complements, but does not replace, `apparatus/locales/manifest.json`. The manifest is the generated inventory. This snapshot is the human-readable apparatus view: which resolution layers have active locale coverage, what each layer is currently composing with, and where the latest full-suite matrix still shows pressure.

Baseline inputs:

- Locale inventory: `apparatus/locales/manifest.json`
- Locale count: 128 total, 106 top-level, 22 nested, 0 manifest warnings
- Latest full-suite matrix available in repo: `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-26-140256-p2/matrix.md`
- Matrix baseline: 23,829 PASS / 27,971 FAIL / 1,199 SKIP; runnable pass rate 46.0%
- Current limitation: many older locales have `status: null` in the generated manifest. Treat this snapshot as a composition map, not a canonical closed/open counter.

---

## I. Layer Summary

| Resolution layer | Matrix pressure | Locale state | Composition read |
|---|---:|---|---|
| Apparatus / measurement | N/A | Mature; active instruments under `pilots/apparatus/` | Test262 categorization, full-suite interpretation, LPA, diff-prod, CRB, TCC/TXC. This layer is producing the coordinates used by substrate locales. |
| Source-to-AST parser / lexical grammar | 903 parser-early-error + 61 missing-lex-feature | Broad coverage, several active/closed tokenization and parser locales | Tokenization work has split into numeric, identifier, string, regexp, private-name, and class-elements follow-ons. Parser early-error residual still needs periodic repartitioning after class/private field closures. |
| AST-to-bytecode / language lowering | 10,839 | Largest unresolved ECMA-262 layer | Existing locales cover portions, but this layer remains too broad for one locale. Needs stratification by class semantics, eval/function declaration semantics, arguments object, assignment/for-head lowering, async/generator lowering, dynamic import, and Annex B language semantics. |
| Runtime ECMA-262 spec built-ins | 7,717 | Mixed: Temporal and Annex B have founded locales; many built-ins remain diffuse | Temporal is the largest single availability gate. Non-Temporal built-ins need grouping around AggregateError, AsyncDisposableStack, AsyncFromSyncIteratorPrototype, Array.from, Date legacy, iterator protocol, species, and descriptor shape. |
| Runtime buffer / typed-array / ArrayBuffer | 2,728 | Founded but mostly baseline-stage | `typed-array-missing-method`, `typed-array-wrong-result`, and resizable-buffer locales exist. Next useful move is exemplar family marginals, not a new spawn. |
| Runtime RegExp | 865 | Active parent locale | `regexp-conformance` has begun closing bridges (`split` captures, `compile`). Remaining pressure should be partitioned into lex/static errors, matcher semantics, unicode/property escapes, legacy accessors, and prototype/String integration. |
| Runtime agent memory | 480 | Undercovered | Atomics / SharedArrayBuffer / wait / waitAsync surfaces are visible in the matrix but not yet represented by a clear top-level conformance locale in this snapshot. |
| Runtime object internals | 451 | Partially covered by object/array discipline locales | Descriptor shape, OrdinaryDefineOwnProperty, read-only throw behavior, realm/prototype chain, and Object.defineProperties remain active pressure. |
| Runtime array exotic | 401 | Covered by several prior locales, still residual | Array exotic length, species, virtual properties, truncation, and create-data-property discipline have locale history. Needs residual audit against current matrix before new spawn. |
| Runtime job queue / Promise | 348 | Undercovered as a matrix coordinate | Promise species, Promise.all, reaction queue ordering, async jobs, and Array.fromAsync indicate an E4 job-queue locale may be warranted after sampling. |
| Host ECMA-402 | 3,045 in matrix, but now has fresh locale work after this matrix | Recently founded and advanced | `intl402-availability` exists after the latest pull. The matrix predates that work; next full-suite run should reclassify this layer. |
| Node / host APIs / package ecosystem | Mostly outside this matrix | Broad pilot coverage | diff-prod and CRB carry this layer: fs, crypto, stream/events/path, HTTP, caps, PM, WebCrypto, TLS, etc. Not directly comparable to Test262 ECMA-262 rows. |
| JIT / shapes / performance substrate | CRB pressure, not Test262 pressure | Mature architecture pilots | LeJIT, shapes, IC, OSR, typed-loop, value-domain, and stub-emitter locales compose through CRB rather than the full-suite ECMA matrix. |

---

## II. Current Top ECMA-262 Pressure

Excluding ECMA-402, the highest matrix resolver buckets are:

| Rank | Resolver instance | Count | Principal locale implication |
|---:|---|---:|---|
| 1 | `ast-to-bytecode/language-lowering` | 10,839 | Needs layer-internal stratification before more broad spawns. |
| 2 | `runtime/spec-builtins` | 7,717 | Temporal plus diffuse ECMA-262 built-ins. |
| 3 | `runtime/buffer-typed-array` | 2,728 | Existing typed-array locales should be advanced. |
| 4 | `source-to-ast/parser-early-error` | 903 | Existing parser/tokenization locales should be re-read against current residuals. |
| 5 | `runtime/regexp` | 865 | Existing `regexp-conformance` parent locale should continue. |
| 6 | `runtime/agent-memory` | 480 | Candidate missing or under-specified. |
| 7 | `runtime/object-internals` | 451 | Existing object/array discipline locales need current residual audit. |
| 8 | `runtime/array-exotic` | 401 | Same as object internals: audit before spawn. |
| 9 | `runtime/job-queue-promise` | 348 | Candidate likely, pending sample inspection. |
| 10 | `runtime/collection-intrinsics` | 162 | Lower pressure; probably defer behind Promise/jobs and Atomics. |

---

## III. Composition State By Layer

### Apparatus Layer

State: operational.

Primary locales:

- `apparatus/test262-categorize/`
- `apparatus/test262-categorize/full-suite/`
- `apparatus/pinart-categorizer-refinement/`
- `apparatus/tokenizer-error-classification-refinement/`
- `apparatus/runner-features-skip-deliberate-omissions/`
- `apparatus/locale-positioning-audit/`
- `apparatus/diff-prod/`
- `apparatus/cross-runtime-bench/`
- `apparatus/ts-consumer-corpus/`
- `apparatus/ts-execute-corpus/`

Composition read:

- This layer is doing its job: formerly blurred coordinates have been split into parser, lex, lowering, runtime, and policy/runner classes.
- `positioning-gaps.md` is stale relative to `intl402-availability`, `regexp-conformance`, Annex B, class/private-field, and recent Intl work. This snapshot is the current replacement until LPA re-renders the full top-N table.

### Lex / Parser Layer

State: active and increasingly stratified.

Representative locales:

- `numeric-literal-conformance/`
- `identifier-tokenization/`
- `string-literal-and-escape-conformance/`
- `private-name-lexing/`
- `class-elements-static-semantics/`
- `parser-precedence-in-flag/`
- `parser-permissiveness-audit/`
- `parser-early-error-residual/`
- `for-of-async-lookahead/`
- `strict-mode-parser-tracking/`
- `strict-binding-eval-arguments/`
- `strict-binding-id-in-assignment-pattern/`

Composition read:

- Tokenization is now split well enough that new lex-tier work should usually be nested or sibling-scoped, not broad.
- Class/private-name work has shown the Rule-23 pattern: apparent lexing failures redirected to parser/static semantics and runtime private slots.
- Parser early-error residual remains large enough to merit a refreshed residual table after the latest class/private-field work.

### AST-to-Bytecode / Language-Lowering Layer

State: highest pressure, insufficiently stratified.

Visible matrix pressure:

- 10,839 total under `ast-to-bytecode/language-lowering`.
- Top surfaces include class statements/expressions, Annex B language, for-await-of, async generator, object expressions, dynamic import, compound assignment, eval-code, arguments object, for-of, with, try, switch, and function/generator surfaces.

Representative existing or related locales:

- `for-head-non-binding-lhs/`
- `array-pattern-rest-trailing-comma/`
- `yield-in-function-params/`
- `non-simple-params-strict-body/`
- `ts-resolve-*` family for TS source lowering, now mostly complete at parse parity

Composition read:

- This is the main missing apparatus view: current locales do not yet form an obvious layer-complete map for language lowering.
- Recommended next apparatus move is not one substrate locale; it is an LPA/T262C derivative table partitioning this layer by syntactic family and failure mode.

### Runtime ECMA-262 Built-Ins

State: mixed; some high-yield chapters founded, many diffuse residuals.

Representative locales:

- `temporal-availability/`
- `annexB-runtime-quirks/`
- `spec-builtins-wrong-result/`
- `iterator-protocol-error-propagation/`
- `iterator-close-on-abrupt/`
- `iterable-primitive-tobject/`
- `promise-executor-functions-meta/`
- `array-search-arg-strict-coerce/`
- `array-sort-tostring-dispatch/`
- `set-ops-object-key-identity/`

Composition read:

- `temporal-availability` is the largest single absent-chapter gate, but still at baseline-only in the observed trajectory.
- Annex B runtime has a good first-rung selection: String HTML methods first, then escape/unescape.
- Non-Temporal spec-builtins need a sharper grouping before more spawns; AggregateError, AsyncDisposableStack, iterator helpers, Date legacy, species, and descriptor shape should not be mixed.

### TypedArray / ArrayBuffer Runtime Layer

State: founded, pending exemplar-driven advancement.

Representative locales:

- `typed-array-missing-method/`
- `typed-array-wrong-result/`
- `typed-array-resizable-buffer-indexed-access/`

Composition read:

- This is a coherent ECMA-262 built-in layer with enough mass to prioritize.
- The next move should be running and recording exemplar baselines/family marginals in the existing locales, not spawning new typed-array siblings yet.

### RegExp Runtime Layer

State: active.

Representative locales:

- `regexp-conformance/`
- `regexp-instance-accessor-shadow/`
- `regexp-proto-test-coercion/`
- `regexp-split-captures-bridge/`
- `regex-engine-substrate/`

Composition read:

- Parent locale exists and has landed two runtime bridges.
- Remaining matrix pressure is multi-origin: literal lexing, syntax error conformance, matcher semantics, unicode/property escapes, legacy accessors, and String integration.
- `regex-literal-lexing` should stay nested or sibling-scoped under this parent unless baseline inspection proves independent multi-rung shape.

### Atomics / Agent Memory Layer

State: visible matrix pressure, under-covered in locale graph.

Visible matrix pressure:

- `runtime/agent-memory`: 480.
- Surface examples: `Atomics.waitAsync`, `Atomics.wait`, SharedArrayBuffer/Atomics descriptors, and `$262` host-hook related rows.

Composition read:

- This likely needs its own candidate after sample inspection.
- Must distinguish real engine semantics from runner/host-hook residue before spawning.

### Object Internals / Array Exotic Layer

State: partially covered, needs residual audit.

Representative locales:

- `array-exotic-virtual-property-discipline/`
- `array-create-data-property-discipline/`
- `array-species-create-discipline/`
- `array-length-setter-truncation/`
- `array-literal-elision-length/`
- `length-of-array-like-propagate/`

Composition read:

- Several historical locales closed important substrate constraints, but the matrix still shows object-internals and array-exotic residuals.
- Next useful action is a residual audit against the current matrix, not immediate new spawn.

### Promise / Job Queue Layer

State: visible but under-covered.

Visible matrix pressure:

- `runtime/job-queue-promise`: 348.
- Top examples include Promise species, Promise.all, Array.fromAsync, and async job/reaction ordering.

Composition read:

- This is a plausible next candidate after sample inspection.
- It should be framed as E4 job-queue / promise reaction semantics rather than as isolated Promise method stubs.

### Host ECMA-402 Layer

State: newly active after the latest pull.

Representative locale:

- `intl402-availability/`

Composition read:

- The current matrix predates the recent Intl work. Do not use the 3,045 count as post-Intl status.
- Next full-suite run should show whether the Intl coordinate moved from availability to value-semantics and missing-method residuals, mirroring the Temporal expected avalanche.

### Node / Host API / Package Ecosystem Layer

State: measured mostly outside Test262.

Representative locales:

- `rusty-js-caps/`
- `rusty-js-http-server/`
- `rusty-js-pm/`
- `fetch-api/`
- `node-fs/`
- `node-http/`
- `node-path/`
- `web-crypto/`
- `tls/`
- `x509/`

Composition read:

- diff-prod and CRB are the primary apparatus here.
- This layer should not be read directly from Test262 ECMA-262 counts.

### JIT / Shapes / Performance Substrate Layer

State: mature architecture chain, measured by CRB/bench, not Test262.

Representative locales:

- `rusty-js-jit/`
- `rusty-js-shapes/`
- `rusty-js-shapes/consumer-migration/`
- `interp-hot-intrinsics/`
- `interp-getprop-ic/`
- `iter-protocol-bytecode-rewrite/`
- `jit-stub-emitter/`
- `jit-value-domain/`
- `jit-typed-loops/`
- `jit-osr/`

Composition read:

- This layer is the strongest example of cross-locale composition: IHI → GPI → IPBR, Shape → CMig, JIT → StubE / Shape / VD / TL.
- Its state should be tracked through CRB and trajectory chapter closes, not Test262 matrix rows.

---

## IV. Immediate Apparatus Gaps

1. **Generated manifest lacks normalized state.**
   `status` is nullable free text. That is acceptable for inventory, but weak for layer-level dashboards. A future manifest extension should add generated fields such as `state: founded|active|closed|blocked|unknown`, while preserving the existing free-text status.

2. **Layer taxonomy is implicit.**
   Resolver layer must currently be inferred from path names, seed prose, and matrix coordinates. A future `apparatus/locales/layers.json` or manifest extension should map locale coord → primary layer(s).

3. **LPA positioning gaps are stale after several spawns.**
   `positioning-gaps.md` still reflects the 2026-05-25 matrix and predates `intl402-availability`, `regexp-conformance`, and several downstream redirects. This snapshot should trigger a full LPA re-render when the next full-suite matrix lands.

4. **AST-to-bytecode needs its own layer partition.**
   It is the largest ECMA-262 pressure surface. Current layer state is too broad; split by syntactic family and projection before spawning more work.

5. **Atomics and Promise/jobs are visible but under-localed.**
   Both need sample inspection before candidate creation.

---

## V. Recommended Next Snapshot Shape

For the next generated or semi-generated version, record one row per locale:

| Field | Purpose |
|---|---|
| `coord` | Stable manifest coordinate. |
| `primary_layer` | Apparatus, lexer, parser, lowering, runtime-builtins, typed-array, regexp, atomics, object-internals, array-exotic, promise-jobs, host-api, jit, shapes, etc. |
| `secondary_layers` | Cross-tier composition, if any. |
| `state` | Normalized state. |
| `current_rung` | Latest trajectory rung. |
| `matrix_coordinate` | Pin-Art matrix coordinate, if any. |
| `composition_edges` | Parent/child or sibling chain edges. |
| `next_action` | Baseline, substrate move, residual audit, close, defer. |

That shape would make this snapshot reproducible from the manifest + seeds + trajectories while preserving the richer prose view here.

