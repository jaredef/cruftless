# T262C failure matrix — 2026-05-25

Source: `results/test262-sample-2026-05-25/results.jsonl`. Total FAIL: **1291** (17.7% of runnable 7288).

## Ranked (pipeline × data-shape) cells

| Rank | Pipeline (structure-axis) | Data-shape (data-axis) | Count | Example test |
|---:|---|---|---:|---|
| 1 | `Object.defineProperty` | `(no-feature-tag)` | 37 | `built-ins/Object/defineProperty/15.2.3.6-4-124.js` |
| 2 | `language.statements.for-of` | `feat:destructuring-binding;negative:SyntaxError;err:SyntaxError` | 27 | `language/statements/for-of/dstr/array-elem-init-yield-ident-invalid.js` |
| 3 | `Array.prototype.sort` | `(no-feature-tag)` | 26 | `built-ins/Array/prototype/sort/bug_596_2.js` |
| 4 | `language.statements.for-of` | `feat:generators;feat:destructuring-binding` | 22 | `language/statements/for-of/dstr/array-elem-init-yield-expr.js` |
| 5 | `language.expressions.arrow-function` | `negative:SyntaxError;err:SyntaxError` | 19 | `language/expressions/arrow-function/rest-params-trailing-comma-early-error.js` |
| 6 | `language.statements.for-of` | `feat:destructuring-binding` | 18 | `language/statements/for-of/dstr/array-elem-nested-array-yield-ident-valid.js` |
| 7 | `language.statements.for-in` | `(no-feature-tag)` | 18 | `language/statements/for-in/cptn-decl-abrupt-empty.js` |
| 8 | `Array.prototype.reduce` | `(no-feature-tag)` | 16 | `built-ins/Array/prototype/reduce/15.4.4.21-10-3.js` |
| 9 | `Array.prototype.indexOf` | `(no-feature-tag)` | 16 | `built-ins/Array/prototype/indexOf/15.4.4.14-10-1.js` |
| 10 | `Array.prototype.map` | `(no-feature-tag)` | 14 | `built-ins/Array/prototype/map/15.4.4.19-1-15.js` |
| 11 | `language.statements.for-of` | `(no-feature-tag)` | 13 | `language/statements/for-of/arguments-mapped-aliasing.js` |
| 12 | `RegExp.prototype.test` | `(no-feature-tag)` | 13 | `built-ins/RegExp/prototype/test/S15.10.6.3_A11.js` |
| 13 | `language.statements.for-of` | `feat:generators` | 12 | `language/statements/for-of/generator-close-via-break.js` |
| 14 | `language.statements.for-of` | `negative:SyntaxError;err:SyntaxError` | 12 | `language/statements/for-of/head-const-bound-names-dup.js` |
| 15 | `language.statements.for-of` | `feat:object-rest;feat:destructuring-binding` | 12 | `language/statements/for-of/dstr/obj-rest-computed-property-no-strict.js` |
| 16 | `language.expressions.arrow-function` | `(no-feature-tag)` | 12 | `language/expressions/arrow-function/arrow/binding-tests-3.js` |
| 17 | `Promise` | `(no-feature-tag)` | 12 | `built-ins/Promise/exec-args.js` |
| 18 | `String.prototype.split` | `(no-feature-tag)` | 11 | `built-ins/String/prototype/split/arguments-are-new-reg-exp-and-hi-and-instance-is-string-hello.js` |
| 19 | `language.statements.for-of` | `feat:Symbol.iterator;feat:destructuring-binding;err:Test262Error;expected-throw-missing` | 11 | `language/statements/for-of/dstr/array-elem-iter-thrw-close-err.js` |
| 20 | `JSON.stringify` | `(no-feature-tag)` | 10 | `built-ins/JSON/stringify/replacer-array-duplicates.js` |
| 21 | `language.statements.for-of` | `feat:Symbol.iterator;feat:generators;feat:destructuring-binding` | 10 | `language/statements/for-of/dstr/array-elem-iter-rtrn-close-null.js` |
| 22 | `language.statements.for-of` | `feat:generators;feat:destructuring-binding;err:Test262Error;expected-throw-missing` | 9 | `language/statements/for-of/dstr/const-ary-ptrn-elision-step-err.js` |
| 23 | `Array.prototype.filter` | `(no-feature-tag)` | 9 | `built-ins/Array/prototype/filter/15.4.4.20-1-15.js` |
| 24 | `Object.defineProperty` | `err:TypeError` | 8 | `built-ins/Object/defineProperty/15.2.3.6-4-218.js` |
| 25 | `language.statements.for-of` | `feat:Symbol.iterator;feat:destructuring-binding;err:TypeError;expected-throw-missing` | 8 | `language/statements/for-of/dstr/array-elem-iter-nrml-close-null.js` |
| 26 | `Number` | `(no-feature-tag)` | 8 | `built-ins/Number/15.7.4-1.js` |
| 27 | `language.statements.for-of` | `feat:destructuring-binding;err:TypeError;expected-throw-missing` | 8 | `language/statements/for-of/dstr/const-obj-init-null.js` |
| 28 | `RegExp.prototype.exec` | `(no-feature-tag)` | 7 | `built-ins/RegExp/prototype/exec/S15.10.6.2_A11.js` |
| 29 | `language.statements.for-of` | `feat:let;feat:destructuring-binding;err:ReferenceError;expected-throw-missing` | 7 | `language/statements/for-of/dstr/array-elem-init-let.js` |
| 30 | `String.prototype.replaceAll` | `feat:String.prototype.replaceAll;feat:Symbol.replace` | 7 | `built-ins/String/prototype/replaceAll/getSubstitution-0x0024-0x003C.js` |
| 31 | `Array.prototype.push` | `err:TypeError;expected-throw-missing` | 7 | `built-ins/Array/prototype/push/length-near-integer-limit-set-failure.js` |
| 32 | `JSON.parse` | `feat:Proxy;err:Test262Error;expected-throw-missing` | 7 | `built-ins/JSON/parse/reviver-array-delete-err.js` |
| 33 | `Object.create` | `(no-feature-tag)` | 6 | `built-ins/Object/create/15.2.3.5-4-11.js` |
| 34 | `String.prototype.replace` | `(no-feature-tag)` | 6 | `built-ins/String/prototype/replace/length.js` |
| 35 | `language.statements.for-in` | `negative:SyntaxError;err:SyntaxError` | 6 | `language/statements/for-in/head-const-bound-names-let.js` |
| 36 | `Map` | `feat:Symbol.iterator;err:Test262Error;expected-throw-missing` | 6 | `built-ins/Map/iterator-close-after-set-failure.js` |
| 37 | `Array.prototype.forEach` | `(no-feature-tag)` | 6 | `built-ins/Array/prototype/forEach/15.4.4.18-1-15.js` |
| 38 | `WeakMap.prototype.getOrInsertComputed` | `feat:WeakMap;feat:upsert;not-callable` | 5 | `built-ins/WeakMap/prototype/getOrInsertComputed/adds-object-element.js` |
| 39 | `Promise.allKeyed` | `feat:await-dictionary` | 5 | `built-ins/Promise/allKeyed/extensible.js` |
| 40 | `Array.prototype.pop` | `err:TypeError;expected-throw-missing` | 5 | `built-ins/Array/prototype/pop/set-length-array-is-frozen.js` |

