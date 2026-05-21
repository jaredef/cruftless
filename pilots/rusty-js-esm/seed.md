# rusty-js-esm — Resume Vector / Seed

**Status as of 2026-05-21**: **FOUNDED**. Locale scoped after Tier-Ω Round 3 catastrophic over-application (commit `1746bc72`, reverted as `25d4bd95`). The revert restored 94.9% (113/119) baseline; the residual six failures cluster heavily around ESM/CJS namespace synthesis, so the work earns its own Pin-Art locale rather than continuing as ad-hoc rounds inside `rusty-js-runtime`.

**Workstream**: ESM ↔ CJS interop and module-namespace synthesis inside the Cruftless runtime. The substrate question is "which bag of keys does `import * as M from 'pkg'` produce, and how does that bag relate to bun's reading of the same package?"

**Author**: 2026-05-21 session, founded immediately after the Round 3 revert sweep confirmed baseline restoration.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).

**Composes with**:
- [Doc 729](../../../corpus-master/corpus/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — module load is **resolver-instance #2**; this locale is the gap-closure substrate for that instance.
- [Doc 730](../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) §XII–§XVII — bidirectional engine-diff oracle. Bun is the reference engine; cruftless is the derived engine; the keyCount/shape parity probe is the diff oracle for this locale.
- [Doc 581](../../../corpus-master/corpus/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction.md) — Pin-Art discipline. The Round 3 revert is the canonical "probe-before-commit-at-sweep-tier caught it" event; this locale absorbs that lesson as its operating discipline.
- [`specs/tier-omega-cutover-audit.md`](../../specs/tier-omega-cutover-audit.md) — the parent audit; this locale is the natural successor for Rounds 2, 3, and 6 from that document, regrouped under one seed.
- `pilots/rusty-js-runtime/derived/src/module.rs` — the dominant edit surface. Lines 684–737 (`resolve_import_binding_value`), 884–929 (CJS import dispatch in `evaluate_module`), 1388–1552 (`populate_cjs_namespace_view`), and the default-synthesis branch at 1543.
- `legacy/host-rquickjs/tools/parity-measure.sh` — the diff-oracle harness. `RB_BIN` defaults to `target/release/cruftless` (post-cutover); this locale measures against that binary.

## I. Telos

Close the ESM/CJS namespace-synthesis gap between cruftless and bun on the 119-package parity corpus to ≥98% (117/119), with the residual 1–2 packages bounded to substrate concerns outside the ESM locale (parser, intrinsics, async-iterator runtime).

Concretely: when `Object.keys(import * as M from "pkg")` is observed in both engines, the two key sets must be **equal modulo a fixed, documented allow-list of bun-specific synthetic names** (currently empty; the locale's audit may admit a small set if a bun behavior turns out to be intentional and outside-engine).

### I.1 Bounded first-cut telos

The locale opens with three known gap families, each with one identified probe package. First cut = one substrate move per family + one §XVI yield (probe flip).

- **Family A — Under-synthesis (missing names)**. cruftless namespace is missing keys present in bun. Probe: **node-fetch** (–2 keys: `fetch` as named-alias of default-function, `FetchBaseError` from transitive errors/base.js).
- **Family B — Over-synthesis (spurious names)**. cruftless namespace has keys not present in bun. Probe: **enquirer** (+21 spurious keys). The mirror image of Round 3's blanket default-synthesis bug; suspected root cause is over-eager `populate_cjs_namespace_view` mirroring on a package that bun treats as pure ESM.
- **Family C — Re-export edge cases**. Specific names missing because of `export * from` / `export { x } from` resolution. Probe: **superstruct** (–1 key).

### I.2 Anti-telos (what NOT to do)

Carry these forward from the Round 3 revert:

- **No blanket synthesis**. Any default-export, named-alias, or re-export synthesis must be **conditional on a property of the source package** (presence/absence of `exports`, `type: "module"`, transpiler banner, etc.), not unconditional on a runtime branch.
- **No probe-skipping**. Every substrate move ships with a parity sweep at sweep-tier before the commit lands. The Round 3 incident proved that round-tier probes (single-package keyCount checks) miss the blast radius of namespace-synthesis changes; only the full 119-package sweep catches them.
- **No "fix one, break two" trades**. If a substrate move flips ≥1 package but breaks ≥1 other, revert and redesign. The locale tolerates slow rounds; it does not tolerate net-negative rounds.

## II. Apparatus

The locale operates on **resolver-instance #2** (module load) of the Doc 729 stack. The edit surface is `module.rs` plus a thin slice of `interp.rs` (the import-binding evaluator). No new resolver-instance is introduced; the locale tightens the existing one.

**Engine-diff oracle**: `legacy/host-rquickjs/tools/parity-measure.sh` against the 119-package basket on `/media/jaredef/T7/cruftless-pm-recon/`. A round's substrate move is judged by sweep-tier delta; a round's design phase uses single-package `parity-probe.mjs` runs.

**Probe shape (per package)**: a `probe-PKG.mjs` script that calls `import * as M from 'PKG'`, prints `Object.keys(M).sort()` plus a shape report (typeof per key), and exits 0. Comparison is line-diff against the bun output. Stored under `pilots/rusty-js-esm/probes/`.

**Reference reading**: for each probe package, the locale records the package's `package.json` `type`/`main`/`module`/`exports` fields and the relevant export statements in the entry file. This is the locale's reading discipline (Doc 581's "eight passes" applied at namespace-shape granularity).

## III. Ceiling

Out of scope for this locale (deferred or handled in other locales):

- **Top-level await** — deferred per Ω.5.b ceiling. Affects 0 of the 6 known failures.
- **Dynamic import resolution semantics** — handled by `rusty-js-runtime` proper; the locale only addresses static `import` shapes.
- **Async-iterator runtime semantics** — affects superagent/arktype crash class via `Array.prototype.rawIn` and dynamic-import errors; investigate, but fixes land in `rusty-js-runtime`, not here.
- **Bun-specific transpiler synthesis** — if bun synthesizes a name from a `.d.ts` file (suspected for `FetchBaseError`), that's a non-spec behavior. The locale will document and decide per-case whether to mirror.

## IV. Rung-0 baseline

| Metric | Value | Source |
|---|---|---|
| Parity at locale founding | 94.9% (113/119) | `results-2026-05-21-r3-reverted.json` |
| Known ESM-locale failures | 3 (node-fetch, enquirer, superstruct) | sweep mismatches |
| Other-locale failures | 3 (arktype load, superagent load, entities timeout) | sweep mismatches |
| Edit surface LOC | ~1500 (module.rs) + ~50 (interp.rs slice) | line-count |

## V. Trajectory pointer

See [`trajectory.md`](./trajectory.md) for the rung log.
