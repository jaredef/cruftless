# regex-engine-substrate — Seed

## Telos

Treat the regex engine as a first-class substrate: the surface methods (test/exec/match/split/replace/search) all route through `regexp_exec`, which delegates match-detection to `CompiledRegex` (dual-backend: Rust `regex` crate + hand-rolled NFA). Engine gaps are now the dominant source of regex-related failure after the spec-filter arc (RPTC-EXT 1-4) closed the surface side.

Reconnaissance (post-RPTC-EXT 4, 2026-05-25): of 27 prev-failing RegExp-pipeline tests in the sample, 21 now pass; 6 residuals remain, all genuinely engine-level:
1. `exec/S15.10.6.2_A1_T6` — `(b+)?` capture-reset across `*` iterations (spec §22.2.2.2: groups reset per repeat iteration).
2. `exec/u-captured-value` — `u` flag capture-position alignment.
3. `exec/u-lastindex-adv` — `/\udf06/u` lone-surrogate.
4. `exec/regexp-builtin-exec-v-u-flag` — `v` flag set-notation features.
5. `exec/duplicate-named-groups-properties` — ES2025 duplicate named groups in alternation.
6. `exec/duplicate-named-indices-groups-properties` — same + `d` flag indices.

Plus: `.groups` property on match results is currently `undefined` — never built, even though both engines parse named groups. This is a substrate-bridge gap, not an engine gap.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/regexp.rs` — `regexp_exec`, `compile_either`, `translate`.
- `pilots/rusty-js-runtime/derived/src/regex_hand.rs` — hand-rolled backtracker, AST, parser, matcher.
- `pilots/rusty-js-runtime/derived/src/value.rs` — `CompiledRegex` enum + dual-backend dispatch.

## Methodology

Per Standing Rule 21 (probe-first scoping): not all engine gaps are equal-cost. Decompose into rungs:

- **RES-EXT 1 (this rung)**: named-group bridge. Both backends already parse `(?<name>...)`; substrate just needs to expose name→index map at `CompiledRegex`, then build a `.groups` Object on exec results. ~50 LOC, unlocks the `.groups`-reading test262 surface (estimated 10+ tests; plus all real-package usage).
- **RES-EXT 2 (candidate)**: capture-reset semantics in hand-engine `match_repeat`. ~30 LOC + careful testing. Closes A1_T6 + similar.
- **RES-EXT 3+ (deferred)**: u-flag UTF-16 corner cases, v-flag set notation, duplicate-named-groups alternation — each its own sub-locale.

## Composes-with

- Doc 729 (resolver-instance pattern: regex engine is the matching resolver, regexp_exec is the spec layer).
- Doc 730 (vertical recurrence: matched substrate spans both engine + spec layers).
- Standing Rule 13 (revert-then-deeper-layer-closure): the shared closure beneath the surface methods is the canonical site for spec-invariant enforcement.
- Standing Rule 21 (probe-first scoping): each rung sized to bounded LOC + bounded test-yield estimate.

## Resume protocol

Read `trajectory.md` tail.
