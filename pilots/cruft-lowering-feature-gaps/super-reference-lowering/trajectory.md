# super-reference-lowering — Trajectory

## SRL-EXT 0 — nested locale spawned from CLFG baseline (2026-05-28)

**Trigger**: CLFG-EXT 1 confirmed 22/32 parent exemplars and 96/113 matrix
rows are `super` compile diagnostics. Collision check found no active locale
owning this exact AST-to-bytecode surface.

**Existing-locale check**:

- `for-head-this-super-target/` is parser-only and closed; it owns invalid
  `this`/`super` in for-in/for-of heads, not `super` member/call lowering.
- Candidate `object-literal-computed-property-semantics` mentions object
  methods, home-object, and `super`, but is deferred/sample-needed and broader
  than the current compile-diagnostic cluster.
- Candidate `class-lowering-residual-repartition` is audit-first and broad;
  this child narrows only the explicit `compile: super...` residue.

**Artifacts**:

- `seed.md`
- `trajectory.md`
- `exemplars/exemplars.txt`
- `exemplars/run-exemplars.sh`

**Next**: run the child exemplar suite and classify the 22 failures into
object-literal home-object, direct-eval context capture, invalid-super
early-error routing, and no-extends/derived-constructor context checks.

## SRL-EXT 1 — child baseline classification (2026-05-28)

**Probe**:

```sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  pilots/cruft-lowering-feature-gaps/super-reference-lowering/exemplars/run-exemplars.sh
```

**Result**:

```text
CLFG exemplars: PASS=0 FAIL=22 SKIP=0 NOJSON=0 / 22
```

The runner summary still says `CLFG` because the child runner delegates to the
parent harness with a child-specific path list.

**Failure distribution by compiler diagnostic**:

| Exemplars | Reason |
|---:|---|
| 10 | `compile: super reference outside of a class` |
| 6 | `compile: bare \`super\` reference is only valid as \`super(...)\` or \`super.method(...)\`` |
| 4 | `compile: super reference in a class with no \`extends\` clause` |
| 2 | `compile: super(...) outside of a class` |

**Semantic split**:

- **Object-literal HomeObject**: three `computed-property-names/object/*`
  rows. These are legal object-method/accessor `super` forms and should not
  require a class frame.
- **Direct-eval context capture**: direct eval in class methods/fields and
  derived constructors currently loses the surrounding super context before
  bytecode compilation.
- **Delete/bare-super early-error routing**: `delete super.x` rows currently
  reach `Expr::Super` as a bare expression in bytecode lowering. These likely
  want parser/static-semantics rejection or a narrow compile-time SyntaxError
  path, not generic lowering support for bare `super`.
- **Super property assignment/update**: compound assignment and increment rows
  need `super[key]` reference lowering, not bare-super rejection.
- **No-extends class super property**: base-class `super.prop` rows currently
  reject during bytecode lowering because no super prototype slot exists. The
  expected behavior likely belongs at runtime (`GetSuperBase` null path) rather
  than at compile rejection.

**Next**:

Start with object-literal HomeObject or super property assignment/update. Avoid
the direct-eval context rows until the active eval-environment work is settled,
and avoid delete/bare-super until the owner is checked against parser/static
semantics.

## SRL-EXT 2 — object-literal HomeObject bridge for `super` (2026-05-28)

**Target selected**: the three object-literal rows from SRL-EXT 1:

- `language/computed-property-names/object/accessor/getter-super.js`
- `language/computed-property-names/object/accessor/setter-super.js`
- `language/computed-property-names/object/method/super.js`

All three are legal object method/accessor forms where `super.m()` resolves
through the object literal's live `[[Prototype]]` after
`Object.setPrototypeOf(object, proto)`.

**Substrate move**:

- Added an object-literal HomeObject local in
  `rusty-js-bytecode::compiler` when lowering `Expr::Object`.
- Method-definition object properties now compile their function bodies under
  a `ClassFrame`-like `super_home_name` context. That context lowers
  `super.x` through a new runtime helper, `__super_get_home(this, home, key)`.
