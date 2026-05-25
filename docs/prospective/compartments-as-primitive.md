# Compartments as Primitive — Architectural Analysis

**Date**: 2026-05-25 (post RS-EXT 2 minimum-realm closure).
**Author**: 2026-05-25 session.
**Status**: prospective — strategic analysis of TC39's Compartments proposal vs cruft's existing substrate.
**Composes with**:
- [Doc 736 capability-passing-runtime](../../docs/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — Compartments are the **user-visible JS expression** of Doc 736's discipline
- [docs/prospective/realm-substrate-architecture.md](realm-substrate-architecture.md) — Realm substrate is the SUBSTRATE LAYER; Compartments are the JS-API LAYER atop it
- [RS-EXT 2](../../pilots/realm-substrate/) — minimum-realm landed; intrinsic isolation primitive exists
- [rusty-js-caps pilot](../../pilots/rusty-js-caps/) — capability handles are the endowment-passing mechanism

## I. Question

The TC39 Compartments proposal (https://github.com/tc39/proposal-compartments, currently Stage 1) introduces a JS primitive for creating isolated module-evaluation environments with explicit endowments:

```js
const c = new Compartment({
    globals: { console, performance },   // explicit endowments
    modules: { 'safe-lib': sourceText },  // per-compartment module map
});
const exports = await c.import('safe-lib');
```

Each Compartment has:
- Its own `globalThis` (isolated from the parent compartment)
- Its own intrinsics (cloned per the underlying Realm)
- An explicit `globals` object — the only ambient bindings visible inside
- A `modules` object — the resolved-and-frozen module graph (or a `resolveHook` for dynamic resolution)
- An `import(specifier)` method returning a Promise of the module's exports
- An `evaluate(source)` method for eval-style source execution

The question: **could Compartments be implemented in cruft as a first-class primitive, given cruft's existing substrate?**

## II. Answer

**Yes — and arguably more naturally than any other TC39 proposal**, because three load-bearing pieces already exist:

1. **Minimum-realm substrate** (RS-EXT 2): `__cruftless_eval_realm(src)` already evaluates source in an isolated realm with cloned intrinsics. Compartment.evaluate is a thin wrapper.
2. **Capability-passing-runtime** (Doc 736 / rusty-js-caps pilot): endowments map directly onto cruft's existing capability-handle pattern. The Compartment's `globals` field IS the endowment set.
3. **Module loader** (rusty-js-pm + rusty-js-runtime module load path): cruft already has per-module sealed evaluation; per-compartment module maps are a natural extension.

Compartments would be the **user-visible JS expression** of Doc 736's architectural property. Where Doc 736 is "architecturally impossible supply-chain attack at the host level," Compartments would be "architecturally impossible supply-chain attack at the JS-API level — usable directly by application code."

## III. What's required to ship Compartment

Five components, mapped to existing cruft substrate:

| # | Component | Maps to | Est. LOC |
|---|---|---|---:|
| 1 | `Compartment` JS class with constructor + `evaluate` + `globalThis` accessor | RS-EXT 2 minimum-realm + new JS-visible ctor | ~80 |
| 2 | `globals` endowment injection at compartment creation | extend `allocate_realm` to accept endowments; populate compartment's globalThis with them | ~40 |
| 3 | Per-compartment module loader + `import` method | thin wrapper over `evaluate_module` calling into compartment's realm; per-compartment module map | ~120 |
| 4 | Per-compartment `globalThis` Object exposed to user JS | allocate a fresh globalThis per realm; populate with endowments; substitute for primordial globalThis during compartment-scoped eval | ~60 |
| 5 | `Compartment.prototype.import` as async Promise-returning | existing module-load returns Promise; wrap | ~30 |

**Total: ~330 LOC** on top of RS-EXT 2's existing ~190.

## IV. Where Compartments diverge from minimum-realm

RS-EXT 2's `__cruftless_eval_realm` is RUNTIME-internal (engine-helper namespace); Compartments would be USER-VISIBLE (Compartment global class). The diff:

| Property | RS-EXT 2 `__cruftless_eval_realm` | TC39 Compartment |
|---|---|---|
| User-visible | No (engine helper) | Yes (global class) |
| Endowments | None (cloned intrinsics only) | Explicit `globals` map |
| Module map | None | Per-compartment registry |
| Async import | No | Yes (Promise-returning) |
| Multiple eval calls | One eval per realm | Many evals per compartment |
| Source-type discrimination | Source only | Script / Module / SourceText |

The minimum-realm covers the SUBSTRATE; Compartments need API + endowments + module map + async + multi-eval reuse.

## V. Doc 736 alignment

Compartments are the cleanest possible expression of Doc 736's capability-passing-runtime model in JS syntax:

```js
// Doc 736 in cruftless host-side (current state):
//   - host hands top-level app capability handles via install_bun_host
//   - app passes capabilities through require() arguments
//   - dep receives only what app explicitly passed

// Doc 736 in JS-visible Compartments (proposed):
const dep = new Compartment({
    globals: {
        fs: capability_handle_for_fs_read_only,
        console: capability_handle_for_console,
        // NO ambient process, NO ambient fs, NO ambient require
    },
    modules: { 'safe-lib': depSource },
});
const { default: result } = await dep.import('safe-lib');
// dep can only do what the globals + modules allow.
// Intrinsic-identity attacks (prototype pollution): blocked by realm isolation.
// Ambient-authority attacks: blocked by empty default globals.
// Supply-chain attacks: blocked by per-compartment module map (no transitive
// imports outside the explicit map).
```

The Compartment is **the API that makes Doc 736's claim accessible to application authors**, not just host authors.

## VI. The full Compartment proposal vs cruft's minimum subset

The full TC39 Compartments proposal has additional surface:
- `Compartment.prototype.evaluate` source-type options
- Module Source records (binary precompiled modules)
- Hooks: `importHook`, `loadHook`, `resolveHook`
- Virtualization of `globalThis`
- HostDefined-data threading

Cruft can ship a USEFUL subset without all of this. The minimum Compartment that DELIVERS the Doc 736 alignment:

- Constructor with `{globals, modules}` options
- `evaluate(source)` — sync eval in compartment
- `import(specifier)` — async module load from per-compartment map
- `globalThis` accessor

That's ~330 LOC per §III, on top of RS-EXT 2.

The full proposal's hooks + Module Source + virtualization are deferred — useful but not load-bearing for the Doc 736 claim.

## VII. Decomposition

| Round | Scope | LOC | Gates |
|---:|---|---:|---|
| 1 | Compartment ctor + `evaluate` (single-eval, intrinsics-isolated) | ~80 | minimum probe: compartment intrinsic isolation matches RS-EXT 2 |
| 2 | `globals` endowment injection | ~40 | probe: `new Compartment({globals: {x: 42}}).evaluate('x')` returns 42 |
| 3 | Per-compartment `globalThis` object | ~60 | probe: compartment's globalThis ≠ outer globalThis; identity preserved across compartment.evaluate calls |
| 4 | Per-compartment `modules` map + `import` | ~120 | probe: `new Compartment({modules: {'a': 'export default 1;'}}).import('a')` returns `{default: 1}`; module from outside map throws |
| 5 | Capability-handle integration (endowments as cap handles per Doc 736) | ~30 | probe: cap handle injected via globals can be invoked from compartment; ambient-authority denied without cap |

Cumulative ~330 LOC across 5 rounds.

## VIII. Why this is more natural than full Realms

The TC39 Realms proposal (now superseded/split) faced opposition because:
- Realms expose intrinsic-identity differences (cross-realm `instanceof` returns false), which breaks many ecosystem assumptions
- Realms don't have built-in capability-passing — they're isolated but everything's still ambient within each realm
- Realms add complexity without obvious end-user benefit

Compartments dodge these:
- Compartments OPT-IN to module loading; default globals are empty (no ambient authority)
- Endowments make capability-passing first-class at the language level
- The end-user benefit is concrete: secure plugin loading, sandbox third-party code, embed untrusted JS

For cruft specifically:
- Doc 736's whole thesis is "ambient authority = supply-chain attacks"
- Cruft's existing architecture is already capability-passing at the host tier
- Compartments expose that discipline to JS-tier code, making it usable BY application authors

It's the API that lets the security model BE used, not just an architectural property documented in corpus.

## IX. Risks

1. **Spec churn**: Compartments is Stage 1; the API may change. Cruft would ship against a frozen snapshot, then track. Bounded risk.
2. **Module Source records**: the most controversial sub-proposal; cruft can defer.
3. **Web compatibility**: Compartments-in-the-browser is its own can. Cruft is not a browser; can ignore browser-specific concerns.
4. **Performance**: each compartment allocates intrinsic clones. For hot-load workflows (e.g. plugin systems instantiating many compartments per second), this could matter. Bounded — premature optimization until measured.

## X. Recommendation

**Spawn `pilots/compartment-primitive/` as a top-level locale.** Decompose into the 5 rounds. Each round has its probe + Pred-* + exemplar verification + Doc 740 multi-tier closure default.

The natural sequence:
- Round 1 + 2 + 3 land as one combined sub-locale (~180 LOC): the JS-visible Compartment class with evaluate + globals + globalThis. Single probe answers all three.
- Round 4 lands as its own sub-locale (~120 LOC): per-compartment module map + import. Larger surface, distinct semantic concerns.
- Round 5 is a capability-passing integration sub-locale (~30 LOC): wire endowments to existing rusty-js-caps capability handles. Validates the Doc 736 alignment empirically.

The total ~330 LOC is bounded; landed against the existing RS-EXT 2 + rusty-js-caps + module-loader substrate, each round delivers a probe-confirmable property. The arc closes with a JS-visible Doc 736 demonstration — application authors can write:

```js
const dep = new Compartment({
    globals: {
        readFile: cap_fs_read_only_for_safe_paths,
        // no console, no process, no eval, no Function
    },
});
const result = dep.evaluate('readFile("/etc/passwd")');
// throws CapabilityError - the endowment doesn't include access to /etc/passwd
```

That's Doc 736's architectural-impossibility claim, expressed in JS code an application author would write.

## XI. Status

Prospective; awaits keeper authorization to spawn `pilots/compartment-primitive/` as new top-level locale.

If authorized, RS-EXT 2's minimum-realm work becomes the substrate dependency; the 5-round Compartment arc layers atop it.
