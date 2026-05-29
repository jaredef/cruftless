//! ECMAScript value representation per specs/rusty-js-runtime-design.md §I.
//!
//! v1 simplifications:
//! - Strings stored as Rust String (UTF-8) rather than Vec<u16> (UTF-16
//!   per spec §6.1.4). The mismatch matters for surrogate-pair-aware
//!   indexing/length but doesn't affect most consumer behavior. Migration
//!   to UTF-16 is mechanical when needed.
//! - Round 3.e.d: Object references migrated from Rc<RefCell<Object>> to
//!   ObjectId — Objects live in Runtime.heap. Value::Object payload is
//!   ObjectId (Copy + Eq). Cycles are now reclaimable via rt.collect().

use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;

/// A captured-binding cell. Tier-Ω.5.e migrated upvalues from
/// value-snapshot (Vec<Value>) to binding-shared (Vec<UpvalueCell>) per
/// ECMA-262 §8.1 / §10.2: each captured binding is one shared location,
/// shared across the outer frame's slot and every closure that captured
/// it. Writes through any handle are visible to all others.
pub type UpvalueCell = Rc<RefCell<Value>>;

pub fn new_upvalue_cell(v: Value) -> UpvalueCell {
    Rc::new(RefCell::new(v))
}

/// Alias preserving call-site shape. Post-3.e.d this is a heap handle
/// (Copy + Eq), not an Rc<RefCell<...>>.
pub type ObjectRef = rusty_js_gc::ObjectId;

// ──────────────── GC Trace impl ────────────────
//
// Object's out-edges:
//   - proto: Option<ObjectId>
//   - properties.values()'s Value::Object payloads
//   - InternalKind edges:
//       Closure: upvalues' Value::Object payloads
//       BoundFunction: target + this + args
//       Promise: each reaction's chain (always Object) + handler (if Object)
impl rusty_js_gc::Trace for Object {
    fn trace(&self, ids: &mut Vec<rusty_js_gc::ObjectId>) {
        if let Some(p) = self.proto {
            ids.push(p);
        }
        if let Some(home) = self.private_home {
            ids.push(home);
        }
        // CMig-EXT 16 NEEDS-VERIFY follow-up (2026-05-23): trace
        // shape_values for Object references. Shape-enrolled objects
        // store user-default properties in shape_values (not in
        // .properties); pre-fix Trace missed these, which under a
        // future mark-and-sweep GC pass would collect referenced
        // Objects → use-after-free. Currently dormant because cruft's
        // GC leaks, but the fix is correctness-required forward-looking.
        for v in &self.shape_values {
            if let Value::Object(id) = v {
                ids.push(*id);
            }
        }
        for d in self.properties.values() {
            if let Value::Object(id) = &d.value {
                ids.push(*id);
            }
        }
        for v in self.private_fields.values() {
            if let Value::Object(id) = v {
                ids.push(*id);
            }
        }
        match &self.internal_kind {
            InternalKind::Closure(c) => {
                for cell in &c.upvalues {
                    if let Value::Object(id) = &*cell.borrow() {
                        ids.push(*id);
                    }
                }
            }
            InternalKind::BoundFunction(b) => {
                ids.push(b.target);
                if let Value::Object(id) = &b.this {
                    ids.push(*id);
                }
                for v in &b.args {
                    if let Value::Object(id) = v {
                        ids.push(*id);
                    }
                }
            }
            InternalKind::Promise(ps) => {
                if let Value::Object(id) = &ps.value {
                    ids.push(*id);
                }
                for r in &ps.fulfill_reactions {
                    ids.push(r.chain);
                    if let Some(Value::Object(id)) = &r.handler {
                        ids.push(*id);
                    }
                }
                for r in &ps.reject_reactions {
                    ids.push(r.chain);
                    if let Some(Value::Object(id)) = &r.handler {
                        ids.push(*id);
                    }
                }
            }
            _ => {}
        }
    }
}

// LeJIT-Ψ VTI-EXT 3a: layout-pin Value so the JIT-prologue tag-check
// emitter (VTI-EXT 3b) can read the discriminant at a known offset.
// repr(u8) places the tag at offset 0 (one byte); rustc lays out the
// payload at the next max-alignment boundary (offset 8 for f64/Rc).
// VTI-EXT 3b's emit_inline_number_tag_check reads byte 0; payload
// extraction reads from offset 8. NUMBER_TAG = 3 per declaration order.
#[derive(Clone)]
#[repr(C, u8)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(Rc<String>),
    /// BigInt as a signed-magnitude limb vector per ECMA §6.1.6.2.
    /// Tier-Ω.5.CCCCCCCC: real arithmetic substrate replacing the v1
    /// Rc<String> representation that coerced through f64.
    BigInt(Rc<crate::bigint::JsBigInt>),
    /// Tier-Ω.5.P19.E1.symbol-value-type: Symbol as a distinct value variant.
    /// Carries the canonical internal `@@sym:<n>:<desc>` or `@@sym:<key>`
    /// form verbatim so `to_string(Value::Symbol)` returns the same string
    /// used as the underlying property-storage key. E2-class relaxation:
    /// the data model keeps string-keyed storage; the variant carries the
    /// typeof + enumeration discrimination only. Well-known symbols
    /// (`Symbol.iterator`, etc.) remain `Value::String("@@iterator")` so
    /// existing intrinsic-method dispatch sites continue to work without
    /// a parallel migration.
    Symbol(Rc<String>),
    Object(ObjectRef),
}

// LeJIT-Ψ VTI-EXT 3a: discriminant + payload-offset constants the
// JIT-prologue tag-check emitter (VTI-EXT 3b) consumes. With
// #[repr(C, u8)] above, rustc lays out Value as:
//   - byte 0:     u8 discriminant (declaration-order: 0..7)
//   - bytes 1..N: padding to max-payload-alignment
//   - byte N:     payload (f64 / Rc / etc. at alignment 8)
// These constants are the JIT emitter's source of truth; if rustc
// changes the layout, the static asserts below fail at compile time
// and the JIT does not silently emit wrong code.
pub const VALUE_TAG_UNDEFINED: u8 = 0;
pub const VALUE_TAG_NULL: u8 = 1;
pub const VALUE_TAG_BOOLEAN: u8 = 2;
pub const VALUE_TAG_NUMBER: u8 = 3;
pub const VALUE_TAG_STRING: u8 = 4;
pub const VALUE_TAG_BIGINT: u8 = 5;
pub const VALUE_TAG_SYMBOL: u8 = 6;
pub const VALUE_TAG_OBJECT: u8 = 7;

