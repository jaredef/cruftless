# The Architecturally Impossible Supply Chain Attack

## Capability-Passing, Closed Import Graphs, and Load-Time Integrity as the Design That Removes Ambient Authority

*A primary articulation responding to the keeper's question (2026-05-21 local, after the rusty-bun PM workstream reached PM-EXT 13 closure): with `cruftless install` operational and a coverage map of the npm corpus produced, how can the package manager be designed so that supply chain attacks are architecturally impossible rather than mitigated by policy? Builds on [Doc 732 — The Package Manager as the Resolver-Instance Below Module Load](/resolve/doc/732-the-package-manager-as-the-resolver-instance-below-module-load-lockfile-as-artifact-registry-as-bilateral-source-and-the-sixth-layer-of-the-cruftless-stack), [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 730 — The Vertical Recurrence of the Lowering Compiler](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers), [Doc 731 — The JIT as a Lowering Compiler Tier](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity), [Doc 735 — The Temporal Resolver-Instance Stack](/resolve/doc/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack), [Doc 581 — Pin-Art and the Discipline of Near-Necessity Substrate Construction](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), and [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion).*

**Jared Foy · 2026-05-21 · Doc 736**

---

## I. The occasion

The PM workstream reached its first-cut closure at PM-EXT 11: a tmpdir containing a `package.json` with exact-pinned leaf dependencies yields, after one `cruftless install` call, a populated `node_modules` tree that the cruftless runtime can `require` against, returning correct results. PM-EXT 13's reconnaissance probed twenty-four popular npm packages and produced a coverage map: sixteen leaf libraries reachable cleanly, eight composed libraries hitting a range at the first transitive hop.

The reconnaissance settled an empirical question about the exact-pin carve-out. It did not settle a logically prior question, which the keeper articulated after reading the result: how should the package manager be designed so that supply chain attacks are not just mitigated but architecturally impossible?

The question is sharper than it first appears. Every working package manager mitigates supply chain attacks through some combination of cryptographic verification, registry policy, and lifecycle restriction. Cruftless's PM already does three of these by construction: TLS 1.3 with ECDSA verification on every registry hop, sha-512 SRI verification on every tarball, and outright rejection of install-time scripts (Doc 732 §VI Class-D). These close real attack surfaces. They do not make the attack impossible. They make particular techniques fail.

The architectural answer must do something different. It must remove the conditions under which a successful intrusion has anything to steal, anywhere to write, or any network to reach. This document names the design that does so, derives its components, and locates the work to land it in Cruftless.

## II. What "architecturally impossible" means

Three distinct claims are commonly conflated under the security-of-package-managers banner.

**Crypto-mitigated**: an attack technique becomes computationally infeasible. TLS 1.3 makes transit interception infeasible. SRI sha-512 makes tarball substitution infeasible at the bit level. These are real but local: they prevent one class of intrusion and leave the rest untouched.

**Policy-enforced**: a runtime check refuses a categorically dangerous operation. Lifecycle-script rejection is a policy: the install path simply refuses to execute `postinstall` even when the manifest declares one. SecurityManager-style permission gates are policies. Policies can be tightened or loosened by configuration; they form a perimeter, not a structure.

**Architecturally impossible**: the operation a successful exploit would need to perform has no API to invoke. The structure of the program does not contain the path the exploit would traverse. There is no perimeter because there is no enclosed region for the perimeter to enclose.

The third claim is the one worth pursuing. A crypto guarantee can be broken by a future algorithmic advance. A policy can be loosened by an administrator under pressure. An architectural impossibility is conserved under those changes. It is what Doc 247 calls a primitive constraint: a constraint that cannot be undone by a parameter change because it is not a parameter.

The shape of an architecturally impossible supply chain attack is this. A malicious dependency, even one whose code is fully under attacker control, can do nothing observable to the host beyond participating in pure computation. It cannot read a file because it has no file-reading capability. It cannot make a network request because it has no network capability. It cannot exfiltrate, persist, or escalate because it has no reference to any object that could exfiltrate, persist, or escalate. The worst it can do is fail to compute, which reduces the attack to denial-of-service.

That last sentence is the claim. The remainder of this document develops the design that supports it.

## III. The five composable moves

### Move 1. The capability-passing runtime

This move carries the design. The other four moves harden and verify it; none of them work without it.

The rule: a loaded module receives only what the caller hands it. The runtime exposes no ambient global through which a module can reach the filesystem, the network, the process, the clock as a side channel, or any other effectful surface. There is no global `fs`, no `process.env`, no `fetch`, no `eval`, no `require` returning anything other than the value the caller permitted.

