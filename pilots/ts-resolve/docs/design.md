# TSR Design — TSR-EXT 1 (2026-05-24)

## 1. Parser architecture decision

**From-scratch, Pin-Art-consistent.** The discipline depends on the derived-from-constraints property; vendoring a Rust TS parser (swc-ecma-parser, biome) forfeits that property at this locale + imports ~30-80K LOC of substrate that has not been derived from spec constraints. The rusty-js-parser sibling is ~4K LOC for ECMAScript module-goal; the TS extension delta is bounded.

**Implementation strategy**: TSR's lexer + parser **wrap and extend** rusty-js-parser rather than duplicating it. Three layers:

```
pilots/ts-resolve/derived/src/
    lexer.rs    — wraps rusty-js-parser's Lexer; adds TS-specific tokens
                  (type-position keywords like `interface`, `type`,
                  `keyof`; not all are reserved at value position)
    parser.rs   — delegates to rusty-js-parser's expression + statement
                  parsers; intercepts at type-annotation positions to
                  consume + discard (erasure) or capture (sidecar)
    erase.rs    — type-position AST → erased rusty_js_ast pass; also
                  the lowering for enum (the only TS construct that
                  has runtime presence beyond JS)
    sidecar.rs  — TypeWitness records + emission API (TSR-EXT 5)
```