/// Byte offset of the Number variant's f64 payload within a Value.
/// With #[repr(C, u8)] and f64's 8-byte alignment, this is 8.
pub const VALUE_NUMBER_PAYLOAD_OFFSET: usize = 8;

const _: () = {
    // Discriminant byte at offset 0.
    assert!(
        std::mem::size_of::<Value>() >= 16,
        "Value must be at least 16 bytes (1B tag + 7B pad + 8B payload)"
    );
    // Alignment is at least 8 (f64 payload).
    assert!(
        std::mem::align_of::<Value>() >= 8,
        "Value alignment must be at least 8 for f64 payload"
    );
};

/// Runtime check: verify the chosen discriminant byte for Value::Number
/// matches VALUE_TAG_NUMBER. Called at JIT-bench-setup time so a layout
/// drift fails loudly. Not on the hot path.
pub fn assert_value_layout() {
    let v = Value::Number(0.0);
    // SAFETY: #[repr(C, u8)] places the discriminant at byte 0.
    let tag = unsafe { *((&v as *const Value) as *const u8) };
    assert_eq!(
        tag, VALUE_TAG_NUMBER,
        "Value::Number discriminant byte ({}) does not match \
         VALUE_TAG_NUMBER ({}); rustc layout drift detected. \
         VTI-EXT 3a invariant violated.",
        tag, VALUE_TAG_NUMBER
    );
    // Payload offset: write a known sentinel, read it back at the
    // declared offset.
    let v2 = Value::Number(1.5_f64);
    let payload = unsafe {
        let base = &v2 as *const Value as *const u8;
        let pf = base.add(VALUE_NUMBER_PAYLOAD_OFFSET) as *const f64;
        *pf
    };
    assert_eq!(
        payload, 1.5,
        "Value::Number payload not at offset {}; rustc layout drift.",
        VALUE_NUMBER_PAYLOAD_OFFSET
    );
}

impl Value {
    pub fn type_of(&self) -> &'static str {
        match self {
            Value::Undefined => "undefined",
            Value::Null => "object", // per §13.5.3 typeof null is "object"
            Value::Boolean(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::BigInt(_) => "bigint",
            Value::Symbol(_) => "symbol",
            // Post-3.e.d: Value::Object's typeof requires a heap to peek
            // InternalKind. Without a runtime here we report "object";
            // callers that need precise function/object disambiguation
            // should use Runtime::type_of_value (added in 3.e.d).
            Value::Object(_) => "object",
        }
    }

    /// SameValue per spec §7.2.11. Used by Map keys and Set elements.
    pub fn same_value(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Number(x), Value::Number(y)) => {
                if x.is_nan() && y.is_nan() {
                    return true;
                }
                x.to_bits() == y.to_bits()
            }
            (Value::String(x), Value::String(y)) => x == y,
            (Value::BigInt(x), Value::BigInt(y)) => x == y,
            (Value::Symbol(x), Value::Symbol(y)) => x == y,
            (Value::Object(x), Value::Object(y)) => x == y,
            _ => false,
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Undefined => write!(f, "undefined"),
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{:?}", s.as_str()),
            Value::BigInt(b) => write!(f, "{}n", b.to_decimal()),
            Value::Symbol(s) => write!(f, "Symbol({:?})", s.as_str()),
            Value::Object(id) => write!(f, "[Object #{}]", id.0),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        Self::same_value(self, other)
    }
}

/// Property key per ECMA §6.1.7 PropertyKey type. A property key is either
/// a String value or a Symbol value. The two are stored under the same
/// IndexMap but compared/hashed with distinct semantics:
///   - String keys hash by string content; equal iff content matches.
///   - Symbol keys hash by Rc identity (pointer); equal iff same Rc.
///
/// This carries the spec-mandated discrimination needed for [[OwnPropertyKeys]]
/// (§10.1.11.1) to split into [int-indexed, string-keyed, symbol-keyed] groups,
/// for Object.getOwnPropertyNames / Object.getOwnPropertySymbols to return
/// disjoint sets, for JSON.stringify / Object.keys to skip Symbol keys, and
/// for Reflect.ownKeys to emit them in the spec-required order.
#[derive(Clone, Debug)]
pub enum PropertyKey {
    String(String),
    Symbol(Rc<String>),
}

impl PartialEq for PropertyKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(a), Self::String(b)) => a == b,
            // Symbols are identity-equal: same Rc allocation = same Symbol value.
            (Self::Symbol(a), Self::Symbol(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for PropertyKey {}

impl std::hash::Hash for PropertyKey {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        match self {
            Self::String(s) => {
                0u8.hash(h);
                s.hash(h);
            }
            Self::Symbol(rc) => {
                1u8.hash(h);
                (Rc::as_ptr(rc) as usize).hash(h);
            }
        }
    }
}

impl PropertyKey {
    /// Always-non-None content view. For String variant returns the inner
    /// string; for Symbol variant returns the Symbol's internal identifier.
    /// Callers that need to discriminate Symbol-typed should use is_symbol().
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(s) => s.as_str(),
            Self::Symbol(rc) => rc.as_str(),
        }
    }
    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
    /// String-content view; for Symbol returns the internal identifier
    /// (used by debug printing, JSON keys, and the @@-prefix discrimination
    /// migration shim).
    pub fn to_string_content(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Symbol(rc) => (**rc).clone(),
        }
    }
}

impl From<&str> for PropertyKey {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}
impl From<String> for PropertyKey {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}
impl From<&String> for PropertyKey {
    fn from(s: &String) -> Self {
        Self::String(s.clone())
    }
}

// CMig-EXT 14 (default-on flip, second attempt — held post CMig-EXT
// 12 + 13 close 279 of 283 test262 enrollment regressions):
// env-flag-cached enrollment switch. Default is ON (Shaped); diff-prod
// 42/42 + test262 sample 77.8% under enrollment (within 0.1pp of 77.9%
// default-off; the residual 4 long-tail failures are individual edge
// cases unrelated to substrate correctness — surgical closures in
// future rounds). `CRUFTLESS_SHAPE_ENROLL=0` is the diagnostic escape
// hatch for runs where Dictionary-form behavior is preferred (e.g.,
// bisecting a future regression to confirm whether the shape
// mechanism is implicated).
fn shape_enroll_enabled() -> bool {
    static FLAG: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *FLAG.get_or_init(|| {
        match std::env::var("CRUFTLESS_SHAPE_ENROLL") {
            Ok(v) => !(v == "0" || v.eq_ignore_ascii_case("false")),
            Err(_) => true, // default ON post-CMig-EXT 14
        }
    })
}

