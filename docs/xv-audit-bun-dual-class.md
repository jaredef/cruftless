# §XV Audit: The Bun-Strict / Cruftless-Tolerant Dual Class

A working document recording the Doc 730 §XV constraint-comprehension audit of the four top500 packages where Bun rejects what cruftless accepts. Per the §XVI engine-diff oracle methodology, the four-case categorization gives the substrate verdict for each.

Date opened: 2026-05-20 (EXT 21 close).

## The four packages

```
later            Bun: ReferenceError: later is not defined
proxyquire       Bun: TypeError: Requested module is not instantiated yet.
ast-types-flow   Bun: BuildMessage: Unexpected type
sentry           Bun: TypeError: Attempted to assign to readonly property.
```

These are the only four packages on the top500 sweep where the post-EXT-21 substrate stack has cruftless succeeding while Bun fails. The §XV audit asks, for each: what invariant is cruftless's leniency absorbing, and is that absorption a legitimate ecosystem tolerance or a real engine bug?

## sentry — strict-mode write-to-non-writable not enforced

**§XVI probe:**
```js
"use strict";
const o = {};
Object.defineProperty(o, "x", { value: 1, writable: false, configurable: false });
o.x = 99;
```

```
bun:        threw: TypeError: Attempted to assign to readonly property.
cruftless:  no throw; o.x = 1   (assignment silently discarded)
```

Same shape for `Object.freeze({a: 1})` — Bun throws on the post-freeze write, cruftless silently swallows.

**Spec section:** ECMA-262 §10.1.9.4 OrdinarySetWithOwnDescriptor step 4. When the receiver's own data descriptor has writable=false, return false. SimpleAssignmentExpression in strict mode throws TypeError on the false return per §13.15.4 step 1.f.iv.

**Categorization (Doc 730 §XVI.a):** case (1). Cruftless violates the spec; Bun is spec-correct.

**§XV protected_invariants if we were to absorb the leniency as a §XIV deviation:** the invariant being absorbed is "post-freeze (or post-readonly-define) defensive checks throw TypeError." Sentry's bundle uses Object.freeze-then-write as a defensive sentinel (a common pattern in module-init hardening). Bun's throw triggers sentry's catch path; cruftless's silent swallow lets execution continue past the intended guard, producing the empty-export-shape behavior observed in the top500 sweep (sentry loads with 0 exported keys in cruftless because the writes silently lost their data).

**Audit verdict:** **DO NOT** promote to §XIV. The absorbed invariant is load-bearing for any code that uses freeze-then-write as a guard pattern. Cruftless's leniency is a real engine bug producing silent data loss. Substrate move: §XII coercion lift in the Op::SetProp / Op::SetIndex path at the runtime, gated on strict mode + descriptor writable=false → emit TypeError.

## later — strict-mode write-to-undeclared-identifier not enforced

**§XVI probe:**
```js
"use strict";
undeclaredVar = 42;
```

```
bun:        threw: ReferenceError: undeclaredVar is not defined.
cruftless:  no throw; globalThis.undeclaredVar = undefined
            (the assignment is silently lost; not even a global is created)
```

Same shape when "use strict" appears inside a function body — cruftless's strict-mode write-rejection is not enforced at the assign-to-undeclared site.

**Spec section:** ECMA-262 §13.15.4 SimpleAssignmentExpression step 1.f. If PutValue returns a Reference Record where IsUnresolvableReference is true and the strict bit is set in the running execution context, throw ReferenceError.

**Categorization (Doc 730 §XVI.a):** case (1). Cruftless violates the spec; Bun is spec-correct.

