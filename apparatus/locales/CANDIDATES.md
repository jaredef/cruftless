# Locale candidates — next-spawn queue

Living document; append-only at the bottom (newest at top within a section). Each candidate is a prospective Pin-Art locale not yet founded as `pilots/<name>/seed.md`. Spawning protocol: per standing rule 11, do a 5-axis pre-spawn check (component A/B + op-set + value-domain + locals-marshaling + emission-shape) before founding. Per standing rule 13, design from the deeper-layer first; do not pay the cache-tier substrate-introduction tax if the closure tier is known.

**Status legend**: `🟢 RIPE` (rule 11 satisfied, ready to spawn) · `🟡 PROBED` (component A/B in progress) · `⚪ HYPOTHETICAL` (not yet probed).

---

## Tier A — empirically-anchored single-pilot yield targets

### (a) `iter-protocol-bytecode-rewrite` — **SPAWNED** 2026-05-24 as [`pilots/iter-protocol-bytecode-rewrite/`](../../pilots/iter-protocol-bytecode-rewrite/seed.md)
**Telos**: close the for-of protocol envelope dispatch in hot loops. Per GPI-EXT 3 cost analysis, after IHI+GPI's hot-method-call closure (~260 → ~15 ns/iter), the per-iter dominator on `string_url_sweep` header_loop is the IterInit/IterNext/IterClose dispatch + the synthetic iterator-result object allocation per `.next()`.
**Anchor**: `string_url_sweep` header_loop A/B probe at ~252-260 ms (post-GPI). Per the cost model, IterInit/IterNext is ≥50% of remaining per-iter cost.
**Deeper-layer design (rule 13 prospective)**: bytecode-rewrite the `IterInit; LabelTop; IterNext; JumpIfDone; ...body...; Jump LabelTop; IterClose` pattern into a single `IterFastLoop(local_idx)` for Array/String receivers where the iterator is the well-known intrinsic (not user-overridden). Eliminates per-`.next()` synthetic object allocation; index-based scan over the underlying String bytes / Array elements.
**LOC estimate**: ~60-100 (the rewrite pattern detection is non-trivial; consider a discovery-pass over FunctionProto on first invocation rather than per-dispatch).
**Cross-tier dual**: would also benefit OSR-eligible loops; consider promoting to JIT tier after interp-tier proves.

### (b) `jit-getprop-method-ic` — 🟡 PROBED
**Telos**: extend Σ stub-emitter's existing String-receiver property-get path to handle method-resolve composition with HI's CallMethodIcCached. Currently Σ handles standalone `s.length` and `s[i]`-style access; method dispatch falls through to a generic call path even when HI would fast-path the call.
**Anchor**: needs JIT-eligible fixture A/B probe; string_url_sweep's hot loop is interp-bound, so JIT GPI requires a different empirical anchor (json_parse_transform's `.charCodeAt` is OSR-eligible per HI close).
**Deeper-layer design**: stub emit `GetProp(method-name) + IcCached(idx)` as a fused JIT instruction sequence that directly invokes HI's IcEntry.fast without round-tripping through the interp dispatcher.
**LOC estimate**: ~80-120 (Cranelift stub assembly + IcEntry handoff).
**Risk**: composition with existing OSR boundary; the JIT-→interp re-entry on bail must handle the rewritten bytecode.

### (c) `ihi-array-entries` — 🟢 RIPE
**Telos**: extend IHI_TABLE with Array.prototype intrinsic entries: push/pop/shift/unshift, forEach (with callback), indexOf, includes, slice, concat. Pattern mirrors existing String entries.
**Anchor**: `json_parse_transform` is Array-method-heavy; pre-GPI median 1773 ms. Per-entry LOC budget per IHI's existing pattern: ~30-50 LOC each.
**Deeper-layer design (rule 13 prospective)**: skip cache-tier substrate work; reuse the existing CallMethodIcCached + GetPropSkipForMethod bytecode-rewrite infrastructure. Each Array entry costs the same shape as a String entry.
**Composition risk**: receiver_kind dispatch must distinguish Object-receivers-that-are-Arrays from generic Objects; the IhiReceiverKind::Array gate already exists per interp_ic_table.rs.
**Predicted yield**: json_parse_transform 1773 → ≤1600 ms (-10%); cruft/node 14.78x → ≤13.3x.

