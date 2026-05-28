# Coverage-gap orphan disposition (2026-05-28)

Per keeper directive Telegram 10160: apply Doc 744 (pipeline-form discovery) + Doc 745 candidate (structured per-Phase emission + SIPE-T fractal fitting) to the eight orphan locales the Plan-agent back-fit (per Telegram 10158) flagged as not cluster-fitting cleanly under any single arc. For each orphan, recover (M, T, I, R, observability) at the locale tier; discriminate the relational form per Doc 744 §IV; propose a disposition (enroll-in-arc, lift-to-meta-arc, scaffold-new-arc, split-into-lattice-meet, defer-as-singleton).

This doc is itself a worked exercise of the SIPE-T fractal heuristic (Doc 745 candidate §IV): each orphan is a candidate substrate move at the locale tier seeking the arc tier it fits.

---

## I. The orphans

The eight orphans the Plan agent flagged (in `apparatus/arcs/2026-05-28-*.md` back-fit analysis): `ecmascript-parity-shared-upstream-arc`, `engine-sentinel-non-enumerable`, `global-builtins-non-enumerable`, `console-log-inspect-formatter`, `finally-abrupt-completion-lowering`, `generator-coroutine-suspension`, `via-method-audit`, `date-month-convention-fix`. Plus `compartment-primitive` (cross-listed, not a true orphan), `ecmascript-parity-shared-upstream-arc` (suspected misfile), and `global-binding-surface-unification` (sub-locale of an existing arc).

---

## II. Per-orphan disposition

### II.1 `ecmascript-parity-shared-upstream-arc`

**(M, T, I, R, observability)**: M = post-ASD test262-sample baseline at 80.6% (5872/7288); T = post-five-shared-substrates target rate ~85% (+340 PASS cascade); I = the five shared-upstream constraints identified at T262C-EXT 2 prospective analysis; R = parent-arc relation to T262C, with sibling-locale relations to FODAS/PPA/REOU/VHTB/IPEP/AEVPD/SDIBP/ASD (sub-locales closed in the first arc round); observability = ordinary test262-sample measurement.

**Per Doc 744 + 745 reading**: this is not a substrate-tier locale. It is an **arc-tier artifact misfiled as a locale**. The name itself ("ecmascript-parity-shared-upstream-ARC") declares its tier. The seed.md §I.1 explicitly orders its sub-locales as the arc's resume-vector projection. Per `apparatus/docs/arc-as-coordinate.md` §B, arcs accumulate sub-locale rosters; the EPSUA seed is performing this accumulation under a locale's directory shape.

**Disposition**: **promote to arc-tier** at `apparatus/arcs/2026-05-25-ecmascript-parity-shared-upstream/` (matching its 2026-05-25 founding date). Migrate seed.md content to arc.md per the arc-as-coordinate.md format. Migrate trajectory.md (if any) to log.md. Remove the `pilots/ecmascript-parity-shared-upstream-arc/` directory from the locale-coordinate space; the apparatus/arcs/ entry replaces it. Update `apparatus/locales/manifest.json` accordingly (the next `apparatus/locales/discover.sh` run will remove it).

This is the first instance of an arc-tier-as-locale mis-categorization the engagement has surfaced; standing rec for future spawns: when a seed's telos statement reads as a multi-substrate program with named sub-substrates each at the same coordinate tier, the spawn belongs in `apparatus/arcs/`, not `pilots/`.

### II.2 `engine-sentinel-non-enumerable`

**(M, T, I, R, observability)**: M = engine-internal sentinel write site (`__map_data`, `__set_data`, `__is_weakmap`, `__date_ms`, `__cruftless_http_*`); T = property installation with `{w:t, e:f, c:f}` descriptor (non-enumerable per source-identifier convention); I = the ~6-10 sites across `interp.rs` + `intrinsics.rs` + `cruftless/src/http.rs` that install sentinels via `rt.object_set` (which uses the default-enumerable descriptor); R = lattice with descriptor-bridge arc + with engine-pillar substrate; observability = ordinary (Object.keys / for-in / JSON.stringify surface the leak).

**Per Doc 744 §IV**: this is a **lattice-meet** between two proposed arcs: the descriptor-bridge arc (currently proposed, not yet scaffolded; "2026-05-28-property-descriptor-bridge") and the engine-pillar substrate (`rusty-js-runtime`). The locale's substrate move (replace `object_set` with `install_engine_sentinel(rt, target, key, value)` using `{w:t, e:f, c:f}`) operates at the engine-runtime tier but the convention it enforces (source-identifier sentinel = non-enumerable) is documented at CLAUDE.md §Source-identifier coordinate conventions.

