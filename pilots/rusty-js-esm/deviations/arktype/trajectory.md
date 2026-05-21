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
