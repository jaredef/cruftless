# strict-mode-destructuring-refs — Trajectory

## SMDR-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive "A" at T262C-EXT 1 close. First substrate-fix sub-locale spawned from T262C's matrix.

**Root cause documented in seed §I**. Fix path: replace pre-allocation + `emit_destructure` with BindingPattern→Expr conversion + `emit_destructure_assign` in the `ForBinding::Pattern(other)` branch at compiler.rs:1361.

**Founding artefacts**: seed + trajectory + scaffolded dirs. SMDR-EXT 1 (implementation + re-measure + close) next.