**Disposition**: enroll in the descriptor-bridge arc when scaffolded. Until then, **leave as orphan with cross-reference annotation** in its seed.md naming the lattice-meet. Add the cross-reference as part of this doc's commit.

### II.3 `global-builtins-non-enumerable`

**(M, T, I, R, observability)**: M = global built-in installation site (`install_global_this` line 433 + Intl namespace install + `globalThis`/`global` self-references); T = property installation with `{w:t, e:f, c:t}` descriptor per ECMA-262 §17 baseline + §19 Global Object; I = the install loop + 3-4 sibling install sites; R = lattice with descriptor-bridge arc + sibling to `engine-sentinel-non-enumerable`; observability = ordinary (test262 `built-ins/{Map,Set,...}/prop-desc.js` failures with "descriptor should not be enumerable").

**Per Doc 744 §IV**: same lattice-meet as II.2 (descriptor-bridge arc + engine-pillar substrate). The two locales share an emission pattern at the same compiler-tier and exchange the same typed-primitive contract (install-with-descriptor); per Doc 744 §IV.3 they are also in **alphabet-exchange relation** with each other at the engine-runtime tier (both consume the `install_X(rt, target, key, value, descriptor_shape)` typed-primitive contract).

**Disposition**: pair with II.1 under the future descriptor-bridge arc. The two locales (II.2 + II.3) plus the previously-closed `via-method-audit` discipline (which audits `set_own_frozen` vs `set_own_internal` vs `set_own` convention; see II.7) form the **install-helper-convention** sub-cluster of the descriptor-bridge arc. Scaffold the descriptor-bridge arc with the three as its initial roster + the seven other descriptor-bridge cluster locales the Plan agent enumerated.

### II.4 `console-log-inspect-formatter`

**(M, T, I, R, observability)**: M = console.log / .error / .warn call site with non-string arg; T = Node-compatible formatted output per `util.inspect` semantics (Arrays as `[ 1, 2, 3 ]`, Objects as `{ key: value }`, Sets as `Set(N) {...}`, etc.); I = the inspect_value recursive formatter + per-type dispatch + depth-cap + circular-reference handling; R = host-runtime API relation (Node util.inspect spec source, not WHATWG); observability = ordinary (console output is human-readable).

**Per Doc 744 §IV**: this locale **does not fit the host-runtime-api umbrella cleanly** because its spec source is Node util.inspect, not WHATWG console. The Plan agent flagged it as "sits orthogonally" to the host-runtime arc. Reading via Doc 745 candidate §III.1: the locale's mouth is at the host-runtime tier (console namespace) but its terminus is at the apparatus-helper tier (the inspect formatter is a substrate-side helper that the host runtime consumes). This is a **DAG relation** per Doc 744 §IV.1: the inspect formatter's terminus feeds the console namespace's mouth.

