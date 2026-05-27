# ast-bytecode-missing-throw-typeerror — Trajectory

## ABMT-EXT 0 — founding + exemplar suite + baseline-TBD (2026-05-25)

**Trigger**: Top-10 spawn batch per keeper directive after canonical
full-suite Pin-Art zoom-out. This is rank #7 of the matrix
(622 fails) and is the highest-yield parity lane shape per heuristics §IV.B.

**Apparatus established**:

- `exemplars/exemplars.txt` — 100 stratified-sample paths.
- `exemplars/run-exemplars.sh` — runner.
- `exemplars/pool-size.txt`, `exemplars/family-breakdown.txt` —
  inventory.

**Baseline**: TBD on next run of `exemplars/run-exemplars.sh`. Expected
near 0/100 given the cluster coherence; record value here.

**Status**: ABMT-EXT 0 founding closed. Apparatus operational; first
substrate rung pending exemplar-fail family-marginal inspection per
heuristics §V row-coherence protocol.

## ABMT-EXT 1 — refreshed baseline + addition TypeError strictness (2026-05-27)

**Trigger**: First active pass after locale selection. The initial exemplar
run against the pre-rebuild release binary reported `PASS=0 FAIL=100`; after
rebuilding `target/release/cruft` from current source the true baseline was
`PASS=13 FAIL=87 / 100 (13.0%)`. The already-present source substrate had
closed object-binding `RequireObjectCoercible` cases; stale measurement was
the apparent zero.

**Baseline after rebuild**:

```text
Cluster exemplars: PASS=13 FAIL=87 / 100  (13.0%)
--- top fails by surface family ---
     53 language/expressions
     21 language/statements
      6 language/arguments-object
      3 language/eval-code
      3 language/computed-property-names
      1 language/types
```

**Rung landed**: `Runtime::op_add_rt` now throws `TypeError` after
`ToPrimitive` when either additive operand is `Symbol` on the numeric path,
or when exactly one operand is `BigInt`. Before this rung the operation
fell through to lossy `to_number`, so test262 observed "Expected a TypeError
to be thrown but no exception was thrown at all".

**Focused test262 fixtures**:

- `language/expressions/addition/bigint-errors.js` — PASS
- `language/expressions/addition/coerce-symbol-to-prim-return-prim.js` — PASS
- `language/expressions/addition/order-of-evaluation.js` — PASS

**Regression tests added**:

- `run_golden::addition_symbol_numeric_path_throws_type_error`
- `run_golden::addition_mixed_bigint_number_throws_type_error`
- `run_golden::addition_mixed_bigint_symbol_throws_type_error`
- `destructure::t14_empty_object_param_requires_object_coercible_null`
- `destructure::t15_empty_object_param_requires_object_coercible_undefined`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=16 FAIL=84 / 100  (16.0%)
--- top fails by surface family ---
     50 language/expressions
     21 language/statements
      6 language/arguments-object
      3 language/eval-code
      3 language/computed-property-names
      1 language/types
```

**Residual trajectory**: next coherent rung is the sibling operator
strictness family (`-`, `*`, `/`, `%`, shifts, bitwise) where `ToNumber` and
BigInt mixing still route through permissive helpers. Keep the additive
case as the template: validate in the runtime opcode/helper after any
object-to-primitive step and before the lossy numeric fast-path.

## ABMT-EXT 2 — numeric operator TypeError strictness (2026-05-27)

**Trigger**: ABMT-EXT 1 left the exemplar residual dominated by
`language/expressions` and the next visible family was the sibling numeric
operator lane. Representative failures all had the same shape:
`Symbol` or mixed `BigInt` operands fell through permissive numeric coercion
and no `TypeError` was thrown.

**Rung landed**:

- Added `Runtime::to_num_coerced_strict`, preserving object `valueOf`
  dispatch but rejecting `Symbol` and `BigInt` before lossy `to_number`.
- Added `Runtime::ensure_no_mixed_bigint_or_symbol` and routed `-`, `*`,
  `/`, `%`, `**`, bitwise `& | ^`, `<<`, `>>`, and `>>>` through the strict
  guard/coercion path.
- Preserved direct `BigInt`/`BigInt` semantics for supported operators.
  `>>>` now rejects `BigInt` because unsigned right shift is not defined for
  BigInt.

**Focused test262 fixtures**:

- `language/expressions/subtraction/bigint-errors.js` — PASS
- `language/expressions/multiplication/bigint-and-number.js` — PASS
- `language/expressions/division/bigint-errors.js` — PASS
- `language/expressions/modulus/bigint-errors.js` — PASS
- `language/expressions/bitwise-and/bigint-and-number.js` — PASS
- `language/expressions/bitwise-or/bigint-errors.js` — PASS
- `language/expressions/bitwise-xor/bigint-errors.js` — PASS
- `language/expressions/left-shift/bigint-and-number.js` — PASS
- `language/expressions/right-shift/bigint-errors.js` — PASS
- `language/expressions/unsigned-right-shift/bigint-and-number.js` — PASS

**Regression tests added**:

- `run_golden::numeric_binary_symbol_throws_type_error`
- `run_golden::numeric_binary_mixed_bigint_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=33 FAIL=67 / 100  (33.0%)
--- top fails by surface family ---
     33 language/expressions
     21 language/statements
      6 language/arguments-object
      3 language/eval-code
      3 language/computed-property-names
      1 language/types
