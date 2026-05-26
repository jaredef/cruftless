# dynamic-import-attributes — Trajectory

## DIA-EXT 0 — FOUNDING (2026-05-26)

Spawned per keeper directive (Telegram 9859) as Cluster D of Tier K.

Pool: 41 fixtures from `language/expressions/dynamic-import/import-attributes/`. Baseline: 0/41 PASS. Verified direct probe: `import('x', { with: { type: 'json' } })` rejects at the comma with `parse: expected ')' in dynamic import()`.

Per RFSDO's standing protocol: do NOT add `import-attributes` to the SKIP-list — the feature is stage-4 and in ECMA-262. cruft should implement the syntax.

## DIA-EXT 1 — LANDED (2026-05-26)

### Edit (~18 LOC in expr.rs)

After parsing the first AssignmentExpression in the dynamic-import branch:
```rust
if matches!(self.current_kind(), TokenKind::Punct(Punct::Comma)) {
    self.bump()?;
    if !matches!(self.current_kind(), TokenKind::Punct(Punct::RParen)) {
        let attrs = self.parse_assignment_expression()?;
        arguments.push(rusty_js_ast::Argument::Expr(attrs));
        if matches!(self.current_kind(), TokenKind::Punct(Punct::Comma)) {
            self.bump()?;
        }
    }
}
```

AST stores both args via `arguments` Vec; runtime stub unchanged.

### Probes (Rule 23 verification at landing)

- `import('x')` → parses ✓ (one-arg backward-compat)
- `import('x', { with: { type: 'json' } })` → parses ✓ (two-arg)
- `import('x',)` → parses ✓ (single-arg trailing comma)
- `import('x', {with:{type:'json'}},)` → parses ✓ (two-arg trailing comma)
- `import('x', {with:{type:'json'}}, extra)` → REJECTS ✓ (arity-3 not allowed by grammar)

All five expected outcomes confirmed.

### Yield

- DIA exemplar pool: **0 → 40/41 PASS (+40, 97.6%)**.
- Diff-prod: 42/42 maintained.
- Cross-locale regression sweep: NLC 147, IDT 261, SLEC 59, LTC 31, HDSB 150 all unchanged.
- WBMS pool denominator drops to 29/264 from 37/264 — NOT a regression: 8 nested-with-import-defer / nested-with-source-phase tests are now SKIPped by the RFSDO deny-list (correctly routed to inapplicable). Engagement-wide PASS+SKIP count is flat.

### Residual (1 test)

`2nd-param-evaluation-sequence.js` — `Expected SameValue(1, 0)`. The test exercises evaluation-sequence semantics of the second-arg attributes expression (specifically, that side-effects in the attributes object run after the specifier evaluates but before module-resolution). cruft's `__dynamic_import` is a throwing stub — the second arg is parsed and stored but never evaluated. Runtime-tier residual; belongs to a future `dynamic-import-runtime` locale, not DIA.

### Findings

**Finding DIA.1 (ratified-spec features earn substrate, not SKIP)**: The RFSDO discipline (apparatus SKIP for stage-X) requires symmetric treatment of ratified features (substrate implementation). DIA is the canonical example: `import-attributes` is a stage-4 / ECMA-262 feature, so the right move is substrate — accept the grammar. Compare to IMM where the cluster's features were stage-3 / stage-X and got the apparatus SKIP. Standing recommendation: when triaging a missing-syntax-feature cluster, first check the spec stage of the feature(s) involved; stage-4+ goes substrate, stage-3 or earlier may go apparatus SKIP.

**Finding DIA.2 (the WBMS denominator drop is a measurement artifact)**: WBMS pool went 37/264 → 29/264 not because cruft regressed but because RFSDO's deny-list (landed in the same arc) correctly re-routed 8 previously-passing-via-with-parse tests to SKIP (they had nested with + import-defer/source). Standing recommendation: when computing yield deltas across locales after an apparatus-tier change, distinguish "regression" (FAIL count rose) from "denominator shift" (PASS or FAIL moved to SKIP for correct reasons). Yield reports should track PASS / SKIP / FAIL separately when the apparatus is in flux.

### Status

DIA-EXT 1 CLOSED. Sole residual is runtime-tier; belongs to a future dynamic-import-runtime locale.
