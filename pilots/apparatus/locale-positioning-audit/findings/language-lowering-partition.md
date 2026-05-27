# Language-Lowering Partition — LPA-EXT 5 output

Partition of the current `ast-to-bytecode/language-lowering` matrix bucket into substrate-shaped work arcs.

This file is a follow-on to `resolution-layer-snapshot.md`. That snapshot identified AST-to-bytecode / language lowering as the largest unstratified ECMA-262 pressure surface. This document splits the 10,839-row bucket into families that can become locales, scope extensions, or residual audits without treating the layer as one broad workstream.

Baseline inputs:

- Latest full-suite interpretation: `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-26-140256-p2/interpreted.jsonl`
- Matrix summary: `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-26-140256-p2/matrix.md`
- Resolver bucket: `ast-to-bytecode/language-lowering`
- Bucket size: 10,839 records

---

## I. Surface Marginal Inside Language Lowering

Top surfaces in this resolver bucket:

| Rank | Surface | Count | Initial read |
|---:|---|---:|---|
| 1 | `language.statements.class` | 2,420 | Class declaration elements, class static semantics, private/async/generator lowering residue. |
| 2 | `language.expressions.class` | 2,257 | Class expression sibling of rank 1; should compose with same class-elements arc. |
| 3 | `annexB.language` | 734 | Annex B grammar/runtime interaction, especially block-level function declarations and HTML comments. |
| 4 | `language.statements.for-await-of` | 646 | Async iteration lowering and AsyncFromSync iterator envelope. |
| 5 | `language.expressions.async-generator` | 568 | Async generator body/default-parameter/yield-star semantics. |
| 6 | `language.expressions.object` | 487 | Object literal `__proto__`, computed names, accessor/method semantics. |
| 7 | `language.expressions.dynamic-import` | 296 | Dynamic import lowering / Promise job bridge / parser ambiguity. |
| 8 | `language.statements.async-generator` | 278 | Statement-form sibling of async-generator expression surface. |
| 9 | `language.literals` | 218 | BigInt / RegExp literal residuals; partly lex/parser and partly runtime regexp. |
| 10 | `language.eval-code` | 212 | Direct eval declaration environment and arguments/function declaration semantics. |
| 11 | `language.expressions.compound-assignment` | 212 | Assignment target and operator semantics, some lowering-feature gaps. |
| 12 | `language.statements.with` | 178 | With environment / parse acceptance / multi-statement body residue. |
| 13 | `language.statements.for-of` | 170 | Iterator protocol, assignment/destructuring, abrupt completion. |
| 14 | `language.arguments-object` | 148 | Mapped arguments object and strict/non-strict parameter interaction. |
| 15 | `language.statements.try` | 137 | Completion propagation and destructuring in catch/finally. |
| 16 | `language.expressions.generators` | 121 | Generator default params/yield/arguments semantics. |
| 17 | `language.identifiers` | 115 | Parser/identifier-tokenization residual. |
| 18 | `language.statements.function` | 112 | Function declaration semantics and early errors. |
| 19 | `language.expressions.assignment` | 105 | Assignment target validity and destructuring evaluation. |
| 20 | `language.statements.generators` | 99 | Statement-form sibling of generator expression surface. |

---

## II. Projection Marginal Inside Language Lowering

| Rank | Projection | Count | Initial read |
|---:|---|---:|---|
| 1 | `availability/missing-method-or-intrinsic` | 4,338 | Usually execution helper/runtime substrate absent from lowering output, not literal missing JS methods. Dominated by classes, for-await-of, async generators, object literals. |
| 2 | `value-semantics/wrong-result` | 1,622 | Semantics present but wrong: completion values, closure capture, assignment/control-flow results. |
| 3 | `parser-form/early-error` | 1,324 | Parser/static-semantics sibling pressure embedded in lowering bucket. |
| 4 | `abrupt-completion/throw-missing` | 1,165 | Missing throw propagation or wrong completion tunneling. |
| 5 | `availability/missing-syntax-feature` | 919 | Syntax accepted/missing only at some grammar contexts, especially Annex B, with/for-of, dynamic import. |
| 6 | `uncategorized/projection` | 707 | Needs reason-text mining before substrate spawn. |
| 7 | `availability/missing-global-or-binding` | 319 | Binding-instantiation / declaration-environment issue, not global object availability in most rows. |
| 8 | `realm-prototype/prototype-chain` | 159 | Class/builtin prototype construction and method object realm/prototype. |
| 9 | `availability/missing-lowering-feature` | 88 | Clean bytecode-compiler unsupported-feature rows. |
| 10 | `runner-harness/$262-or-host-hook` | 70 | Measurement residue or harness capability gaps. |