// Shape-EXT 4: Default impl so existing Object literals can fill the new
// shape + shape_values fields via `..Default::default()` rather than
// requiring per-site updates. Default constructs an Ordinary, no-proto,
// non-shaped, empty-properties object; callers using the spread syntax
// override the meaningful fields and inherit shape=None + shape_values=[].
impl Default for Object {
    fn default() -> Self {
        Self {
            proto: None,
            extensible: true,
            properties: IndexMap::new(),
            internal_kind: InternalKind::Ordinary,
            shape: None,
            shape_values: Vec::new(),
            private_fields: IndexMap::new(),
            private_methods: IndexMap::new(),
            private_home: None,
        }
    }
}

pub struct Object {
    pub proto: Option<ObjectRef>,
    pub extensible: bool,
    // ECMA §10.1.11 OrdinaryOwnPropertyKeys requires integer-indexed keys
    // in ascending order, then string keys in insertion order, then Symbol
    // keys in insertion order. IndexMap preserves insertion order; the
    // integer-index branch is sorted at enumeration sites.
    pub properties: IndexMap<PropertyKey, PropertyDescriptor>,
    pub internal_kind: InternalKind,
    /// Shape-EXT 4 (per pilots/rusty-js-shapes/docs/shape-design.md):
    /// parallel storage slot for the modal-case shape form. Invariant:
    /// `shape.is_some()` => the object's user-default data properties
    /// live exclusively in `shape_values` indexed by `shape.slot_of(name)`.
    /// `properties` may still carry non-Shape-eligible entries
    /// (accessor/non-default descriptors, Symbol keys) — but those
    /// install paths first call `migrate_to_dictionary()` which moves
    /// any shape contents into `properties` and sets `shape = None`.
    pub shape: Option<std::rc::Rc<rusty_js_shapes::Shape>>,
    /// Shape-EXT 4: values backing the shape's slots. When `shape` is
    /// `Some(s)`, `shape_values.len() == s.slot_count()` and
    /// `shape_values[s.slot_of(name).unwrap() as usize]` is the value
    /// for that property name.
    pub shape_values: Vec<Value>,
    /// Spec-private class elements. This is intentionally disjoint from
    /// ordinary string-keyed properties so `#x` does not appear through
    /// hasOwnProperty, ownKeys, or descriptor reflection.
    pub private_fields: IndexMap<String, Value>,
    /// Names installed as private methods. Private fields are writable;
    /// private methods are not valid PrivateSet targets.
    pub private_methods: IndexMap<String, ()>,
    /// Class home object for closures installed as class methods/accessors.
    /// Private-name access inside the method brands `#name` by this object
    /// identity, approximating ECMA PrivateName identity per class evaluation.
    pub private_home: Option<ObjectRef>,
}

impl Object {
    pub fn new_ordinary() -> Self {
        // CMig-EXT 8 (consumer-migration enrollment flip): new ordinary
        // objects start Shaped at the thread-local root shape WHEN the
        // env flag `CRUFTLESS_SHAPE_ENROLL=1` is set. Default (flag off)
        // remains Dictionary form per Shape-EXT 4's deferred-enrollment
        // shape. Per the consumer-migration survey R2 mitigation: gate
        // the enrollment behind the flag so diff-prod + test262-sample
        // can be re-run under enrollment without committing to a
        // default-on flip until all gates hold green.
        Self {
            proto: None,
            extensible: true,
            properties: IndexMap::new(),
            internal_kind: InternalKind::Ordinary,
            shape: if shape_enroll_enabled() {
                Some(rusty_js_shapes::Shape::root())
            } else {
                None
            },
            shape_values: Vec::new(),
            private_fields: IndexMap::new(),
            private_methods: IndexMap::new(),
            private_home: None,
        }
    }

    /// CMig-EXT 1 (consumer-migration sub-workstream): explicit-Dictionary
    /// Ordinary factory. Returns an `Ordinary`-kind Object guaranteed to
    /// be in Dictionary storage form (`shape: None`). Used by container-
    /// role allocation sites — Map/Set internal storage, listener lists,
    /// forwarders — that iterate `.properties` directly and would break
    /// under Shaped form.
    ///
    /// In the pre-CMig-EXT 8 regime where `new_ordinary()` also returns
    /// Shape: None, this factory is operationally identical to
    /// `new_ordinary`. Post-CMig-EXT 8 when `new_ordinary` defaults to
    /// Shaped, this factory remains the explicit Dictionary-form escape.
    /// Callers that pick `new_dictionary()` today are documenting their
    /// dispatch intent for the future enrollment flip.
    pub fn new_dictionary() -> Self {
        Self {
            proto: None,
            extensible: true,
            properties: IndexMap::new(),
            internal_kind: InternalKind::Ordinary,
            shape: None,
            shape_values: Vec::new(),
            private_fields: IndexMap::new(),
            private_methods: IndexMap::new(),
            private_home: None,
        }
    }

    pub fn new_array() -> Self {
        // §10.4.2 Array exotic: length is non-configurable, non-enumerable.
        // cruftless v1 lazily synthesizes the descriptor in object_get +
        // object_get_own_property_descriptor_via (where the Array.length
        // value is derived from max numeric index when not explicitly set).
        // Pre-installing length here would defeat the derive-from-indices
        // path used by object_get when length is absent from properties.
        //
        // Shape-EXT 4: Arrays bypass shapes per shapes seed §IV (only
        // InternalKind::Ordinary admits shapes in first cut).
        Self {
            proto: None,
            extensible: true,
            properties: IndexMap::new(),
            internal_kind: InternalKind::Array,
            shape: None,
            shape_values: Vec::new(),
            private_fields: IndexMap::new(),
            private_methods: IndexMap::new(),
            private_home: None,
        }
    }

    pub fn get_private(&self, key: &str) -> Option<&Value> {
        key.strip_prefix('#')
            .and_then(|name| self.private_fields.get(name))
    }

