# rusty-js-json-fast — Resume Vector / Seed

**Locale tag**: `L.rusty-js-json-fast` (top-level per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (JSF-EXT 0)**. No code yet. Spawned per keeper directive 2026-05-23 18:12-local alongside three sibling spawns (rusty-js-regex-fast, web-crypto/subtle-wireup, rusty-js-http-server) addressing the engagement's named substrate gaps from CRB-EXT 9 reading.

**Workstream**: hand-rolled fast-path JSON.stringify (and possibly JSON.parse if surface allows). Replaces the current character-by-character Rust-interp-tier implementation with a tighter-emission engine. Per CRB-EXT 9 reading + Findings VI.1 HIGH priority: JSON.stringify is estimated at 2-3× contributor to cruft's 20.34× cruft/node gap on json_parse_transform realistic workload.

**Author**: 2026-05-23 session.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`). Standalone top-level pilot.
**Composes with**:
- [Findings doc VI.1](../rusty-js-jit/findings.md) — HIGH priority forward-derived candidate pilot
- [CRB-EXT 9 jit-eligible-vs-realistic doc](../cross-runtime-bench/docs/jit-eligible-vs-realistic.md) — empirical anchor for the 20.34× gap + JSON's 5-10× contribution estimate
- [Findings doc IV.4 standing fuzz](../rusty-js-jit/findings.md) — any default-on flip at this pilot uses the canonical fuzz per standing rule 10
- [CMig-EXT 17.bis](../rusty-js-shapes/consumer-migration/trajectory.md) — shape-aware property iteration; this pilot's stringify path reuses the shape-iter-chain pattern
- [Doc 731](../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) §XIV.d typed-i64 alphabet — number-stringify fast path
- [Doc 735 §X.h.b](../../../corpus-master/corpus/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — (P2) four-sub-case categorization
- [Doc 738 §II](../../../corpus-master/corpus/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md) — source-tier convention space

## I. Telos

**Empirical answer to**: can cruft's JSON.stringify reach within 2-3× of node's hand-coded implementation on the CRB json_parse_transform fixture?

Per CRB-EXT 9: cruft 2481 ms vs node 122 ms (20.34×); estimated component decomposition placed JSON.stringify at ~5-10× contributor + JSON.parse at ~2-3×. Closing the JSON.stringify gap from ~7× → ~2× would cut json_parse_transform cruft-time from ~2481 ms to ~700-800 ms (an estimated −68% wall-clock; cruft/node ratio drops from 20× to ~6×).

### I.1 First-cut scope

- **JSON.stringify only** at first cut. JSON.parse is a separate sub-pilot if telos extends.
- **Aarch64 only** (the engagement's reference hardware).
- **Number-stringify fast path** for typed-i64 + small-f64 cases.
- **String-escape fast path** with branchless ASCII detection + SIMD-or-equivalent for the common case.
- **Object/Array iteration** via shape-aware path (per CMig-EXT 16.bis pattern).
- **Chunked output buffer** (avoid Rust String reallocation per byte).

Out of scope: JSON.parse; replacer fn; spaces param; toJSON callback.

### I.2 Falsifiers

**Pred-jsf.1**: post-implementation, json_parse_transform CRB fixture drops by ≥40% wall-clock (target 2481 ms → ≤1500 ms). Falsifier: <40% reclaim → JSON.stringify wasn't the bottleneck or implementation didn't deliver.

**Pred-jsf.2**: canonical fuzz (CMig-EXT 17 fuzz-canonical.mjs) remains byte-identical post-implementation (acc=-932188103 vs node). Falsifier: divergence → (P2.c) illegal-speed bug.

**Pred-jsf.3**: diff-prod 42/42 holds. Falsifier: any regression.

**Pred-jsf.4**: number-stringify fast path correctly handles edge cases (NaN, ±Infinity, ±0, denormals, integer-valued f64). Falsifier: any divergence from node on these cases.

**Pred-jsf.5**: composes with shape default-on (CMig-EXT 14) + the iter chain pattern (CMig-EXT 16.bis). Falsifier: regression on shape-enrolled object stringify.

## II. Apparatus

Hand-rolled JSON.stringify replacement at the rusty-js-runtime tier. Composes with:
- **Shape substrate** (default-on): iteration via shape-iter-chain pattern
- **LeJIT-tier pilots** (default-on): no direct composition; JSON.stringify runs in interp not JIT
- **CRB pilot** (standing): bench against json_parse_transform fixture
- **Canonical fuzz** (standing): correctness via fuzz-canonical.mjs

Per Doc 738 §II.e: code lands at `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (extending or replacing the current json_stringify implementation). Per §II.b: helper functions follow post-§A8.32 receiver-discriminated form (no `_via` since not Runtime-dispatching; pure-primitive helpers).

## III. Methodology

1. **JSF-EXT 0** — workstream founding (this seed + trajectory + scaffold).
2. **JSF-EXT 1** — bench probe baseline. Read current json_stringify implementation; measure per-op cost via micro-benchmarks (object stringify N=10k; number stringify N=1M; string-escape N=1M). Output: `docs/bench-baseline.md`.
3. **JSF-EXT 2** — design doc. Hot-path component decomposition; fast-path strategies (branchless ASCII; integer-int-to-decimal lookup table; chunked buffer); algorithm choices (Ryu for f64? Grisu? Direct manual emission?). Output: `docs/design.md`.
4. **JSF-EXT 3** — implementation behind `CRUFTLESS_JSF_FAST=1` env flag. Substrate-introduction.
5. **JSF-EXT 4** — composition with shape + canonical fuzz + diff-prod + CRB.
6. **JSF-EXT 5** — fuzz probe (canonical + JSON-specific edge cases).
7. **JSF-EXT 6** — default-on flip if Pred-jsf.1-.5 all hold.

## IV. Carve-outs and bounded scope

- JSON.stringify only (not .parse) at first cut
- Aarch64 only
- No toJSON callback / replacer / spaces support
- Shape-aware iteration via existing pattern; no new shape work
- Per Findings rule 5 + standing rule 10: default-on flip uses canonical fuzz as fuzz-probe-level instrument

## V. Standing artefacts

- `pilots/rusty-js-json-fast/seed.md`, `trajectory.md`
- `pilots/rusty-js-json-fast/docs/` for bench-baseline + design
- `pilots/rusty-js-json-fast/fixtures/` for JSON-specific fuzz / bench fixtures
- Implementation lands in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read CRB-EXT 9's jit-eligible-vs-realistic doc + Findings doc VI.1 entry for empirical context.