## Structure-axis marginal (per-pipeline failure counts)

| Pipeline | Failures |
|---|---:|
| `language.statements.for-of` | 221 |
| `language.expressions.arrow-function` | 70 |
| `Object.defineProperty` | 54 |
| `language.statements.for-in` | 33 |
| `Array.prototype.sort` | 31 |
| `Array.prototype.map` | 27 |
| `String.prototype.replaceAll` | 26 |
| `JSON.stringify` | 25 |
| `Array.prototype.concat` | 24 |
| `Array.prototype.reduce` | 22 |
| `Array.prototype.filter` | 21 |
| `Array.prototype.indexOf` | 21 |
| `JSON.parse` | 20 |
| `Object.assign` | 19 |
| `String.prototype.split` | 19 |
| `Map` | 17 |
| `Promise` | 17 |
| `String.prototype.replace` | 17 |
| `RegExp.prototype.test` | 16 |
| `Array.prototype.splice` | 15 |
| `Map.prototype.getOrInsertComputed` | 15 |
| `WeakMap` | 15 |
| `WeakMap.prototype.getOrInsertComputed` | 14 |
| `Array.prototype.forEach` | 13 |
| `Array.prototype.includes` | 11 |
| `Error` | 11 |
| `Object.fromEntries` | 11 |
| `RegExp.prototype.exec` | 11 |
| `Set.prototype.intersection` | 11 |
| `Set.prototype.isDisjointFrom` | 11 |