---

## III. Recommended Work Arcs

### Arc A — Class Elements And Class Lowering

Coverage:

- `language.statements.class`: 2,420
- `language.expressions.class`: 2,257
- Combined visible pressure: 4,677 rows

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| class statements + missing method/intrinsic | 1,420 | `language/statements/class/async-gen-method-static/dflt-params-arg-val-not-undefined.js` |
| class expressions + missing method/intrinsic | 1,396 | `language/expressions/class/async-gen-method-static/dflt-params-arg-val-undefined.js` |
| class statements + wrong result | 248 | `language/statements/class/accessor-name-inst-computed-yield-expr.js` |
| class statements + throw missing | 232 | `language/statements/class/accessor-name-inst/computed-err-to-prop-key.js` |
| class expressions + wrong result | 218 | `language/expressions/class/accessor-name-inst-computed-in.js` |
| class statements + uncategorized projection | 206 | `language/statements/class/async-gen-method-static/yield-star-expr-abrupt.js` |
| class expressions + uncategorized projection | 204 | `language/expressions/class/async-gen-method-static/yield-star-getiter-async-get-abrupt.js` |
| class statements + parser early error | 197 | `language/statements/class/async-gen-meth-escaped-async.js` |
| class expressions + throw missing | 170 | `language/expressions/class/accessor-name-inst/computed-err-to-prop-key.js` |
| class expressions + parser early error | 164 | `language/expressions/class/async-gen-method-static/await-as-binding-identifier-escaped.js` |

Existing locale anchors:

- `class-elements-static-semantics/`
- `private-field-runtime-slots/`
- `private-name-lexing/`

Recommended next move:

- Do not spawn a generic class-lowering locale yet.
- First re-run or derive a focused class residual table after `class-elements-static-semantics` and `private-field-runtime-slots` progress. The main partition should split:
  - async class methods,
  - async-generator class methods,
  - computed class element names,
  - accessor/method property-key abrupt completion,
  - private brand/slot semantics,
  - class constructor/prototype realm shape.

### Arc B — Async Iteration And Async Generators

Coverage:

- `language.statements.for-await-of`: 646
- `language.expressions.async-generator`: 568
- `language.statements.async-generator`: 278
- Combined visible pressure: 1,492 rows

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| for-await-of + missing method/intrinsic | 557 | `language/statements/for-await-of/async-gen-decl-dstr-array-elem-init-assignment.js` |
| async-generator expressions + missing method/intrinsic | 399 | `language/expressions/async-generator/dflt-params-arg-val-undefined.js` |
| async-generator statements + missing method/intrinsic | 200 | `language/statements/async-generator/dflt-params-arg-val-not-undefined.js` |
| async-generator expressions + throw missing | 106 | `language/expressions/async-generator/dflt-params-abrupt.js` |
| for-await-of + wrong result | 58 | `language/statements/for-await-of/async-from-sync-iterator-continuation-abrupt-completion-get-constructor.js` |
| async-generator statements + throw missing | 53 | `language/statements/async-generator/dflt-params-abrupt.js` |

Existing locale anchors:

- `for-of-async-lookahead/`
- `iter-protocol-bytecode-rewrite/` (performance-tier sibling, not conformance closure)
- `private-field-runtime-slots/` has a narrow async class-method runner bridge, but not full async-generator semantics.

Recommended next move:

- Candidate: `async-generator-and-for-await-lowering`.
- Baseline inspection must distinguish parser early errors, runner async harness behavior, async generator object protocol, AsyncFromSync iterator wrapping, and abrupt completion propagation.