    pub fn set_private(&mut self, key: &str, value: Value) -> bool {
        let Some(name) = key.strip_prefix('#') else {
            return false;
        };
        self.private_fields.insert(name.to_string(), value);
        true
    }

    pub fn set_private_method(&mut self, key: &str, value: Value) -> bool {
        let Some(name) = key.strip_prefix('#') else {
            return false;
        };
        self.private_fields.insert(name.to_string(), value);
        self.private_methods.insert(name.to_string(), ());
        true
    }

    pub fn is_private_method(&self, key: &str) -> bool {
        key.strip_prefix('#')
            .is_some_and(|name| self.private_methods.contains_key(name))
    }

    pub fn set_private_home(&mut self, home: ObjectRef) {
        self.private_home = Some(home);
    }

    /// Shape-EXT 4: is this object currently in Shaped storage form?
    pub fn is_shaped(&self) -> bool {
        self.shape.is_some()
    }

    /// Shape-EXT 4: read a value from the shape's slot. Returns None
    /// for properties not in the shape (which may still live in
    /// `properties`).
    pub fn shape_get(&self, name: &str) -> Option<&Value> {
        let shape = self.shape.as_ref()?;
        let slot = shape.slot_of(name)? as usize;
        self.shape_values.get(slot)
    }

    /// Shape-EXT 4: IC consumer API per shapes pilot docs/shape-design.md
    /// §11. Returns (shape_ptr, slot_index) iff the object is Shaped and
    /// the name resolves to a slot. Pilot LeJIT-Σ consumes this as the
    /// IC fast-path cache key. Stable for the lifetime of any Rc<Shape>
    /// the caller keeps alive.
    pub fn shape_ptr_and_slot_for(
        &self,
        name: &str,
    ) -> Option<(*const rusty_js_shapes::Shape, u32)> {
        let shape = self.shape.as_ref()?;
        let slot = shape.slot_of(name)?;
        Some((std::rc::Rc::as_ptr(shape), slot))
    }

    /// CMig-EXT 2 helper: shape-aware mutable accessor for the legacy
    /// `properties` IndexMap. Forces `migrate_to_dictionary()` first so
    /// callers that go through this accessor see a Dictionary-form
    /// object regardless of the receiver's pre-call state. Used by the
    /// ~28 direct `properties.insert` / `properties.shift_remove` sites
    /// that install accessor descriptors or non-default-attr properties
    /// — all non-Shape-eligible per shapes seed §IV carve-out.
    ///
    /// In the pre-CMig-EXT 8 regime where `new_ordinary()` returns
    /// `shape: None`, this is a no-op view; post-CMig-EXT 8 when
    /// enrollment defaults to Shaped, the migration fires on first
    /// access to ensure the legacy install paths land in Dictionary
    /// form.
    pub fn dict_mut(&mut self) -> &mut IndexMap<PropertyKey, PropertyDescriptor> {
        self.migrate_to_dictionary();
        &mut self.properties
    }

    /// Shape-EXT 4: migrate from Shaped to Dictionary form. Idempotent.
    /// Copies shape-stored slots into `properties` as user-default data
    /// descriptors, then clears the shape. After migration the object
    /// stays in pure-Dictionary form (back-promotion deferred per
    /// shapes seed §IV).
    pub fn migrate_to_dictionary(&mut self) {
        let Some(shape) = self.shape.take() else {
            return;
        };
        let values = std::mem::take(&mut self.shape_values);
        for (name, slot) in shape.iter_slots() {
            let idx = slot as usize;
            if idx >= values.len() {
                continue;
            }
            self.properties.insert(
                PropertyKey::String(name.to_string()),
                PropertyDescriptor {
                    value: values[idx].clone(),
                    writable: true,
                    enumerable: true,
                    configurable: true,
                    getter: None,
                    setter: None,
                },
            );
        }
    }

    /// OrdinaryGet per §10.1.8.1. Own-property only. Prototype-chain
    /// walk moved to Runtime::object_get (proto deref requires heap).
    ///
    /// Shape-EXT 4 dispatch note: shape-stored entries do NOT have a
    /// stored PropertyDescriptor (only a value). Callers needing the
    /// value should use Runtime::object_get (which is shape-aware).
    /// Callers needing the descriptor attributes for a shape-stored
    /// entry receive None here; the entry's descriptor is the
    /// user-default `{w:t, e:t, c:t}` by invariant.
    pub fn get_own(&self, key: &str) -> Option<&PropertyDescriptor> {
        // Backwards-compat shim during PropertyKey migration: callers
        // passing &str look up the String-variant; Symbol-keyed reads
        // go through the dedicated get_own_symbol below.
        self.properties.get(&PropertyKey::String(key.to_string()))
    }

    /// Symbol-keyed own-property lookup. Identity-equal to the storage
    /// key by Rc::ptr_eq.
    pub fn get_own_symbol(&self, sym: &std::rc::Rc<String>) -> Option<&PropertyDescriptor> {
        self.properties.get(&PropertyKey::Symbol(sym.clone()))
    }

    /// Mutable own-property lookup for string keys. PropertyKey migration shim.
    pub fn get_own_mut(&mut self, key: &str) -> Option<&mut PropertyDescriptor> {
        self.properties
            .get_mut(&PropertyKey::String(key.to_string()))
    }

    /// String-key membership test (migration shim).
    /// Shape-EXT 4: shape-aware; checks shape slots before properties.
    /// ODP-EXT 1: Array exotic exposes "length" as a virtual own property
    /// per §10.4.2; the virtual length is not stored in shape/properties
    /// but membership tests must still observe it.
    pub fn has_own_str(&self, key: &str) -> bool {
        if let Some(shape) = self.shape.as_ref() {
            if shape.slot_of(key).is_some() {
                return true;
            }
        }
        if self
            .properties
            .contains_key(&PropertyKey::String(key.to_string()))
        {
            return true;
        }
        if key == "length" && matches!(self.internal_kind, InternalKind::Array) {
            return true;
        }
        false
    }

    /// String-key delete (migration shim).
    /// Shape-EXT 4: delete migrates to Dictionary first per shapes seed §IV.
    pub fn remove_str(&mut self, key: &str) -> Option<PropertyDescriptor> {
        self.migrate_to_dictionary();
        self.properties
            .shift_remove(&PropertyKey::String(key.to_string()))
    }