```

**Residual trajectory**: expression residual now likely shifts toward
assignment/delete/private-reference/non-extensible-object throws rather than
ordinary numeric operator coercion. Re-read exemplar fail families before
next substrate edit; the operator lane has paid its broad dividend.

## ABMT-EXT 3 — relational Symbol TypeError strictness (2026-05-27)

**Trigger**: Post-operator residual read showed two simple expression
fixtures still in the same coercion family, now through relational comparison:
`3n < Symbol("2")` and `3n > Symbol("2")` returned normal comparison
results instead of throwing.

**Rung landed**: `Op::Lt | Op::Gt | Op::Le | Op::Ge` now rejects either
operand being `Symbol` before calling `abstract_relational_compare`.
This is intentionally narrow: it does not attempt a full relational
ToPrimitive rewrite, only closes the visible ABMT `Symbol` throw-missing
coordinate.

**Focused test262 fixtures**:

- `language/expressions/greater-than/bigint-and-symbol.js` — PASS
- `language/expressions/less-than/bigint-and-symbol.js` — PASS

**Regression test added**:

- `run_golden::relational_symbol_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=35 FAIL=65 / 100  (35.0%)
--- top fails by surface family ---
     31 language/expressions
     21 language/statements
      6 language/arguments-object
      3 language/eval-code
      3 language/computed-property-names
      1 language/types
```

**Residual trajectory**: remaining expression residual is no longer primarily
numeric coercion. Visible next families include strict assignment/delete,
private reference write/brand checks, `instanceof` RHS prototype validation,
and destructuring iterator-close-on-nullish sources.

## ABMT-EXT 4 — strict delete and non-extensible write TypeErrors (2026-05-27)

**Trigger**: Delete-family residuals were split: two were true delete
completion bugs, while `language/expressions/delete/11.4.1-5-a-27-s.js`
was actually strict assignment to a deleted property after
`Object.preventExtensions`.

**Rung landed**:

- `Op::DeleteProp` / `Op::DeleteIndex` now throw TypeError for strict
  deletion of non-configurable own properties instead of returning `false`.
- Member-delete through nullish bases now throws TypeError, while
  non-null primitive bases keep the existing no-op-success behavior.
- `Op::SetProp` / `Op::SetIndex` now reject adding a new own property to a
  non-extensible object in strict mode; sloppy assignment remains a no-op
  with the RHS preserved as the expression value.

**Focused test262 fixtures**:

- `language/expressions/delete/11.4.1-4.a-3-s.js` — PASS
- `language/expressions/delete/member-computed-reference-undefined.js` — PASS
- `language/expressions/delete/11.4.1-5-a-27-s.js` — PASS

**Regression tests added**:

- `run_golden::strict_delete_non_configurable_throws_type_error`
- `run_golden::delete_nullish_property_reference_throws_type_error`
- `run_golden::strict_assign_non_extensible_new_property_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=41 FAIL=59 / 100  (41.0%)
--- top fails by surface family ---
     26 language/expressions
     21 language/statements
      6 language/arguments-object
      3 language/eval-code
      3 language/computed-property-names
