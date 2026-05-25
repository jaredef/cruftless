# compartment-primitive — Resume Vector / Seed

**Locale tag**: `L.compartment-primitive` (top-level)

**Status as of 2026-05-25**: **WORKSTREAM FOUNDED (CP-EXT 0)**.

**Workstream**: implement the TC39 Compartments proposal (Stage 1) as a first-class JS primitive in cruft, scoped to the load-bearing subset that delivers Doc 736's capability-passing architectural property at the JS-API level. Where Doc 736 is "architecturally impossible supply-chain attack at the host level," this locale makes the same property usable directly by application authors via `new Compartment({globals, modules}).evaluate(src)` / `.import(specifier)`.

**Author**: 2026-05-25 session.
**Parent**: none (top-level).
**Composes with**:
- [Doc 736 capability-passing-runtime](../../docs/corpus-ref/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — Compartments are the JS-API expression of this discipline
- [docs/engagement/prospective/compartments-as-primitive.md](../../docs/engagement/prospective/compartments-as-primitive.md) — strategic analysis informing this locale
- [realm-substrate locale](../realm-substrate/) — RS-EXT 2 minimum-realm substrate is the load-bearing dependency
- [rusty-js-caps pilot](../rusty-js-caps/) — capability-handle pattern; endowments wire into this
- TC39 proposal: https://github.com/tc39/proposal-compartments

## I. Telos

A JS-visible `Compartment` class such that application authors can write:

```js
const dep = new Compartment({
    globals: {
        readFile: cap_fs_read_safe_paths_only,
    },
    modules: { 'safe-lib': depSource },
});
const result = dep.evaluate('readFile("/tmp/data.json")');
// CapabilityError without the endowment; intrinsic-pollution blocked by realm.
const exports = await dep.import('safe-lib');
```

The compartment isolates intrinsics (per RS-EXT 2 minimum-realm), restricts ambient bindings to the endowment set, and locks module resolution to the per-compartment map (no transitive imports outside it).

### I.1 First-cut scope (5 sub-rounds)

| Round | Scope | LOC | Pred-cp.* |
|---:|---|---:|---|
| CP-EXT 1 | Compartment class + ctor + `evaluate` (intrinsic-isolated, no endowments yet) | ~80 | probe: `new Compartment().evaluate('Array.prototype.x = 1; "ok"') === "ok"` and outer `Array.prototype.x === undefined` |
| CP-EXT 2 | `globals` endowment injection | ~40 | probe: `new Compartment({globals:{x:42}}).evaluate('x') === 42`; without globals, `x` throws ReferenceError |
| CP-EXT 3 | Per-compartment `globalThis` | ~60 | probe: `c.evaluate('globalThis')` ≠ outer `globalThis`; identity preserved across multiple `c.evaluate` calls |
| CP-EXT 4 | Per-compartment `modules` map + `import` | ~120 | probe: `new Compartment({modules:{'a':'export default 1'}}).import('a')` returns `{default:1}` Promise; ambient module throws |
| CP-EXT 5 | cap-handle integration per Doc 736 | ~30 | probe: cap-handle injected via globals invokable; ambient authority denied without cap |

**Total ~330 LOC across 5 rounds.**

### I.2 Combined-landing strategy

Per Doc 740 multi-tier closure default + Finding T262C.5: CP-EXT 1+2+3 land as one combined sub-locale (~180 LOC; single probe answers all three; tight coupling between ctor, endowments, and globalThis). CP-EXT 4 lands as its own sub-locale (~120 LOC; module-loader surface is distinct). CP-EXT 5 is the Doc 736 validation round (~30 LOC; wires existing cap pattern).

### I.3 Constraints

```
C1. Existing test262 baseline preserves (no PASS->FAIL regressions per
    Doc 740 default).
C2. Each sub-round exemplar-verifies before any further round.
C3. Per Finding T262C.6 + EPSUA.6: per-round pre-scoping probe in
    /tmp/cp_probe_*.js verifies the substrate doesn't surface a deeper
    requirement than projected.
C4. Capability-handle integration (Round 5) defers to rusty-js-caps's
    existing handle shape; this locale does NOT redesign cap handles.
C5. Full TC39 proposal surface (hooks: importHook/loadHook/resolveHook,
    Module Source records, browser virtualization) is OUT OF SCOPE.
C6. Spec-churn risk per prospective doc: freeze against Stage 1 snapshot
    2025-12-01; track later via a separate sub-locale.
```

### I.4 Falsifiers

**Pred-cp.1**: each sub-round closes in 1 implementation round (per Finding ACDPD.1 / RPTP.1 / ASCD.1 100%-per-sub-spec-section pattern).
**Pred-cp.2**: cumulative LOC ≤350 (allows 6% drift on the ~330 projection).
**Pred-cp.3**: probe verifies Doc 736 capability-passing-runtime claim at JS-API level — an endowment-less compartment evaluating a fs-using source throws CapabilityError.
**Pred-cp.4**: zero PASS→FAIL on test262 regression sample per sub-round.
**Pred-cp.5 (DISCIPLINE — standing rule 13)**: prospective application throughout — no substrate-introduction prefixes; each sub-round delivers a probe-confirmable property.

## II. Apparatus + Methodology

- Existing: RS-EXT 2's `allocate_realm` / `enter_realm` / `exit_realm` / `clone_intrinsic_proto` / `__cruftless_eval_realm`.
- New: `Compartment` JS-visible ctor (registered via `register_global_fn` in `install_globals` or a dedicated `install_compartment`).
- New: per-sub-round probes at `pilots/compartment-primitive/probes/`.

Methodology:
1. **CP-EXT 0** — workstream founding (this seed + trajectory + scaffolded dirs).
2. **CP-EXT 1+2+3** — combined sub-locale: class + evaluate + globals + globalThis.
3. **CP-EXT 4** — per-compartment modules + import.
4. **CP-EXT 5** — cap-handle endowment validation.
5. Each round: probe + Doc 740 multi-tier R + commit + exemplar regression check.

## III. Carve-outs

- TC39 Compartments hooks (importHook/loadHook/resolveHook): deferred.
- Module Source records (precompiled binary modules): deferred.
- Browser-specific compartment behavior (DOM, etc.): N/A (cruft is not a browser).
- Cross-compartment value transfer policy beyond passing-through-globals: deferred.
- Compartment async iteration / async generators: deferred.

## IV. Standing artefacts

- `pilots/compartment-primitive/seed.md`, `trajectory.md`
- `pilots/compartment-primitive/probes/` — per-round JS probes
- Substrate edits at `pilots/rusty-js-runtime/derived/src/` (intrinsics.rs primarily; possibly interp.rs for module-load integration)

## V. Resume protocol

Read seed + trajectory tail. The locale has 3 sub-locale-shaped rounds (1+2+3, 4, 5). Each round's seed is in this trajectory; per-round probes at `probes/`. Verify RS-EXT 2 still operational (`__cruftless_eval_realm` available; `prototype_pollution_realm.mjs` prints ATTACK_BLOCKED) before beginning any CP round — RS-EXT 2 is the load-bearing substrate.
