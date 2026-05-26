# Capability-Passing Runtime → Architecturally-Impossible Supply-Chain Attack

## Induced property

A supply-chain attack against a cruftless-loaded module is **architecturally impossible** for the probe-covered surface — not merely defended against, not merely detectable, but ruled out by the runtime's authority composition. A malicious dependency that obtains code execution inside the runtime cannot reach the filesystem, network, environment, or any host capability not explicitly granted to it by a chain of authorized hand-offs originating at the embedder.

Anchor: [Doc 736](../corpus-ref/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md). Empirical anchor: `pilots/rusty-js-caps/` first-cut close; 9/9 synthetic-adversary probes refused under `--sealed`.

## The accumulation

Each constraint adds a restriction to the runtime's authority surface and induces a property the prior configuration did not have. Removing any one collapses the impossibility class to a defense-in-depth class.

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) ambient-authority runtime (Node default) | — | nothing impossible; everything is policy/audit |
| 1 | **No ambient authority** — modules receive no global `process.env`, no `fs`, no `net`, no `child_process`; the global scope contains only ECMA-262 intrinsics | a capability requirement at the host boundary | property: "host access requires explicit grant" |
| 2 | **Sealed capability handles** — host capabilities (file, socket, env) are passed as object handles that cannot be forged from their public API; sealing prevents `Object.getPrototypeOf` traversal to find the constructor | unforgeability | property: "a held handle is the only path to its capability; no synthesis" |
| 3 | **Closed import graph** — the module loader resolves all imports at load time against a pinned manifest; runtime `import()` is permitted only when explicitly granted as a capability | no late-bound dependency injection | property: "the dependency graph is the set the embedder authorized at load time" |
| 4 | **Load-time integrity** — every module hash is verified against the manifest before evaluation; mismatch refuses load | bit-identity to authorization | property: "what loads is what was authorized" |
| 5 | **Capability passing (functional explicit-parameter discipline)** — modules receive their granted capabilities as constructor arguments; no global lookup, no ambient retrieval | call-graph-visible authority flow | property: "any code path's authority is statically derivable from its call ancestry" |
| 6 *(optional)* | Capability revocation — granted handles can be invalidated at runtime by the embedder | dynamic narrowing | property: "compromise of a dependency does not require process restart to contain" |

The named composition (1+2+3+4+5) is the **capability-passing runtime**. The induced property is architecturally-impossible supply chain attack for the probe-covered surface: a malicious dependency cannot read the filesystem (no handle was granted), cannot reach the network (no socket capability), cannot exfiltrate environment variables (no `env` capability), cannot inject code (no late import, no eval-without-grant).

Removing constraint 1 (allow ambient authority) collapses everything downstream; the runtime returns to the Node default.
Removing constraint 2 (allow handle forgery) makes constraint 5's functional discipline cosmetic; capabilities can be synthesized.
Removing constraint 3 (allow late `import()`) admits supply-chain-attack-via-late-resolution.
Removing constraint 4 (skip integrity) admits supply-chain-attack-via-substitution.
Removing constraint 5 (allow global lookup) admits supply-chain-attack-via-ambient-retrieval-of-handles-leaked-elsewhere.

The composition is multiplicative: each constraint closes a distinct attack class. The set of closed classes is the induced security property.

## Tag on the DAG

The capability-passing runtime is an **apparatus-tier instance** of FCA — its constraints inhabit the substrate boundary between embedder and runtime, not the per-test262-fixture coordinate space. Per Doc 728's "tag on the DAG" naming, the coordinate is:

```
runtime/host-surface :: E4/host-hook-or-authority-composition ::
  availability/policy-constrained-handle :: cut/capability-policy ::
  property/impossibility-class
```

The closest test262 coordinate is `host-intrinsic/intl402` family (matrix rank #2, 2,008 fails) — but those are availability-class missing-global failures, not authority-composition failures. Test262 does not enumerate authority composition at the runtime tier; the surface is invisible to the conformance projection. This is why capability-passing is articulated at the corpus tier (Doc 736) rather than emerging from the matrix top.

The induced impossibility-class property is not measurable as a percent against test262; it is measurable as 9/9 synthetic-adversary refusal under `--sealed` (the engagement's internal probe surface). The probe is the per-engagement instrument for an apparatus-tier coordinate that has no test262 surface.

## Composes-with

- [`docs/fca-instances/resolver-instance-directive-free-artifact.md`](resolver-instance-directive-free-artifact.md) — capability-passing is the resolver-instance pattern applied at the authority tier.
- Doc 729 — Cruftless (the comprehensive design).
- Doc 736 — primary articulation.
- `pilots/rusty-js-caps/` — empirical anchor.

## Falsification

The impossibility claim holds only for the probe-covered surface. Expansion of the synthetic-adversary probe set to surfaces not yet tested (eval-with-injected-source, side-channels via timing, prototype-pollution-across-realms) may surface attack classes the composition does not yet close; each addition would either fall under one of the 5 constraints (in which case the constraint is being tested) or surface a sixth constraint not yet named. Doc 736 §IV invites this expansion explicitly.