### Arc C — Annex B Language Semantics

Coverage:

- `annexB.language`: 734

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| Annex B language + missing syntax feature | 474 | `annexB/language/eval-code/direct/func-if-decl-else-decl-a-eval-func-block-scoping.js` |
| Annex B language + missing binding/global | 135 | `annexB/language/eval-code/direct/block-decl-nostrict.js` |
| Annex B language + wrong result | 60 | `annexB/language/comments/multi-line-html-close.js` |
| Annex B language + runner harness residue | 43 | `annexB/language/eval-code/direct/script-decl-lex-no-collision.js` |

Existing locale anchors:

- `annexB-runtime-quirks/` explicitly excludes Annex B grammar/lowering.
- Parser/static-semantics locales cover adjacent shapes but not Annex B block-level function semantics.

Recommended next move:

- Candidate: `annexB-language-semantics`.
- Scope should be language/lowering only:
  - block-level function declarations in sloppy mode,
  - eval/direct-eval declaration instantiation,
  - HTML comment lexical behavior if not already redirected,
  - web-legacy compatibility rules.

### Arc D — Object Literal, Computed Property, And Super

Coverage:

- `language.expressions.object`: 487
- `language.computed-property-names`: 23
- `language.expressions.super`: 46
- Combined visible pressure: 556 rows

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| object expressions + missing method/intrinsic | 201 | `language/expressions/object/11.1.5-0-1.js` |
| object expressions + throw missing | 123 | `language/expressions/object/accessor-name-computed-err-to-prop-key.js` |
| object expressions + wrong result | 67 | `language/expressions/object/__proto__-permitted-dup-shorthand.js` |
| object expressions + parser early error | 61 | `language/expressions/object/__proto__-duplicate.js` |
| super expressions + missing lowering feature | 29 | `language/expressions/super/prop-dot-cls-val-from-eval.js` |

Existing locale anchors:

- `dynamic-import-attributes/` and class/private locales are adjacent but not owners.
- No clear object-literal computed-name locale in current snapshot.

Recommended next move:

- Candidate after sampling: `object-literal-computed-property-semantics`.
- Should separate:
  - `__proto__` duplicate/semantics,
  - computed property-name ToPropertyKey abrupt completion,
  - method/accessor home-object and `super` lowering,
  - object literal method kind metadata.

### Arc E — Dynamic Import And Module-Like Lowering

Coverage:

- `language.expressions.dynamic-import`: 296

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| dynamic import + uncategorized projection | 190 | `language/expressions/dynamic-import/assignment-expression/additive-expr.js` |
| dynamic import + wrong result | 29 | `language/expressions/dynamic-import/catch/nested-arrow-import-catch-eval-rqstd-abrupt-urierror.js` |
| dynamic import + parser early error | 29 | `language/expressions/dynamic-import/catch/nested-arrow-import-catch-instn-iee-err-ambiguous-import.js` |
| dynamic import + missing syntax feature | 24 | `language/expressions/dynamic-import/assignment-expression/await-identifier.js` |
| dynamic import + missing method/intrinsic | 24 | `language/expressions/dynamic-import/catch/nested-async-gen-await-eval-rqstd-abrupt-urierror.js` |

Existing locale anchors:

- `dynamic-import-attributes/`
- module-loader and TS module-loader work in TSR/TXC layers.

Recommended next move:

- Residual audit first. The `uncategorized/projection` share is too high to spawn substrate work from the label alone.
- Likely needs a categorizer refinement or reason-text partition before candidate creation.

### Arc F — Direct Eval, Function Declarations, And Arguments Object

Coverage:

