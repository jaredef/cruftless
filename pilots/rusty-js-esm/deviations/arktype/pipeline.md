# Deviation-Resolution Pipeline: arktype

Per **Doc 730 §XII–§XVII**: a structured pipeline for resolving an engine-vs-engine deviation without reading the package's source as if it were one's own. The package is opaque on purpose; the pipeline produces the substrate gap as its output.

**Subject**: arktype@latest, load-time crash inside `@ark/schema`. cruftless throws `TypeError: callee is not callable: undefined (method='rawIn')` 7 levels deep into the import; bun completes the import and exposes 21 namespace keys.

**Composes with**: parent locale [rusty-js-esm](../../seed.md) §I.2 anti-telos (the pipeline IS the discipline that catches a Round-3-shaped mistake) and `legacy/host-rquickjs/tools/parity-probe.mjs` (the diff oracle).

---

## Stages (Doc 730 §XII–§XVII)

### §XII — Capture (deviation under observation)

A single command, run on both engines, that produces a structured deviation report. Inputs: package specifier + entry shape (here, `import * as M from 'arktype'; Object.keys(M).length`). Outputs: per-engine `{status, error?, keyCount?, shape?}` plus a textual diff.

Artifact: `captures/L0-import.json` — the L0 (zeroth-reduction) capture. Already present from the parity sweep.

### §XIII — Reduce (minimize the surface area)

Successive reductions of the input that preserve the deviation. Each reduction is a new L-level. The reduction strategy is **bisection of the package's import chain**, not bisection of cruftless's substrate. We progressively cut arktype's transitive imports down to the smallest subset that still produces the crash.

Strategy for arktype:
- L1: cut user-facing API (`import { type } from 'arktype'`) — does crash repro at API entry?
- L2: drop arktype and import @ark/schema directly — does crash repro at schema entry?
- L3: cut to just the file containing the crash chain root (Scope.export) — does crash repro?
- L4: instrument the crash-chain functions with deviation probes (see §XIV).

Each Lk produces an artifact under `captures/`.

### §XIV — Localize (substrate-side identification)

Instrumentation at the crash chain. For each level in the captured trace (`Scope.export → maybeResolve → def → ctx → $ → inner → CRASH`), insert a logging shim that records the receiver shape, callee shape, and result. The shim is engine-agnostic; the same code runs under both engines. The shim's output is a per-engine trace. Diffing the two traces produces the **first divergent point** — the substrate primitive that behaves differently.

Artifact: `traces/Lk-{bun,cruftless}.jsonl` — engine traces. `traces/Lk-diff.md` — first divergent point.

### §XV — Bracket (cordon the substrate candidate)

Once the first divergent point is identified, write a minimal probe that exercises only that primitive (e.g., "class getter inheritance through subclass + super.method() chain"). The probe is engine-agnostic and produces a 1-line PASS/FAIL on each engine. Bracketing prevents §XVI moves from over-applying.

Artifact: `probes/bracket-PRIMITIVE.mjs` per candidate primitive.

### §XVI — Yield (substrate move + probe flip)

Apply the cordoned substrate change in cruftless. Verify the bracket probe flips. Run the full L0 capture and verify the deviation closes. Run the 119-package parity sweep and verify no regressions. Commit.

### §XVII — Iterate (until convergence)

If §XVI closes L0, the deviation is resolved. If §XVI flips bracket but L0 still crashes (further down the chain), re-enter §XIV at the next level.

---

## Pipeline invariants

- **No package-internals reading beyond what the trace forces.** The trace localizes; do not read source to confirm. Source-reading is the failure mode the pipeline exists to prevent.
- **Every level produces an artifact.** Reduction without artifact = lost work. The pipeline's value is the artifact trail, not just the eventual fix.
- **Bracket is mandatory.** A move that lands without a bracket probe is a Round-3-shaped risk. The bracket primitive must be exercisable without arktype installed.
- **Sweep is mandatory before commit.** Per parent locale §I.2.

---

## Status

Stage §XII captured. §XIII–§XIV in progress (see [`trajectory.md`](./trajectory.md)).
