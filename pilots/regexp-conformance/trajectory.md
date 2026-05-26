# regexp-conformance — Trajectory

## RC-EXT 0 — founding + parent-scope decision (2026-05-26)

**Trigger**: Keeper directive "spawn spawn regexp-conformance" after investigation of the deferred `regex-literal-lexing` tokenization candidate.

**Apparatus established**:

- `seed.md` — parent locale for the regexp conformance cluster.
- `docs/` — future design notes.
- `fixtures/` — future direct probes.
- `exemplars/` — future stratified test262 exemplar list.

**Matrix anchor**:

LPA-EXT 3 Class C: **491 fails** across two regexp coordinates:

- rank 19: 262 rows, `runtime/regexp :: value-semantics/wrong-result :: SyntaxError`
- rank 23: 229 rows, `runtime/regexp :: regexp-semantics :: failure/other`

**Founding decision**:

`regex-literal-lexing` is not spawned as a top-level sibling. It is recorded as a nested candidate under this locale. Reason: current evidence shows the major regexp surface is runtime/static-semantics conformance, while raw literal lexing is a smaller lex/goal-symbol slice. Per Doc 737 and apparatus-tax discipline, the parent regexp coordinate should absorb the source-text rung if RC-EXT 0 proves it coherent.

**Current local source read**:

- `lexer.rs::read_regex_literal` already handles body accumulation, character classes, escapes, flags, and line-terminator rejection.
- `spec_golden.rs` already covers simple regex, character class slash, escapes, and unterminated regex.
- `parser.rs::derive_lex_goal_after` owns div-vs-regexp goal selection.

**Next move**:

RC-EXT 0 baseline-inspection: partition the regexp rows into lexer, parser-goal, regexp static-semantics, runtime matcher, and prototype/String integration classes. The first substrate move should be chosen from that partition, not from the top-level matrix label alone.

**Status**: FOUNDED. No substrate code yet.

## RC-EXT 1 — String split regex captures bridge (2026-05-26)

**Trigger**: RES Audit-2 left `String.prototype.split` with a regex separator dropping capture groups, despite the regex engine already exposing capture vectors.

**Move**:

- Replaced the `RegExp.prototype[@@split]` path's capture-dropping `split_str` call with a local split bridge that consumes `CompiledRegex::captures_at`.
- Added capture insertion into the result array after each separator match, including `undefined` for unmatched groups.
- Preserved the split limit as an output-element cap, so captured substrings count toward the limit.
- Added a string-index advance helper to avoid zero-width separator loops while staying byte-safe for UTF-8 boundaries.

**Evidence**:

- `cargo test -p rusty-js-runtime t13 -- --nocapture`
  - `t13_string_split_regex ... ok`
  - `t13b_string_split_regex_includes_captures ... ok`
  - `t13c_string_split_regex_capture_limit_counts_captures ... ok`

**Status**: CLOSED. First regexp-conformance substrate bridge landed in the runtime.

## RC-EXT 2 — RegExp.prototype.compile rebind bridge (2026-05-26)

**Trigger**: Parent-locale coherence pass after RC-EXT 1. The seed names Annex B `RegExp.prototype.compile` as part of the regexp surface, and source inspection showed the method was only a presence/no-op stub.

**Move**:

- Replaced the no-op `compile` method with a receiver-mutating Annex B path.
- Added `regexp_compile_args` to derive pattern/flags with the RegExp-input carve-out: copy source/flags when flags are undefined; throw TypeError when a RegExp pattern is paired with explicit flags.
- Rebound receiver internals (`source`, `flags`, compiled matcher) and reset `lastIndex` to 0.

**Evidence**:

- `cargo test -p rusty-js-runtime t14 -- --nocapture`
  - `t14_constructor_form ... ok`
  - `t14b_compile_rebinds_receiver ... ok`
  - `t14c_compile_copies_regexp_pattern ... ok`
  - `t14d_compile_regexp_with_flags_throws ... ok`

**Finding RC.1 (presence stubs are conformance debt, not harmless scaffolding)**: the compile method already existed, so module-init probes passed, but the object semantics were absent. In the regexp parent locale, method presence is only a weak signal; the next coherent probe should check whether a method mutates the same internal state that `.source`, `.flags`, matcher dispatch, and `lastIndex` expose.

**Status**: CLOSED. Annex B compile now participates in the same RegExp internal-state substrate as constructor-created and literal-created regexps.
