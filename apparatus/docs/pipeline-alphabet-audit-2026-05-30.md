# Pipeline alphabet audit — 2026-05-30

A worked instance of the resolver-instance-pipeline tracing protocol per Doc 729, run under keeper directive Telegram 10582 ("precisely discover where the beginning of the alphabet is; and if the ecmascript tokenizer, IR, etc pipeline all the way down to the host layer can explain the resolution pipeline"). The audit's substrate target is `ta[i] = v` typed-array element-set, motivated by the engine-architectural constraint surfaced at TAECSF-EXT 1 NEGATIVE (Finding TAECSF.3 — `ArrayBufferData.data: Vec<Value>` breaks view-aliasing under spec coercion).

The deliverable: a precise per-tier articulation of alphabet + resolution + type commitment from lexer through host substrate, identifying where the engine first commits to a representation that determines whether typed-array storage is byte-shaped or Value-cell-shaped. The finding is load-bearing for the proposed precursor architectural rung (candidate locale `typed-array-byte-storage-conformance`).

## Workspace pillars enumerated

- **lexer**: `pilots/rusty-js-parser/derived/src/lexer.rs` — tokenize source into Token stream with no type information.
- **parser/ast**: `pilots/rusty-js-parser/derived/src/expr.rs` — parse tokens into untyped AST.
- **ir**: `pilots/rusty-js-ir/derived/src/ir.rs` — generic `AssignIndex { name, value }` (line 302); still untyped, routes through `Expr::Call` for dispatch.
- **bytecode**: `pilots/rusty-js-bytecode/derived/src/op.rs` — `Op::SetIndex` at `0x83` (line 212); generic property-set opcode; no TA distinction.
- **runtime**: `pilots/rusty-js-runtime/derived/src/interp.rs` — `Op::SetIndex` handler (line ~14058); first tier with TA-vs-generic dispatch via runtime object inspection.
- **storage**: `pilots/rusty-js-runtime/derived/src/interp.rs` — `ArrayBufferRecord { data: Vec<Value> }` (line 451); stores Values directly at byte indices.

## Per-tier alphabet + resolution for `ta[i] = v`

| Tier | Alphabet (input) | Resolution (output) | Type commitment | Evidence |
|---|---|---|---|---|
| 1 Lexer | source text | token stream `[Ident, LBracket, Ident, RBracket, Eq, Ident]` | none | identical tokens for `ta[i]`, `obj[x]`, `dict[key]` |
| 2 Parser/AST | token stream | `AssignExpr { target: MemberExpr { computed: true } }` | none | `expr.rs` parses computed-key; AST node structurally generic; carries no object-kind tag |
| 3 IR | AST `AssignExpr` | `AssignIndex { name, value }` (`ir.rs:302`) | none | IR node is untyped; routes through `Expr::Call` |
| 4 Bytecode | IR `AssignIndex` | `Op::SetIndex` (`op.rs:212`) emitted by compiler | none | single generic opcode for all computed-key writes |
| 5 Runtime dispatch | stack `[obj, key, value]` + `Op::SetIndex` | object-kind dispatch (`interp.rs:14058`); canonical-numeric-index branch routes to `typed_array_set_index_checked` if `typed_array_views.get(&id)` matches | **FIRST TA-VS-GENERIC COMMITMENT** | line 14137–14144 checks view via runtime introspection |
| 6 Storage | `(id: ObjectRef, idx: usize, value: Value)` | `buf.data[byte_index] = value` where `buf: ArrayBufferRecord { data: Vec<Value> }` | **STORAGE COMMITMENT (Vec<Value>)** | `typed_array_set_index` line 604–625 |
| 7 Host | `Vec<Value>` over Rust allocator | bytes in process address space | none | just memory; no JS semantics |

## A. Where does the alphabet begin?

**Tier 5 (Runtime dispatch) is where the alphabet for TA element writes truly begins.**

Tiers 1–4 are uniformly **type-agnostic by ECMAScript's design** — JavaScript is dynamically typed; the parse-time and emit-time representation of `ta[i] = v` is structurally identical to `obj[x] = v` and `dict[k] = v`. The lexer, parser, IR, and bytecode tiers have no input that distinguishes a typed-array receiver from any other object receiver. The bytecode `Op::SetIndex` is the singular emit-time terminus for every computed-key property write.

Only at runtime, when the engine inspects `self.typed_array_views.get(&id)` at the canonical-numeric-index dispatch site (line 14137), does the engine **first learn**: "this is a typed-array element write, not a generic property write." This is the load-bearing introspection that establishes the TA-specific alphabet.