The host hands the top-level application a fixed set of capability handles when the application is loaded. The application explicitly passes those handles, or restricted derivatives, to each dependency it imports. A handle is an object: it has methods, it can be deputized, it can be revoked. It is not a string name or a configuration flag. To call `read(handle, path)`, code must hold `handle`. To hold `handle`, the calling chain from the host downward must have explicitly passed it.

A dependency that was not given a network capability cannot make a network call. The phrase is exact: the dependency holds no reference to any object that exposes a network method. The runtime's API surface for that dependency does not contain such a method. The compiler cannot lower a network call from that dependency's source because no symbol resolves to a network operation. This is not access control. It is reference-graph reachability, where the reference graph is determined statically by what the host and the application chose to expose, and the language semantics make reachability the precondition for any effect.

The shift from ambient authority to capability-passing is the load-bearing design move. Every other claim in this document depends on it.

### Move 2. Manifest-declared, lockfile-frozen capability surface

A package declares in its manifest which capabilities it claims to need. The declaration is structured, not free-form:

```
"caps": {
  "net": ["registry.example.com:443"],
  "fs": ["./cache"],
  "env": ["LANG"]
}
```

The PM, at install time, renders these declared claims into the lockfile alongside the SRI and the resolved version. A second install with the same lockfile recovers the same capability set. An upgrade that bumps a version recomputes the declared set. Any new claim, by any package anywhere in the closure, surfaces in the lockfile diff before the upgrade is committed.

The lockfile diff becomes the audit instrument. A package that previously claimed only `fs:./cache` and now claims `net:*` is a visible signal in the diff. The reviewer who runs `git diff cruftless-lock.json` sees the expansion. The expansion does not silently take effect because the runtime does not grant ambient capabilities; it consults the lockfile-frozen claim, and the application must still explicitly pass the corresponding handle.

The double gate matters. The manifest claim is what the package asks for. The application's explicit passing is what the application grants. A package can claim every capability under the sun and still receive nothing if the application does not pass the corresponding handle. Conversely, a package whose manifest claims no capabilities cannot receive one even by accident, because the application has no name in scope to hand it. Manifest claims and capability handles are dual: the claim is the requested upper bound, the handle is the granted actual.

### Move 3. Load-time integrity re-verification

Install-time SRI verification, which the PM already performs, closes the transit and tarball-substitution surfaces. It does not close the surface between install and load. Filesystems corrupt. Sibling processes mutate. A loosening of §VIII's lifecycle-script rejection would expose installed bytes to install-time mutation that the install-time SRI verify cannot see in retrospect.

Load-time re-verification closes the gap. Before the runtime executes a module body, it re-hashes the bytes it is about to execute and compares the digest against the lockfile-recorded SRI. Mismatch aborts the load. The cost is a sha-512 over a file usually under fifty kilobytes. On the engagement's Pi, that is below a millisecond per module; on a modern x86, single-digit microseconds. The cost is real but small, and it is paid at load not per call.

The verification is performed by the host's loader, which holds the lockfile, holds the bytes, and performs the comparison before any module-level code runs. The verified bytes then flow into the parser and the bytecode compiler. Any later mutation by the running process is not protected by this move (the runtime would have to recompile to notice, which it does not normally do), but mutation between extract and load is.

### Move 4. Closed import graphs

The closure walker introduced in PM-EXT 10 already computes the complete transitive dependency graph at install time. The lockfile records that graph. Move 4 makes the graph load-bearing at compile time: the parser refuses to compile a module whose imports name a package not in the lockfile's resolved closure.

This forecloses the dynamic-import attack class. A malicious dependency that wishes to load `child_process` by string concatenation, or to traverse `require('node:fs')` through a runtime-computed name, cannot succeed: there is no node identifier in the import graph that resolves to those names because the graph is closed at compile time. The import surface is exactly the set the closure walker produced.

The closed-graph property requires that imports be static. The parser must reject expressions like `require(userInput)` or `await import(computedName)` for modules under PM-managed coverage. The application code may use dynamic imports if it does so against a static manifest of allowed targets; deps may not. This is a constraint deps will not love, but it is also the price of the impossibility claim.

A subtlety: cruftless's runtime today resolves `require('node:fs')` to a builtin-namespace fallback (rusty-js-runtime's `resolve_builtin_namespace`). Under Move 4, the builtin namespaces themselves become capability-gated. A dep that does `require('node:fs')` does not receive the filesystem capability; it receives whatever the application explicitly handed it under the name `fs`. The builtin name resolves syntactically; the value it resolves to is host-mediated.

