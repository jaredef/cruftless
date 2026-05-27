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

## ABMT-EXT 11 — catch parameter destructuring TypeErrors (2026-05-27)

**Trigger**: ABMT-EXT 10 left the `try/dstr` residual as the nearest
destructuring sibling. Fixtures such as
`language/statements/try/dstr/obj-init-undefined.js` expected a TypeError
when a thrown nullish value was bound through a catch object pattern, but
Cruft parsed patterned catch params as absent optional-catch-bindings.

**Rung landed**:

- `CatchClause::param` now preserves `Option<BindingPattern>` instead of
  identifier-only state.
- The parser routes `catch (...)` through `parse_binding_target`, retaining
  object and array binding patterns.
- The compiler stores identifier catch params directly, and for object/array
  catch params spills the thrown value to a hidden source slot before calling
  the shared `emit_destructure` binding-pattern path.

**Focused test262 fixtures**:

- `language/statements/try/dstr/ary-ptrn-elem-obj-val-null.js` — PASS
- `language/statements/try/dstr/obj-init-undefined.js` — PASS
- `language/statements/try/dstr/obj-ptrn-prop-obj-value-undef.js` — PASS

**Regression tests added**:

- `statement_grammar::try_catch_object_pattern_binding`
- `run_golden::catch_object_pattern_null_throws_type_error`
- `run_golden::catch_nested_object_pattern_undefined_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=77 FAIL=23 / 100  (77.0%)
--- top fails by surface family ---
      9 language/expressions
      6 language/arguments-object
      5 language/statements
      3 language/eval-code
```

**Residual trajectory**: destructuring nullish/catch surfaces in the sample
are closed. Remaining coherent substrates are now (1) private-reference
brand/readonly/write TypeErrors, (2) arguments-object strict and mapped
delete semantics, and (3) indirect-eval non-definable global declarations.

## ABMT-EXT 12 — strict function caller and named-expression self binding (2026-05-27)

**Trigger**: The post-catch residual left an old strict-function cluster
under `language/statements/function/*caller*` plus a named function
expression self-name reassignment fixture. The shared coordinate is
function-object restricted properties and immutable function-internal
bindings: strict functions must expose `%ThrowTypeError%` on `caller`, and a
named function expression's internal self binding cannot be reassigned from a
nested arrow.

**Rung landed**:

- Function prototypes now mark named function-expression self slots as
  `Const`, matching ECMA-262's immutable function-name binding.
- Identifier assignment lowering now resolves captured upvalues explicitly
  and throws TypeError when the captured source is a const local, closing the
  nested-arrow path that bypassed the local-only const guard.
- Strict non-arrow function closures install a non-configurable `caller`
  accessor whose getter and setter throw TypeError.

**Focused test262 fixtures**:

- `language/statements/function/13.2-19-b-3gs.js` — PASS
- `language/statements/function/13.2-21-s.js` — PASS
- `language/statements/function/13.2-5-s.js` — PASS
- `language/expressions/function/named-strict-error-reassign-fn-name-in-body-in-arrow.js` — PASS

**Regression tests added**:

- `run_golden::strict_function_caller_read_throws_type_error`
- `run_golden::strict_function_caller_write_throws_type_error`
- `run_golden::named_function_expression_self_binding_is_const`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=81 FAIL=19 / 100  (81.0%)
--- top fails by surface family ---
      8 language/expressions
      6 language/arguments-object
      3 language/eval-code
      2 language/statements
