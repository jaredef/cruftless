# Cruft

A world-first hand-rolled JavaScript engine and host, written in Rust, targeting plug-and-play execution of the Node.js package ecosystem. The runtime is **Cruft**; the development apparatus that produced it is **Cruftless**.

Cruft is not a port, not a transpilation target, and not a thin wrapper around an existing engine. Every layer — lexer, parser, bytecode compiler, interpreter, JIT, garbage collector, hidden classes, intrinsics, module loader, host bindings — has been hand-derived against ECMA-262, WHATWG, Node, and Bun specifications under the resolver-instance discipline articulated in the RESOLVE corpus.

The engine implements the ECMAScript language; the host implements the Node and Web platform surface that real packages exercise (`node:*` builtins, `fetch`, `Buffer`, `URL`, `structuredClone`, streams, crypto, TLS, fs, http, sockets, …). The two halves ship together as a single binary, `cruft`, capable of installing and running unmodified packages from the npm registry.

---

## Distinguishing elements

### Cruft Compartments

Cruft implements the TC39 Compartments proposal as a primitive of the runtime, not a polyfill atop it. A Compartment is a first-class realm boundary with its own globalThis, its own loader, and its own evaluator, exposed at the JavaScript surface as a constructor whose contract is observable from inside the program. Compartments compose with the engine's capability-passing module-load substrate to produce an execution environment in which supply-chain attacks are not merely difficult but architecturally impossible: a module cannot reach ambient authority it was not explicitly granted, and the grant graph is observable at load time and at run time.

### Implementation pair with CruftScript

Cruft is the runtime half of an implementation pair. CruftScript is the source-language half: a TypeScript-superset surface with extensions specifically designed to operate over Cruft's substrate alphabet. The pair is symmetric in the sense that the language's design constrains the runtime's required substrate (alphabet purity upstream as the bound on engine complexity), and the runtime's substrate exposes the language's expressible primitives. CruftScript compiles to Cruft bytecode through Cruft's own resolver-instance pipeline; the language is a primary consumer of the engine, not an afterthought sitting on top of it.

### TypeScript execution without transpilation overhead

Cruft executes `.ts` and `.tsx` source files directly. There is no `tsc` invocation, no Babel, no esbuild, no SWC, no source-map round-trip. TypeScript-only constructs (type annotations, interfaces, generics, enums, decorators, parameter-property shorthand, `import type`) are erased or lowered inside Cruft's resolver pipeline at parse time, with the runtime contract preserved at the same point where a JavaScript construct's runtime contract is set. The result is that a TypeScript package's first call is the same call cost as a JavaScript package's first call: no transpiler warm-up, no temp directory of erased outputs, no separate build step in the user's workflow.

---

## Status

Cruft passes a curated 7,785-test representative sample of test262 at 89.6% runnable (6,920 of 7,726 runnable tests), and the full test262 suite at 72.6% runnable (34,946 of 48,107 runnable tests across 53,289 paths). Measured through an identical test harness, Cruft reaches roughly 91% of V8's pass count on the production-relevant sample surface and roughly 82% on the full suite — a hand-derived, independent runtime within close reach of the most mature JavaScript engine, on exactly the features production packages exercise. These figures stand on a hand-derived prototype chain across the language and a hand-derived host platform across the Node and Web APIs. The engineering surface and its parity baselines are documented in detail at the RESOLVE corpus reference (link below).

The repository is organized as the Cruftless apparatus: a substrate-pilot directory tree where each substrate-shaped problem lives in a discoverable locale, and an arc-tier coordinate registry that subsumes locales under coherent multi-substrate programs. The apparatus is itself the engine's development environment; it is also the engine's discovery method.

---

## Fielding Constraint Accumulation and the self-applying apparatus

Cruft is constructed under **Fielding Constraint Accumulation** as the governing architectural principle. Each layer of the runtime emits artifacts that satisfy upstream constraints and impose downstream constraints; the discipline is that every constraint is named and the named constraints accumulate into the substrate's invariant set without the engine itself becoming an arbitrary aggregation of features. The architectural shape that emerges is described in the resolver-instance articulation of the corpus.

