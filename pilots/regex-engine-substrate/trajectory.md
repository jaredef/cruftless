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

## RES-EXT 2 — match-indices bridge (/d flag) (2026-05-25)

**Trigger**: RES.2 predicted /d match-indices was the same shape as RES.1 (substrate-bridge gap, not engine gap) — engine already returns capture positions, substrate just needs to surface them.

**Edits** (~70 LOC):
- `value.rs::CompiledRegex::captures_positions_at()`: parallel to `captures_at`, returns `Vec<Option<(byte_start, byte_end)>>` instead of `Vec<Option<String>>`. Both backends already compute positions; the Rust backend was throwing them away in the existing path.
- `regexp.rs::regexp_exec`: hoist `has_indices = re.flags.contains('d')`. When set, re-acquire positions from the engine (separate call to keep the surrounding flow stable), build `.indices` Array per §22.2.7.7 MakeMatchIndicesArray: each element `[start_u16, end_u16]` (or undefined for non-participating). When named groups exist, attach `.indices.groups` mirror with name → [start, end] pairs. Positions go through `byte_to_utf16` per RPTC-EXT 3 convention.

**Verification**:
- Probe: `/a/d.exec("a").indices` → `[[0,1]]`, `Array.isArray` true ✓
- Probe: `/(?<year>\d{4})-(?<month>\d{2})/d.exec("2026-05")` → `.indices = [[0,7],[0,4],[5,7]]`, `.indices.groups = {year:[0,4], month:[5,7]}` ✓
- Probe: `/a/.exec("a").indices` → `undefined` (no /d flag) ✓
- test262 `built-ins/RegExp/match-indices/`: **0 → 11/14**
- test262 `built-ins/RegExp/named-groups/`: 10 → **10** (no regression)
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding RES.3 (RES.1 prediction confirmed)**: the bridge-gap shape replicates. RES-EXT 1 closed 10/36 named-groups by surfacing already-computed engine output; RES-EXT 2 closes 11/14 match-indices by the same pattern. Cumulative bridge-only LOC: ~115 across both rungs; cumulative test-yield: 21 newly passing. Engine internals untouched.

**Predicts**: continuing to audit "what does the engine already compute that the substrate isn't surfacing?" is likely higher-yield-per-LOC than extending the engine itself. Standing recommendation: at locale-entry, do a bridge-audit before any engine work.

**Finding RES.4 (residual decomposition for next rung)**: post-RES-EXT 2:
- 3 residual match-indices tests likely depend on duplicate-named-groups (alternation arms each get an indices entry, ES2025).
- The replacer-callback named-args bridge — String.prototype.replace with a function replacer must pass `.groups` as the last positional arg when the regex has named groups. Same bridge-shape as RES.1/RES.2.

**Status**: RES-EXT 2 CLOSED. Locale at 11/14 match-indices + 10/36 named-groups + collateral.
