# array-prototype-sort - Trajectory

## 2026-05-29 - APS-EXT 0 - Phase 0 spawn + Phase 2 probe

### Directive

Helmsman directed R3 to spawn a new `array-prototype-sort` locale for the post-EPSUA top-ranked `Array.prototype.sort` sample matrix cell. Scope is Phase 0 and Phase 2 only; no runtime substrate edits are authorized in this founding round.

### Phase 0

Locale founded at `pilots/array-prototype-sort/` with seed and trajectory. The coordinate is broader than the older `array-sort-tostring-dispatch` locale: ASD closed the object ToString dispatch slice, while APS targets the current post-EPSUA precise sort semantics cluster.

Rule 11 pre-spawn coverage:

- **A1 component A/B**: Array.prototype.sort host implementation versus Array exotic property/accessor semantics.
- **A2 op-set**: `Array.prototype.sort(comparefn)`, default SortCompare string coercion, comparator call/throw path, HasProperty/Get/Delete/Set during sort collection and writeback.
- **A3 value-domain**: sparse arrays, explicit `undefined`, prototype numeric accessors, own accessors with side effects, primitive receivers, non-Array array-like receivers with negative length.
- **A4 locals-marshaling**: generated `array_prototype_sort` shim into `Runtime::array_proto_sort_via`, current `this` receiver, optional comparator.
- **A5 emission-shape**: runtime helper refactor in `pilots/rusty-js-runtime/derived/src/interp.rs`, not parser/lowering work.

### Phase 2 Baseline

Inspected `pilots/apparatus/test262-categorize/results/2026-05-29/matrix.md` and `categorized.jsonl`.

Matrix entry:

- Rank 1: `Array.prototype.sort` / `(no-feature-tag)` - 25 rows, example `built-ins/Array/prototype/sort/S15.4.4.11_A1.2_T2.js`.

Structure-axis marginal shows 26 total `Array.prototype.sort` failures in the sample. The extra row is `call-with-primitive.js` with `feat:Symbol;feat:BigInt`, outside the rank-1 no-feature-tag cell but still part of the current sort pipeline failure surface.

Segmentation across the 26 current rows:

| Bucket | Count | Share | Shape |
|---|---:|---:|---|
| Precise accessor/prototype side-effect ordering | 19 | 73.1% | `precise-*` tests expect sort to observe prototype accessors, own getters/setters, mutations, and comparator throw timing through spec `HasProperty` / `Get` / `Set` / `Delete` ordering. Current eager dense `object_get` snapshot and dense writeback collapse holes/accessors and mutate too eagerly. |
| Holes / explicit undefined compaction | 2 | 7.7% | Sparse or `undefined` entries with comparator should move present values before holes/undefined; current dense snapshot preserves `undefined` in front. |
| Default ToString comparator remnants | 2 | 7.7% | Older Sputnik tests still fail string coercion ordering in some mixed primitive/object cases despite ASD. |
| Array-like negative length preservation | 1 | 3.8% | Sort called on non-Array object with negative length should no-op and preserve original `length`; current path writes `length = 0`. |
| Sparse trailing-hole deletion | 1 | 3.8% | `bug_596_2.js` expects trailing hole to be deleted/preserved as a hole, not materialized as an own `undefined` property. |
| Primitive receiver ToObject | 1 | 3.8% | `call-with-primitive.js` fails before sort semantics due BigInt/Symbol receiver handling. |

C4 passes: the precise accessor/prototype side-effect bucket is 19/26 (73.1%), well above the 40% coherence threshold.

### Sampled Failures

Sampled rows:

- `S15.4.4.11_A1.2_T2.js`: sparse array with comparator; current result leaves `x[0]` undefined instead of present value `1`.
- `S15.4.4.11_A1.4_T2.js`: explicit undefined plus comparator; current compaction puts `undefined` before `1`.
- `S15.4.4.11_A2.2_T3.js`: mixed values with custom comparator using `String`; current order fails ToString check.
- `S15.4.4.11_A4_T3.js`: array-like negative length; current sort changes `length` to `0` instead of preserving `-4294967294`.
- `bug_596_2.js`: sparse trailing hole; current sort materializes index `2` as own property.
- `precise-comparefn-throws.js`: expected prototype getter log before comparator throw; current first log is the thrown message, showing accessor observation order is wrong.
- `precise-getter-appends-elements.js`: own getter side effects should append outside sorted range while preserving sort writeback semantics; current hole/accessor ownership differs.
- `precise-prototype-accessors.js`: prototype numeric accessor should be observed and setter invoked during writeback; current path misses getter/setter ordering.
- `precise-setter-sets-predecessor.js`: own setter side effect should mutate predecessor during writeback; current dense writeback collapses that side effect.

### Runtime Cross-Reference

Current implementation:

