---
locale: global-binding-surface-unification
coord: pilots/global-binding-surface-unification
parent: (top-level)
status: SPAWNED
opened: 2026-05-26
---

# Global Binding Surface Unification

## Telos

Collapse cruftless's bifurcated global variable environment record into one canonical surface: **the globalThis Object IS the global VarEnvRec** (per ECMA-262 §9.1.1.4 + §16.1). Today the substrate carries two surfaces — `Runtime.globals: HashMap<String, Value>` (interp.rs:87) and the globalThis JS Object (intrinsics.rs:466) — with a snapshot-copy at install time and runtime bridges (ES-EXT 3+4) patching the divergence downstream.

Per Doc 731 (alphabet purity upstream as the bound on downstream complexity): the current runtime alphabet exposes two binding surfaces, so the compiler can't speak a single canonical word and the runtime patches at every site. This locale removes one alphabet symbol.

## Apparatus

- **diff-prod**: 42/42 PASS standing gate; verified at every rung.
- **test262-sample**: current baseline 86.7% (6311/965/397 post-ES-EXT 3+4). Target: ≥86.7% maintained through rungs 1-4; potential further gain at rung 5 (ES-EXT 2 v2 re-enable).
- **Per-rung probes**: P1-P7 from eval-scope-binding-chain founding (especially P6/P7 for bridge equivalence under the new surface).

## Methodology

Five rungs in dependency order, each a single substrate move (Rule 4):

1. **Add `global_object: ObjectId` field to Runtime**; populate at install_global_this time. No removal yet. Pure metadata accessor — zero behavioral change.
2. **Migrate readers** (Op::LoadGlobal / LoadGlobalOrUndef + intrinsics readers) to read `global_object` first, HashMap fallback. ES-EXT 4 reverse-fallback collapses into the unified primary path.
3. **Migrate writers** (Op::StoreGlobal + intrinsics installation + host-globals registration) to write `global_object` only; remove ES-EXT 3 HashMap-then-mirror logic; HashMap becomes dual-write read-only mirror.
4. **Delete `Runtime.globals` field**; replace remaining call sites; remove ES-EXT 4 fallback paths (no longer needed). Rule 13 deeper-layer closure: bridges removed only after the unified surface is verified.
5. **Re-enable ES-EXT 2 v2**: pre-allocation passes UNCHANGED (IC.1 satisfied); top-level script-var emits StoreLocal AND a globalThis alias via DefineGlobalAliasedLocal opcode (or compiler-side write-site rewrite). Sub-rung set under es-foundation/.

## Carve-outs

- `Runtime.engine_helpers` (interp.rs:104) stays untouched. Doc 729 §VII.B bilateral boundary is orthogonal — engine-internal lowerings (__await, __apply, etc.) are NOT JS-visible and must NOT appear on globalThis.
- WeakRef/proxy/host-binding properties on globalThis stay routed through existing object_set paths.
- Realm overrides (interp.rs ~9060-9099) get their own migration step if needed; not in initial scope.

## Composes-with

- Doc 729 §VII.B (engine-internal bilateral) — preserved invariant
- Doc 731 (alphabet purity upstream) — guiding thesis
- Doc 737 (locale as coordinate) — locale topology
- Tier-Ω.5.dddd, Tier-Ω.5.qq (compiler pre-allocation invariant IC.1) — preserved through rung 5 design
- Tier-Ω.5.P55.E1 (engine_helpers bilateral) — preserved

## Parents

This locale parents the existing `pilots/eval-scope-binding-chain/` locale: ES-EXT 2 v2 (re-enabling script-mode top-level var) becomes a sub-rung set under this locale's rung 5, since the unified surface is the precondition for a constraint-respecting v2.

## Resume protocol

1. Read this seed + trajectory.md tail
2. Inspect current rung status; pick next un-landed rung
3. Verify standing gates (diff-prod 42/42, test262-sample ≥86.7%) before AND after each rung
4. Update trajectory at landing time; refresh manifest if locale topology changes