```

**Residual trajectory**: strict object-mutation throws were a compact,
high-yield coordinate. Remaining expression residual should be re-read from
the exemplar list before the next edit; likely next lanes are private
reference validation, `instanceof` RHS/prototype validation, and
destructuring/iterator close throw edges.

## ABMT-EXT 10 — generator parameter prologue boundary (2026-05-27)

**Trigger**: Post-ABMT-EXT 9 residuals concentrated in generator and
async-generator destructuring fixtures such as
`language/expressions/generators/dstr/obj-init-null.js`. The decisive
semantic clue: test262 expects destructuring parameter `TypeError`s to be
thrown when the generator function is called, while ordinary generator body
throws must remain deferred until `.next()`.

**Rung landed**:

- Added `FunctionProto::param_prologue_end`, the byte offset immediately
  after compiler-emitted formal parameter default/destructure initialization.
- `Runtime::call_function` now runs only that bytecode prefix at generator
  call time when present, keeps the initialized frame locals, then resumes
  the existing eager-yield collection from `param_prologue_end`.
- This preserves the current v1 eager generator representation while adding
  the missing call/body boundary needed for parameter initialization errors.

**Focused test262 fixtures**:

- `language/expressions/generators/dstr/obj-init-null.js` — PASS
- `language/expressions/generators/dstr/dflt-obj-init-null.js` — PASS
- `language/expressions/generators/dstr/obj-ptrn-prop-ary-value-null.js` — PASS
- `language/expressions/async-generator/dstr/named-dflt-obj-init-null.js` — PASS
- `language/statements/generators/dstr/dflt-obj-init-null.js` — PASS
- `language/statements/try/dstr/obj-init-undefined.js` — still FAIL
  (`catch` destructuring lowering is a separate statement-path residual).

**Regression tests added**:

- `run_golden::generator_object_param_null_throws_at_call_time`
- `run_golden::generator_body_throw_is_deferred_to_next`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=74 FAIL=26 / 100  (74.0%)
--- top fails by surface family ---
      9 language/expressions
      8 language/statements
      6 language/arguments-object
      3 language/eval-code
```

**Residual trajectory**: the generator/async-generator parameter lane paid a
large dividend. Next coherent substrate is `catch` parameter destructuring
(`try/catch` statement lowering missing the object/array destructure
RequireObjectCoercible path), followed by the broader arguments-object and
eval global-definability lanes.

## ABMT-EXT 5 — unary plus BigInt ToNumber TypeError (2026-05-27)

**Trigger**: `language/expressions/unary-plus/bigint-throws.js` remained as
an isolated expression-operator residual after the binary/arithmetic
coercion rungs.

**Rung landed**: `Op::Pos` now routes through `to_num_coerced_strict`,
preserving object `valueOf` coercion while rejecting BigInt and Symbol
operands with TypeError.

**Focused test262 fixture**:

- `language/expressions/unary-plus/bigint-throws.js` — PASS

**Regression test added**:

- `run_golden::unary_plus_bigint_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=42 FAIL=58 / 100  (42.0%)
--- top fails by surface family ---
     25 language/expressions
     21 language/statements
      6 language/arguments-object
      3 language/eval-code
      3 language/computed-property-names
```

**Residual trajectory**: isolated primitive-coercion expression operators
are now largely exhausted in the sample. The remaining compact expression
coordinate is `instanceof` RHS/prototype validation; larger lanes are
private-reference write/brand TypeErrors and destructuring nullish/iterator
close behavior.

## ABMT-EXT 6 — instanceof non-object prototype TypeError (2026-05-27)

**Trigger**: `language/expressions/instanceof/primitive-prototype-with-object.js`
showed that the fallback prototype-chain path treated a constructor whose
`prototype` property is primitive as `false`, where `OrdinaryHasInstance`
requires a TypeError once the left operand is object-valued.

**Rung landed**: `Op::Instanceof` now throws TypeError when the RHS object's
`prototype` property is not an object in the ordinary fallback path.
The existing `Symbol.hasInstance` dispatch and primitive-left false path are
left intact.

**Focused test262 fixture**:

- `language/expressions/instanceof/primitive-prototype-with-object.js` — PASS

**Regression test added**:

- `run_golden::instanceof_non_object_prototype_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=43 FAIL=57 / 100  (43.0%)
--- top fails by surface family ---
     24 language/expressions
     21 language/statements
      6 language/arguments-object
      3 language/eval-code
      3 language/computed-property-names
```

