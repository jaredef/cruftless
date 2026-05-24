//! TS-only AST nodes — produced by `TsParser` at type position; consumed
//! by `erase::erase_module` (dropped) or `sidecar.rs::emit_witnesses`
//! (captured for downstream IC/JIT consumers at TSR-EXT 5).
//!
//! No TS-only node ever reaches `rusty-js-ir`. C3 holds per seed.md §I.2.

use rusty_js_ast::Span;

/// Reference to a TS type at a type-position site.
#[derive(Debug, Clone, PartialEq)]
pub enum TsTypeRef {
    /// Primitive: `string` | `number` | `boolean` | `any` | `unknown` |
    /// `never` | `void` | `null` | `undefined` | `bigint` | `symbol` |
    /// `object`.
    Primitive(String),
    /// Named type reference; possibly generic. `Foo`, `Foo<T>`,
    /// `Foo<T, U>`.
    Named { name: String, type_args: Vec<TsTypeRef> },
    /// `T[]` (sugar for `Array<T>`); preserved separately to keep the
    /// witness output's shape close to source.
    Array(Box<TsTypeRef>),
    /// `[A, B, C]`.
    Tuple(Vec<TsTypeRef>),
    /// `A | B | C`.
    Union(Vec<TsTypeRef>),
    /// `A & B & C`.
    Intersection(Vec<TsTypeRef>),
    /// `{ x: T; y?: U }`.
    ObjectLit(Vec<TsObjectMember>),
    /// `(x: T) => U`.
    FnType { params: Vec<TsFnParam>, ret: Box<TsTypeRef> },
    /// `T[K]` indexed access.
    Indexed { target: Box<TsTypeRef>, index: Box<TsTypeRef> },
    /// `typeof Foo`.
    TypeOf(String),
    /// Literal type: `42`, `"abc"`, `true`.
    Literal(TsLiteralVal),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TsObjectMember {
    pub name: String,
    pub ty: TsTypeRef,
    pub optional: bool,
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TsFnParam {
    pub name: String,
    pub ty: TsTypeRef,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TsLiteralVal {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Undefined,
}

/// Annotation attached to a binding/parameter/return position. Captured
/// during parse; either erased before IR or emitted as a TypeWitness.
#[derive(Debug, Clone, PartialEq)]
pub struct TsAnnotation {
    pub ty: TsTypeRef,
    pub span: Span,
}

// ─── TSR-EXT 5 sidecar channel ──────────────────────────────────────────
//
// TypeWitness records carry erased type information from the parser to
// downstream substrate consumers (IHI/GPI/IPBR/VD/JIT). Each witness
// names a runtime construct (local slot / fn param / fn return / class
// field / enum) and the static type the annotation asserted.

#[derive(Debug, Clone, PartialEq)]
pub struct TypeWitness {
    pub kind: TypeWitnessKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeWitnessKind {
    /// Annotation on a let/const/var binding. The slot_hint is the
    /// source-order index of the binding within its enclosing function
    /// (the bytecode compiler resolves to a concrete slot during lowering).
    LocalBinding { name: String, ty: TsTypeRef },
    /// Annotation on a function parameter.
    FnParam { fn_name: Option<String>, param_idx: u8, ty: TsTypeRef },
    /// Annotation on a function return.
    FnReturn { fn_name: Option<String>, ty: TsTypeRef },
    /// Annotation on a class field.
    ClassField { class_name: String, field: String, ty: TsTypeRef },
    /// Enum declaration lowered to const-object; preserved here so JIT
    /// can recognize the frozen-shape opportunity.
    EnumLowering { name: String, members: Vec<(String, f64)> },
}
