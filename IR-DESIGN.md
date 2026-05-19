# IR-between-ECMA-and-runtime — draft design

Author: 2026-05-19 session (post-P62.E25). Composes with seed §A8.28–§A8.32.

## 1. Motivation

The 2026-05-18 → 2026-05-19 P62.E1→E25 stretch closed ~30 substrate-tier conformance bugs across eight test262 chapters (slice-level lifts of 10–50 pp). The honest pattern in those fixes:

> **Most of the 30+ substrate fixes were *transcription errors* between ECMA prose and ad-hoc Rust.**

Examples:
- §22.1.3.7 String.prototype.includes step 4: "If IsRegExp(searchString) is true, throw a TypeError" — missed in cruftless's includes impl for years, surfaced by test262 in P62.E13.
- §27.2.3.1 Promise step 1: "If NewTarget is undefined, throw a TypeError" — missed in cruftless's Promise ctor, fixed in P62.E24.
- §10.2.4 Function constructor `.prototype` descriptor `{writable: true, enumerable: false, configurable: false}` — cruftless installed it frozen (P61.E20 collateral), broke Con.prototype-style chains, fixed in P62.E7.
- §6.2.5.5 ToPropertyDescriptor steps 5/7: "If has property 'get' and get is not callable and not undefined, throw TypeError" — missed in Object.defineProperty for years, fixed in P62.E12.

ECMA's spec is itself written in a remarkably regular IR — numbered steps, named abstract operations, explicit throw classes, explicit dispatch order. Every gap above was a manual transcription error between that regular IR and ad-hoc Rust.

**Claim**: an IR layer that mirrors ECMA's spec structure 1:1 would catch these errors at IR-construction time, before they reach the runtime.

## 2. Architecture

```
ECMA spec algorithm (XML)
        ↓ (parse)
   IR function
        ↓ (lint vs spec — Section 6)
   IR function (validated)
        ↓ (lower — Section 5)
   Rust function
        ↓ (compile)
   Existing rusty-js runtime helpers
        ↓
   Existing bytecode VM
```

The IR is a **compile-time source-of-truth artifact**, not a runtime layer. The bytecode VM stays unchanged. The IR adds a discipline-enforcement layer between the spec and the Rust callsites.

**Scope honestly delimited**: the IR covers the *spec-built-in-method* surface (Array.prototype.*, Object.*, Number.*, Promise.*, RegExp.*, etc.). It does NOT cover user JS — user code still goes through parse → bytecode-compile → bytecode-VM. The IR is for spec-encoded operations, not for user code.

## 3. Node set

~50 nodes in 9 categories. Each lowers 1:1 to a Rust expression or statement using the existing Runtime helpers.

### 3.1 Coercion / type-check (§A8.29)

| node | spec ref | lowering |
|---|---|---|
| `RequireObjectCoercible(v)` | §7.2.1 | `rt.require_object_coercible(&v)?` |
| `ToObject(v)` | §7.1.18 | `rt.to_object(&v)?` |
| `ToPrimitive(v, hint)` | §7.1.1 | `rt.to_primitive(&v, hint)?` |
| `ToString(v)` | §7.1.17 | `rt.to_string_strict(&v)?` |
| `ToNumber(v)` | §7.1.4 | `rt.coerce_to_number(&v)?` |
| `ToInteger(v)` | §7.1.5 | `rt.to_integer(&v)?` |
| `ToLength(v)` | §7.1.20 | `rt.to_length(&v)?` |
| `ToUint32(v)` | §7.1.7 | `rt.to_uint32(&v)?` |
| `ToBoolean(v)` | §7.1.2 | `abstract_ops::to_boolean(&v)` |
| `ToPropertyKey(v)` | §7.1.19 | `rt.to_property_key(&v)?` |
| `IsCallable(v)` | §7.2.4 | `rt.is_callable(&v)` |
| `IsConstructor(v)` | §7.2.5 | `rt.is_constructor(&v)` |
| `IsArray(v)` | §7.2.2 | `rt.is_array(&v)` |
| `IsRegExp(v)` | §7.2.8 | `rt.is_regexp(&v)?` |
| `SameValue(a, b)` | §7.2.10 | `abstract_ops::same_value(&a, &b)` |
| `SameValueZero(a, b)` | §7.2.11 | `abstract_ops::same_value_zero(&a, &b)` |

### 3.2 Slot / property (§A8.28 + §A8.30)

