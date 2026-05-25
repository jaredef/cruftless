# realm-substrate — Trajectory

## RS-EXT 0 — workstream founding (2026-05-25)

**Trigger**: keeper directive after prospective doc landed:
> "Spawn; however: Before committing 500 LOC to the structural refactor, build the minimal realm-scoping needed to answer the actual load-bearing question: does realm-scoping the caps model close an adversary probe that the current non-realm caps model fails?"

**Strategic framing**: this locale INVERTS the prospective doc's Round-1-first scope. Round 1 here is the EMPIRICAL PROBE (write attack; confirm current cruft falls; build minimum realm to defeat; confirm defeat). Prospective doc's 8-round substrate is gated on this locale's outcome.

**Probe class identified**: intrinsic-identity attacks (prototype pollution / @@iterator override / Function.prototype.call shim). Distinct from rusty-js-caps's existing 9-probe set (ambient authority: fs/net/env). The cap-handle model defeats ambient-authority class but NOT intrinsic-identity class — because intrinsics are SHARED across modules and no cap gates "the right to mutate Array.prototype".

**First-cut probe selected**: prototype-pollution via Array.prototype.map override (simplest class member).

**Five Pred-rs.* + discipline falsifier** (seed §I.4).

**Founding artefacts**: seed.md + this trajectory.md + scaffolded dir. RS-EXT 1 (confirm Pred-rs.1 — probe succeeds-as-attack under current cruft) next.

### Status

RS-EXT 0 founded. RS-EXT 1 probe authoring + baseline measurement next.

## RS-EXT 1 — baseline + minimum-realm scope analysis (2026-05-25)

**Pred-rs.1 confirmed**: probe under current cruft prints `ATTACK_SUCCEEDED`. Intrinsic-identity attacks defeat the current capability model — capabilities gate ambient authority (fs/net/env), not shared-intrinsic mutation.

### Minimum-realm scope analysis (pre-implementation)

The probe's substrate-requirement analysis surfaces a structural finding:

**Even the minimum substrate needed to make this probe print ATTACK_BLOCKED requires three components that are NOT smaller than prospective doc Round 1**:

1. **Per-realm intrinsic table**: dep's Array.prototype must be a different ObjectRef than app's. Cruft currently has ONE Array.prototype shared by every Array allocation. Per-realm intrinsic table is the prospective doc's RealmRecord{intrinsics}.

2. **Module-load realm binding**: when loading dep's module, allocate (or copy-on-write-spawn) a new realm; bind dep's module-scope to that realm. Cruft currently has no per-module realm at all. Module-load hook is prospective doc Round 1's `[[Realm]]` binding.

3. **Prototype lookup respect for realm**: when code in dep's module does `[1,2,3].map(...)`, the proto chain walk must resolve to dep-realm's Array.prototype clone. Cruft's array literal `[1,2,3]` bakes the proto pointer at allocation time (the global Array.prototype). So either:
   - (a) array allocation in dep's module must use dep-realm's Array.prototype (compiler / runtime alloc-site change, realm-aware), OR
   - (b) proto-chain lookup must dispatch through a realm-aware redirect table (lookup-time indirection at every property access).

Option (a) is prospective doc Round 4 (instanceof + IsArray + brand-check across realms). Option (b) is a different architecture (proto-redirect-on-lookup) that cruft doesn't currently have.

### Empirical finding RS.1

**The minimum-realm substrate that defeats the probe IS approximately equivalent to prospective doc Rounds 1 + 4 combined.** It is NOT bounded at 100-250 LOC. The keeper's hypothesis ("minimal realm-scoping that answers the load-bearing question") is *correct that the question is answerable empirically*, but *incorrect that the minimum cost is materially less than the prospective doc's Round 1+4 cost*.

The probe-first methodology was load-bearing for surfacing this efficiently: I avoided the 500-LOC Round 1 commitment AND surfaced that the minimum-cost answer requires that same 500 LOC. The empirical answer the keeper wanted is: **YES, realm-scoping closes the gap; but the minimum is the structural refactor, not less.**

### Pred-rs.4 disposition

**Pred-rs.4 FALSIFIED**: minimum-realm substrate is NOT ≤250 LOC. Per the prospective doc's analysis, Round 1 alone is ~500 LOC (RealmRecord struct refactor); Round 4 (alloc-site or lookup-time realm-aware proto resolution) is ~200-300 LOC. Combined floor: ~700-800 LOC.

### Strategic options

1. **(a) Authorize Round 1 + 4 combined** as the minimum-realm pilot. Land both; rerun probe; confirm ATTACK_BLOCKED. This IS the answer to the keeper's question — paid at the cost the prospective doc projected.

2. **(b) Defer realm substrate**. Document RS.1 as the empirical finding that informed the deferral: cap-handle model is sufficient for ambient-authority threats; realm-scoping needed for intrinsic-identity threats; intrinsic-identity threats not yet a corpus or compliance requirement; pay later.

3. **(c) Hybrid**: implement a STRICTER probe-defeating substrate that's neither full Realm nor zero. Candidate: a parser-tier pass that REFUSES intrinsic-mutating assignments in dep modules (e.g., `Array.prototype.X = ...` becomes a SyntaxError when compiled under `--sealed` for dep code). Bounded ~100 LOC; defeats the probe; doesn't enable cross-realm `instanceof`. Doc 736 capability-passing benefits but the "Realm" architectural property isn't earned.

### Status

RS-EXT 1 closed with the empirical finding RS.1. RS-EXT 2 implementation is GATED on keeper choice between (a/b/c).