### (d) `gpi-override-safety` — ⚪ HYPOTHETICAL
**Telos**: harden Op::GetPropSkipForMethod against user-installed own-property override of an intrinsic key on the receiver type. First-cut GPI assumes frozen-prototype semantics; a user adding `String.prototype.toLowerCase = function() { ... }` after GPI rewrite would not invalidate the cache.
**Anchor**: a synthetic correctness fixture, not a perf surface. Spawn only if a real-world consumer-app surfaces the divergence.
**Deeper-layer design**: per-IhiEntry override-version counter at Runtime; bump on String.prototype mutation; check at GetPropSkipForMethod dispatch.
**Cost**: adds a per-dispatch counter compare; may regress GPI's reclaim.
**Disposition**: deferred until consumer-app shows the need.

---

## Tier B — broader surface targets (rule 11 component A/B still pending)

### (e) `arith-tight-loop-closure` — ⚪ HYPOTHETICAL
**Telos**: arith_tight_loop @ 422 ms / cruft/node 2.10x is the best ratio on CRB. The remaining gap to node is a candidate for closure via I64 typed-op promotion in interp (analogous to JIT's AddI64/MulI64 closure).
**Anchor**: needs component A/B probe to identify the dominator (typed-op dispatch vs. f64 boxing per iter).
**Deeper-layer design**: TL's existing I64 unbox at JIT extends to interp by treating loops with all-I64-typed locals as I64-direct.

### (f) `module-loader-eager-cache` — ⚪ HYPOTHETICAL
**Telos**: ESM import resolution cost on cold-start. Surfaces in consumer-app tests, not CRB.
**Anchor**: needs a consumer-app cold-start timing instrument; CRB fixtures are single-module so no anchor.

### (g) `regex-jit-precompile` — ⚪ HYPOTHETICAL
**Telos**: regex compile-on-first-test → precompile-at-LoadConst. version_regex + id_regex deltas (14, 20 ms per string_url_sweep A/B) are small but compound across modules.
**Anchor**: A/B probe variants V3, V4 (already exist in component-ab-probe.mjs).
**Deeper-layer design**: precompile regex literals at FunctionProto load time (constant pool); cache the compiled matcher.

---

## Tier D — strategic / language-tier (new 2026-05-24)

### (l) `ts-consumer-corpus` — **SPAWNED** 2026-05-24 as [`pilots/apparatus/ts-consumer-corpus/`](../../pilots/apparatus/ts-consumer-corpus/seed.md)
**Telos**: empirical measurement instrument for TSR's coverage of real consumer `.ts` source on npm. Failure-table drives priority order for the downstream TSR sub-locale arc (enums, classes, generics-calls, decorators, namespaces, conditional-types, JSX).
**Status**: SPAWNED. TCC-EXT 1 (corpus assembly) is the next round.

### (m-s) `ts-resolve-*` sub-locales — ⚪ QUEUED PENDING TCC FAILURE-TABLE
- `ts-resolve-enums/` — runtime-bearing; enum reverse-mapping + lowering
- `ts-resolve-classes/` — ctor-param shorthand, abstract, accessor modifiers
- `ts-resolve-generics-calls/` — f<T>() angle-bracket disambig vs `<` operator
- `ts-resolve-decorators/` — Stage 3 decorators; runtime descriptors
- `ts-resolve-namespaces/` — legacy but persistent in tooling code
- `ts-resolve-conditional/` — cond + mapped + template-literal types (strip-only; combined)
- `ts-resolve-jsx/` — separate locale; JSX/TSX
**Disposition**: priority order will be set by TCC-EXT 2's failure-table. Each sub-locale targets ≤3 implementation rounds per standing-rule-13 thesis.



### (j) `ts-resolve` — **SPAWNED** 2026-05-24 as [`pilots/ts-resolve/`](../../pilots/ts-resolve/seed.md)
**Telos**: native `.ts` execution by cruft via a TS source-language resolver upstream of rusty-js-ir. Empirical-first stage of a two-locale arc with `cruftscript-spec/`. Load-bearing research question: do erased TS annotations carry substrate-actionable signal for downstream IC/JIT/VD tiers?
**Status**: SPAWNED. TSR-EXT 1 (design doc) is the next round.

### (k) `cruftscript-spec` — ⚪ DEFERRED; TSR-EXT 5 PROBE RETURNED NULL AT IPBR CONSUMER 2026-05-24
**Telos**: design and specify CruftScript — a sound statically-typed sibling language to TS, following the Typed Racket model (typed code internally sound; runtime contracts at typed/untyped FFI boundary). The key architectural lever: types as first-class substrate input (drives JIT IC specialization, IHI/GPI/IPBR shape probes, VD's NaN-boxed tag schema) rather than erased upstream as tsc and Typed Racket do.
**Disposition**: deferred until TSR-EXT 5's annotation-sidecar probe data lands. Positive signal → cruftscript-spec founded on grounded substrate claims. Null signal → cruftscript-spec proceeds on soundness-alone grounds (still valuable but smaller corpus claim).
**Anchor**: TSR's empirical data on annotation-as-substrate-hint will inform the language design's scope + grammar bounds.
**2026-05-24 UPDATE per TSR-EXT 5 Finding TSR.1**: probe returned NULL at the IPBR consumer (per-iter shape-lookup cost too small for annotation-driven elimination to surface above noise). Load-bearing claim for cruftscript-spec shifts from "iter-protocol-shape-skip substrate" to **"JIT IC specialization on typed function args + VD NaN-box tag preservation through typed numerics"**. Each follow-on consumer needs its own empirical probe before substrate-leverage claim can be made for that consumer. Locale remains worth spawning, but on weaker (still valuable) grounds.

---

## Tier C — discipline / methodology / corpus work

### (h) `standing-rules-codification-pass` — **FORMALIZED** 2026-05-24 as [`apparatus/docs/standing-rule-13-prospective-application.md`](../../apparatus/docs/standing-rule-13-prospective-application.md)
**Telos**: review findings.md (20 findings; 13 standing rules) for consolidation. Standing rule 13's prospective application across IHI → GPI is a candidate Doc 7xx corpus publication (multi-tier-cascade-revival applied PROSPECTIVELY, not retrospectively).
**Output**: 1 corpus doc (~150-200 lines); refresh findings.md Addendum X.
**Status**: working draft landed in `docs/`; candidate for promotion to corpus Doc 742 after one additional empirical corroboration (e.g., `ihi-array-entries`) or keeper review of thesis at current anchor.

### (i) `crypto-sha256-batch-investigation` — ⚪ HYPOTHETICAL
**Telos**: CRB crypto_sha256_batch FAIL is pre-existing (CRB-EXT 0-6 baseline bb212c3c). Investigate root cause; could be missing host stub or `Buffer` API gap.
**Anchor**: stderr from `cruft pilots/apparatus/cross-runtime-bench/fixtures/crypto_sha256_batch/main.mjs`.

---

## Tier E — coordinate-driven (matrix-surfaced, post-2026-05-25 full-suite run)

Candidates surfaced by the LPA-EXT 3 positioning-gap audit (`pilots/apparatus/locale-positioning-audit/findings/positioning-gaps.md`) + PCR-EXT 1+2 new top-rank coordinates. Each is anchored to a specific Pin-Art coordinate with a named fail-count from the full-suite matrix.

### (t) `intl402-availability` — 🟢 RIPE
**Telos**: Implement-Chapter mirroring `temporal-availability/`'s shape. Covers ECMA-402 (Intl namespace) absent-chapter coordinates. Three matrix coordinates roll up to 2,613 fails: missing-global-binding (rank 2 / 2,008), value-semantics/wrong-result (rank 14 / 382), missing-method (rank 25 / 223).
**Anchor**: full-suite matrix ranks 2, 14, 25; sibling subsystem to Temporal.
**Methodology**: stratified exemplar suite (100 fixtures by Intl class — Collator, DateTimeFormat, NumberFormat, etc.); subsystem registration MVP; class-by-class implementation.
**Status**: queued; second of LPA-EXT 3 recommendations.

### (u) `regexp-conformance` — **SPAWNED** 2026-05-26 as [`pilots/regexp-conformance/`](../../pilots/regexp-conformance/seed.md)
**Status**: FOUNDED. Parent locale for the 491-fail regexp matrix cluster; `regex-literal-lexing` is recorded as a possible nested rung pending RC-EXT 0 baseline-inspection.

### (v) `cruft-parser-feature-gaps` — 🟢 RIPE
**Telos**: substrate work on cruft's parser to close the unimplemented-syntax cases surfaced by PCR-EXT 1's new `availability/missing-parser-feature` coordinate (~471 fails @ post-PCR rank 11). Per the categorization, reasons of shape `parse: ...` are cruft's parser refusing test source — typically TypeScript generics, decorators, other parser-feature gaps in JavaScript tests that exercise parser edge cases.
**Anchor**: PCR-EXT 1+2 categorizer surfaced the coordinate; sample reasons include `parse: unexpected token in expression: Punct(Gt)` (generic-arg disambig), `parse: expected binding identifier or pattern` (destructure shapes).
**Methodology**: pull 10 sample reasons; cluster by parse-tier mechanism; close per-mechanism rungs.
**Status**: queued, post-PCR; not yet rule-11-checked.

### (w) `cruft-lowering-feature-gaps` — 🟢 RIPE
**Telos**: substrate work on cruft's bytecode compiler to close unimplemented-feature rejections surfaced by PCR-EXT 2's new `availability/missing-lowering-feature` coordinate (~115 fails). Sibling to (v) at the lowering tier; reasons of shape `compile: complex assignment target not yet supported` etc.
**Anchor**: PCR-EXT 2 categorizer surfaced the coordinate.
**Methodology**: enumerate `compile: ...` reasons; cluster by lowering-tier mechanism; close per-mechanism rungs.
**Status**: queued, post-PCR.

### (x) `annexB-runtime-quirks` — 🟢 RIPE
**Telos**: implement Annex B legacy intrinsics surfaced by PCR-EXT 1's annexB-resolver routing (~398 fails @ post-PCR rank 12 + ~202 fails in String.prototype html-methods cluster). Annex B covers Date.prototype.{getYear/setYear/toGMTString/toUTCString}, String.prototype.{anchor/big/blink/bold/etc.}, RegExp.prototype.compile, and the global escape/unescape.
**Anchor**: PCR-EXT 1's annexB/built-ins/* resolver routing made these legible as a class.
**Methodology**: surface-by-surface implementation; many are small per-method.
**Status**: queued, post-PCR.

### (y) Class-D scope extensions (LPA-EXT 3 recommendation 4) — ⚪ HYPOTHETICAL
**Telos**: extend existing top-10-batch locales to absorb sibling matrix coordinates at next chapter close. NOT new locale spawns; rung extensions to existing locales per R4-discipline (cluster-coherence multiplier's apparatus-tax non-amortization avoidance).
**Anchor**: LPA-EXT 3's Class D enumeration:
  - `ast-bytecode-missing-throw-typeerror/` extends rank 15 (Test262Error, +350) + rank 29 (ReferenceError, +178)
  - `ast-bytecode-missing-method/` extends rank 28 (missing-global, +180)
  - `typed-array-wrong-result/` extends rank 21 (TypeError throw-missing, +239)
**Methodology**: chapter-close-inspect (R15) the existing locales' current scope; extend to siblings.
**Status**: pending the existing locales' next substrate-rung activity.

### (z) `pinart-categorizer-refinement` — **SPAWNED** 2026-05-25 as [`pilots/apparatus/pinart-categorizer-refinement/`](../../pilots/apparatus/pinart-categorizer-refinement/seed.md)
**Telos**: refine `full_pinart.rs` categorizer rules to extract specific projection classes from `uncategorized/*` records. PCR-EXT 1+2 reduced uncategorized records 3,681 → 378 (-89.7%).
**Status**: CLOSED at EXT 2; PCR-EXT 3 (canonical re-categorize against next test262 raw run) deferred per keeper.

---

## Tier F — FCA-amortization spinoff chain (today's substrate work, post-spawn)

These were spawned mid-session as the spinoff chain emerged (per `pilots/apparatus/locale-positioning-audit/findings/spinoff-chains.md` Chain 1). Recorded here for audit-trail completeness; not new candidates.

### (aa) `lexer-goal-symbol-selection` (LGSS) — **SPAWNED + CLOSED** 2026-05-25 at [`pilots/lexer-goal-symbol-selection/`](../../pilots/lexer-goal-symbol-selection/seed.md)
3-rung closure. Lexer-tier; surfaced PPIF as spinoff.

### (bb) `parser-precedence-in-flag` (PPIF) — **SPAWNED** 2026-05-25 at [`pilots/parser-precedence-in-flag/`](../../pilots/parser-precedence-in-flag/seed.md)
EXT 1+2 landed (-48 LOC at the bare-ident fast-path deletion); EXT 3 (for-* audit) open. Precedence-climber-tier.

### (cc) `for-head-non-binding-lhs` (FHNB) — **SPAWNED, FOUNDED** 2026-05-25 at [`pilots/for-head-non-binding-lhs/`](../../pilots/for-head-non-binding-lhs/seed.md)
Bytecode/runtime tier; substrate work pending. Per R4 should land as multi-tier coherent unit.

---

## Tier G — meta-apparatus (audit / process)

### (dd) `locale-positioning-audit` (LPA) — **SPAWNED + operationally complete** 2026-05-25 at [`pilots/apparatus/locale-positioning-audit/`](../../pilots/apparatus/locale-positioning-audit/seed.md)
Three-rung methodology (stale-claim sweep, spinoff-chain mapping, positioning-gap detection); all three executed at least once. Future runs re-render the findings docs per triggers.

### (ee) `pinart-categorizer-refinement` — see (z) above.

---

## Tier I — tokenization above the ECMA IR (substrate; 2026-05-25 brief)

Candidates surfaced by `docs/engagement/tokenization-above-ir-candidate-brief.md` against the test262 lex-tier surface (~802 fixtures across 7 subdirs). Read: lex-tier yield isn't "close more parse: errors" but "find wrong-result downstream coordinates whose ROOT CAUSE is at the lex tier and surface them as their own named coordinates."

### (pp) `numeric-literal-conformance` — **SPAWNED** 2026-05-25 as [`pilots/numeric-literal-conformance/`](../../pilots/numeric-literal-conformance/seed.md)
**Status**: FOUNDED. NLC-EXT 0 baseline-inspection corrected post-IDT Rule-23 verification; NLC-EXT 1-revised is lex-tier malformed numeric rejection.

### (qq) `identifier-tokenization` — 🟢 RIPE
**Telos**: §11.6 IdentifierName + ReservedWord + UnicodeID ranges + **had-escape preservation** (the A3 axis from prior parser-permissiveness work — the lexer must preserve a "had-escape" bit on identifier tokens so the parser's reserved-word gate can reject escaped reserved-words like `let in`).
**Pool**: 268 fixtures in `language/identifiers/` (largest single lex-tier sub-dir).
**LOC estimate**: ~30-50 for had-escape; variable for unicode-id range extension.
**Status**: queued.

### (rr) `string-literal-and-escape-conformance` — 🟢 RIPE
**Telos**: §12.9 StringLiteral cooked/raw separation, escape decoding (\u{XXXX}, surrogate pairs, lone surrogates, hex escapes, line continuations).
**Pool**: 73 fixtures in `language/literals/string/` + downstream wrong-value tests.
**LOC estimate**: ~40-80.
**Status**: queued.

### (ss) `regex-literal-lexing` — 🟡 PROBED
**Telos**: §12.9.5 RegularExpressionLiteral lex production — pattern + flags accumulator + line-terminator rejection inside literal. Separate from regex-engine semantics.
**Pool**: composes with `regexp-conformance/`; likely nested rung inside that locale rather than sibling.
**LOC estimate**: ~20-40.
**Status**: deferred as nested candidate under `regexp-conformance/`; RC-EXT 0 baseline-inspection decides whether the lexing partition has multi-rung shape.

### (tt) `private-name-lexing` — ⚪ HYPOTHETICAL
**Telos**: §13.3 PrivateIdentifier `#name` tokenization for class private members.
**Pool**: small visible surface (1 in PCR's parse: bucket) but large potential in class-elements test262 sub-dirs.
**Status**: deferred until class-elements work is on the critical path.

---

## Tier J — apparatus-pilot extensions (sibling to PCR)

Apparatus-pilot candidates that extend PCR's coordinate-refinement discipline to additional tiers. Each is small (~15-30 LOC) and produces named coordinates that substrate locales can target with clear move shapes.

### (uu) `tokenizer-error-classification-refinement` (TECR) — 🟢 RIPE
**Telos**: extend PCR's categorizer (`pilots/apparatus/test262-categorize/derived/src/bin/full_pinart.rs`) to split the `availability/missing-parser-feature` projection class into lex-tier vs syntax-tier sub-classes. Today these collapse together; sharpening them surfaces lex-tier substrate work explicitly per the apparatus §XI lexical-grammar coordinate class.
**Composes with**: PCR-EXT 2's `missing-lowering-feature` pattern — same shape applied at the lex tier.
**LOC estimate**: ~15 LOC in `full_pinart.rs::projection_axis`.
**Status**: queued, FIRST apparatus-tier spawn from tokenization-above-IR brief; **lands BEFORE Tier-I substrate locales** per LPA-EXT 3 Finding LPA.5 (apparatus-tier refinement precedes substrate-tier spawns).

---

## Tier H — top-10 spawn batch from 2026-05-25 matrix (post-hoc registration)

Spawned 2026-05-25 from the full-suite Pin-Art matrix's top-10 coordinates (commit 561b7aa4). Registered here for audit-trail; each is anchored to a specific matrix coordinate per its seed.

| Ref | Locale | Matrix rank | Pool | Status |
|---|---|---:|---:|---|
| (ff) | `temporal-availability/` | 1 | 4,152 | FOUNDED |
| (gg) | `ast-bytecode-syntaxerror-cluster/` | 2 | 1,296 | FOUNDED |
| (hh) | `ast-bytecode-wrong-result/` | 3 | 1,244 | FOUNDED |
| (ii) | `ast-bytecode-missing-method/` | 4 | 1,088 | FOUNDED |
| (jj) | `parser-early-error-residual/` | 5 | 809 | ACTIVE (BBND nested closed 95 tests of sub-cluster) |
| (kk) | `ast-bytecode-uncategorized-projection/` | 6 | 659 | FOUNDED, apparatus-gap (per heuristics §IX); blocked on (v)/(w) precedence |
| (ll) | `ast-bytecode-missing-throw-typeerror/` | 7 | 622 | FOUNDED |
| (mm) | `typed-array-wrong-result/` | 8 | 614 | FOUNDED |
| (nn) | `typed-array-missing-method/` | 9 | 469 | FOUNDED |
| (oo) | `spec-builtins-wrong-result/` | 10 | 389 | FOUNDED |

---

## Spawning protocol

1. Read this file + identify the candidate.
2. Run rule 11 5-axis pre-spawn check (component A/B is the load-bearing one).
3. If 🟢 RIPE: spawn `pilots/<name>/{seed.md,trajectory.md,docs/,fixtures/}` with the seed founding-pattern (telos, constraints, falsifiers, methodology, carve-outs, resume protocol).
4. Refresh `apparatus/locales/manifest.json` via `apparatus/locales/discover.sh`.
5. Commit founding + manifest in one change.
6. Per standing rule 13: design the deeper-layer closure from the founding round if known.

## Tier K — missing-syntax-feature concentration (2026-05-26, post TECR-EXT 2 lift)

Spawned from the post-TECR-EXT-2 matrix's `availability/missing-syntax-feature` projection class (1,017 records). Drilled by reason-shape clustering: 17 distinct shapes total, top 5 account for 937 records (92%). Extreme cluster-coherence per the multiplier conditions in BBND findings §IV — each candidate is a coherent single-mechanism substrate target.

| Ref | Cluster | Records | Mechanism | Status |
|---|---|---:|---|---|
| (vv) | `hoistable-declaration-as-statement-body` (HDSB) | 475 | Annex B B.3.4 — `if (x) function f(){}` parser rule | 🟢 RIPE |
| (ww) | `with-body-multi-statement-parse` (WBMS) | ~150 | parse_with_stmt fails on multi-stmt body across LT | 🟢 RIPE |
| (xx) | `import-meta-metaproperty` (IMM) | 76 | §13.3.13 ImportMeta — parser + IR + runtime stub | 🟢 RIPE |
| (yy) | `dynamic-import-attributes` (DIA) | 41 | stage-4 import-attributes — ImportCall second-param | 🟢 RIPE |
| (zz) | `compound-assignment-rbrace` (CAR) | 44 | parser quirk in CompoundAssignment LHS (drill-pending) | 🟡 PROBED |

### (vv) `hoistable-declaration-as-statement-body` (HDSB) — 🟢 RIPE
**Telos**: §13.6 IfStatement / Annex B B.3.4 — accept FunctionDeclaration as the Statement body of `if`/`else` in sloppy mode (and certain Annex B function-code contexts). cruft currently rejects with `parse: HoistableDeclaration is not allowed as Statement body`.
**Pool**: 475 records (190 annexB/language/eval-code/direct, 100 annexB/language/eval-code/indirect, 185 annexB/language/function-code/*).
**Verified probe**: `if (true) function f(){ return 1; } f();` → CompileError (cruft); spec says accept in sloppy mode.
**LOC estimate**: ~10-20 LOC at parser's Statement / IfStatement production + strict-mode carve-out.
**R13 prospective C1-C4**: C1 sibling (existing FunctionDeclaration parse path), C2 shape-compat (additive grammar rule), C3 cost-positive (single rule), C4 bail-safe (parser-only).

### (ww) `with-body-multi-statement-parse` (WBMS) — 🟢 RIPE
**Telos**: §14.11 WithStatement — accept Statement body that spans multiple lines / contains multiple statements. cruft parses `with(p){x;}` but rejects `with(p){\n console.log(x); \n}` with `parse: unexpected token in expression: Punct(RBrace)`.
**Pool**: ~150 records (130 in language/statements/with/, ~20 in Proxy/has + sm/lexical-environment).
**LOC estimate**: ~5-15 LOC at parse_with_stmt (likely the body is parsed via a single-expression path instead of Statement → BlockStatement).
**Carve-out**: strict mode forbids WithStatement entirely; cruft already rejects in strict — this only addresses the sloppy-mode body-parse bug.

### (xx) `import-meta-metaproperty` (IMM) — ★ REDIAGNOSED 2026-05-26 → apparatus

**Rule 23 baseline-inspection refuted the seed**. cruft already handles `import.meta` correctly (`console.log(import.meta)` → `{ url: ..., dir: ... }`). All 76 records were stage-X proposals:
- 38 `import-defer` (stage-3 deferred dynamic import)
- 38 `source-phase-imports` (stage-3 source-phase import)

Cluster redirected to apparatus-pilot: `pilots/apparatus/runner-features-skip-deliberate-omissions/` (RFSDO-EXT 1, LANDED). 76 records moved from FAIL to SKIP. No substrate work needed.

**Standing lesson**: when a candidate's reason text matches a feature name cruft already implements, the Rule-23 founding probe is especially load-bearing — the text overlap can suggest the wrong substrate target.

### (yy) `dynamic-import-attributes` (DIA) — 🟢 RIPE
**Telos**: stage-4 import-attributes — `import(specifier, { with: { type: 'json' } })` second-param syntax. cruft rejects with `expected ')' in dynamic import()` because it stops at the first comma.
**Pool**: 41 records (language/expressions/dynamic-import/import-attributes/*).
**LOC estimate**: parser extension to ImportCall production (~10 LOC); runtime can stub the attributes pending real implementation.

### (zz) `compound-assignment-rbrace` (CAR) — ★ REDIRECTED 2026-05-26 → WBMS-EXT 2

**Rule 23 baseline-inspection (post WBMS-EXT 1 landing)**: all 44 records now fail at RUNTIME with `scope.x === N. Actual: undefined`, not at parse. The compound-assignment tests use `with(scope) { x ^= 3 }` to exercise binding semantics under a `with`-extended scope chain. WBMS-EXT 1's parse-tier fix unblocked the parser; the residual is identical in mechanism to WBMS's 227 runtime-semantic residuals.

Substrate move IS WBMS-EXT 2 (real with-runtime-semantics: Stmt::With AST + bytecode emission + ScopeChain extension). No separate locale spawn. WBMS-EXT 2's pool when spawned will include these 44 CAR records.

**Standing lesson**: when a matrix-coordinate cluster's reason-text matches a parser-tier failure shape, baseline-inspect AFTER any sibling parser-tier moves have landed — Rule 23 may surface that the cluster is already absorbed by a sibling locale's deferred deeper-tier extension.

---

## Tier L — Temporal implementation (2026-05-26, multi-session program; restructured TI-EXT 1)

Spawned per keeper directive (Telegram 9873) immediately after RFSDO-EXT 2 SKIPped 6,694 Temporal records. Restructured per keeper directive (Telegram 9879): per-class work has its own multi-rung shape, so each class is a PARENT locale with nested sub-rungs.

True surface (built-ins + intl402 + staging): ~6,700 tests across 9 classes. Per-class substrate cost ~900-1100 LOC across 5-7 sub-rungs each. Total program ~8-10k LOC, dozens of sessions.

### Spawned

| Ref | Locale | Status |
|---|---|---|
| (TI) | `temporal-implementation/` (parent) | FOUNDED + restructured |
| (TF) | `temporal-implementation/temporal-foundation/` | LANDED — namespace + class stubs |
| (TN) | `temporal-implementation/temporal-now/` (parent) | FOUNDED — execution blocked on temporal-tz-string-parse |

### Planned per-class parent locales (each with 4-7 nested sub-rungs of its own)

| Ref | Parent locale | Surface | First rung to spawn |
|---|---|---:|---|
| (TDur) | `temporal-duration/` | ~559 | `duration-ctor-fields/` |
| (TInst) | `temporal-instant/` | ~482 | `instant-ctor-fields/` |
| (TPT) | `temporal-plain-time/` | ~505 | `plain-time-ctor-fields/` |
| (TPD) | `temporal-plain-date/` | ~1143 | `plain-date-ctor-fields/` (after ISO calendar) |
| (TPDT) | `temporal-plain-date-time/` | ~1254 | (after PlainDate + PlainTime) |
| (TPMD) | `temporal-plain-month-day/` | ~289 | (after ISO calendar) |
| (TPYM) | `temporal-plain-year-month/` | ~834 | (after ISO calendar) |
| (TZDT) | `temporal-zoned-date-time/` | ~1481 | LAST — needs IANA TZ database |

### Planned shared sub-substrate locales

| Ref | Locale | Needed by |
|---|---|---|
| (TISP) | `temporal-iso-string-parse/` | every per-class string conversion + from() |
| (TTZP) | `temporal-tz-string-parse/` | Now, ZonedDateTime |
| (TBN) | `temporal-bigint-nanoseconds/` | Instant, ZonedDateTime |
| (TIC) | `temporal-iso-calendar/` | PlainDate, PlainDateTime, PlainMonthDay, PlainYearMonth, ZonedDateTime |

### Execution order (restructured TI-EXT 1)

1. temporal-foundation ✓ LANDED
2. duration-ctor-fields (lowest entanglement: no calendar, no TZ)
3. instant-ctor-fields (BigInt nanoseconds — sibling shape)
4. plain-time-ctor-fields (wall-clock, no calendar)
5. arithmetic / string-conversion sub-rungs for Duration / Instant / PlainTime
6. temporal-iso-calendar (shared substrate)
7. PlainDate / PlainDateTime / PlainMonthDay / PlainYearMonth ctor + arithmetic rungs
8. temporal-tz-string-parse + temporal-now
9. ZonedDateTime LAST

### RFSDO sync protocol (TI.4)

Single `Temporal` flag is too coarse. When first per-class ctor rung lands, RFSDO needs `PARTIALLY_IMPLEMENTED` map (feature → path-prefix-allowlist). Tests matching an allowlist entry opt OUT of the SKIP. This change lands with `duration-ctor-fields`.

---

## Standing edits

- When a locale is founded, **move its entry from this file to its own `pilots/<name>/seed.md`**; leave a one-line "**SPAWNED** as `pilots/<name>/` at YYYY-MM-DD" stub here for the audit trail.
- When a candidate is empirically falsified (component A/B shows the dominator is not what was predicted), strike through + annotate why.
- When new candidates surface from chapter-close disposition sections of other locales, append them under the appropriate tier.