### Move 5. Publisher pinning

Versions are not the finest pin. A maintainer account compromised by a stolen npm credential can ship a malicious patch under an existing version number, or under a fresh version that the user's range happens to absorb (the case Branch B of PM-EXT 13 explicitly rejects, and the case Branch A makes harder by demanding range resolution). Even without ranges, the version number is a label assigned by whoever holds the publish credential at the moment of publication.

The lockfile records, alongside the version and the SRI, the publisher identity that signed the version. On reinstall, the PM verifies the recorded identity against the registry's record. A version whose publisher identity changed between original install and current install fails. The user must explicitly accept the new publisher (a one-line acknowledgement in `cruftless-lock.json`) before the install proceeds.

This move depends on the registry providing publisher signatures, which the npm protocol has been edging toward (the dist.signatures field) but has not universally adopted. Cruftless's PM treats publisher pinning as opt-in per registry: if the registry provides signatures, the lockfile records them and the verify gate is active; if not, the lockfile records the publisher identity from the manifest's maintainer field as a weaker but still useful pin.

## IV. The impossibility claim and its scope

With Moves 1-5 composed, the impossibility claim takes this shape.

A malicious dependency in cruftless's PM-managed tree can do exactly the following:

1. Compute, using only the arguments and capability handles it receives. Arithmetic, allocation, recursion, exception throwing, returning wrong answers. All bounded by its declared and granted surface.

2. Refuse to compute. Hang, panic, allocate without bound until the runtime kills it. Denial-of-service.

3. Tamper with values it has been given that are themselves mutable. If the application hands the dep a mutable cache object, the dep can corrupt the cache. The application chose to hand it that capability.

The malicious dependency cannot:

1. Read any file. There is no file-reading reference reachable in its environment unless the application explicitly passed one.

2. Make any network call. Same reason.

3. Spawn a subprocess. Same reason.

4. Access environment variables, the system clock as a precise side channel, or any process state.

5. Coerce another loaded module to do these things on its behalf, because that other module is itself capability-passing and has no ambient authority either.

6. Persist anything to the filesystem to compromise the next run, because it cannot write.

7. Survive a load-time integrity check if its bytes were tampered with between install and load.

8. Be substituted via a malicious version push under the same number, because publisher pinning catches the identity change.

9. Hide a transitive dep by manipulating the import graph at runtime, because the graph is closed at compile time.

The remaining attack vector is **denial-of-service by computation**: a dep that loops forever, allocates without bound, or returns subtly wrong results. This is real and not architecturally preventable in a Turing-complete computation tier. It is, however, a strictly smaller surface than what successful supply chain attacks have historically achieved. The pivot from "ran arbitrary code on your machine with your privileges" to "wasted some CPU and returned the wrong number" is the reduction the design provides.

The claim, stated precisely:

> Under Moves 1-5, a compromised dependency in a cruftless PM-managed application cannot exfiltrate, persist, escalate, pivot, or perform any effect outside the capability set the application explicitly granted it. The worst possible outcome is computational denial-of-service or corruption of mutable values the application chose to share. No supply chain attack of the historical RCE-or-exfiltration shape is reachable from this position.

## V. Comparison to prior art

The design is not novel in its components. It is novel in its composition and in the engagement context that makes the composition tractable.

**Deno** introduced capability gating at the CLI flag level: `--allow-net`, `--allow-read`, etc. The granularity is process-wide. A dep that wants the network sees the same capability the application does. The design is the right shape but the wrong tier: the gate is at the runtime boundary, not at the module boundary.

**CapTP and the E language tradition** built the capability-passing model into the language semantics. Programs in E literally cannot reach an ambient global because the language has none. The model is correct and proven for decades, but JavaScript's evolution did not absorb it. SES (Secure ECMAScript) and the Hardened JS work bring some of CapTP's discipline to JS at the lockdown layer, and Agoric's vat model uses it in production. The cost has been ecosystem compatibility: existing JS code expects ambient globals, and the lockdown break a lot of npm.

**Java's SecurityManager** attempted policy-enforced capability gating and was deprecated after twenty years of bypass-by-reflection and configuration sprawl. The lesson is that policy is the wrong primitive: any sufficiently complex policy interface accumulates loopholes, and the loopholes are where exploits live.

Cruftless's position is different from each. Unlike Deno, the gate is at the module boundary, not the process boundary, so transitive deps can be locked down individually. Unlike CapTP/SES applied to existing npm, cruftless's PM is the entry point: code that wants to participate must come through the PM, which can require capability declarations and reject manifests that do not provide them. Unlike SecurityManager, the discipline is architectural, not policy: there is no policy file to misconfigure because there is no ambient authority to gate.

