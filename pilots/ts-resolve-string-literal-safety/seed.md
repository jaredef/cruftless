# ts-resolve-string-literal-safety — Resume Vector / Seed

**Locale tag**: `L.ts-resolve-string-literal-safety` (top-level per Doc 737 §IV)

**Status as of 2026-05-24**: **CHAPTER CLOSED at TRSLS-EXT 1 (1 implementation round; Pred-trsls.5 HELD; sixth corroboration of standing rule 13)**. All five Pred-trsls.* HELD. Parse-success lifted 37.7% → **47.1%** (+9.4 pp; 3× over the ≥3 pp target). 60 files transitioned out of STRIP-error category. Substrate bug eliminated. Post-fix failure-table now cleanly separates TS-feature gaps from TSR bugs.

**Historical status (founding)**: WORKSTREAM FOUNDED (TRSLS-EXT 0). Spawned per keeper directive at the close of TCC-EXT 1. This locale is the **first data-driven sub-locale** from TCC's failure-table — addresses Finding TCC.3 (substrate bug: TSR's strip step corrupts source inside string/template literals on ~11 corpus files; root cause located before founding).

**Workstream**: fix TSR's `strip.rs` scanner so it does not corrupt template-literal substitution boundaries. Root cause located prior to founding: the Scanner uses `LexerGoal::Div` for every `next_token` call; when the lexer is at the `}` closing a template substitution, it needs `LexerGoal::TemplateTail` to re-enter template mode. Without the goal switch, the lexer treats the `}` as a Punct, then the subsequent template-tail bytes lex as fresh tokens and the `` ` `` reads as a stray punctuator → `UnterminatedString` / `InvalidIdentifier` lex errors.

**Author**: 2026-05-24 session.
**Parent**: none (top-level; sibling of `ts-resolve`).
**Siblings**: `ts-resolve/` (the substrate this fix lands in), `ts-consumer-corpus/` (the measurement instrument that surfaced the bug).
**Composes with**:
- [TCC-EXT 1 Finding TCC.3](../ts-consumer-corpus/trajectory.md) — bug discovery + categorization
- [TCC failure-table](../ts-consumer-corpus/results/2026-05-24/failure-table.md) — quantification (rows 8-9)
- [TSR-EXT 3 strip.rs](../ts-resolve/derived/src/strip.rs) — substrate to modify
- [rusty-js-parser Lexer](../rusty-js-parser/derived/src/lexer.rs) — LexerGoal::TemplateTail semantics (line 22-24)
- [docs/standing-rule-13-prospective-application.md](../../docs/standing-rule-13-prospective-application.md) — fifth prospective application; expected ≤3 rounds

## I. Telos

**Empirical answer to**: does adding template-substitution-aware goal-switching to TSR's Scanner eliminate the lex-error class of failures in the corpus (Finding TCC.3) without regressing the 141 currently-OK files?

The bench-anchored target: post-fix, TCC re-measure shows parse-success-rate lifts from 37.7% by **at least the count of lex-error files** (11 from rows 8-9 in the failure table) — and possibly more if those PARSE errors downstream were masked by the corrupted strip.

### I.1 First-cut scope

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. Skip any "wrap the issue with a guard" intermediate; **directly track template-substitution nesting + select the correct LexerGoal per call**.

- Scanner state: add a small Vec<()> "template-substitution-depth" stack (each `${` opens, each matching `}` closes inside template context)
- Goal selection rule: when the previous token was a Template{Head, Middle} OR when we're at `}` and inside a template substitution → use `LexerGoal::TemplateTail` for the next `next_token`
- This is exactly how `rusty-js-parser`'s parser handles it (consult its expr.rs template-handling code if precedent useful)

### I.2 Constraints (Pin-Art enumeration)

```
C1. Existing 24/24 ts-resolve tests continue to PASS.
C2. TCC's 141 currently-OK files remain OK post-fix (no regression).
C3. TCC's lex-error files (~11) lift to OK or to a different
    failure category (no longer corrupted by the strip).
C4. No new bytes added to strip ranges as a side effect; the fix
    is purely in goal selection during token scan.
C5. Per Doc 731 alphabet-purity claim: the JS/TS alphabet boundary
    is preserved — TSR still uses the JS parser's token alphabet,
    just selects the right goal-symbol per call.
C6. Per docs/standing-rule-13-prospective-application.md §3 conditions:
    (C1.sibling-anchor) rusty-js-parser's parser already does this
                        goal-switching correctly; precedent exists
    (C2.shape-compat)   Scanner is mine; can add state freely
    (C3.cost-positive)  one extra small enum check per token; <1ns/token
    (C4.bail-safe)      regression test in TSR's own test suite
                        catches any breakage of currently-OK paths
```

### I.3 Falsifiers

**Pred-trsls.1**: fix in ≤40 LOC (small enough that the disposition is unambiguous).

**Pred-trsls.2**: TSR's 24/24 unit tests continue to PASS post-fix.

**Pred-trsls.3**: TCC re-measure shows ≥10 of the 11 lex-error files transition out of lex-error category (either to OK or to a different failure tag). The substrate bug's specific manifestation is gone.

**Pred-trsls.4**: TCC parse-success-rate **lifts by at least 3 percentage points** (from 37.7% to ≥40.7%) — i.e., at least 11 files become OK, even if some templates inside the corrected files reveal new downstream issues.

**Pred-trsls.5 (DISCIPLINE FALSIFIER per docs/standing-rule-13-prospective-application.md §5)**: locale closes in ≤3 implementation rounds. Fifth prospective application; expected to close in 1 round since the root cause is pre-located.

## II. Apparatus

- **Code change**: `pilots/ts-resolve/derived/src/strip.rs::Scanner::lex_all` — add template-substitution-depth tracking + select goal per call
- **Regression test**: add a strip-test case with `` `pre${x}post` `` and `` `${a}${b}` `` in a `.ts` source
- **Re-measurement**: re-run `cargo run --release -p ts-consumer-corpus --bin tcc-measure`; compare summary.md + failure-table.md
- **Bench instruments**: TSR unit suite + diff-prod (untouched)

## III. Methodology

1. **TRSLS-EXT 0** — workstream founding (this seed + trajectory + manifest refresh + CANDIDATES update).
2. **TRSLS-EXT 1** — implementation + regression test + re-measure + chapter close. (One-round close expected per Pred-trsls.5.)

## IV. Carve-outs and bounded scope

- Template-substitution goal-switching only at first cut. Regex-literal goal-switching (`InputElementRegExp`) is a separate concern and deferred unless TCC's `lex-invalid-identifier` rows turn out to be regex-related (inspect the 4 such files at TRSLS-EXT 1 time).
- No change to the strip range logic; only to goal selection.
- No new TS feature support.

## V. Standing artefacts

- `pilots/ts-resolve-string-literal-safety/seed.md`, `trajectory.md`
- `pilots/ts-resolve/derived/src/strip.rs` (edit; the fix lands here, not in this locale's dir)
- `pilots/ts-resolve/derived/tests/strip.rs` (test addition)

## VI. Resume protocol

Read this seed, then trajectory.md tail. The root cause is documented in seed §I above; the fix should land in TSR's `strip.rs::Scanner::lex_all` by replacing the unconditional `LexerGoal::Div` with goal selection based on a template-substitution-depth stack and the kind of the previous token. Verify by re-running TCC; expect parse-success to lift ≥3pp.
