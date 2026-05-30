# tokenizer-error-classification-refinement — Trajectory

## TECR-EXT 0+1 — split missing-parser-feature into lex vs syntax (2026-05-25)

**Trigger**: Keeper directive (Telegram 9820) "B and then A." First apparatus-tier spawn from the tokenization-above-IR brief. Lands BEFORE the Tier-I substrate locales per LPA-EXT 3 Finding LPA.5.

**Discriminator** (pre-implementation survey): cruft's lexer prefixes its errors with `parse: lex error:` (e.g., `parse: lex error: unterminated regex (UnterminatedRegex)`). Other `parse: ...` reasons are syntactic-grammar errors (unexpected token, expected X, HoistableDeclaration-as-Stmt, etc.). Pre-TECR survey on the 2026-05-25 interpreted.jsonl: 36 lex-tier vs 529 syntax-tier within the previously-monolithic `missing-parser-feature` projection class.

**Edit** (~10 LOC in `full_pinart.rs::projection_axis`):

Old:
```rust
} else if r.starts_with("parse: ") || r.contains("parse error") {
    "availability/missing-parser-feature".into()
```

New:
```rust
} else if r.starts_with("parse: lex error:") {
    "availability/missing-lex-feature".into()
} else if r.starts_with("parse: ") || r.contains("parse error") {
    "availability/missing-syntax-feature".into()
```

Rename: `missing-parser-feature` → `missing-syntax-feature` to make the symmetry with PCR-EXT 2's `missing-lowering-feature` explicit. The apparatus now carries three named "missing-X-feature" classes: lex, syntax, lowering. Each is a substrate-tier where cruft has unimplemented features; each substrate locale targets its own tier without confounding the others.

**Build**: `cargo build --release --bin t262-full-pinart` completes cleanly.

**Re-interpretation** (dry-run; raw still sidecar-only):

| Metric | Pre-TECR | Post-TECR |
|---|---:|---:|
| Distinct pins | 269 | 269 (same; split produces 2 pins from 1 but ranking unchanged) |
| Records in `availability/missing-parser-feature` | 565 (across rungs) | 0 (renamed) |
| Records in `availability/missing-lex-feature` | — | **36** |
| Records in `availability/missing-syntax-feature` | — | **529** |

The 36-vs-529 split surfaces lex-tier substrate work as its own named coordinate. Tier-I substrate locales (numeric-literal-conformance, identifier-tokenization, string-literal-conformance) can now target the lex-tier coordinate without confounding their work with the much larger syntax-tier surface.

**Refreshed matrix** written to `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-25-TECR-EXT-1-rerun/`.

**Findings**

**Finding TECR.1 (the lex-tier coordinate emerges small but well-shaped)**: 36 records is small relative to the engagement-wide pool (~23k FAIL); it ranks outside the top-30. But the cluster-coherence multiplier conditions don't require LARGE coordinates — they require COHERENT ones. The 36 records correspond to test262 fixtures whose root cause is a single lex-tier rule (unterminated regex, unterminated string, line-terminator-in-regex, invalid-numeric, etc.). A substrate locale targeting them via numeric-literal-conformance or string-escape-conformance can close most of them with single-spec-rule rungs. The lex-tier surface is small-but-coherent, which is the cluster-coherence-multiplier shape. Standing recommendation: when an apparatus refinement splits a previously-monolithic coordinate into sub-classes, the resulting smaller coordinates can still satisfy the cluster-coherence multiplier if their COHERENCE is high; size and yield are independent properties.

**Finding TECR.2 (the apparatus now carries 3 named "missing-X-feature" classes — one per substrate tier)**: lex, syntax, lowering. These are the apparatus's articulation of "where in cruft's substrate stack the feature isn't yet implemented," partitioned by tier. The symmetry maps cleanly onto cruft's source-text pipeline (per the apparatus §VI two-pipeline diagram): source → tokens (lex) → AST (syntax) → bytecode (lowering) → runtime. Each tier has its own availability coordinate. Standing recommendation: extend this pattern to the runtime tier (`availability/missing-runtime-feature`) if/when runtime feature-gaps become distinguishable in cruft's traces — completes the source-text pipeline's coverage of the missing-X-feature family.

