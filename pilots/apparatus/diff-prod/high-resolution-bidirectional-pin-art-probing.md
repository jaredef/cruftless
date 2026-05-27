# High-Resolution Bidirectional Pin-Art Probing Methodology

A theorized methodology for applying the corpus's Pin-Art apparatus (Docs 270, 619, 658, 705, 707) to the cruftless engine's ECMA-262 conformance surface at high resolution, using diff-prod fixtures as the probe set and the test262 matrix as the surface.

This document composes three corpus-level forms into one operational pipeline:

1. **Doc 707's bidirectional reading**: each probe carries information in both directions — forward (the engine must satisfy this behavior) and backward (the engine reveals an implicit constraint the probe's designer did not target).
2. **Doc 658's ring-stratified constraint hierarchy**: probes organize by ring density (Ring 1: lifecycle boundaries with superlinear leverage; Ring 2: structural completion; Ring N: diminishing-returns residuals).
3. **Doc 705's intra-architectural seam detection**: the joint pattern of probe positions across the engine's resolution layers reveals architectural seams that namespace decomposition obscures.

---

## I. The Probe Set

A diff-prod fixture is a Pin-Art probe. It presses against the engine's behavioral surface at a specific point and records one bit: resistance encountered (FAIL) or not (PASS). The fixture's stdout is the probe tip; the byte-for-byte diff against bun is the resistance signal.

The current probe set: 112 fixtures, 58 PASS / 54 FAIL. Each fixture probes 5-20 individual behaviors, making the effective probe density approximately 800-1,200 individual assertions across the behavioral surface.

The probes are peer-independent at the point of contact (Doc 619 §I component 1): each fixture runs in its own process, shares no state with other fixtures, and produces output determined solely by the engine's behavior on its input. The discriminator is location-on-surface (which ECMA-262 feature), not depth-of-probe (all probes use the same canon() → JSON → stdout → diff pipeline).

### Probe taxonomy

The five pin classes from Doc 707, instantiated for this apparatus:

| Pin class | Cruftless instantiation | Current count |
|---|---|---|
| Spec invariant | Fixture behaviors derived directly from ECMA-262 sections | ~60 fixtures |
| Test rep | Behaviors from the existing 42-fixture diff-prod baseline | 42 fixtures |
| Consumer expectation | Behaviors exercised by real npm packages (implicit in diff-prod fixture design) | ~10 fixtures |
| Conformance suite | test262 full-suite results (50,506 tests) | 49,065 runnable |
| Implementation-source probe | Source-level analysis of compiler.rs, interp.rs, lexer.rs | 6 identified gaps |

---

## II. The Surface

The engine's behavioral surface is the totality of ECMA-262 observable behavior: every value a conformant program can produce, every error it must throw, every property it must have, every ordering it must preserve. The surface is not a line or a plane; it is a high-dimensional manifold where each dimension is a spec section's behavioral contract.

The test262 matrix projects this surface onto 12 resolver instances, 294 distinct coordinates, and 735 distinct surfaces. The diff-prod suite samples it at 112 points with effective density ~1,000.

### Surface topology

The surface has seams (Doc 705 §1) — boundaries between distinct architectural forms:

| Seam | Engine boundary | Probe signal |
|---|---|---|
| Lexer → Parser | Token emission → AST construction | `asi-rules` PASS, `regex-division-ambiguity` PASS: seam is clean |
| Parser → Compiler | AST → Bytecode | `tagged-template-raw` FAIL: raw data crosses the seam but is dropped |
| Compiler → Runtime | Bytecode → Execution | `generator-suspension` crash: compiler emits eager-collect, runtime expects coroutine |
| Runtime → Host | JS semantics → Node API | `node-os-timers` PASS, `node-assert` FAIL: seam is partially clean |
| Spec → Engine | ECMA-262 → Implementation | `abstract-equality` PASS, `coercion-pipeline` FAIL: ToPrimitive hint dispatch is a spec-engine seam leak |