- `language.eval-code`: 212
- `language.arguments-object`: 148
- `language.statements.function`: 112
- `language.expressions.function`: 56
- `language.function-code`: 27
- `language.global-code`: 27
- Combined visible pressure: 582 rows

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| eval-code + parser early error | 142 | `language/eval-code/direct/arrow-fn-a-following-parameter-is-named-arguments-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js` |
| arguments object + missing method/intrinsic | 104 | `language/arguments-object/10.6-12-2.js` |
| function statements + parser early error | 39 | `language/statements/function/13.0-13-s.js` |
| eval-code + wrong result | 31 | `language/eval-code/direct/arrow-fn-body-cntns-arguments-func-decl-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js` |
| function statements + throw missing | 30 | `language/statements/function/13.2-10-s.js` |

Existing locale anchors:

- `strict-binding-eval-arguments/`
- `non-simple-params-strict-body/`
- `promise-executor-functions-meta/` only covers Promise executor function metadata, not general function declaration/eval.

Recommended next move:

- Candidate: `eval-function-arguments-binding-semantics`.
- This should not be mixed with Annex B language semantics unless baseline inspection shows the same binding-instantiation mechanism dominates both.

### Arc G — Assignment, Compound Assignment, And For-Head Targets

Coverage:

- `language.expressions.compound-assignment`: 212
- `language.expressions.assignment`: 105
- `language.statements.for-of`: 170
- `language.statements.for-in`: 23
- `language.statements.for`: 41
- Combined visible pressure: 551 rows

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| compound assignment + wrong result | 99 | `language/expressions/compound-assignment/S11.13.2_A4.10_T1.1.js` |
| for-of + wrong result | 64 | `language/statements/for-of/arguments-mapped-aliasing.js` |
| for-of + missing syntax feature | 51 | `language/statements/for-of/body-dstr-assign-error.js` |
| for-of + throw missing | 50 | `language/statements/for-of/array-key-get-error.js` |
| assignment + wrong result | 46 | `language/expressions/assignment/8.14.4-8-b_1.js` |
| compound assignment + missing syntax feature | 44 | `language/expressions/compound-assignment/S11.13.2_A5.10_T1.js` |

Existing locale anchors:

- `for-head-non-binding-lhs/`
- `for-head-assignment-pattern-validity/`
- `for-of-destructuring-assignment-semantics/`
- `for-of-rhs-is-assignment-expression/`
- `parser-precedence-in-flag/`

Recommended next move:

- Extend existing for-head/for-of locales before spawning a generic assignment locale.
- If a new locale is needed, scope it to `compound-assignment-reference-semantics` after sample inspection.

### Arc H — With / Try / Switch / Completion Records

Coverage:

- `language.statements.with`: 178
- `language.statements.try`: 137
- `language.statements.switch`: 28
- `language.statementList`: 38
- Combined visible pressure: 381 rows

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| with + missing syntax feature | 140 | `language/statements/with/12.10-0-1.js` |
| try + missing binding/global | 34 | `language/statements/try/dstr/ary-name-iter-val.js` |
| try + wrong result | 33 | `language/statements/try/S12.14_A10_T2.js` |
| try + missing syntax feature | 27 | `language/statements/try/S12.14_A14.js` |
| switch + wrong result | 23 | `language/statements/switch/cptn-a-abrupt-empty.js` |
| try + throw missing | 21 | `language/statements/try/dstr/ary-init-iter-get-err-array-prototype.js` |

Existing locale anchors:

- `with-body-multi-statement-parse/`
- `with-unscopables-proxy-has/`
- `var-hoisting-through-try-block/`

Recommended next move:

- Residual audit against current matrix first.
- Likely split `with` environment parsing/execution from completion-record propagation in `try`/`switch`.

### Arc I — Literal And Identifier Residuals

Coverage:

- `language.literals`: 218
- `language.identifiers`: 115
- `language.future-reserved-words`: 26
- Combined visible pressure: 359 rows

Dominant sub-shapes:

| Surface + projection | Count | Example |
|---|---:|---|
| literals + parser early error | 192 | `language/literals/bigint/exponent-part.js` |
| identifiers + parser early error | 115 | `language/identifiers/start-zwj-escaped.js` |
| future-reserved-words + parser early error | 26 | `language/future-reserved-words/class.js` |
| literals + wrong result | 15 | `language/literals/regexp/S7.8.5_A1.3_T5.js` |

Existing locale anchors:

