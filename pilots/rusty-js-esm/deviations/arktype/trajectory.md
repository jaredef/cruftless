# arktype deviation-resolution: trajectory

Pin-Art-shaped rung log for the arktype crash, executed via the [pipeline](./pipeline.md).

---

## §XII — Capture (closed)

**Artifact**: [`captures/L0-result.json`](./captures/L0-result.json)

- bun: `{status:"OK", keyCount:21}`
- cruftless: `TypeError: callee is not callable: undefined (method='rawIn') (receiver=Array(len=1))` at `union.js:661`

Crash chain (7 frames): `Scope.export → maybeResolve → def → ctx → impl.$ → inner → CRASH`.

---

## §XIII — Reduce (closed)

Three reductions, each a separate capture.

| Level | Input | bun | cruftless |
|---|---|---|---|
| L0 | `import * as M from 'arktype'` | OK, 21 keys | crash |
| L1 | `import * as S from '@ark/schema'` | OK, 179 keys | same crash |
| L3 | `import '@ark/schema'` (no exports query) | OK | same crash |

**Verdict**: deviation is in `@ark/schema` load-time code, independent of namespace query. The crash fires during ESM init.

---

## §XIV — Localize (closed)

Two trace iterations (L4, L5, L6) with progressively richer instrumentation in `roots/union.js:reduceBranches`. The same source patches ran under both engines via [`run-pipeline.sh trace`](./run-pipeline.sh).

**L4** — per-iteration shape probe: confirmed `branches[i]` is a function-typed Node in both engines; `constructor.name` is non-empty in bun (`"UnitNode"`), **empty string in cruftless** (a separate, minor class-name gap — recorded but not blocking).

**L5** — try/catch around `intersectNodesRoot` call: bun's first iteration succeeds and returns; cruftless's first iteration also succeeds. The crash occurs at iteration 2's intersectNodesRoot — but no `L5-before-intersect` log fired for iteration 2. The crash is therefore not at the call itself but in code reached BETWEEN intersect-OK and the next iteration's pre-eq probe.

**L6** — probes inserted after intersect-OK, after the `instanceof Disjoint` check, after `assertDeterminateOverlap`:

```
bun:        L6-post-intersect: { int_typeof:"object", is_disjoint:"Disjoint" }  → instanceof Disjoint true → continue
cruftless:  L6-post-intersect: { int_typeof:"object", is_disjoint:"Array" }     → instanceof Disjoint false → falls through
```

**First divergent point**: `intersection.constructor.name`. Bun: `"Disjoint"`. Cruftless: `"Array"`.

Reading `shared/disjoint.js:4`: `export class Disjoint extends Array`. Bun preserves the subclass; cruftless does not. The downstream consequence is that `intersection instanceof Disjoint` evaluates `false` in cruftless, the `continue` is skipped, and `.equals(branches[i].rawIn)` is called on a plain Array (which has no Disjoint methods, and the downstream `inner()` call then hits the rawIn-on-Array crash).

---

## §XV — Bracket (closed)

**Artifact**: [`probes/bracket-class-extends-array.mjs`](./probes/bracket-class-extends-array.mjs)

The bracket probe exercises `class X extends Array` in isolation (no arktype, no @ark/schema). It records five observables: `ctor.name`, `instanceof Subclass`, `instanceof Array`, `Array.isArray`, `typeof <subclass method>`.

| Observable | bun | cruftless | spec |
|---|---|---|---|
| `ctor.name` | `"MySub"` | `"Array"` | `"MySub"` |
| `instanceof MySub` | `true` | `false` | `true` |
| `instanceof Array` | `true` | `true` | `true` |
| `Array.isArray` | `true` | `true` | `true` |
| `typeof hello` | `"function"` | `"undefined"` | `"function"` |

**Substrate gap (precise)**: cruftless's class-constructor path for `class X extends Array` does not install `X.prototype` as the constructed instance's `[[Prototype]]`. The result is an Array instance, not an X instance. Subclass methods are unreachable; `instanceof X` is false.

