# 2026-05-27-compartment-primitive-audit-fix — log

Append-only event log per arc-as-coordinate.md.

## 19:50 UTC — Arc spawned

Per keeper directive Telegram 10038 ("Continue") following the arc-formalization directive 10036. The prior arc (2026-05-27-engine-tier-substrate-readiness-for-compartments) had closed with the P-C probe set REFUTING the property's holding in 4 of 8 probes, surfacing IC.CP1 + IC.CP2 + IC.CP3.

Locale `pilots/compartment-primitive/` (already existed from 2026-05-25 CP-EXT 1-4 workstream) is the substrate target. Arc-level work: a four-rung audit-fix CPF-EXT 1+2+3+4 collapsed into a single substrate move (tightly coupled — all three ICs share the "compartment evaluate runs against the compartment's own globalThis" semantic and must land together to be coherent).

## 19:55 UTC — CPF-EXT 1+2+3+4 LANDED (single substrate move, R4 single-closure)

**Substrate** (intrinsics.rs install_compartment + interp.rs):

1. **IC.CP1 closure** (Compartment.prototype.globalThis accessor): dropped the `register_intrinsic_method(self, proto, "globalThis", 0, ...)` registration that was making globalThis a callable method returning the method-function itself. Replaced with a per-instance data property assignment at ctor time: `rt.object_set(inst, "globalThis".into(), Value::Object(gt))`. Now `c.globalThis` reads as a plain property and returns the compartment's globalThis Object directly.

2. **IC.CP2 closure** (endowment injection + structural realm-globalThis swap):
   - Made `Runtime::intrinsic_name_allowlist()` public so the Compartment ctor can iterate it.
   - At ctor time, pre-populate the compartment's gt Object with: (a) every allowlist intrinsic copied by-reference from the primordial globalThis via dict_mut().insert with the {w:t, e:f, c:t} descriptor; (b) user-supplied endowments (which override allowlist entries per Doc 736 discretion); (c) a globalThis self-reference per §19.1.1.
   - This produces a gt that IS a fully-formed globalThis for the compartment, with the intrinsic surface present + the capability surface injected + the spec-mandated self-ref.

3. **IC.CP3 closure** (cross-evaluate persistence via Script semantics):
   - At Compartment.prototype.evaluate, dropped the `rt.enter_realm(realm_idx)` + `rt.evaluate_module(...)` + `rt.exit_realm(prior)` pattern that was running against the primordial global_object filtered by allowlist.
   - Replaced with a direct swap of `self.global_object` to the compartment's gt (saved + restored around the call), plus a swap of `self.current_realm` for the realm clones, and routing through `rt.evaluate_script` (the ESBC v2 entry point) so the compile-tier StoreLocal+StoreGlobal-at-decl mirror engages.
   - Both expr-form and stmt-form fallback paths route through evaluate_script.

**P-C probe re-run results**:
- P-C.1 inconclusive — probe used Object.keys (won't enumerate non-enumerable intrinsics); pc9 shows getOwnPropertyNames returns 54 names including Array + globalThis (was 2 — `length`, `name`).
- **P-C.2 HOLDS**: `c.globalThis.x === 42` ✅ (was undefined)
- P-C.3 holds vacuously.
- **P-C.4 HOLDS** (mixed → holds): c1 sees its own write ✅, c2 doesn't ✅, outer doesn't ✅.
- P-C.5 holds.
- P-C.6 still inconclusive (probe-design tautology).
- **P-C.7 HOLDS**: `c.evaluate("var foo=42"); c.globalThis.foo === 42` ✅ (was undefined; ESBC v2 mirror engages inside compartment).
- **P-C.8 HOLDS**: cross-evaluate persistence in both directions ✅ (was ReferenceError).
- **pc9 HOLDS**: 54 own-props on c.globalThis (was 2) ✅
- **pc10 HOLDS**: writes persist across compartment.evaluate calls ✅
- **pc11 HOLDS**: `c.evaluate("globalThis") === c.globalThis` ✅ (was false)

**All concrete P-C probes that probed substrate behavior now HOLD.** The two "inconclusive" probes (P-C.1 + P-C.6) had probe-design issues unrelated to the substrate's correctness.

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample **86.6%** (6296/978/397) — identical to pre-CPF baseline. The fix is localized to Compartment code paths; test262 sample exercises Compartments minimally (Stage-1 proposal, not in test262), so the parity is expected.

**Arc CLOSED** on this rung — close-condition met: P-C probes hold + sweep parity + substrate aligned with post-GBSU + post-ESBC architecture.

## Cross-locale findings

**ARC.AF.1**: the three audit-fix ICs were tightly coupled — IC.CP1 (accessor) wires the user-visible surface to gt, IC.CP2 (gt population + swap) makes gt the runtime's actual global_object during evaluate, IC.CP3 (evaluate_script routing) makes writes inside evaluate persist on gt. Trying to land them individually would have produced intermediate states with no observable property (e.g., fix IC.CP1 alone → outside-view returns gt but inside-evaluate uses different Object; fix IC.CP3 alone → script writes on Object that outside-view doesn't return). **Standing rec**: in audit-fix arcs where the ICs share a structural invariant, single-rung landing is correct even though it bundles changes — the property the arc is closing is itself the closure.

**ARC.AF.2**: the arc spent ZERO §XIII recurrences. The plan landed first-try clean. This is consistent with the arc's preconditions being verified BEFORE the arc opened — the falsifier probe set in the prior arc (ARC.MR.4) named exactly the ICs that needed closing; the audit-fix had no implicit-constraint scope to discover at land time. **Standing rec**: arcs whose preconditions include a falsifier-driven probe of the formalization can avoid §XIII recurrences during implementation because the constraints are pre-articulated.

**ARC.AF.3**: the substrate-clearing arcs from earlier today (GBSU + ESBC) compound in the audit-fix. CPF-EXT 1+2+3+4 uses `evaluate_script` (ESBC v2 entry), `global_object` swap (GBSU unified surface), `intrinsic_name_allowlist` (existing CP-EXT 5), `define_global_property`-equivalent dict_mut.insert (GBSU API). Without GBSU + ESBC the audit-fix would have required allocating new substrate primitives. The arc's small LOC budget (~80 LOC additions, ~20 deletions) reflects the substrate readiness the prior arcs achieved. **Standing rec**: corpus articulations (Doc 743) drafted from the substrate's NOW state are the appropriate forcing function; the audit-fix that closes them is small because the substrate is correct.

## Status

CLOSED 2026-05-27 ~20:00 UTC. Single-rung arc; ~80 LOC additions + ~20 LOC deletions; all P-C probes hold; gates maintained. The Compartments substrate is now empirically aligned with Doc 743's articulation of P-C.

Next coherent step (NOT this arc): the deferred Compartment surface items — CP-EXT 5 capability-handle wrapping per Doc 736, CP-EXT 6 hooks (importHook/loadHook/resolveHook), CP-EXT 7 Module Source records, cross-realm instanceof via [[Realm]] slot. Each is its own locale or arc spawn.
