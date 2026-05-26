# locale-positioning-audit — Trajectory

## LPA-EXT 0 — founding (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9802), opened in response to the apparatus §XI.1.b amendment necessitated by PPIF-EXT 2's deletion. That amendment was the first instance where a sibling locale's close dissolved a prior locale's claimed-irreducible carrier without the apparatus auto-noticing; this locale exists to systematize the audit of such drift.

**Apparatus established**:

- `seed.md` — telos, three-rung methodology, trigger conditions, carve-outs, composes-with, R13 prospective check.
- No findings docs yet; the `findings/` subdirectory will house per-rung output (`stale-claims.md`, `spinoff-chains.md`, `positioning-gaps.md`).

**Initial inventory** (snapshot at founding):
- Locale count: 109 (per `apparatus/locales/manifest.json`).
- Deletions ledger entries: 2 (PPIF-EXT 2; LGSS-EXT 2).
- Visible spinoff chains: 1 confirmed (LGSS → PPIF → FHNB).
- Apparatus-doc amendments triggered by sibling-locale drift this session: 1 (apparatus §XI.1.b, this commit + the prior).

**R13 prospective C1-C4 all hold (per seed §Methodology)**:

- C1: Doc 727 basin-stability + Doc 415 retraction-ledger are corpus-tier siblings.
- C2: manifest is JSON, locales are markdown — both grep-able and walkable.
- C3: TBV at LPA-EXT 1; bounded by locale count.
- C4: append-only; never edits prior trajectories.

**Status**: LPA-EXT 0 FOUNDED. LPA-EXT 1 (stale-claim survey) is the first substantive rung; runs on-trigger rather than on a schedule, so the first run waits for the next deletion-ledger entry or keeper directive.

**Findings**

