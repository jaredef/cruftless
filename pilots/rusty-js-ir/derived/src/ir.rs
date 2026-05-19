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
    SameValue(Box<Expr>, Box<Expr>),
    SameValueZero(Box<Expr>, Box<Expr>),

    // ── Slot / property (§A8.28 + §A8.30) ──
    HasSlot(Box<Expr>, Slot),
    GetSlot(Box<Expr>, Slot),
    Get(Box<Expr>, Box<Expr>),
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
    Throw {
        class: ErrorClass,
        message: String,
    },

    /// Return a value.
    Return(Expr),

    /// Conditional execution.
    If {
        cond: Expr,
        then_body: Vec<Step>,
        else_body: Vec<Step>,
    },

    /// While loop.
    While {
        cond: Expr,
        body: Vec<Step>,
    },

    /// Reassign a previously bound local (for loop counters etc.).
    Assign { name: String, value: Expr },

    /// Side-effecting call whose return value is discarded.
    Expr(Expr),
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
