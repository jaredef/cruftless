# The Package Manager as the Resolver-Instance Below Module Load

## Lockfile-as-Artifact, Registry-as-Bilateral-Source, and the Sixth Layer of the Cruftless Stack

*A primary articulation responding to the keeper's observation (2026-05-21 00:15-local): with the baseline JIT operational, the next substrate domain Cruftless must absorb is package management. Builds on [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 730 — The Vertical Recurrence of the Lowering Compiler](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers), [Doc 731 — The JIT as a Lowering Compiler Tier](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity), [Doc 432 — SERVER](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration), [Doc 426 — PRESTO](/resolve/doc/426-presto-an-architectural-style-for-representation-construction), [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion), [Doc 581 — Pin-Art and the Discipline of Near-Necessity Substrate Construction](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), [Doc 725 — The Cluster-to-Walk Mode Transition](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction), and [Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations).*

**Jared Foy · 2026-05-21 · Doc 732**

---

## I. The occasion

Doc 729 identified five resolver-instances stacked vertically in the rusty-bun engagement: Cargo build, bootstrap, module load, execution, and job-queue drain. The enumeration was bounded by what the engagement had touched. The module-load instance (#3) was named at the level of "given an on-disk node_modules tree, resolve a specifier to a module record." The question of how the on-disk tree comes to exist was left unspecified.

Bun closes that question with `bun install`: a binary that walks `package.json`, resolves dependency specifiers against a remote registry, fetches and extracts tarballs, computes a lockfile, and lays out `node_modules`. Cruftless, to be a comprehensive Cruftless realization rather than only the runtime half of one, must close the same question. This document names the resolver-instance that does so, locates it in the Doc 729 stack, derives the four bootstrap properties at this level, and identifies the first-cut scope.

The package manager is structurally a sixth resolver-instance, sitting below module load in the stack. Its source is bilateral in a way the other five instances are not: half lives on disk (`package.json`, lockfile), and half lives in a remote service (the npm registry's metadata + tarball endpoints). The bilateral structure is what makes the instance worth articulating as its own document rather than as an extension to Doc 729.

## II. The instance below module load

Doc 729 §IV's enumeration runs from substrate-deepest (Cargo build) to runtime-shallowest (job-queue drain). The package manager sits between Cargo build (#1) and module load (#3) in semantic level, but in execution order it runs before module load and after Cargo build, producing the on-disk artifact that module load consumes.

The instance is **#0 in the Doc 729 stack, sitting below the module loader**:

**(0) The package install.** Source: `package.json` declarations + lockfile (if present) + remote registry metadata + remote tarball contents. Directives: dependency specifiers, version ranges, conditional exports, peer-dependency declarations, install-time scripts, workspace declarations. Resolver: the specifier-to-version resolver + the registry fetcher + the tarball extractor + the on-disk linker. Artifact: a populated `node_modules` tree + a normalized lockfile recording the exact resolution decisions.

The numbering choice matters. Calling it #0 (rather than re-numbering the existing five to make room) preserves Doc 729's existing references. The instance is below module load in the sense of upstream-in-pipeline-order, not in the sense of more-substrate-deep than Cargo. Cargo builds the binary that the package install runs inside; the package install produces the source the module loader walks.

A six-instance picture, in execution order:

1. Cargo build → `cruftless` binary
2. **Package install → `node_modules` tree + lockfile**
3. Bootstrap → populated Runtime graph
4. Module load → ModuleRecord with Namespace
5. Execution → resolved JavaScript values
6. Job-queue drain → quiescent runtime

The numbering reads execution order, not Doc 729's substrate-depth ordering. Both orderings are valid; both should appear in the trajectory log so cuts can be located unambiguously.

## III. The bilateral source

The four bootstrap properties of [Doc 432 §2](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#the-bootstrap-as-resolver) apply at every resolver-instance per Doc 729 §V. At the package-install instance, the source's bilaterality changes how *totality of consumption* and *medium preservation* are realized.

The on-disk half of the source is conventional: `package.json` is a finite text file with a bounded set of fields; the lockfile (if present) is a finite text or binary file with a bounded schema. The directives in this half are enumerable and consumable in the same way the bootstrap's install-call sequence is enumerable.

The remote half is not finite in the same way. The npm registry's `/{pkg}` endpoint returns a JSON document whose `versions` field can have thousands of entries; the `/{pkg}/-/{pkg}-{ver}.tgz` endpoints return arbitrary tarball contents. The directives in the remote half are: the version manifest (registry chose to publish these versions); the per-version metadata (registry chose to record these `dependencies`, `peerDependencies`, `optionalDependencies`, `exports`, `bin`, `scripts`); the tarball contents (registry chose to host these bytes).

Bilateral source means the resolver's totality-of-consumption obligation is satisfied against *whichever subset of the remote source the local source's directives reach into*. The lockfile is precisely the document that makes the subset finite and recordable. A lockfile-less first install computes the closure of the local directives over the remote source and records the closure as the lockfile. A lockfile-present subsequent install verifies that the recorded closure still resolves and uses the recorded versions verbatim. The lockfile is the artifact that converts the bilateral-source into a unilateral-source for subsequent runs.

This is structurally identical to bytecode at the PRESTO instance (#4). Bytecode is the artifact of the parse + compile stage that the execute stage consumes; the bytecode converts the textual-source-with-grammar into a typed-IR-without-grammar. The lockfile converts the bilateral-source-with-version-ranges into a unilateral-source-with-pinned-versions. PRESTO's two-stage prepare-then-execute pattern recurs inside the package-install instance: stage 1 (resolution) produces the lockfile; stage 2 (linking) consumes the lockfile and produces the `node_modules` tree.

## IV. The four bootstrap properties at this level

**Totality of consumption.** Every directive in `package.json` is consumed by the install. `dependencies` resolved and linked; `devDependencies` resolved and linked when the install is dev-mode; `peerDependencies` checked against the resolved graph; `optionalDependencies` attempted, allowed to fail; `bin` symlinks created in `node_modules/.bin`; `scripts` triggered at the documented lifecycle hooks; `exports` and `imports` recorded into the lockfile so module load (#3) reads them from a single source. The remote source's directives are consumed transitively: each fetched package's `package.json` is itself a source to the same resolver.

**Ordering determinism.** Same `package.json` plus same registry state yields the same lockfile, regardless of when the install runs. This is the property npm originally lacked (install order affected the on-disk shape) and that pnpm and Bun's lockfile preserve. The determinism is over `(package.json, registry-state-at-time-T)`. The lockfile records T's resolution so subsequent installs reproduce it. The §VII induced property includes a strict reproducibility claim that depends on this.

**Medium preservation.** The `node_modules` tree's medium is the filesystem the consuming runtime walks. The install must not leave residue in adjacent namespaces (no global cache state that the consumer can observe through the install API; no environment variables set during install; no half-extracted tarballs from a failed run). Bun's hardlinked global cache is medium-preserving because it lives in a separate path the consumer code does not traverse; cruftless's first-cut equivalent should preserve the same property even when the cache is absent (the absence is medium-preservation in the limit).

**Boundary integrity.** The package-install instance must not leak its directives into the artifact the module loader sees. `dependencies` fields in `package.json` files inside `node_modules/<pkg>/` are visible to module-walk semantics (npm's flat layout means a sub-package's `dependencies` are consulted when its requires can't be satisfied by the local subtree). This is unavoidable per the legacy Node convention, but the install's *resolution decisions* should not leak: the lockfile records which version was chosen for which path; consumer code does not observe the resolution mechanism, only the result.

The four properties are violable in the obvious ways. A first-cut package install that runs `scripts` non-deterministically violates ordering; one that leaves orphaned tarballs in `node_modules/.staging/` violates medium preservation; one that re-resolves dependencies at module-load time (rather than at install time) violates boundary integrity by leaking install-resolver behavior into module-load.

## V. The induced property

Doc 729's runtime-tier induced property is *vertically-recursive directive consumption with stage-deterministic emission*. At the package-install instance, the level-specific induced property is:

**Pinned-closure reproducibility under bilateral-source memoization.** The package install produces an artifact (the lockfile) that memoizes a finite slice of the bilateral source, such that any subsequent install against the same `package.json` and the same lockfile produces a byte-identical `node_modules` tree, regardless of intervening changes to the registry. The artifact is the closure of `package.json`'s direct dependencies under the transitive-dependency relation, projected against the registry state at the install moment T.

The property is testable. Two installs of the same project on different machines, hours or days apart, against a registry whose `latest` tags have moved, against caches in different states: if both consume the same lockfile, both produce a byte-identical (modulo per-OS timestamps) `node_modules` tree. The bilateral source has been collapsed to a unilateral source, and the unilateral source is the lockfile.

Three immediate consequences:

**Reproducible builds, structurally.** The reproducibility is induced by the bootstrap properties at this level, not implemented as a separate feature. Tools that retrofit reproducibility onto a non-Cruftless installer (lockfile-only modes, frozen-lockfile flags, vendoring directories) are working around an installer that violates the property. Cruftless's installer satisfies the property by construction.

**Auditable supply chain.** The lockfile records every fetched URL and every fetched tarball's content hash. A supply-chain audit is the lockfile-walk; a build-cache invalidation is the lockfile-diff. The property is what makes both tractable. The auditability is induced, not added.

**Cross-runtime reproducibility.** A Cruftless lockfile that records the registry's response shape (versions + URLs + hashes), not the installer's internal state, is consumable by any installer that satisfies the bootstrap properties at this level. The Doc 247 across-substrate inversion holds at the package-install level: the lockfile is a constraint specification; npm, pnpm, yarn, Bun, and cruftless are conformant resolver implementations whose only structural difference is the on-disk layout they choose.

## VI. The morph trajectory: what to build first

Per Doc 581's Pin-Art discipline and the Doc 731 §VII first-cut carve-out shape, the first cut is the smallest end-to-end loop that demonstrates the alphabet, defers the hard tail, and produces an artifact the next moves can extend.

**First-cut scope (PM-EXT 1):**

- Input: a `package.json` whose `dependencies` are all exact-version pins (no ranges, no tags, no git, no file, no workspace).
- Registry: `registry.npmjs.org`, anonymous, public packages only.
- Network: tarball fetch + content-hash verification (the registry-recorded `dist.shasum` plus `dist.integrity`).
- Extraction: gzipped-tar unpack into `node_modules/<pkg>/`.
- Layout: flat `node_modules/` (every dep at the top level; first-write wins on collision; the first cut rejects collisions rather than nesting).
- Lockfile: a text-format file recording resolved versions + URLs + hashes. Binary `bun.lockb` parity deferred.
- Lifecycle scripts: not run. The install reports which scripts were declared but skipped.
- Subsequent installs: with lockfile present, verify on-disk tree matches lockfile; fetch only what's missing; no network traffic when complete.

**Carve-outs:**

- **No semver range resolution.** The pinning constraint is what makes the first cut tractable. Range resolution (caret, tilde, complex ranges) is the second cut.
- **No peer-dependency reconciliation.** The first cut warns on unmet peers and continues. Reconciliation is the third cut.
- **No workspace support.** Monorepo features are queued.
- **No lifecycle scripts.** `preinstall`, `install`, `postinstall`, `prepare` are recorded but not executed. Script execution surfaces a different resolver-instance (subprocess invocation) whose Cruftlessness deserves its own design pass.
- **No git / file / link / workspace specifiers.** Only registry specifiers.
- **No private registry, no auth, no scope-aware routing.** Public npm only.
- **No nested layout fallback.** Collisions error out rather than triggering a nested-layout repair pass.

These carve-outs are spec-aligned: each is a region where the complexity-vs-yield ratio is unfavorable for a first-cut baseline, in the same shape Doc 731 §VI established for the JIT first cut.

**Sub-resolvers inside the package-install instance.** Even the first cut decomposes into three sub-resolvers, in Doc 730 §III–§VII lowering-compiler shape:

- **PM-R1 — Specifier resolver.** Source: `package.json` `dependencies` entries (specifier strings, exact-version-pinned in first cut). Resolver: a function from specifier to a `(name, version, url, hash)` tuple. Artifact: the lockfile's `dependencies` section.
- **PM-R2 — Fetcher / extractor.** Source: the lockfile's `(url, hash)` per dep. Resolver: HTTP GET + hash verify + tar.gz extract. Artifact: each dep present at `node_modules/<pkg>/` with verified content.
- **PM-R3 — Linker.** Source: the extracted on-disk staging plus the resolved-graph. Resolver: a function that places each dep at its final on-disk path and creates `bin` symlinks per `package.json` `bin` field. Artifact: a `node_modules` tree the module loader (#3) reads as input.

The three sub-resolvers compose under Doc 730 §IV's vertical recurrence: PM-R1's artifact (the lockfile) is PM-R2's source; PM-R2's artifact (extracted tarballs) is PM-R3's source; PM-R3's artifact (linked tree) is the module loader's source. The package-install instance is itself a three-stage pipeline.

## VII. Reference engine and the §XVI bidirectional oracle

Per Doc 730 §XVI, every substrate move at this instance is gated on the four-case categorization against a reference engine. Bun's `bun install` is the natural reference because cruftless is the Bun port and because Bun's lockfile is what consumer projects in the parity basket already carry.

The four-case categorization at the package-install instance:

- **Case 1 — cruftless violates Bun's behavior, Bun is spec-correct.** Cruftless's installer produced a different resolution than Bun for the same input; Bun's resolution matches npm's documented semver behavior. Cruftless absorbs the difference.
- **Case 2 — Bun violates npm-documented behavior, cruftless is correct.** Bun's installer diverges from documented semver in ways downstream packages don't depend on. Cruftless documents the divergence and chooses the spec-correct behavior.
- **Case 3 — both Bun and npm-documented behavior diverge from package expectations.** Ecosystem-compat case: the package was authored against a non-documented installer behavior. Cruftless absorbs the de-facto behavior per Doc 729 §XII Axis-H precedent and records the absorption.
- **Case 4 — implementation freedom.** Neither Bun nor npm-documented behavior is uniquely correct (e.g., the order of `bin` symlink creation, the layout of `.staging/` directories). Cruftless picks the simpler choice and records the freedom.

Lockfile format: deriving Bun's binary `bun.lockb` parity is deferred to a later cut. The first cut emits a text-format lockfile (`cruftless.lock` or similar) that records the resolved closure in a format readable by both humans and the second-pass install. Binary parity is added when consumer-project compatibility requires it.

## VIII. Falsifiers

Per Doc 729 §IX, the claims here are testable.

**Pred-732.1.** The four bootstrap properties admit a finite specification at the package-install instance. Falsifier: a `package.json` field whose semantics cannot be bounded under the four properties (e.g., a directive whose effect is "execute arbitrary code at install time with full FS access"). Lifecycle scripts are the candidate failure; the §VI carve-out defers them precisely because their Cruftlessness is open.

**Pred-732.2.** Pinned-closure reproducibility holds against an adversarial registry. Two installs against the same lockfile, where the registry has revoked or replaced some published versions between installs, both produce a byte-identical `node_modules` tree (or both fail with the same revocation error). Falsifier: an install that silently substitutes a different version when the recorded one is gone. The fix is to fail-loudly; the prediction is that failing-loudly is induced by the bootstrap properties, not implemented separately.

**Pred-732.3.** The cross-installer portability of the lockfile holds. A cruftless-generated lockfile, when consumed by a Bun installer (or by a hypothetical second cruftless implementation in a different substrate), produces the same `node_modules` tree. Falsifier: a divergence that is not attributable to documented installer-layout freedom. This requires Bun to consume the cruftless lockfile format, which it currently does not; the prediction is that a lockfile-format-translation pass is constructively possible because both lockfiles record the same closure.

**Pred-732.4.** The package-install instance's residue tends monotonically toward zero as substrate rounds proceed. Each PM-EXT either closes a residue (less observable cruft) or surfaces a previously-invisible residue (more named cruft inventory, not more actual cruft). Falsifier: a round that introduces new cruft into a previously-clean part of the install. Same enforcement mechanism as Doc 729 Pred-729.4.

**Pred-732.5.** The package-install instance composes with the module-load instance without an additional cross-level coupling. Module load reads what package install wrote; package install does not need to know anything about module load except the on-disk layout module load expects. Falsifier: a case where the module loader needs to consult installer state (e.g., a runtime lookup against the lockfile) to resolve a specifier. The first-cut design forbids this; the prediction is that the forbidding is sustainable across the morph.

## IX. Relation to prior corpus work

- [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs). This document extends Doc 729's five-instance enumeration to six. The package-install instance is the resolver-instance Doc 729 §IV did not enumerate because the engagement had not yet touched it. Bringing it into the enumeration converts the open-ended phrase "the on-disk node_modules tree" in Doc 729 §IV.3 into an explicit upstream artifact of an explicit resolver.

- [Doc 730 — The Vertical Recurrence of the Lowering Compiler](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers). The three sub-resolvers inside the package install (§VI's PM-R1, PM-R2, PM-R3) are an instance of Doc 730's vertical-recurrence claim at the package-management tier. PM-R1's artifact is PM-R2's source; PM-R2's artifact is PM-R3's source; PM-R3's artifact is the module loader's source. The recurrence holds.

- [Doc 731 — The JIT as a Lowering Compiler Tier](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity). Structural template. Doc 731 named a new tier (the JIT) whose complexity is bounded by upstream alphabet purity. This document names a new tier (the package manager) whose reproducibility is bounded by upstream-source memoization (the lockfile). Both documents extend the engagement's substrate vocabulary by one tier; both document the first-cut carve-out shape.

- [Doc 432 — SERVER](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration). The four bootstrap properties of Doc 432 §2 generalize to the package-install instance per Doc 729 §V's general claim. The §IV derivation in this document is the case-specific instantiation.

- [Doc 426 — PRESTO](/resolve/doc/426-presto-an-architectural-style-for-representation-construction). The two-stage prepare-execute pattern inside the package install (resolve-then-link, with lockfile as the prepare-time artifact) is a PRESTO realization at this tier. The lockfile is to the package install what bytecode is to the execution loop: the typed intermediate that converts a richly-namespaced source into a directive-free artifact for the next stage.

- [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion). The cross-installer portability of the lockfile (Pred-732.3) is a specialization of Doc 247's across-substrate inversion to the package-install instance. The lockfile is the constraint specification; the installers are conformant resolver implementations.

- [Doc 581 — Pin-Art and the Discipline of Near-Necessity Substrate Construction](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction). The morph trajectory of §VI proceeds under Pin-Art. Each PM-EXT is a substrate move that absorbs one residue at the package-install instance; the seed.md and trajectory.md for the new pilot follow the standard Doc 581 shape.

- [Doc 725 — The Cluster-to-Walk Mode Transition](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction). Walk-mode operates over packages whose load chain includes the package-install instance as the first stage. A package-install fault surfaces at the resolver-instance address #0; the walk-mode trajectory log gains the sixth instance as a valid first-coordinate value.

- [Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations). Naming the package-install instance as a resolver-instance changes how subsequent substrate work dispatches. Bugs that previously read as "module load doesn't find X" become locatable as either package-install bugs (X is not on disk) or module-load bugs (X is on disk but the loader doesn't find it). The diagnostic vocabulary widens by one address.

## X. Honest scope

This document records a destination for the engagement's package-management work. It does not specify the implementation; the implementation is the engagement's continuation under Pin-Art. Specifically:

*The first-cut scope of §VI is not a complete package manager.* It is the smallest end-to-end loop sufficient to demonstrate the bootstrap properties at this instance. Range resolution, peer reconciliation, workspaces, lifecycle scripts, private registries, and binary lockfile parity are queued for subsequent cuts, in the same way Doc 731's JIT first cut deferred ICs, deopt, and GC integration.

*The reference-oracle choice of §VII is provisional.* Bun is the natural reference because cruftless is the Bun port. If the engagement later targets npm-documented or pnpm-style behavior as the spec-correct case for some divergence, the §XVI four-case categorization absorbs that. The oracle is the gate, not the goal.

*The cross-installer portability prediction (Pred-732.3) is not engineering-test-ready.* It requires either Bun consuming a cruftless lockfile or cruftless emitting a Bun-format lockfile. Both are constructively possible; neither is in the first cut.

*The induced property (§V) is a structural claim, not an engineering test result.* Whether the first-cut implementation actually satisfies it is a substrate-tier question for the PM-EXT 1 round. The property is the target the substrate moves accumulate against; the substrate work is what demonstrates the accumulation.

Per Doc 372's hypostatic boundary, this document sits at the corpus tier. The substrate tier's work (founding `pilots/rusty-js-pm/`, walking the first-cut alphabet, landing PM-EXT 1 through whatever cardinality the morph requires) lives in the engagement's continuation under the Doc 581 trajectory-log discipline.

## XI. Closing

The package manager is the sixth resolver-instance in the Cruftless stack. Its source is bilateral (local declarations plus remote registry); its lockfile is the artifact that converts the bilateral source into a unilateral source for subsequent runs. The four bootstrap properties apply; the induced property is pinned-closure reproducibility under bilateral-source memoization. The morph trajectory begins with a first cut that installs exact-pinned dependencies from the public npm registry without ranges, peers, workspaces, or lifecycle scripts. The §XVI four-case oracle against Bun gates each move. The three sub-resolvers inside the install (specifier, fetch, link) are themselves a vertical-recurrence instance per Doc 730.

Doc 729 named the destination. Doc 730 named the recurrence. Doc 731 added the JIT tier. This document adds the package-manager tier. The Cruftless stack now has six named instances with the bootstrap-property vocabulary applied at each. The engagement's package-management substrate work proceeds with this articulation as the gating reference.

The work continues. The corpus has added one more resolver-instance to the Cruftless enumeration. The next substrate moves are PM-EXT 1's first cut. The reflexive structure of Doc 722 holds: subsequent rounds dispatch with the destination explicit.

---

*Companion documents in addition to those linked in the masthead: [Doc 250 — The SERVER Seed](/resolve/doc/250-the-server-seed); [Doc 372 — The Method of the Corpus as Derivation, Not Collection](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection); [Doc 723 — Diagnostic Tags as Semiotic Signs](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies); [Doc 727 — Basin Stability from Inside](/resolve/doc/727-basin-stability-from-inside-why-a-corpus-cannot-distinguish-self-reinforcement-from-substrate-coherence).*