ECMA-262 §22.1.2.1 (Array) and §10.1.13 ([[Construct]] semantics for Array exotic objects) require the subclass to be honored via the `newTarget` argument: `OrdinaryCreateFromConstructor(newTarget, "%Array.prototype%")` reads `newTarget.prototype` (the subclass's prototype), not `%Array.prototype%`.

---

## §XVI — Yield (LANDED, commit 764d7f88)

**Substrate move**: in `Op::New` (interp.rs), detect Array-subclass by walking `callee.prototype`'s proto chain for the canonical Array.prototype id (capped at 32 hops). When the chain reaches Array.prototype, pre-allocate the `this` object via `Object::new_array()` instead of `Object::new_ordinary()`, then install `callee.prototype` as the resulting object's proto.

**Effect**: when Array's intrinsic constructor runs in `super(...args)`, it sees the receiver is already an Array-kind object and mutates it in place (rather than allocating a sibling Array that would discard the derived-class proto wiring).

**Edit surface**: ~30 LOC, single function (Op::New handler). The change is local and well-gated.

**Bracket flip**:
```
before:  ctor=Array, instanceof MySub=false, typeof hello=undefined
after:   ctor=?,     instanceof MySub=true,  typeof hello=function
```

`ctor.name` is empty (not `"MySub"`) — a separate substrate gap: derived-class constructors not picking up their identifier as `.name`. Recorded as L4 trace finding; not blocking arktype.

**Sweep**: 95.7% (114/119) preserved, **no regressions**. arktype not yet PASS — the original rawIn-on-Array crash signature is GONE; arktype now fails at a downstream substrate gap (§XVII).

---

## §XVII — Iterate (NEW deviation surfaced)

**New crash**: `Cannot read property 'filter' of undefined (receiver='references')` at `scope.js:405` (the bootstrapAliasReferences function), reached via `discriminate → overlaps → intersect → finalize → node`.

**L7 instrumentation** (probe inside bootstrapAliasReferences):
```
bun:       resolution_type=function, kind=intersection/union, referencesById_keys=[5 ids]
cruftless: resolution_type=object,   keys=["0"],              referencesById=undefined
```

**Reading**: `resolution` should be a Node-as-function (`class BaseNode extends Callable`; `class Callable { constructor(fn, ...) { return Object.setPrototypeOf(fn.bind(...), this.constructor.prototype); } }` — explicit-return-from-constructor pattern). In bun, `typeof resolution === "function"`. In cruftless, `typeof resolution === "object"` with one own key `"0"` — this is an Array(len=1) shape. Likely a Disjoint instance escaping where a Node-function was expected, or a separate explicit-return-from-derived-constructor substrate gap.

**Deferred**: this is the next deviation. Pipeline re-entered. Closing it requires either:
1. A new bracket probe for `class X extends Callable; constructor() { return Object.setPrototypeOf(fn.bind(...), this.constructor.prototype); }` — to isolate whether cruftless honors explicit-return-from-derived-constructor + setPrototypeOf chains correctly.
2. Or: trace further up the maybeResolve / bindReference chain to find where the wrong-shape value enters.

This is its own §XII–§XV cycle queued as a future focused-session rung. The pipeline's value is exactly this: each iteration produces a smaller, sharper substrate gap, and the artifacts compose.

### §XVII.§XV — Bracket: Callable explicit-return pattern (PASS in both engines)

**Artifact**: [`probes/bracket-callable-explicit-return.mjs`](./probes/bracket-callable-explicit-return.mjs)

The probe exercises the @ark/util `Callable` idiom in isolation: a base class whose constructor returns `Object.setPrototypeOf(fn.bind(...), this.constructor.prototype)`. A subclass adds methods; the probe asserts `typeof === "function"`, `instanceof Sub`, `instanceof Base`, method dispatch on both classes, and direct invocation.

**Result**: bun and cruftless produce **identical** output. All assertions pass on both engines.

```
typeof s: function
callable: inner(42)
instanceof Sub: true
instanceof Base: true
typeof s.ping: function
typeof s.bark: function
s.ping(): pong
s.bark(): woof
```

**Verdict**: the Callable explicit-return pattern is NOT the substrate gap. cruftless handles it correctly. The L7 deviation (resolution being Array-shaped) comes from somewhere upstream — likely a Disjoint propagating where a Node is expected, driven by an earlier control-flow divergence the pipeline hasn't yet localized.

### §XVII follow-up — defer

Closing this iteration. The next move requires further upstream instrumentation (probing `bindReference`, `parseDefinition`, or the per-kind intersection implementations to find where Disjoint enters a Node-expecting channel). That work is genuine arktype-internal probing, not substrate work; it warrants a focused session rather than continued in-line iteration.

The pipeline's discipline holds: when an iteration's bracket comes back PASS, the substrate is not the gap. Either the deviation is package-internal logic (out of cruftless's scope) or there's a much subtler substrate primitive still hidden. Either way, declaring the iteration closed and queuing the next is the right move — not chasing it without a bracket signal.

---

## Pipeline outcome (after §XVII iteration 1)

- **Stages closed**: §XII, §XIII, §XIV, §XV, §XVI, §XVII (iteration 1).
- **Substrate gaps closed**: 1 (Array-subclass identity).
- **Substrate gaps localized but PASS in bracket**: 1 (Callable explicit-return — not actually a gap).
- **Substrate gaps remaining**: 1 (the as-yet-unlocalized upstream divergence producing Disjoint where Node expected).
- **Parity delta**: 95.7% preserved, no regressions. arktype itself still FAIL.

The pipeline has done substantial work: one closed gap, one verified-non-gap (valuable too — it tells us this Callable pattern is safe to write into future arktype-style code in cruftless), and clear next-move shape.

---

## §XVII iter 2 — ArraySpeciesCreate substrate gap (CLOSED, LANDED)

**Pipeline trail** (artifacts under traces/ and captures/):
- L8 (root.js:intersect entry): bun reports `result.constructor.name === "Disjoint"` and `instanceof_Disjoint: true`. cruftless reports `"Array"` and `false`. The deviation surfaces immediately after `rawIntersect` returns; nothing between.
- L9 (intersectOrPipeNodes cache-store): both engines report `instanceof_Disjoint: true`. The Disjoint exists AT cache-store time.
- L10 (intersectOrPipeNodes cache-hit-direct): never fires in cruftless on the failing call. The lr-cache path is not the source.
- L11 (root.js:intersect class identity): `result.proto === Disjoint.prototype: false`, `result.proto.constructor.name: "Array"`. Confirms result's [[Prototype]] is literally Array.prototype, not Disjoint.prototype.

**§XV bracket** (`probes/bracket-array-species` in spirit — minimal repro inline):
```js
class Sub extends Array { static init(x) { return new Sub(x); } }
const s = Sub.init({a:1});
const mapped = s.map(x => x);
console.log(Object.getPrototypeOf(mapped) === Sub.prototype);
```
- bun: `true` (subclass preserved through `.map`).
- cruftless: `false` (plain Array returned).

The substrate gap: `array_species_create` ignored the source's `O.constructor`, always allocating a plain Array. ECMA-262 §22.1.3.17 step 3-6 require constructing via `O.constructor[@@species]` (defaulting to `O.constructor`) when O is an Array-subclass instance whose constructor is a function.

**§XVI substrate move** (commit `533443e6`):
- Check O is Array-kind.
- Read `O.constructor`; skip if it's the intrinsic `%Array%`.
- Mirror Op::New's allocation: pre-allocate Array-kind body with `proto = ctor.prototype`, dispatch via `call_function` with `pending_new_target` set, honor explicit-return per §10.2.1.1.
- Fall back to plain Array on any non-match.

Risk gated by: (a) is-array check on O, (b) is-function check on ctor, (c) is-not-intrinsic-Array check on ctor, (d) explicit-return-honor mirroring §10.2.1.1 (so a Callable-style constructor's return value wins over the pre-allocated this).

**Bracket flip**: cruftless now matches bun on the inline repro. `class Sub extends Array; new Sub(...).map(...).constructor === Sub`.

**Sweep**: 99.1% (118/119) preserved, no regressions across the 119-package corpus.

**arktype outcome**: original `rawIn-on-Array` crash signature is **gone**. The intersect → invert path now propagates Disjoint identity through `.map`. arktype now fails at a different downstream substrate gap (§XVII iter 3).

---

## §XVII iter 3 — generic.js paramDefs gap (READING, deferred)

**New crash**: `Cannot read property 'map' of undefined (receiver='paramDefs')` at `generic.js:56:43`. Call chain: `constraints → MergeHkt → ...`. The receiver expression is `this.paramDefs`; on the relevant Generic instance it evaluates to undefined where bun has the array of param definitions.

**Source context**: `class GenericRoot extends Callable` with a class-field declaration `paramDefs;` and a constructor that does `this.paramDefs = paramDefs` after `super(...)`. `Callable.constructor` returns an explicit value (a bound function with mutated prototype). The downstream `this.params` getter calls `this.paramDefs.map(...)`.

**§XV bracket** (`class Sub extends Base { paramDefs; constructor(paramDefs) { super(fn); this.paramDefs = paramDefs; } show() { return this.paramDefs; } }`): **PASS in both engines**. The obvious pattern (class field + explicit-return-from-super + later assignment + later read) works identically in cruftless and bun.

**Verdict for this rung**: the bracket-PASS means the substrate is not the obvious gap. Either:
1. The arktype call site constructs a GenericRoot via a less-obvious path (e.g., via `new reference.constructor(...)` after a property table mutation that loses `paramDefs`).
2. There's a subtler interaction between Callable's explicit-return, class-field hoisting, AND a downstream `Object.defineProperty` or `Object.assign` that drops the field.
3. A different deviation closer to the GenericRoot construction site is the real source — needs L12+ instrumentation at `parseGeneric` and `bindReference` to pinpoint.

**§XVIII recovery axis** (the structural-set-membership protocol from Doc 730 §XVIII): the next reading move is to project the divergence not against descriptor properties but against the set of intermediate constructors invoked between `parseGeneric` and the failing `params` getter. The reference set might be: the constructors used by `bindReference` (`reference.constructor`), the `withId`/`withMeta` rewrite path's allocated proxies, or the Callable-bind chain's intermediate function-typed objects.

**Deferred** as its own focused-session rung. Pipeline framework + L11 + bracket-extends-array + bracket-callable-explicit-return + bracket-class-field-explicit-return are all in place; the next session resumes from these artifacts.

---

## Pipeline outcome (after §XVII iter 2)

- **Substrate gaps closed**: 2 (Array-subclass identity in Op::New; ArraySpeciesCreate honoring O.constructor).
- **Substrate gaps verified-non-gap by bracket**: 2 (Callable explicit-return; class-field + explicit-return + later assignment).
- **Substrate gaps remaining**: 1 (the §XVII iter 3 gap — needs deeper upstream instrumentation; obvious bracket disproved).
- **Parity**: 95.7% → 99.1% across the pipeline's life. arktype itself still FAIL but two substrate gaps deeper than at pipeline founding.

The pipeline framework has paid for itself: each substrate move that lands is package-independent (Array-subclass identity helps ANY package extending Array; ArraySpeciesCreate helps ANY consumer of Array.prototype.map/filter/slice on a subclass), and each non-gap bracket leaves a re-runnable artifact ruling out one hypothesis.

The substrate move lands the spec-aligned `new <Subclass-of-Array>(args)` semantics. Edit surface (anticipated, not yet confirmed): cruftless's `new` operator / class constructor logic in `rusty-js-bytecode` and/or `rusty-js-runtime`. Likely 50–200 LOC.

**Probe flip target**: cruftless's bracket probe matches bun's trace exactly.

**Estimated parity delta at sweep tier**: +1 package (arktype). Possible incidental flips on any corpus package that subclasses a built-in (Array, Map, Set, Error). Mandatory sweep before commit.

**Risk**: spec-compliant subclassing of built-ins is one of the trickier parts of ES2015 class semantics. The fix must be careful not to over-apply (e.g., affecting non-built-in subclasses, breaking `super()` chaining). The bracket probe is the load-bearing artifact; if it flips and no regressions appear, the gate is correct.

**Deferred**: this is a deeper engine fix than the locale's previous rungs and warrants its own focused session.

---

## §XVII — Iterate (not yet entered)

Reserved for follow-up after §XVI. If §XVI closes L0, the deviation is resolved. If L0 still crashes downstream (the rawIn-on-Array path was only the first of several Disjoint-vs-Array branches), re-enter §XIV at the next divergent point.

---

## Pipeline outcome (current)

- **Stages closed**: §XII, §XIII, §XIV, §XV.
- **Stages queued**: §XVI, §XVII.
- **Substrate gap localized**: `class X extends Array` subclass identity loss.
- **Bracket artifact**: in place, currently divergent (`MySub` vs `Array`).
- **Source touched outside cruftless**: only via instrumentation runs; restored to vendor state after L6.

The pipeline did exactly what Doc 730 §XII–§XVII designs it to do: surfaced the substrate gap with surgical precision (`new Subclass-of-Array`) without requiring source-reading of arktype as if it were the engine's own code. The bracket probe is package-independent and can be re-run any time to verify the fix.