This observation is itself a finding (recorded as a new apparatus-tier entry below): in a dynamically-typed language, type-specific element semantics necessarily begin at runtime introspection; the upstream pipeline cannot encode them because the upstream pipeline has no input that distinguishes the type.

## B. Does the pipeline explain `Vec<Value>`?

**The pipeline explains `Vec<Value>` as accumulated drift, not as a deliberate substrate move.**

Evidence:

1. **Tiers 1–4 contribute zero TA-awareness**. The generic bytecode `Op::SetIndex` is the bottleneck; no parse-time proof of TA receiver type exists.
2. **Tier 5 is the first TA-aware junction**. By the time the runtime sees a TA, it has already committed to generic property storage as the universal fallback.
3. **No trajectory entry records `Vec<Value>` as a deliberate choice**. Searching `pilots/*/trajectory.md` and `pilots/*/findings.md` yields no entry stating "we chose `Vec<Value>` over `Vec<u8>` because...". Finding TAECSF.3 (dated 2026-05-30) discovers `Vec<Value>` as an engine-architectural constraint, not a documented design decision. The discovery only arrived after a spec-faithful coercion attempt failed in cluster testing.
4. **`ArrayBufferRecord` construction** allocates `data: Vec::with_capacity(byte_length)` populated with `Value::Number(0.0)` defaults. The byte_length is passed in; the engine stores Values at byte indices with no explicit choice-point documenting the byte-vs-Value decision.
5. **Symmetry locks the architecture**: `typed_array_get_index` (line ~558) reads return `Value`, paired symmetrically with `typed_array_set_index` writes. Any change to storage representation requires both read and write to migrate jointly; partial migration breaks the symmetry.

**Conclusion**: the pipeline does NOT explain `Vec<Value>` as a load-bearing design decision; it explains it as **the only representation possible given the type-agnostic upstream pipeline**, retrofit into typed arrays as the universal property-storage fallback. The architecture is incoherent with spec-faithful coercion the moment coercion enters at Tier 5; pre-coercion it was invisible because the storage pass-through hid the conflict.

## C. Where is the rectifying rung?

**Option (a): at the storage tier, migrating `ArrayBufferRecord.data` from `Vec<Value>` to `Vec<u8>` with NumberToRawBytes encoding per ECMA-262 §6.1.6.1 + view-tier dispatch on read.**

This rectification is **not within the TAECSF locale's telos**. The substrate scope spans:

1. `ArrayBufferRecord` struct migration: `Vec<Value>` → `Vec<u8>` with `byte_length` first-class.
2. `typed_array_set_index` rewrite: dispatch on view's element_kind; call `NumberToRawBytes(kind, coerced_value, isLittleEndian)` to produce `[u8; 1..8]`; write bytes into `buf.data[byte_index..byte_index + bpe]`.
3. `typed_array_get_index` rewrite: dispatch on view's element_kind; read `[u8; 1..8]` from `buf.data[byte_index..byte_index + bpe]`; decode via `RawBytesToNumeric(kind, bytes, isLittleEndian)`.
4. View construction sites: every `typed_array_views.insert` needs to encode the element kind (not just `bytes_per_element`); helps the read/write dispatch and the existing `__kind` slot may be promoted to a typed field.
5. Resizable-buffer paths: `resize_array_buffer` already does `buf.data.resize(new_len, Value::Number(0.0))` — must migrate to `buf.data.resize(new_len, 0u8)`.
6. DataView setters/getters at `intrinsics.rs:19842–19865`: currently use Rust saturating `as` cast on the Value-stored representation; migrate to call the same `NumberToRawBytes` helper for symmetry.
7. `__kind` internal slot: currently String-typed; consider a typed `TypedArrayKind` enum field on `TypedArrayViewRecord` to avoid string-comparison dispatch hot-path.

Recommended landing coordinate: **new locale `pilots/typed-array-byte-storage-conformance/`** founded as a sibling within arc `2026-05-28-array-exotic-substrate`. Multi-rung; the precursor rung (EXT 0) lands the struct migration + read/write rewrite + view-construction updates. Subsequent rungs (EXT 1+) land DataView migration + per-kind coercion (subsuming TAECSF sub-substrates (a) + (b)).

Once the precursor locale closes, TAECSF-EXT 2 becomes trivial — integer-kind coercion already lives in the byte-storage architecture; no separate dispatch needed.

## D. Architecture: broken or locally incompatible?

