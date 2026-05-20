# arktype Deep-Trace — Layered Class-Semantics Divergences

A live working document of the trace into arktype's loading failure under cruftless. The session began with the top500 parity sweep classifying arktype as "rb-strict / bun-tolerant" in the §XIV-candidate bucket (the assumption being that Bun absorbs a deviation cruftless rejects). The trace falsified that classification: arktype is not a deviation-pipeline target. It is a stress test that surfaces a *layered set of ES2022 class-semantics spec divergences* in cruftless, where each layer hides the next.

This doc records what has been found so far, what the methodology has been, and the open frontier. Per Doc 730's §XII targeting heuristic, the resolution path is becoming its own diagnostic: each substrate move makes one more stage of the chain legible, and the next divergence surfaces at the point the prior one was masking. Nothing is a black box; the implicit constraints get named as they are encountered.

Date opened: 2026-05-20. Engagement context: EXT 20 close + §XIII alphabet promotion stretch.

## Methodology

For each failure, the trace proceeds:

1. **Run cruftless against arktype's parity-probe**, capture the structured error message — class, message, file:line, in-method/in-call call chain.
2. **Read the source at file:line** to identify the runtime expression that threw.
3. **Build a minimal repro** that reproduces the throw outside arktype. If the repro is small enough, it goes in `/tmp` and becomes a regression test on the fix.
4. **Compare cruftless's behavior to Bun's on the same repro.** Disagreements localize the spec divergence; agreements rule it out.
5. **Read the spec section** the divergence touches. Name the spec contract cruftless violates.
6. **Pick a substrate strategy:** §XII coercion/dispatch lift, §XIII alphabet promotion, or §XIV deviation primitive with protected_invariants. Default toward §XIII unless the divergence is a genuine ecosystem tolerance.
7. **Land the substrate, run parity-fast + the targeted repro, then re-run arktype**. The next wall (if any) becomes visible.

This is the Doc 730 §XII pipeline operating with the trace itself as the diagnostic. Each pass refines the resolution path.

## Wall 1 — class-field-after-super (RESOLVED)

**Surface:** `Cannot read property 'replace' of undefined (receiver='metaJson') @ @ark/schema/out/node.js:426:50`. compileMeta is called from a class field initializer with `this.metaJson` undefined.

**Source:** node.js:148:
```js
compiledMeta = compileMeta(this.metaJson);
```

**Spec:** ECMA-262 §15.7.13 step 11. In a derived constructor, `this` is uninitialized until super() returns; field initializers reference `this` and per InitializeInstanceElements they run as part of SuperCall *after* the parent constructor returns. The pre-substrate cruftless prepended field-init statements to the entire constructor body — so `this.field = value` writes landed on the pre-allocated `this` which super() then replaced (Callable-style return-of-non-this).

**Spec divergence:** field-init timing. Field inits prepended instead of post-super-inserted.

**Substrate:** `Ω.5.P03.E2.class-field-after-super` (commit `3ceab019`). compile_class now walks the explicit ctor body for the first top-level statement containing a super() call and inserts field inits immediately after it.

**Recovers:** 2-level Callable-style class hierarchies (Derived extends Callable) where the parent constructor returns an Object.assign'd bound function.

## Wall 2 — super-new-target (RESOLVED, co-evolutionary with Wall 1)

**Surface:** Wall 1's fix recovered the 2-level case but the 3-level case (Outer → Derived → Base where Base does Callable's return-of-bound-fn) still showed `d.metaJson: undefined`.