| node | spec ref | lowering |
|---|---|---|
| `HasSlot(O, [[Slot]])` | (engine) | direct check on InternalKind / properties |
| `GetSlot(O, [[Slot]])` | (engine) | direct read; **auto-emits TypeError if absent** |
| `Get(O, P)` | §7.3.2 | `rt.read_property(O, P)?` |
| `GetV(V, P)` | §7.3.3 | `rt.get_v(&V, P)?` |
| `Set(O, P, V, Throw)` | §7.3.4 | `rt.set_property(O, P, V, Throw)?` |
| `HasProperty(O, P)` | §7.3.10 | `rt.has_property(O, P)` |
| `HasOwnProperty(O, P)` | §7.3.11 | `rt.has_own_property(O, P)` |
| `OrdinaryObjectCreate(proto, slots)` | §10.1.13 | `rt.ordinary_object_create(proto, slots)` |
| `OrdinaryDefineOwnProperty(O, P, Desc)` | §10.1.6.1 | `rt.ordinary_define_own_property(O, P, Desc)?` |
| `CreateDataPropertyOrThrow(O, P, V)` | §7.3.6 | `rt.create_data_property_or_throw(O, P, V)?` |
| `DeletePropertyOrThrow(O, P)` | §7.3.9 | `rt.delete_property_or_throw(O, P)?` |
| `GetMethod(O, P)` | §7.3.10 | `rt.get_method(O, P)?` |

### 3.3 Calls (§A8.32 extended)

| node | spec ref | lowering |
|---|---|---|
| `Call(F, this, args)` | §7.3.13 | IsCallable check + `rt.call_function(F, this, args)?` |
| `Construct(C, args, newTarget?)` | §7.3.14 | IsConstructor check + construct |
| `Invoke(O, P, args)` | §7.3.15 | `GetV(O, P)` + `Call(_, O, args)` |

### 3.4 Control flow