**§XV protected_invariants if we were to absorb:** the invariant absorbed is "strict-mode catches typos in identifier writes" — the canonical example being `let foo = 1; ... fooo = 2;` where the typo creates a new global in sloppy mode. Strict mode catches this; cruftless does not, even in modules that opted into strict via the "use strict" prologue. Many npm packages depend on this catch firing (it's the canonical reason strict mode exists).

**Audit verdict:** **DO NOT** promote to §XIV. Same shape as sentry: the absorbed invariant is a guard that defensive code expects to fire. Substrate move: §XII coercion lift in the assign path, gated on strict mode + unresolvable reference → emit ReferenceError.

Note that fixing this in cruftless will likely surface code in older CJS packages that depended on sloppy-mode global creation. The CJS wrapper in cruftless does not (and should not) auto-strict the inner body; bare CJS without "use strict" stays sloppy. Only modules that explicitly opt into strict are affected.

## proxyquire — Bun-specific ESM circular-import strictness

**Bun's error:** `TypeError: Requested module is not instantiated yet.`

This error comes from Bun's ESM linker. Bun's circular-import handling rejects cycles where a re-export is requested before the source module has finished instantiation. The spec (ECMA-262 §16.2.1.6.4 InitializeEnvironment step 11) allows TDZ-bound names in cyclic imports, but Bun's linker is stricter than the spec mandate and throws on patterns that Node and cruftless tolerate.

**Categorization:** case (3) per §XVI.a — both engines diverge from the spec in different directions. Bun is stricter than the spec at the circular-import handling; cruftless conforms to the spec's permissiveness.

**Audit verdict:** **DO NOT** promote to §XIV. This is not a cruftless tolerance worth absorbing — it's already spec-correct. Bun's behavior here is Bun's own implementation choice. No substrate move warranted; cruftless's permissiveness is the right behavior.

This is an interesting reverse-§XIV: a candidate for *Bun's* deviation alphabet if Bun were operating under Doc 730's discipline. Outside the rusty-bun engagement's scope.

## ast-types-flow — Bun-specific BuildMessage on Flow-annotated source

**Bun's error:** `BuildMessage: Unexpected type`

This error comes from Bun's transpiler / builder when it encounters Flow type annotations (`@flow` headers, `: Type` annotations) that it cannot parse. Bun's builder is opinionated about which extensions trigger type-stripping and which do not. Cruftless's parser does not strip types and may treat Flow-annotated source as plain JavaScript (rejecting only the parts that are not valid JS) — or, more likely, the specific arc of code that ast-types-flow exports does not actually require type-stripping at runtime, and cruftless's parser happily parses what survives.

**Categorization:** case (4) per §XVI.a — both engines may be conforming to the spec at the level the spec governs (the spec does not mandate type-stripping; it is a build-tool choice). Bun's BuildMessage is a build-tool diagnostic, not a JavaScript-language diagnostic. Cruftless's load is also conformant at the language level.

**Audit verdict:** **DO NOT** promote to §XIV. The divergence is below the spec-mandated discrimination (implementation freedom of the build tool). No substrate move warranted; both behaviors are admissible.

This is the dual-class's instance of case (4): the §XVI categorization explicitly admits implementation-freedom divergences. Naming it as such is the audit's contribution.

## Summary

```
package          audit verdict
later            §XII fix (strict-mode write-to-undeclared not enforced)
sentry           §XII fix (strict-mode write-to-non-writable not enforced)
proxyquire       no-op (cruftless spec-correct; Bun's choice)
ast-types-flow   no-op (case 4 implementation freedom)
```

**§XV audit result for the dual class: 0 deviation primitives warranted.**

Two of the four are real cruftless engine bugs (case 1) that should be fixed in a future stretch — both touch the assign-op path's strict-mode enforcement. The other two are not cruftless leniency at all (cases 3 and 4): one is Bun's stricter-than-spec choice; the other is build-tool implementation freedom.

This is itself a corroborating instance of Doc 730 §XVI.a's four-case categorization: the §XV audit, run rigorously, separates real deviations from cases that look like deviations but aren't. The dual-class shape on top500 had four candidate deviations; the categorization shrank that set to zero.

## Successor substrate moves (queued for next stretch)

1. **strict-mode write-to-non-writable enforcement** — gate the runtime's Op::SetProp / Op::SetIndex path on strict mode + descriptor writable=false, emit TypeError. Recovers sentry. Likely affects other packages that use freeze-then-write guards.

2. **strict-mode write-to-undeclared enforcement** — gate the assign path on strict mode + unresolvable reference, emit ReferenceError. Recovers later. May surface latent typos in other strict-mode packages.

Both are §XII coercion lifts at the engine substrate, identifiable by their §XVI categorization as case (1). The §XV audit document above is the precondition for landing them; without the audit, a naive fix risks breaking packages that depend on the leniency. The audit's verdict ("no §XIV deviation warranted") permits the §XII fix to proceed safely.

---

*Doc 730 §XV audit, EXT 21 close. Companion to docs/arktype-deep-trace.md.*
