# T262C failure matrix — 2026-05-24

Source: `/home/jaredef/rusty-bun/results/test262-sample-2026-05-24/results.jsonl`. Total FAIL: **1606** (22.4% of runnable 7182).

## Ranked (pipeline × data-shape) cells

| Rank | Pipeline (structure-axis) | Data-shape (data-axis) | Count | Example test |
|---:|---|---|---:|---|
| 1 | `language.expressions.arrow-function` | `feat:arrow-function;feat:destructuring-assignment;negative:SyntaxError;err:SyntaxError` | 45 | `language/expressions/arrow-function/dstr/syntax-error-ident-ref-break-escaped.js` |
| 2 | `language.statements.for-of` | `feat:destructuring-binding;err:ReferenceError;expected-throw-missing` | 43 | `language/statements/for-of/dstr/array-elem-put-unresolvable-strict.js` |
| 3 | `language.statements.for-of` | `feat:Symbol.iterator;feat:destructuring-binding;err:Test262Error;expected-throw-missing` | 40 | `language/statements/for-of/dstr/array-elem-iter-nrml-close-err.js` |
| 4 | `Object.defineProperty` | `(no-feature-tag)` | 38 | `built-ins/Object/defineProperty/15.2.3.6-4-118.js` |
| 5 | `Array.prototype.sort` | `(no-feature-tag)` | 29 | `built-ins/Array/prototype/sort/bug_596_1.js` |
| 6 | `language.statements.for-of` | `feat:generators;feat:destructuring-binding` | 28 | `language/statements/for-of/dstr/array-elem-init-yield-expr.js` |
| 7 | `language.statements.for-of` | `feat:destructuring-binding;negative:SyntaxError;err:SyntaxError` | 27 | `language/statements/for-of/dstr/array-elem-init-yield-ident-invalid.js` |
| 8 | `language.statements.for-of` | `feat:destructuring-binding` | 27 | `language/statements/for-of/dstr/array-elem-init-yield-ident-valid.js` |
| 9 | `language.statements.for-of` | `negative:SyntaxError;err:SyntaxError` | 24 | `language/statements/for-of/decl-cls.js` |
| 10 | `language.expressions.arrow-function` | `negative:SyntaxError;err:SyntaxError` | 22 | `language/expressions/arrow-function/params-duplicate.js` |
| 11 | `String.prototype.trim` | `(no-feature-tag)` | 22 | `built-ins/String/prototype/trim/15.5.4.20-2-51.js` |
| 12 | `language.statements.for-in` | `negative:SyntaxError;err:SyntaxError` | 17 | `language/statements/for-in/decl-cls.js` |
| 13 | `language.statements.for-of` | `feat:destructuring-binding;err:TypeError;expected-throw-missing` | 16 | `language/statements/for-of/dstr/array-elision-val-bool.js` |
| 14 | `Array.prototype.reduce` | `(no-feature-tag)` | 16 | `built-ins/Array/prototype/reduce/15.4.4.21-10-3.js` |
| 15 | `Array.prototype.indexOf` | `(no-feature-tag)` | 16 | `built-ins/Array/prototype/indexOf/15.4.4.14-10-1.js` |
| 16 | `language.statements.for-of` | `feat:Symbol.iterator;feat:destructuring-binding` | 15 | `language/statements/for-of/dstr/array-elem-iter-nrml-close.js` |
| 17 | `language.statements.for-of` | `(no-feature-tag)` | 14 | `language/statements/for-of/arguments-mapped-aliasing.js` |
| 18 | `Array.prototype.map` | `(no-feature-tag)` | 14 | `built-ins/Array/prototype/map/15.4.4.19-1-15.js` |
| 19 | `RegExp.prototype.test` | `(no-feature-tag)` | 14 | `built-ins/RegExp/prototype/test/S15.10.6.3_A11.js` |
| 20 | `language.expressions.arrow-function` | `feat:destructuring-binding;feat:default-parameters;err:ReferenceError;expected-throw-missing` | 13 | `language/expressions/arrow-function/dstr/dflt-ary-ptrn-elem-id-init-unresolvable.js` |
| 21 | `String.prototype.split` | `(no-feature-tag)` | 13 | `built-ins/String/prototype/split/arguments-are-new-reg-exp-and-hi-and-instance-is-string-hello.js` |
| 22 | `language.statements.for-in` | `(no-feature-tag)` | 13 | `language/statements/for-in/cptn-decl-abrupt-empty.js` |
| 23 | `language.expressions.arrow-function` | `feat:destructuring-binding;err:ReferenceError;expected-throw-missing` | 13 | `language/expressions/arrow-function/dstr/ary-ptrn-elem-id-init-unresolvable.js` |
| 24 | `language.statements.for-of` | `feat:generators` | 12 | `language/statements/for-of/generator-close-via-break.js` |
| 25 | `Promise` | `(no-feature-tag)` | 12 | `built-ins/Promise/exec-args.js` |
| 26 | `language.statements.for-of` | `feat:object-rest;feat:destructuring-binding` | 12 | `language/statements/for-of/dstr/obj-rest-computed-property.js` |
| 27 | `JSON.stringify` | `(no-feature-tag)` | 10 | `built-ins/JSON/stringify/replacer-array-duplicates.js` |
| 28 | `language.statements.for-of` | `feat:Symbol.iterator;feat:generators;feat:destructuring-binding` | 10 | `language/statements/for-of/dstr/array-elem-iter-rtrn-close.js` |
| 29 | `Array.prototype.filter` | `(no-feature-tag)` | 9 | `built-ins/Array/prototype/filter/15.4.4.20-1-15.js` |
| 30 | `language.expressions.arrow-function` | `(no-feature-tag)` | 9 | `language/expressions/arrow-function/arrow/capturing-closure-variables-1.js` |
| 31 | `language.statements.for-of` | `feat:generators;feat:destructuring-binding;err:Test262Error;expected-throw-missing` | 9 | `language/statements/for-of/dstr/const-ary-ptrn-elision-step-err.js` |
| 32 | `Number` | `(no-feature-tag)` | 8 | `built-ins/Number/15.7.4-1.js` |
| 33 | `RegExp.prototype.exec` | `(no-feature-tag)` | 8 | `built-ins/RegExp/prototype/exec/S15.10.6.2_A11.js` |
| 34 | `Object.defineProperty` | `err:TypeError` | 8 | `built-ins/Object/defineProperty/15.2.3.6-4-218.js` |
| 35 | `language.statements.for-of` | `feat:Symbol.iterator;feat:destructuring-binding;err:TypeError;expected-throw-missing` | 8 | `language/statements/for-of/dstr/array-elem-iter-nrml-close-null.js` |
| 36 | `String.prototype.replaceAll` | `feat:String.prototype.replaceAll;feat:Symbol.replace` | 7 | `built-ins/String/prototype/replaceAll/getSubstitution-0x0024-0x003C.js` |
| 37 | `JSON.parse` | `feat:Proxy;err:Test262Error;expected-throw-missing` | 7 | `built-ins/JSON/parse/reviver-array-define-prop-err.js` |
| 38 | `Array.prototype.push` | `err:TypeError;expected-throw-missing` | 7 | `built-ins/Array/prototype/push/length-near-integer-limit-set-failure.js` |
| 39 | `String.prototype.replace` | `(no-feature-tag)` | 7 | `built-ins/String/prototype/replace/length.js` |
| 40 | `language.statements.for-of` | `feat:let;feat:destructuring-binding;err:ReferenceError;expected-throw-missing` | 7 | `language/statements/for-of/dstr/array-elem-init-let.js` |

