# Spinoff Chains — LPA-EXT 2 output

Cross-locale chains where one locale's Finding (or chapter-close output) surfaced another locale's spawn. Renders the FCA-amortization stack the keeper's amortization-conjecture predicted (Telegram 9794) at engagement-graph scope.

A **chain** here means: a parent locale's substantive output identified a specific successor coordinate that became its own founded locale. Chains are read parent-to-child along the spawn-causation arrow; the spawn-causation marker is typically a Finding or chapter-close clause naming the successor.

Per LPA seed §Methodology, this rung is **opportunistically refreshed**; the snapshot below is as of 2026-05-25. Future spawns extend the chains; this doc gets re-rendered when a chain extends.

---

## Confirmed chains (parent → child via explicit Finding-or-chapter-close surfacing)

### Chain 1 — LGSS → PPIF → FHNB (today's FCA-amortization chain across 3 substrate tiers)

```
LGSS (lexer-tier)
  L  Finding LGSS.5 + §XI.1.b spinoff-candidate name
  L  ⇒ PPIF (precedence-climber tier, sibling not nested)
       L  Finding PPIF.2 (substrate-WIN, not just structural cleanup)
       L  ⇒ FHNB (bytecode/runtime tier, sibling)
```

- **LGSS** = `pilots/lexer-goal-symbol-selection/` — named the goal-symbol-selection constraint at the lexer/parser boundary
- **PPIF** = `pilots/parser-precedence-in-flag/` — named the [+In]/[-In] grammar parameter at the precedence-climber boundary
- **FHNB** = `pilots/for-head-non-binding-lhs/` — names the AssignmentTarget variant at the bytecode/runtime tier (currently FOUNDED, substrate work pending)

Spawn-causation evidence: LGSS-EXT 3 trajectory entry explicitly names PPIF as "parser-precedence-in-flag spinoff candidate"; PPIF seed §Trigger and §Composes-with explicitly cite LGSS-EXT 3; FHNB seed §Trigger and §Composes-with explicitly cite PPIF-EXT 1's Finding PPIF.2. Each is a Fielding-style constraint accumulation across substrate tiers, with each tier's named constraint making the next tier's named-constraint candidate visible.

This chain is the canonical instance of the **cluster-coherence multiplier** at the engagement-graph scope (per `docs/engagement/prospective/cluster-coherence-multiplier-as-sipe-t-instance.md`). Standing reading: each tier picks up the prior tier's unblocked shape and names the constraint at the right tier to make new correctness possible.

### Chain 2 — TSR → TS-resolve family (historical TSR-tier cascade)

```
TSR (ts-resolve)
  L  Failure-table surfaced multiple TS-construct sub-clusters
  L  ⇒ TRSLS, TRCAPS, TRGC, TRMLE, TROI, TRE  (sibling spawns)
       L  Various class/generics/module-loader/enum/string-literal sub-locales
            (ts-resolve-class-and-param-shapes/, ts-resolve-generics-calls/,
             ts-resolve-module-loader-extension/, ts-resolve-string-literal-safety/,
             ts-resolve-enums/, ts-resolve-type-only-imports/, ...)
```

- **TSR** = `pilots/ts-resolve/` — TypeScript source-language resolver
- **Children**: ts-resolve-* family, 11 sub-locales per CLAUDE.md "TSR-tier work is COMPLETE; remaining execute-parity gap is genuine runtime-substrate territory"

Spawn-causation evidence: TSR's CANDIDATES.md tier-D entries explicitly named the sub-locale candidates; spawns were sequenced after TSR's chapter close lifted parse-success to 37.7% baseline.

### Chain 3 — LeJIT cascade IHI → GPI → IPBR (historical multi-tier cascade per Doc 740/741)

```
IHI (interp-hot-intrinsics)
  L  Closure at IHI-EXT 11 (bytecode-rewrite tier)
  L  ⇒ GPI (interp-getprop-ic)  — sibling, applies rule 13 prospectively
       L  Finding GPI.3 (cost surface shift to for-of envelope)
       L  ⇒ IPBR (iter-protocol-bytecode-rewrite)
            L  Closure at IPBR-EXT 2 with 1 implementation round
```

