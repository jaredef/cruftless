# iter-protocol-bytecode-rewrite — Resume Vector / Seed

**Locale tag**: `L.iter-protocol-bytecode-rewrite` (top-level per Doc 737 §IV)

**Status as of 2026-05-24**: **CHAPTER CLOSED at IPBR-EXT 2 (P2.a at deeper-layer-first design tier)**. All six Pred-ipbr.* HELD including the Pred-ipbr.6 discipline falsifier (≤3 rounds). CRB string_url_sweep 685 → 584 ms (-14.7% additional; -21.4% cumulative vs 743 baseline); cruft/node 6.99x → 6.21x (first sub-6.5×); A/B header_loop -21.8% additional (median 197 ms vs GPI 252). Locale closed in 1 implementation round; standing-rule-13 prospective-application thesis gets its second empirical corroboration.

**Historical status (founding)**: WORKSTREAM FOUNDED (IPBR-EXT 0). Spawned from `apparatus/locales/CANDIDATES.md` tier-A entry (a) after GPI chapter close identified the for-of protocol envelope as the new per-iter dominator on `string_url_sweep` header_loop. First test case of standing rule 13's prospective-application thesis (per `apparatus/docs/standing-rule-13-prospective-application.md`) at a second locale — the C1-C4 conditions all hold; design from the deeper-layer first.

**Workstream**: bytecode-rewrite the for-of dispatch pattern for Array/String receivers where the iterator is the well-known intrinsic (not user-overridden). Introduces a new `Op::ForOfFastNext` opcode (or pair) that runs an index-based scan over the underlying Array elements / String code units, eliminating per-`.next()` synthetic iterator-result-object allocation + the GetProp("next") / GetProp("done") / GetProp("value") trio that today's compiler emits for every iteration.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**:
- `interp-hot-intrinsics/` — IHI (cross-tier method-call IC; closed at IHI-EXT 11)
- `interp-getprop-ic/` — GPI (companion bytecode rewrite to IHI; closed at GPI-EXT 3)
**Composes with**:
- [apparatus/docs/standing-rule-13-prospective-application.md](../../apparatus/docs/standing-rule-13-prospective-application.md) — IPBR is the first **prospective falsifier** for the thesis; must close in ≤3 implementation rounds for the conditions to hold
- [Doc 741](../../docs/corpus-ref/741-the-multi-tier-cascade-pipeline-connects-an-empirical-materialization-of-doc-740-across-four-sibling-pilots-on-a-cruftless-cross-runtime-bench-fixture.md) — multi-tier cascade pattern; IPBR extends the cascade to the iter-protocol tier
- [Doc 740 §IV.2](../../docs/corpus-ref/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) — substrate-introduction signature; the rule of design-from-deeper-layer applies
- [interp-getprop-ic chapter close](../interp-getprop-ic/trajectory.md) — empirical anchor: post-GPI the per-iter dominator is the iter envelope
- [for-of compiler emission](../rusty-js-bytecode/derived/src/compiler.rs) at lines 1270-1427 — current emission shape; rewrite target
- [string_url_sweep component A/B probe](../cross-runtime-bench/fixtures/string_url_sweep/component-ab-probe.mjs) — empirical anchor; header_loop A/B delta currently ~252 ms post-GPI

## I. Telos

**Empirical answer to**: can a bytecode-rewrite IC at the for-of dispatch pattern close the per-iter cost surface of `.next() → {value, done}` protocol calls + result-object allocation, for Array/String receivers with the well-known intrinsic iterator?

The bench-anchored target: post-implementation, `string_url_sweep` header_loop drops an additional ≥15% beyond GPI-EXT 3's ~252 ms (toward ≤214 ms). Combined with IHI+GPI's cumulative reclaim, would push string_url_sweep CRB toward ≤600 ms (from 685 GPI), cruft/node ≤6× (from 6.99×).

### I.1 Current emission shape (per compiler.rs:1270-1427)

```
LoadLocal arr_slot          → [arr]
__engine_get_iterator       → [iter]            (helper call; cold-path tolerable)
StoreLocal iter_slot
LOOP:
  LoadLocal iter_slot       → [iter]
  GetProp "next"            → [iter, next_fn]
  Call 0                    → [result]           (heavy: dispatches into __engine_iter_next)
  Dup                       → [result, result]
  GetProp "done"            → [result, done]
  JumpIfTrue DONE           → [result]            (pops done)
  GetProp "value"           → [value]
  StoreLocal bind_slot
  ...body...
  Jump LOOP
DONE:
  Pop                       → []
```