## Structure-axis marginal (per-pipeline failure counts)

| Pipeline | Failures |
|---|---:|
| `language.statements.for-of` | 346 |
| `language.expressions.arrow-function` | 152 |
| `Object.defineProperty` | 55 |
| `language.statements.for-in` | 38 |
| `Array.prototype.sort` | 34 |
| `Array.prototype.map` | 33 |
| `Array.prototype.filter` | 27 |
| `JSON.stringify` | 27 |
| `String.prototype.replaceAll` | 27 |
| `Array.prototype.concat` | 26 |
| `Array.prototype.reduce` | 23 |
| `Array.prototype.splice` | 23 |
| `String.prototype.split` | 22 |
| `String.prototype.trim` | 22 |
| `Array.prototype.indexOf` | 21 |
| `JSON.parse` | 20 |
| `Object.assign` | 19 |
| `String.prototype.replace` | 19 |
| `Array.prototype.slice` | 18 |
| `RegExp.prototype.test` | 18 |
| `Map` | 17 |
| `Promise` | 17 |
| `Map.prototype.getOrInsertComputed` | 15 |
| `WeakMap` | 15 |
| `Array.prototype.forEach` | 14 |
| `WeakMap.prototype.getOrInsertComputed` | 14 |
| `RegExp.prototype.exec` | 13 |
| `Set.prototype.intersection` | 13 |
| `Set.prototype.isDisjointFrom` | 13 |
| `Set.prototype.difference` | 12 |