| node | semantics |
|---|---|
| `Throw(class, msg)` | `return Err(RuntimeError::class(msg.into()))` where class ∈ {TypeError, RangeError, ReferenceError, SyntaxError} |
| `Return(v)` | `return Ok(v)` |
| `ReturnIfAbrupt(v)` | implicit via `?` on every lowering (spec's `?` prefix) |
| `If(cond, then, else)` | direct Rust `if` |
| `Loop(cond, body)` | direct Rust `while` |
| `Break` / `Continue` | direct |

### 3.5 Constants

`Undefined` / `Null` / `True` / `False` / `Number(n)` / `String(s)` / `BigInt(b)` / `Symbol(s)` — Rust literal Values.

### 3.6 Operators (§A8.32)

| node | lowering |
|---|---|
| `OpAdd(l, r)` | `rt.op_add_rt(&l, &r)?` |
| `OpSub(l, r)` / `OpMul(l, r)` / etc. | `rt.op_numeric_rt(&l, &r, Op)?` |
| `LooseEq(a, b)` | `rt.is_loosely_equal_rt(&a, &b)?` |
| `StrictEq(a, b)` | `abstract_ops::is_strictly_equal(&a, &b)` |
| `RelCompare(a, b, ordering)` | `rt.abstract_relational_compare_rt(&a, &b, ordering)?` |

### 3.7 Property descriptors (§6.2.5.x)

| node | lowering |
|---|---|
| `ToPropertyDescriptor(O)` | `rt.to_property_descriptor(O)?` — validates per §6.2.5.5 |
| `CompletePropertyDescriptor(Desc)` | `desc.complete()` |
| `IsAccessorDescriptor(Desc)` | `desc.is_accessor()` |
| `IsDataDescriptor(Desc)` | `desc.is_data()` |
| `IsGenericDescriptor(Desc)` | `desc.is_generic()` |
| `FromPropertyDescriptor(Desc)` | builds the public descriptor Object |

### 3.8 Iteration (§7.4)

| node | lowering |
|---|---|
| `GetIterator(O, hint)` | `rt.get_iterator(&O, hint)?` |
| `IteratorNext(iter)` | `rt.iterator_next(iter)?` |
| `IteratorComplete(result)` | `rt.iterator_complete(result)?` |
| `IteratorValue(result)` | `rt.iterator_value(result)?` |
| `IteratorClose(iter, completion)` | `rt.iterator_close(iter, completion)?` |

### 3.9 Realm / species

| node | lowering |
|---|---|
| `ArraySpeciesCreate(O, length)` | `rt.array_species_create(O, length)?` |
| `SpeciesConstructor(O, default)` | `rt.species_constructor(O, default)?` |
| `NewPromiseCapability(C)` | `rt.new_promise_capability(C)?` |

## 4. Worked example: §23.1.3.20 Array.prototype.map

### 4.1 Spec text (ECMA-262)

```
1. Let O be ? ToObject(this value).
2. Let len be ? LengthOfArrayLike(O).
3. If IsCallable(callbackfn) is false, throw a TypeError exception.
4. Let A be ? ArraySpeciesCreate(O, len).
5. Let k be 0.
6. Repeat, while k < len,
   a. Let Pk be ! ToString(𝔽(k)).
   b. Let kPresent be ? HasProperty(O, Pk).
   c. If kPresent is true, then
      i. Let kValue be ? Get(O, Pk).
      ii. Let mappedValue be ? Call(callbackfn, thisArg, « kValue, 𝔽(k), O »).
      iii. Perform ? CreateDataPropertyOrThrow(A, Pk, mappedValue).
   d. Set k to k + 1.
7. Return A.
```

`?` = ReturnIfAbrupt (propagate throw). `!` = no abrupt completion possible.

### 4.2 IR translation (1:1 with spec steps)

```
function Array.prototype.map(this, args) ; spec=23.1.3.20
  callbackfn := args[0] ?? Undefined
  thisArg    := args[1] ?? Undefined

  O   := ToObject(this)                  ; step 1
  len := LengthOfArrayLike(O)            ; step 2
  if not IsCallable(callbackfn):         ; step 3
    Throw(TypeError, "callback must be callable")
  A   := ArraySpeciesCreate(O, len)      ; step 4
  k   := 0                               ; step 5
  while k < len:                         ; step 6
    Pk := ToString(Number(k))            ;   6a (!)
    if HasProperty(O, Pk):               ;   6b/6c
      kValue := Get(O, Pk)               ;     6c.i
      mapped := Call(callbackfn,
                     thisArg,
                     [kValue, Number(k), O])
                                         ;     6c.ii
      CreateDataPropertyOrThrow(A, Pk, mapped)
                                         ;     6c.iii
    k := k + 1                           ;   6d
  Return(A)                              ; step 7
```

### 4.3 Lowering to Rust

```rust
fn array_prototype_map(rt: &mut Runtime, this: Value, args: &[Value])
    -> Result<Value, RuntimeError>
{
    let callbackfn = args.first().cloned().unwrap_or(Value::Undefined);
    let this_arg  = args.get(1).cloned().unwrap_or(Value::Undefined);

    let o   = rt.to_object(&this)?;                                 // step 1
    let len = rt.length_of_array_like(o)?;                          // step 2
    if !rt.is_callable(&callbackfn) {                               // step 3
        return Err(RuntimeError::TypeError(
            "Array.prototype.map: callback is not callable".into()));
    }
    let a = rt.array_species_create(o, len)?;                       // step 4
    let mut k: usize = 0;                                           // step 5
    while k < len {                                                 // step 6
        let pk = k.to_string();                                     //   6a
        if rt.has_property(o, &pk) {                                //   6b/6c
            let k_value = rt.read_property(o, &pk)?;                //     6c.i
            let mapped = rt.call_function(callbackfn.clone(),
                this_arg.clone(),
                vec![k_value, Value::Number(k as f64), Value::Object(o)])?;
                                                                    //     6c.ii
            rt.create_data_property_or_throw(a, &pk, mapped)?;      //     6c.iii
        }
        k += 1;                                                     //   6d
    }
    Ok(Value::Object(a))                                            // step 7
}
```

Every spec step has exactly one IR statement and exactly one Rust line. The `?` operator at each abrupt-emitting call lowers ECMA's `?` prefix automatically.

## 5. §A8 disciplines encoded by the IR

| discipline | IR encoding | bug class eliminated |
|---|---|---|
| §A8.28 descriptor-shape | `OrdinaryObjectCreate(proto, slots)` + `OrdinaryDefineOwnProperty(O, P, Desc)` take typed Desc records. Three lowering variants (set_own / set_own_internal / set_own_frozen) auto-selected from Desc shape. | "leaked {w:true, e:true, c:true} default into spec-tight context" (P61.E19/E20/P62.E2/E3/E7) |
| §A8.29 abstract-ops duality | IR has only the *dispatching* form. The pure-primitive form is a lowering optimization the compiler may select. No callsite can pick the wrong form. | "called abstract_ops::to_string when receiver might be Object" (every E13/E14/E15/E17 fix) |
| §A8.30 brand-check | `GetSlot(O, [[XData]])` auto-emits TypeError when slot is absent. Discipline is the operator's semantics, not a separate check. | "silently fell through on missing internal slot" (P62.E19/E19.2/E23/E24) |
| §A8.31 SyntaxError canonical | `Throw(class, msg)` takes class as typed enum variant. Can't typo TypeError for SyntaxError. | "emitted SyntaxError-shaped error as TypeError" (P62.E22) |
| §A8.32 ToPrimitive at operator | `OpAdd` / `LooseEq` / `StrictEq` are IR nodes. Lowering routes through op_add_rt / is_loosely_equal_rt. | "called abstract_ops::op_add on Object operand" (P62.E21) |

## 6. Linter design — spec-vs-IR diff

ECMA-262 publishes its algorithms as parseable XML (the `<emu-alg>` markup in the [Specification source](https://github.com/tc39/ecma262)). Each algorithm step has a stable identifier (`<emu-alg-step id="...">`).

### 6.1 The diff algorithm

```
Input:
  spec_section: ECMA section reference (e.g. "23.1.3.20")
  ir_function:  IR translation of that section

Procedure:
1. Parse spec_section's <emu-alg> to a list of canonical step records:
     [{step_id: "1", text: "Let O be ? ToObject(this value)", calls: ["ToObject"]},
      {step_id: "2", text: "Let len be ? LengthOfArrayLike(O)",
       calls: ["LengthOfArrayLike"]},
      ...]
2. Walk ir_function's statements in order.
3. For each (spec_step, ir_stmt) pair:
   a. Assert spec_step.calls ⊆ ir_stmt's IR node names
      (the IR must invoke every abstract op the spec step names).
   b. Assert ir_stmt has no IR nodes outside spec_step.calls
      (the IR must not invoke extra abstract ops the spec doesn't name).
   c. Assert spec_step.throws == ir_stmt.throws
      (if spec says "throw TypeError", IR must Throw(TypeError, _)).
4. Report any drift: step-count mismatch, step-order mismatch,
   per-step IR-op-vs-spec-op mismatch, throw-class mismatch.
```

### 6.2 Drift detection in practice

The 30+ substrate fixes from this session would have surfaced as:

| spec section | drift class | linter output (hypothetical) |
|---|---|---|
| §22.1.3.7 step 4 (String.includes) | missing step | "IR for §22.1.3.7 step 4 lacks IsRegExp(searchString) call" |
| §27.2.3.1 step 1 (Promise ctor) | missing step | "IR for §27.2.3.1 step 1 lacks IsConstructor(NewTarget) check" |
| §10.2.4 (Function.prototype) | wrong descriptor | "IR for §10.2.4 produces {w:false} but spec produces {w:true}" |
| §6.2.5.5 step 5/7 (ToPropertyDescriptor) | missing step | "IR for §6.2.5.5 step 5 lacks IsCallable(get) check" |
| §25.5.1 (JSON.parse) | wrong throw class | "IR for §25.5.1.1 throws TypeError; spec throws SyntaxError" |

### 6.3 What the linter can't catch

- **Algorithmic correctness** — the linter checks that you *call* the named abstract ops; it doesn't check that your bytecode-VM implements them correctly.
- **Performance** — IR fidelity says nothing about lowering efficiency.
- **Spec ambiguity** — when the spec itself has a known issue (cf. tc39/ecma262 errata), the linter follows the published text.

The linter is a *necessary* condition for spec conformance, not a *sufficient* one. Test262 is still the ground-truth check.

## 7. Implementation plan

### 7.1 Tier 1 — minimal viable IR

1. Define `IRNode` enum + `IRFunction` struct in a new crate `pilots/rusty-js-ir`.
2. Implement lowering: `IRFunction` → Rust source string (or proc-macro).
3. Implement Runtime helpers that lowering targets: `rt.to_object`, `rt.to_integer`, `rt.to_length`, `rt.create_data_property_or_throw`, `rt.array_species_create`, etc.
4. Hand-translate ~10 spec sections (Array.prototype.{map, filter, forEach, every, some, find, indexOf}, Object.{keys, values, entries}) into IR.
5. Generate Rust from IR; replace the corresponding hand-written sites.
6. Run test262 — the slices should hold their post-P62.E25 numbers (regression check).

### 7.2 Tier 2 — spec parser

1. Fetch ECMA-262 source from tc39/ecma262 (the Bikeshed/emu source).
2. Parse `<emu-alg>` blocks into canonical step records.
3. Add the linter (Section 6).
4. Run the linter against the Tier-1 IR functions; iterate until no drift.

### 7.3 Tier 3 — broad coverage

1. Translate the remaining ~80–100 spec sections covering the built-in surface cruftless implements.
2. Establish the IR as the *canonical* source-of-truth: hand-written method sites in intrinsics.rs / prototype.rs become "deprecated" and are migrated as opportunity arises.
3. Eventually delete the hand-written paths.

### 7.4 Open implementation questions

- **Lowering output**: Rust source generation vs. proc-macro vs. compile-time const-fn? Proc-macro is most ergonomic but adds a compile-time dep. Source generation is simplest. Const-fn is constrained.
- **Error message provenance**: should IR carry the spec section id so runtime errors can report "thrown at §23.1.3.20 step 3"? This is the P62.E21 §A8.32 trace-level visibility cost from option (b) reincarnated — but at IR-construction time, not runtime.
- **Optimization passes**: when lowering knows receiver is a primitive (e.g., inside Number.prototype.toString), can it substitute pure-primitive forms (abstract_ops::to_number) for the dispatching ones? Yes, but the discipline is to do this *only* when statically provable.
- **Generic algorithms**: Array.prototype.indexOf and lastIndexOf share most of their structure. The IR could express this via an `Algorithm(name, params, body)` form with parametric body. Defer until pattern emerges from Tier-1 translations.

## 8. Composition with existing seed disciplines

The IR doesn't replace §A8.28–§A8.32 — it *encodes* them, making each discipline structurally inviolable at the IR boundary. The disciplines remain as the conceptual surface; the IR is their operational expression.

§A8.28 (descriptor-shape): IR's typed Desc records.
§A8.29 (abstract-ops duality): IR uses dispatching form; lowering picks pure.
§A8.30 (brand-check): IR's GetSlot auto-throws.
§A8.31 (SyntaxError canonical): IR's Throw is typed.
§A8.32 (ToPrimitive at operator): IR's OpAdd/LooseEq route automatically.

This is the *operational mechanism* for the disciplines, parallel to how Doc 729 §VII.B's bilateral boundary tightening (named at §A8.26) operates through the three-stratum stack: §A8.28 names the *fact*; the IR names the *enforcement*.

## 9. The corpus-tier conjecture

The 30+ substrate fixes per session pattern suggests cruftless is converging toward spec conformance via *measured drift correction*. An IR layer changes the dynamics:

- **Pre-IR**: a missing spec step is caught by a failing test262 case (downstream signal); the fix is a Rust edit; the next missing step is caught the same way.
- **Post-IR**: a missing spec step is caught by the IR-vs-spec linter (upstream signal); the fix is an IR edit; the linter prevents the next missing step at construction time.

The conjecture is that **spec conformance gets monotonically easier post-IR**: each new built-in method translation goes through the linter once, never drifts again.

The test of the conjecture: after Tier 2 lands, the rate of test262 substrate-tier bugs (per slice, per chapter) should drop sharply. If it doesn't, the IR is over-specified or under-specified relative to the actual gap.

## 10. Risks

1. **The spec is a moving target.** ECMA-262 publishes yearly; algorithms evolve. The IR-vs-spec linter must accept versioned spec inputs.
2. **The lowering is one more translation layer that can drift.** Lowering bugs become a new bug class. Mitigation: lowering is mechanical (1:1 IR-node → Rust expr); bugs there should be rare and centralized.
3. **The IR may not cover all of cruftless's surface.** Some built-ins (Bun-specific APIs, host-v2 native code) aren't spec'd by ECMA. These remain hand-written; the IR covers only the ECMA-spec surface.
4. **The discipline reinforcement may over-constrain.** If the IR is too rigid, it might block legitimate optimizations (e.g., inlining pure ToString when receiver type is known). Mitigation: lowering knows local type info and can substitute.

## 11. Next decision

Three orthogonal moves from here:

(a) **Hand-translate one IR function end-to-end** (Array.prototype.map). Validates the node set + lowering + Runtime helper coverage. Smallest possible Tier-1 step. ~1 session of work.

(b) **Parse the ECMA-262 source** and produce canonical step records for one chapter (say §23.1.3). Validates that the spec text *is* parseable into the step-record shape the linter needs. ~1 session.

(c) **Sketch the proc-macro lowering interface** so an IR function declared in Rust attribute syntax compiles directly. Validates that the lowering is ergonomic enough to be the default authoring surface. ~half-session.

Recommended sequence: (a) → (b) → (c). (a) proves the design; (b) proves the linter; (c) makes it ergonomic enough to use.