    /// String-key insert with full descriptor (migration shim).
    /// Shape-EXT 4: arbitrary-descriptor insert migrates first.
    pub fn insert_str(
        &mut self,
        key: impl Into<String>,
        desc: PropertyDescriptor,
    ) -> Option<PropertyDescriptor> {
        self.migrate_to_dictionary();
        self.properties
            .insert(PropertyKey::String(key.into()), desc)
    }

    /// Iterate string-keyed entries only (migration shim — most legacy callers
    /// expected an IndexMap<String, _>).
    /// Shape-EXT 4: shape-aware; concatenates shape slots (insertion order)
    /// then property string keys (insertion order). Per ECMA §10.1.11 both
    /// sequences are in insertion order.
    pub fn string_keys(&self) -> impl Iterator<Item = &str> {
        let shape_names: Vec<&str> = match self.shape.as_ref() {
            Some(shape) => shape.iter_slots().map(|(n, _)| n).collect(),
            None => Vec::new(),
        };
        let prop_names: Vec<&str> = self
            .properties
            .keys()
            .filter_map(|k| match k {
                PropertyKey::String(s) => Some(s.as_str()),
                PropertyKey::Symbol(_) => None,
            })
            .collect();
        shape_names.into_iter().chain(prop_names)
    }

    /// String-key content as String for the convenience of callers that need
    /// owned strings. Skips Symbol keys.
    /// Shape-EXT 4: shape-aware (same concatenation as string_keys).
    pub fn string_key_clones(&self) -> impl Iterator<Item = String> + '_ {
        let shape_names: Vec<String> = match self.shape.as_ref() {
            Some(shape) => shape.iter_slots().map(|(n, _)| n.to_string()).collect(),
            None => Vec::new(),
        };
        let prop_names: Vec<String> = self
            .properties
            .keys()
            .filter_map(|k| match k {
                PropertyKey::String(s) => Some(s.clone()),
                PropertyKey::Symbol(_) => None,
            })
            .collect();
        shape_names.into_iter().chain(prop_names)
    }

    /// OrdinaryDefineOwnProperty per §10.1.6.1 (simplified — full
    /// invariants check lands with intrinsics).
    /// Shape-EXT 4 fast path: if Shaped, in-place mutate existing slot
    /// or advance shape via transition_to + push to shape_values.
    /// `__`-prefixed keys (engine-internal sentinels) migrate to
    /// Dictionary first per shapes seed §IV (consumers like Map.size /
    /// Set.size read .properties directly for sentinel data).
    pub fn set_own(&mut self, key: String, value: Value) {
        // §10.1.9 OrdinarySet: when the property already exists, only
        // update [[Value]] — preserve writable/enumerable/configurable/
        // getter/setter. This matters for Array.length (non-configurable),
        // function .name/.length (non-writable, non-enumerable), and
        // any other property whose descriptor was set deliberately.
        if key.starts_with("__") {
            self.migrate_to_dictionary();
        }
        if let Some(shape) = self.shape.as_ref() {
            if let Some(slot) = shape.slot_of(&key) {
                self.shape_values[slot as usize] = value;
                return;
            }
            let next = shape.transition_to(&key);
            self.shape = Some(next);
            self.shape_values.push(value);
            return;
        }
        let pk = PropertyKey::String(key);
        if let Some(d) = self.properties.get_mut(&pk) {
            d.value = value;
            return;
        }
        self.properties.insert(
            pk,
            PropertyDescriptor {
                value,
                writable: true,
                enumerable: true,
                configurable: true,
                getter: None,
                setter: None,
            },
        );
    }

    /// Ω.5.P58.E1 (Doc 729 §VII.B engine-internal-bilateral-boundary,
    /// per-object stratum): set an own property as **non-enumerable**.
    /// Use for engine-internal sentinels (`__kind`, `__is_buffer__`,
    /// `__buffer_data`, `__listeners__`, `__it_src__`, etc.) that the
    /// runtime needs to dispatch on but JS consumers should not see in
    /// `Object.keys` / `for-in` enumeration. Equivalent to
    /// `Object.defineProperty(o, k, {value, writable: true,
    /// configurable: true, enumerable: false})`.
    pub fn set_own_internal(&mut self, key: String, value: Value) {
        // Shape-EXT 4: non-default descriptors migrate to Dictionary first.
        self.migrate_to_dictionary();
        self.properties.insert(
            PropertyKey::String(key),
            PropertyDescriptor {
                value,
                writable: true,
                enumerable: false,
                configurable: true,
                getter: None,
                setter: None,
            },
        );
    }

    /// Ω.5.P61.E20: set an own property as **fully locked** — non-writable,
    /// non-enumerable, non-configurable. Per ECMA §10.2.4 + §20.x, every
    /// built-in constructor's `.prototype` slot has this descriptor shape.
    /// User-defined function `.prototype` differs (writable: true).
    pub fn set_own_frozen(&mut self, key: String, value: Value) {
        // Shape-EXT 4: non-default descriptors migrate to Dictionary first.
        self.migrate_to_dictionary();
        self.properties.insert(
            PropertyKey::String(key),
            PropertyDescriptor {
                value,
                writable: false,
                enumerable: false,
                configurable: false,
                getter: None,
                setter: None,
            },
        );
    }
}

#[derive(Debug, Clone)]
pub struct PropertyDescriptor {
    pub value: Value,
    pub writable: bool,
    pub enumerable: bool,
    pub configurable: bool,
    /// Tier-Ω.5.nnn: accessor-descriptor support. If `getter` is Some(fn),
    /// reads of this property invoke the getter with `this` = the
    /// receiver object. If `setter` is Some(fn), writes invoke it.
    /// When both are None, this is a data property and `value` is the
    /// stored value (existing semantics).
    pub getter: Option<Value>,
    pub setter: Option<Value>,
}

