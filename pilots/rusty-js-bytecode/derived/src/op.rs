//! Bytecode instruction set per specs/rusty-js-bytecode-design.md §II.
//!
//! Each Op is a single byte. Operands (where present) follow inline in
//! little-endian form. Operand widths per opcode are documented in the
//! `operand_size` table.

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    // Stack ops
    PushNull = 0x01,
    PushUndef = 0x02,
    PushTrue = 0x03,
    PushFalse = 0x04,
    /// PUSH_I32 <i32>
    PushI32 = 0x05,
    /// PUSH_CONST <u16>
    PushConst = 0x06,
    Pop = 0x07,
    Dup = 0x08,
    Swap = 0x09,

    // Variable / scope (v1 round 3.c.b: only global resolution implemented)
    /// LOAD_LOCAL <u16>
    LoadLocal = 0x10,
    /// STORE_LOCAL <u16>
    StoreLocal = 0x11,
    /// LOAD_ARG <u16>
    LoadArg = 0x12,
    /// STORE_ARG <u16>
    StoreArg = 0x13,
    /// LOAD_GLOBAL <u16> — string-name constant index
    LoadGlobal = 0x14,
    /// STORE_GLOBAL <u16>
    StoreGlobal = 0x15,
    /// LOAD_UPVALUE <u16>
    LoadUpvalue = 0x16,
    /// STORE_UPVALUE <u16>
    StoreUpvalue = 0x17,
    /// DEFINE_LOCAL <u16>
    DefineLocal = 0x18,
    /// RESET_LOCAL_CELL <u16> — clear frame.local_cells[slot] to None so the
    /// next CaptureLocal at this slot promotes to a fresh upvalue cell. Used
    /// by for-of with `let`/`const` head to give each iteration a fresh
    /// binding per ECMA-262 §14.7.5.5. Closures captured in iteration N keep
    /// their Rc handle to iteration N's cell; iteration N+1 starts from None.
    /// Tier-Ω.5.g.1.
    ResetLocalCell = 0x19,

    // Arithmetic
    Add = 0x20,
    Sub = 0x21,
    Mul = 0x22,
    Div = 0x23,
    Mod = 0x24,
    Pow = 0x25,
    Neg = 0x26,
    Pos = 0x27,
    Inc = 0x28,
    Dec = 0x29,

    // Comparison / equality / relational
    Lt = 0x30,
    Gt = 0x31,
    Le = 0x32,
    Ge = 0x33,
    Eq = 0x34,
    Ne = 0x35,
    StrictEq = 0x36,
    StrictNe = 0x37,
    In = 0x38,
    Instanceof = 0x39,

    // Bitwise / shift
    BitAnd = 0x40,
    BitOr = 0x41,
    BitXor = 0x42,
    BitNot = 0x43,
    Shl = 0x44,
    Shr = 0x45,
    UShr = 0x46,

    // Logical
    Not = 0x50,

    // Control flow
    /// JUMP <i32>
    Jump = 0x60,
    /// JUMP_IF_TRUE <i32> — pops condition
    JumpIfTrue = 0x61,
    /// JUMP_IF_FALSE <i32> — pops condition
    JumpIfFalse = 0x62,
    /// JUMP_IF_TRUE_KEEP <i32> — for || short-circuit
    JumpIfTrueKeep = 0x63,
    /// JUMP_IF_FALSE_KEEP <i32> — for && short-circuit
    JumpIfFalseKeep = 0x64,
    /// JUMP_IF_NULLISH <i32> — for ?? operator
    JumpIfNullish = 0x65,

    // Calls / returns
    /// CALL <u8>
    Call = 0x70,
    /// NEW <u8>
    New = 0x71,
    Return = 0x72,
    ReturnUndef = 0x73,
    /// CALL_METHOD <u8> — stack layout: [..., receiver, method, arg0..argN-1].
    /// Pops args + method + receiver, invokes method with receiver as `this`.
    /// Added Tier-Ω.5.a for prototype-chain instance-method dispatch.
    CallMethod = 0x74,
    /// PUSH_THIS — push the current frame's `this` value onto the operand stack.
    PushThis = 0x75,
    /// PUSH_IMPORT_META — push the current module's `import.meta` object onto
    /// the operand stack. The runtime populates the frame's import_meta slot
    /// at evaluate_module entry with an object `{ url, dir }` reflecting the
    /// module's resolved URL. Falls back to Undefined for frames not entered
    /// via the module loader (e.g. ad-hoc compile + run_module callers).
    /// Tier-Ω.5.r.
    PushImportMeta = 0x76,
    /// PUSH_NEW_TARGET — push the current frame's `new.target` value onto
    /// the operand stack. The runtime populates Frame::new_target inside
    /// Op::New before dispatching the constructor call; frames invoked
    /// via plain Call leave the slot None, in which case PushNewTarget
    /// pushes Undefined per ECMA-262 §15.7.5.1. Tier-Ω.5.s.
    PushNewTarget = 0x77,
    /// SET_THIS — Tier-Ω.5.nnnnn. Pops the top of stack; if it is an
    /// Object, assigns it to the current frame's this_value. Used after
    /// super(...) to implement ECMA-262 §15.4.5.4 step 9 (when parent
    /// constructor returns an object, that object replaces `this` for
    /// the derived ctor body).
    SetThis = 0x78,
    /// PROPAGATE_NEW_TARGET — Ω.5.P03.E2.super-new-target. Reads the
    /// current frame's `new.target` and writes it into the runtime's
    /// pending_new_target slot, so the next dispatch (CallMethod for a
    /// super(...) call) propagates new.target through to the base
    /// constructor's frame per ECMA-262 §10.2.1.3 step 4 (SuperCall:
    /// new.target is the active newTarget of the calling derived
    /// constructor). Without this, super(...) routes via CallMethod
    /// with no construct semantics, and the base ctor sees
    /// new.target=undefined plus loses the implicit-return-this rule
    /// at the calling derived's super-call sequence. Emit immediately
    /// before the CallMethod (or __apply path) in compile_super_call.
    PropagateNewTarget = 0x79,

    // Member access
    /// GET_PROP <u16>
    GetProp = 0x80,
    /// SET_PROP <u16>
    SetProp = 0x81,
    GetIndex = 0x82,
    SetIndex = 0x83,
    /// SET_PROTOTYPE — pops [target, proto] (proto on top); sets
    /// target.[[Prototype]] = proto when proto is Object, or to None when
    /// proto is Null. Tier-Ω.5.f: class-extends chain wiring.
    SetPrototype = 0x84,

    // Object / array construction
    NewObject = 0x90,
    /// NEW_ARRAY <u16>
    NewArray = 0x91,
    /// INIT_PROP <u16>
    InitProp = 0x92,
    /// INIT_INDEX <u32>
    InitIndex = 0x93,

    // Unary / type
    Typeof = 0xA0,
    Void = 0xA1,
    Delete = 0xA2,
    /// DELETE_PROP <u16 name_idx> — pop obj, remove obj's own
    /// property `name`, push Boolean true if existed and was removed.
    /// Tier-Ω.5.BBBBBBBB.
    DeleteProp = 0xA3,
    /// DELETE_INDEX — pop key, pop obj, remove obj[ToString(key)],
    /// push Boolean true if existed and was removed.
    DeleteIndex = 0xA4,

    // Function / closure
    /// MAKE_CLOSURE <u16>
    MakeClosure = 0xB0,
    /// MAKE_ARROW <u16>
    MakeArrow = 0xB1,
    /// CAPTURE_LOCAL <u16> — pop top-of-stack closure, read frame.locals[slot],
    /// append into the closure's upvalues, push closure back. Emitted after
    /// MakeClosure for each captured outer local (Tier-Ω.5.c).
    CaptureLocal = 0xB2,
    /// CAPTURE_UPVALUE <u16> — like CaptureLocal but reads from the current
    /// frame's upvalues slot (transitively-captured outer upvalue).
    CaptureUpvalue = 0xB3,

    // Exception handling
    Throw = 0xC0,
    /// TRY_ENTER <u32>
    TryEnter = 0xC1,
    TryExit = 0xC2,

    // Iteration
    IterInit = 0xD0,
    IterNext = 0xD1,
    IterClose = 0xD2,

    // Miscellaneous
    Nop = 0xE0,
    Debugger = 0xE1,

    // ────────── Doc 731 §XIV.d typed-operand alphabet promotion ──────────
    // Typed-I64 arithmetic + comparison primitives. The bytecode tier's
    // §XIII-shape alphabet promotion: the previously-collapsed dispatch
    // inside Op::Add (string-concat vs ToPrimitive-then-arithmetic vs
    // numeric-add) is split out as separate typed primitives. The plain
    // Op::Add stays for cases where operand types are not provable; the
    // typed ops are emitted whenever an upstream type-inference pass
    // can prove both operands fit in i64. The JIT's verifier accepts
    // these directly (no cheating — the typed assumption is encoded
    // in the alphabet); the interpreter assumes both operands are
    // Number values with integer-valued f64 representation and dispatches
    // pure-integer arithmetic without ToPrimitive.
    //
    // Values are still boxed as Value::Number(f64); the typed-i64 ops
    // unbox-as-integer at the interpreter, and the JIT lowers them
    // directly to Cranelift iadd/isub/imul/icmp.
    //
    // Failure mode: if a typed-I64 op encounters a non-Number-integer
    // operand at runtime, the interpreter throws TypeError (or, under
    // a future deviation primitive, traps to a deopt point in JIT'd
    // code). v1: throws TypeError uniformly.
    AddI64 = 0xF0,
    SubI64 = 0xF1,
    MulI64 = 0xF2,
    IncI64 = 0xF3,
    DecI64 = 0xF4,
    LtI64  = 0xF5,
    LeI64  = 0xF6,
    GtI64  = 0xF7,
    GeI64  = 0xF8,
    EqI64  = 0xF9,
    NeI64  = 0xFA,

    /// GET_PROP_ON_OBJECT <u16>
    ///
    /// JIT-EXT 19 (Doc 731 §XIV.d β-path for property access): typed
    /// variant of GetProp where the upstream emitter has proven the
    /// receiver is an Object. Bytecode shape is identical to GetProp
    /// (u16 prop-name index); semantics are identical in the interpreter
    /// (the typed assertion is a static claim, not a runtime check).
    /// The JIT lowering at JIT-EXT 20+ exploits the typed claim by
    /// emitting a direct property-fetch path (with IC) instead of a
    /// generic dispatch.
    GetPropOnObject = 0xFB,

    /// CALL_METHOD_IC_CACHED <u8 ihi_table_idx>
    ///
    /// IHI-EXT 11 (2026-05-24, Finding IHI.1+IHI.2 deeper-layer closure
    /// per Doc 740 §IV.2): bytecode-rewrite IC cached dispatch.
    /// Op::CallMethod's dispatcher rewrites the op byte to this opcode +
    /// rewrites the arity byte to the IHI_TABLE index on first successful
    /// IC fast-path hit. Subsequent dispatches skip ALL cache/lookup
    /// machinery and run entry.fast directly. Per-call cost drops from
    /// ~60ns (cache+lookup) to ~10ns (byte fetch + IHI_TABLE[idx]).
    ///
    /// The arity is implicit (entry.arity); the dispatcher pops the
    /// matching number of stack values. Bails to slow path if the IC
    /// guard fails at the cached entry (override / type mismatch / etc.).
    CallMethodIcCached = 0xFC,

    /// GET_PROP_SKIP_FOR_METHOD <u16 prop-name-idx>
    ///
    /// GPI-EXT 2 (2026-05-24, interp-getprop-ic locale, deeper-layer
    /// closure per Doc 740 §IV.2 + standing rule 13): bytecode-rewrite
    /// companion to CallMethodIcCached. Op::CallMethod's IC-hit branch
    /// (when receiver_kind == String) walks back to the preceding
    /// Op::GetProp site (recorded at Frame::pending_method_getprop_pc)
    /// and rewrites that op byte to this opcode. Same 2-byte operand
    /// as GetProp permits in-place op-byte rewrite without operand
    /// shift. The handler pops the receiver (discarding the Dup-top
    /// copy) and pushes Value::Undefined as a sentinel for the
    /// following CallMethodIcCached, which is bail-aware (re-resolves
    /// from string_prototype on cold-path bail). Eliminates the
    /// descriptor walk on hot String-method-call sites.
    GetPropSkipForMethod = 0xFD,

    /// FOR_OF_FAST_NEXT <u16 iter_slot> <u16 bind_slot> <i32 done_offset> <i16 next_iter_offset>
    ///
    /// IPBR-EXT 2 (2026-05-24, iter-protocol-bytecode-rewrite locale,
    /// deeper-layer-first design per docs/standing-rule-13-prospective-
    /// application.md): fused for-of fast-path opcode emitted by the
    /// compiler at loop_start in front of the existing slow-path
    /// emission. On each iteration, probes iter_slot for the well-known
    /// intrinsic ArrayIterator shape (`__it_src__` + `__it_idx__` + src
    /// is InternalKind::Array). If eligible: reads arr[idx] directly,
    /// stores to bind_slot, increments idx, jumps to next_iter_offset
    /// (skipping the slow path). If exhausted: jumps to done_offset
    /// (post-Pop). If ineligible: falls through to the slow path
    /// unchanged. Eliminates per-iter result-object allocation + the
    /// GetProp"next"/Call/GetProp"done"/GetProp"value" cost surface.
    ForOfFastNext = 0xFE,

    /// REOU-EXT 1: silent-undef variant of LoadGlobal. Returns Undefined
    /// when the name is not bound (instead of throwing ReferenceError per
    /// ECMA-262 §9.1.1.4.4). Used by compiler at `typeof <Ident>` and
    /// `delete <Ident>` sites only, where §13.5.3 step 3.b.iii (typeof)
    /// and §13.5.1.2 (delete of unresolvable returns true) require the
    /// silent path. All other identifier reads go through Op::LoadGlobal
    /// which throws.
    LoadGlobalOrUndef = 0xFF,
}

