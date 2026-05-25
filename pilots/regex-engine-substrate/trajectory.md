# regex-engine-substrate — Trajectory

## RES-EXT 1 — named-group bridge (2026-05-25)

**Trigger**: keeper "let's zoom out and work on a regex substrate locale itself" after RPTC-EXT 4. Reconnaissance: both backends already parse `(?<name>...)` but `.groups` is never built on exec results — a substrate-bridge gap, not an engine gap.

**Edits** (~45 LOC):
- `regex_hand.rs::HandRolledRegex`: add `pub named_groups: HashMap<String, usize>`; populate in `compile()` from the parser.
- `value.rs::CompiledRegex::named_groups()`: unified accessor. Rust backend uses `regex::Regex::capture_names()` (enumerate + filter_map); hand backend reads its own HashMap.
- `regexp.rs::regexp_exec`: after building the result Array, query `compiled.named_groups()`. If non-empty, build a null-prototype Object with name → captured-string-or-undefined, set as `.groups`. Else set `.groups = undefined` (spec §22.2.7.2 step 33 — `.groups` is `undefined` when the pattern has no named groups).

**Verification**:
- Probe: `/(?<year>\d{4})-(?<month>\d{2})/.exec("2026-05")` → groups `{year:"2026", month:"05"}` ✓
- Probe: `/(\d+)/.exec("1").groups` → `undefined` ✓
- Probe: `/(?<a>x)|(?<b>y)/.exec("y")` → `groups = {a:undefined, b:"y"}` ✓ (non-participating named group)
- test262 `built-ins/RegExp/named-groups/`: **0 → 10/36** (across non-duplicate cases)
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding RES.1 (substrate-bridge gap surfaces as engine gap)**: pre-fix, named-group test failures looked like engine inadequacies (`.groups` not built). Both backends already had the parsing + capture infrastructure; the gap was at the SUBSTRATE layer (regexp_exec wasn't asking the engine for its named-group map, and wasn't building the .groups Object). 45 LOC at the bridge closed 10 tests across multiple surfaces (.exec/.match/.matchAll/.replace named-group dispatch). The lesson: before extending the engine, audit whether existing engine output is fully surfaced at the substrate.

**Predicts**: similar bridge-gaps may exist for other engine outputs that the substrate doesn't currently expose — match-indices arrays (/d flag), capture spans, named-index map for /d. Each is a candidate for a similarly cheap bridge-rung before reaching into engine internals.

**Finding RES.2 (residual decomposition for next rungs)**: post-RES-EXT 1, the 26 still-failing named-groups tests split into:
- Duplicate-named-groups in alternation (ES2025): needs both engines to disambiguate same-name-different-slot; Rust regex crate may reject; hand engine needs alternation-aware name resolution. RES-EXT 4 candidate.
- Named groups + /d flag (match-indices array): bridge-gap of the same shape as RES-EXT 1 (engine has positions; substrate doesn't surface them). RES-EXT 2 candidate.
- Named groups + Unicode-property: engine-level (requires `\p{...}` parsing in hand engine).
- Replace/replaceAll callbacks with named-groups arg: substrate-level (need to pass .groups into replacer callback as the last arg).

**Status**: RES-EXT 1 CLOSED. Locale established with reconnaissance + decomposition for next rungs.