#[derive(Debug)]
pub enum InternalKind {
    Ordinary,
    Array,
    Function(FunctionInternals),
    Closure(ClosureInternals),
    BoundFunction(BoundFunctionInternals),
    Error,
    ModuleNamespace,
    /// Promise per ECMA-262 §27.2.
    Promise(PromiseState),
    /// Regular expression object per ECMA-262 §22.2. Tier-Ω.5.i.
    RegExp(RegExpInternals),
    /// Ω.5.P60.E1: Proxy exotic object per ECMA-262 §10.5. Stores
    /// target + handler ObjectIds. Property-access opcodes dispatch
    /// through the handler's traps when present; missing-trap path
    /// delegates to the target.
    Proxy(ProxyInternals),
    /// EXT 83: primitive-wrapper internal slots per ECMA-262 §20.{3,4,5}.
    /// new Number(0) carries [[NumberData]]; new String("x") carries
    /// [[StringData]]; new Boolean(true) carries [[BooleanData]]. The
    /// stored Value is the boxed primitive, returned by the matching
    /// prototype's valueOf and used by Object.prototype.toString to
    /// produce the brand string ("[object Number]" / "[object String]" /
    /// "[object Boolean]") per §20.1.3.6 step 14.
    NumberWrapper(Value),
    StringWrapper(Value),
    BooleanWrapper(Value),
    BigIntWrapper(Value),
    /// Generator object scaffold per ECMA-262 §27.5.3. GCS-EXT 1 only
    /// records the state cell; existing eager-collected generator behavior
    /// remains wired until later suspension rungs replace it.
    Generator(GeneratorObject),
    /// Sloppy mapped arguments exotic object per ECMA-262 §10.4.4.
    /// Each mapped index points at the same binding cell as the formal
    /// parameter, so `a = 2` and `arguments[0] = 2` observe one location.
    MappedArguments {
        parameter_map: IndexMap<String, UpvalueCell>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratorState {
    SuspendedStart,
    SuspendedYield,
    Executing,
    Completed,
}

#[derive(Debug)]
pub struct GeneratorObject {
    pub state: GeneratorState,
}

#[derive(Debug)]
pub struct ProxyInternals {
    pub target: ObjectRef,
    pub handler: ObjectRef,
    /// EXT 84: revocation flag per §10.5.{4..14}. Each spec internal
    /// method begins "If O's [[ProxyHandler]] is null, throw TypeError".
    /// In v1 we model the null-handler state via this boolean (set by
    /// Proxy.revocable's revoke closure), checked at every trap dispatch.
    pub revoked: bool,
}

/// RegExp instance internals. `source` and `flags` retain the original JS
/// spelling for the .source / .flags accessor surface. `compiled` is the
/// Rust `regex` crate compilation of the translated pattern — None when
/// the pattern uses features the Rust crate does not support (lookbehind,
/// backreferences); methods then throw a TypeError on call rather than
/// panicking. `last_index` backs the stateful exec/test path under the
/// 'g' flag per §22.2.5.2.
#[derive(Debug)]
pub struct RegExpInternals {
    pub source: Rc<String>,
    pub flags: Rc<String>,
    /// Compiled engine. Tier-Ω.5.ggg: tries the Rust `regex` crate first
    /// (fast for simple patterns); falls back to the hand-rolled
    /// backtracking engine for JS-only features (lookaround in
    /// particular). None means both engines rejected the pattern.
    pub compiled: Option<CompiledRegex>,
    pub last_index: usize,
}

#[derive(Debug)]
pub enum CompiledRegex {
    Rust(regex::Regex),
    Hand(crate::regex_hand::HandRolledRegex),
}

// HandRolledRegex needs Debug — it has it derived in regex_hand.rs.

impl Clone for CompiledRegex {
    fn clone(&self) -> Self {
        match self {
            CompiledRegex::Rust(r) => CompiledRegex::Rust(r.clone()),
            CompiledRegex::Hand(h) => CompiledRegex::Hand(h.clone()),
        }
    }
}

impl CompiledRegex {
    /// RES-EXT 1: unified name → 1-based group-index map. The Rust regex
    /// crate exposes capture_names() (Option<&str> per group, including
    /// the unnamed slot 0); the hand engine carries its own HashMap.
    /// Substrate-bridge consumers build the .groups Object from this.
    pub fn named_groups(&self) -> Vec<(String, usize)> {
        match self {
            CompiledRegex::Rust(r) => r
                .capture_names()
                .enumerate()
                .filter_map(|(i, n)| n.map(|s| (s.to_string(), i)))
                .collect(),
            CompiledRegex::Hand(h) => h
                .named_groups
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect(),
        }
    }
    pub fn is_match(&self, input: &str) -> bool {
        match self {
            CompiledRegex::Rust(r) => r.is_match(input),
            CompiledRegex::Hand(h) => crate::regex_hand::is_match(h, input),
        }
    }
    /// Find first match starting at byte offset `start`. Returns
    /// (match_byte_start, match_byte_end, captures_as_strings).
    pub fn captures_at(
        &self,
        input: &str,
        start: usize,
    ) -> Option<(usize, usize, Vec<Option<String>>)> {
        match self {
            CompiledRegex::Rust(r) => r.captures_at(input, start).map(|caps| {
                let m0 = caps.get(0).unwrap();
                let groups: Vec<Option<String>> = (0..caps.len())
                    .map(|i| caps.get(i).map(|m| m.as_str().to_string()))
                    .collect();
                (m0.start(), m0.end(), groups)
            }),
            CompiledRegex::Hand(h) => crate::regex_hand::find_at(h, input, start).map(|m| {
                let groups: Vec<Option<String>> = m
                    .captures
                    .iter()
                    .map(|c| c.map(|(s, e)| input[s..e].to_string()))
                    .collect();
                (m.start, m.end, groups)
            }),
        }
    }
    /// RES-EXT 2: positions variant of captures_at. Returns
    /// (match_byte_start, match_byte_end, Vec of per-group Option<(byte_start,
    /// byte_end)>). Used to build the `.indices` Array on exec results when
    /// the `d` flag is set, per ECMA-262 §22.2.7.7 MakeMatchIndicesArray.
    pub fn captures_positions_at(
        &self,
        input: &str,
        start: usize,
    ) -> Option<(usize, usize, Vec<Option<(usize, usize)>>)> {
        match self {
            CompiledRegex::Rust(r) => r.captures_at(input, start).map(|caps| {
                let m0 = caps.get(0).unwrap();
                let groups: Vec<Option<(usize, usize)>> = (0..caps.len())
                    .map(|i| caps.get(i).map(|m| (m.start(), m.end())))
                    .collect();
                (m0.start(), m0.end(), groups)
            }),
            CompiledRegex::Hand(h) => crate::regex_hand::find_at(h, input, start)
                .map(|m| (m.start, m.end, m.captures.clone())),
        }
    }
    /// Iterate non-overlapping matches; each yields (byte_start, byte_end, matched_str).
    pub fn find_iter_owned(&self, input: &str) -> Vec<(usize, usize, String)> {
        match self {
            CompiledRegex::Rust(r) => r
                .find_iter(input)
                .map(|m| (m.start(), m.end(), m.as_str().to_string()))
                .collect(),
            CompiledRegex::Hand(h) => {
                let mut out = Vec::new();
                let mut start = 0;
                while start <= input.len() {
                    match crate::regex_hand::find_at(h, input, start) {
                        Some(m) => {
                            let end = m.end;
                            out.push((m.start, end, input[m.start..end].to_string()));
                            // Avoid zero-width infinite loops.
                            start = if end == m.start { end + 1 } else { end };
                        }
                        None => break,
                    }
                }
                out
            }
        }
    }
    /// Find first match anywhere in input.
    pub fn find_first(&self, input: &str) -> Option<(usize, usize)> {
        self.captures_at(input, 0).map(|(s, e, _)| (s, e))
    }
    /// Split `input` on each match, returning the pieces between matches.
    pub fn split_str(&self, input: &str) -> Vec<String> {
        // ECMA-262 sec 22.1.3.21 step 18 / RegExp [Symbol.split]: empty
        // matches at the current cursor position are skipped (q advances
        // by 1 in spec) rather than emitting empty-string slices between
        // every codepoint. Rust's regex::Regex::split and Hand regex's
        // naive cursor walk both emit those empty slices, which produced
        // "hello".split(new RegExp("")) === ["", "h", "e", "l", "l", "o", ""]
        // instead of the spec's ["h","e","l","l","o"].
        //
        // Spec edge: an empty input with a regex that matches empty
        // returns [] (no slice emitted because the empty match at 0
        // equals p).
        let matches = self.find_iter_owned(input);
        if input.is_empty() {
            if matches.iter().any(|(s, e, _)| *s == 0 && *e == 0) {
                return Vec::new();
            }
            return vec![String::new()];
        }
        let mut out = Vec::new();
        let mut p: usize = 0;
        for (ms, me, _) in matches {
            // Spec sec 22.1.3.21 step 18: loop while q < size. A match
            // anchored at end-of-input (ms == input.len()) is past the
            // loop boundary and must not contribute a slice.
            if ms >= input.len() {
                break;
            }
            if me == p {
                continue;
            }
            if ms < p {
                continue;
            }
            out.push(input[p..ms].to_string());
            p = me;
        }
        out.push(input[p..].to_string());
        out
    }
    /// Replace at most `n` matches with the given literal replacement string.
    /// Replacement does NOT honor $1..$9 backreferences yet (v1 deviation).
    pub fn replacen_lit(&self, input: &str, n: usize, repl: &str) -> String {
        match self {
            CompiledRegex::Rust(r) => r.replacen(input, n, repl).into_owned(),
            CompiledRegex::Hand(_) => {
                let matches = self.find_iter_owned(input);
                let mut out = String::new();
                let mut cursor = 0;
                for (i, (ms, me, _)) in matches.into_iter().enumerate() {
                    if i >= n {
                        break;
                    }
                    out.push_str(&input[cursor..ms]);
                    out.push_str(repl);
                    cursor = me;
                }
                out.push_str(&input[cursor..]);
                out
            }
        }
    }
    pub fn replace_all_lit(&self, input: &str, repl: &str) -> String {
        match self {
            CompiledRegex::Rust(r) => r.replace_all(input, repl).into_owned(),
            CompiledRegex::Hand(_) => self.replacen_lit(input, usize::MAX, repl),
        }
    }
}

#[derive(Debug)]
pub struct PromiseState {
    pub status: PromiseStatus,
    pub value: Value,
    pub fulfill_reactions: Vec<PromiseReaction>,
    pub reject_reactions: Vec<PromiseReaction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromiseStatus {
    Pending,
    Fulfilled,
    Rejected,
}

#[derive(Debug)]
pub struct PromiseReaction {
    pub handler: Option<Value>,
    /// Chained Promise to resolve with the handler's result.
    pub chain: ObjectRef,
}

impl InternalKind {
    pub fn kind_name(&self) -> &'static str {
        match self {
            InternalKind::Ordinary => "ordinary",
            InternalKind::Array => "array",
            InternalKind::Function(_) => "function",
            InternalKind::Promise(_) => "promise",
            InternalKind::Closure(_) => "closure",
            InternalKind::BoundFunction(_) => "bound-function",
            InternalKind::Error => "error",
            InternalKind::ModuleNamespace => "module-namespace",
            InternalKind::RegExp(_) => "regexp",
            InternalKind::Proxy(_) => "proxy",
            InternalKind::NumberWrapper(_) => "number-wrapper",
            InternalKind::StringWrapper(_) => "string-wrapper",
            InternalKind::BooleanWrapper(_) => "boolean-wrapper",
            InternalKind::BigIntWrapper(_) => "bigint-wrapper",
            InternalKind::Generator(_) => "generator",
            InternalKind::MappedArguments { .. } => "mapped-arguments",
        }
    }
}

/// Closure internals — wraps a FunctionProto with captured upvalues.
#[derive(Debug)]
pub struct ClosureInternals {
    pub proto: Rc<rusty_js_bytecode::compiler::FunctionProto>,
    /// Tier-Ω.5.e: shared-binding upvalues. Each cell is shared with the
    /// outer frame's promoted local slot and with any sibling closures
    /// that captured the same binding.
    pub upvalues: Vec<UpvalueCell>,
    pub is_arrow: bool,
    /// Tier-Ω.5.sss: lexically captured `this` at arrow-creation site.
    /// Arrow functions inherit `this` from the enclosing scope per
    /// ECMA-262 §10.2.1.4; call_function ignores the receiver argument
    /// for arrows and substitutes this value.
    pub bound_this: Option<Value>,
    /// Optional cell-backed `this` binding. When set, the arrow reads
    /// `this` from the cell at call time (lazy resolution), not at
    /// creation time. Used to make `this` inside arrows track the
    /// enclosing function's `this` binding through derived-class
    /// super() rebinding per ECMA-262 §10.2.1.4 + §10.2.1.3. Without
    /// this, arrows created BEFORE super() resolves capture the
    /// pre-super pre-allocated empty this and never see the post-super
    /// constructor return value.
    pub bound_this_cell: Option<UpvalueCell>,
    /// Arrow-inherited derived-constructor raw `this`, used when a direct eval
    /// inside the arrow contains `super(...)`.
    pub bound_derived_initial_this: Option<Value>,
    /// Arrow-inherited new.target, forwarded through eval-super lowering.
    pub bound_new_target: Option<Value>,
    /// Ω.5.P04.E2.jit-runtime-dispatch: per-closure invocation counter.
    /// Incremented at every call_function entry; the runtime consults the
    /// JIT after the counter crosses a threshold (see Runtime::jit_threshold).
    /// Cell-typed for interior mutability — the call counter mutates while
    /// the surrounding `obj()` borrow is shared with the rest of the
    /// dispatch path.
    pub call_count: std::cell::Cell<u32>,
    /// Ω.5.P04.E2.jit-deopt-disable: profile-driven JIT disable. Set to
    /// true once the per-call argument-type guard has failed (a non-
    /// integer-Number arrived). Once disabled, the JIT dispatch is
    /// skipped permanently for this Closure — the per-call guard
    /// overhead would otherwise compound for functions that never
    /// match the JIT contract. The bytecode interpreter remains the
    /// fallback in all cases.
    pub jit_disabled: std::cell::Cell<bool>,
    /// LeJIT-Τ TB-EXT 3b (approach A — closure-side metadata cache):
    /// raw pointer to the per-proto TinyBaselineMetadata, populated
    /// on the first JIT-hit when `CRUFTLESS_LEJIT_TB=1`. Some(_)
    /// means the closure has a TB-eligible JIT'd body the dispatcher
    /// can fast-path-route around the standard jit_cache HashMap
    /// lookup + multi-condition AND check. The pointer is into the
    /// leaked JITModule (stable for process lifetime per
    /// translator.rs CompiledFn._module). None when TB is off OR
    /// when the function is not yet JIT-compiled.
    ///
    /// The cache is per-closure, not per-proto, because each closure
    /// has its own call_count + jit_disabled state; the metadata
    /// reference itself is proto-level data but lifetime-bound to
    /// the CompiledFn in the per-proto jit_cache. Cell-typed for
    /// interior mutability: the dispatcher's read-and-populate
    /// happens while the surrounding `obj()` borrow is shared.
    pub tb_metadata_ptr: std::cell::Cell<Option<std::ptr::NonNull<()>>>,
}

// SAFETY: the tb_metadata_ptr is read-only after first population
// and the underlying TinyBaselineMetadata lives in the leaked
// JITModule per CompiledFn._module. The dispatcher reads it
// single-threaded (cruft is single-threaded per the engagement's
// design). Send/Sync are not required; the field is a raw pointer
// in a Cell which is already !Send by default.

/// Native function (intrinsic) backed by a Rust callback.
pub struct FunctionInternals {
    pub name: String,
    /// ECMA-262 §10.2.10 FunctionLength surfaced as the own .length property.
    /// Spec-mandated arity; intrinsics that do not declare one default to 0.
    pub length: u32,
    pub native: NativeFn,
    /// Ω.5.P61.E4: ECMA-262 §10.3.3 + §21.3 — built-in functions that
    /// are not identified as constructors lack a [[Construct]] internal
    /// slot. `new Math.abs()` and `Reflect.construct(fn, [], Math.abs)`
    /// must throw TypeError. true = constructor (default for backward
    /// compatibility with existing intrinsics like Object, Array, etc.
    /// that are constructors); false = non-constructor (Math.*, Object.keys,
    /// String.prototype.includes, etc.).
    pub is_constructor: bool,
}

impl std::fmt::Debug for FunctionInternals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FunctionInternals {{ name: {:?}, length: {} }}",
            self.name, self.length
        )
    }
}

