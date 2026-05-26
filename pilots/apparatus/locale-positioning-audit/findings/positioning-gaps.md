# Positioning Gaps — LPA-EXT 3 output

Cross-references the current full-suite Pin-Art matrix (`pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-25-165734-p2/matrix.md`) against the locale-coordinate space. Flags top-N coordinates without locales, candidate-pending spawns, drift cases, and apparatus-refinement candidates per heuristics §IX.

Per LPA seed §Triggers, this rung is opportunistically refreshed; the snapshot below is as of 2026-05-25.

---

## I. Top-30 matrix coordinates vs locale coverage

The audit walked every `pilots/*/seed.md` and `pilots/*/*/seed.md`, extracted any 4-tuple `pin` string the seed cites, built a pin→locale map, then cross-referenced against the top-30 matrix coordinates.

| Rank | Count | Resolver / Tier | Cut-kind shape | Locale | Status |
|---:|---:|---|---|---|---|
| 1 | 4,152 | runtime/spec-builtins :: temporal | availability/missing-global | `temporal-availability/` | **COVERED**, FOUNDED |
| 2 | 2,008 | host-intrinsic/intl402 | availability/missing-global | — | **GAP** |
| 3 | 1,417 | ast-to-bytecode | availability/missing-method | `ast-bytecode-missing-method/` | **COVERED**, FOUNDED |
| 4 | 1,296 | ast-to-bytecode | parser-form/early-error :: SyntaxError | `ast-bytecode-syntaxerror-cluster/` | **COVERED**, FOUNDED |
| 5 | 1,244 | ast-to-bytecode | value-semantics/wrong-result | `ast-bytecode-wrong-result/` | **COVERED**, FOUNDED |
| 6 | 847 | runtime/buffer-typed-array | availability/missing-method | `typed-array-missing-method/` | **COVERED**, FOUNDED |
| 7 | 809 | source-to-ast/parser-early-error | parser-form/early-error :: SyntaxError | `parser-early-error-residual/` (+ nested `block-bound-names-dup/`) | **COVERED**, ACTIVE (BBND closed 95 tests of sub-cluster) |
| 8 | 659 | ast-to-bytecode | uncategorized/projection :: failure/other | `ast-bytecode-uncategorized-projection/` | **COVERED**, flagged as **apparatus-gap** per heuristics §IX |
| 9 | 622 | ast-to-bytecode | abrupt-completion/throw-missing :: TypeError | `ast-bytecode-missing-throw-typeerror/` | **COVERED**, FOUNDED |
| 10 | 614 | runtime/buffer-typed-array | value-semantics/wrong-result | `typed-array-wrong-result/` | **COVERED**, FOUNDED |
| 11 | 602 | uncategorized/resolver | uncategorized/projection :: failure/other | — | **GAP** (apparatus-refinement candidate per §IX) |
| 12 | 459 | uncategorized/resolver | value-semantics/wrong-result | — | **GAP** (apparatus-refinement candidate) |
| 13 | 389 | runtime/spec-builtins | value-semantics/wrong-result | `spec-builtins-wrong-result/` | **COVERED**, FOUNDED |
| 14 | 382 | host-intrinsic/intl402 | value-semantics/wrong-result | — | **GAP** (intl402 cluster member) |
| 15 | 350 | ast-to-bytecode | abrupt-completion/throw-missing :: Test262Error | — | **GAP** (sibling of rank 9; could absorb into ast-bytecode-missing-throw-typeerror at next chapter close) |
| 16 | 306 | runtime/buffer-typed-array | uncategorized/projection :: failure/other | — | **GAP** (apparatus-refinement candidate) |
| 17 | 282 | runtime/spec-builtins | uncategorized/projection :: failure/other | — | **GAP** (apparatus-refinement candidate) |
| 18 | 269 | uncategorized/resolver | availability/missing-global | — | **GAP** (apparatus-refinement candidate) |
| 19 | 262 | runtime/regexp | value-semantics/wrong-result :: SyntaxError | — | **GAP** (regexp cluster member) |
| 20 | 251 | runtime/spec-builtins :: temporal | value-semantics/wrong-result :: ReferenceError | — | downstream of rank 1; **will absorb** as temporal-availability TA-EXT 1+ lands |
| 21 | 239 | runtime/buffer-typed-array | abrupt-completion/throw-missing :: TypeError | — | **GAP** (sibling of rank 10) |
| 22 | 231 | runtime/spec-builtins | availability/missing-global | — | **GAP** (small absent-chapters: AsyncDisposableStack, etc) |
| 23 | 229 | runtime/regexp | regexp-semantics :: failure/other | — | **GAP** (regexp cluster member) |
| 24 | 225 | runtime/buffer-typed-array | runner-harness/$262 :: ReferenceError | — | **GAP** (measurement residue per heuristics §IV.E) |
| 25 | 223 | host-intrinsic/intl402 | availability/missing-method | — | **GAP** (intl402 cluster member) |
| 26 | 189 | runtime/spec-builtins | iteration/iterator-protocol :: failure/other | — | **GAP** |
| 27 | 187 | runtime/spec-builtins :: temporal | abrupt-completion/wrong-throw-type :: TypeError | — | downstream of rank 1; will absorb with temporal MVP |
| 28 | 180 | ast-to-bytecode | availability/missing-global | — | **GAP** (sibling of rank 3) |
| 29 | 178 | ast-to-bytecode | abrupt-completion/throw-missing :: ReferenceError | — | **GAP** (sibling of rank 9) |
| 30 | 174 | runtime/spec-builtins | availability/missing-method | — | **GAP** |