Per-iter cost (post-GPI baseline):
- GetProp "next" + Call: ~250ns (CallMethodIcCached would help if "next" were in IHI; currently not)
- Result-object alloc + Object init + Dup: ~120ns
- GetProp "done" + JumpIfTrue: ~150ns
- GetProp "value" + StoreLocal: ~100ns
- **Total per-iter envelope: ~620ns**, dominating the inner-body work (~100-300ns of actual string-method invocation) by 2-6×.

### I.2 First-cut scope

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. Skip any intermediate IC-cache rounds; design the fully-rewritten dispatch from founding.

- **Op::ForOfArrayFastNext** (new opcode 0xFE) — operands: 2-byte iter-state slot, 2-byte bind slot, 4-byte done-jump target. Runs ONE iteration of the loop in a single dispatch: bumps index in iter-state, reads `arr[i]`, stores to bind slot, or jumps to DONE if exhausted. No iterator-result-object allocation.
- **Op::ForOfStringFastNext** (new opcode 0xFF) — symmetric for String receivers, code-unit-by-code-unit.
- **Substrate-introduction**: the iter_slot's stored Value becomes a polymorphic IterState (Array-with-index | String-with-byte-offset | Generic-iter-object). The rewrite ONLY fires when iter_slot was populated from an Array or String receiver via __engine_get_iterator's known fast path.
- **Rewrite trigger**: at first execution of the `__engine_get_iterator` helper, when receiver is Array or String and `%ArrayIteratorPrototype%.next` / `%StringIteratorPrototype%.next` are the well-known unmodified intrinsics, write the iter_slot with the specialized IterState + rewrite the IPBR target sequence in bytecode.

### I.3 Constraints (Pin-Art enumeration)

```
C1. Existing default-on paths byte-identical post-IPBR (only the
    rewritten paths take the fast route; user-overridden iterators,
    generators, Map/Set iterators, etc. take the unchanged path).
C2. Iterator-protocol observable contract preserved: a body that
    relies on .next()'s side effects (e.g., __engine_get_iterator
    returning a wrapped iterator) must NOT see the rewrite. Probe
    the receiver's [[Symbol.iterator]] identity at rewrite time;
    bail on mismatch.
C3. Bytecode rewrite SAFE per the IHI/GPI precedent: cruft single-
    threaded; bytecode is owned Vec<u8>; byte-aligned writes; idempotent.
    The rewrite spans MORE bytes than IHI/GPI (a multi-op pattern,
    not a single op-byte). Detailed at IPBR-EXT 1 design.
C4. break/continue/throw inside the body MUST land correctly; the
    rewritten op sequence preserves the loop-stack frame's break_patches
    and DONE-jump target.
C5. Override-safety gate: bail to slow path if Array.prototype[Symbol.iterator]
    or String.prototype[Symbol.iterator] is user-modified after first
    rewrite. Per-Runtime override-version counter (mirrors GPI hardening
    candidate; here it IS a first-cut concern because the iter protocol
    is more semantically rich).
C6. Per standing rule 13 + Doc 740 §IV.2: design from deeper-layer
    first; skip the iter-cache substrate-introduction mis-design tier.
C7. Per apparatus/docs/standing-rule-13-prospective-application.md §3 conditions
    C1-C4 must all hold prospectively:
    (C1.sibling-anchor) IHI + GPI bytecode-rewrite-at-first-hit pattern
                        is the empirical anchor
    (C2.shape-compat)  cruft's bytecode is Vec<u8>; iter_slot's Value
                        is a Runtime-managed slot; both compatible
    (C3.cost-positive) per-iter ~620ns → ~80ns predicted (≥7× reduction)
    (C4.bail-safe)     bail re-enters the original loop emission via
                        a recorded "original-bytes" stash in iter_slot
C8. Rule 11 5-axis pre-spawn check:
    (A1) component A/B: DONE via GPI-EXT 3 cost analysis + Doc 741
         instance precedent — iter envelope is the new dominator
    (A2) op-set coverage: 2 new opcodes; both index-based scans
    (A3) value-domain coverage: Array/String receivers only; covered
    (A4) locals-marshaling: N/A
    (A5) emission-shape coverage: for-of's emission shape at
         compiler.rs:1270-1427 is the load-bearing target; one shape
```