## Data-axis marginal (per-feature failure counts)

| Data-shape | Failures |
|---|---:|
| `(no-feature-tag)` | 363 |
| `negative:SyntaxError;err:SyntaxError` | 63 |
| `err:TypeError;expected-throw-missing` | 62 |
| `feat:destructuring-binding;err:ReferenceError;expected-throw-missing` | 56 |
| `feat:arrow-function;feat:destructuring-assignment;negative:SyntaxError;err:SyntaxError` | 45 |
| `feat:Symbol.iterator;feat:destructuring-binding;err:Test262Error;expected-throw-missing` | 44 |
| `feat:set-methods;not-callable` | 34 |
| `feat:generators;feat:destructuring-binding` | 32 |
| `feat:resizable-arraybuffer` | 32 |
| `feat:set-methods;err:TypeError;expected-throw-missing` | 28 |
| `feat:destructuring-binding` | 27 |
| `feat:destructuring-binding;negative:SyntaxError;err:SyntaxError` | 27 |
| `err:TypeError` | 26 |
| `feat:Symbol.species` | 24 |
| `err:Test262Error;expected-throw-missing` | 22 |
| `feat:set-methods` | 20 |
| `feat:destructuring-binding;err:TypeError;expected-throw-missing` | 18 |
| `feat:Symbol` | 17 |
| `feat:Symbol.iterator;feat:destructuring-binding` | 16 |
| `feat:Symbol.species;err:TypeError;expected-throw-missing` | 16 |
| `feat:Symbol.iterator;err:Test262Error;expected-throw-missing` | 13 |
| `feat:Symbol;err:TypeError;expected-throw-missing` | 13 |
| `feat:destructuring-binding;feat:default-parameters;err:ReferenceError;expected-throw-missing` | 13 |
| `feat:generators` | 12 |
| `feat:generators;feat:destructuring-binding;err:Test262Error;expected-throw-missing` | 12 |
| `feat:object-rest;feat:destructuring-binding` | 12 |
| `feat:Proxy;err:Test262Error;expected-throw-missing` | 11 |
| `feat:Symbol.iterator;feat:generators;feat:destructuring-binding` | 11 |
| `feat:cross-realm;feat:Symbol.species` | 11 |
| `feat:await-dictionary` | 10 |
| `feat:json-parse-with-source` | 10 |
| `feat:Proxy;err:TypeError;expected-throw-missing` | 9 |
| `feat:Symbol.iterator;feat:destructuring-binding;err:TypeError;expected-throw-missing` | 9 |
| `feat:Symbol.toStringTag` | 9 |
| `feat:WeakMap;feat:upsert;not-callable` | 9 |
| `err:Test262Error` | 8 |
| `feat:Proxy` | 8 |
| `feat:String.prototype.replaceAll;feat:Symbol.replace` | 7 |
| `feat:Symbol.iterator` | 7 |
| `feat:Symbol.species;err:Test262Error;expected-throw-missing` | 7 |