The seams are where probe density should be highest (Doc 658 §4: Ring 1 constraints at lifecycle boundaries). The current probe set is densest at the Runtime layer (35 fixtures) and thinnest at the Compiler → Runtime seam (5 fixtures targeting the 5 newly spawned compiler locales). This density mismatch is itself a finding.

---

## III. The Bidirectional Reading

### Forward direction

Each FAIL fixture is a constraint on future substrate work. The engine must satisfy the behavior the fixture asserts. The 54 FAIL fixtures compress into 9 mechanism gaps (LPA-EXT 10 Finding LPA.22); each mechanism gap is a forward constraint:

| Mechanism gap | Forward constraint |
|---|---|
| #1 ToPrimitive hint dispatch | The runtime's `to_primitive.rs` must thread the hint ("number", "string", "default") to `[Symbol.toPrimitive]` |
| #2 IteratorClose protocol | The compiler must emit `Op::IterClose` at every consuming site where the spec requires it |
| #3 Generator suspension | The compiler+runtime must implement coroutine-style suspend/resume, not eager-collect |
| #4 Eval lexical capture | The eval compiler must resolve against the enclosing declarative environment record |
| #5 Finally on abrupt exit | The compiler must emit `TryExit` before break/continue/return that crosses a try boundary |
| #6 OrdinaryOwnPropertyKeys | The property storage layer must partition integer-indexed keys from string keys |
| #7 Proxy trap invariants | The runtime's proxy handler must enforce §10.5 invariants for all 13 traps |
| #8 Arguments object shape | The runtime must create an exotic Arguments object, not a plain Array |
| #9 strings.raw | The compiler must thread raw quasis from the lexer through to the runtime template object |

### Backward direction

Each PASS fixture reveals an implicit invariant of the engine that was not necessarily designed for. These are Doc 707's "behavioral commitments the implementation is implicitly committed to." The 58 PASS fixtures reveal:

| PASS fixture | Backward reading: the engine is committed to |
|---|---|
| `asi-rules` | Correct `preceded_by_line_terminator` tracking across all ASI-sensitive positions, including do-while, method chaining, and return+newline |
| `closure-capture-order` | Per-iteration fresh binding in for-let loops via `ResetLocalCell` — each iteration's closures capture independent cells |
| `computed-property-order` | Left-to-right evaluation of computed property keys, including Symbol keys, with ToPropertyKey coercion |
| `microtask-ordering` | Promise.resolve().then() microtask ordering matches V8/Bun — nested .then chains interleave correctly |
| `samevalue-algorithms` | SameValueZero in Map/Set/includes correctly distinguishes NaN (finds it) from ±0 (treats as same) |
| `destructuring-iterators` | Array destructuring routes through the iterator protocol for generators, Sets, Maps, and custom iterables |
| `switch-fallthrough` | Switch fallthrough semantics including default-in-middle, strict === comparison, and block scoping in cases |

The backward column is information that exists nowhere in the engine's source comments, test suite, or documentation. The diff-prod apparatus surfaces it empirically.

### The implicit-constraint discovery rate

From LPA-EXT 7's resolver-axis gap partition: 25 of 49 gaps (51%) were **implicit constraints** (`░` markers) discovered only when fixture probes collided with the engine surface. These are the backward-direction pins that the probe set discovered — constraints the engine's own declared gap surface did not represent.

This 51% implicit rate is the apparatus's backward-direction yield. For every 2 probes pressed against the surface, roughly 1 discovers a constraint the engine's developers were unaware of. At the current probe density (~1,000 effective assertions), this suggests ~500 implicit constraints are already surfaced by the apparatus, awaiting extraction.

---

## IV. The Ring Hierarchy

Per Doc 658, probes organize by constraint-density ring:

### Ring 1 — Lifecycle boundary constraints (superlinear leverage)

Probes at resolution-layer seams where a single fix flips failures across multiple surfaces:

