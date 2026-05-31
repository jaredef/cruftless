# array-from-interleave-discipline (AFID) — Seed

## Telos

Materialize the engine-DAG coordinate

```
runtime/array-from :: E2/internal-method:abstract-op :: interleave-iter-and-map :: IteratorClose-on-abrupt-mapfn
```

at the Array.from intrinsic. ECMA-262 §23.1.2.1 step 6 mandates that Array.from iterate the iterable AND apply mapfn to each value AS THEY ARE PRODUCED, not by collecting all values first and then mapping. On any abrupt completion during the per-element loop (next() throw, non-Object result, mapfn throw, CreateDataPropertyOrThrow failure), IteratorClose is invoked with the abrupt completion as `completion`; per §7.4.9 step 4, if `completion.[[Type]] is throw`, IteratorClose returns the ORIGINAL completion even when GetMethod/Call of `iter.return` themselves throw (the original abrupt-completion-throw shadows any close-thrown).

## Origin

Founded 2026-05-31 per keeper Telegram 10684 ("Continue"). Surfaced during the ICES chapter-close residual audit: probing whether Array.from with a mapfn that throws on the first element closes the iterator surfaced a hard-OOM under `ulimit -v 1 GiB`. Diagnosis: `array_from_via` (interp.rs:6589) calls `collect_iterable` to FULLY drain the iterable into a `Vec<Value>` BEFORE applying mapfn — so on an infinite iter + throwing mapfn, the collect loop OOMs before mapfn ever runs.

Distinct from ICES: ICES closes the for-of compiler-tier emission paths (break / return / throw inside the loop body lowering). AFID closes the runtime-tier intrinsic shape (Array.from's own iteration loop, which is hand-written in Rust, not bytecode-emitted).

## Work shape

**Heuristics §IV classification**: D (Runtime abstract-op semantics) at the Array.from surface.

Two structural changes at one intrinsic site:

1. **Interleave**: rewrite `array_from_via`'s iterable branch as a per-element loop that calls iter.next(), checks result-Object + done + value, calls mapfn(value, k) if mapping, writes the result into the out array, and loops — instead of fully collecting then mapping. Matches §23.1.2.1 step 6.
2. **Close-on-abrupt**: if mapfn throws (or any per-element step abrupt-completes), call IteratorClose(iter) before propagating the throw. The throw is preserved — close-thrown errors do NOT shadow the original per §7.4.9 step 4.

## Apparatus

- **Direct probe**: `/tmp/probe-afid-0.js` (8-cell assertion suite). 8/8 PASS post-rung.
- **Runtime helper**: new `iter_close_rt(rt, iter_id) -> Result<(), RuntimeError>` in intrinsics.rs (mirrors `__destr_iter_close` for Rust callers). Returns silently for null/undefined .return; throws TypeError for non-callable non-null/undefined .return; calls + checks Object result otherwise.

## Methodology

Per Doc 744 four-tuple + Rule 17 segmentation: scoped to Array.from. Sibling Doc-721 abrupt-iteration sub-bundles (Array spread `[...iter]` close on throw, spread-call close on throw, Map/Set constructors with iterable, Promise.all/Promise.allSettled iteration close) remain separate substrate-spawns at their own intrinsics.

## Carve-outs

- **Bytecode-tier consumers**: out of scope. Array.from is invoked from JS as a method call; the bytecode-tier handles the call but not the body — Array.from's body is hand-written Rust.
- **Async-iter Array.fromAsync**: out of scope; sibling intrinsic surface (would need its own locale if/when implemented).
- **CreateDataPropertyOrThrow strictness**: the current impl uses `object_set` which is permissive. Strict CreateDataPropertyOrThrow (which DefinePropertyOrThrow with own-data-property descriptor) is a separable sub-locale.

## Composes-with

- `pilots/iterator-protocol-throw-discipline/` (IPTD-EXT 1: helper-tier `__destr_iter_close` discipline) — AFID's `iter_close_rt` mirrors the same semantics at the Rust-caller surface.
- `pilots/iterator-close-emission-sites/` (ICES-EXT 1/2/3: compiler-tier IteratorClose emission for for-of break/return/throw) — sibling locale at the bytecode tier.
- `apparatus/docs/predictive-ruleset.md` Rule 15 (chapter-close-inspect surfaced this gap during ICES residual probe), Rule 17 (segmentation: Array.from-only scope, not the broader cross-intrinsic spread family).
- `apparatus/docs/findings-ledger.md` for Finding AFID.1 surfaced (see trajectory).

## Resume protocol

Read `seed.md` then `trajectory.md` tail. AFID-EXT 0 is the founding rung (interleave + close-on-throw + iter_close_rt helper). Future rungs at this locale would address:
- CreateDataPropertyOrThrow strictness (if surfaced by failing tests)
- Array.fromAsync if Array.fromAsync intrinsic lands
- Other Array.from-specific spec compliance gaps surfaced by test262 sweep
