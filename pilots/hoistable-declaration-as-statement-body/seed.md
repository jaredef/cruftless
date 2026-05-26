---
name: hoistable-declaration-as-statement-body
description: §13.6 IfStatement + Annex B B.3.4 — accept plain FunctionDeclaration as the Statement body of `if` / `else` under sloppy mode.
type: project
---

# hoistable-declaration-as-statement-body — Seed

## Substrate-pilot — Tier K, missing-syntax-feature concentration (HDSB).

Spawned per keeper directive (Telegram 9853) selecting Cluster A from the TECR-EXT 2 lift surface. Largest single shape in `availability/missing-syntax-feature` (475 of 1017, 46.7%).

## Telos

§13.6 IfStatement parser extended per Annex B B.3.4: accept plain `FunctionDeclaration` as the Statement body of `if` / `else` in sloppy (non-strict) mode. Strict mode and `function*` generators remain rejected. for / while / do-while / with / labelled bodies are NOT in the carve-out.

Coordinate:
```
parser-form / annex-B :: E0/parse-tier ::
  cut/IfStatement-Statement-body ::
  property/sloppy-FunctionDeclaration-permitted-in-if-body
```

## Apparatus

- `pilots/rusty-js-parser/derived/src/parser.rs` — new `Parser::allow_annex_b_function_in_substatement` field (default false).
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_substatement` — sloppy-mode + carve-out branch that falls through to `parse_statement` instead of erroring.
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_if_statement` — save/restore the flag around `parse_substatement` calls for consequent and alternate.
- **Exemplar suite**: `pilots/hoistable-declaration-as-statement-body/exemplars/exemplars.txt` — 475 fixtures (all matrix records emitting the rejection).

## Baseline (FOUNDING)

PASS=0/475 at HDSB-EXT 0 (all 475 records reject at parse-time).

## Methodology

### HDSB-EXT 1 — Annex B parser carve-out (LANDED)

Edits above. ~25 LOC across parser.rs + stmt.rs.

**Yield**: 150/475 PASS (31.6%). Diff-prod 42/42 maintained. Cross-locale tokenization sweep: no regression (NLC 147, IDT 261, SLEC 59, LTC 31 all unchanged).

### HDSB-EXT 2 — Annex B B.3.4 binding semantics (DEFERRED)

The parser-extension opens the door but cruft's lowering tier does not create the right binding shape. Annex B B.3.4 requires:
- A LEXICAL binding scoped to the IfStatement's block, AND
- A VAR binding hoisted to the enclosing function scope (with specific evaluation-path conditions about whether the var binding is initialized at function entry).

Residual yield decomposition (325 fails):
- ~120 fail on binding semantics ("Expected SameValue undefined vs function", "An initialized binding is not created", "value is not updated", "f is not defined").
- ~25 fail on cruft's own "Identifier `f` cannot be redeclared (lexical/var conflict)" — cruft creates a too-strict lexical binding that conflicts with var redeclaration.

**Substrate site**: lowering tier (bytecode emitter for IfStatement-with-FunctionDeclaration-body). Substantial scope (Annex B B.3.4 has multiple conditional branches per spec text); separate locale candidate.

### HDSB-EXT 3 — eval-scope harness visibility (DEFERRED, runtime tier)

~120 fails are `assert is not defined` / `fnGlobalObject is not defined` inside direct/indirect eval. The harness binds `assert` at the top level; cruft's eval scope chain doesn't surface that binding into the eval frame. This is unrelated to HDSB — it's a cruft eval-scope cluster. Belongs to its own locale (eval-scope-binding-chain).

## Composes-with

- `apparatus/locales/CANDIDATES.md` Tier K — sibling missing-syntax-feature clusters (WBMS, IMM, DIA, CAR).
- `pilots/apparatus/tokenizer-error-classification-refinement/` — the categorizer refinement that surfaced this cluster's true size (TECR-EXT 2 lift reclaimed the records from the value-semantics catch-all).

## R13 prospective C1-C4

- C1 (sibling): HOLDS — existing FunctionDeclaration parse path at parse_statement.
- C2 (shape-compat): HOLDS — single conditional branch in parse_substatement + flag plumbing.
- C3 (cost-positive): HOLDS — single rule, 1-round closure.
- C4 (bail-safe): HOLDS — parser-only, gated by strict-mode + generator carve-out.

## Status

HDSB-EXT 1 LANDED. 150/475 (31.6%) direct yield. HDSB-EXT 2 (lowering-tier binding semantics) and HDSB-EXT 3 (eval-scope harness visibility) deferred as separate substrate scopes.