| Ring 1 probe | Seam | Surfaces affected | Estimated flip count |
|---|---|---|---|
| `generator-suspension` | Compiler → Runtime | generators, async generators, for-await-of, yield* | 1,492 test262 rows |
| `iterator-close` | Compiler emission | for-of, destructuring, yield*, Array.from, spread | ~500 test262 rows |
| `coercion-pipeline` (ToPrimitive) | Runtime abstract ops | +, ==, <, template, comparison | ~400 test262 rows |
| `eval-lexical-capture` | Compiler → Runtime | eval, Function(), indirect eval | ~212 test262 rows |

Ring 1 constraints have superlinear leverage: fixing 4 mechanisms flips ~2,600 test262 rows (15% of remaining failures). This is Doc 658's prediction operationalized: Ring 1 constraints at lifecycle boundaries close the largest class of failures per unit of specification.

### Ring 2 — Structural completion constraints

Probes that close individual built-in surfaces:

| Ring 2 probe | Surface | Estimated flip count |
|---|---|---|
| `tagged-template-raw` | Template object construction | ~27 test262 rows |
| `object-seal-freeze` | Strict-mode integrity enforcement | ~50 test262 rows |
| `regexp-named-groups` | Named capture group population | ~30 test262 rows |
| `string-normalize` | Unicode normalization | ~20 test262 rows |
| `symbol-toprimitive` | Hint dispatch | ~40 test262 rows |
| `property-key-order` | OrdinaryOwnPropertyKeys | ~80 test262 rows |

Ring 2 constraints have linear leverage: each closes one surface. The aggregate (~247 rows) is smaller than any single Ring 1 fix. Doc 658's prediction: Ring 2 work is structural completion, not edge-case erasure.

### Ring N — Diminishing-returns residuals

Probes that close individual edge cases:

| Ring N probe | Example | Estimated flip |
|---|---|---|
| `finally-return-override` (loop interaction only) | `break` inside try-finally in a loop | ~5 test262 rows |
| `string-position-methods` (endsWith position arg) | `"hello".endsWith("hel", 3)` | ~3 test262 rows |
| `weakref-registry` (Symbol target) | `new WeakRef(Symbol())` | ~2 test262 rows |

Ring N constraints have sublinear leverage. They are real conformance gaps but do not drive ecosystem compatibility.

---

## V. The Operational Pipeline

Composing Doc 705's five-step pipeline with the diff-prod apparatus:

### Step 1 — Probe extraction

Run the 112-fixture diff-prod suite. Each fixture is a probe. The output is a PASS/FAIL vector of length 112 with per-fixture divergence details.

### Step 2 — Signal-cluster identification

Cluster the FAIL fixtures by mechanism gap. The current clustering: 54 FAIL → 9 mechanism gaps. Each cluster is a detection-hedging signal (Doc 619 §4): the engine "hedges" at these points, producing divergent output where it should produce identical output.

### Step 3 — Cross-layer seam reading

Map each mechanism-gap cluster to the architectural seam it straddles:

```
Mechanism gap → Resolution layer boundary
#1 ToPrimitive  → IR/Runtime boundary (to_primitive.rs sections vs interp.rs dispatch)
#2 IterClose    → Compiler/Runtime boundary (emission site vs execution handler)
#3 Generators   → Compiler/Runtime boundary (eager-collect vs coroutine)
#4 Eval capture → Compiler scoping boundary (module scope vs eval scope)
#5 Finally      → Compiler control-flow boundary (TryEnter/TryExit vs Jump)
#6 PropKeyOrder → Runtime property-storage boundary (HashMap vs spec-ordered)
#7 Proxy invs   → Runtime MOP boundary (trap handler vs target invariants)
#8 Arguments    → Compiler/Runtime boundary (slot allocation vs object construction)
#9 strings.raw  → Lexer/Compiler/Runtime triple boundary (token → AST → bytecode → object)
```

The seam reading reveals that 5 of 9 mechanism gaps straddle the **Compiler → Runtime boundary**. This is the dominant seam. Doc 705's prediction: the seam with the highest probe-density signal is the seam most in need of architectural attention.

