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

## RES-EXT 3 — replacer-callback named-args bridge (2026-05-25)

**Trigger**: RES.4 predicted the String.prototype.replace function-replacer was the same bridge-shape — engine has named-group info, substrate wasn't passing it to user callback per §22.1.3.18 step 11.a.iii.

**Edits** (~15 LOC at `regexp.rs::string_replace_impl`):
- Cache `rx.named_groups()` outside the match loop.
- After (match, ...captures, offset, input), push a final `groups` Object arg ONLY when named groups exist (passing undefined would change callback arity; spec says omit).
- Bug-pattern cleanup (RPTC.7): `abstract_ops::to_string(&r)` → `rt.coerce_to_string(&r)?` for replacer return-value coercion (replacer may return an Object with toString/@@toPrimitive).
- Bug-pattern cleanup (RPTC.7): same fix in `coerce_regexp` for the v→pattern coercion path.

**Verification**:
- Probe: `"2026-05-25".replace(/(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})/, function(m,y,mo,d,off,s,groups){return "Y="+groups.y})` → `Y=2026` ✓
- Probe: replacer with no named groups → 7th arg is `undefined` (omitted) ✓
- Probe: replacer returning `{toString(){return "Y"}}` → `"Y"` (was `[object Object]`) ✓
- test262 `built-ins/RegExp/named-groups/`: 10 → **13** (+3 from replacer-callback shape)
- test262 `built-ins/RegExp/match-indices/`: 11 → **11** (no regression)
- String.prototype.replace+match prev-fails (43 in sample): **16 newly pass** cumulatively across the RES arc + RPTC arc
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding RES.5 (bridge-audit triangle)**: three replicated bridge-rungs (RES-EXT 1 .groups, RES-EXT 2 .indices+.indices.groups, RES-EXT 3 replacer-callback groups arg) all surface the same engine output (named-group name→index map) into three different consumer surfaces. The map is computed once at compile, surfaced via one CompiledRegex method, and consumed three places. Pattern: cache the cheap computation once; surface broadly. Cumulative LOC across the three rungs: ~130; cumulative test-yield: 24 in cluster + 16 cumulative in collateral surfaces.

**Finding RES.6 (RPTC.7 bug pattern persists across review-passes)**: this rung found TWO more instances of `abstract_ops::to_string(&X)` in regexp.rs that survived the RPTC-EXT 4 sweep (replacer return-value, coerce_regexp pattern arg). Standing recommendation hardens: a periodic grep-sweep is necessary; ad-hoc per-rung audits miss sites. Candidate for tooling: a CI check that flags new `abstract_ops::to_string(&args...)` introductions.

**Status**: RES-EXT 3 CLOSED. Locale at 13/36 named-groups + 11/14 match-indices + replacer-callback support.