- Object-literal methods install via `__install_method_obj__`, preserving
  ordinary object-method descriptor shape (`writable:true`,
  `enumerable:true`, `configurable:true`) while recording the literal object
  as the closure home.
- Object-literal accessors installed via `__install_accessor_obj__` now also
  record the literal object as home.
- `__super_get_home` reads `home.[[Prototype]]` at call time, so later
  `Object.setPrototypeOf` calls are reflected.

**Verification**:

```text
cargo check -p rusty-js-bytecode: PASS (existing warnings)
cargo check -p rusty-js-runtime: PASS (existing warnings)
cargo build --bin cruft -p cruftless: PASS (existing warnings)
```

Focused object-literal subset with the freshly built debug binary:

```text
CLFG exemplars: PASS=3 FAIL=0 SKIP=0 NOJSON=0 / 3
```

Full child suite with the freshly built debug binary:

```text
CLFG exemplars: PASS=3 FAIL=19 SKIP=0 NOJSON=0 / 22
```

Debug diff-prod run:

```text
total: 112
PASS:  62
FAIL:  50
```

**Residual**:

- Direct-eval context capture remains `compile: super reference outside of a
  class`.
- Delete/bare-super and super property assignment/update remain
  `compile: bare \`super\` reference ...`.
- Base-class no-extends rows remain
  `compile: super reference in a class with no \`extends\` clause`.
- Derived-constructor direct-eval `super()` rows remain
  `compile: super(...) outside of a class`.

**Next**: choose between super property assignment/update and no-extends
runtime behavior. Keep direct-eval rows deferred until the active eval
environment arc settles.

## SRL-EXT 3 — super PutValue base/key ordering for object methods (2026-05-28)

**Target selected**: the two object-method `super[key]` PutValue ordering rows:

- `language/expressions/super/prop-expr-getsuperbase-before-topropertykey-putvalue-compound-assign.js`
- `language/expressions/super/prop-expr-getsuperbase-before-topropertykey-putvalue-increment.js`

Both rows rely on `GetSuperBase` happening before `ToPropertyKey`: the computed
key's `toString()` mutates the object method HomeObject's prototype, but the
super reference must still read and write through the originally captured
super base.

**Substrate move**:

- Added compiler branches for object-method `super` compound assignment and
  update expressions. The lowering captures `home.[[Prototype]]` into a temp
  before evaluating/coercing the computed key, then reads/writes through that
  captured base.
- Added runtime helpers:
  - `__super_base_home(home)` captures the live `HomeObject.[[Prototype]]`.
  - `__super_get_base(this, base, key)` performs side-effectful key coercion
    and getter-aware property lookup with the original receiver.
  - `__super_set(base, this, key, value)` performs side-effectful key coercion
    and writes data properties through the receiver, while preserving setter
    dispatch when the super chain resolves to an accessor setter.
- Routed the helpers through the engine-helper allowlist.

**Verification**:

```text
cargo check -p rusty-js-bytecode: PASS (existing warnings)
cargo check -p rusty-js-runtime: PASS (existing warnings)
cargo build --bin cruft -p cruftless: PASS (existing warnings)
```

Focused PutValue subset with the freshly built debug binary:

```text
CLFG exemplars: PASS=2 FAIL=0 SKIP=0 NOJSON=0 / 2
```

Full child suite with the freshly built debug binary:

```text
CLFG exemplars: PASS=5 FAIL=17 SKIP=0 NOJSON=0 / 22
```

Debug diff-prod run:

```text
total: 112
PASS:  62
FAIL:  50
```

**Residual**:

- Direct-eval context capture remains `compile: super reference outside of a
  class`.
- Delete/bare-super rows remain `compile: bare \`super\` reference ...`.
- Base-class no-extends rows remain
  `compile: super reference in a class with no \`extends\` clause`.
- Derived-constructor direct-eval `super()` rows remain
  `compile: super(...) outside of a class`.

**Next**: no-extends runtime behavior is now the lowest-collision remaining
cluster. Keep direct-eval rows deferred behind the eval-environment arc, and
check delete-super against static-semantics ownership before editing.