**Locally incompatible with spec-faithful coercion; globally coherent without it.**

Pre-EXT 1, the engine passed test262 view-aliasing tests because `Vec<Value>` round-tripped any value unchanged through cross-view writes. The architecture is **not broken per se**; it is **spec-incoherent the moment coercion enters at Tier 5**. Two requirements collide:

1. **Spec-faithful coercion at the write site** (TAECSF telos): different TA kinds coerce the same input Value differently per ECMA-262 §10.4.5.16.
2. **View-aliasing pass-through invariant** (test262 harness assumption + IEEE-754 NaN bit-pattern preservation per §6.1.6.1): a value written through one view's storage cell must be observable identically through any other view aliased to the same buffer cell.

These two cannot coexist at Tier 5 when values are stored as Values. Moving to bytes resolves the collision by making the coercion lossy at the write site (which is correct per spec — different kinds store different bytes) while preserving the view-aliasing invariant at the byte tier (which is also correct per spec — different views interpret the same bytes via different kind dispatches).

The architecture is not "broken" in isolation — it is **a partial fixed-point**: stable when coercion is absent, unstable when coercion is introduced. The fix is not a bug-fix-style local edit; it is a **fixed-point migration** that moves the substrate to a new fixed-point that includes spec coercion.

## Apparatus-tier finding (candidate)

**Finding APP.PIPELINE-1 (the dynamic-typing pipeline starts the type-specific alphabet at runtime)**: in a dynamically-typed language pipeline (lexer → parser → IR → bytecode → runtime → storage), type-specific element semantics cannot be encoded in any tier upstream of the runtime introspection. The "beginning of the alphabet" for a typed-array element write is the first introspection site that distinguishes typed-array receivers from generic objects — for this engine, the canonical-numeric-index branch of the `Op::SetIndex` handler. Substrate moves that attempt to encode TA-specific semantics in upstream tiers (e.g., by emitting a separate `Op::SetTypedArrayIndex`) require parse-time type proof that ECMAScript does not provide and would constitute a substantial language-feature deviation (akin to TypeScript's optional typing) rather than a runtime conformance fix.

**Predicts**: substrate rungs that try to push TA element-write coercion upstream of Tier 5 will require speculative type guards at parse / IR / bytecode emit that the dynamic language does not support; such rungs will either regress correctness (false-positive guards) or under-deliver (false-negative guards fall back to generic dispatch). The only sound rectification of architectural-coercion conflicts is at Tier 5 (dispatch) or Tier 6 (storage), never upstream.

**Promotion-readiness**: trajectory-and-findings-embedded; one-more-observation. Candidate apparatus standing-rule. Awaiting a second cross-pillar instance (e.g., the same pattern surfaces in BigInt arithmetic, RegExp coercion, or another type-specific dynamic-language fix). Recorded as new findings-ledger entry Entry 013.

## Recommended next action

Found locale `pilots/typed-array-byte-storage-conformance/` under arc `2026-05-28-array-exotic-substrate` with a multi-rung trajectory:

- **EXT 0**: struct migration + read/write rewrite + view-construction updates. Gates: TAMM ≥86; TAWR ≥67; diff-prod ≥64/48; no regression. Yield prediction: +0 to +5 in TAMM / TAWR / diff-prod purely from refactor coherence; downstream rungs deliver the coercion yield.
- **EXT 1+**: per-kind coercion subsuming TAECSF sub-substrates (a) integer-kind + (b) Float32 canonical-NaN preservation. Promoted readiness deferred until EXT 0 closes.

The locale's spawning requires keeper APPROVED of a proposal at `apparatus/proposals/pending/<slug>/proposal.md`. The substrate scope is large enough that the proposal should explicitly cite the four risks identified in this audit:

1. Read/write symmetry must be migrated jointly; partial migration breaks the architecture worse than pre-migration.
2. DataView setters/getters at `intrinsics.rs:19842–19865` carry the same `Vec<Value>` storage assumption; either migrate jointly or accept temporary DataView divergence per a recorded carve-out.
3. View aliasing (`copyIntoArrayBuffer` harness pattern in test262/harness/testTypedArray.js:107) must work at the byte tier post-migration; this is the same harness pattern that surfaced TAECSF.3 and is the most reliable gate cell for the precursor rung.
4. The `__kind` slot's role in dispatch may benefit from promotion to a typed `TypedArrayKind` enum on `TypedArrayViewRecord` to avoid string-comparison hot path; this is a co-yield optimization, not a precondition.