impl Op {
    /// Number of operand bytes following this opcode.
    pub fn operand_size(self) -> usize {
        use Op::*;
        match self {
            PushNull | PushUndef | PushTrue | PushFalse | Pop | Dup | Swap
            | Add | Sub | Mul | Div | Mod | Pow | Neg | Pos | Inc | Dec
            | Lt | Gt | Le | Ge | Eq | Ne | StrictEq | StrictNe | In | Instanceof
            | BitAnd | BitOr | BitXor | BitNot | Shl | Shr | UShr | Not
            | Return | ReturnUndef
            | GetIndex | SetIndex | SetPrototype | NewObject
            | Typeof | Void | Delete | DeleteIndex
            | Throw | TryExit
            | IterInit | IterNext | IterClose
            | Nop | Debugger | PushThis | PushImportMeta | PushNewTarget | SetThis
            | PropagateNewTarget
            | AddI64 | SubI64 | MulI64 | IncI64 | DecI64
            | LtI64 | LeI64 | GtI64 | GeI64 | EqI64 | NeI64 => 0,
            Call | New | CallMethod | CallMethodIcCached => 1,
            PushConst | LoadLocal | StoreLocal | LoadArg | StoreArg
            | LoadGlobal | LoadGlobalOrUndef | StoreGlobal | LoadUpvalue | StoreUpvalue
            | DefineLocal | ResetLocalCell | GetProp | GetPropOnObject | GetPropSkipForMethod | SetProp | NewArray | InitProp
            | MakeClosure | MakeArrow | CaptureLocal | CaptureUpvalue | DeleteProp => 2,
            PushI32 | Jump | JumpIfTrue | JumpIfFalse
            | JumpIfTrueKeep | JumpIfFalseKeep | JumpIfNullish
            | InitIndex | TryEnter => 4,
            ForOfFastNext => 10,
        }
    }
}

