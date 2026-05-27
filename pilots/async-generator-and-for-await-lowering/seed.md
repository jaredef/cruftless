# async-generator-and-for-await-lowering — Seed

## Telos

Materialize the language-lowering partition coordinate surfaced by LPA-EXT 5:

```
ast-to-bytecode/language-lowering ::
  E2/internal-method:execution-semantics ::
  async-generator-and-for-await ::
  conformance/residual-partition
```

This locale targets the coherent async iteration / async-generator subset of the 10,839-row `ast-to-bytecode/language-lowering` bucket in the latest full-suite interpretation. It covers three visible surfaces:

- `language/statements/for-await-of/`
- `language/expressions/async-generator/`
- `language/statements/async-generator/`

Current pool size from `test262-full-2026-05-26-140256-p2`: 1,492 visible records (for-await-of 646, async-generator expressions 568, async-generator statements 278).

## Apparatus

- **Partition source**: `pilots/apparatus/locale-positioning-audit/findings/language-lowering-partition.md`.
- **Repartition algorithm**: `pilots/apparatus/locale-positioning-audit/findings/repartition-audit-algorithm.md`.
- **Exemplar suite**: `exemplars/exemplars.txt`, 100 paths stratified across the three surfaces (43 / 38 / 19).
- **Exemplar runner**: `exemplars/run-exemplars.sh`, runs paths through the Test262 harness wrapper using `CRUFT_BIN` and `T262_ROOT` from `scripts/env.sh`.
- **Substrate sites to inspect after baseline**:
  - `pilots/rusty-js-parser/derived/`
  - `pilots/rusty-js-bytecode/derived/`
  - `pilots/rusty-js-runtime/derived/`

## Methodology

This is a Rule-23 baseline-first locale. The founding move is not a substrate edit. The first task is to prove which resolver layer dominates the exemplar failures:

1. Run the 100-path exemplar suite.
2. Partition failures into:
   - parser early error / syntax acceptance,
   - async test harness behavior,
   - async-generator object protocol,
   - AsyncFromSync iterator wrapping,
   - Promise/job queue dependency,
   - abrupt completion propagation,
   - destructuring / iterator-close interaction.
3. If Promise/job queue dominates, redirect to a runtime E4 job-queue locale.
4. If parser early errors dominate, split a parser sub-locale rather than forcing lowering work.
5. If async-generator object protocol or for-await lowering dominates, proceed to the first substrate rung here.

## Carve-outs

- This locale does not own general Promise reaction ordering unless baseline proves it is inseparable from the async-generator/for-await failure shape.
- This locale does not own all generator semantics; synchronous generator residuals remain in their existing parser/runtime lanes.
- This locale does not own class async-generator methods unless residual inspection shows they reduce to the same runtime protocol; class-specific rows remain under class/private-field arcs.

## Composes-with

- `pilots/apparatus/locale-positioning-audit/` LPA-EXT 5 and LPA-EXT 6.
- `pilots/for-of-async-lookahead/` for parser lookahead adjacency.
- `pilots/iter-protocol-bytecode-rewrite/` as a performance-tier sibling, not a conformance owner.
- Runtime Promise/job-queue surfaces in the full-suite matrix.

## Resume protocol

Read `trajectory.md` tail, then run:

```
pilots/async-generator-and-for-await-lowering/exemplars/run-exemplars.sh
```

Use the exemplar fail partition to select the first substrate rung or redirect.

