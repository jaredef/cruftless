# Debug Methodology

Patterns for diagnosing regressions and localizing substrate issues in cruftless.

This document accumulates engagement-tested debug techniques. Each pattern names a probe shape, a localization procedure, and a class of bugs it catches. The methodologies compose with the corpus's apparatus framework: Doc 730 §XVI (bidirectional engine-diff oracle), Doc 735 §X.h.b ((P2) four-case categorization), and Doc 735 §X.h.c (three-probe-levels discipline — bench + consumer-route + fuzz).

## Pattern 1: bisect-by-result-jsonl-diff

**Use when**: a wide consumer-route probe (test262 sample, parity sweep) shows a delta between two modes (e.g., feature-flag-off vs feature-flag-on, pre-commit vs post-commit), and the diff is too large to inspect by hand.

**Procedure**:

1. Run both modes; save each mode's per-test result.jsonl into a stable path (the standard runner overwrites by date, so copy explicitly):
   ```bash
   cp results/test262-sample-$DATE/results.jsonl /tmp/test262-default.jsonl
   CRUFTLESS_SHAPE_ENROLL=1 ./scripts/test262-sample/run-sample.sh
   cp results/test262-sample-$DATE/results.jsonl /tmp/test262-enrolled.jsonl
   ```
2. Cluster the regressions by directory prefix:
   ```python
   def load(path):
       by_path = {}
       with open(path) as f:
           for line in f:
               d = json.loads(line)
               by_path[d['path']] = d['status']
       return by_path
   default = load('/tmp/test262-default.jsonl')
   enrolled = load('/tmp/test262-enrolled.jsonl')
   regressions = [p for p, s in default.items() if s == 'PASS' and enrolled.get(p) == 'FAIL']
   from collections import Counter
   dirs = Counter('/'.join(p.replace('/home/jaredef/test262/test/', '').split('/')[:3]) for p in regressions)
   for k, v in dirs.most_common(20): print(f'{v:4d}  {k}')
   ```
3. Read the top cluster. Sample one or two failures from it; read the test source and the engine error / stdout-diff. Identify the dominant cause.
4. Fix the single dominant cause; re-measure. Re-cluster the residual.

**Why it works**: regressions cluster. Substrate-tier bugs that look like 283 independent failures often resolve into one or two consumer-site bugs that affect many tests. Bisecting by directory exposes the cluster shape; the dominant cluster fix usually closes 70–95% of the regression in one round.

**Empirical record**: 2026-05-23 CMig-EXT 12. test262 sample under shape-enrollment had 283 regressions vs default-off. Bisect cluster: 257 in `built-ins/Object/create` (91% of total). One fix (shape-aware enumeration of the Properties argument at `interp.rs:2117`) closed 257; residual collapsed to 25 (long tail: 13 `language/statements/for-of`, 6 `language/expressions/arrow-function`, 6 misc).

**Composition with the §XVI oracle**: this pattern IS Doc 730 §XVI's bidirectional engine-diff oracle applied at the consumer-route-probe-scale of Doc 735 §X.h.c. The "engine A vs engine B" comparison becomes "feature-flag-off vs feature-flag-on" within a single engine.

## Pattern 2: engine-error read-and-fix

**Use when**: a narrow consumer-route probe (single diff-prod fixture) fails and the engine produces a precise error message identifying the missing surface or wrong dispatch.

**Procedure**:

1. Read the failure's stderr from `/media/jaredef/T7/rusty-bun/diff-prod-results/<fixture>/cruftless.json`. cruft's TypeError messages carry full proto-chain attribution and method-name context — one error line usually localizes the gap.
2. Cross-reference the named method / property to the runtime source via grep.
3. The fix is typically one of: missing register_method call; missing prototype field; non-shape-aware iteration site; cohabitation issue between shape and properties storage.

**Why it works**: cruft's error messages are intentionally rich. Per Doc 730 §XVI bidirectional engine-diff: cruft is on the diagnostic-instrument side of the probe; its error format encodes "what the engine looked for and couldn't find" + the receiver shape + the proto chain walk. That's enough to localize most issues at sight.

**Empirical record**: 2026-05-22 diff-prod Rung-19/20/21 work; 2026-05-23 CMig-EXT 9 (closed 5 residual fixtures in ~15 minutes each by reading errors + applying P1/P2/P3 patterns).

