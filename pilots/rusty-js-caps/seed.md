# rusty-js-caps — Resume Vector / Seed

**Workstream**: the capability-passing runtime substrate per Doc 736 §III Move 1, with subsidiary verifiers per Moves 2-5. The dominant pilot for the "supply-chain-attack architecturally impossible" design.
**Author**: 2026-05-21 session, founded immediately after PM-EXT 13 close + Doc 736 publication.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Composes with**:
- [Doc 736](../../../corpus-master/corpus/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — the primary articulation of the design this pilot lands.
- [Doc 729](../../../corpus-master/corpus/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — Cruftless five-instance enumeration; this pilot transforms instances #2-#4 (module load, execution, runtime) into capability-passing form.
- [Doc 730](../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) §XII-§XVI — deviation-resolution pipeline. The reference engine is Node + npm (the ambient-authority baseline); divergence here is structural, not buggy.
- [Doc 732](../../../corpus-master/corpus/732-the-package-manager-as-the-resolver-instance-below-module-load-lockfile-as-artifact-registry-as-bilateral-source-and-the-sixth-layer-of-the-cruftless-stack.md) — PM workstream; Moves 2-5 extend its lockfile schema and verification.
- [Doc 581](../../../corpus-master/corpus/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction.md) — Pin-Art discipline; this pilot follows the canonical seed.md/trajectory.md/probes/ shape.
- [Doc 733](../../../corpus-master/corpus/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale.md) — fractal seeds-and-trajectories; this pilot pair extends engagement coverage one level deeper.
- `pilots/rusty-js-runtime/derived/src/` — the dominant edit surface. The substrate moves rewrite global-object setup, module load, and the builtin-namespace resolver to enforce capability-passing.
- `host-v2/src/lib.rs` (`install_bun_host`) — the host's capability-handoff site. The host hands the top-level application its capability set; that handoff becomes the load-bearing seam.
- `pilots/rusty-js-pm/derived/src/` — Moves 2-5 extend `ResolvedDep` and the lockfile codec with capability declarations + publisher pinning.

## I. Telos

Land the capability-passing runtime substrate so that a loaded module in Cruftless's runtime can perform exactly the operations its caller authorized through explicit capability-handle passing, and no others. With that substrate in place, layer the four verifier moves on top:

- **Move 2**: manifest-declared and lockfile-frozen capability surface
- **Move 3**: load-time integrity re-verification
- **Move 4**: closed import graphs at compile time
- **Move 5**: publisher pinning

The success criterion is the impossibility claim of Doc 736 §IV mechanically realized: a synthetic malicious dependency, installed via cruftless's PM and required by an application, cannot read any file, make any network call, spawn any subprocess, access process state, persist, or escalate, **unless** the application's top-level code explicitly passed it the corresponding capability handle. The denial-of-service surface (compute-bound abuse) is acknowledged but out of scope.

### I.1 Bounded first-cut telos

Move 1 (the capability-passing runtime) is the dominant work and the gating substrate. Its first cut:

- A loaded module has no ambient access to filesystem, network, process, environment, clock-as-side-channel, or eval.
- A capability is an object with methods. The host constructs the root set; the top-level application receives them through `install_bun_host`; deps receive only what the application explicitly passes via `require` arguments or module-scope hoisting that the parser refuses to elide.
- The runtime's existing surface (rusty-js-runtime intrinsics + node_stubs) is bisected into "pure" (computation only) and "effectful" (capabilities). Pure surface stays globally available. Effectful surface moves behind capability gates.
- `require('node:fs')` resolves to a thin facade that produces no useful operations without a capability handle. The handle, when not provided, makes every facade method throw `CapabilityError`.

Move 1's first-cut probe: a sentinel JS file requires a synthetic dep that does `try { fs.readFileSync('/etc/passwd') } catch (e) { return e.message }`. The probe PASSES if the dep returns `CapabilityError: no fs capability granted`. The probe FAILS if the dep returns the file contents.

### I.2 Verifier moves (subsidiary)

- **Move 2 first cut**: extend `ResolvedDep` with `caps: Caps` field where `Caps = { net: Vec<String>, fs: Vec<String>, env: Vec<String>, exec: Vec<String> }`. Manifest parser reads `caps` from package.json (treating absence as empty). Lockfile serializes the rendered set per package. ~150 LOC.
- **Move 3 first cut**: a hook in `evaluate_module` / `cjs_require` that recomputes sha-512 over the source bytes immediately before parsing and compares against the lockfile-recorded SRI. Mismatch raises a load-time error. ~80 LOC.
- **Move 4 first cut**: a parser pass that collects every static `require` / `import` target and a compiler pass that validates each against the lockfile's closure. Mismatch refuses to compile. Dynamic `require(expr)` rejected in dep code (allowed in application code). ~120 LOC.
- **Move 5 first cut**: lockfile field `publisher: Option<String>` populated from registry response. Reinstall verifies match. Per-registry opt-in. ~100 LOC.

## II. Apparatus

The pilot operates on **resolver-instances #2-#4** of the Doc 729 stack (module load, execution, runtime), with subsidiary work at instance #0 (the PM). It does not introduce a new resolver-instance; it transforms the existing instances to expose a capability-passing API rather than an ambient one.

The structural shift: Cruftless's runtime currently composes with Node's ambient-authority model because the runtime grew out of demonstrated-parity work. Doc 736 names ambient authority as the design flaw that makes the supply chain attack class possible. Move 1 corrects the structural error at the runtime layer; Moves 2-5 prevent regression and extend the structural guarantee through the PM.

The reference engine for §XVI gating: there is no in-ecosystem reference for the capability-passing form. Comparison points (Deno, SES, E/CapTP, Java SecurityManager) are documented in Doc 736 §V but are not co-evaluable engines. The §XVI oracle for this pilot is **synthetic adversary probes**: each substrate move is gated by a probe-suite of synthetic malicious deps that attempt the specific attack the move closes. A move that does not close its declared probe is reverted.

Engagement-internal composition:

- `pilots/rusty-js-runtime/derived/src/interp.rs` + `module.rs` — the bytecode interpreter + module loader. The capability-handle threading lives here.
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (if present) — the pure-vs-effectful bisection lives here.
- `host-v2/src/lib.rs` + `node_stubs.rs` — the host capability-handoff and the node:* facade rewrites.
- `pilots/rusty-js-pm/derived/src/{resolver,lockfile,install}.rs` — Move 2 schema + Move 3 verifier hook + Move 5 publisher pinning.
- `pilots/rusty-js-bytecode/derived/src/` — Move 4's compile-time closed-import check.

## III. Methodology

The pilot follows Pin-Art discipline (Doc 581) with the canonical seed/trajectory/probes layout. Substrate moves are tagged `Ω.5.P05.L2.caps-*` (the L2 indicates this is a runtime-tier extension layered above the install-time L1 of the PM).

The five-move decomposition orders the work:

1. **CAPS-EXT 0 (this entry)**: workstream founding. No code. Seed.md + trajectory.md only.

2. **CAPS-EXT 1**: ambient-authority audit. Enumerate every effectful surface currently reachable from a loaded module. Source: walk `rusty-js-runtime/derived/src/intrinsics.rs` + `host-v2/src/{lib,node_stubs,fs,crypto,os,http}.rs`. Output: `pilots/rusty-js-caps/docs/ambient-authority-audit.md`, a per-method table classifying as pure / effectful / mixed. Reading + classifying, no code change. Comparable to PM-EXT 1's manifest field-coverage table.

3. **CAPS-EXT 2**: capability-API design. Output: `pilots/rusty-js-caps/docs/capability-api.md`. The structured surface: `Fs`, `Net`, `Process`, `Env`, `Time`, `Exec` capability types; each with its method set; constructors only at the host boundary; deputation/restriction operators (`.subDir(path)`, `.allowHost(name)`); failure mode (throwing `CapabilityError` rather than returning sentinel).

4. **CAPS-EXT 3**: synthetic-adversary probe harness. Output: `pilots/rusty-js-caps/probes/` with a set of `.mjs` files representing the attack classes Doc 736 §IV catalogs. Each probe is a small dep + an application file that does or does not pass the relevant capability. Pre-Move-1 state: all probes FAIL (the malicious dep succeeds in its attack). Move 1's success is flipping every probe to PASS (attack refused).

5. **CAPS-EXT 4-N (Move 1 substrate work)**: stepwise bisection of the effectful surface. Each EXT round picks one effectful surface (e.g., `fs.readFileSync`, then `fs.writeFileSync`, then `http.request`, then `process.env`, then `child_process.spawn`), refactors its call path to require a capability handle, and lands a probe flip. Roughly two surfaces per EXT round. Estimated 8-12 EXT rounds.

6. **CAPS-EXT M (Move 1 closure)**: the synthetic-adversary probe suite is comprehensive: every probe PASSES. The capability-passing runtime is operational.

7. **CAPS-EXT M+1**: Move 3 first cut (load-time SRI re-verifier). No-regret hardening that does not depend on Move 1's full closure. Could land in parallel; the dependency-ordering choice is captured per round in trajectory.md.

8. **CAPS-EXT M+2 to M+3**: Move 2 first cut (manifest+lockfile caps schema).

9. **CAPS-EXT M+4 to M+5**: Move 4 first cut (closed import graph compile-time check).

10. **CAPS-EXT M+6**: Move 5 first cut (publisher pinning).

11. **CAPS-EXT M+7**: end-to-end impossibility-claim probe. A synthetic malicious lodash-shaped dependency attempts each Doc 736 §IV attack vector and is refused by exactly the move that closes it. The probe is the engagement's standing demonstration of the impossibility claim.

## IV. Carve-outs and bounded scope

Per Doc 736 §VII, the design does not address:

- **Host compromise**. The cruftless binary itself being malicious is out of scope; defense is reproducible builds and binary signing (separate workstream).
- **Application-level malice**. The application holds every capability the host granted. Malicious user code can do whatever the host permitted. This pilot protects users from their *dependencies*, not from their own code.
- **Computational side channels**. Timing and memory-pressure side channels remain. Gas-limited execution and per-call allocation limits are queued as a separate runtime concern (CAPS-EXT N+1 candidate, deferred).
- **Denial-of-service by computation**. A dep that loops forever or allocates without bound is not preventable in a Turing-complete tier. Acknowledged; not closed by this pilot.
- **Capability leakage by the application**. An application that hands every dep every capability degrades the model to ambient authority by user error. A `cruftless audit` subcommand is queued under Move 2 ergonomics; the architecture does not prevent self-undermining.

Within Move 1's first cut:

- **Coarse capability set first**. The capability types are coarse (`Fs`, `Net`, etc.) before refinement to fine-grained (per-path, per-host) subforms. Refinement is a follow-on substrate move.
- **Sync APIs first**. The synchronous capability-passing path is exercised before the async path. Async deputation has its own subtleties (capability passing across `await` boundaries) that warrant a separate substrate move.
- **No ESM-specific work beyond what CJS forces**. The ESM module path inherits Move 1's structure naturally because static imports are already explicit at the parser level; closed-import-graph (Move 4) extends the inheritance.
- **Builtins facades only**. Cruftless's existing `node:*` builtin facades are rewritten to be capability-gated. Third-party npm packages that re-export `require('fs')` simply receive the rewritten facade and inherit the gating; no per-package work.

## V. Standing artefacts

- `pilots/rusty-js-caps/seed.md` — this file.
- `pilots/rusty-js-caps/trajectory.md` — chronological substrate-move log.
- `pilots/rusty-js-caps/docs/ambient-authority-audit.md` — produced at CAPS-EXT 1.
- `pilots/rusty-js-caps/docs/capability-api.md` — produced at CAPS-EXT 2.
- `pilots/rusty-js-caps/probes/` — synthetic-adversary harness, produced at CAPS-EXT 3 and grown each subsequent round.
- `pilots/rusty-js-caps/derived/` — **not used in first cut**. The substrate moves edit existing crates (rusty-js-runtime, host-v2, rusty-js-pm) directly. The pilot directory holds discipline artifacts; the code lives in the crates the discipline transforms.

## VI. Conjectures (Pred-736.1 through Pred-736.5)

Per Doc 736 §IV, the pilot will corroborate or falsify the following:

- **Pred-736.1**: A capability-passing runtime can be retrofit onto Cruftless without a full runtime rewrite. Falsifier: the retrofit at CAPS-EXT 4+ encounters a load-bearing surface whose effectful-vs-pure bisection requires rewriting more than 30% of rusty-js-runtime/derived/src/. The 30% threshold is the engagement's tolerance for "retrofit" vs "rewrite."

- **Pred-736.2**: The synthetic-adversary probe suite is the right §XVI oracle for capability-passing work. Falsifier: a probe-class is identified that the harness cannot represent (i.e., requires runtime introspection the harness lacks). Such a class would force a methodology pivot.

- **Pred-736.3**: Doc 736 §VI's LOC estimates (2-3k for Pilot α, ~150-120 for each verifier move) are within 2× of reality. Falsifier: any single move exceeds 2× its estimate. The engagement records the actual count per EXT round.

- **Pred-736.4**: Move 1 alone delivers the bulk of the impossibility claim. Moves 2-5 are auditability + resilience harden-ups, not gating. Falsifier: a Doc 736 §IV attack vector remains reachable after Move 1's first-cut closure (CAPS-EXT M).

- **Pred-736.5**: The capability-passing transform is compositional with PM-EXT 11's runtime-smoke gate. The existing `require('lodash').identity(42) === 42` test continues to pass under the new runtime once the application explicitly passes lodash zero capabilities (since identity is pure). Falsifier: the smoke test fails for any reason other than a deliberately introduced capability requirement.

## VII. Resume protocol

Read this seed, then `trajectory.md`'s most recent EXT entry. The next substrate move is whatever the trajectory's open-scope list at the most recent EXT close identifies.

At CAPS-EXT 0 (founding), no code is committed. The next move is CAPS-EXT 1: produce the ambient-authority audit. Reading + classifying; no Cargo work needed.

Pin-Art tag prefix: `Ω.5.P05.L2.caps-*` (the L2 marks runtime-tier work distinct from the PM's L1 install-time work).

---

*The capability-passing runtime is the dominant Doc 736 pilot. Subsequent rounds add substrate moves that close synthetic-adversary probes one effectful surface at a time, until the impossibility claim of Doc 736 §IV is mechanically realized.*
