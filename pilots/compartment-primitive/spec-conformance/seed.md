---
locale: compartment-primitive/spec-conformance
coord: pilots/compartment-primitive/spec-conformance
parent: pilots/compartment-primitive
status: SPAWNED
opened: 2026-05-27
---

# compartment-primitive / spec-conformance

Sub-locale of `compartment-primitive` spawned by arc `2026-05-27-compartment-spec-conformance` per keeper directive Telegram 10043.

## Telos

Close the nine user-observable spec-conformance gaps named at Telegram 10041/10042 between Cruft Compartments (Doc 743 P-C) and the TC39 Compartments proposal (Stage 1, frozen 2025-12-01) + Doc 736 capability-passing discipline. Each gap is named as a factor with a falsifier probe; each factor maps to a single-rung CPF-EXT substrate move.

The locale is a sub-locale (not a top-level locale) because the substrate target — `intrinsics.rs::install_compartment` + the realm machinery — is the same one that `pilots/compartment-primitive/` already owns. The sub-locale carves out the conformance-specific probes + landings within that ownership.

## Apparatus

- **Probe set at `probes/factor-N-*.js`** (one probe per factor; 9 probes total at locale open). Authored at the locale's founding rung; each probe runs against the current substrate and reports REFUTED/HOLDS.
- **diff-prod 42/42** standing gate.
- **test262-sample ≥86.6%** parity gate.
- **P-C probe re-run** at each rung land to confirm no regression on prior closure.

## Methodology

1. Founding rung: author all nine probes, run them, capture REFUTED/HOLDS in trajectory.
2. Per-rung landing: order per arc.md table (factor 3, 8, 2, 4, 7, 6, 1, 5, 9). Each rung addresses one factor's REFUTED probe with a single substrate move. Sweep verification. §XIII recurrence handling if a regression surfaces.
3. Arc close: per arc's close-condition (all 9 probes hold or keeper-accepted residuals).

## Carve-outs

- The factor-9 substrate cost is undetermined; if the realm-arena work exceeds R4 single-rung scope, it becomes its own locale.
- Factor-1 (hook API) is the largest single surface; may split into hook-protocol + per-hook-implementation sub-rungs.
- Factor-6 (cross-realm Error identity) ties into RS-EXT 3+ work that has its own prospective locale; coordination at land time.

## Composes-with

- Parent: `pilots/compartment-primitive/`
- Sibling: arc 2026-05-27-compartment-primitive-audit-fix (closed; CPF-EXT 1-4)
- Doc 743 (Cruft Compartments)
- Doc 736 (capability-passing)
- TC39 Compartments proposal
- ARC.MR.4 standing rec (formalization-before-implementation discipline)
- ARC.AF.2 standing rec (probe set in same trajectory step as articulation enables zero §XIII recurrences)

## Resume protocol

1. Read this seed + trajectory.md tail.
2. If probe set not yet authored, write it as the founding rung.
3. Re-run probe set; identify next REFUTED factor.
4. Apply the corresponding CPF-EXT-N rung; verify probe + gates.
5. Update trajectory at landing time.
