# parser-early-error-residual — Trajectory

## PEER-EXT 0 — founding + exemplar suite + baseline-TBD (2026-05-25)

**Trigger**: Top-10 spawn batch per keeper directive after canonical
full-suite Pin-Art zoom-out. This is rank #5 of the matrix
(809 fails) and is parser-tier work continuing today's arc.

**Apparatus established**:

- `exemplars/exemplars.txt` — 100 stratified-sample paths.
- `exemplars/run-exemplars.sh` — runner.
- `exemplars/pool-size.txt`, `exemplars/family-breakdown.txt` —
  inventory.

**Baseline (2026-05-25)**: **PASS=0, FAIL=100 (0.0%)**.

Surface-family breakdown of fails:

| Count | Family |
|---:|---|
| 36 | language/statements |
| 36 | language/expressions |
| 12 | language/literals |
| 6 | language/line-terminators |
| 4 | language/global-code |
| 4 | language/block-scope |
| 2 | language/import |

**Sample-vs-arc note**: this 100-exemplar slice was drawn from the
canonical full-suite run (2026-05-25 16:57 local) which **predates**
today's parser arc (FHAPV / FORA / SBAP / FHLA / FAOF / ALTA / RPDF /
ARTC, +32 closures). None of today's closures are represented in this
sample. The true residual against the full 809-pool after today's work
is approximately 777, not 809; a fresh full-suite categorize will
confirm. The 100-exemplar baseline is a clean reading of the
*not-yet-attempted* surface inside the cluster.

**Findings**

**Finding PEER.1 (top sub-cluster: BoundNames duplication)**: first 3
sampled fails are all `language/block-scope/syntax/redeclaration/*` —
duplicate-name binding within a block (§13.2.1 LexicallyDeclaredNames
duplicate check across let / const / function / async function /
generator / async generator / class). Next substrate-rung candidate:
extend cruft's parser-side duplicate-binding check to cover the
generator / async / class cross-product in block scope. Heuristics §V
row-coherence: 3 of 3 inspected share one mechanism; mechanism is
shared, candidate qualifies for substrate move.

**Status**: PEER-EXT 0 founding closed; baseline pinned. PEER-EXT 1
candidate identified per heuristics §V row-coherence; awaiting keeper
direction or sibling-locale baseline runs before kicking off.
