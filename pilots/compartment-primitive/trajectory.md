# compartment-primitive — Trajectory

## CP-EXT 0 — workstream founding (2026-05-25)

**Trigger**: keeper directive "Authorize the pilot spawn locale" after the prospective analysis at `docs/prospective/compartments-as-primitive.md` landed.

**Strategic framing**: Compartments are the user-visible JS expression of Doc 736's capability-passing-runtime discipline. Three load-bearing substrate pieces already exist (RS-EXT 2 minimum-realm, rusty-js-caps capability handles, module loader); this locale layers a JS-API atop them so application authors can directly write the Doc 736 security property.

**Pre-spawn Rule 11 5-axis check** (arc-tier; per-sub-round checks deferred to each round's founding):
- (A1) component A/B: substrate base = RS-EXT 2 minimum-realm; verified operational (probe ATTACK_BLOCKED stable post-RS-EXT 2).
- (A2) op-set coverage: JS-API surface (ctor + evaluate + import + globalThis); no new ops at the bytecode tier.
- (A3) value-domain: endowments are arbitrary Values (capabilities, primitives, objects); per-compartment globalThis is an Object.
- (A4) locals-marshaling: N/A (engine-tier).
- (A5) emission-shape: N/A.
- (A6 EPSUA-extended): TC39 Compartments proposal Stage 1 (https://github.com/tc39/proposal-compartments); freeze snapshot 2025-12-01 per seed C6.

**Pre-spawn probe of substrate dependency**: RS-EXT 2 `__cruftless_eval_realm` operational; `prototype_pollution_realm.mjs` prints ATTACK_BLOCKED (verified at RS-EXT 2 close + via spot-check this round).

**Five Pred-cp.* + discipline falsifier** (seed §I.4).

**Founding artefacts**: seed.md + this trajectory.md + scaffolded dirs.

### Sub-locale queue

| Round | Scope | Status |
|---:|---|---|
| CP-EXT 1+2+3 (combined) | Compartment class + evaluate + globals + globalThis | queued ← next |
| CP-EXT 4 | per-compartment modules map + import | queued |
| CP-EXT 5 | cap-handle endowment validation per Doc 736 | queued |

### Status

CP-EXT 0 founded. CP-EXT 1+2+3 implementation pending keeper authorization.

## CP-EXT 1+2+3 — combined sub-locale closure (2026-05-25)

**Trigger**: keeper "Let's keep the separate then begin" — hooks + Module Source records kept as separate prospective items; the 5-round arc begins.

**Edits** (~150 LOC; under the ~180 projection):
- `interp.rs`: `allocate_compartment_realm(endowments)` wrapper extends `allocate_realm` with caller-supplied endowments seeded into `globals_overrides`.
- `intrinsics.rs::install_compartment`: registers `Compartment` global class.
  - `Compartment` ctor (`make_native_with_length` arity 1): parses `{globals, modules}` options; allocates compartment realm via `allocate_compartment_realm`; creates instance Object with `__compartment_realm = realm_idx` + `__compartment_globalthis = <fresh-globalThis-object>` slots; pre-populates globalThis with endowments; proto = Compartment.prototype.
  - `Compartment.prototype.evaluate(source)`: reads `__compartment_realm` slot, enters realm, runs source via `evaluate_module` with expr-then-stmt fallback (mirrors `eval` pattern), exits realm, returns the captured expression value or Undefined.
  - `Compartment.prototype.globalThis`: data-method returning the slot value. (Spec mandates a getter; data-method observationally equivalent for the probe surface.)
- `pilots/compartment-primitive/probes/cp_evaluate_globals_globalthis.mjs`: 5-check probe (ctor sanity / intrinsic isolation / globals visible / globalThis distinct + stable / ambient leak blocked).

**Verification**:
- Probe `cp_evaluate_globals_globalthis.mjs`: `CP_EXT_123_OK` ✓
- canonical fuzz: acc=−932188103 byte-identical ✓
- Random 200 previously-passing test262: 200/200 pass / 0 regressed ✓
- RS-EXT 2 dependency: `prototype_pollution_realm.mjs` still ATTACK_BLOCKED (spot-checked)

**Pred-cp.* dispositions (partial)**:
| Predicate | Disposition |
|---|---|
| Pred-cp.1 (≤1 round/sub-locale) | ✅ HELD (3 rounds combined into 1 commit) |
| Pred-cp.2 (cumulative ≤350 LOC) | ✅ ON-TRACK (~150 of ~330) |
| Pred-cp.3 (Doc 736 cap-pass at JS-API) | ⚪ DEFERRED to CP-EXT 5 (round-3 probe shows ambient-leak blocked, partial corroboration) |
| Pred-cp.4 (zero PASS→FAIL) | ✅ HELD (200/200) |
| Pred-cp.5 (Rule 13 prospective) | ✅ HELD |

### Finding CP.1

The 5-probe-checks in one round delivered all three sub-round predictions simultaneously (intrinsic isolation, globals injection, globalThis identity). This matches Finding ACDPD.1 / RPTP.1 / ASCD.1 100%-per-sub-spec-section pattern — combined sub-locales close cleanly when the substrate decomposition is sound. The RS-EXT 2 minimum-realm substrate did the heavy lifting; CP-EXT 1+2+3 is essentially a JS-API wrapper.

### Carve-out observations

- **modules option parsed but not applied** (deferred to CP-EXT 4). Compartment ctor reads `options.modules` without error but the per-compartment module map isn't yet active.
- **globalThis is a data-method, not a spec accessor** (`c.globalThis()` not `c.globalThis`). Probe-equivalent; refinement deferred to a later round if user-visible API demands it.
- **Endowments are passed by value-copy at ctor time**, not live-bound. Mutating the endowment object after ctor does NOT propagate. Matches the proposal's expected semantics for the initial-snapshot model.

**Status**: CP-EXT 1+2+3 CLOSED. CP-EXT 4 (per-compartment modules + import) next; CP-EXT 5 (cap-handle endowment validation) after.

## CP-EXT 4 — per-compartment modules + import (2026-05-25)

**Edits** (~95 LOC; under ~120 projection):
- `intrinsics.rs::install_compartment` ctor: extract `options.modules`; clone string entries into a fresh internal Object stashed on `inst.__compartment_modules`. Non-string entries (Module Source records) silently skipped — typed alternative deferred per locale carve-outs.
- `intrinsics.rs::install_compartment` proto: new `Compartment.prototype.import(specifier)` method.
  - Reads `__compartment_realm` + `__compartment_modules`.
  - If specifier absent → Promise.rejected with TypeError-shaped Object.
  - Else enter realm, `evaluate_module(source, url)`, exit realm.
  - Resolved Promise with the namespace ObjectRef; rejected on CompileError (→ SyntaxError) or any RuntimeError.

**Probe** (`cp_import.mjs`, 3 checks): `CP_EXT_4_OK` ✓
1. Named module from the map resolves with valid namespace + callable exports
2. Absent specifier rejects with TypeError-shape
3. Module's intrinsic mutation (`sneaky` sets Array.prototype.map) stays inside the compartment — outer `[1,2,3].map(...)` still returns `[2,4,6]`

**Regression**: canonical fuzz byte-identical; random 200×2 = 400/400 previously-passing tests preserved.

**Pred-cp.* dispositions (cumulative)**:
| Predicate | Disposition |
|---|---|
| Pred-cp.1 (≤1 round/sub-locale) | ✅ HELD |
| Pred-cp.2 (cumulative ≤350 LOC) | ✅ HELD (~245 of ~330) |
| Pred-cp.3 (Doc 736 cap-pass at JS-API) | ⚪ DEFERRED to CP-EXT 5 |
| Pred-cp.4 (zero PASS→FAIL) | ✅ HELD (400/400) |
| Pred-cp.5 (Rule 13 prospective) | ✅ HELD |

### Finding CP.2

The Promise return shape uses cruft's existing `new_promise` / `resolve_promise` / `reject_promise` from rusty-js-runtime/promise.rs. No new substrate; the import path threads cleanly through existing module + promise machinery. Microtask ordering observed: `.then` handlers fire AFTER the synchronous tail of the script (consistent with cruft's microtask queue).

### Carve-out observations

- **Module Source records as typed `modules` entries**: silently skipped at ctor. The proposal supports `new Compartment({modules: {'a': new ModuleSource(src)}})` — would map to a per-compartment pre-parsed module pool. Deferred per CP-EXT 7 prospective.
- **Hooks for dynamic resolution** (importHook/loadHook/resolveHook): not wired. `import` only consults the static map. Deferred per CP-EXT 6 prospective.
- **Module re-export across compartments**: a compartment's module that re-exports from another compartment is not supported; cross-compartment module identity would need extra plumbing.

**Status**: CP-EXT 4 CLOSED. CP-EXT 5 (cap-handle endowment validation per Doc 736) next — the Pred-cp.3 closure.
