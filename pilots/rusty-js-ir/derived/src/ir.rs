//! IR data structures per IR-DESIGN.md §3.
//!
//! Each `IRFunction` corresponds to one ECMA-262 algorithm section (e.g.
//! §23.1.3.20 Array.prototype.map). Steps map 1:1 to the spec's numbered
//! step list, preserving spec-step IDs for the linter (Tier 2).

/// Canonical error classes per ECMA + seed §A8.31.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    TypeError,
    RangeError,
    ReferenceError,
    SyntaxError,
}

impl ErrorClass {
    pub fn rust_variant(self) -> &'static str {
        match self {
            ErrorClass::TypeError => "TypeError",
            ErrorClass::RangeError => "RangeError",
            ErrorClass::ReferenceError => "ReferenceError",
            ErrorClass::SyntaxError => "SyntaxError",
        }
    }
}

/// Internal-slot identifiers per IR-DESIGN.md §3.2. `Slot` is opaque at the
/// IR tier — its meaning is in the lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Slot {
    /// Spec-named slot in double-bracket form (e.g. "[[NumberData]]").
    Named(&'static str),
    /// cruftless-specific sentinel (e.g. "__primitive__", "__set_data").
    Sentinel(&'static str),
}

/// IR expression — value-producing operations.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Reference to a parameter or previously bound local.
    Var(String),
    /// Constant: Undefined / Null / Boolean / Number / String literal.
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    Str(String),

    // ── Coercion / type-check (§A8.29) ──
    RequireObjectCoercible(Box<Expr>),
    ToObject(Box<Expr>),
    ToPrimitive(Box<Expr>, &'static str),
    ToString(Box<Expr>),
    ToNumber(Box<Expr>),
    ToInteger(Box<Expr>),
    ToLength(Box<Expr>),
    ToUint32(Box<Expr>),
    ToBoolean(Box<Expr>),
    ToPropertyKey(Box<Expr>),
    IsCallable(Box<Expr>),
    IsConstructor(Box<Expr>),
    IsArray(Box<Expr>),
    IsRegExp(Box<Expr>),
    /// IR-EXT 95 / Tier-1.5: ECMA-262 §6.1 Type(V) === Object discrimination.
    /// Doc 730 §XIII alphabet promotion: pre-substrate, sections discriminated
    /// "Type(V) is Object" via `Expr::TypeOf(V) === "object"` (with a paired
    /// `=== "function"` to capture functions). That pair collapses spec-Null
    /// (typeof "object", spec Type Null) into the spec-Object branch — the
    /// dual of EXT 72b's spec-Object-with-typeof-"function" case. ToPrimitive's
    /// §7.1.1 step 1 "If Type(input) is not Object, return input" failed to
    /// short-circuit for null inputs, falling through to the @@toPrimitive /
    /// toString / valueOf walk (none defined on null), reaching step 6's
    /// "Cannot convert object to primitive value" throw — observable as the
    /// arktype @ark/schema/roots/unit.js:52 failure on `\${null}` template
    /// coercion. IsSpecObject lowers to `matches!(v, Value::Object(_))`,
    /// which in this engine's Value enum already covers spec-Object
    /// (ordinary + function objects) and excludes spec-Null, spec-Undefined,
    /// and primitives.
    IsSpecObject(Box<Expr>),
    SameValue(Box<Expr>, Box<Expr>),
    SameValueZero(Box<Expr>, Box<Expr>),

    // ── Slot / property (§A8.28 + §A8.30) ──
    HasSlot(Box<Expr>, Slot),
    GetSlot(Box<Expr>, Slot),
    Get(Box<Expr>, Box<Expr>),
    /// EXT 82 / Tier-1.5: ECMA-262 §7.3.2 Get(O, P) — the spec's `[[Get]]`
    /// internal method. Distinct from `Get` (which is the runtime
    /// internal-slot read used when the spec explicitly says "the [[X]]
    /// internal slot"). SpecGet invokes Proxy traps, inherited accessors,
    /// and propagates user-thrown errors; use it wherever the spec step
    /// reads `? Get(...)` or invokes `[[Get]]`. The verifier-time
    /// discrimination this carries was named in Doc 730 §XIII as the
    /// first Tier-1.5 alphabet promotion: the spec uses different fonts
    /// for `Get` and `[[X]]` internal-slot reads; the IR now mirrors
    /// that typographic distinction as two distinct primitives.
    SpecGet(Box<Expr>, Box<Expr>),
    /// EXT 85 / Tier-1.5: ECMA-262 §7.3.10 GetMethod(V, P) — the spec
    /// wrapper around Get that enforces the spec post-condition
    /// "callable-or-undefined-or-throw" on the result. Lowering performs
    /// SpecGet, then: returns Undefined if the result is undefined OR
    /// null (the §7.3.10 step 2.a normalisation); throws TypeError if
    /// the result is defined but not callable (step 3); returns the
    /// value otherwise. Promotes a pattern that recurs throughout IR
    /// sections (the EXT 84c trap-is-not-callable check inlined this in
    /// Rust for one site per Proxy trap; 10+ IR sections do an
    /// equivalent inline check on method lookups). The §XIII Pass B
    /// trace named this as the first cleanly-promotable derivative of
    /// SpecGet — same lowering site, different post-condition.
    GetMethod(Box<Expr>, Box<Expr>),
    HasProperty(Box<Expr>, Box<Expr>),
    HasOwnProperty(Box<Expr>, Box<Expr>),
    OrdinaryObjectCreate {
        proto: Box<Expr>,
        slots: Vec<(Slot, Expr)>,
    },
    /// ArraySpeciesCreate(O, length) per §23.1.3.27.
    ArraySpeciesCreate {
        o: Box<Expr>,
        length: Box<Expr>,
    },

    // ── Calls (§A8.32 extended) ──
    Call {
        function: Box<Expr>,
        this: Box<Expr>,
        args: Vec<Expr>,
    },
    Construct {
        ctor: Box<Expr>,
        args: Vec<Expr>,
    },
    Invoke {
        object: Box<Expr>,
        method: Box<Expr>,
        args: Vec<Expr>,
    },

    // ── Operators (§A8.32) ──
    OpAdd(Box<Expr>, Box<Expr>),
    OpSub(Box<Expr>, Box<Expr>),
    OpMul(Box<Expr>, Box<Expr>),
    LooseEq(Box<Expr>, Box<Expr>),
    StrictEq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),

    // IR-EXT 67 alphabet promotion: typed Number arithmetic on Value::Number.
    // The original OpAdd/OpSub/OpMul/Lt/Le were designed for usize/i64 counter
    // math; these Number* variants explicitly operate on Value::Number,
    // dispatching through coerce_to_number at the lowering boundary. Used by
    // higher-resolution-IR spec-step sections (e.g. §10.4.2.1) that
    // manipulate spec-Number values like Array length.
    NumberAdd(Box<Expr>, Box<Expr>),
    NumberSub(Box<Expr>, Box<Expr>),
    NumberLt(Box<Expr>, Box<Expr>),
    NumberGe(Box<Expr>, Box<Expr>),

    // IR-EXT 68 alphabet promotion: §13.5.3 typeof operator. Returns a
    // Value::String matching one of "undefined", "boolean", "number",
    // "string", "bigint", "symbol", "function", "object".
    TypeOf(Box<Expr>),

    /// Argument access — args[i], defaulting to Undefined.
    Arg(usize),

    /// ArgsRest(start) — lowers to `&args[start..]` (a slice from `start`
    /// onward, or empty if args has fewer than `start` items). Used by
    /// variadic spec ops whose first N args are positional (e.g.,
    /// Object.assign with target + sources, Reflect.construct with
    /// ctor + args-array + newTarget).
    ArgsRest(usize),

    /// AllArgs — lowers to `args` (the raw Rust slice). Used by
    /// variadic spec ops (Math.max, Math.min, Math.hypot, etc.) whose
    /// Runtime helpers consume the full arg list. Distinct from
    /// `Arg(i)` which extracts one positional arg.
    AllArgs,

    /// HasArg(i) — true iff args.len() > i. Distinguishes "arg passed
    /// as undefined" from "arg not passed at all". Used by methods whose
    /// spec distinguishes these cases (e.g., Array.prototype.reduce's
    /// initialValue presence check per §23.1.3.24 step 4).
    HasArg(usize),

    /// CallBuiltin(name, args) — invoke a Runtime helper that isn't a
    /// JS-side method dispatch. Used for spec-prescribed abstract ops
    /// that don't model as Get-then-Call (EnumerableOwnPropertyNames,
    /// SpeciesConstructor, NewPromiseCapability, etc.). Lowers to
    /// `rt.<name>(arg1, arg2, ...)?`.
    CallBuiltin {
        name: &'static str,
        args: Vec<Expr>,
    },

    /// Receiver — `this` value.
    This,

    /// LengthOfArrayLike per §7.3.20 — returns a `usize`-typed index.
    LengthOfArrayLike(Box<Expr>),

    /// CreateDataPropertyOrThrow per §7.3.6 — side-effecting; returns unit.
    CreateDataPropertyOrThrow(Box<Expr>, Box<Expr>, Box<Expr>),

    /// Typed integer literal — lowers to `usize`. Used for loop counters.
    IntConst(i64),

    /// Typed `usize` addition (no ToPrimitive dispatch; pure arithmetic).
    /// Used in loop-counter increments where the spec's `k + 1` is over
    /// math-Numbers but the engine models them as usize indices.
    IndexAdd(Box<Expr>, Box<Expr>),

    /// Convert a value-typed Number to a typed `usize` (saturating on
    /// negatives / non-finite).
    AsIndex(Box<Expr>),

    /// Convert a typed `usize` to a Value::Number for callback args.
    IndexAsValue(Box<Expr>),

    /// Convert a typed `usize` to its decimal string representation
    /// for ECMA's `! ToString(𝔽(k))` operation.
    IndexAsKey(Box<Expr>),

    // ── Ω.5.P63.E55 Alphabet closures ──────────────────────────────────
    //
    // ECMA-262 specifies many algorithms that allocate fresh built-in
    // functions with captured slot state (Promise.all Resolve Element
    // Functions §27.2.4.1.2, Promise.allSettled / .any analogues,
    // NewPromiseCapability executor, AsyncFromSyncIteratorContinuation
    // step-7 callback, etc.). Pre-E55 the IR couldn't model these
    // first-class — the via-helper layer constructed them as Rust
    // closures with Rc<RefCell<_>> captures. E55 lifts the pattern to
    // the alphabet so the IR can spec-faithfully describe each closure's
    // capture set + body, and the lowering emits the corresponding
    // `make_native(move |rt, args| { ... })`.
    //
    // The closure model is three-piece:
    //   - `Capture::*` — how each captured name is bound to the enclosing
    //     scope at closure-construction time. Cell vs Value distinguishes
    //     "mutable shared state via Rc<RefCell<_>>" from "moved snapshot".
    //   - `Expr::Closure` — construct the closure value. Lowers to a
    //     `make_native` whose `move` closure pre-binds the captures.
    //   - `Expr::CellNew / CellGet` and `IRNode::CellSet` — explicit
    //     handles for the Rc<RefCell<_>> idiom so the linter sees the
    //     spec's "Internal slot of F" assignments structurally.
    /// Allocate a fresh shared mutable cell holding the given Value-typed
    /// initial. Lowers to `std::rc::Rc::new(std::cell::RefCell::new(<init>))`.
    /// Cells are the substrate for spec slots assigned by closures (the
    /// Promise.all Resolve Element Function's [[AlreadyCalled]] / [[Values]] /
    /// [[RemainingElementsCount]] slots).
    CellNew(Box<Expr>),

    /// Read the current Value held in a shared mutable cell. Lowers to
    /// `(*<cell>.borrow()).clone()`.
    CellGet(Box<Expr>),

    /// Construct a closure value capturing the named locals from the
    /// enclosing IR section's scope. The `label` is a debug name used
    /// in the function's NativeFn "name" slot; `params` declares the
    /// positional-argument bindings the closure body sees; `captures`
    /// names the locals from the enclosing scope to move/clone into the
    /// closure's environment.
    ///
    /// Lowers to:
    ///   {
    ///     let <captured_1> = <captured_1>.clone();
    ///     let <captured_2> = <captured_2>.clone();
    ///     let f = crate::intrinsics::make_native("<label>", move |rt, args| {
    ///         let <param_1> = args.get(0).cloned().unwrap_or(Value::Undefined);
    ///         <body lowered to Rust>
    ///     });
    ///     Value::Object(rt.alloc_object(f))
    ///   }
    Closure {
        label: &'static str,
        params: Vec<String>,
        captures: Vec<String>,
        body: Vec<Step>,
    },
}