**Residual trajectory**: compact operator/ordinary-object TypeError leaves
are nearly exhausted. Next meaningful lanes are larger: static class
`prototype` definition guards, private-reference brand/write TypeErrors,
global non-definable declaration throws, and destructuring nullish/iterator
close semantics.

## ABMT-EXT 7 — static class prototype definition TypeErrors (2026-05-27)

**Trigger**: The three `computed-property-names/class/static/*-prototype`
fixtures showed that computed static methods/accessors named `"prototype"`
were being installed onto the class constructor, where class definition
evaluation must throw TypeError.

**Rung landed**: class-side `__install_method__` and `__install_accessor__`
now reject key `"prototype"` when the target already has an own
`prototype` property. This catches computed static method, getter, setter,
and generator installs at the runtime helper boundary while leaving instance
members and object-literal accessors outside the guard.

**Focused test262 fixtures**:

- `language/computed-property-names/class/static/getter-prototype.js` — PASS
- `language/computed-property-names/class/static/setter-prototype.js` — PASS
- `language/computed-property-names/class/static/generator-prototype.js` — PASS

**Regression test added**:

- `run_golden::static_computed_prototype_method_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=46 FAIL=54 / 100  (46.0%)
--- top fails by surface family ---
     24 language/expressions
     21 language/statements
      6 language/arguments-object
      3 language/eval-code
```

**Residual trajectory**: computed-property-name static prototype cluster is
closed. The sample is now dominated by destructuring nullish/iterator-close
throws, private-reference brand/write throws, old strict function/arguments
throws, and indirect-eval non-definable global declaration TypeErrors.

## ABMT-EXT 8 — destructuring IteratorClose non-object TypeError (2026-05-27)

**Trigger**: `language/expressions/assignment/dstr/array-empty-iter-close-null.js`
showed the destructuring iterator-close helper calling `return()` but
accepting a primitive/null completion value.

**Rung landed**: `__destr_iter_close` now enforces ECMA-262 IteratorClose
step 9: if the iterator `return` method returns a non-object, throw
TypeError. The existing behavior for absent/nullish/non-callable `return`
remains unchanged.

**Focused test262 fixture**:

- `language/expressions/assignment/dstr/array-empty-iter-close-null.js` — PASS

**Regression test added**:

- `run_golden::array_destructure_iterator_close_non_object_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=49 FAIL=51 / 100  (49.0%)
--- top fails by surface family ---
     22 language/expressions
     20 language/statements
      6 language/arguments-object
      3 language/eval-code
```

**Residual trajectory**: iterator-close non-object is a compact helper-level
coordinate and paid three exemplar passes. Remaining destructuring residuals
seem split between nullish object-binding sources in generated function /
try/class surfaces and iterator-protocol access/step validation.

## ABMT-EXT 9 — deleting well-known Symbol iterator fallback (2026-05-27)

**Trigger**: The `ary-init-iter-get-err-array-prototype` residual family
deletes `Array.prototype[Symbol.iterator]` and then expects array
destructuring to fail during `GetIterator`. Cruft still observed the
iterator because built-in well-known Symbol methods are stored under the
legacy string key `"@@iterator"` while `delete obj[Symbol.iterator]` removed
only the Symbol-key bucket.

**Rung landed**: `Op::DeleteIndex` now honors the well-known Symbol →
string-key transitional fallback. If a Symbol-keyed delete misses the
Symbol bucket but finds an own string-key fallback with the same internal
symbol name, it applies the same configurable/strict delete rules to that
string-key entry.

**Focused test262 fixtures**:

- `language/statements/variable/dstr/ary-init-iter-get-err-array-prototype.js` — PASS
- `language/expressions/function/dstr/ary-init-iter-get-err-array-prototype.js` — PASS

**Regression test added**:

- `run_golden::deleting_array_symbol_iterator_breaks_destructure_iteration`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=57 FAIL=43 / 100  (57.0%)
--- top fails by surface family ---
     19 language/expressions
     15 language/statements
      6 language/arguments-object
      3 language/eval-code
```

**Residual trajectory**: this was the largest single ABMT payoff since the
numeric-operator lane, because one Symbol/delete substrate mismatch blocked
many generated destructuring surfaces. Remaining destructuring rows now lean
toward nullish nested object-binding values and async-generator/class method
parameter initialization, rather than missing `Array.prototype[Symbol.iterator]`
delete semantics.
