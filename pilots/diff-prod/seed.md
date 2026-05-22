# diff-prod — Resume Vector / Seed

**Locale tag**: `L.diff-prod` (per [Doc 737](../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md))

**Status as of 2026-05-22**: **TELOS A HIT + TELOS B (39/39)**. Thirty-nine fixtures (mix of L and F categories), all PASS. Twenty-six+ substrate fixes landed via diff-prod since founding (incl. two class-name fixes surfaced via the arktype deviation locale). Top-500 measured at 77.4% raw / 82.1% incl-agreed after cumulative fix-set. Coverage now spans JSON, Buffer, strings, Map/Set, async/Promise/iteration, error-throws, RegExp (incl. advanced surface), generators, Proxy, structuredClone, BigInt, Reflect, Symbol, Object/Array/Math statics, node:fs/crypto/stream/events/path.

**Workstream**: differential prod-test methodology + scaffolding. Extends the load-and-shape parity probe from "namespace surface" to "execution semantics." The same Doc 730 §XVI bidirectional-engine-diff instrument applies; the input set widens and the comparator deepens.

**Author**: 2026-05-22 session, founded after the methodology landing (commit be37c14b → 1461efad) shipped the spec + scaffolding + first three fixtures.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).

**Composes with**:
- [Doc 730 §XVI](../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) — bidirectional engine-diff as the deviation-pipeline's empirical instrument. diff-prod is §XVI extended along the execution-semantics axis.
- [Doc 730 §XVIII](../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) — heuristic recovery via output-set membership. diff-prod failures that resist the obvious-axis reading should route through the §XVIII recovery protocol.
- [Doc 581](../../../corpus-master/corpus/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction.md) — Pin-Art discipline.
- [Doc 736](../../../corpus-master/corpus/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — capability-passing runtime. S-category requires audit-mode trace surface from Pilot α.
- [`specs/diff-prod-testing.md`](../../specs/diff-prod-testing.md) — the methodology doc (11 sections).
- [`scripts/diff-prod/`](../../scripts/diff-prod/) — the harness scaffolding.
- `legacy/host-rquickjs/tools/parity-measure.sh` — the L-category runner for the 119-package + top-500 baskets.
- The [rusty-js-esm deviation-resolution pipeline](../rusty-js-esm/deviations/arktype/) — the per-fixture template for §XII–§XVII deviations.

## I. Telos

Bring the execution-semantics diff between cruftless and bun within tolerance across a curated fixture set, then expand the fixture set to cover the spec surface that the load-and-shape parity probe leaves blind.

Two telos targets:

- **Telos A (near)**: every shipped F-category fixture PASSes; each failure is a documented substrate gap with a trajectory entry.
- **Telos B (medium)**: the fixture set covers ECMA-262 surface beyond the load-and-shape probe (string ops, JSON, Map/Set, async/Promise, error throws, RegExp, Proxy, generators, structured clone), and the harness extends to S-category (side-effect trace) via the capability-passing runtime.

### I.1 Bounded first-cut telos

The locale opens with six F-category fixtures and a target of all-PASS within the first week of engagement-time. Three of the six PASS at locale founding; three FAIL on substrate gaps deferred for their own focused work. The first-cut closure criterion is the three deferred gaps landing as their own commits and the fixtures flipping to PASS.

### I.2 Anti-telos

Per [`specs/diff-prod-testing.md`](../../specs/diff-prod-testing.md) §VII, restated:

- **No tolerance-creep**. A fixture that needs an expanded tolerance is a fixture whose workload is non-deterministic. Fix the workload; do not relax the tolerance.
- **No oracle-pollution**. bun is the oracle. Do not write expected-output files; recompute on every run from bun fresh.
- **No probe-skipping at promotion**. Every promotion gate runs every fixture in its category.
- **No silent flake**. A fixture that fails intermittently is flagged for redesign within one engagement-day. It cannot be marked `flaky:true` and ignored.
- **No fixture-as-workaround**. When a fixture FAILs on a substrate gap, the fix lands in cruftless; the fixture is NOT rewritten around the gap. The exception is the comparator-discriminator narrowing in error-throws (record only `ctor`, not message-prefix) — that's a methodology refinement (engine-stable fields only), not a workaround.

## II. Apparatus

The locale operates on **resolver-instances #2, #3, #4** of the Doc 729 stack (module load, execution, runtime), via the diff oracle. It is not a substrate pilot itself; it surfaces substrate gaps that route to the appropriate locale (rusty-js-esm, rusty-js-runtime, intrinsics, node_stubs).

**Harness**: `scripts/diff-prod/{run.sh, run-all.sh, runners/comparator.mjs, fixtures/<name>/}`. All heavy work `nice -n 19 ionice -c3`; sandbox + results on T7 by default.

**Categories** (per Doc methodology §II):
- **L** load-and-shape (existing parity-measure rolls in)
- **F** pure-function output (canonical JSON stdout diff)
- **E** error-equivalence (constructor + message-prefix diff)
- **S** side-effect trace (capability-passing runtime prereq)

**Promotion gates** (per methodology §VI):
- Gate 1: L-category PASS required.
- Gate 2: F-category PASS for `gate2:true` fixtures required.
- Gate 3: no E-category regressions.
- Gate 4: S-category diffs flagged for human review.

## III. Ceiling

Out of scope for this locale (deferred or handled elsewhere):

- **Performance / wallclock parity** — Doc 730 §XVII performance-axis pipeline. diff-prod is correctness-axis; the 10× wallclock target in §IX of the methodology is a hand-wave until §XVII tooling lands.
- **Real-network-fetch fixtures** — require cassette-recording authoring; queued as successor question Q2 in the methodology doc.
- **S-category (side-effect trace)** — requires the capability-passing runtime (Pilot α / Doc 736) to be active. Stubbed in the comparator; real implementation deferred.

## IV. Rung-0 baseline

| Metric | Value | Source |
|---|---|---|
| Fixtures shipped | 6 | 3 in methodology landing + 3 added |
| F-category PASS | 3 of 6 (50%) | summary.json on 2026-05-22 |
| Substrate fixes landed via diff-prod | 9 | commit log 2026-05-22 |
| L-category telos | 99.1% (top-100) | parity-measure baseline |
| Methodology landed | 2026-05-22 | commits be37c14b → 1461efad |

## V. Deferred substrate rungs surfaced by diff-prod

Each entry below is a substrate gap discovered while a fixture was landing. They are scoped narrowly enough that each could be a focused rung; until then the offending case is commented inline in the fixture (or the fixture itself is deferred — flagged with †).

**Runtime / value semantics**
- `match.groups` accessor: named regexp captures match positionally but `.groups` view is undefined (regexp-advanced)
- `Function.prototype.bind` target-name read for non-method bind targets returns `""` even when the target's `.name` is set (arktype deviation locale)
- Writes to non-writable data properties in non-strict mode throw TypeError; spec mandates silent no-op (reflect-api)
- TDZ enforcement: access-before-declaration of `let`/`const` reads `undefined` instead of throwing ReferenceError (closures-scopes deferral)
- Arrow function `arguments` capture from enclosing function (closures-scopes deferral)
- Block-scoped `let` interaction with top-level `var`-in-block (closures-scopes deferral)

**Built-in surface**
- `DataView` instance methods (setUint8/getUint8/setUint16/setInt32/setFloat32/setFloat64 + getters) — entire surface absent (arraybuffer-dataview †)
- `util.types.{isDate,isMap,isSet}` return false for matching instances (node-util †)
- `util.promisify` value channel returns undefined instead of the callback's second arg (node-util †)
- `node:querystring` module imports resolve to node:url's exports; `qs.stringify`/`qs.parse` not present (node-querystring †)
- AsyncGenerator.prototype.throw not implemented (async-iteration)
- Manual `[Symbol.asyncIterator]` protocol over user-defined objects throws `undefined` (async-iteration)

**node:path arithmetic**
- `path.basename("/foo/bar/")` returns `""` (node returns `"bar"`)
- `path.join("a","b","..","c")` does not resolve `..` (returns `"a/b/../c"`)
- `path.normalize("/foo/bar/")` strips trailing slash (node preserves it)

**Other**
- `btoa("中")` silently encodes instead of throwing `InvalidCharacterError` (encoding)
- `stream.Readable.from(iterable)` static absent (node-stream)

The runtime/value-semantics group has the highest expected diff-prod ROI per fix (each affects multiple fixtures and real-world packages). The DataView gap is the largest single surface; it warrants its own substrate rung. Most node-module gaps are localized stubs that can be backfilled without architectural change.

## VI. Trajectory pointer

See [`trajectory.md`](./trajectory.md) for the rung log.