```

**Residual trajectory**: the strict-function surface is mostly closed in
the sample. The frontier is now narrower and deeper: private-reference
brand/write/read-only TypeErrors, the arguments-object strict/mapped delete
semantics, and indirect-eval global declaration definability.

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

## ABMT-EXT 13 — frozen tagged-template objects (2026-05-27)

**Trigger**: After ABMT-EXT 12, the residual exemplar
`language/expressions/tagged-template/template-object-frozen-strict.js`
was a singleton-shaped miss: tagged templates passed an ordinary extensible
array as the first argument, so strict writes to the template object and
its `.raw` object did not throw.

**Rung landed**: tagged-template lowering now wraps the cooked strings
array in the hidden engine helper `__template_object__`. The helper creates
the `.raw` twin, freezes `.raw`, installs `.raw` as a non-writable,
non-enumerable, non-configurable property, then freezes the template object
before user code observes it. The parser still preserves only cooked
strings; for the no-substitution fixture closed here, cooked and raw are
the same string coordinate.

**Focused test262 fixture**:

- `language/expressions/tagged-template/template-object-frozen-strict.js` — PASS

**Regression test added**:

- `run_golden::tagged_template_object_and_raw_are_frozen`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=82 FAIL=18 / 100  (82.0%)
--- top fails by surface family ---
      7 language/expressions
      6 language/arguments-object
      3 language/eval-code
      2 language/statements
```

**Residual trajectory**: the tagged-template singleton is closed. The
remaining expression residuals now lean private-reference brand/write
semantics; the other major lanes are mapped arguments-object invariants and
indirect-eval non-definable global declaration TypeErrors.

## ABMT-EXT 14 — private method/getter readonly PutValue (2026-05-27)

**Trigger**: The post-ABMT-EXT 13 private-reference residual split into
two mechanisms. Five fixtures expected `PrivateSet` to reject writes to
private methods or getter-only private accessors, but cruftless fell back
to creating/updating a private field when no private setter was found.

**Rung landed**: private methods now carry a small readonly marker in the
object's private-element storage. The private `SetProp` path now throws
TypeError when the target is a private method or a getter-only private
accessor, while ordinary private fields remain writable. Because current
private-method lookup walks the prototype chain, the marker check also
walks that chain.

**Focused test262 fixtures**:

- `language/expressions/compound-assignment/left-hand-side-private-reference-method-sub.js` — PASS
- `language/expressions/compound-assignment/left-hand-side-private-reference-readonly-accessor-property-exp.js` — PASS
- `language/expressions/compound-assignment/left-hand-side-private-reference-readonly-accessor-property-srshift.js` — PASS
- `language/expressions/logical-assignment/left-hand-side-private-reference-readonly-accessor-property-nullish.js` — PASS
- `language/expressions/logical-assignment/left-hand-side-private-reference-readonly-accessor-property-or.js` — PASS

**Regression tests added**:

- `run_golden::private_method_compound_assignment_throws_type_error`
- `run_golden::private_getter_without_setter_assignment_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=87 FAIL=13 / 100  (87.0%)
--- top fails by surface family ---
      6 language/arguments-object
      3 language/eval-code
      2 language/statements
      2 language/expressions
```

**Residual trajectory**: private readonly PutValue is closed. The remaining
private failures are separate brand/field-installation coordinates:
multiple-evaluation private static brand checks and private field set before
the field entry exists. The largest unresolved lanes are now arguments-object
invariants and indirect-eval global-definability TypeErrors.

## ABMT-EXT 15 — private field declaration vs assignment split (2026-05-27)

**Trigger**: `language/statements/class/elements/privatefieldset-typeerror-1.js`
expects `this.#x = 1` to throw when it appears before the `#x` private
field declaration has installed an entry. Cruftless used the same `SetProp`
path for both private field declarations and ordinary private assignments,
so the assignment silently created the missing private slot.

**Rung landed**: private field declarations now lower through the hidden
engine helper `__init_private_field__(target, key, value)`, while ordinary
private `SetProp` requires an existing private element unless it dispatches
to a private setter. This preserves private declaration initialization and
makes user-code assignment to a missing private field throw TypeError.
Static private fields use the same helper so the declaration/assignment
distinction is shared across instance and static elements.

**Focused test262 fixture**:

- `language/statements/class/elements/privatefieldset-typeerror-1.js` — PASS

**Regression test added**:

