# rusty-js-jit — Trajectory

Chronological resume anchors for the JIT workstream. Reads seed.md first; this file is the time-ordered record of substrate moves and their yields.

Format: one section per "EXT" (extension round); each round closes with a status block, a cumulative numbers table, and an open-scope list. Same shape as `pilots/rusty-js-ir/trajectory.md` and the top-level `trajectory.md`.

## JIT-EXT 0 — 2026-05-20 (workstream founding)

### Headline

Workstream founded at EXT 21 close of the parent rusty-bun engagement. Preconditions for Doc 731's R1–R8 baseline JIT are in place after EXT 21's twelve substrate moves: bytecode alphabet is P1–P4-faithful at the Op-enum level, IR alphabet has begun §XIII promotions (IsSpecObject), strict-mode tracking is plumbed end-to-end, CJS wrapper sloppy-default matches Node convention. No JIT code yet; this round establishes the workstream's scaffolding.

### Commits

| commit | tag | recognition |
|---|---|---|
| (pending) | (workstream founding) | `pilots/rusty-js-jit/seed.md` + `trajectory.md` written. Doc 731 §VII R1–R8 is the design target; §XIV §XV §XVI methodology applies as the gating discipline. Pin-Art tag prefix `Ω.5.P03.??.jit-*` (compiler-side) and `Ω.5.P04.??.jit-*` (runtime-side). |

### Substrate at JIT-EXT 0 close

- **No JIT code committed.** Seed and trajectory only.
- **Cranelift dependencies**: not yet added.
- **P4 site enumeration**: not yet started (queued as the first substrate move per seed §VI).
- **Bytecode alphabet snapshot**: ~50 Ops in `pilots/rusty-js-bytecode/derived/src/op.rs`; classification table pending.

### Conjecture status

The Doc 731 strong-form conjecture — that the JIT's structural complexity is bounded by upstream alphabet impurity to the point of LuaJIT-class line count — is currently a structural claim with no engagement-tier corroboration. JIT-EXT 0 founds the workstream that will provide the corroboration (or falsify it via §X Q1's failure mode: too many P4 sites for the strong form).

### Open scope at JIT-EXT 0 boundary

1. **First substrate move**: produce `pilots/rusty-js-jit/docs/op-p4-classification.md` by walking the Op enum and classifying each op as P1-pure (single Cranelift instruction or small composition) or P4 site (call into runtime helper). Cardinality of P4 column is the JIT's IC surface upper bound.

2. **Cranelift dependency PR**: once the classification suggests the JIT is structurally viable, add Cranelift codegen + frontend + jit + module crates to `Cargo.toml`. Create `pilots/rusty-js-jit/derived/Cargo.toml`.

3. **First end-to-end JIT compile**: pick the simplest function shape (an integer arithmetic function with no property access), produce JIT-compiled machine code, link into the runtime, verify it runs and produces the same result as the interpreter.

### Resume protocol

Read seed.md, then this trajectory's JIT-EXT 0 entry. The next substrate move is the P4 site enumeration; no Cranelift integration needed for that move. The classification is reading + thinking, not implementation.

Pin-Art tag count: 0 substrate moves so far (workstream founding only).

---

*JIT-EXT 0 closes the founding round. Subsequent rounds add substrate moves at the JIT tier.*