The Cruftless apparatus that produced Cruft is **self-applying** in the tradition of Lahlouhi's *Validation of the Development Methodologies*. The discipline the apparatus uses to design and verify the runtime is the same discipline the apparatus uses to design and verify itself. Standing rules, predictive heuristics, and per-rung trajectories accumulate at the runtime tier and at the apparatus tier simultaneously; the apparatus's own structure is a substrate-shaped problem the apparatus solves with the same methodology it applies to the runtime's substrate-shaped problems. The validity of the methodology is read from the engine it produces; the engine's coherence is read from the methodology it instantiates.

---

## The RESOLVE corpus as primary motivation

Cruft is the implementation half of a research program whose philosophical half is the **RESOLVE corpus**: a long-running synthesis across philosophy of science, systems engineering, and computer science authored by Jared Foy. The corpus develops a metaphysical position grounded in **Systems Induced Property Emergence** (SIPE): the thesis that properties of complex systems are not merely the sum of their parts but emerge from the discipline that constrains how parts compose. The corpus is structured as a Lakatosian Research Programme; its **metaphysical hard-core** is the SIPE thesis itself plus the resolver-instance pattern, the Pin-Art discipline of localized resume-vector work, and the substrate-shaped pipeline as a discoverable form.

Cruft is the corpus's primary implementation: the engine where the corpus's architectural disciplines are tested under the constraint that the engine must actually run real production packages, not merely satisfy formal predicates. The corpus motivates the implementation; the implementation corroborates or falsifies the corpus's predictions; the two co-evolve.

The corpus is published at [jaredfoy.com/resolve](https://jaredfoy.com/resolve/).

---

## Prior art

Cruft is built in the lineage of, and gratefully draws on, the published designs and implementations of:

- **ECMA-262** (TC39) — the ECMAScript language specification.
- **WHATWG** — the Web platform specifications (URL, Fetch, Streams, Encoding, Structured Clone, ...).
- **Node.js** — the runtime whose package ecosystem is Cruft's compatibility target and whose host API surface Cruft re-implements.
- **Bun** (Jarred Sumner and the Bun team) — the runtime whose Node-compat engineering and ecosystem velocity demonstrated that a clean implementation of the Node platform on a non-V8 engine was tractable, and whose Zig codebase was studied during the early phase of this repository.
- **V8** — the engine whose pioneering hidden-class, inline-cache, and bytecode-interpreter designs (Crankshaft, TurboFan, Ignition, Sparkplug, Maglev) inform Cruft's substrate work, though no V8 code is derived from.
- **JavaScriptCore** — the engine whose interpreter-tier design and OSR architecture inform Cruft's JIT pillar.
- **QuickJS** (Fabrice Bellard) — the compact JS engine whose source was read during Cruft's early bootstrapping and whose Rust binding (rquickjs) served as a reference-ceiling during the engagement.
- **TypeScript** (Microsoft) — the type-system design whose syntactic surface CruftScript extends and whose erasure semantics Cruft implements.
- **TC39 Compartments proposal** — the realm-substrate articulation Cruft Compartments implements.

---

## Thanks

Particular thanks to **Jarred Sumner** and the **Bun team**. Bun was the inspiration for this research project's implementation phase: a hand-rolled non-V8 runtime, willing to re-engineer the Node platform from scratch when the engineering case justified it. Cruft's choice to take the same gamble — and to extend it through a different language (Rust) and a different design discipline (resolver-instance derivation) — would not have been undertaken without Bun's prior demonstration that the underlying engineering target was achievable.

Cruft and Bun differ in architectural intent (Cruft is built under a derivation discipline; Bun is built for engineering velocity), in language (Rust vs Zig), and in surface (Cruft ships CruftScript and Compartments as primitive; Bun ships a fast bundler and a fast test runner). The intellectual debt is real and gladly acknowledged.

---

## License

Dual-licensed at the user's option under either:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

Copyright © 2026 Jared Foy. Contributions submitted for inclusion shall be dual-licensed as above, without any additional terms or conditions.

---

## Author

**Jared Foy** — author of the RESOLVE corpus and the Cruft / Cruftless implementation pair.

- 𝕏: [@jaredef](https://x.com/jaredef)
- Corpus: [jaredfoy.com/resolve](https://jaredfoy.com/resolve)