**Finding LPA.0 (the apparatus's drift class is bounded by its own discipline)**: cruftless's discipline is heavy on append-only artifacts (findings.md, trajectories, deletions ledger, this locale's findings/). The drift class the audit catches is specifically the kind that append-only doesn't catch: claims in prior artifacts that become STALE because sibling work elsewhere dissolved their basis. The audit's role is to surface staleness, not to correct it (correction is the original locale's amendment-by-new-trajectory-entry move; the audit just makes the case for amendment legible). Standing recommendation: any apparatus discipline whose primary mechanism is append-only is structurally vulnerable to claim-staleness; a co-running audit locale is the dyadic-ascent counterpart that restores coherence.

---

## LPA-EXT 1 — Phase 2 path-staleness sweep after bilateral-pilot-tier housekeeping (2026-05-25)

**Trigger**: Keeper directive (Telegram 9806) "Move to phase 2." Phase 2 refers to the bilateral pilot tier landing (commit 84798b0a) which deferred cross-citation sweeps to lazy resolution. LPA is the natural home for this sweep (a stale-claim survey of one specific class: path-staleness across the 6-locale bilateral move).

**Scope**:

- **In scope**: all `.md` files under `pilots/` (except `rusty-js-jit/findings.md` — protected as the canonical findings ledger under Doc 727 §X basin-stability), `apparatus/docs/`, `apparatus/locales/`, `docs/engagement/`, `docs/fca-instances/`.
- **Out of scope**: `docs/corpus-ref/` (read-only mirror of the published corpus per the apparatus tier-separation §0). `pilots/rusty-js-jit/findings.md` (canonical findings ledger).

**Carve-out preserved**: `docs/corpus-ref/737-the-locale-as-coordinate-...md` retains its pre-move path reference. The corpus doc's text is a historical record from publication time; updating it would constitute editing-the-corpus-from-substrate, a tier-separation violation. Path-staleness in corpus-ref is the keeper's to resolve at next corpus-publish if material.

**Execution**: mechanical `sed -i` over 20 files with six 1:1 pattern replacements:

```
pilots/test262-categorize/       → pilots/apparatus/test262-categorize/
pilots/diff-prod/                → pilots/apparatus/diff-prod/
pilots/cross-runtime-bench/      → pilots/apparatus/cross-runtime-bench/
pilots/ts-consumer-corpus/       → pilots/apparatus/ts-consumer-corpus/
pilots/ts-execute-corpus/        → pilots/apparatus/ts-execute-corpus/
pilots/locale-positioning-audit/ → pilots/apparatus/locale-positioning-audit/
```

**Verification**: post-sweep grep for stale refs (excluding the two carve-outs) returns zero matches.

**Yield**:

- **20 files updated** (63 insertions / 63 deletions; pure 1:1 path rewrites, no semantic content edited)
- **0 files in the protected carve-outs** touched
- **64 stale references** resolved
- **1 reference** intentionally left stale in corpus-ref/737 (documented)

**Findings**

**Finding LPA.1 (path-staleness is the most-mechanical staleness class)**: of the staleness classes the audit can surface (stale irreducibility-claims, stale orphan-claims, stale spinoff-pending claims, path-staleness, coordinate-drift), path-staleness is fully mechanical to detect (grep for old paths) and fully mechanical to resolve (sed). It is the easiest first instance of the audit's value proposition. The harder classes (stale irreducibility-claims) require semantic comparison across locales' Findings and remain LPA-EXT 2+ work. This first execution closes the easy case as a working-discipline demonstration; subsequent rungs require richer reasoning.

**Finding LPA.2 (the 2-tier carve-out is principled, not lazy)**: the sweep preserved two carve-outs that are NOT laziness but apparatus-discipline: `docs/corpus-ref/` is the published-corpus mirror (editing it crosses the apparatus/docs tier-separation that §0 of repository-apparatus.md makes load-bearing); `rusty-js-jit/findings.md` is the canonical append-only ledger (editing prior entries violates Doc 727 §X basin-stability). The audit RECORDS the carve-outs rather than working around them; future readers chasing pre-move paths in those files now have this trajectory as the navigation breadcrumb. Standing recommendation: every audit sweep should produce a per-file disposition (updated / carve-out / protected); silent skipping is incompatible with the audit's claim-coherence telos.

**Status**: LPA-EXT 1 CLOSED. The bilateral-pilot-tier housekeeping is now fully landed (Phase 1 structural move + Phase 2 reference sweep). LPA-EXT 2 (spinoff-chain mapping) and LPA-EXT 3 (positioning-gap detection) remain on the methodology and run on next trigger.

---

## LPA-EXT 2 — spinoff-chain mapping (2026-05-25)

**Trigger**: Keeper directive (Telegram 9808) "continue with ext 2."

**Produced**: `pilots/apparatus/locale-positioning-audit/findings/spinoff-chains.md` — the first chain-map snapshot.

**Survey method**:

1. Grep `pilots/` for explicit markers: "spinoff", "spawned from", "surfaced from", "surfaced by", "opened in response", "nested under", "nested locale".
2. For each hit, walk the cited parent locale's trajectory tail to confirm the spawn-causation arrow.
3. Group findings into chain types (3-tier substrate cascade, multi-sibling spawn, parent→nested rung, apparatus-pilot cascade, matrix fan-out, self-reflexive).

**Snapshot at 2026-05-25**: **7 confirmed chains + 1 self-reflexive locale (LPA)**.

| # | Chain | Type | Tier-span |
|---|---|---|---|
| 1 | LGSS → PPIF → FHNB | 3-tier substrate cascade | lexer → climber → bytecode/runtime |
| 2 | TSR → 11 ts-resolve-* siblings | multi-sibling spawn | TSR-tier (single-tier broad fan-out) |
| 3 | IHI → GPI → IPBR | 3-tier substrate cascade (LeJIT) | interp → bytecode-rewrite (Doc 740/741 instance) |
| 4 | Shape → CMig (nested) | parent→nested rung | shapes-tier sub-workstream |
| 5 | TCC → TXC | apparatus-pilot cascade | parse-parity → execute-parity instruments |
| 6 | PEER → BBND (nested) → corpus-candidate Doc 743 | parent→nested rung→corpus | parser-tier; nested BBND surfaced apparatus-tier corpus draft |
| 7 | full-suite matrix → top-10 batch | matrix fan-out | one matrix read → 10 coordinate-shaped sibling locales |
| 8 | LPA → (self-reflexive) | meta-apparatus | no children yet |

**Findings**

**Finding LPA.3 (chain types are not all alike — each predicts different yield shape)**: the audit surfaced six distinct chain shapes across the 8 instances. 3-tier substrate cascades (Chains 1 + 3) produce the strongest amortization-conjecture corroboration — each tier's named constraint enables the next. Multi-sibling spawns (Chain 2) produce the strongest yield-per-spawn-event but no sequential depth. Parent→nested rungs (Chains 4, 6) are R4-disciplined (the parent's scope was correct, the sub-shape just needed its own seed). The audit's value scales with the engagement's chain count; today's 7 confirmed chains suggest the engagement has crossed from "spawn-per-need" into "spawn-via-chain-causation" as the dominant locale-spawn mode. Standing recommendation: tag new locales at spawn time with the chain-type they belong to (or are likely to anchor); this gives future audits a coordinate to read against, rather than re-deriving chain causation from grep.

**Finding LPA.4 (the 7 confirmed chains corroborate the keeper's amortization conjecture engagement-wide, not just at LGSS→PPIF→FHNB)**: the keeper's amortization conjecture (Telegram 9794) was framed in response to LGSS's specific case. The audit shows the pattern was operating ENGAGEMENT-WIDE before LGSS — TSR's 11-sibling cascade, IHI/GPI/IPBR's 3-tier cascade, Shape's parent→nested. The conjecture's empirical track is older + broader than the LGSS→PPIF→FHNB chain implied; LGSS made the pattern legible at the engagement-discipline tier (via Findings + the cluster-coherence-multiplier prospective doc), but the substrate-engineering tier has been operating this way for the entire session arc. Standing recommendation: the prospective Doc 743 (cluster-coherence multiplier) should cite the 7-chain engagement-wide track, not just LGSS, as its empirical anchor. Update pending keeper review.

**Status**: LPA-EXT 2 CLOSED. The spinoff-chains.md is the first standing output of this rung; refreshed opportunistically per the triggers (new spawn, locale close, full-suite re-categorize). LPA-EXT 3 (positioning-gap detection) remains.