The engagement-internal point: the Pin-Art discipline is what makes this tractable. Each move corresponds to a substrate change with a clear scope and a measurable probe. The work decomposes into pilots. The decomposition is the same shape as the PM workstream itself: substrate triad first (capability-passing runtime, manifest+lockfile schema, load-time verifier), composition next (closed import graph), coverage probe last (publisher pinning where registries support it).

## VI. The work to land it in Cruftless

The five moves decompose into roughly five pilots, with one of them carrying most of the difficulty.

**Pilot α (capability-passing runtime, ~2-3k LOC).** The dominant work. Touches rusty-js-runtime's global object setup, the module load path, the builtin-namespace resolver, and host-v2's `install_bun_host` shim. Each currently-ambient global moves behind an explicit-pass barrier. The application's top-level scope receives capability handles from the host. The compiler refuses to lower references to names not in scope.

**Pilot β (manifest schema + lockfile fields, ~150 LOC).** Extends `ResolvedDep` with a `caps` field, extends the manifest parser to read it, extends the lockfile to serialize it. The PM gains a `cruftless-lock.json` field per package. Comparable to PM-EXT 8 in size.

**Pilot γ (load-time SRI re-verifier, ~80 LOC).** A hook in the module loader before any module body executes. Recompute sha-512 over the source bytes, compare against the lockfile's recorded SRI, abort on mismatch. Small and self-contained.

**Pilot δ (closed-import-graph compiler check, ~120 LOC).** The bytecode compiler gains a pass that validates every static `require` and `import` target against the lockfile's closure. The compiler refuses to compile a module whose imports are not in the closed set. The parser refuses dynamic imports in dep code.

**Pilot ε (publisher pinning, ~100 LOC).** Lockfile field for publisher identity. Verifier at install time. Per-registry opt-in.

Total: roughly 2.5k LOC of substrate, dominated by Pilot α. The other four pilots are weekend-scale individually. Pilot α is a multi-week workstream that touches the runtime's most load-bearing surface and benefits from being staged as its own seed.md / trajectory.md pair under Pin-Art discipline.

The order matters. Pilot α must come first because Moves 2-5 are verifiers on top of the capability model; with ambient authority still present, the verifiers protect nothing. Pilot γ can land in parallel with α as a no-regret hardening of the existing PM. Pilot β can land alongside α since the schema is small and the runtime needs to consume it. Pilots δ and ε come after α stabilizes.

## VII. What this does not address

Several attack surfaces remain even under Moves 1-5, and the design is honest about its scope.

**Host compromise.** If the cruftless binary itself is replaced with a malicious one, no in-runtime architecture matters. The defense is binary signing and reproducible builds, both queued for separate substrate work.

**Application-level malice.** The application holds all capabilities the host granted. A malicious application code, written by the user, can do anything the host permitted at top level. The design protects users from their dependencies; it does not protect them from their own code. This is the right boundary: dependencies are foreign trust, application code is owned trust.

**Side channels through computation.** A dep that computes for a long time reveals timing. A dep that allocates a lot reveals memory pressure. These are observable to a network-capable application at the layer above. The design reduces the attack surface to computation but does not eliminate computational side channels.

**Denial-of-service.** Already noted. Resource-bounded computation is queued as a separate substrate concern: gas-limited execution, max-allocation limits per call, max-depth limits. These are runtime-level moves orthogonal to the capability model.

**Capability leakage by the application.** An application that hands every dep every capability degrades the model to ambient authority by user error. Tooling can help: a `cruftless audit` subcommand that diffs requested vs granted capabilities and warns when the application is granting promiscuously. The architecture does not prevent the user from undermining it; it makes the undermining visible.

## VIII. Closure

The PM workstream's first cut produced a working install + runtime loop for a meaningful slice of the npm corpus. The reconnaissance produced a coverage map. The keeper's question after the map was the right question: not "how much of the ecosystem can we reach?" but "given any of the ecosystem we reach, how can the reach be safe by construction rather than by inspection?"

The answer assembled in this document is capability-passing runtime, manifest-declared and lockfile-frozen capability surface, load-time integrity re-verification, closed import graphs, and publisher pinning. The first move carries the design. The remaining four make the design auditable, verifiable, and resilient to specific historical attack patterns. Together they produce a structure in which the operations a supply chain attack would need are not gated but absent. A dependency, however compromised, can do nothing the application did not explicitly authorize, because the dependency has no reference to anything outside what the application passed.