/// Emit one opcode (+ optional operand) into a byte buffer. Returns the
/// offset at which the operand begins (useful for forward-jump patching).
pub fn encode_op(buf: &mut Vec<u8>, op: Op) -> usize {
    buf.push(op as u8);
    buf.len()
}

pub fn encode_u8(buf: &mut Vec<u8>, v: u8) {
    buf.push(v);
}

pub fn encode_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}

pub fn encode_i32(buf: &mut Vec<u8>, v: i32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

pub fn encode_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

pub fn decode_u16(bc: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([bc[off], bc[off + 1]])
}

pub fn decode_i32(bc: &[u8], off: usize) -> i32 {
    i32::from_le_bytes([bc[off], bc[off + 1], bc[off + 2], bc[off + 3]])
}

pub fn decode_u32(bc: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([bc[off], bc[off + 1], bc[off + 2], bc[off + 3]])
}

pub fn op_from_byte(b: u8) -> Option<Op> {
    use Op::*;
    Some(match b {
        0x01 => PushNull, 0x02 => PushUndef, 0x03 => PushTrue, 0x04 => PushFalse,
        0x05 => PushI32, 0x06 => PushConst, 0x07 => Pop, 0x08 => Dup, 0x09 => Swap,
        0x10 => LoadLocal, 0x11 => StoreLocal, 0x12 => LoadArg, 0x13 => StoreArg,
        0x14 => LoadGlobal, 0x15 => StoreGlobal, 0x16 => LoadUpvalue, 0x17 => StoreUpvalue,
        0x18 => DefineLocal, 0x19 => ResetLocalCell,
        0x20 => Add, 0x21 => Sub, 0x22 => Mul, 0x23 => Div, 0x24 => Mod, 0x25 => Pow,
        0x26 => Neg, 0x27 => Pos, 0x28 => Inc, 0x29 => Dec,
        0x30 => Lt, 0x31 => Gt, 0x32 => Le, 0x33 => Ge,
        0x34 => Eq, 0x35 => Ne, 0x36 => StrictEq, 0x37 => StrictNe,
        0x38 => In, 0x39 => Instanceof,
        0x40 => BitAnd, 0x41 => BitOr, 0x42 => BitXor, 0x43 => BitNot,
        0x44 => Shl, 0x45 => Shr, 0x46 => UShr,
        0x50 => Not,
        0x60 => Jump, 0x61 => JumpIfTrue, 0x62 => JumpIfFalse,
        0x63 => JumpIfTrueKeep, 0x64 => JumpIfFalseKeep, 0x65 => JumpIfNullish,
        0x70 => Call, 0x71 => New, 0x72 => Return, 0x73 => ReturnUndef,
        0x74 => CallMethod, 0x75 => PushThis, 0x76 => PushImportMeta, 0x77 => PushNewTarget, 0x78 => SetThis,
        0x79 => PropagateNewTarget,
        0x80 => GetProp, 0x81 => SetProp, 0x82 => GetIndex, 0x83 => SetIndex,
        0x84 => SetPrototype,
        0x90 => NewObject, 0x91 => NewArray, 0x92 => InitProp, 0x93 => InitIndex,
        0xA0 => Typeof, 0xA1 => Void, 0xA2 => Delete, 0xA3 => DeleteProp, 0xA4 => DeleteIndex,
        0xB0 => MakeClosure, 0xB1 => MakeArrow,
        0xB2 => CaptureLocal, 0xB3 => CaptureUpvalue,
        0xC0 => Throw, 0xC1 => TryEnter, 0xC2 => TryExit,
        0xD0 => IterInit, 0xD1 => IterNext, 0xD2 => IterClose,
        0xE0 => Nop, 0xE1 => Debugger,
        0xF0 => AddI64, 0xF1 => SubI64, 0xF2 => MulI64, 0xF3 => IncI64, 0xF4 => DecI64,
        0xF5 => LtI64, 0xF6 => LeI64, 0xF7 => GtI64, 0xF8 => GeI64, 0xF9 => EqI64, 0xFA => NeI64,
        0xFB => GetPropOnObject,
        0xFC => CallMethodIcCached,
        0xFD => GetPropSkipForMethod,
        0xFE => ForOfFastNext,
        0xFF => LoadGlobalOrUndef,
        _ => return None,
    })
}
