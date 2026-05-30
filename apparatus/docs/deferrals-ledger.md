# Deferrals Ledger

A standing apparatus-tier record of candidate locales (or candidate sub-substrate moves within an open locale) that a substrate rung surfaced but did not yet found. Per keeper directive (Telegram 10176): deferrals currently live scattered across `apparatus/locales/CANDIDATES.md` DEFERRED class + per-rung trajectory cross-locale notes, and the latter does not aggregate — once the trajectory tail rolls past, a flagged candidate is functionally invisible.

This ledger is the aggregator. Modeled on `apparatus/docs/deletions-ledger.md` (Telegram 9800 precedent: an asymmetry between "tracked substrate moves" and "untracked substrate moves" is corrosive when both classes carry methodological signal).

## Why a ledger

Cruftless's apparatus has rich machinery for tracking new substrate work (locale seeds, trajectories, manifest, standing-rule additions, findings.md, deletions-ledger). It had nothing for tracking **substrate work that was surfaced but deliberately not yet undertaken**.

The asymmetry is real and corrosive:

- A founded locale carries seed + trajectory + manifest entry; the cybernetic loop reads it.
- A deferral is named once at the surfacing rung and then — apart from CANDIDATES.md's DEFERRED class, which is keeper-curated and lossy — disappears from the apparatus's read surface.
- When the gating condition later closes (the upstream DAG terminus lands, the spawn threshold is reached, the consumer-app surfaces the divergence), no one is reading for the un-defer signal. The flagged candidate is forgotten until accidentally re-surfaced.

This ledger restores the binding. Each entry records:

1. **Candidate name** — the proposed locale (or sub-substrate move within an open locale) that was surfaced but deferred.
2. **Originating rung** — the locale + EXT # whose Phase-5 chapter-close-inspect (or equivalent surfacing) flagged the candidate. Often a `**Cross-locale note**` section in the rung's trajectory entry.
3. **Date flagged** (absolute, per CLAUDE.md em-dash discipline anchor).
4. **Class** — one of:
   - **mouth-gating** (Doc 744 §IV.1.a) — gated by an unclosed upstream-DAG terminus that another locale or rung will close.
   - **spawn-threshold** (Doc 737 §II) — the duplication count of the same substrate-shape across cells is below the locale-promotion threshold; defer until more cells exhibit the shape.
   - **cost-positive** (Doc 744 §V.3 C3) — the value/cost ratio is below 1 at the current consumer surface; defer until a consumer-app surfaces the need.
   - **consumer-app-driven** — speculative substrate work whose empirical anchor is absent; defer until a real-world divergence surfaces.
   - **probe-pending** — a baseline-inspect probe is needed before the locale can be founded with M-T-I-R confidence.
5. **Gating predicate** — the named upstream condition (specific DAG terminus / specific cell count / specific consumer surface / specific probe).
6. **Un-defer condition** — the observable signal that would promote the candidate from DEFERRED to FOUNDED. Future readers of this ledger should be able to detect un-defer events without re-deriving the original rung's reasoning.
7. **Status** — DEFERRED (open) / PROMOTED (founded; cite the founding rung) / SUPERSEDED (the gating condition resolved by a different route; cite the route) / RETRACTED (the candidate is no longer thought to carry yield; cite the rung that retracted).

## Discipline (append-only)

Per Doc 727 §X basin-stability discipline (same as findings.md, deletions-ledger.md): this file is **append-only**. New entries go at the bottom in chronological order. Older entries are never edited; if a candidate is PROMOTED / SUPERSEDED / RETRACTED, append a NEW entry citing the prior with a back-reference and update the prior entry's Status field in place (the single allowed in-place edit, per the deletions-ledger.md precedent for status-flips on prior entries).

## Discovery hook