## Data-axis marginal (per-feature failure counts)

| Data-shape | Failures |
|---|---:|
| `(no-feature-tag)` | 334 |
| `err:TypeError;expected-throw-missing` | 60 |
| `negative:SyntaxError;err:SyntaxError` | 37 |
| `feat:set-methods;not-callable` | 34 |
| `feat:resizable-arraybuffer` | 32 |
| `feat:destructuring-binding;negative:SyntaxError;err:SyntaxError` | 27 |
| `feat:generators;feat:destructuring-binding` | 26 |
| `err:Test262Error;expected-throw-missing` | 22 |
| `err:TypeError` | 22 |
| `feat:set-methods` | 20 |
| `feat:Symbol.species` | 18 |
| `feat:destructuring-binding` | 18 |
| `feat:Symbol` | 17 |
| `feat:set-methods;err:TypeError;expected-throw-missing` | 14 |
| `feat:Symbol.iterator;err:Test262Error;expected-throw-missing` | 13 |
| `feat:Symbol;err:TypeError;expected-throw-missing` | 13 |
| `feat:generators` | 12 |
| `feat:generators;feat:destructuring-binding;err:Test262Error;expected-throw-missing` | 12 |
| `feat:object-rest;feat:destructuring-binding` | 12 |
| `feat:Proxy;err:Test262Error;expected-throw-missing` | 11 |
| `feat:Symbol.iterator;feat:destructuring-binding;err:Test262Error;expected-throw-missing` | 11 |
| `feat:Symbol.iterator;feat:generators;feat:destructuring-binding` | 11 |
| `feat:cross-realm;feat:Symbol.species` | 11 |
| `feat:await-dictionary` | 10 |
| `feat:destructuring-binding;err:TypeError;expected-throw-missing` | 10 |
| `feat:json-parse-with-source` | 10 |
| `feat:Symbol.iterator;feat:destructuring-binding;err:TypeError;expected-throw-missing` | 9 |
| `feat:Symbol.toStringTag` | 9 |
| `feat:WeakMap;feat:upsert;not-callable` | 9 |
| `err:Test262Error` | 8 |
| `feat:Proxy` | 8 |
| `feat:Symbol.species;err:TypeError;expected-throw-missing` | 8 |
| `feat:String.prototype.replaceAll;feat:Symbol.replace` | 7 |
| `feat:Symbol.iterator` | 7 |
| `feat:cross-realm;feat:Reflect` | 7 |
| `feat:let;feat:destructuring-binding;err:ReferenceError;expected-throw-missing` | 7 |
| `feat:upsert` | 7 |
| `feat:Proxy;feat:Symbol.species` | 6 |
| `feat:Symbol;feat:WeakMap;feat:symbols-as-weakmap-keys;feat:upsert;not-callable` | 6 |
| `err:ReferenceError;expected-throw-missing` | 5 |