- `pilots/rusty-js-runtime/derived/src/generated.rs::array_prototype_sort` delegates to `Runtime::array_proto_sort_via`.
- `Runtime::array_proto_sort_via` validates comparator callability, computes `len` with `try_array_length`, snapshots `0..len` with raw `object_get`, sorts a dense `Vec<Value>`, then writes every index back with `object_set` and unconditionally writes `"length"`.

This shape explains most rows:

- `object_get` misses accessor getter abrupts/order and inherited accessor side effects.
- Dense `Vec<Value>` materializes holes as `Value::Undefined`, losing absent-versus-present distinction.
- Dense writeback with `object_set` materializes trailing holes and invokes setters in an order not modeled by spec SortIndexedProperties/writeback.
- Unconditional `length` write changes non-Array array-like receivers whose ToLength was zero.

### C4 Decision

C4 passes for a coherent Phase 3 substrate move. Recommended move is not another ToString-only patch. The next rung should introduce a sort element record layer for Array.prototype.sort:

- collect `SortRecord { index, present, value }` using accessor-aware `HasProperty` + `spec_get` for `0..len`;
- sort only present, non-undefined values through the existing comparator/default compare paths;
- preserve explicit `undefined` and absent holes as distinct tails;
- write back using spec-shaped Set/Delete ordering, deleting trailing absent indices instead of materializing undefined;
- avoid writing `length` for non-Array array-like receivers unless the spec step actually mutates it.

Estimated closure: two substrate rungs. Rung 1 should close the 19-row precise accessor/prototype side-effect bucket plus adjacent sparse deletion rows by replacing dense snapshot/writeback. Rung 2 may be needed for leftover primitive receiver and mixed ToString/comparator edge cases.

## 2026-05-29 - APS-EXT 1 - SortRecord layer and precise bucket closure

### Directive

Helmsman directed R3 to implement the Phase 3 sort-record move proposed by APS-EXT 0: replace dense raw snapshot/writeback in `Runtime::array_proto_sort_via` with accessor-aware collection, present/undefined/absent separation, spec-shaped Set/Delete writeback, and no non-Array array-like length mutation.

### Substrate Move

`array_proto_sort_via` now:

- coerces `this` through the existing Array-method `ToObject` path and captures `len` once;
- collects only present indices using `has_property_with_proxy` plus `spec_get`, so inherited accessors, proxy `has`/`get`, and getter abrupts are observed during the collection phase;
- separates present non-`undefined` values from explicit `undefined`, leaving absent holes out of the sortable item list;
- sorts only present non-`undefined` values through the existing comparator/default string compare path;
- writes sorted values first, then explicit `undefined`, through `reflect_set_via` so inherited/own setters and proxy `set` traps are visible;
- deletes trailing absent slots through `reflect_delete_property_via`, avoiding the old materialization of holes as own `undefined` properties;
- removes the old unconditional `length = len` write, preserving non-Array array-like receivers whose ToLength is zero;
- adds an Array-only length floor after writeback when setter side effects shrink length below the highest written explicit value, mirroring the length growth an indexed Array `Set` should have performed without restoring non-Array length mutation.

This is the intended APS-EXT 1 layer. It does not attempt the primitive BigInt/Symbol receiver gap.

### Measurement

Build:

- `cargo build --release --bin cruft -p cruftless`: PASS.

Targeted post-EPSUA `Array.prototype.sort` 26-row set:

| Probe | Baseline | APS-EXT 1 |
|---|---:|---:|
| All current sort pipeline rows | 0 PASS / 26 FAIL | 25 PASS / 1 FAIL |
| Precise accessor/prototype bucket | 0 PASS / 19 FAIL | 19 PASS / 0 FAIL |
| Adjacent sparse deletion (`bug_596_2.js`) | FAIL | PASS |
| Non-Array negative length (`S15.4.4.11_A4_T3.js`) | FAIL | PASS |

Rule 14 mirror over `built-ins/Array/prototype/sort/*.js`:

| Probe | Baseline `origin/main` | APS-EXT 1 |
|---|---:|---:|
| Full sort directory | 28 PASS / 26 FAIL | 49 PASS / 5 FAIL |

No previously passing sort-directory rows regressed in the baseline comparison.

### Residuals

Five sort-directory rows remain after APS-EXT 1:

- `call-with-primitive.js`: BigInt/Symbol primitive receiver `ToObject` behavior; defer to APS-EXT 2 or a generic primitive-wrapper rung.
- `comparefn-grow.js`, `comparefn-shrink.js`, `comparefn-resizable-buffer.js`, `resizable-buffer-default-comparator.js`: resizable ArrayBuffer / typed-array-adjacent sort behavior outside this Array.prototype.sort precise sparse/accessor rung.

The named 19-row precise bucket and adjacent sparse deletion bucket are closed.
