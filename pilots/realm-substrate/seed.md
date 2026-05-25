# realm-substrate — Resume Vector / Seed

**Locale tag**: `L.realm-substrate` (top-level)

**Status as of 2026-05-25**: **WORKSTREAM FOUNDED (RS-EXT 0)**. Spawned per keeper directive — but with a sharp constraint that overrides the prospective doc's 8-round scope:

> "Before committing 500 LOC to the structural refactor, build the minimal realm-scoping needed to answer the actual load-bearing question: does realm-scoping the caps model close an adversary probe that the current non-realm caps model fails?"

This locale therefore inverts the prospective doc's Round-1-first approach. **Round 1 here is the EMPIRICAL PROBE — not the structural refactor.** The structural refactor is gated on the probe's outcome.

**Workstream**: identify a specific adversary probe class that defeats the current (non-realm) capability-passing runtime (Doc 736) by exploiting SHARED INTRINSIC IDENTITY (prototype pollution / well-known Symbol override / global-mutation-as-side-channel). Implement the minimum realm-scoping needed to defeat that probe. The diff (probe FAILS → probe SUCCEEDS-as-attack-blocked) is the empirical answer to the load-bearing architectural question.

**Author**: 2026-05-25 session.
**Parent**: none (top-level).
**Composes with**:
- [Doc 736](../../docs/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — the model this probe stress-tests
- [rusty-js-caps pilot](../rusty-js-caps/) — the existing 9 probe set against ambient-authority attacks (fs/net/env/etc.); the new probe class extends the threat model to intrinsic-identity attacks
- [docs/prospective/realm-substrate-architecture.md](../../docs/prospective/realm-substrate-architecture.md) — the FULL substrate decomposition (8 rounds); only relevant if this locale's probe answers YES
- ECMA-262 §9.3 Realm Records

## I. Telos

**The load-bearing question** (keeper's framing): does realm-scoping the caps model close an adversary probe that the current non-realm caps model fails?

This locale answers the question empirically. NOT by building the full realm substrate; by building the minimum substrate that lets one probe run.

### I.1 Adversary probe class (intrinsic-identity attacks)

The existing rusty-js-caps pilot covers AMBIENT-AUTHORITY attacks (dep reads /etc/passwd via fs). Capability handles defeat those: deny the fs capability, dep can't read.

This locale targets a different class: SHARED-INTRINSIC attacks. A malicious dep doesn't need fs — it pollutes a shared intrinsic the application later uses. Examples:

1. **Prototype pollution via Array.prototype.map override**: dep assigns `Array.prototype.map = function(){ return "pwned"; }`. Application later calls `[1,2,3].map(x => x*2)` → gets "pwned" (intended map function never runs).

2. **@@iterator override**: dep assigns `Array.prototype[Symbol.iterator] = function*(){ yield "trojan"; }`. Application's for-of loops over arrays yield trojan values.

3. **Function.prototype.call shim**: dep replaces `Function.prototype.call` with a logger that exfiltrates arguments to a dep-controlled buffer (then sees every cross-module call).

The current cruft caps model does NOT defeat any of these because intrinsics are SHARED across all loaded modules. No capability handle gates "the right to assign to Array.prototype.map".

### I.2 First-cut probe

Single probe: **prototype pollution via Array.prototype.map override** (the simplest of class 1).

`pilots/realm-substrate/probes/prototype_pollution.mjs`:
```js
// Adversary dep
function malicious_dep() {
  Array.prototype.map = function() { return "PWNED"; };
  return "dep loaded";
}

// Application
malicious_dep();
const result = [1, 2, 3].map(x => x * 2);
if (result === "PWNED") {
  console.log("ATTACK_SUCCEEDED");
} else {
  console.log("ATTACK_BLOCKED");
}
```

Run against current cruft: **expect ATTACK_SUCCEEDED** (intrinsic shared; pollution propagates).

Build the minimum realm-scoping: load the dep in a separate "realm view" with its own Array.prototype. Mutating dep-realm's Array.prototype doesn't affect application-realm's Array.

Run again: **expect ATTACK_BLOCKED**.

The diff = the empirical answer.

### I.3 Minimum-realm scope

NOT the full 8-round substrate. Just enough to load ONE dep in an isolated intrinsic namespace. Minimum substrate:

1. **Per-module intrinsic-rebind table**: a HashMap<ModuleId, IntrinsicOverrides> where each module's prototype lookups consult a per-module override before the global intrinsics.
2. **Module-load entry hook**: when loading a module, clone the intrinsic prototypes the dep can mutate (Array.prototype, Object.prototype, Function.prototype) into the module's IntrinsicOverrides.
3. **Property-lookup respect**: when an Array's prototype chain is consulted from within the dep's module scope, walk to dep-realm's Array.prototype clone; from the application's module scope, walk to the original.

This is approximately ONE realm-pair, with minimal slot copying. ~100-200 LOC for the probe-passing baseline. NOT factory pattern, NOT cross-realm instanceof, NOT capability `[[Realm]]` slot — just per-module intrinsic-rebinding for the probe-relevant intrinsics (Array.prototype, Object.prototype, Function.prototype).

### I.4 Falsifiers

**Pred-rs.1 (PRE-FIX BASELINE)**: the prototype-pollution probe under current cruft prints `ATTACK_SUCCEEDED`. (Confirms the threat model is real.)

**Pred-rs.2 (POST-MINIMUM-REALM)**: the probe under the minimum realm-scoping prints `ATTACK_BLOCKED`. (Confirms realm-scoping closes the gap.)

**Pred-rs.3 (RULE 14 MIRROR)**: existing test262-sample baseline (5946 PASS) preserves to within ±10 tests. (Adding restriction = false-positive risk.)

**Pred-rs.4 (METHODOLOGY)**: the minimum-realm substrate is ≤250 LOC. If the probe can only be defeated by paying the 500-LOC structural-refactor tax, the keeper's framing was wrong and the prospective doc's Round 1 is unavoidable.

**Pred-rs.5 (DISCIPLINE — Doc 740)**: zero PASS→FAIL regressions on currently-passing tests across the substrate-touched modules.

## II. Apparatus + Methodology

- `pilots/realm-substrate/probes/prototype_pollution.mjs` — the probe script.
- `pilots/realm-substrate/probes/run.sh` — runs the probe under cruft, reports SUCCEEDED/BLOCKED.
- Substrate edit at `pilots/rusty-js-runtime/derived/src/` per §I.3 minimum scope.
- Test262-sample regression check (exemplar; not full sweep per keeper directive).

Methodology:
1. **RS-EXT 0** — workstream founding (this seed + trajectory + probe script).
2. **RS-EXT 1** — confirm Pred-rs.1: probe under current cruft prints ATTACK_SUCCEEDED.
3. **RS-EXT 2** — design + implement minimum realm-scoping; probe under modified cruft prints ATTACK_BLOCKED; test262 within ±10 of baseline.
4. **RS-EXT 3** — chapter close + report: the answer to the keeper's question (YES/NO/PARTIAL).

If the answer is YES with bounded substrate cost, the prospective doc's Round 1 substrate refactor becomes empirically authorized. If NO, the prospective doc's framing needs revision before any substrate work.

## III. Carve-outs (vs prospective doc's 8 rounds)

- Full RealmRecord struct + 500-LOC refactor: OUT OF SCOPE for this locale; gated on RS-EXT 3 outcome.
- $262.createRealm (~38 test262 fixtures): OUT OF SCOPE; addressed only if RS proves out and prospective Round 3 spawns.
- Cross-realm instanceof / IsArray / brand-checks: OUT OF SCOPE.
- ArraySpeciesCreate realm-correct fallback: OUT OF SCOPE.
- Per-realm module cache (Doc 736 dep-isolation): IN SCOPE only insofar as the probe needs it (the probe DOES need the dep's module to see a different Array.prototype than the application's module — that's the minimum-realm point).

## IV. Standing artefacts

- `pilots/realm-substrate/seed.md`, `trajectory.md`
- `pilots/realm-substrate/probes/prototype_pollution.mjs`
- Substrate edit at runtime (~100-250 LOC if minimum-realm proves bounded)

## V. Resume protocol

Read seed + trajectory tail. The locale's structure is:
1. Probe written + Pred-rs.1 confirmed (RS-EXT 1).
2. Minimum-realm designed + implemented + Pred-rs.2 confirmed (RS-EXT 2).
3. Empirical-answer report (RS-EXT 3).

The full prospective-doc 8-round substrate is GATED ON THIS LOCALE'S OUTCOME. Don't begin Round 1 of the prospective doc until this locale closes with a YES answer + bounded LOC.