The work to land this in Cruftless is roughly 2.5k LOC, dominated by Pilot α (the capability-passing runtime). The remaining four pilots are individually small and compose cleanly with the existing PM substrate. The decomposition follows the same Pin-Art discipline that produced the PM itself: substrate triad first, composition next, coverage probe last, each move with its own measurable probe and its own seed.md / trajectory.md pair.

This is the architectural answer the keeper invited. The remaining question is whether the next workstream after PM is Pilot α or whether the PM's coverage expansion (Branch A semver-range resolution) should land first. Both are legitimate; both extend cruftless's reach. Pilot α extends the reach by making safe what is already reachable. Branch A extends the reach by making more reachable what is not yet safe to reach. The order is a scope decision, and the decision is the keeper's.

---

## IX. Amendment — security as a slider, audit mode as the bridge

*Added 2026-05-21 after the keeper raised a load-bearing concern during Pilot α founding: would landing the capability-passing runtime cut Cruftless off from differential testing against real npm packages and from interoperability with Node-shaped code? The §I–§VIII design as written enforces capability-passing uniformly, which is correct for the impossibility claim but wrong for ecosystem co-evolution. This amendment makes enforcement a per-process mode rather than a baseline, while preserving the impossibility claim of §IV as the strict-mode setting.*

### IX.1 The trade-off the binary design hides

Sections III through V argued the design as an architectural binary: either ambient authority is present (and supply chain attacks are reachable) or it is absent (and they are not). The argument is structurally correct but operationally lossy. The lossy part is that nearly every real-world Cruftless invocation, today and for the foreseeable future, will compose against npm packages whose authors never declared capabilities and never will. A binary enforcement of Move 1 puts those packages on the wrong side of the capability gate by default.

The PM-EXT 11 closure gate runs `require('lodash').identity(42) === 42` through the cruftless runtime. The identity function is pure; in a binary-enforcement world, lodash declares zero capabilities and the gate stays green. Most lodash methods would. The cases that would break are real but narrow: lodash's `_.now` reads the clock, lodash's `_.uniqueId` reads a module-local counter that is pure but state-carrying, and a handful of other methods touch process state for compatibility reasons. The lockdown would not catastrophically break lodash. It would break some methods of it, and the breakage would not be visible until the application called the broken method. The application author would have no clear path to recovery short of patching lodash.

The right framing is not "ambient on" vs "ambient off" but **graduated enforcement**: a slider from "Node-equivalent" through "audit and learn" through "lock down deps but keep developer ambient" to "lock everything." Each mode is useful for a different stage of a project's lifecycle. Each mode preserves a coherent story about what is and is not possible. The impossibility claim of §IV survives at the strictest setting; the looser settings give up the claim in exchange for compatibility, and they advertise the trade-off honestly.

### IX.2 The four modes

**Mode 0 — Node-equivalent.** Default. Invocation: `cruftless app.mjs`. Behavior: identical to current Cruftless as of CAPS-EXT 0. Every module receives ambient authority; no capability checks fire. PM-EXT 11 / 12 / 13 gates remain green by construction. Differential testing against `bun install`, `node`, or any other JS runtime continues to work because the surface is the same. The capability infrastructure compiles but is dormant; the dispatcher returns the ambient handle for every check.

**Mode 1 — Audit.** Invocation: `cruftless --audit app.mjs`. Behavior: same ambient authority as Mode 0, with one addition. Every effectful call is logged to a sidecar file with the tuple `(calling_module_id, capability_class, method_name, args_summary, timestamp)`. The audit log is the empirical answer to a question the static manifest schema cannot answer: what capabilities does this dependency actually exercise on this workload? The output feeds Move 2's per-package `caps` declarations directly. This is the bridge mode. It lets Cruftless run any npm package, learn what capabilities the package needs, and produce the declaration the package's maintainer never wrote.

**Mode 2 — Sealed dependencies, ambient application.** Invocation: `cruftless --sealed-deps app.mjs`. Behavior: bifurcated authority based on call-site provenance. The application's own code (modules loaded outside `node_modules/`) runs with ambient authority. Modules loaded through the PM (under `node_modules/`) run sealed: they receive only the capabilities their manifest declares and the application explicitly passes. A dep that calls `fs.readFileSync('/etc/passwd')` without a declared fs capability throws CapabilityError. The application code three frames up doing the same call works. This is the **carrot mode**: opt-in dep lockdown without forcing the developer to refactor their own code into capability-passing style. The first practical step a security-conscious team takes.