**Summary**: top-10 fully covered by the top-10 spawn batch (commit 561b7aa4). **18 of the top-30 are gaps**, totaling ~5,400 fails across the 11–30 range.

---

## II. Gaps by class (per heuristics §III partition discipline)

### Class A — apparatus-refinement candidates (per heuristics §IX)

Coordinates with `uncategorized/projection :: failure/other` or `uncategorized/resolver`. Per heuristics §IX, these are the apparatus's own measurement residue: failure/other still contains parseable reason-text the categorizer hasn't extracted. The right move is **refine the categorizer first**; spawning substrate locales for these would be working from blurred coordinates.

| Rank | Count | Pin | Move shape |
|---:|---:|---|---|
| 8 | 659 | ast-bytecode :: uncategorized/projection | apparatus refinement (already flagged at spawn) |
| 11 | 602 | uncategorized/resolver :: uncategorized/projection | apparatus refinement |
| 12 | 459 | uncategorized/resolver :: wrong-result | apparatus refinement |
| 16 | 306 | typed-array :: uncategorized/projection | apparatus refinement |
| 17 | 282 | spec-builtins :: uncategorized/projection | apparatus refinement |
| 18 | 269 | uncategorized/resolver :: missing-global | apparatus refinement |
| 24 | 225 | typed-array :: runner-harness :: ReferenceError | measurement residue (heuristics §IV.E) |

**Total in this class: 2,802 fails across 7 coordinates**. Recommended: spawn a `pinart-categorizer-refinement` apparatus-pilot to extract parseable reason-text into more specific projection classes. Doing this BEFORE substrate work on these ranks would convert apparatus-tier mass into substrate-tier coordinates with clear move shapes.

### Class B — subsystem-absent-chapter clusters (per heuristics §IV.A)

Coordinates that name an absent subsystem (Temporal already covered at rank 1; Intl402 across 3 ranks; small absent-chapters at rank 22).

| Cluster | Ranks | Combined count | Move shape |
|---|---|---:|---|
| **Intl402** | 2 + 14 + 25 | **2,613** | Implement-Chapter; spawn `intl402-availability/` as canonical instance |
| Small absent-chapters | 22 | 231 | AsyncDisposableStack, AsyncDisposableContext, etc; small + diffuse; defer until classed |

**Recommended**: spawn `intl402-availability/` mirroring temporal-availability/'s pattern (subsystem availability gate first; downstream value-semantics + missing-method ranks will absorb as the subsystem skeleton lands).

### Class C — regexp coordinate cluster (per heuristics §IV.D)

| Rank | Count | Pin |
|---:|---:|---|
| 19 | 262 | regexp :: wrong-result :: SyntaxError |
| 23 | 229 | regexp :: regexp-semantics :: failure/other |

**Total: 491 fails across 2 coordinates**. Both intrinsic-semantics shape per heuristics §IV.D. Recommended: spawn a single `regexp-conformance/` locale that covers both as sibling rungs (likely one cluster of related fixtures by surface family).

### Class D — siblings of already-covered ranks (potential rung-extension)

Ranks that share resolver+rung with covered top-10 ranks but differ in failure-mode or projection.

| Existing locale | Sibling rank | Sibling pin | Recommendation |
|---|---|---|---|
| `ast-bytecode-missing-throw-typeerror/` (rank 9) | 15 | abrupt-completion :: Test262Error (350) | extend the existing locale's coverage to include this failure-mode at next chapter close |
| `ast-bytecode-missing-throw-typeerror/` (rank 9) | 29 | abrupt-completion :: ReferenceError (178) | same — extend |
| `ast-bytecode-missing-method/` (rank 3) | 28 | availability :: missing-global (180) | extend; siblings absorb on close |
| `typed-array-wrong-result/` (rank 10) | 21 | abrupt-completion :: TypeError (239) | extend |

**Total in this class: 947 fails across 4 ranks** that can absorb into existing locales' scope rather than spawn new ones.

### Class E — temporal-downstream (will absorb as temporal-availability lands)

| Rank | Count | Pin | Recommendation |
|---:|---:|---|---|
| 20 | 251 | temporal :: wrong-result :: ReferenceError | no new locale; will absorb at TA-EXT 1+ |
| 27 | 187 | temporal :: wrong-throw-type :: TypeError | same |

**Total: 438 fails across 2 ranks** that should resolve as a side-effect of TA-EXT 1+ (registration MVP).

### Class F — net-new substrate coordinates not yet locale'd