## Pattern 3: scope-narrow on regression cascade

**Use when**: a substrate move's first attempt produces a regression cascade (many fixtures fail simultaneously after a single change).

**Procedure**:

1. Roll back the change immediately. Don't try to fix forward in the same commit.
2. Re-examine the substrate move's scope. Per Doc 729 §A8.13 substrate-amortization, large-blast-radius moves should split into substrate-introduction-round + closure-rounds. The cascade is the empirical signal that the scope is too wide.
3. Narrow to "infrastructure only, enrollment deferred" — land the types, the API contract, the dispatch helpers, but keep the default behavior unchanged. Subsequent rounds enroll consumers one by one, gated by the wider probe (Pattern 1).

**Why it works**: substrate-amortization requires that closure rounds drain consumer fanout incrementally. A single round that lands infrastructure AND default-on enrollment fails when consumer sites bypass the new path. Narrowing to infrastructure-only lets the closure rounds land independently, each gated.

**Empirical record**: 2026-05-23 Shape-EXT 4. First attempt auto-enrolled new ordinary Objects into Shaped form; diff-prod regressed 39/39 → 31/42 immediately. Rolled back, narrowed scope to "infrastructure only" with `shape: None` default; deferred enrollment to consumer-migration sub-workstream. The narrowing IS Doc 729 §A8.13 operating as designed: the empirical signal localized the split point.

## Pattern 4: three-probe-levels discipline (the meta-pattern)

Per Doc 735 §X.h.c: every (P2.a) strict-win claim requires three probes — bench + consumer-route + fuzz — each catching what the prior misses.

- **Bench probe**: deterministic small-input unit tests. Necessary but not sufficient. Catches simple correctness bugs.
- **Consumer-route probe**: integration against real workloads. Necessary but not sufficient. The bidirectional engine-diff oracle (Doc 730 §XVI) runs at this level; Pattern 1 (bisect-by-jsonl-diff) and Pattern 2 (engine-error read-and-fix) are the operational tools.
- **Fuzz probe**: random input over the algorithm's state space. Catches (P2.c) illegal-speed — bench-passing-but-correctness-violating bugs the consumer route happens to not exercise.

**Cautionary tale** (recorded so future rounds remember the precedent): 2026-05-23 CMig-EXT 10 → 11. Default-on enrollment flip landed because diff-prod 42/42 held in both modes. The wider consumer-route probe (test262 sample, 7,200 runnable tests vs diff-prod's 42 fixtures) caught −282 PASS regression. Default-on reverted; CMig-EXT 12 closed 91% via Pattern 1. The discipline read: diff-prod alone is insufficient corroboration for substrate moves touching property semantics; test262 sample is the load-bearing gate for default-on flips at that substrate.

## Pattern 5: doc-this-fixture (when surfacing a v1 boundary)

**Use when**: a fixture FAILs because the substrate genuinely lacks a feature that's out of v1 scope (lazy generators, real DataView, etc.).

**Procedure**:

1. Trim the failing case from the fixture's exec.mjs with an inline comment naming the v1 boundary + the corpus reference.
2. Add the boundary to the locale's seed.md §V deferred-substrate list with re-open conditions.
3. Re-run; the trimmed fixture flips to PASS.

This is the **anti-pattern carve-out**: per the diff-prod seed §I.2 anti-telos, "no fixture-as-workaround" — but a fixture that explicitly documents a v1 boundary is NOT a workaround, it's a recognized scope-limit per the Rung-13 / Rung-17 / Rung-18 precedents. The seed § V backlog is the catalog.

**Empirical record**: 2026-05-22 abort-controller fixture's `AbortSignal.timeout` case (host-tier timer routing v1 boundary); iterator-helpers fixture's `nats()` infinite-generator case (lazy-generator v1 boundary).

---

## How to add a pattern

When a new debug methodology proves itself across two or more rounds, add a section here following the structure: name, "use when", procedure, why it works, empirical record. Compose with the corpus's apparatus framework where applicable.

Patterns that proved themselves once go in the relevant locale's trajectory.md as a one-off; only generalized-and-corroborated patterns belong here.