- `numeric-literal-conformance/`
- `identifier-tokenization/`
- `string-literal-and-escape-conformance/`
- `regexp-conformance/`

Recommended next move:

- Do not spawn from language-lowering directly.
- Route residuals to existing tokenization/parser locales after confirming current state against the focused exemplar suites.

---

## IV. Diff-Prod Empirical Cross-Check (2026-05-27)

Empirical triangulation against the partition using a 92-fixture diff-prod suite (42 original + 50 new fixtures targeting the arcs). Fixtures run under both `cruft` and `bun`, diffing stdout byte-for-byte. 55 PASS / 37 FAIL.

### Fixture-to-Arc mapping and results

| Arc | LPA rows | Diff-prod fixtures | PASS | FAIL | PASS rate |
|---|---:|---|---:|---:|---:|
| A — Class elements | 4,677 | `private-field-encapsulation`, `super-new-target`, `computed-property-order` | 1 | 2 | 33% |
| B — Async iter / generators | 1,492 | `generator-suspension`, `destructuring-iterators`, `iterator-close` | 1 | 2 | 33% |
| C — Annex B language | 734 | `directive-prologues`, `hoisting-semantics` | 1 | 1 | 50% |
| D — Object / computed / super | 556 | `computed-property-order`, `super-new-target`, `coercion-pipeline` | 1 | 2 | 33% |
| E — Dynamic import | 296 | (not probed; module-loader layer, not bytecode-observable) | — | — | — |
| F — Eval / function / arguments | 582 | `eval-lexical-capture`, `arguments-object`, `directive-prologues` | 0 | 3 | 0% |
| G — Assignment / for-head | 551 | `reference-semantics`, `for-in-for-of-lowering`, `optional-chaining-nullish` | 0 | 3 | 0% |
| H — With / try / switch / completion | 381 | `with-scoping`, `finally-return-override`, `switch-fallthrough`, `labeled-control-flow` | 2 | 2 | 50% |
| I — Literal / identifier residuals | 359 | `asi-rules`, `unicode-identifiers`, `numeric-literals`, `string-escapes`, `regex-division-ambiguity` | 4 | 1 | 80% |

Additional cross-cutting fixtures not mapped to a single arc:

| Category | Fixtures | PASS | FAIL |
|---|---|---:|---:|
| Abstract operations | `coercion-pipeline`, `abstract-equality`, `abstract-relational`, `samevalue-algorithms` | 2 | 2 |
| Deep engine (MOP) | `proxy-invariants`, `proxy-prototype-chain`, `property-key-order` | 0 | 3 |
| Deep engine (GC/memory) | `weakref-registry`, `atomics-ordering` | 0 | 2 |
| Deep engine (protocol) | `iterator-close`, `symbol-species`, `symbol-toprimitive` | 0 | 3 |
| Deep engine (scope) | `with-scoping` | 0 | 1 |
| Bootstrap host install | `dataview-methods`, `node-util`, `node-assert`, `process-info`, `console-assert`, `buffer-concat`, `readable-from` | 0 | 7 |
| Bytecode lowering | `temporal-dead-zone`, `closure-capture-order`, `arrow-edge-cases`, `expression-precedence` | 2 | 2 |

### Arc-level empirical readings