Phase 5 chapter-close-inspect (Rule 15) and Phase 6 deferral-emission (proposed sibling, per the TAWR-EXT 3 cross-locale note pattern) are the two phases at which deferrals are surfaced. When a rung's Phase-5 inspection surfaces a candidate that does not meet the founding threshold of its proposed coordinate tier, the rung MUST emit a deferrals-ledger entry as part of the close, in addition to the trajectory's cross-locale note.

The standing rule the ledger formalizes: **a substrate rung that surfaces a candidate locale but does not found it owes the apparatus a ledger entry, not merely a trajectory note.** The trajectory note records the rung-local reasoning; the ledger entry makes the candidate readable from outside the originating locale.

---

## Entries

### Entry 001 — `bigint-arithmetic-wrongness` (2026-05-28)

- **Candidate name**: `bigint-arithmetic-wrongness` (proposed locale at `pilots/bigint-arithmetic-wrongness/`).
- **Originating rung**: `typed-array-wrong-result/trajectory.md` TAWR-EXT 3, "Cross-locale note" section.
- **Class**: spawn-threshold (Doc 737 §II) — currently one cell observed (asIntN/asUintN closed by EXT 3); the locale's locus repeats across at least three more cells in the BigInt namespace that share the same substrate shape (passthrough-stub-as-deferment + ordered-coercion-as-substrate-concern).
- **Gating predicate**: cell count of cells exhibiting the bigint-arithmetic-wrongness shape ≥ 3. Currently 1 (the EXT 3 close).
- **Un-defer condition**: a substrate rung in this engagement surfaces a second BigInt-namespace passthrough-stub / wrong-arithmetic / wrong-coercion-ordering failure on test262 or diff-prod that shares Finding TAWR.3's shape. Candidate cells to inspect: `BigInt(arg)` constructor, `BigInt.prototype.toLocaleString` (currently routes through `to_radix(10)` without locale handling), `BigInt.prototype.toString(radix)` (radix validation), bigint operator-overloaded arithmetic edge cases (modulo sign, divmod by zero error path), the Atomics.* BigInt overloads. If two of these surface failures matching the shape, found the locale.
- **Status**: DEFERRED.

### Entry 002 — `cruftscript-spec` (2026-05-24, back-fill from CANDIDATES.md (k))

- **Candidate name**: `cruftscript-spec` (proposed locale at `pilots/cruftscript-spec/`).
- **Originating rung**: TSR-EXT 5 (annotation-sidecar probe at IPBR consumer); citation in `apparatus/locales/CANDIDATES.md` Tier-A § (k).
- **Class**: probe-pending + cost-positive (Doc 744 §V.3 C3) — the C3 cost-positive condition failed at the 2026-05-24 probe; the probe returned NULL signal at the IPBR consumer for substrate-grounded rationale.
- **Gating predicate**: TSR-EXT 5's annotation-sidecar probe re-runs and returns either (a) positive signal (consumer-tier rationale for cruftscript-spec → found on grounded substrate claims) or (b) confirmed-null signal a second time (found on soundness-alone grounds with a smaller corpus claim per the candidate's note).
- **Un-defer condition**: second probe lands with either signal; the spec proceeds along the matching branch.
- **Status**: DEFERRED.

### Entry 003 — `ts-resolve-*` sub-locales (2026-05-XX, back-fill from CANDIDATES.md (m-s))

- **Candidate name**: `ts-resolve-{import-graph, declaration-files, paths-mapping, project-references, ...}` — sub-locales of the TypeScript-resolver arc.
- **Originating rung**: TCC-measurement instrument at `pilots/apparatus/ts-consumer-corpus/`; citation in `apparatus/locales/CANDIDATES.md` Tier-A § (m-s).
- **Class**: probe-pending — the TCC failure table is needed to gate per-sub-locale spawning.
- **Gating predicate**: TCC parse-parity measurement instrument runs against the curated TS-consumer corpus and emits a failure table partitioned by ts-resolve-sub-substrate.
- **Un-defer condition**: TCC failure table lands; sub-locales spawn against the empirically anchored failure-shape coordinates.
- **Status**: DEFERRED.