- `run_golden::private_field_set_before_declaration_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=88 FAIL=12 / 100  (88.0%)
--- top fails by surface family ---
      6 language/arguments-object
      3 language/eval-code
      2 language/expressions
      1 language/statements
```

**Residual trajectory**: private field installation order is closed. The
remaining private failures are brand identity / nested private accessor
coordinates rather than missing-entry writes. Arguments-object invariants
are now the largest unresolved ABMT family.

## ABMT-EXT 16 — strict arguments `callee` poison pill (2026-05-27)

**Trigger**: three strict-mode arguments-object fixtures expected reads or
writes of `arguments.callee` to invoke the `%ThrowTypeError%` poison pill,
but cruftless allocated `arguments` as a plain Array-like object with indexed
properties and no strict `callee` accessor.

**Rung landed**: strict function calls now allocate their arguments object
with a non-enumerable, non-configurable `callee` accessor whose getter and
setter are the runtime `%ThrowTypeError%` native. Sloppy-mode arguments are
left untouched; this rung is only the strict unmapped poison-pill coordinate.

**Focused test262 fixtures**:

- `language/arguments-object/10.6-13-c-1-s.js` — PASS
- `language/arguments-object/10.6-14-c-4-s.js` — PASS
- `language/arguments-object/10.6-2gs.js` — PASS

**Regression tests added**:

- `run_golden::strict_arguments_callee_read_throws_type_error`
- `run_golden::strict_arguments_callee_write_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=91 FAIL=9 / 100  (91.0%)
--- top fails by surface family ---
      3 language/eval-code
      3 language/arguments-object
      2 language/expressions
      1 language/statements
```

**Residual trajectory**: strict arguments `callee` is closed. The remaining
arguments-object failures are mapped-arguments parameter-map semantics around
non-configurable deletes, not the strict poison-pill accessor. The other
visible families remain indirect-eval global-definability TypeErrors and
private-name brand identity / shadowing.

## ABMT-EXT 17 — indirect eval global declaration definability (2026-05-27)

**Trigger**: three indirect-eval fixtures expected `EvalDeclarationInstantiation`
to reject global declarations that cannot be defined: `function NaN() {}`,
`function* NaN() {}`, and `var unlikelyVariableName` after the global object
has been made non-extensible. Cruftless evaluated the eval source without a
global-declaration preflight, so the declarations were silently accepted.

**Rung landed**: the eval intrinsic now parses eval source as a Script-shaped
module before execution and performs a narrow global-declaration-instantiation
guard over top-level `var`, function, and generator declarations. The guard
uses the actual `globalThis` object descriptor/extensibility state:
`CanDeclareGlobalVar` accepts existing own properties or extensible globals,
while `CanDeclareGlobalFunction` rejects non-configurable immutable globals
such as `NaN`. The global value-property descriptors for `Infinity`, `NaN`,
and `undefined` are now installed as non-writable, non-enumerable, and
non-configurable on `globalThis`.

**Focused test262 fixtures**:

- `language/eval-code/indirect/non-definable-global-function.js` — PASS
- `language/eval-code/indirect/non-definable-global-generator.js` — PASS
- `language/eval-code/indirect/non-definable-global-var.js` — PASS

**Regression tests added**:

- `run_golden::indirect_eval_non_definable_global_function_throws_type_error`
- `run_golden::indirect_eval_non_definable_global_generator_throws_type_error`
- `run_golden::indirect_eval_non_extensible_global_var_throws_type_error`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=94 FAIL=6 / 100  (94.0%)
--- top fails by surface family ---
      3 language/arguments-object
      2 language/expressions
      1 language/statements