### Step 4 — Resistance-as-boundary verification

Per Doc 693: verify that each seam-straddling gap is genuine by checking that the resistance is structural (the two layers have incompatible representations) rather than incidental (a simple bug).

| Gap | Structural or incidental? |
|---|---|
| #3 Generators | **Structural**: compiler emits `__yield_push__` call; runtime expects frame-stack coroutine. Incompatible representations. |
| #2 IterClose | **Incidental**: opcode exists (0xD2); compiler just doesn't emit it. Single-layer fix. |
| #9 strings.raw | **Structural**: lexer produces raw data, AST doesn't carry it, compiler doesn't emit it, runtime doesn't construct it. Triple-layer gap. |
| #4 Eval capture | **Structural**: compiler creates a fresh module context for eval; needs to thread enclosing scope. |
| #8 Arguments | **Incidental**: runtime fills arguments_slot with Array; needs exotic object instead. Single-layer fix. |

Structural gaps require multi-layer coordination (new locales). Incidental gaps require single-layer fixes (scope extensions of existing locales).

### Step 5 — Revised decomposition

The seam reading revises the work decomposition:

**Before (matrix-count-derived):** 12 resolver instances ranked by failure count. Runtime/spec-builtins (6,114) dominates. Work appears to be broadly distributed.

**After (Pin-Art seam-derived):** One dominant seam (Compiler → Runtime) owns 5 of 9 mechanism gaps and gates ~2,600 test262 rows. The seam is served by the 5 newly spawned compiler locales. The remaining 4 gaps are in Runtime-internal surfaces (ToPrimitive, property ordering, proxy invariants, arguments shape).

The revised decomposition is structurally simpler: close the compiler seam first, then close the runtime surfaces. The raw count matrix would have directed attention to runtime/spec-builtins (6,114 rows); the Pin-Art seam reading directs attention to the compiler seam (fewer rows but higher leverage per fix).

---

## VI. The Cybernetic Loop

Per Doc 615's substrate-dynamics loop, the apparatus operates as a closed cycle:

```
1. Run diff-prod probes           → probe-position vector (PASS/FAIL)
2. Cluster by mechanism gap        → detection-hedging signals
3. Map to architectural seams      → seam decomposition
4. Spawn/extend locales at seams   → substrate work plan
5. Land substrate fixes            → engine changes
6. Re-run diff-prod probes         → updated probe-position vector
7. Measure FAIL→PASS flips         → verification
8. Discover new implicit constraints → backward-direction yield
9. Return to step 2                → loop
```