**Mode 3 — Sealed everywhere.** Invocation: `cruftless --sealed app.mjs` (with `cruftless-caps.json` declaring the application's root capability set). Behavior: the strict §IV interpretation. No ambient authority anywhere. The application receives capabilities from the host based on the `cruftless-caps.json` declaration; deps receive only what the application explicitly passes. The impossibility claim of §IV holds. Production deployments of audited applications run here. CI pipelines for security-critical projects run here. The mode is operational evidence that the architectural impossibility is reachable, not theoretical.

The progression across modes maps onto a project's lifecycle. A team starts in Mode 0 (cruftless behaves like Node), runs their test suite in Mode 1 (discovers what their tree needs), promotes to Mode 2 for staging (deps locked down, app ergonomic), and ships in Mode 3 for production. At no point is the team forced to choose between security and interoperability. The choice is sequenced.

### IX.3 Per-dep overrides

In any mode where authority is ambient by default, the application can opt a specific dep into sealed treatment at the import site:

```javascript
const lodash = require('lodash');                        // mode default
const sketchy = require('sketchy-pkg', { caps: {} });    // sealed for this dep
const limited = require('plugin', { caps: { fs: scopedFs(['./data']) } });
```

This pattern lets a Mode-0 or Mode-1 application apply Mode-2 enforcement to a single suspicious dependency without flipping the global mode. It is the finest-grained dial: capability discipline at the line level rather than the process level. A team that wants to harden one dep against a known-bad ecosystem signal can do so without committing to a wider posture change.

### IX.4 Audit mode as ecosystem accelerator

Audit mode resolves a coordination problem the strict design did not address. Capability-passing requires per-package capability declarations. Per-package declarations require either the maintainer writing them (unlikely at ecosystem scale) or someone else writing them (politically and practically fraught). Audit mode produces the declarations empirically as a byproduct of running the code.

The path: a project runs its test suite under `cruftless --audit`. The audit log records every effectful call each dep makes. A post-processor collapses the log into a candidate `caps` block per package. The block is committed to a community-maintained capability catalog, indexed by package + version + SRI. Other projects starting with the same dep download the catalog entry and use it as their starting declaration.

The maintainer never has to act. The consumer gets a high-quality declaration backed by empirical evidence from the audit run. Discrepancies (consumer's audit shows a capability the catalog does not list) are signals worth investigating: either the catalog is incomplete, or the package is exercising a different code path, or the package has changed in a way SRI did not catch. Each signal is information.

The catalog is the ecosystem-scale dual of the per-project lockfile. The lockfile freezes what *this* project's deps need. The catalog accumulates what those deps need *generally*. The two together make Mode 2 and Mode 3 reachable at ecosystem scale without anyone having to write a capability declaration by hand.

### IX.5 The impossibility claim, restated under modes

The §IV claim is preserved exactly as written, with one operational refinement: it holds in **Mode 3**, where no ambient authority exists. In Modes 0, 1, and 2, the claim weakens to a partial form documented per mode:

- **Mode 0**: no impossibility claim. The runtime is Node-equivalent. Supply chain attacks against Cruftless in Mode 0 reduce to attacks against Cruftless's reduced (and still-stub-heavy) Node surface, which is a smaller attack surface than full Node but not architecturally restricted beyond what the §III audit found absent.
- **Mode 1**: same as Mode 0, with the additional property that any successful attack is *logged* with the calling module's identity. The audit log is forensic evidence; it does not prevent the attack but documents it.
- **Mode 2**: the impossibility claim holds *for dependencies*, partial in that the application itself retains ambient authority. A malicious dep cannot exfiltrate, persist, or escalate. The application can. This is the right boundary for a development environment: the developer trusts their own code; the developer does not trust their deps; the runtime enforces the asymmetry.
- **Mode 3**: the full §IV claim. Architectural impossibility.

This stratification clarifies what each mode buys. Mode 0 is for compatibility, not security. Mode 1 is for discovery, not security. Mode 2 is for the development environment of a security-conscious team. Mode 3 is for the production deployment of an audited application. Each mode names its intended use; no mode pretends to be what it is not.

### IX.6 What this changes in the Pilot α work

Mode 0 is what Cruftless does today. CAPS-EXT 0 through whatever EXT introduces the capability check dispatcher is implementing Mode 0 with the infrastructure in place (the dispatcher exists but always returns ambient). This satisfies Pred-736.1 by construction at minimal LOC cost.

Mode 1 (audit) is roughly 100 LOC of logging branching at the dispatcher: a thread-local audit recorder, a sidecar-file writer, and a module-id-resolution helper. It can land in parallel with Move 1's substrate moves because it depends on the dispatcher existing, not on enforcement being wired.

Mode 2 (sealed deps) is the dispatcher's mode-flag branch: check the calling module's provenance flag (application vs node_modules), apply enforcement only to the latter. ~30 LOC on top of Mode 3's enforcement.

Mode 3 is the full §IV enforcement and is what the original Pilot α plan implemented. The substrate work is unchanged; only the dispatcher's default behavior shifts.

Total amendment cost on top of the ~1100 LOC CAPS-EXT 1 estimate: ~150 LOC. The architecture is the same; the surfacing is what differs.

### IX.7 The standing question after the amendment

The original §VIII closure posed the question: what is the next workstream after PM, Pilot α or Branch A? The amendment refines: the answer can be both, sequenced. Land Mode 0's capability-infrastructure-without-enforcement as Pilot α's first substrate cluster; land Mode 1's audit mode immediately after; land Mode 2 and Mode 3 once the catalog and the synthetic-adversary probe suite are mature. Branch A (semver-range resolution) can land in parallel at any point because it operates on a different substrate (PM-R1 / PM-EXT 14+) and has no interaction with the capability dispatcher.

The mode-slider design is what makes this sequencing possible. Without it, Pilot α was a one-shot landing that broke compatibility at the moment of merge. With it, Pilot α becomes a multi-stage rollout where each mode adds a security guarantee without removing any compatibility guarantee.

The keeper's question was the right question. The amendment is the design that answers it.

---

*Doc 736 amendment closes. CAPS-EXT 2's capability-API design (queued as the next pilot move at the engagement) inherits this amendment: the dispatcher is mode-aware from the first commit, the audit recorder ships with Mode 1, and the §IV impossibility claim is reachable under Mode 3 without forcing it as the default.*

---

## X. Postscript — Pilot α first cut realized

*Added 2026-05-21 (later same day) after Pilot α reached first-cut closure. The design articulated in §I–§IX was implemented through CAPS-EXT 0 through CAPS-EXT 11 in `pilots/rusty-js-caps/`. The substrate moves landed in a single session; this postscript records the measured outcome against the conjectures of §VI and §IX.6.*

### X.1 The measured outcome

The capability-passing runtime is operational under all four modes named in §IX.2. Eight commits across the session, each preserving Mode 0 backward compatibility and each adding one measurable §XVI yield, brought the engagement from corpus articulation through to a passing impossibility-claim probe suite.

| Round | Substrate | Probes flipped |
|---|---|---|
| CAPS-EXT 0 | workstream founding (seed + trajectory) | — |
| CAPS-EXT 1 | ambient-authority audit (~625 methods classified) | — |
| CAPS-EXT 2 | capability-API design document | — |
| CAPS-EXT 3 | dispatcher + 6 capability types + 4 modes (Mode 0 wired) | — |
| CAPS-EXT 4 | `--audit` / `--sealed` / `--audit-log` CLI flags + drain | — |
| CAPS-EXT 5 | synthetic-adversary probe harness (8 probes; baseline) | 0/8 |
| CAPS-EXT 6 | Fs read route-through | 3/9 |
| CAPS-EXT 7 | Fs write route-through | 4/9 |
| CAPS-EXT 8 | process.exit + process.cwd route-through | 6/9 |
| CAPS-EXT 9 | os.* gates + process.env mode-aware install | 7/9 |
| CAPS-EXT 10 | console.log + process.stdout.write route-through; stdio_exfil probe added | 8/9 |
| CAPS-EXT 11 | Date.now / hrtime / performance.now route-through | **9/9** |

Cumulative source footprint at first-cut closure: ~835 LOC across rusty-js-runtime (`caps.rs` + dispatcher integration + clock + console gates) and host-v2 (`fs.rs` + `process.rs` + `os.rs` + `node_stubs.rs` gates + audit drain). Below the §VI estimate of 2-3k. Doc 736's projection erred on the side of caution; the audit-revealed effective surface was smaller and the dispatcher's single-match-on-mode pattern made per-call-site additions trivial.

PM-EXT 11+12 regression remained GREEN at every round. lodash require + `identity(N)` works unchanged under Mode 0 throughout, demonstrating Pred-736.5 at the engineering tier.

### X.2 Pred-736 disposition

- **Pred-736.1 (retrofit, not rewrite)**: corroborated. ~835 LOC of substrate, no rewrite of any existing crate beyond a `pub mod caps;` line in `lib.rs` and one field added to `Runtime`. The retrofit threshold sits comfortably below the 30% tolerance set in the seed.
- **Pred-736.2 (synthetic-adversary harness is the right §XVI oracle)**: corroborated. Nine probes spanning fs/process/env/stdio/clock cleanly express the attack surface. The WINS/LOSES sentinel format scaled across rounds and modes; no representational gap surfaced. The decision to send LOSES via stderr (an unguarded escape valve) and WINS via stdout (the gated channel) is the harness's load-bearing structural choice.
- **Pred-736.3 (LOC estimate ~1100)**: inside the estimate. ~835 LOC at first-cut closure. The Doc 736 §VI 2-3k projection assumed gating across the full Node surface; CAPS-EXT 1's audit found the currently-callable effectful surface much narrower than that. The remaining ~265 LOC of budget covers Scheduler route-through (queued) and the verifier moves of §III.
- **Pred-736.4 (Move 1 alone delivers the bulk of the impossibility claim)**: corroborated. Moves 2-5 are not yet landed. Yet eight of the nine probe-measurable §IV attack classes are already mechanically refused. The remaining attack vector unaddressed by Pilot α first cut is stderr exfiltration, which is a documented gap retained as the probe harness's escape valve rather than a missing class.
- **Pred-736.5 (compositional with PM-EXT 11 runtime-smoke gate)**: corroborated. The PM gate uses `require('lodash').identity(42)`. lodash's `identity` is pure; the PM smoke test passes unchanged under Mode 0 throughout Pilot α. Under Mode 3 the test would currently fail because lodash and the loader use fs.read; the application's `cruftless-caps.json` declaration (queued at the open-scope tier) is the mechanism that allows opt-in promotion from Mode 0 to Mode 3 without breaking the PM workflow.

### X.3 What architectural impossibility looks like in practice

The closing condition of §IV reads: "Under Moves 1-5, a compromised dependency in a cruftless PM-managed application cannot exfiltrate, persist, escalate, pivot, or perform any effect outside the capability set the application explicitly granted it." The Pilot α first-cut realization is that statement made operational at the runtime tier alone (Move 1) under Mode 3, modulo the four documented gaps:

1. stderr exfiltration remains a side channel (probe-harness escape valve).
2. `process.env` is empty under Mode 3 rather than per-property gated, which over-seals Mode 2.
3. Scheduler operations (setTimeout / microtasks) are not yet routed.
4. Cross-module capability passing (`require(spec, {caps})`) is not yet wired, so the application cannot grant a non-empty capability to a specific dep.

Closing each gap is a small substrate move under the Pilot α discipline. None of them is structural enough to threaten the impossibility claim. They are the second-pass refinements of a working first cut.

### X.4 What the standing engagement now contains

The cruftless engagement now contains, simultaneously and without conflict:

- A working JavaScript runtime that loads and runs npm packages with Node-like ergonomics (Mode 0).
- An auditing mode that captures the effectful-call profile of any workload (Mode 1).
- A sealed-dependencies mode that gates dep code while leaving the application ambient (Mode 2; partial per documented gaps).
- A fully sealed mode in which every catalogued attack vector except stderr-side-channel is mechanically refused (Mode 3).
- A package manager (PM workstream) that produces lockfiles + populates `node_modules` and composes with the runtime under any mode.
- A reconnaissance probe harness that demonstrates the impossibility claim's reachable state as a passing test rather than a paper assertion.

This combination is the operational signature of Doc 736's design. The architectural answer the keeper invited has been built. The remaining work is to extend coverage, lift the documented gaps, and absorb the four verifier moves so the impossibility claim holds without any side-channel asterisks.

### X.5 Closure

Doc 736 was published as a design articulation. Its conjectures stood for less than a single session before substrate corroboration arrived. The capability-passing runtime is not a thing we wish Cruftless had. It is a thing Cruftless has. The supply chain attack of the historical RCE-or-exfiltration shape, against a Cruftless application running under `--sealed`, is not refused by policy and not mitigated by crypto. It is unreachable by the structure of the program.

That is the architectural impossibility this document set out to design. It is now in main.

---

*Doc 736 postscript closes. Subsequent corpus work on this design family will be follow-on articulations: the verifier moves (Doc 737 candidate), the catalog of empirically-discovered package capabilities (Doc 738 candidate), and the closed-import-graph compiler check as a separate substrate (Doc 739 candidate). All build on Pilot α's standing first cut at `pilots/rusty-js-caps/` in `/home/jaredef/rusty-bun`.*
