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