/// IR step — corresponds to one ECMA-262 algorithm step (e.g. "step 1",
/// "step 6.c.ii"). The `spec_step` field carries the step identifier for
/// the linter (Tier 2).
#[derive(Debug, Clone)]
pub struct Step {
    pub spec_step: String,
    pub node: IRNode,
}

/// IR statement — non-value-producing operations.
#[derive(Debug, Clone)]
pub enum IRNode {
    /// Bind a Value-typed local.
    Let { name: String, value: Expr },

    /// Bind a `usize`-typed mutable index local (loop counter).
    LetIndex { name: String, value: Expr },

    /// Assign to a `usize`-typed index local.
    AssignIndex { name: String, value: Expr },

    /// Throw a canonical error class with a message.
    Throw { class: ErrorClass, message: String },

    /// Return a value.
    Return(Expr),

    /// Conditional execution.
    If {
        cond: Expr,
        then_body: Vec<Step>,
        else_body: Vec<Step>,
    },

    /// While loop.
    While { cond: Expr, body: Vec<Step> },

    /// Reassign a previously bound local (for loop counters etc.).
    Assign { name: String, value: Expr },

    /// Side-effecting call whose return value is discarded.
    Expr(Expr),

    /// Ω.5.P63.E55: Write a Value into a shared cell. Lowers to
    /// `*<cell>.borrow_mut() = <value>;`. Used inside closure bodies
    /// modeling spec-step assignments like "Set F.[[AlreadyCalled]] to true".
    CellSet { cell: Expr, value: Expr },
}

/// IR function — one ECMA-262 algorithm section, hand-translated.
#[derive(Debug, Clone)]
pub struct IRFunction {
    /// Spec reference (e.g. "23.1.3.20").
    pub spec_section: String,
    /// Human name for the lowered Rust function.
    pub rust_name: String,
    /// Spec algorithm title (e.g. "Array.prototype.map ( callbackfn [ , thisArg ] )").
    pub title: String,
    /// Body — list of spec-step-annotated IR nodes.
    pub body: Vec<Step>,
}