The loop terminates when the probe set produces no new detection-hedging signals (all fixtures PASS) or when the remaining FAIL fixtures are all Ring N (diminishing returns below the engagement's cost threshold).

### Loop iteration metrics

| Iteration | Date | Fixtures | PASS | FAIL | Mechanism gaps | Seams identified |
|---|---:|---:|---:|---:|---:|---:|
| 0 (baseline) | pre-session | 42 | 42 | 0 | 0 | 0 |
| 1 | 2026-05-27 | 112 | 58 | 54 | 9 | 5 |
| 2 (predicted) | next session | 112+ | ~75 | ~37 | ~5 | ~3 |

The predicted iteration 2 assumes the 5 compiler locales and the Ring 1 ToPrimitive fix land. The FAIL count should drop by ~17 (the fixtures directly gated by the 4 Ring 1 mechanisms), and new implicit constraints discovered by the now-passing fixtures may surface 2-3 new mechanism gaps at deeper layers.

---

## VII. Density and Resolution

The apparatus's resolution is bounded by probe density. At 112 fixtures with ~1,000 effective assertions, the resolution is approximately:

- **Layer resolution**: 13 resolution layers, ~8 fixtures per layer average, ~77 assertions per layer
- **Seam resolution**: 5 identified seams, ~2 fixtures per seam (the Compiler → Runtime seam has 5 dedicated fixtures; other seams have 1-2)
- **Surface resolution**: 735 distinct test262 surfaces, ~1.4 assertions per surface average (extremely sparse)

The density mismatch is the apparatus's primary limitation. The test262 matrix covers 735 surfaces; the diff-prod suite covers ~100. Each additional fixture increases resolution linearly. The high-leverage density increase is at the Compiler → Runtime seam (currently 5 fixtures, should be 15-20) and at the Runtime abstract-operations layer (currently 4 fixtures, should be 10-15).

### Density targets for the next iteration

| Layer | Current fixtures | Target | Gap |
|---|---:|---:|---:|
| Compiler → Runtime seam | 5 | 20 | +15 |
| Runtime abstract operations | 4 | 15 | +11 |
| TypedArray / ArrayBuffer | 4 | 10 | +6 |
| RegExp | 5 | 10 | +5 |
| Promise / Jobs | 5 | 10 | +5 |
| ECMA-402 Intl | 2 | 8 | +6 |

Target: ~160 fixtures at ~1,600 effective assertions, doubling the seam-resolution at the dominant boundary.

---

## VIII. Falsifiers

Per Doc 658 §7, the methodology's claims are falsifiable:

**F1 (Ring 1 leverage prediction):** The 4 Ring 1 mechanisms (#1-#4) should flip ≥2,000 test262 rows when fixed. If they flip <1,000, the ring hierarchy's leverage prediction is wrong.

**F2 (Seam dominance prediction):** The Compiler → Runtime seam should account for >50% of fixture FAIL→PASS flips in the next iteration. If it accounts for <30%, the seam reading is wrong.

**F3 (Implicit constraint rate):** The backward-direction implicit discovery rate should remain >30% of new probe assertions (i.e., >30% of newly passing fixture assertions should reveal engine commitments not in the engine's declared scope). If <10%, the backward direction has exhausted its yield.

**F4 (Loop convergence):** The FAIL count should decrease monotonically across iterations (assuming no regressions in the existing PASS set). If a non-regression FAIL appears, the probe set has a false-positive or the engine has an environment-dependent behavior.

---

## IX. Composition with the Engagement's Apparatus

This methodology slots into the existing apparatus stack:

| Apparatus | Role | Frequency |
|---|---|---|
| **diff-prod** (this methodology) | High-resolution bidirectional probing at behavioral surface | Per-substrate-move |
| **test262 sample** (7,598 tests) | Curated conformance measurement | Per-session |
| **test262 full-suite** (50,506 tests) | Comprehensive matrix for coordinate identification | Per-major-arc |
| **CRB** (cross-runtime bench) | Performance measurement | Per-JIT/shapes move |
| **TCC / TXC** | TypeScript parse/execute parity | Per-TSR move |
| **LPA** | Locale positioning and gap audit | On-trigger |

Diff-prod is the fastest feedback loop (seconds per fixture, minutes per suite). Test262 sample is medium (minutes). Full-suite is slowest (30-60 minutes). The Pin-Art methodology uses diff-prod for probe iteration and test262 for verification of leverage predictions.

---

## X. Standing Recommendation

The methodology is operational at the current probe density (112 fixtures). The next-leverage move is not more fixtures — it is **landing the Ring 1 fixes** (generator suspension, IteratorClose, ToPrimitive, eval capture) and measuring the test262 flip count. The flip count is the empirical test of the ring hierarchy's leverage prediction (falsifier F1). If the prediction holds, the methodology's value claim is empirically grounded; if it fails, the ring assignment needs revision.

The apparatus produces two outputs per iteration, per Doc 707:

1. **Forward: the substrate work plan** — which mechanisms to fix, in which order, at which resolution layer.
2. **Backward: the implicit-constraint map** — which engine commitments the probe set has surfaced that were previously invisible.

Both outputs have standalone value. The work plan drives engineering. The constraint map drives architecture. The Pin-Art apparatus is the composition that produces both from the same probe set.