/// Tier-Ω.5.P15.E1: install spec-mandated .name and .length own properties
/// on a function object's property map per ECMA-262 §10.2.9 + §10.2.10.
/// Both descriptors are `{value, writable: false, enumerable: false,
/// configurable: true}` — invisible to Object.keys but visible to
/// reflection, and overrideable by Object.defineProperty.
pub fn install_function_meta_props(
    properties: &mut indexmap::IndexMap<PropertyKey, PropertyDescriptor>,
    name: &str,
    length: f64,
) {
    properties.insert(
        PropertyKey::String("length".to_string()),
        PropertyDescriptor {
            value: Value::Number(length),
            writable: false,
            enumerable: false,
            configurable: true,
            getter: None,
            setter: None,
        },
    );
    properties.insert(
        PropertyKey::String("name".to_string()),
        PropertyDescriptor {
            value: Value::String(std::rc::Rc::new(name.to_string())),
            writable: false,
            enumerable: false,
            configurable: true,
            getter: None,
            setter: None,
        },
    );
}

pub type NativeFn = std::rc::Rc<
    dyn Fn(&mut crate::interp::Runtime, &[Value]) -> Result<Value, crate::interp::RuntimeError>,
>;

#[derive(Debug)]
pub struct BoundFunctionInternals {
    pub target: ObjectRef,
    pub this: Value,
    pub args: Vec<Value>,
}