**Disposition**: **scaffold a small "engine-side-formatters" sub-arc** that subsumes inspect-formatter + future siblings (toString-customization, error-stack-formatter, etc.) under one arc. OR — preferred per Doc 744 §IV's discriminator — enroll in the future host-runtime-api arc's "host-node-compat" sub-arc (per the Plan agent's recommendation to split the host-runtime umbrella) since util.inspect IS the node-compat surface for console.log formatting.

**Recommendation**: defer until the host-runtime-api umbrella is scaffolded with its sub-arc decomposition; enroll then.

### II.5 `finally-abrupt-completion-lowering`

**(M, T, I, R, observability)**: M = try/catch/finally AST + break/continue/return inside try block; T = bytecode emission that routes break/continue/return THROUGH the finally block per ECMA-262 §14.15.3; I = the compile_try + emit_break + emit_continue + emit_return paths that currently bypass TryExit; R = DAG ↓ bytecode emit (finally-lowering is parser-tier compiler work feeding bytecode); R = lattice with iterator-protocol arc (the for-of/yield* abrupt-completion paths share the same finally-routing primitive); observability = ordinary (test fixtures observe finally-block side-effects).

**Per Doc 744 §IV**: the Plan agent noted overlap between iterator-protocol arc + parser-lowering. Reading per Doc 745 §III.1: the locale's substrate move is **compiler-tier emit-site work**, not iterator-protocol-tier work. The shared primitive ("route abrupt-completion through finally") is what iterator-protocol's IteratorClose-on-abrupt also depends on, but the substrate fix lives in the compiler.

**Disposition**: enroll in **`2026-05-28-parser-early-error-conformance`** arc (already scaffolded). Although this locale is more of a "parser-lowering" than "parser-early-error" surface, the parser arc is the closest existing scaffold; alternatively scaffold a separate "compiler-lowering-conformance" arc when its sibling-roster grows (mechanism gaps #1, #5, #6, #7 from the diff-prod analysis are compiler-tier; finally-abrupt is #5). The parser arc enrollment is the **provisional disposition** until a compiler-lowering arc is justified by ≥3 enrolled compiler-tier locales.

### II.6 `generator-coroutine-suspension`

**(M, T, I, R, observability)**: M = generator function declaration (is_generator=true on FunctionProto) + yield/yield* call sites; T = coroutine suspend/resume semantics per ECMA-262 §27.5 (yield is suspension point, next(val) delivers value back, throw(err) lands at yield site); I = the generator-frame-as-coroutine primitive + yield call-site emission + the suspended-state Value sentinel; R = lattice with iterator-protocol arc (generator IS an iterator implementation); observability = ordinary (test fixtures observe yield-returned values + suspended-state semantics).

**Per Doc 744 §IV**: the locale's terminus IS an iterator-protocol surface (generators implement the iterator protocol via §27.5 coroutine mechanics). However, the substrate move (replace eager-collect with proper suspend/resume) is a **substantial compiler+runtime substrate change** that's a sub-arc-sized program, not a single rung.

**Disposition**: enroll in **`2026-05-28-iterator-protocol-substrate`** arc (already scaffolded). The locale is already named in that arc's roster (per the arc.md). The overlap with parser/compiler-tier is genuine but the iterator-protocol arc is the right home because the terminus is observed at the iterator-protocol surface. The arc's IPS.1 finding-sketch covers the cross-locale recurrence.

### II.7 `via-method-audit`

**(M, T, I, R, observability)**: M = `_via` method invocation across cruft's 228 `_via` methods in `interp.rs`; T = spec-step-ordered emission per the original ECMA-262 algorithm (no reordering); I = the audit-pass enumeration + per-method classification (Pattern 1: spec-order-divergence; Pattern 2: static-coerce-on-user-arg per RPTC.7); R = apparatus-tier relation (the audit IS apparatus, not substrate); observability = ordinary.

**Per Doc 744 + 745**: this is **not a substrate-tier locale** but a **methodology-tier discipline artifact**. The audit's terminus is a finding-promotion to standing-rule status (Rule 21 already enforces "minimum-scope and verified"). The locale is more accurately an **apparatus pilot**, per the bilateral pilot tier distinction in `apparatus/docs/repository-apparatus.md` §0.

**Disposition**: **relocate to `pilots/apparatus/via-method-audit/`** per the bilateral pilot tier convention (apparatus-pilots vs substrate-pilots; established 2026-05-25). Update `apparatus/locales/manifest.json` on next discover.sh refresh. Cross-reference from `apparatus/docs/repository-apparatus.md` §II (measurement instruments table) since the audit produces apparatus-grade findings, not substrate yield.

### II.8 `date-month-convention-fix`

**(M, T, I, R, observability)**: M = `new Date(Y, M, D, ...)` calls + Date arithmetic via ymd_to_ms; T = correct Howard Hinnant chrono algorithm dispatching month 0 (January) → February pivot per the algorithm's convention vs cruft's off-by-N-day error; I = the single ymd_to_ms helper at intrinsics.rs:9932; R = singleton-substrate; observability = ordinary (test262 + diff-prod date fixtures).

**Per Doc 744 + 745**: the locale is a **single-rung substrate fix** (~3 LOC per its seed.md "+4 sibling yield" close). It does not have arc-class roster density — it's a discovered-as-side-finding fix during IDTP-EXT 1 (Telegram 9899). The Plan agent suggested a "legacy-Date intrinsics mini-arc paired with Annex B Date legacy methods". Reading via Doc 744 §IV.2: this is a **lattice-meet** between the Temporal arc (Date math shared with Temporal Instant) AND a future Annex B Date legacy arc.

**Disposition**: leave as **closed singleton-locale** (already LANDED per IDTP-EXT 1 +4 sibling yield). Future "annex-B Date intrinsics" arc (per the Plan agent's proposed arc 9: 2026-05-28-annex-b-language-partition) can retroactively enroll it. **Cross-reference annotation** to the Temporal arc + the future Annex B arc as lattice-meets. No relocation needed; closed singletons can persist in `pilots/` without arc enrollment if their cross-arc relations are explicit.

---

## III. Cross-orphan patterns

Three patterns emerge from the per-orphan analysis.

### III.1 The arc-tier-as-locale mis-categorization

Orphan II.1 (`ecmascript-parity-shared-upstream-arc`) is the canonical instance: a multi-substrate program with named sub-substrates at the same coordinate tier was filed under `pilots/` rather than `apparatus/arcs/`. The mis-categorization went unnoticed because the seed.md format accommodates either tier; only the per-Doc-745 SIPE-T fractal reading discriminates them. Standing rec: at locale spawn (Phase 1 emission), check whether the seed's telos statement enumerates sub-substrates at the same coordinate tier as the seed itself. If yes, the spawn belongs in `apparatus/arcs/`.

### III.2 The lattice-meet repetition

Orphans II.2 + II.3 (sentinel-non-enumerable + builtins-non-enumerable) are a **lattice-meet pair**: same substrate tier, distinct mouth-terminus pairs, shared install-helper-convention emit shape. Per Doc 744 §IV.5 (composition of forms) the meet is a sub-cluster within the future descriptor-bridge arc. Together with `via-method-audit` (II.7, methodology-tier) they form the install-helper-convention discipline triplet. Standing rec: when two locales share emit-site enumeration shape AND substrate tier, they are lattice-meet candidates; promote both into the same arc's roster with explicit lattice-meet annotation in the arc.md.

### III.3 The apparatus-vs-substrate mis-categorization

Orphan II.7 (`via-method-audit`) is the apparatus-pilot version of III.1's category error: an audit-discipline artifact filed as a substrate-pilot. The bilateral pilot tier (established 2026-05-25; per `apparatus/docs/repository-apparatus.md` §0) discriminates apparatus-pilots from substrate-pilots; via-method-audit fits the former. Standing rec: locales whose primary output is per-method classification / audit findings (consumed by other pilots' substrate work) are apparatus-pilots; relocate to `pilots/apparatus/<name>/`.

---

## IV. Dispositions summary

| Orphan | Disposition | Action |
|---|---|---|
| II.1 `ecmascript-parity-shared-upstream-arc` | Promote to arc-tier | Migrate to `apparatus/arcs/2026-05-25-ecmascript-parity-shared-upstream/`; update manifest |
| II.2 `engine-sentinel-non-enumerable` | Enroll (future) descriptor-bridge arc | Add cross-reference annotation in seed.md naming the lattice-meet |
| II.3 `global-builtins-non-enumerable` | Enroll (future) descriptor-bridge arc | Pair with II.2 + II.7 as install-helper-convention triplet |
| II.4 `console-log-inspect-formatter` | Defer; enroll in host-runtime-api when scaffolded | Cross-reference annotation to future host-node-compat sub-arc |
| II.5 `finally-abrupt-completion-lowering` | Provisional: enroll in parser-early-error arc | Add to roster; promote to compiler-lowering arc when ≥3 compiler-tier locales justify the split |
| II.6 `generator-coroutine-suspension` | Enroll in iterator-protocol arc (already enrolled) | Verify roster entry; cross-reference compiler-tier substrate scope |
| II.7 `via-method-audit` | Relocate to apparatus-pilot | Move to `pilots/apparatus/via-method-audit/`; update manifest |
| II.8 `date-month-convention-fix` | Closed singleton; defer to future Annex B arc | Cross-reference annotation to Temporal + future Annex B arc |

---

## V. Recommended action sequence

1. **Promote II.1** to arc-tier (highest-confidence misfile; preserves the EPSUA program's coherent multi-substrate shape).
2. **Annotate II.2 + II.3 + II.7 cross-references** so the install-helper-convention triplet is discoverable when the descriptor-bridge arc scaffolds.
3. **Relocate II.7** to `pilots/apparatus/` per the bilateral pilot tier discipline.
4. **Verify II.6 enrollment** in the iterator-protocol arc's roster (the arc.md already lists `generator-coroutine-suspension`; no action needed if roster is correct).
5. **Add provisional roster entry for II.5** in the parser-early-error arc's arc.md.
6. **Defer II.4 + II.8** with cross-reference annotations only; no relocation needed.
7. **Refresh manifest** via `apparatus/locales/discover.sh` after II.1 + II.7 relocations.

---

## VI. Heuristic validation

This doc itself is a worked instance of the SIPE-T fractal heuristic (Doc 745 candidate §III) applied at the **orphan-disposition tier**. Each orphan is treated as a candidate substrate move; the test is whether the candidate's (M, T, I, R) tuple fits an existing or proposed arc's tuple by the §IV.3 sub-shape correspondence. The three cross-orphan patterns (III.1, III.2, III.3) are arc-tier findings the heuristic surfaced; per Doc 745 candidate §III.2's correspondence, they should promote to standing-rule candidates if they recur across future orphan-disposition exercises.

Pattern III.1 (arc-tier-as-locale mis-categorization) is the strongest standing-rule candidate. If it recurs at 2+ future orphan-disposition exercises, promote to Rule 29: "Multi-substrate-tier telos at locale spawn ⇒ arc-tier classification". This would be the first standing rule promoted via the Doc 745 candidate fractal heuristic operating at the apparatus-tier discipline scale.