- **IHI** = `pilots/interp-hot-intrinsics/`
- **GPI** = `pilots/interp-getprop-ic/`
- **IPBR** = `pilots/iter-protocol-bytecode-rewrite/`

Spawn-causation evidence: documented in corpus Doc 740 (Multi-Tier Cascade-Revival) and Doc 741 (the empirical materialization across the four sibling pilots). This is the empirical anchor for standing rule 13 prospective application; chain is the first observed engagement instance of cross-substrate-tier cascade closure.

### Chain 4 — Shape → CMig (nested-locale spawn from Shape-EXT 4 deferred-enrollment)

```
Shape (rusty-js-shapes)
  L  Shape-EXT 4 deferred-enrollment finding (~41 sites bypass shape mechanism)
  L  ⇒ CMig (consumer-migration) — NESTED at pilots/rusty-js-shapes/consumer-migration/
```

- **Shape** = `pilots/rusty-js-shapes/`
- **CMig** = `pilots/rusty-js-shapes/consumer-migration/` (nested per Doc 737 §II multi-rung promotion)

Spawn-causation evidence: CMig-EXT 0 trajectory explicitly names "Sub-workstream of pilots/rusty-js-shapes/ spawned per Shape-EXT 4's deferred-enrollment finding."

### Chain 5 — TCC → TXC (apparatus-pilot cascade)

```
TCC (ts-consumer-corpus, parse-parity measurement)
  L  TCC's first chapter close surfaced 90.1pp gap between parse-parity and execute-parity
  L  ⇒ TXC (ts-execute-corpus, execute-parity measurement) — sibling apparatus-pilot
       L  TXC's execute-parity baseline 5.1% revealed the runtime-substrate territory
```

- **TCC** = `pilots/apparatus/ts-consumer-corpus/`
- **TXC** = `pilots/apparatus/ts-execute-corpus/`

Spawn-causation evidence: TXC-EXT 0 trajectory (TXC seed cites TCC as the measurement instrument it extends). Both are apparatus-pilots per the bilateral-pilot-tier housekeeping (2026-05-25).

### Chain 6 — PEER → BBND (nested-locale spawn from PEER baseline row-coherence)

```
PEER (parser-early-error-residual, top-10 batch spawn from full-suite matrix)
  L  PEER baseline (0/100, 3-of-3 inspected = redeclaration-shape)
  L  ⇒ BBND (block-bound-names-dup) — NESTED at pilots/parser-early-error-residual/block-bound-names-dup/
       L  BBND-EXT 1+2 closure: 76/95 → 95/95 (full-dir close)
            L  Findings BBND.1+2+3+4 + the §IV cluster-coherence-multiplier articulation
                 L  ⇒ corpus-candidate prospective doc (`docs/engagement/prospective/`)
```

- **PEER** = `pilots/parser-early-error-residual/`
- **BBND** = `pilots/parser-early-error-residual/block-bound-names-dup/` (nested)

Spawn-causation evidence: BBND seed §Trigger explicitly cites "PEER baseline surfaced 3/3 inspected fails as block-scope/syntax/redeclaration/* (Finding PEER.1)." BBND's findings.md §IV articulated the five-condition multiplier that subsequently anchored the prospective corpus-candidate Doc 743.

### Chain 7 — full-suite Pin-Art matrix → top-10 spawn batch (fan-out, not chain)

```
test262-categorize/full-suite/results/.../matrix.md
  L  Matrix top-10 coordinate enumeration (LPA-style read across availability partitions)
  L  ⇒ 10-locale spawn-batch (fan-out, not a sequential chain):
        - temporal-availability/ (rank #1, absent-chapter subsystem)
        - ast-bytecode-syntaxerror-cluster/ (rank #2, available-surface)
        - ast-bytecode-wrong-result/ (rank #3)
        - ast-bytecode-missing-method/ (rank #4)
        - parser-early-error-residual/ (rank #5)  ←  spawned PEER, leading to Chain 6 + BBND
        - ast-bytecode-uncategorized-projection/ (rank #6 — apparatus-gap)
        - ast-bytecode-missing-throw-typeerror/ (rank #7)
        - typed-array-wrong-result/ (rank #8)
        - typed-array-missing-method/ (rank #9)
        - spec-builtins-wrong-result/ (rank #10)
```

