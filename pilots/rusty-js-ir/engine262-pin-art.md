# engine262 Pin-Art Probes — High-Fidelity ECMA IR Constraints

Probe series surfacing the implicit structural commitments of a 1:1-with-spec
JavaScript engine (engine262, TC39's reference). Output is the alphabet of
spec-distinctions any §XIII Tier-1.5 spec-IR must preserve to claim
"high-fidelity ECMA IR".

Methodology: Pin-Art recovery-and-articulate (Doc 581) on engine262's
`src/abstract-ops/` + `src/intrinsics/` + `src/runtime-semantics/`.

Engine version: shallow-cloned 2026-05-20 from
https://github.com/engine262/engine262 (main).

---

## Pass A — Abstract-Op Coverage Matrix

**engine262**: 165 distinct named abstract ops in `src/abstract-ops/`
(40 files), plus several hundred more in `intrinsics/` and `runtime-semantics/`.

**cruftless**: 281 `pub fn`s in `pilots/rusty-js-runtime/derived/src/interp.rs`
— roughly 1:1 ratio to engine262's abstract-op surface, but the mapping is
*not* faithful at the boundary. Cruftless functions fall in three buckets:

1. **Spec-named** (e.g., `to_object`, `to_primitive`, `length_of_array_like`)
   — direct equivalent of a named abstract op.
2. **Spec-derived** (e.g., `coerce_to_string`, `read_property_via`,
   `spec_get`) — operationally correct lowering of an abstract op but
   named for the implementation pattern, not the spec verb.
3. **Engine-specific** (e.g., `array_length`, `obj_mut`, `alloc_object`)
   — internal-slot accessors and storage shims with no spec correspondence.

The structural recognition: a *high-fidelity ECMA IR* would name every
spec abstract op explicitly at the IR boundary (bucket 1), reserve
implementation-pattern helpers for the lowering surface only (bucket 2),
and forbid bucket 3 from appearing in any IR section.

### A.1 type-conversion.mts gap (29 ops)

engine262's `type-conversion.mts` exports these spec-named abstract ops:

```
ToPrimitive, OrdinaryToPrimitive, ToBoolean, ToNumeric, ToNumber,
StringToNumber, ToIntegerOrInfinity, ToFixedSizeInteger (generic),
ToInt32, ToUint32, ToInt16, ToUint16, ToInt8, ToUint8, ToUint8Clamp,
ToBigInt, StringToBigInt, ToBigInt64, ToBigUint64,
ToString, ToObject, ToPropertyKey, ToLength, CanonicalNumericIndexString,
ToIndex
```

cruftless explicit equivalents:

```
ToPrimitive       — to_primitive_via (IR-lifted at EXT 72)
ToBoolean         — abstract_ops::to_boolean (static)
ToNumber          — abstract_ops::to_number (static, no Object dispatch)
ToString          — abstract_ops::to_string (static) + to_string_strict (rt) + coerce_to_string (rt)
ToObject          — to_object (rt)
ToBigInt          — abstract_ops::to_bigint (rt) — EXT 78
ToInt32 / ToUint32 — inline at use sites
ToLength          — length_of_array_like (rt)
ToPropertyKey     — inline at use sites (EXT 77)
ToIntegerOrInfinity — inline at three Number.prototype.* sites (EXT 80)
```

**MISSING from cruftless** (spec-named ops with no explicit carrier):

```
OrdinaryToPrimitive         — inline inside to_primitive_via
ToNumeric                   — Number-or-BigInt dispatch missing
StringToNumber              — spec's numeric-literal parser absent
ToFixedSizeInteger (generic) — typed-array work absent
ToInt16, ToUint16, ToInt8, ToUint8, ToUint8Clamp — typed-array support absent
StringToBigInt              — inline inside to_bigint
ToBigInt64, ToBigUint64     — BigInt-typed-array support absent
CanonicalNumericIndexString — array-index detection absent
ToIndex                     — inline at use sites
```

**§XIII alphabet promotion candidates**: each missing op above is a
discrimination the spec carries that cruftless's IR currently collapses.
Promoting them to typed IR primitives would surface each at the
verifier-time discrimination boundary.

### A.2 testing-comparison.mts gap (16 ops)

engine262:

```
RequireObjectCoercible, IsArray, IsCallable, IsConstructor, IsExtensible,
IsIntegralNumber, IsPropertyKey, IsRegExp, IsStringPrefix,
SameValue, SameValueZero, SameValueNonNumber,
IsLessThan, IsLooselyEqual, IsStrictlyEqual
```

cruftless has: `is_callable`, `is_constructor` (inline), `is_extensible`,
`is_array` (inline), `is_regexp` (via Object internal kind check),
`require_object_coercible`, `is_loosely_equal_rt`, `is_strictly_equal`.

**MISSING**: IsIntegralNumber, IsPropertyKey (exposed), IsStringPrefix,
SameValueNonNumber.

### A.3 objects.mts / object-operations.mts gap (44 ops)

engine262 names every internal-method-of-Ordinary-objects explicitly:

```
OrdinaryDefineOwnProperty, OrdinaryDelete, OrdinaryGet,
OrdinaryGetOwnProperty, OrdinaryGetPrototypeOf, OrdinaryHasProperty,
OrdinaryIsExtensible, OrdinaryOwnPropertyKeys, OrdinaryPreventExtensions,
OrdinarySet, OrdinarySetPrototypeOf, OrdinarySetWithOwnDescriptor,
ValidateAndApplyPropertyDescriptor, IsCompatiblePropertyDescriptor,
OrdinaryObjectCreate, OrdinaryCreateFromConstructor,
GetPrototypeFromConstructor,
Call, Construct, CopyDataProperties, CreateArrayFromList,
CreateDataProperty, CreateDataPropertyOrThrow, CreateListFromArrayLike,
CreateMethodProperty, CreateNonEnumerableDataPropertyOrThrow,
DefinePropertyOrThrow, DeletePropertyOrThrow,
EnumerableOwnProperties, Get, GetFunctionRealm, GetMethod, GetV,
GroupBy, HasOwnProperty, HasProperty, Invoke,
LengthOfArrayLike, MakeBasicObject, OrdinaryHasInstance,
Set, SetIntegrityLevel, SetterThatIgnoresPrototypeProperties,
SpeciesConstructor, TestIntegrityLevel
```

cruftless has many of these (CreateDataPropertyOrThrow, HasProperty,
LengthOfArrayLike, Set, SpeciesConstructor) but as inline patterns or
under different names. The Ordinary* family (OrdinaryGet, OrdinarySet,
OrdinaryDefineOwnProperty, etc.) is implicit in cruftless — when a
non-exotic Object is the receiver, the Object's own internal methods
ARE the Ordinary* implementations. Spec separates them so that exotic
objects (Arrays, Proxies, TypedArrays) can override individually.

**Key recognition**: the Ordinary*-vs-exotic distinction is the same
discrimination as our `InternalKind` enum, but engine262 names *each
Ordinary method separately*. The EXT 84 Proxy work has been re-implementing
this dispatch table site-by-site (Reflect.defineProperty, then
Object.defineProperty, then bytecode Op::SetProp, etc.). engine262's
shape suggests a different substrate: a single dispatch table
`InternalMethods<Kind>` with one entry per exotic, each named after
the spec method. The EXT 84 sites would collapse to one call per spec
algorithm.

---

## Pass A coverage matrix summary

| Category | engine262 ops | cruftless explicit | gap |
|---|---|---|---|
| type-conversion | 29 | 9 | 20 |
| testing-comparison | 16 | 8 | 8 |
| objects + object-operations | 44 | ~15 | ~29 |
| iterator-operations | 5 | 2 | 3 |
| array-objects | 8 | ~3 | ~5 |
| function-operations | 11 | ~4 | ~7 |
| keyed-collections (Map/Set) | 4 | 3 | 1 |
| promise-operations | 6 | ~3 | ~3 |
| proxy-objects | 12 | (just landed in EXT 84) | 0 |
| **Total** | **~135** | **~50** | **~85** |

(Excludes typed-array / shared-array-buffer / dataview / async-generator /
shadow-realm / module-records / temporal — out of cruftless v1 scope.)

**Headline**: cruftless has ~37% of engine262's spec-abstract-op surface
explicitly named. The other ~63% is inline at use sites OR collapsed into
implementation-pattern helpers. Each gap is a candidate §XIII Tier-1.5
alphabet promotion — a spec discrimination that, if surfaced as a typed
IR primitive, would let the verifier catch the collapse class that
EXT 73 / 78 / 79c / 82c each discovered by test262 failure rather than
by audit.

---

## Pass B (queued): trace ToPrimitive through both engines

Pick §7.1.1 ToPrimitive (the EXT 72b motivator). Walk one input case
(`'' + new Proxy(target, handler)`) through engine262 and through
cruftless. Diff the call graphs. The diff names the abstract ops cruftless
inlines that engine262 keeps as discrete spec-named operations.

## Pass C (queued): §10.5 Proxy internal methods trace

Walk the 12 Proxy internal methods (§10.5.{1..14}) through engine262.
For each, name the invariants the spec requires the trap-return to satisfy
against the target's [[InternalMethod]] equivalent. Output: the
per-trap-invariant work list deferred at EXT 84e (47 TypeError tests).

## Pass D (queued): Realm-passing inventory

Inventory which engine262 abstract ops take Realm explicitly vs derive
it from the receiver. Output: the substrate-architecture change list
for multi-Realm (the 37 carve-out tests would become reachable if Realm
threading were instantiated).

---

## Pass B — ToPrimitive trace diff (engine262 vs cruftless)

**Input case**: `'' + p` where `p = new Proxy({}, {get(t,k){trace.push(k); return undefined}})`.
The EXT 72b motivator. Selected because cruftless already shipped two
IR-section corrections here (EXT 72 lift, EXT 72b typeof-function fix) and
EXT 82's SpecGet first carrier converted one of its sites. The call graph
diff names what the remaining gap is.

### B.1 engine262 call graph

```
BinaryExpression(+) evaluator
  → ApplyStringOrNumericBinaryOperator('+', '', p)
    → ToPrimitive(p)              [type-conversion.mts §7.1.1]
      → GetMethod(p, @@toPrimitive)  [object-operations.mts §7.3.10]
        → GetV(p, @@toPrimitive)     [object-operations.mts §7.3.2]
          → ToObject(p)              [type-conversion.mts §7.1.18]
          → p.[[Get]](@@toPrimitive, p)  [Proxy internal method §10.5.8]
            → trap = OrdinaryGet(handler, 'get', handler)
            → if IsCallable(trap): Call(trap, handler, [target, '@@toPrimitive', receiver])
            → else: target.[[Get]](@@toPrimitive, receiver)
        → return null/undefined → undefined; else → check IsCallable, throw if not
      → exoticToPrim is undefined → fall through
      → OrdinaryToPrimitive(p, 'number')  [type-conversion.mts §7.1.1.1]
        → for name in [valueOf, toString]:
          → method = Get(p, name)              [object-operations.mts §7.3.2]
            → p.[[Get]](name, p)  [Proxy internal method]
              → trap = OrdinaryGet(handler, 'get', handler)
              → if IsCallable(trap): Call(trap, handler, [target, name, receiver])
              → else: target.[[Get]](name, receiver)
          → if IsCallable(method): Call(method, p) → result
            → if Type(result) ≠ Object: return result
        → throw TypeError
```

Depth: 6 spec-named functions in the longest chain (ToPrimitive → GetMethod
→ GetV → ToObject + p.[[Get]] → OrdinaryGet on handler → Call).

### B.2 cruftless call graph (post-EXT 82c)

```
Op::Add bytecode handler
  → op_add_rt(l, r)                                          [interp.rs:293]
    → to_primitive(self, l, "default")                       [interp.rs:283]
      → crate::generated::to_primitive(rt, v, args)          [generated.rs §7.1.1]
        → IR step 1.fast: TypeOf → check object/function     (EXT 72b inline)
        → IR step 2.a.lookup: Expr::SpecGet(value, "@@toPrimitive")
          → rt.spec_get(&value, "@@toPrimitive")             [interp.rs:407]
            → proxy_target_handler_checked(id)?
            → if Proxy: object_get(handler, "get") → call_function(trap, handler, [target, key, receiver])
            → else: read_property(id, key)                   [interp.rs:5095]
        → IR step 2.b.has_exotic: IsCallable check
          → (no callable-but-non-undefined → TypeError path; spec's GetMethod does)
        → IR step 3.order / 3.swap: method1/method2 = "valueOf" / "toString"
        → IR step 4.m1.lookup: Expr::CallBuiltin{name:"get_via", args:[value, method1]}
          → rt.get_via(&value, &method1)                     [interp.rs:2260]
            → coerce_to_string(method1)
            → spec_get(value, &key_str)                      (EXT 82b promotion)
        → IR step 4.m1.callable: IsCallable
        → IR step 4.m1.call: Expr::Call → call_function
        → IR step 4.m1.check / 4.m1.fn_check: typeof not in {object, function} → return
        → IR step 5.m2: same shape for toString
        → IR step 6.throw: TypeError
```

Depth: same number of dispatch levels, but the composition shape differs.

### B.3 Diff

| spec function | engine262 | cruftless | gap |
|---|---|---|---|
| ToPrimitive (§7.1.1) | top-level abstract op | IR section | aligned |
| GetMethod (§7.3.10) | wraps GetV + IsCallable check + TypeError | inline IsCallable in IR; no TypeError on non-null-non-callable | **missing** |
| GetV (§7.3.2) | wraps ToObject + [[Get]] | implicit (rt.spec_get accepts Value, returns Undefined for primitives instead of ToObject + dispatch) | **missing** |
| ToObject (§7.1.18) | first step of GetV | rt.to_object — exists but not threaded into spec_get's primitive-receiver path | **partial** |
| O.[[Get]] (§10.1.8 / §10.5.8) | dispatched per exotic kind | rt.spec_get dispatches Proxy + accessor | **partial** (Proxy + accessor covered; Array exotic [[Get]] / String exotic [[Get]] not separately addressed) |
| OrdinaryToPrimitive (§7.1.1.1) | distinct abstract op | inlined inside ToPrimitive IR section | **flattened** |
| Get (§7.3.2 alias of GetV) | shared with GetV | reused get_via runtime helper | **aligned** post-EXT 82b |

### B.4 §XIII alphabet promotions surfaced

Pass B names four concrete Tier-1.5 promotions that would close cruftless's
ToPrimitive trace to engine262's:

1. **GetMethod as a first-class IR primitive**, not inlined. Spec calls
   GetMethod precisely because it has a load-bearing post-condition
   ("callable or undefined or throw"). Inlining loses the verifier-time
   guarantee that we always throw when a defined-but-non-callable
   @@toPrimitive / valueOf / toString is encountered.

2. **GetV as the primitive-receiver-aware [[Get]]**. The spec carries
   ToObject(V) inside GetV explicitly. cruftless's spec_get returns
   Undefined for non-Object receivers; engine262 boxes via ToObject then
   dispatches the wrapper's [[Get]]. The behavioral gap surfaces when
   user code calls `(42)[@@toPrimitive]` — spec returns undefined via
   the Number-wrapper chain; cruftless returns undefined via short-circuit.
   Same outcome but different chain — and the chain difference matters
   if `Number.prototype` is monkey-patched (rare but spec-required).

3. **OrdinaryToPrimitive as a separate IR section**. The §7.1.1.1 spec
   text is a distinct algorithm with its own pre/post-conditions. Folding
   it into ToPrimitive's IR section is correct-by-construction today
   (both EXT 72 and EXT 72b stayed inside the section), but it forecloses
   the spec's modularity — a hypothetical override of OrdinaryToPrimitive
   (e.g., for Date objects per Annex B) can't be expressed cleanly.

4. **Exotic-[[Get]] dispatch table**. The spec splits O.[[Get]] into one
   essential internal method per exotic-object kind (Ordinary, Module
   Namespace, Proxy, Integer-Indexed Exotic, String Exotic, Array Exotic,
   Bound Function). cruftless's spec_get currently dispatches Proxy
   explicitly and OrdinaryGet implicitly via read_property; the other
   exotics inherit from Object.prototype.[[Get]] which doesn't model the
   spec-specific overrides (e.g., String exotic's exotic-own-property
   length read). Promoting to an `InternalMethods<Kind>` table per spec
   method (the Pass A structural recognition) collapses the gap.

The four promotions above are the §XIII work list for one spec section.
Pass C will produce the analogous list for the 12 Proxy internal methods.

---

## Pass C — Proxy per-trap invariant inventory (§10.5)

Walks the 12 Proxy internal methods in engine262's `src/abstract-ops/proxy-objects.mts` and inventories the post-conditions each one enforces *after*
calling the handler trap. cruftless's EXT 84 family landed the dispatch
shape (trap callable, fall-through to target, boolean coerce, falsy throws)
but stopped short of the trap-vs-target consistency checks the spec mandates.
Output: the per-trap work list for the 47 remaining TypeError tests.

| # | Internal method | engine262 lines | Post-conditions cruftless skips |
|---|---|---|---|
| 10.5.1 | `[[GetPrototypeOf]]()` | 37–63 | If target is non-extensible, trap-returned proto must SameValue(target.[[GetPrototypeOf]]()); else TypeError. |
| 10.5.2 | `[[SetPrototypeOf]](V)` | 65–92 | If target is non-extensible, trap-returned-true requires V === target.[[GetPrototypeOf]](); else TypeError. |
| 10.5.3 | `[[IsExtensible]]()` | 94–113 | Trap result must SameValue(target.[[IsExtensible]]()); else TypeError. |
| 10.5.4 | `[[PreventExtensions]]()` | 115–136 | If trap returned true, target must already be non-extensible; else TypeError. |
| 10.5.5 | `[[GetOwnProperty]](P)` | 138–217 | (12 distinct sub-checks) — trap-returned-desc must be coerced via ToPropertyDescriptor; compared via IsCompatiblePropertyDescriptor against targetDesc; if targetDesc non-configurable, trap-returned-desc must match. Non-extensible target + trap-undefined: must match targetDesc-undefined. |
| 10.5.6 | `[[DefineOwnProperty]](P, Desc)` | 218–291 | Trap-returned-true must be IsCompatiblePropertyDescriptor against target; settingConfigFalse must match targetDesc's configurable; non-writable data property invariant. |
| 10.5.7 | `[[HasProperty]](P)` | 293–322 | If trap returned false but target has P as non-configurable own (or non-extensible), throw TypeError. |
| 10.5.8 | `[[Get]](P, Receiver)` | 323–353 | Non-configurable non-writable own data property: trap must SameValue(targetDesc.[[Value]]). Non-configurable accessor with undefined getter: trap must return undefined. |
| 10.5.9 | `[[Set]](P, V, Receiver)` | 354–387 | Non-configurable non-writable own data property: V must SameValue(targetDesc.[[Value]]). Non-configurable accessor with undefined setter: throw TypeError. |
| 10.5.10 | `[[Delete]](P)` | 388–435 | Trap-returned-true: target.[[GetOwnProperty]](P) must not exist as non-configurable. |
| 10.5.11 | `[[OwnPropertyKeys]]()` | 436–499 | Trap result coerced via CreateListFromArrayLike; must be List of property keys; must contain all target's non-configurable own keys; if target non-extensible, must equal target's own keys exactly (and target must not have extra keys not in trap result). |
| 10.5.12 | `[[Call]](thisArg, args)` | 500–516 | (cruftless covers in EXT 84 apply path; trap signature only.) |
| 10.5.13 | `[[Construct]](args, newTarget)` | 517–end | (cruftless covers in EXT 84 — non-Object return throws TypeError.) |

### C.1 The invariant pattern

Each Proxy internal method follows the same shape:

```
1. Read trap from handler[trap_name].
2. If trap absent: delegate to target.[[InternalMethod]](...).
3. Else: call trap, get raw result.
4. Coerce / convert trap result to spec type (ToBoolean, ToPropertyDescriptor, CreateListFromArrayLike, ...).
5. Compute "the target's version" of the result via target.[[InternalMethod]]/[[GetOwnProperty]]/[[IsExtensible]]/[[GetPrototypeOf]].
6. Apply spec invariants: trap result vs target version must satisfy the specific constraints in §10.5.N steps 9+.
7. If invariants violated: throw TypeError with a specific message.
8. Else return the trap result.
```

cruftless EXT 84 implements steps 1–4 + 8. Step 5 (target peek) and step 6 (invariant check) and step 7 (specific error message) are the residue.

### C.2 What the engine262 surface tells us about IR shape

The 12 internal methods are *methods on the ProxyExoticObject value*; they
are not separate top-level abstract ops. engine262 packages them inside
`ProxyExoticObjectValue.prototype` — each Proxy carries its full dispatch
table as part of the object's identity. The trap-vs-target invariants
referenced inside are calls to `target.[[InternalMethod]]` (the target's
own dispatch table).

This recovers a structural commitment of the spec: **internal methods
are per-Object-kind virtual functions, not per-operation switches**.
cruftless's EXT 84 site-by-site rewriting (intrinsic closure handles
Proxy → call generated::* for Ordinary) is the same data flow but
expressed as N copies of the dispatch table rather than 1.