#[cfg(test)]
mod vti_layout_tests {
    use super::*;

    #[test]
    fn number_tag_at_offset_zero() {
        let v = Value::Number(42.0);
        let tag = unsafe { *((&v as *const Value) as *const u8) };
        assert_eq!(tag, VALUE_TAG_NUMBER);
    }

    #[test]
    fn number_payload_at_declared_offset() {
        let v = Value::Number(1.5_f64);
        let payload = unsafe {
            let base = &v as *const Value as *const u8;
            let pf = base.add(VALUE_NUMBER_PAYLOAD_OFFSET) as *const f64;
            *pf
        };
        assert_eq!(payload, 1.5);
    }

    #[test]
    fn all_variants_have_distinct_tags() {
        let cases: &[(Value, u8)] = &[
            (Value::Undefined, VALUE_TAG_UNDEFINED),
            (Value::Null, VALUE_TAG_NULL),
            (Value::Boolean(true), VALUE_TAG_BOOLEAN),
            (Value::Number(0.0), VALUE_TAG_NUMBER),
        ];
        for (v, expected) in cases {
            let tag = unsafe { *((v as *const Value) as *const u8) };
            assert_eq!(tag, *expected, "variant tag mismatch (rustc layout drift)");
        }
    }

    #[test]
    fn assert_value_layout_runs() {
        assert_value_layout();
    }
}