Spawn-causation evidence: the full-suite Pin-Art matrix (`pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-25-165734-p2/matrix.md`) was the input; each spawned locale's seed cites the matrix coordinate and ranking. This is a one-to-many fan-out (matrix as collective parent); not a sequential chain. PEER (rank #5) became Chain 6's root by spawning BBND.

### Chain 8 — LPA → (self-reflexive, no children yet)

```
LPA (locale-positioning-audit, meta-apparatus)
  L  Founded 2026-05-25 in response to PPIF-EXT 2's §XI.1.b amendment necessity
  L  ⇒ no spawned children yet; runs as opportunistic audit over the locale graph
```

LPA is the audit that produced this very document; it has no spawned children but its findings (LPA.0+, this chain map) inform future amendments to other locales' claims.

---

## Chain shape analysis

| Chain shape | Examples | Observation |
|---|---|---|
| **3-tier substrate cascade (sibling spawns across tiers)** | LGSS→PPIF→FHNB; IHI→GPI→IPBR | The keeper's amortization-conjecture instances; each tier's named constraint enables the next |
| **Multi-sibling-spawn from one parent** | TSR→TRSLS/TRCAPS/TRGC/.../TROI | Failure-table-driven; siblings work in parallel |
| **Parent→nested rung** | Shape→CMig; PEER→BBND | Doc 737 §II promotion; sub-workstream has multi-rung shape |
| **Apparatus-pilot cascade** | TCC→TXC | Measurement-instrument lineage; rare but high-yield |
| **Matrix fan-out** | full-suite matrix→top-10 batch | One coordinate enumeration spawns N siblings; chain depth = 1 |
| **Self-reflexive (no children yet)** | LPA→ | Meta-apparatus loci; expected to fan out only when claim-staleness surfaces |

**Observation (LPA.3)**: the chain types are not all alike. 3-tier cascades produce the strongest amortization claims (each tier's enabling); multi-sibling spawns produce the strongest yield-per-spawn-event but no sequential depth; nested rungs are R4-disciplined (the parent's scope was correct, the sub-shape just needed its own seed). The audit's value scales with the engagement's chain count: today's 7 confirmed chains suggest the engagement has crossed from "spawn-per-need" into "spawn-via-chain-causation" as the dominant mode.

**Open chain candidates** (not yet confirmed as parent→child causation):

- Today's 7 closed parser-arc locales (FHAPV / FORA / SBAP / FHLA / FAOF / ALTA / RPDF / ARTC) are sequential rungs in the SyntaxError curated cluster, not strictly spinoffs across locales. They share a parent (the SyntaxError cluster sample) but each is its own locale; this is more "sequential rungs in one cluster's progression" than "parent-finding-spawned-child."
- The 9 sibling top-10 batch locales (excluding PEER which became Chain 6) have not yet emitted findings that surface children. They may become roots of future chains as their substrate work lands.

---

## How this doc gets refreshed

Per LPA seed §Triggers (opportunistic, not scheduled):

- **After any new locale spawn**: check whether the spawn was caused by a Finding in another locale; if yes, extend the appropriate chain.
- **After any locale CLOSES**: check whether the close emitted spinoff-candidate findings that have since become locales; render the chain backward.
- **After any full-suite categorize re-run**: check for matrix fan-outs (one matrix top entry spawning multiple coordinate-shaped locales).

The doc is append-only at the chain level: chains are added or extended, but chains' historical text is preserved when superseded. Per LPA seed §Carve-outs, this audit produces text findings, not substrate edits.

---

*Snapshot 2026-05-25 post-LGSS/PPIF/FHNB. 7 confirmed chains + 1 self-reflexive locale (LPA). Re-render on next chain extension or new chain emergence.*
