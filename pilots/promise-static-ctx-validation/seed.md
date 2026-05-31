# promise-static-ctx-validation (PSCV) — Seed

## Telos

```
runtime/promise-static :: E3/intrinsic-object:ecma-262 :: receiver-validation :: TypeError-if-this-not-Object
```

at Promise.{all, race, any, allSettled, resolve, reject} static methods. ECMA-262 §27.2.4.1/2/3/5/7 step 1-2 mandates: "Let C be the this value. If Type(C) is not Object, throw a TypeError exception." Prior impl silently substituted the default Promise when `this` was non-Object/non-callable; spec violation across 6 statics × ~6 primitive cases per static = 36 test262 cells.

## Origin

Founded 2026-05-31 per keeper Telegram 10709 ("A") authorizing Promise-cluster deep-dive. Surfaced by test262 sample run 2026-05-31 fail bucket analysis: `ctx-non-object.js` appears in 5 of the 6 Promise-static fail directories.

## Work shape

Single substrate move at each of the 6 registration closures: insert `Object` check on `current_this()` before delegating to the generated *_via path. ~6 LOC per method × 6 = ~36 LOC.

## Carve-outs

- **Subclass case (ctx-ctor)**: out of scope. Promise.all.call(SubPromise, []) should return a SubPromise instance, not a Promise. Requires routing new_promise through the subclass's NewPromiseCapability. Sibling rung PSCV-EXT 1.

## Composes-with

- AFID-EXT 0 (substrate prefix discipline)
- `apparatus/docs/predictive-ruleset.md` Rule 17 (per-method segmentation)