**Diagnosis:** super(...) was lowered as `CallMethod` with no construct semantics. The base ctor's frame had `nt_for_this_call=None`, so PushNewTarget inside the parent read undefined (where the spec says new.target = the active newTarget of the calling derived ctor per ECMA §10.2.1.3 SuperCall step 4), AND `call_function`'s implicit-return-this branch — gated on `nt_for_this_call.is_some()` — didn't fire. The parent's frame.this_value (rebound by SetThis under Callable's return-of-non-this) was discarded; the parent returned `Value::Undefined`; the derived's SetThis after CallMethod saw Undefined (not Object) and left this_value unchanged.

**Why 2-level worked by accident:** Op::New's outer construct fallback masked the inner gap. For 2-level chains, the OUTER `new` saw Undefined returned from the derived ctor, and its own implicit-return-this fallback returned `inner.this_value` — which after SetThis was the bound fn. So 2-level chains accidentally rebinded at the outermost step. 3-level chains broke because the *inner* super() lost the rebinding before it reached the outer super()'s SetThis.

**Spec divergence:** new.target not propagated through super-call dispatch; implicit-return-this not firing in derived ctors invoked via super.

**Substrate:** `Ω.5.P03.E2.super-new-target` (commit `72f2bf47`). Added `Op::PropagateNewTarget` (opcode 0x79) that reads the current frame's new.target and writes it into pending_new_target so the next dispatch sees construct semantics. compile_super_call emits it immediately before CallMethod (non-spread) and before `__super_apply` (spread). `__super_apply` is a new runtime helper mirroring `__apply` but re-emitting current_new_target into pending before the inner call_function — required because `__apply` itself consumes pending on its own frame entry.

**Recovers:** N-level Callable-style class chains. Co-evolves with Wall 1: either substrate alone recovers 2-level chains; both together recover N-level chains.

## Wall 3 — is-spec-object (RESOLVED)

**Surface:** `Cannot convert object to primitive value @ @ark/schema/out/roots/unit.js:52:14 (in-method='compiledValue')`.

**Diagnosis via instrumentation:** patched unit.js to log `compiledValue` before the template literal coercion. Result: `compiledValue: object ctor: null toString: object json.unit: null`. The value is JS null (typeof null === "object") being template-literal-coerced. cruftless threw "Cannot convert object to primitive value"; Bun returns the string `"null"`.

**Minimal repro:**
```js
`${null}`
// bun: "null"   cruftless: TypeError
```

**Source of the throw:** `pilots/rusty-js-runtime/derived/src/generated.rs::to_primitive`. The IR section at `pilots/rusty-js-ir/derived/src/sections/to_primitive.rs:26-42` discriminated "Type(V) is Object" via `Expr::TypeOf(value)` compared against the string literals `"object"` and `"function"`. That captures spec-Object correctly for ordinary objects and functions (the EXT 72b dual where typeof "function" but spec Type Object) but collapses **spec-Null into the spec-Object branch** because typeof null === "object" while Type(null) === Null per ECMA §6.1. ToPrimitive(null) failed the step 1 short-circuit, walked the @@toPrimitive / toString / valueOf chain (none defined on null), and reached step 6's TypeError.

**Spec divergence:** runtime-typeof and spec-Type collapsed at the IR alphabet level. The IR has no primitive that distinguishes "typeof tag" from "Type abstract operation."

**Substrate:** `Ω.5.P04.E1.is-spec-object` (commit `d1ab22cb`). Doc 730 §XIII upward-additive alphabet promotion: added `Expr::IsSpecObject(Box<Expr>)` to the rusty-js-ir alphabet, lowering to `matches!(v, Value::Object(_))`. Rewrote to_primitive.rs's four collapse-sites (§7.1.1 step 1, §7.1.1 step 2.b.ii, §7.1.1.1 step 4.m1.check, step 5.m2.check) to use IsSpecObject. Five intermediate t/t1/t2 let-bindings and the fn_check sub-steps eliminated as scaffolding the alphabet now subsumes.

**Recovers:** any code that does ToPrimitive(null) or ToPrimitive(an-object-whose-method-returns-null). Including template literals like `${null}`.

## Wall 4 — class-method-enumerability + prototype-as-this (OPEN)

**Surface:** `Cannot read property 'parseDefinition' of undefined (receiver='$') @ @ark/schema/out/node.js:216:39 (in-method='equals') (in-call='$') (in-call='$') (in-method='discriminate')`.

**Source:** node.js:216:
```js
equals(r) {
    const rNode = isNode(r) ? r : this.$.parseDefinition(r);
```

**Diagnosis via instrumentation:** patched equals() to log `this`'s shape on failure. Result:
- `typeof this.$`: undefined
- `'$' in this`: false
- `this.kind`: undefined
- `Object.keys(this)`: `constructor,shallowMorphs,createRootApply,cacheGetter,description,references,traverse,in,rawIn,out,rawOut,getIo,toJSON,toString,equals,ifEquals,hasKind,assertHasKind,hasKindIn,assertHasKindIn,isBasis,isConstraint,isStructural,isRefinement,isRoot,isUnknown,isNever,hasUnit,hasOpenIntersection,nestableExpression,select,_select,transform,_createTransformContext,_transform,configureReferences`

The keys are *method names of BaseNode (or a subclass) plus `constructor`*. This is the shape of a **class prototype object owning the methods as own properties**, not a regular instance with data properties.

Successful equals() calls earlier in the same run have `typeof this.$` of "object" and `'$' in this` true (regular instances). One specific call in the run hits a prototype receiver.

**Spec divergence #1 (found, not yet fixed):** Class methods in cruftless are **enumerable** on the prototype. Per ECMA §15.7 ClassDefinitionEvaluation step 28 + §15.7.10 ClassElementEvaluation, class methods are installed via `MethodDefinitionEvaluation` which uses `enumerable: false`. Direct test:
```js
class A { foo() {} }
Object.getOwnPropertyDescriptor(A.prototype, 'foo').enumerable
   bun: false   cruftless: true
```
This is the reason `Object.keys(prototype)` returns method names; per spec it should return `[]` (constructor is also non-enumerable on class prototypes per spec).

**Spec divergence #2 (found, not yet fixed):** ESM modules are not strict in cruftless. Per ECMA §15.5 ModuleDeclarationLinking + §16.2.1.6 InitializeEnvironment, module-scoped code is in strict mode. Test:
```js
class Foo { bar() { return this; } }
const fn = (new Foo()).bar;
fn() === globalThis
   bun: false   cruftless: true   (should be false; this should be undefined in strict)
```

**Root question — open:** WHERE in arktype is `BaseNode.prototype.equals(intrinsic.boolean)` being called with the prototype as receiver?

Hypotheses to test:
- arktype iterates branchGroups or a similar collection whose first element is, under some control-flow path cruftless takes but Bun doesn't, the class prototype rather than an instance. The two enumerability + strict-mode divergences above could individually or jointly produce this state (e.g., a `for…in` or `Object.assign` upstream picks up the prototype because methods are enumerable; a missing strict-mode this-binding produces a globalThis that gets coerced somewhere; etc.).
- arktype's class hierarchy is initialized with prototypes shared by multiple classes (mixin-style), and one of those shared prototype references leaks into a branchGroups slot.
- Tracking the call chain via stack trace is blocked because cruftless's `new Error().stack` returns the empty string (another spec gap, lower priority).

The next trace step is to add a stack trace via explicit logging at each plausible callsite — branchGroups assignment, intrinsic boolean initialization, etc. — until the source of the prototype-as-this assignment is named.

## What this trace establishes about the discipline

arktype is not a §XIV target. It is a sequence of §XIII alphabet-promotion candidates and concrete spec-correctness gaps in the engine substrate. Each wall is a single, well-defined divergence. The walls are layered because Doc 730 §XII's diagnostic-legibility property is operating *in reverse*: the deepest wall is masked by the shallower one's symptoms; once the shallower one is fixed, the deeper one's distinct symptom surfaces.

This is what "the pipeline is its own diagnostic" looks like in practice. Each fix doesn't just close one bug — it advances the resolution path to the next. The trace itself is the engagement-tier instrument that produces the substrate moves.

## Open log of substrate moves driven by this trace

| commit | tag | wall closed |
|---|---|---|
| `3ceab019` | `Ω.5.P03.E2.class-field-after-super` | Wall 1: derived-class field-init timing |
| `72f2bf47` | `Ω.5.P03.E2.super-new-target` | Wall 2: new.target through super(...) |
| `d1ab22cb` | `Ω.5.P04.E1.is-spec-object` | Wall 3: ToPrimitive Type-vs-typeof collapse |
| (pending) | `Ω.5.P04.E1.class-method-enumerability` | Wall 4: methods non-enumerable on prototype |
| (pending) | `Ω.5.P03.??.module-strict-mode` | Wall 4: ESM is strict by default |
| (pending) | (TBD) | Wall 4: prototype-as-this root in arktype |

Frontier: continue identifying the root for the prototype-as-this state. Each layer is its own engagement-tier substrate move under the discipline established at EXT 20.

---

*Live document. Will be updated as walls are closed.*