§XIII alphabet promotion implied:
- `InternalMethods<Kind>` table per spec method.
  - One row per `InternalKind` variant (Ordinary, Array, Function,
    Proxy, NumberWrapper, StringWrapper, ...).
  - One column per spec internal method ([[Get]], [[Set]],
    [[GetOwnProperty]], [[DefineOwnProperty]], [[Delete]],
    [[HasProperty]], [[OwnPropertyKeys]], [[GetPrototypeOf]],
    [[SetPrototypeOf]], [[IsExtensible]], [[PreventExtensions]],
    [[Call]], [[Construct]]).
  - Each cell is a function pointer (or trait method) whose signature
    matches the spec's parameter list.

Every Op::GetProp / Reflect.get / Object.X call site collapses to
`internal_methods[kind].get(...)` — one dispatch per site, the
right implementation per object kind, no site-by-site re-implementation.
The EXT 84 family's 5 EXTs (84/84b/84c/84d/84e) would have been
1 EXT under this shape: add the Proxy row to the table.

### C.3 Per-trap work list (deferred, 47 test262 tests)

Each cell below is one Proxy invariant the engine262 trace surfaces:

```
GetPrototypeOf    : non-extensible target + trap-proto-mismatch  (~3 tests)
SetPrototypeOf    : non-extensible target + trap-true-but-V≠target.proto  (~2 tests)
IsExtensible      : trap-vs-target-mismatch  (~3 tests)
PreventExtensions : trap-true-but-target-still-extensible  (~4 tests)
GetOwnProperty    : multiple sub-invariants (~7 tests)
DefineOwnProperty : multiple sub-invariants (~6 tests)
HasProperty       : non-configurable own / non-extensible (~2 tests)
Get               : non-configurable non-writable data mismatch (~2 tests)
Set               : non-configurable non-writable data mismatch (~2 tests)
Delete            : non-configurable own can't be reported deleted (~2 tests)
OwnKeys           : trap missing target non-configurable keys / extra keys (~14 tests)
```

Total: ~47, matching the EXT 84e post-sweep residue.

The InternalMethods table promotion (§XIII alphabet) lets these be added
as one row's worth of invariant code in one place, rather than 12 spots
per trap × 2 sites (intrinsic + bytecode VM) = 24 sites that would need
updating in the current shape.