```

**Residual trajectory**: eval global definability is closed for the visible
ABMT fixtures. The remaining six split into mapped-arguments parameter-map
semantics and private-name identity / shadowing. Both are deeper substrate
coordinates than missing TypeError checks: mapped arguments need live
parameter-index coupling, and private-name failures need per-evaluation
private-name identity rather than string-keyed private slots.

## ABMT-EXT 18 — mapped arguments parameter-map cells (2026-05-27)

**Trigger**: three mapped-arguments fixtures expected a failed strict delete
of non-configurable `arguments[0]` to preserve the mapping between formal
parameter `a` and `arguments[0]`. Cruftless copied argument values into an
Array-like object at call entry, so later writes through `a`,
`Object.defineProperty(arguments, "0", {value: ...})`, or `arguments[0] = ...`
did not update the other side.

**Rung landed**: sloppy function calls now allocate mapped arguments objects
whose mapped indices point at the same `UpvalueCell` used by the formal
parameter local. `object_get`, `object_set`, and `Object.defineProperty`
consult/update that parameter map for mapped string indices. Strict
arguments objects keep the unmapped poison-pill shape from ABMT-EXT 16.

**Focused test262 fixtures**:

- `language/arguments-object/mapped/mapped-arguments-nonconfigurable-strict-delete-2.js` — PASS
- `language/arguments-object/mapped/mapped-arguments-nonconfigurable-strict-delete-3.js` — PASS
- `language/arguments-object/mapped/mapped-arguments-nonconfigurable-strict-delete-4.js` — PASS

**Regression test added**:

- `run_golden::mapped_arguments_parameter_and_index_share_binding`

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=97 FAIL=3 / 100  (97.0%)
--- top fails by surface family ---
      2 language/expressions
      1 language/statements
```

**Residual trajectory**: arguments-object invariants are closed for the ABMT
sample. The remaining three are all private-name identity / shadowing:
multiple-evaluation static private methods/setters and nested private
accessor shadowing. This is no longer a missing-throw patch lane; it requires
private names to become per-evaluation identities instead of shared string
keys.

## ABMT-EXT 19 — class private home branding bridge (2026-05-27)

**Trigger**: the final three ABMT exemplars expected same-spelled private
names from distinct class evaluations/nested class bodies to be distinct
PrivateNames. Cruftless stored private elements by raw string name (`#m`),
so `C1.access.call(C2)` and nested `#m` accessor shadowing collapsed onto
one textual key and incorrectly succeeded instead of throwing TypeError.

**Rung landed**: class method/accessor install helpers now attach a
`private_home` object identity to installed function objects and brand
private methods/accessors by the installing target object. Function frames
thread that home into private access resolution. Private reads/writes now
prefer the branded key for the active home and fall back to the legacy raw
key so existing private-field storage remains stable while the method and
accessor identity substrate becomes per-class-evaluation. Object tracing also
follows the home edge.

**Focused test262 fixtures**:

- `language/expressions/class/private-static-method-brand-check-multiple-evaluations-of-class-function-ctor.js` — PASS
- `language/expressions/class/private-static-setter-multiple-evaluations-of-class-function-ctor.js` — PASS
- `language/statements/class/elements/private-getter-shadowed-by-setter-on-nested-class.js` — PASS

**Verification**:

- `cargo check -p rusty-js-runtime` — PASS
- `cargo build --release -p cruftless` — PASS
- `cargo test -p rusty-js-runtime --test classes -- --nocapture` — PASS
- `cargo test -p rusty-js-runtime --test class_accessors -- --nocapture` — PASS
- `cargo test -p rusty-js-runtime --test class_fields -- --nocapture` — PASS
- `CRUFTLESS_TEST262_JOBS=1 pilots/ast-bytecode-missing-throw-typeerror/exemplars/run-exemplars.sh` — PASS

**Post-rung exemplar yield**:

```text
Cluster exemplars: PASS=100 FAIL=0 / 100  (100.0%)
--- top fails by surface family ---
      1 /
```

**Residual trajectory**: the ABMT exemplar cluster is closed. The private
home bridge is intentionally narrower than a finished ECMA PrivateName model:
instance private fields still use the raw-key substrate with branded lookup
fallback. That is coherent as a no-regression bridge, but the full parity DAG
eventually wants class-evaluation PrivateName allocation as a first-class
runtime object/identifier rather than string branding.