Estimated LOC at MVP: ~1500-1800 (well within Pred-tsr.1's ≤2000).

## 2. TS-surface-feature MVP budget

Frequency-ranked list of TS-over-JS surface forms (empirical scan deferred to TSR-EXT 2; this is the prior). Each row is one parsing concern.

| Tier | Feature | Frequency | LOC est | In MVP |
|---|---|---|---:|---|
| **A** | Type annotation on let/const | very high | 50 | ✅ |
| A | Type annotation on function param | very high | 60 | ✅ |
| A | Type annotation on function return | very high | 30 | ✅ |
| A | `interface Foo { ... }` declaration | very high | 150 | ✅ |
| A | `type Foo = ...` alias | very high | 80 | ✅ |
| A | `as` cast expression | high | 30 | ✅ |
| A | `!` non-null postfix | high | 10 | ✅ |
| A | Generic params on function `<T>` | high | 100 | ✅ |
| A | Generic params on call `f<T>()` | medium | 50 | ✅ |
| A | `enum Foo { A, B }` | medium | 200 | ✅ (only runtime construct) |
| **B** | Class field type annotations | medium | 40 | ✅ |
| B | Constructor `public x: T` params | medium | 60 | ✅ |
| B | `readonly` modifier | medium | 10 | ✅ |
| B | `?` optional property/param | high | 30 | ✅ |
| B | Union/intersection types `A \| B`, `A & B` | high | 120 | ✅ |
| B | Function type literals `(x: T) => U` | high | 80 | ✅ |
| B | Object type literals `{ x: T }` | high | 80 | ✅ |
| B | Tuple types `[A, B]` | medium | 40 | ✅ |
| B | Array types `T[]` and `Array<T>` | very high | 30 | ✅ |
| B | Indexed access types `T[K]` | medium | 40 | ✅ |
| B | `typeof` type query | medium | 30 | ✅ |
| **C** | Conditional types `T extends U ? X : Y` | low | 80 | ⚪ deferred |
| C | Mapped types `{ [K in keyof T]: ... }` | low | 100 | ⚪ deferred |
| C | Template literal types | low | 80 | ⚪ deferred |
| C | Decorators | low (legacy) | 200 | ⚪ deferred |
| C | Namespaces (with merging) | very low (legacy) | 300 | ⚪ deferred |
| C | JSX/TSX | medium (in React code) | 400 | ⚪ deferred (separate locale) |
| C | Triple-slash directives | low | 30 | ⚪ deferred |

**Tier-A + Tier-B totals**: ~1430 LOC. Well within Pred-tsr.1. Tier-C deferred to follow-on locales; for ~80% coverage of consumer `.ts` files Tier-A alone should suffice for application code (utility libs needing tier-B annotations).

## 3. AST integration

Reuse `rusty_js_ast::*` verbatim for everything that has runtime semantics. New types live in `pilots/ts-resolve/derived/src/ts_ast.rs`:

```rust
pub struct TsAnnotation {
    pub kind: TsTypeRef,
    pub span: Span,
}

pub enum TsTypeRef {
    Primitive(String),         // "string", "number", "boolean", "any", "unknown", "never", "void"
    Named { name: String, type_args: Vec<TsTypeRef> },
    Array(Box<TsTypeRef>),
    Tuple(Vec<TsTypeRef>),
    Union(Vec<TsTypeRef>),
    Intersection(Vec<TsTypeRef>),
    ObjectLit(Vec<(String, TsTypeRef)>),
    FnType { params: Vec<(String, TsTypeRef)>, ret: Box<TsTypeRef> },
    Indexed { target: Box<TsTypeRef>, index: Box<TsTypeRef> },
    TypeOf(String),            // typeof Foo
    Literal(TsLiteralVal),     // 42 | "abc" | true
}

pub enum TsLiteralVal { Str(String), Num(f64), Bool(bool), Null, Undefined }
```

These are constructed by the TS parser at type-position, then either:
- Dropped by `erase.rs::erase_module` before handing the AST to rusty-js-ir, OR
- Captured into the sidecar channel for downstream consumer opt-in (TSR-EXT 5)

C3 holds: no TS-specific residue reaches rusty-js-ir. The IR sees pure ECMAScript.

## 4. Erasure semantics

| Construct | Erasure |
|---|---|
| `let x: T = e` | `let x = e` (annotation dropped) |
| `function f(x: T, y?: U): R { ... }` | `function f(x, y) { ... }` (annotations + `?` dropped) |
| `interface I { ... }` | entire declaration dropped (no runtime presence) |
| `type Alias = ...` | entire declaration dropped |
| `e as T` | `e` (cast dropped) |
| `e!` | `e` (non-null dropped) |
| `class C<T> { x: T; constructor(public y: number) {...} }` | `class C { constructor(y) { this.y = y; } }` (public-param → assignment in ctor body) |
| `enum E { A, B = 5, C }` | lowered to `const E = Object.freeze({ A: 0, B: 5, C: 6, 0: "A", 5: "B", 6: "C" })` per TS reverse-mapping semantics for numeric enums |
| String enums | similar but no reverse mapping |
| `<T>e` cast (legacy syntax) | `e` (no MVP support; rejected at parse) |
| Generic instantiation `f<T>(x)` | `f(x)` (type args dropped) |
| `namespace Foo { ... }` | NOT IN MVP; parse error |

## 5. CLI routing

`cruftless/src/main.rs` (or the script-load entry) detects file extension. New dispatch:

```rust
let module_ast = match path.extension().and_then(|s| s.to_str()) {
    Some("ts") => ts_resolve::parse_module(&src)?.erase(),
    Some("tsx") => return Err("tsx not in MVP".into()),
    _ => rusty_js_parser::parse_module(&src)?,
};
```

The `.erase()` call returns a `rusty_js_ast::Module` that rusty-js-ir consumes identically to JS.

For `import "./foo"` resolution without explicit extension: try `.ts` then `.js` then `.mjs` (TS-convention order; matches tsc's `module: "node16"` resolution).

## 6. Annotation sidecar channel (TSR-EXT 5 design preview)

```rust
pub struct TypeWitness {
    pub kind: TypeWitnessKind,
    pub span: Span,
}

pub enum TypeWitnessKind {
    LocalBinding { slot_hint: u16, ty: TsTypeRef },
    FnParam { fn_id: FnId, param_idx: u8, ty: TsTypeRef },
    FnReturn { fn_id: FnId, ty: TsTypeRef },
    ClassField { class_id: ClassId, field: String, ty: TsTypeRef },
    EnumLowering { name: String, members: Vec<(String, f64)> },
}
```

The TS parser produces `(Module, Vec<TypeWitness>)`. The runtime opts in via `Runtime::with_type_witnesses(...)`; downstream consumers (HI/IHI/GPI/IPBR/VD/JIT) inspect the witnesses at compile-to-bytecode or JIT-trigger time and use them as profile-equivalent hints.

**Probe target at TSR-EXT 5**: a `.ts` fixture with `function process(items: string[]): string` produces a witness for `items: Array<string>`. The downstream consumer (IPBR's ForOfFastNext eligibility check) can SKIP the runtime `_arr`/`_i` shape probe when the iter_slot's source is statically witnessed as Array. Per IPBR-EXT 2's measured ~50ns/iter on shape probing, this is a measurable additional reclaim.

If the probe shows ≥10% additional reclaim on a TS-annotated version of string_url_sweep header_loop vs the JS version, **the research question (2) is empirically vindicated** and CruftScript's substrate-claim has grounding.

## 7. Per-feature LOC budget summary

| Round | Feature subset | LOC |
|---|---|---:|
| TSR-EXT 2 | Lexer (TS tokens) | ~150 |
| TSR-EXT 3 | Parser type-position consumer + annotation/interface/alias/cast/non-null | ~600 |
| TSR-EXT 3 | Parser generics + union/intersection/fn-type/object-type/tuple/array | ~480 |
| TSR-EXT 4 | erase.rs + enum lowering + CLI dispatch | ~250 |
| TSR-EXT 5 | sidecar.rs + Runtime opt-in + IPBR consumer probe | ~150 |
| **TOTAL** | | **~1630** |

Within Pred-tsr.1's ≤2000 LOC.

## 8. Open risks

- **R1**: TS uses contextual keywords (`type`, `interface`, `keyof`, `as`, `is`) — these are valid identifiers at value position but reserved at type position. The lexer cannot disambiguate; the parser must. Standard solution: lexer emits Identifier tokens for these; parser tracks context. ~30 LOC overhead.
- **R2**: `<T>e` (angle-bracket cast) vs `<T>(arg)` (generic call) is genuinely ambiguous in TS — TS itself rejects angle-bracket casts in `.tsx`. MVP follows the `.tsx` policy: only `as`-cast supported, angle-bracket cast rejected at parse. ~5 LOC.
- **R3**: Enum lowering correctness — TS's reverse-mapping semantics (`E[0] === "A"`) needs precise emission. Test fixture mandatory at TSR-EXT 4.
- **R4**: `public/protected/private` constructor params → assignment in ctor body. Rewrite step at TSR-EXT 4. ~40 LOC.
- **R5**: Pred-tsr.5 (TS-vs-JS IR identity) requires enum to NOT be in test fixtures (enums emit different IR by design). Document fixture-selection discipline at TSR-EXT 5.

## 9. Methodology for TSR-EXT 2-5

- **TSR-EXT 2**: TS lexer — wrap rusty-js-parser's Lexer; add contextual-keyword detection + arrow-token disambiguation (`=>` in type-fn vs value-arrow). Build: `cargo build -p ts-resolve`.
- **TSR-EXT 3**: TS parser — tier-A features end-to-end. Build + per-feature fixture test (`.ts` file → parse → erase → AST equals hand-written `.js` equivalent's AST).
- **TSR-EXT 4**: erasure pass + enum lowering + CLI extension dispatch. End-to-end `cruft examples/hello.ts`. Pred-tsr.2/.3/.4 booking.
- **TSR-EXT 5**: annotation sidecar + IPBR consumer probe + Pred-tsr.5 booking + chapter close.

## 10. Status

TSR-EXT 1 design committed. TSR-EXT 2 (lexer) next.
