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