### I.4 Falsifiers

**Pred-ipbr.1**: per-rewrite-rule LOC ≤80 (raised from GPI's 50 because the rewrite spans more bytes + needs an IterState type).

**Pred-ipbr.2**: canonical fuzz (acc=-932188103) byte-identical post-implementation.

**Pred-ipbr.3**: diff-prod 42/42 holds.

**Pred-ipbr.4**: composition with all defaults ±5%.

**Pred-ipbr.5**: string_url_sweep header_loop drops ≥15% additional beyond GPI's ~252 ms (target ≤214 ms). Sub-target: CRB string_url_sweep cumulative reclaim ≥15% vs 743 baseline (target ≤631 ms from 685 GPI).

**Pred-ipbr.6 (DISCIPLINE FALSIFIER per apparatus/docs/standing-rule-13-prospective-application.md §5)**: locale closes in ≤3 implementation rounds. If it takes more, prospective-application thesis is partially falsified; diagnose which of C1-C4 in §I.3 conditions failed.

## II. Apparatus

- **Bytecode**: new opcodes `Op::ForOfArrayFastNext` (0xFE), `Op::ForOfStringFastNext` (0xFF). Operand shape detailed at IPBR-EXT 1.
- **Runtime state**: new `IterState` variants of Value or a sidecar table on Runtime; populated by `__engine_get_iterator` when receiver type matches.
- **Rewrite trigger**: at runtime entry into `__engine_iter_next` from a recognized fast-path iter_slot, walk back to the for-of pattern + rewrite to the FastNext opcode pair.
- **Override-safety**: per-Runtime counter on Array.prototype[@@iterator] and String.prototype[@@iterator]; check at rewrite time + on bail.
- **Bench instruments**: string_url_sweep CRB + component A/B probe at each round; arith_tight_loop + json_parse_transform for composition regression check.
- **Correctness instruments**: canonical fuzz + diff-prod + per-feature test fixtures (Array, String, Map, Set, generators, user-iterables, break/continue/throw, destructuring head).

## III. Methodology

1. **IPBR-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **IPBR-EXT 1** — design doc: bytecode shape + IterState representation + rewrite-pattern detection + bail-recovery mechanism + per-op LOC budget.
3. **IPBR-EXT 2** — infrastructure + Array fast-path implementation (the higher-frequency receiver type per CRB fixtures).
4. **IPBR-EXT 3** — String fast-path + composition probe + Pred-ipbr.* booking + chapter close.

(**Discipline target**: ≤3 implementation rounds, per Pred-ipbr.6.)

## IV. Carve-outs and bounded scope

- Array + String receivers only at first cut. Map/Set/generators/user-iterables fall through to the existing emission unchanged.
- Sync for-of only; for-await-of (which currently lowers identically per compiler.rs:1271) inherits the rewrite automatically but tested at IPBR-EXT 3.
- Aarch64 only.
- No JIT-tier dual at first cut; the JIT's existing tier may pick up the new opcodes if/when emission shape stabilizes (deferred candidate).

## V. Standing artefacts

- `pilots/iter-protocol-bytecode-rewrite/seed.md`, `trajectory.md`
- `pilots/iter-protocol-bytecode-rewrite/docs/design.md` (IPBR-EXT 1)
- `pilots/iter-protocol-bytecode-rewrite/fixtures/` for per-receiver-type synthetic tests
- Implementation lands in `pilots/rusty-js-bytecode/derived/src/op.rs` (new opcodes) + `pilots/rusty-js-runtime/derived/src/interp.rs` (handlers + rewrite trigger) + possibly minor adjustment to `compiler.rs` to mark emission sites with a sentinel byte for rewrite detection

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read GPI's chapter-close trajectory entries for the bytecode-rewrite + bail-mitigation precedent. Read apparatus/docs/standing-rule-13-prospective-application.md §3 (the four C-conditions) and §5 (the falsifier this locale tests). Read compiler.rs:1270-1427 for the current for-of emission shape that the rewrite must compose with.