### Entry 004 — `gpi-override-safety` (back-fill from CANDIDATES.md (d))

- **Candidate name**: `gpi-override-safety` (proposed locale).
- **Originating rung**: GPI-EXT 3 cost analysis; citation in `apparatus/locales/CANDIDATES.md` Tier-A § (d).
- **Class**: consumer-app-driven — a synthetic correctness fixture surface; no empirical anchor in the current consumer-app surface.
- **Gating predicate**: a real-world consumer-app (npm-package-tier surface) surfaces the divergence where user-installed own-property override of an intrinsic key is observed under GPI cache.
- **Un-defer condition**: any test262 failure or consumer-app divergence whose root-cause is the GPI override-safety hole.
- **Status**: DEFERRED.

### Entry 005 — `module-loader-eager-cache` (back-fill from CANDIDATES.md (f))

- **Candidate name**: `module-loader-eager-cache` (proposed locale).
- **Originating rung**: cold-start perf observation; citation in `apparatus/locales/CANDIDATES.md` Tier-B § (f).
- **Class**: consumer-app-driven — no current empirical anchor; would need a "module-load-perf" arc with ≥3 sibling locales to justify the arc-tier coordinate.
- **Gating predicate**: a consumer-app surfaces a cold-start surface where module-loader is the dominator; AND ≥2 sibling module-load-perf candidates surface.
- **Un-defer condition**: CRB or consumer-app probe surfaces a cold-start fixture whose component A/B identifies module-loader as the dominator.
- **Status**: DEFERRED.

### Entry 006 — `regex-jit-precompile` (back-fill from CANDIDATES.md (g))

- **Candidate name**: `regex-jit-precompile` (proposed locale).
- **Originating rung**: regex-perf observation; citation in `apparatus/locales/CANDIDATES.md` Tier-B § (g).
- **Class**: consumer-app-driven — no current empirical anchor.
- **Gating predicate**: CRB or consumer-app probe surfaces a regex-heavy fixture whose component A/B identifies regex-compile/match as the dominator.
- **Un-defer condition**: empirical anchor lands.
- **Status**: DEFERRED.

### Entry 007 — `crypto-sha256-batch-investigation` (back-fill from CANDIDATES.md (i))

- **Candidate name**: `crypto-sha256-batch-investigation` (proposed locale within host-runtime-api umbrella).
- **Originating rung**: host-runtime-api surface speculation; citation in `apparatus/locales/CANDIDATES.md` Tier-C § (i).
- **Class**: consumer-app-driven — speculative.
- **Gating predicate**: a consumer-app surfaces a sha256-batched workload where the per-batch dispatch overhead is observable.
- **Un-defer condition**: empirical anchor lands.
- **Status**: DEFERRED.

### Entry 008 — `prototype-constructor-reverse-edge-audit` (2026-05-28)

- **Candidate name**: `prototype-constructor-reverse-edge-audit` (proposed apparatus-pilot at `pilots/apparatus/prototype-constructor-reverse-edge-audit/` — audit-tier, NOT a substrate locale per orphan-disposition Pattern III.3).
- **Originating rung**: `typed-array-wrong-result/trajectory.md` TAWR-EXT 4, Phase 6 deferral emission.
- **Class**: spawn-threshold (Doc 737 §II) — three offenders surfaced at one rung (ArrayBuffer, DataView, BigInt). Below the ≥5 threshold for an audit-pilot LIFT scan. Defer until a fourth offender surfaces OR until the keeper directs a sweep.
- **Gating predicate**: any future rung surfaces a fourth built-in whose `prototype.constructor` reverse-edge is missing (the failure shape: `Object.getPrototypeOf(instance).constructor === Object` rather than the expected ctor).
- **Un-defer condition**: fourth offender lands, OR keeper directs an apparatus-pilot sweep over the constructor-registration helpers in `intrinsics.rs` to enumerate every `set_own_frozen("prototype", proto)` site and verify a paired `set_own_internal("constructor", ctor)` exists.
- **Status**: DEFERRED.

