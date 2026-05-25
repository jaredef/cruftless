# ecmascript-parity-shared-upstream-arc — Trajectory

## EPSUA-EXT 0 — workstream founding (2026-05-25)

**Trigger**: keeper directive after T262C-EXT 2 chapter close: "Before doing so, close the chapter in the current locale and open this new one and project the resume vector in the trajectory.md". Succeeds T262C's first ECMAScript-parity-arc round (eight sub-locales, +300 PASS, 0 regressions, runnable rate 77.6% → 80.6%).

**Strategic framing**: T262C-EXT 2 produced two engagement-grade findings that scope this successor arc:
- **Finding T262C.4** identifies five shared-upstream constraints accounting for ~340 of the post-ASD 1416 fails (~24%; ~7.5× leverage vs mutually-exclusive long-tail). The arc operationalizes these.
- **Finding T262C.5** observes that the Doc 740 multi-tier closure became the engagement-default discipline mid-arc, producing 7 consecutive zero-regression cycles. This arc carries that discipline forward as C1.

**Pre-spawn Rule 11 5-axis check** (arc-tier; per-sub-locale checks deferred to each sub-locale's founding):
- (A1) component A/B — N/A; arc-tier coordinator
- (A2) op-set coverage — per sub-locale
- (A3) value-domain — per sub-locale
- (A4) locals-marshaling — per sub-locale
- (A5) emission-shape — per sub-locale
- (A6 — proposed engagement-tier extension per PPA seed): spec-section enumeration for the construct family. Each sub-locale's seed must name the spec sections it touches; failure to enumerate is the Rule 11 mirror for ECMAScript-parity sub-locales.

**Six Pred-epsua.* + discipline falsifier** (per seed §I.3).

**Founding artefacts**: seed.md + this trajectory.md + scaffolded dirs. **EPSUA-EXT 1** (`host-262-shim`, constraint #3, smallest-blast-radius first) is the next sub-locale founding per the §I.1 ordering and the keeper's authorization queue.

### Resume vector projection (carries forward through the arc)

The five queued sub-locales in dependency order:

| Order | Sub-locale dir | Constraint | Projected cascade | Status |
|---:|---|---|---:|---|
| 1 | `pilots/host-262-shim/` | #3 ($262 host hooks) | ~38 | queued |
| 2 | `pilots/iterator-close-on-abrupt/` | #4 (IteratorClose §7.4.9) | ~25 | queued |
| 3 | `pilots/parser-permissiveness-audit-extensions/` | #5 (escaped-of, dup-params, for-in-const, for-in-destr) | ~50 | queued |
| 4 | `pilots/strict-mode-parser-tracking/` | #2 (yield/let/static reserved per mode) | ~80 | queued |
| 5 | `pilots/host-method-prologue-discipline/` | #1 (RequireObjectCoercible + brand-check) | ~150 | queued |

Each sub-locale follows the EXT 0 (founding) → EXT 1 (multi-tier closure + exemplar verify + chapter close) pattern per C1+C2.

### Per-sub-locale founding template (carry-in for whoever picks up EPSUA-EXT 1)

When founding the next sub-locale:
1. `mkdir -p pilots/<name>/`
2. Write `seed.md` with: telos, apparatus, methodology, falsifiers (5 + 1 discipline), carve-outs, composes-with (must include EPSUA, T262C, this trajectory, the relevant ECMA-262 §, Doc 740, Doc 742, standing-rule-13-prospective).
3. Write `trajectory.md` with EXT 0 founding entry (rule 11 5-axis check + Pred-*).
4. `bash scripts/locales/discover.sh` and commit the refreshed manifest with the seed+trajectory.
5. Implement per the multi-tier R identified pre-implementation (Doc 740 + Finding T262C.5 default discipline).
6. Exemplar-verify before full-sweep (C2). Include regression-probe on adjacent previously-passing tests (C3).
7. Update this trajectory with the sub-locale's chapter-close summary (cumulative-vs-projected ratio per C5).

### Status

EPSUA-EXT 0 CLOSED at founding. Next: keeper authorization to spawn EPSUA-EXT 1 (`host-262-shim`).
