# block-bound-names-dup — Seed

Nested locale under `parser-early-error-residual`. Doc 737 §II promotion: BoundNames-duplication at block scope is a multi-test-coherent sub-shape worth its own seed/trajectory pair.

## Telos

§13.2.1 Block static-semantics early errors:
- LexicallyDeclaredNames of StatementList must not contain duplicates
- LexicallyDeclaredNames ∩ VarDeclaredNames must be empty

LDN includes BoundNames of: let, const, class, AsyncFunctionDeclaration, GeneratorDeclaration, AsyncGeneratorDeclaration, and (in strict mode only) plain FunctionDeclaration. VDN includes var declarations and (per Annex B B.3.2 in non-strict only) plain FunctionDeclarations.

Cluster surface: `test262/language/block-scope/syntax/redeclaration/` — 95 tests of the 8×8 declaration-kind cross-product.

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_block_statement` — call site after body parse.
- `pilots/rusty-js-parser/derived/src/stmt.rs::check_block_bound_names` — new helper that buckets each top-level decl into LDN or VDN per spec + Annex B, then enforces dup-in-LDN and LDN-intersect-VDN rules.

## Methodology

Single substrate rung. Walk parsed body once, partition by declaration kind:

- `Stmt::Variable` with kind Let/Const → LDN; Var → VDN.
- `Stmt::FunctionDecl { is_async, is_generator, .. }`:
  - `is_plain_function && !strict_mode` → VDN (Annex B B.3.2 carve-out)
  - otherwise → LDN
- `Stmt::ClassDecl` → LDN.

Reject on first duplicate-in-LDN or LDN/VDN-overlap.

## Carve-outs

- Module-tier (`script` body and `module` body) duplicate checks are at a different production (§16.1.1 / §16.2.1). Same shape, separate site; not done here.
- Function-body StatementList duplicate checks are at §15.1.1. Same shape, separate site.
- The check operates on top-level body only; nested blocks self-recurse via their own parse_block_statement call.

## Composes-with

- Today's parser arc (FHAPV/FORA/SBAP/FHLA/FAOF/ALTA/RPDF/ARTC) — sibling parser-tier closures.
- Parent locale `parser-early-error-residual` — this is the first rung against it.
- `apparatus/docs/predictive-ruleset.md` R14 (conservative-strip) applies: the strict-mode/Annex-B gate matters for not over-rejecting sloppy function-function.

## Resume protocol

Read `trajectory.md` tail.