## SRL-EXT 4 — no-extends SuperProperty base fallback (2026-05-28)

**Target selected**: the four base-class/no-extends SuperProperty rows:

- `language/expressions/optional-chaining/member-expression.js`
- `language/expressions/super/prop-expr-cls-err.js`
- `language/expressions/super/prop-expr-cls-key-err.js`
- `language/statements/class/syntax/class-body-method-definition-super-property.js`

These rows show that `super.prop` and `super[key]` are valid inside a class
body even when the class has no `extends` clause. Instance methods and base
constructors must read through `Object.prototype`; static methods read through
`Function.prototype`. Computed keys must still execute and undergo
side-effectful `ToPropertyKey` before any lookup result is observed.

**Substrate move**:

- Removed the compiler rejection for class frames with no recorded
  `super_proto_name`/`super_ctor_name` when lowering SuperProperty reads.
- Lowered missing instance/base-constructor super bases to
  `Object.prototype`, and missing static super bases to `Function.prototype`.
- Routed `__super_get` keys through side-effectful string coercion before
  property-key lookup, so `super[badToString]` observes the required key
  coercion error.

**Verification**:

```text
cargo check -p rusty-js-bytecode: PASS (existing warnings)
cargo build --bin cruft -p cruftless: PASS (existing warnings)
```

Focused no-extends subset with the freshly built debug binary:

```text
CLFG exemplars: PASS=4 FAIL=0 SKIP=0 NOJSON=0 / 4
```

Full child suite with the freshly built debug binary:

```text
CLFG exemplars: PASS=9 FAIL=13 SKIP=0 NOJSON=0 / 22
```

Debug diff-prod run:

```text
total: 112
PASS:  61
FAIL:  51
```

**Residual**:

- Direct-eval context capture remains `compile: super reference outside of a
  class`.
- Delete/bare-super rows remain `compile: bare \`super\` reference ...`.
- Derived-constructor direct-eval `super()` rows remain
  `compile: super(...) outside of a class`.

**Next**: static-semantics audit for delete-super routing is now the
lowest-collision local move. Keep direct-eval rows behind the eval-environment
arc.

## SRL-EXT 5 — delete SuperReference runtime throw (2026-05-28)

**Target selected**: the four `delete super` rows:

- `language/expressions/delete/super-property-method.js`
- `language/expressions/delete/super-property-null-base.js`
- `language/expressions/delete/super-property-uninitialized-this.js`
- `language/expressions/delete/super-property.js`

The tests confirm this is not a parse-time or blanket compile-time rejection.
The delete operator first evaluates the SuperProperty reference enough to
observe `this` binding failures and computed-key effects, then throws
`ReferenceError` because deleting a SuperReference is forbidden.

**Substrate move**:

- Added a `delete super.property` / `delete super[expr]` branch in unary
  delete lowering before the ordinary member-delete path.
- The branch emits helper call setup, then `PushThis`, then property-key
  evaluation. This preserves the order where an uninitialized derived
  constructor `this` throws before the computed key expression runs.
- Added hidden runtime helper `__super_delete`, routed through the
  engine-helper allowlist, which throws the required `ReferenceError`.

**Verification**:

```text
cargo build --bin cruft -p cruftless: PASS (existing warnings)
```

Focused delete-super subset with the freshly built debug binary:

```text
CLFG exemplars: PASS=4 FAIL=0 SKIP=0 NOJSON=0 / 4
```

Full child suite with the freshly built debug binary:

```text
CLFG exemplars: PASS=13 FAIL=9 SKIP=0 NOJSON=0 / 22
```

**Residual**:

- Direct-eval context capture remains `compile: super reference outside of a
  class`.
- Derived-constructor direct-eval `super()` rows remain
  `compile: super(...) outside of a class`.

**Next**: the remaining SRL rows are all eval-context capture. Either hand off
to the active eval-environment arc or spawn a direct-eval `super` nested rung
only after reading that arc's current settlement.