| Rank | Count | Pin | Recommendation |
|---:|---:|---|---|
| 26 | 189 | spec-builtins :: iteration/iterator-protocol :: failure/other | spawn or extend `iter-protocol-bytecode-rewrite/` to absorb |
| 30 | 174 | spec-builtins :: availability/missing-method | small; spawn only if part of a larger cluster |

---

## III. Drift cases

Locales whose declared coordinate may have shifted since spawn:

- **`ast-bytecode-uncategorized-projection/`** — at spawn (2026-05-25) explicitly flagged as APPARATUS-GAP per heuristics §IX (the seed says "refine the categorizer first; substrate work would be working from blurred coordinates"). Locale is FOUNDED, no substrate work has happened. Drift status: still apparatus-pending; locale is correctly positioned but its successor (the categorizer refinement) has not been spawned. Recommendation: spawn `pinart-categorizer-refinement/` (Class A above) as the prerequisite; ast-bytecode-uncategorized-projection's substrate work begins after that.

- **`ts-resolve-*/` family** (11 sub-locales) — TSR cascade completed parse-parity to 100% on the rxjs+ajv+pino corpus per CLAUDE.md. Their declared coordinates were TSR-tier; with parse-parity closed, the residual TXC execute-parity gap (70.9%) is runtime-substrate territory per Finding IX.9. Drift status: the TSR-family locales are CLOSED but their original-coordinate-claims may now be supersededby the TXC residual. The audit defers per-locale verification; surfacing as a candidate for LPA-EXT 1-style sweep (older locales' coordinates re-read after a major resolver-tier close).

- **PEER (parser-early-error-residual)** — at spawn the 100-exemplar suite was a fresh-surface read; BBND (nested) closed 95 redeclaration tests + 4 of the 100 exemplars; the remaining 96 exemplars are NOT redeclaration shapes (PEER trajectory's surface-family breakdown shows language/statements + language/expressions dominating). Drift status: PEER's seed says 809 fails pool; post-BBND the pool is ~714. The seed's number is stale but the coordinate is still correct.

---

## IV. Locales without matrix coordinates (substrate-only work)

The LPA found some locales whose seed does NOT cite a specific matrix `pin` string. These are substrate-engineering work driven by other inputs (corpus directives, surface-completeness audits, capability work). Not gaps in the matrix-coverage sense, but a separate class.

Examples (sampled):
- `rusty-js-jit/` and nested LeJIT-family locales — driven by CRB (cross-runtime-bench) + Doc 740 cascade rather than matrix coordinates
- `rusty-js-caps/` — driven by Doc 736 capability-passing rather than matrix
- `rusty-js-shapes/` + nested CMig — driven by JIT-tier substrate composition rather than matrix
- `lexer-goal-symbol-selection/`, `parser-precedence-in-flag/`, `for-head-non-binding-lhs/` — driven by FCA-amortization spinoff chain rather than matrix

These constitute the **substrate-architecture work-class** (vs. the matrix-coverage work-class). Both are legitimate; the audit just notes the partition: matrix-driven locales target named coordinates; architecture-driven locales target structural-completeness goals.

---

## V. Recommendations summary (priority-ordered)

1. **`pinart-categorizer-refinement/` apparatus-pilot** — would unblock Class A (2,802 fails across 7 coordinates) by extracting parseable structure from the current uncategorized projections. Highest leverage; spawning is purely apparatus-tier work with no substrate risk.

2. **`intl402-availability/` substrate-pilot** — mirrors temporal-availability/'s shape; covers Class B's 2,613 fails across 3 ranks via the same single-decision-avalanche pattern Finding TA.1 surfaced.

3. **`regexp-conformance/` substrate-pilot** — covers Class C's 491 fails across 2 ranks; one locale absorbs both sibling rungs.

4. **Extend existing top-10-batch locales to absorb sibling coordinates** (Class D, 947 fails across 4 ranks) at next chapter close. No new locales needed; the existing locales' scope expands naturally.

5. **Class E (438 fails)** auto-resolves with TA-EXT 1+ substrate work; no new spawns needed.

6. **Drift-case action**: spawn pinart-categorizer-refinement before any substrate work on ast-bytecode-uncategorized-projection.

**Cumulative addressable via recommendations 1–4**: **6,853 fails (~29% of all FAIL records)** addressable through 3 new apparatus/substrate-pilot spawns + scope-extension of 4 existing locales.

---

## VI. How this doc gets refreshed

Per LPA seed §Triggers:

- **After any new locale spawn**: update §I + §II's locale-coverage column.
- **After any full-suite categorize re-run**: re-walk the matrix and rebuild the table.
- **After any locale CLOSES**: check whether covered ranks have shrunk + which sibling ranks were absorbed.
- **On keeper directive**: explicit re-audit.

The doc is per-snapshot (not append-only) — re-rendering at a new matrix run is the discipline. Prior snapshots survive in git history.

---

*Snapshot 2026-05-25 against test262-full-2026-05-25-165734-p2 matrix. 18 of top-30 ranks are gaps; 6,853 fails addressable through 3 new spawns + 4 scope-extensions; 1 apparatus-tier categorizer-refinement spawn would unblock 2,802 of those.*