**Status**: TECR-EXT 1 CLOSED. The apparatus is ready for Tier-I substrate locale spawns (numeric-literal-conformance is next per the keeper's A sequence).

## TECR-EXT 2 — runtime tier added + missing-X-feature family lifted (2026-05-26)

**Trigger**: Keeper directive (Telegram 9847) "work on tokenizer error classification." Picks up the standing rec from Finding TECR.2 (extend the missing-X-feature family to the runtime tier).

**Survey** (against `apparatus/benchmarks/test262/2026-05-25-full/results.jsonl`):

- `compile: ... not yet supported`: 21 records (correct cruft-lowering markers)
- `not yet supported` without `compile:` prefix: 10 records ("bangla is not yet supported ..." — Intl/Temporal runtime gaps)
- `(stub)` markers: 0 in this matrix (WebAssembly stubs exist in cruft source but no tests trip them here)
- `webassembly`: 0
- All 10 non-`compile:` "not yet supported" records were being routed to `value-semantics/wrong-result` because the broad `r.contains("expected")` rule was firing first on "Expected a RangeError ...".

**Two-part edit** to `projection_axis`:

1. Added `availability/missing-runtime-feature` branch matching `not yet supported | not implemented | (stub) | webassembly` (after the `compile:` lowering check).
2. **Lifted the entire missing-X-feature family** (lex, syntax, lowering, runtime) above the generic assertion-text rules (`expected`, `samevalue`, `expected and thrown`, `wrong error`). Cruft self-tags these reasons with unambiguous tier discriminators (`parse: lex error:`, `parse:`, `compile:`, `not yet supported`); they must beat any generic assertion-text match that could siphon them.

**Net LOC**: ~25 (12 new + 13 reorder).

### Yield (against the 2026-05-25-full matrix)

| Projection class | Pre EXT 1 | Post EXT 1 | Post EXT 2 + reorder |
|---|---:|---:|---:|
| `availability/missing-lex-feature` | — | 36 | **61** (+25) |
| `availability/missing-syntax-feature` | 565 (as missing-parser-feature) | 478 | **1015** (+537) |
| `availability/missing-lowering-feature` | 115 | 115 | 115 |
| `availability/missing-runtime-feature` | — | — | **10** (new) |
| `value-semantics/wrong-result` (catch-all) | — | 5342 | 4771 (−571) |
| `parser-form/early-error` | — | 2393 | 2393 |

### Findings

**Finding TECR.3 (catch-all-rule siphon was material)**: The generic `r.contains("expected")` rule was siphoning ~571 records that belong to the missing-X-feature family. The pre-lift TECR-EXT 1 measurement of "36 lex vs 529 syntax" under-counted by the same factor. Standing recommendation: when adding new tier-discriminator rules to a categorizer with a broad assertion-text catch-all, lift the new discriminators above the catch-all in the same change — otherwise the new coordinate undercount silently propagates into downstream substrate-prioritization decisions.

**Finding TECR.4 (the missing-X-feature family is complete)**: The apparatus now carries the full source-text pipeline coverage (lex → syntax → lowering → runtime) as named coordinates. The family is closed against the pipeline diagram in apparatus/docs/repository-apparatus.md §VI. Future categorizer work should extend the family only when a new substrate tier emerges; otherwise refinement happens within tiers (sub-classes), not above them.

**Status**: TECR-EXT 2 CLOSED. Matrix re-rendered at `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-25-TECR-EXT-2-rerun/` (relocated 2026-05-30 from the originally-written stray path `pilots/pilots/test262-categorize/full-suite/results/2026-05-25-full/`; the doubled-pilots prefix came from an ad-hoc runner invocation against the cwd, not the canonical runner). The runtime-tier coordinate is small (10 records: Temporal/Intl) but coherent, cluster-coherence-multiplier shape per Finding TECR.1.