### Entry 009 — `resizable-buffer-detection-per-access` (2026-05-28)

- **Candidate name**: `resizable-buffer-detection-per-access` (proposed locale at `pilots/resizable-buffer-detection-per-access/`).
- **Originating rung**: `typed-array-wrong-result/trajectory.md` TAWR-EXT 5, Phase 6 deferral emission.
- **Class**: spawn-threshold (Doc 737 §II) — three cells observed (DataView `custom-proto-access-resizes-buffer-{invalid-by-length, invalid-by-offset, valid-by-offset}.js`). At the ≥3 candidate-locale threshold for promotion; deferred only because the resizable-buffer substrate work is non-trivial (per-access OOB recompute + ArrayBuffer.prototype.resize tracking) and the current arc is closing toward Phase 5 chapter-close.
- **Gating predicate**: arc `2026-05-28-array-exotic-substrate` reaches Phase 5 chapter-close-inspect; OR a fourth resizable-buffer cell surfaces (TypedArray-side resizable-buffer OOB cluster — there are ~10 TypedArray exemplars in the "resizable-buffer-length-tracking" family that probably share this substrate's mouth).
- **Un-defer condition**: arc closure OR TypedArray-side resizable-buffer surface count reaches the threshold to promote this candidate to the founding rung of the new locale.
- **Status**: PROMOTED — founded 2026-05-30 at `pilots/resizable-buffer-detection-per-access/` (RBDPA-EXT 0). See ledger Entry 016 for the un-defer back-reference.

### Entry 010 — `ta-element-coercion-spec-faithful` (2026-05-28)

- **Candidate name**: `ta-element-coercion-spec-faithful` (proposed locale at `pilots/ta-element-coercion-spec-faithful/`).
- **Originating rung**: `typed-array-wrong-result/trajectory.md` TAWR-EXT 6 (negative-result Rule-13 revert; Phase 5 + Phase 6 emission).
- **Class**: probe-pending — the substrate move spans three sub-substrates that need joint design before founding: (a) ConvertNumberToTypedArrayElement per §10.4.5.16 for integer kinds (-0 normalization, range wrap, fractional truncation); (b) toFloat32 with canonical-NaN preservation for Float32Array (a naive `n as f32 as f64` cast breaks `Set/conversion-operation-consistent-nan`); (c) ToBigInt with error-propagation through `object_set_pk` for BigInt64/BigUint64 TAs (requires converting `object_set_pk` signature to Result, or threading error via a different path).
- **Gating predicate**: probe rung needed to choose the architecture for (c) — either (i) lift `object_set_pk` to `Result<(), RuntimeError>` (wide downstream impact; many callers) or (ii) route TA element-set through a dedicated Result-returning path before reaching `object_set_pk` (smaller blast radius but introduces a separate dispatcher). The cell-set in the TAWR exemplar pool is ~10 cells (TAC `from/new-instance-from-zero`, `internals/Set/BigInt/string-tobigint`, `internals/DefineOwnProperty/BigInt/key-is-numericindex-desc-not-writable`, etc.); above spawn threshold but the architecture decision is the gate.
- **Un-defer condition**: a probe-rung in this engagement lands a minimal `Result`-threading scaffold (either (i) or (ii) above) that closes one BigInt-TA cell without regressing TAMM. Once the scaffold exists, the remaining ~9 cells become a multi-rung close within this candidate's founded locale.
- **Lattice-meet**: Entry 001 (`bigint-arithmetic-wrongness`) — the ToBigInt branch is shared substrate. When both un-defer, they may either share the locale or pair-enroll as siblings under a "bigint-coercion-substrate" arc per orphan-disposition Pattern III.2.
- **Status**: DEFERRED.

### Entry 011 — `sidecar-outbox-ack-surfacing` (2026-05-29)

- **Candidate name**: `sidecar-outbox-ack-surfacing` (proposed apparatus-pilot enhancement to `apparatus/caacp-server/`).
- **Originating rung**: Helmsman 2026-05-29 EPSUA arc Phase-2 quartet round, surfaced via R1 message 52badab1 reporting "no APPROVED H262S-EXT 1 landing message is pending for this instance" after helmsman delivered approval as ack-body on R1's plan-message (892c195f). Cross-machine ack-propagation gap confirmed.
- **Class**: cybernetic-protocol enhancement (CAACP sidecar). Below substrate-locale tier; targets `apparatus/caacp-server/server.ts` polling loop.
- **Gating predicate**: any future cybernetic round surfaces a second instance of recipient-invisible substantive-direction-in-ack-body, OR more than two helmsman/arbiter sessions per week incur the workaround tax of authoring approvals as fresh outbound messages.
- **Un-defer condition**: extend sidecar polling to surface outbox-ack-changes when ack bodies meet a substantive-content threshold (proposed signal: `surface_to_recipient=true` flag in the ack payload OR ack body length ≥ N characters with intent-bearing keywords). Implementation lands new `data/inbound-acks-<role>[-<instance>].json` channel + extends `/local/inbox` to merge ack-surfaced content. Until landed: helmsman discipline §V.5 (approval-as-fresh-outbound) is the workaround.
- **Workaround in force**: agent-init-protocol §V.5 (added 2026-05-29). All substantive cybernetic direction rides fresh outbound messages with intent=response, related_to=<plan>.
- **Status**: DEFERRED.

### Entry 012 — `cjs-parent-directory-resolution` (2026-05-30)

- **Candidate name**: `cjs-parent-directory-resolution` (proposed locale under module-loader / CJS resolution residuals).
- **Originating rung**: `pilots/missing-intrinsic-loader-failures/trajectory.md` MILF-EXT 4, Phase 5 inspection after closing the toStringTag descriptor receiver bug.
- **Class**: cluster-pending — surfaced as the next `mongoose` blocker after the directive's toStringTag failure advanced, but currently observed in one package path only.
- **Gating predicate**: a second package smoke or top500/refined sweep row surfaces `require("..")` / parent-directory CJS resolution from inside a package subdirectory, OR keeper/helmsman directs immediate closure for the `mongoose` path.
- **Un-defer condition**: evidence reaches sibling threshold or direct directive lands; found a locale that audits CJS directory resolution relative to nested package files, including package boundary and `package.json` main/index fallback behavior.
- **Status**: DEFERRED.

### Entry 013 — `node-zlib-gunzip-sync-host-intrinsic` (2026-05-30)

- **Candidate name**: `node-zlib-gunzip-sync-host-intrinsic` (proposed locale under host-runtime-api / Node builtin parity).
- **Originating rung**: `pilots/missing-intrinsic-loader-failures/trajectory.md` MILF-EXT 5, Phase 5 inspection after closing the SharedArrayBuffer descriptor blocker surfaced by `mongoose`.
- **Class**: consumer-package-driven host intrinsic residual. Current evidence is the `mongoose` loader advancing to `node:zlib.gunzipSync not yet implemented (Tier-Ω.5.y stub)` after parent-directory resolution and SharedArrayBuffer descriptor visibility both pass.
- **Gating predicate**: a second package smoke/top500 row surfaces `node:zlib.gunzipSync`, `zlib.gunzipSync`, or adjacent sync zlib decompression as an import-time blocker, OR keeper/helmsman directs immediate closure for the `mongoose` path.
- **Un-defer condition**: evidence reaches sibling threshold or direct directive lands; found a locale that audits the sync zlib host intrinsic surface used during package import, starting with `gunzipSync` and its expected Buffer/input coercion behavior.
- **Status**: DEFERRED.

### Entry 014 — `buffer-read-uint32be-host-method` (2026-05-30)

- **Candidate name**: `buffer-read-uint32be-host-method` (proposed locale under host-runtime-api / Buffer numeric readers).
- **Originating rung**: `pilots/missing-intrinsic-loader-failures/trajectory.md` MILF-EXT 6, Phase 5 inspection after closing the `node:zlib.gunzipSync` blocker surfaced by `mongoose`.
- **Class**: consumer-package-driven host intrinsic residual. Current evidence is `mongoose` advancing to `callee is not callable: undefined ... (method='readUInt32BE')` in `@mongodb-js/saslprep/dist/memory-code-points.js` after zlib sync APIs pass.
- **Gating predicate**: a second package smoke/top500 row surfaces `Buffer.prototype.readUInt32BE` or sibling numeric Buffer readers (`readUInt16BE`, `readUInt32LE`, signed variants, etc.) as an import-time blocker, OR keeper/helmsman directs immediate closure for the `mongoose` path.
- **Un-defer condition**: evidence reaches sibling threshold or direct directive lands; found a locale that audits Buffer numeric read/write methods as a batch, starting with `readUInt32BE`.
- **Status**: DEFERRED.

### Entry 015 — `node-zlib-brotli-compress-sync` (2026-05-30)

- **Candidate name**: `node-zlib-brotli-compress-sync` (proposed locale under compression / host-runtime-api).
- **Originating rung**: `pilots/missing-intrinsic-loader-failures/trajectory.md` MILF-EXT 6, which implemented zlib/gzip/deflate sync methods plus Brotli decompression but left Brotli compression as an explicit unsupported operation.
- **Class**: substrate-missing — no Brotli encoder exists in `pilots/compression/derived`; implementing this correctly requires a Brotli encode substrate or a consciously borrowed encoder dependency.
- **Gating predicate**: a package smoke, top500 row, or test262/host-parity fixture executes `zlib.brotliCompressSync` as a required import-time or runtime path, OR keeper directs a compression substrate expansion.
- **Un-defer condition**: found a Brotli encode locale that either derives a minimal encoder or adopts a vetted pure-Rust encoder dependency under apparatus rationale.
- **Status**: DEFERRED.

### Entry 016 — `resizable-buffer-detection-per-access` PROMOTED (2026-05-30)

- **Candidate name**: `resizable-buffer-detection-per-access` (founded locale at `pilots/resizable-buffer-detection-per-access/`).
- **Back-reference**: Entry 009 (2026-05-28). Entry 009's Status field flipped in-place to PROMOTED per the discipline at this file's §Discipline (the single allowed in-place edit on prior entries).
- **Originating rung**: this entry — helmsman session 2026-05-30 under keeper directive Telegram 10558 ("Begin with 1") following the helmsman deferrals-vs-substrate audit (2026-05-30) that classified Entry 009 as UN-DEFER READY.
- **Un-defer signal**: spawn-threshold ≥3 cells was met at Entry 009's surfacing rung (TAWR-EXT 5 Phase 6) but deferred pending arc Phase-5 chapter-close. The arc's productive-rung sequence closed at the Phase-5 inflection (TAWR-EXT 5 LANDED + TAWR-EXT 6 NEGATIVE Rule-13 REVERT) per `pilots/typed-array-wrong-result/trajectory.md` EXT 6 conclusion ("`2026-05-28-array-exotic-substrate` arc reaches Phase-5 inflection at TAWR-EXT 5"). Gating condition satisfied; un-defer authorized.
- **Founding rung**: RBDPA-EXT 0 (2026-05-30) — apparatus scaffold and baseline pending; founding entry records the lift and the parent-rung citation.
- **Arc enrollment**: `2026-05-28-array-exotic-substrate` sub-locale roster updated in the same change.
- **Status**: RECORDED (this is a back-reference entry; Entry 009 carries the PROMOTED status flip).