**Arc A — Class Elements (33% pass rate).** `computed-property-order` passes: object/class computed key evaluation order, Symbol keys, accessor/getter computed names all correct. `private-field-encapsulation` diverges on `#brand in obj` (ergonomic brand check returns false instead of true); encapsulation itself (Object.keys exclusion) is correct. `super-new-target` diverges: `new.target` not propagated through arrow (arrow inherits undefined instead of enclosing constructor's new.target); `this-before-super` does not throw ReferenceError. The class-elements arc's primary gap is super binding machinery (PropagateNewTarget + SetThis) and private brand check, not class syntax parsing.

**Arc B — Async Iteration / Generators (33% pass rate).** `destructuring-iterators` passes fully: generators, Sets, Maps, custom iterables, rest, skip, defaults all unpack via the iterator protocol. `generator-suspension` crashes (exit 70): lazy suspension (bidirectional send, throw-into, return value on done) is not implemented; the engine uses eager-collect. `iterator-close` crashes: IteratorClose/`.return()` protocol not wired for `for-of` break/throw/return, destructuring partial consumption, or yield* delegation close.

**Arc C — Annex B Language (50% pass rate).** `hoisting-semantics` passes: function declaration hoisting, var-undefined-init, fn-overrides-var, var-across-blocks, nested function hoisting all correct. `directive-prologues` diverges: module top-level `this` reports `true` for strict check (Bun reports `false` — Bun binds module `this` to `module.exports` per CJS compat rather than `undefined`). Sloppy-mode `arguments` aliasing returns `42` instead of `99`. Strict eval scope returns `"number"` instead of `"undefined"`.

**Arc D — Object / Computed / Super (33% pass rate).** `computed-property-order` passes fully. `coercion-pipeline` diverges: `${bothObj}` calls `valueOf` (returns `"10"`) instead of `toString` (should return `"ten"` for string hint). This is the ToPrimitive hint dispatch bug, same root as `symbol-toprimitive`'s failure. `super-new-target` fails per Arc A reading.

**Arc F — Eval / Function / Arguments (0% pass rate).** `eval-lexical-capture` crashes (exit 70): direct eval cannot resolve outer `const`/`let` bindings at all. `arguments-object` diverges: `Array.isArray(arguments)` returns `true` (should be `false` — arguments is array-like, not an Array); arrow does not inherit outer `arguments` (returns `0` instead of `3`); sloppy aliasing not implemented. `directive-prologues` diverges per Arc C reading.

**Arc G — Assignment / For-Head (0% pass rate).** `reference-semantics` crashes (exit 70): compound member assignment (`getObj().x += 5`) or related reference resolution fails. `for-in-for-of-lowering` diverges: `for-in` with `delete` during iteration does not skip the deleted key (enumerates `d` despite deletion). `optional-chaining-nullish` diverges: `delete obj?.missing?.prop` returns `false` (should return `true` per spec — delete on undefined reference returns `true`).

**Arc H — With / Try / Switch / Completion (50% pass rate).** `switch-fallthrough` and `labeled-control-flow` pass fully: all fallthrough semantics, default position variants, strict === comparison, nested labeled break/continue, triple-nested labels, for-of labeled break. `with-scoping` crashes (exit 70): `with` statement execution not operational. `finally-return-override` diverges: `finally` block does not execute on `break` or `continue` in a loop (missing `finally-1` in the break log; missing `finally-1` in the continue log).

**Arc I — Literal / Identifier Residuals (80% pass rate).** Highest arc pass rate. `asi-rules`, `unicode-identifiers`, `numeric-literals`, `regex-division-ambiguity` all pass fully. `string-escapes` diverges only on `String.raw` output: the tagged template `String.raw` returns the cooked value instead of the raw backslash-escaped source (same root as `tagged-template-raw`'s `strings.raw` gap).

### Cross-cutting mechanism gaps surfaced

1. **ToPrimitive hint dispatch** (affects Arcs A, D, and cross-cutting abstract ops). `+obj` sends `"default"` hint instead of `"number"` to `[Symbol.toPrimitive]`. Template interpolation `${obj}` dispatches `valueOf` instead of `toString`. Root: the `to_primitive.rs` IR section or its runtime caller does not thread the hint correctly.

2. **IteratorClose protocol** (affects Arcs B, G). `.return()` is never called on break, throw, return, or partial destructuring. The IterClose opcode exists but the compiler does not emit it at the required consuming sites.

3. **Lazy generator suspension** (affects Arc B). The engine uses eager-collect (all yields materialized upfront). Bidirectional send (`next(val)`), `throw()` into suspended generator, and return value on the terminal `{done:true}` step all require coroutine-style suspension.

4. **Direct eval lexical capture** (affects Arc F). `eval("x")` where `x` is an outer `const`/`let` crashes. The eval compiler does not resolve against the enclosing declarative environment record.

5. **Finally on abrupt loop exit** (affects Arc H). `finally` block does not run when control leaves `try` via `break` or `continue`. The TryEnter/TryExit lowering does not account for labeled or unlabeled abrupt completions that bypass the normal completion path.

6. **OrdinaryOwnPropertyKeys ordering** (cross-cutting). `Object.getOwnPropertyNames` returns insertion order instead of integer-indexed-first. The property storage layer does not partition integer-indexed keys from string keys.

7. **Proxy trap invariant enforcement** (cross-cutting MOP). Several §10.5 invariants not checked: non-configurable accessor getter, HasProperty on non-extensible target, [[Delete]] on non-configurable. Only `[[Get]]` on non-configurable non-writable data property is enforced.

8. **Arguments object shape** (affects Arc F). `arguments` is implemented as a plain Array rather than an exotic Arguments object. `Array.isArray(arguments)` returns `true`; sloppy-mode parameter aliasing is absent.

9. **strings.raw on tagged templates** (affects Arcs D, I). The template object passed to tag functions has `raw: undefined` instead of a frozen array of raw source strings.

### Leverage ranking update

Empirical pass rates reorder the candidate ranking from §IV. Arcs with 0% pass rate have the highest unmet pressure:

1. **Arc F — Eval / function / arguments** (0%, 582 rows). Direct eval crash is a hard blocker for any eval-dependent ecosystem code. Arguments-as-Array is a shape violation visible to `Array.isArray` checks.
2. **Arc G — Assignment / for-head** (0%, 551 rows). Compound member assignment crash blocks real-world compound-assignment patterns. for-in delete behavior affects iteration-during-mutation idioms.
3. **Arc B — Async iteration / generators** (33%, 1,492 rows). Generator suspension is the highest-leverage single mechanism: unlocking lazy generators unblocks async generators, for-await-of, and yield* delegation.
4. **Arc A — Class elements** (33%, 4,677 rows). Largest row count but most gaps are in super binding and private brand check, not class syntax.
5. **Arc D — Object / computed / super** (33%, 556 rows). ToPrimitive hint dispatch is the dominant mechanism; the fix propagates across multiple arcs.
6. **Arc H — With / try / switch / completion** (50%, 381 rows). Finally-on-abrupt-loop-exit is the primary gap; switch and labeled control flow are green.
7. **Arc C — Annex B language** (50%, 734 rows). Hoisting passes; sloppy-mode-specific behaviors are the gaps.
8. **Arc I — Literal / identifier residuals** (80%, 359 rows). Nearly green. Only `strings.raw` (shared with tagged-template-raw) diverges.

---

## V. Candidate Ordering

Recommended next arcs by leverage and clarity:

1. **Class residual re-partition**: largest count, but must account for existing class/private-field locales before spawning.
2. **Async generator / for-await lowering**: coherent and large; likely best fresh substrate candidate after baseline inspection.
3. **Annex B language semantics**: coherent and distinct from `annexB-runtime-quirks`.
4. **Object literal / computed-property semantics**: coherent medium-sized cluster with clear spec mechanisms.
5. **Eval/function/arguments binding semantics**: likely high leverage but may overlap Annex B and strict-binding locales.
6. **Assignment/for-head scope extensions**: prefer extending existing locales.
7. **With/try/switch completion residual audit**: medium size, needs split.
8. **Dynamic import residual audit**: high uncategorized share; categorize first.
9. **Literal/identifier residual routing**: route to existing tokenization locales.

---

## V. Immediate Apparatus Recommendation

Create no new substrate locale from the full `ast-to-bytecode/language-lowering` bucket.

The next concrete move should be one of:

1. Run a class residual re-partition after the latest class/private-field work.
2. Spawn or baseline-inspect `async-generator-and-for-await-lowering` if the keeper wants a new substrate arc.
3. Add `annexB-language-semantics` to `CANDIDATES.md` if the keeper wants a clean Annex B language/lowering arc distinct from Annex B runtime.

The partition-before-rank discipline is load-bearing here: rank and count alone would push a generic class/lowering locale, but the data shows multiple resolver-layer mechanisms inside the bucket.

