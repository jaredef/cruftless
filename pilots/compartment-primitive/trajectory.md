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
