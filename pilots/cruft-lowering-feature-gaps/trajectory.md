# cruft-lowering-feature-gaps — Trajectory

## CLFG-EXT 0 — parent locale founded and current reason partition captured (2026-05-28)

**Trigger**: keeper selected the lowering-feature-gap candidate after the
top-level arc survey and directed the work to proceed under the apparatus
methodology.

**Orientation loaded**:

- `AGENTS.md`
- `apparatus/docs/repository-apparatus.md`
- `apparatus/docs/predictive-ruleset.md`
- `apparatus/docs/standing-rule-13-prospective-application.md`
- `apparatus/docs/agent-feedback-schema.md`
- `apparatus/docs/arc-as-coordinate.md`
- `apparatus/locales/manifest.json`
- `apparatus/locales/CANDIDATES.md`
- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`
- `pilots/apparatus/locale-positioning-audit/findings/repartition-audit-algorithm.md`

**Matrix input**:

`pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-27-161641/interpreted.jsonl`

The file contains seven malformed JSONL rows with embedded control characters,
so the founding aggregation used a tolerant line-by-line JSON parser and
ignored malformed rows. The selector was:

```text
projection == "availability/missing-lowering-feature"
  OR reason starts with "compile:"
```

**Current partition**:

| Count | Reason |
|---:|---|
| 68 | `compile: super reference outside of a class` |
| 18 | `compile: bare \`super\` reference is only valid as \`super(...)\` or \`super.method(...)\`` |
| 8 | `compile: super reference in a class with no \`extends\` clause` |
| 12 | `compile: for-in with destructure head not yet supported` |
| 4 | `compile: update on non-identifier non-member target not yet supported` |
| 2 | `compile: super(...) outside of a class` |
| 1 | `compile: complex assignment target not yet supported` |

**Founding decision**:

This is an arc-shaped parent locale, not an immediate substrate-edit locale.
`super` dominates the current coordinate, but the for-in, update-target, and
complex-assignment tails are separate lowering mechanisms. The parent locale's
job is to run Rule-23 baseline inspection, then spawn or redirect the first
coherent child.

**Artifacts added**:

- `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/arc.md`
- `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/log.md`
- `pilots/cruft-lowering-feature-gaps/seed.md`
- `pilots/cruft-lowering-feature-gaps/trajectory.md`
- `pilots/cruft-lowering-feature-gaps/exemplars/exemplars.txt`
- `pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh`

**Next**: run `pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh`
and use the actual current failures to choose either a `super` child locale or
a redirect for rows that are parser/early-error residue rather than true
bytecode lowering.

## CLFG-EXT 1 — Rule-23 baseline confirms compiler-residue shape and first child (2026-05-28)

**Probe**:

```sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh
```

**Result**:

```text
CLFG exemplars: PASS=0 FAIL=32 SKIP=0 NOJSON=0 / 32
```

**Failure distribution**:

| Exemplars | Reason |
|---:|---|
| 10 | `compile: super reference outside of a class` |
| 6 | `compile: bare \`super\` reference is only valid as \`super(...)\` or \`super.method(...)\`` |
| 4 | `compile: super reference in a class with no \`extends\` clause` |
| 2 | `compile: super(...) outside of a class` |
| 6 | `compile: for-in with destructure head not yet supported` |
| 3 | `compile: update on non-identifier non-member target not yet supported` |
| 1 | `compile: complex assignment target not yet supported` |

**Read**:

The Rule-23 baseline confirms the categorizer's projection is not a runner
blur: all exemplars still fail before execution with compiler diagnostics.
The current coordinate should split. The first child locale should own
`super` reference/call/member lowering plus context-frame propagation, because
it covers 96/113 matrix rows and 22/32 exemplars. For-in destructuring heads,
invalid update targets, and complex assignment targets remain separate tails
to triage after the `super` child is either spawned or redirected.

**Follow-on**: collision check found no active exact owner, so
`super-reference-lowering/` was spawned as a nested child locale. The next move
belongs there: SRL-EXT 1 child baseline classification.

## CLFG-EXT 2 — child super-lowering progress checkpoint (2026-05-28)

`super-reference-lowering/` has now closed four low-collision subclusters:

- SRL-EXT 2: object-literal HomeObject `super` for computed method/accessor
  rows, `+3`.
- SRL-EXT 3: object-method `super[key]` PutValue base/key ordering for
  compound assignment and update rows, `+2`.
- SRL-EXT 4: no-extends class SuperProperty base fallback and key coercion,
  `+4`.
- SRL-EXT 5: delete SuperReference evaluation and ReferenceError routing,
  `+4`.

Child suite checkpoint:

```text
CLFG exemplars: PASS=13 FAIL=9 SKIP=0 NOJSON=0 / 22
```

Remaining `super` rows are direct-eval context capture and
derived-constructor direct-eval `super()` capture. Direct eval should stay
deferred behind the active eval-environment arc unless that arc is explicitly
joined.

## CLFG-EXT 3 — second sibling-child landing (2026-05-28)

`for-in-destructure-head/` sub-locale spawned + closed in one rung (FIDH-EXT 1
at 572fa682). All 6 target exemplars flip FAIL → PASS via for-in handler
mirroring the for-of destructure-head substrate pattern (compiler.rs:2140–
2395). See `pilots/cruft-lowering-feature-gaps/for-in-destructure-head/
trajectory.md` for the FIDH-EXT 0 + FIDH-EXT 1 record.

## CLFG-EXT 4 — update-target cover-id lowering (2026-05-28)

Per keeper directive Telegram 10233 ("continue the arc"). Third tail-cluster
close; folded into parent trajectory rather than spawned as a sub-locale
because the cluster is small (3 exemplars / 4 matrix rows) and the substrate
move is a single-line unwrap.

**Phase 1 (Spawn)**:
- **M** = `(x)++`, `(x)--`, `++(x)`, `--(x)` and analogous parenthesized
  identifier or member targets.
- **T** = the compiler unwraps the ParenthesizedExpression cover and lowers
  to the underlying target's update path per ECMA-262 §12.4.1 + §13.5.1.1 +
  §13.15.2 RS:IsValidSimpleAssignmentTarget (parens preserve target
  validity).
- **I** = `compile_update` (compiler.rs:6665). Add a `Parenthesized` unwrap
  before the structural match.
- **R** = lattice with the broader parenthesized-cover-grammar handling
  already present at compiler.rs:518, 688, 3716, 3984, 5973, 6008, 6258
  (every other site already unwraps; `compile_update` was an omission, not
  a deliberate restriction).
- **Observability**: ordinary (test262 assertion).

**Phase 2 (Baseline-inspect)**: all 3 parent-list exemplars (postfix-
decrement, postfix-increment, prefix-decrement target-cover-id) fail with
the same `compile: update on non-identifier non-member target not yet
supported` diagnostic at compiler.rs:6801. Adjacent prefix-increment
exemplar (not on parent list) shares the diagnostic; would also flip.

**Substrate** (3 LOC inserted at compiler.rs:6671):

```rust
if let Expr::Parenthesized { expr, .. } = argument {
    return self.compile_update(span, operator, expr, prefix);
}
```

Recurs into self with the unwrapped expression; handles nested parens
naturally.

**Yield**:
```text
PRE-CLFG-EXT 4: 3 target exemplars FAIL (parent list) + 1 adjacent FAIL
POST-CLFG-EXT 4: 3 target exemplars PASS + 1 adjacent PASS
```
Parent CLFG exemplar suite: 19/32 → 26/32 (+7). Of the +7, 3 are
attributable to this rung's parenthesized fix; the other +4 surfaced
when the rebase brought in `4f3bd525 "Thread direct eval super context"`
which partially closed the super-direct-eval cluster at the compile tier
(remaining 5 super-direct-eval cells now fail at runtime rather than
compile, indicating eval-environment threading is partial).

**Gates** (all unchanged): TAMM 82/100, TAWR 63/100, diff-prod 61/51,
build clean, sanity intact.

**Tag**: `cluster-update-target-cover-id-parenthesized-unwrap-4`.

**Finding** (none required): the omission is the canonical instance of a
substrate location that should have followed the broader codebase pattern
(every other expression handler already unwraps Parenthesized) but didn't.
The fix is to apply the established pattern at the omitted site. No new
standing rule warranted; the substrate cross-reference (compiler.rs:518,
688, etc.) is the implicit precedent.

**Phase 6 (deferral emission)**: no new deferrals. Remaining CLFG parent
tail-clusters: super-direct-eval cells (5; runtime-tier work after 4f3bd525
partial close — defer per existing super-deferred-behind-eval-environment-
arc policy), complex-assignment-target (1; would be CLFG-EXT 5 if pursued,
edge case in compound short-circuit NamedEvaluation).

**Status**: CLFG-EXT 4 CLOSED. Parent CLFG locale's three tail-clusters
named in CLFG-EXT 0 baseline are now: super (super-reference-lowering child
closed + super-direct-eval residual), for-in destructure (FIDH-EXT 1
closed), update-target (CLFG-EXT 4 closed). One tail-cluster remains
(complex-assignment-target, 1 cell).

## CLFG-EXT 5 — complex-assignment-target close: parenthesized + NamedEvaluation in compound short-circuit (2026-05-28)

Per keeper directive Telegram 10235. Final originally-named tail-cluster of the
parent CLFG locale.

**Phase 1 (Spawn)**:
- **M** = `(a) ??= function(){}` and analogous `(a) ||=` / `(a) &&=` with
  parenthesized identifier targets; also bare `a ??= function(){}` etc. where
  NamedEvaluation should apply per ECMA-262 §13.15.4 step 1.f.
- **T** = the compiler unwraps the ParenthesizedExpression cover for compound
  short-circuit assignments. For bare-identifier targets, threads the
  identifier name as a NamedEvaluation hint to the value expression so
  anonymous function values receive the target's name. For parenthesized
  targets, suppresses NamedEvaluation per spec (parens disqualify).
- **I** = `compile_logical_assign` (compiler.rs:6463). Three changes:
  1. Add Parenthesized unwrap at the top routing to a new
     `compile_logical_assign_no_named_eval` variant.
  2. In the Identifier arm (both `&&=`/`||=` and `??=` branches), replace
     `self.compile_expr(value)` with
     `self.compile_expr_with_name_hint(value, Some(name))`.
  3. New `compile_logical_assign_no_named_eval` function: same lowering
     shape as `compile_logical_assign` but passes `None` as the name hint,
     so NamedEvaluation does not apply.
- **R** = lattice with CLFG-EXT 4 (same Parenthesized-unwrap pattern at a
  sibling expression handler); DAG ↑ `compile_expr_with_name_hint` helper
  (line 3642) already used for `let a = ...` initializers.

**Substrate** (~70 LOC in `pilots/rusty-js-bytecode/derived/src/compiler.rs`):
Parenthesized unwrap + name-hint threading + parallel no-NamedEval variant
function.

**Yield**:
```text
PRE-CLFG-EXT 5:  staging/sm/expressions/short-circuit-compound-assignment-anon-fns.js FAIL
POST-CLFG-EXT 5: PASS
```
Parent CLFG exemplar suite: 26/32 → 27/32 (+1).

**Direct probe verifies all six sub-assertions**:
- `a ??= function(){}` → `a.name === "a"` ✓
- `(a) ??= function(){}` → `a.name === ""` ✓
- `a ||= function(){}` → `a.name === "a"` ✓
- `(a) ||= function(){}` → `a.name === ""` ✓
- `a &&= function(){}` → `a.name === "a"` ✓
- `(a) &&= function(){}` → `a.name === ""` ✓

**Gates** (all unchanged): TAMM 82/100, TAWR 63/100, diff-prod 61/51, build
clean, sanity intact.

**Tag**: `cluster-complex-assignment-target-paren-and-namedeval-5`.

**Standing observation (not a new rule)**: the apparent "1-cell" yield
understates the substrate amortization. NamedEvaluation in compound short-
circuit assignments is now functional engagement-wide; every future test or
consumer-app that relies on `obj.method ||= function(){}` getting the
method's name will benefit silently. The test262 sample's curated
representative set may not exercise this directly; the full test262 surface
likely has more cells under `availability/missing-lowering-feature` or
`value-semantics/wrong-result` that this rung incidentally addresses.

**Phase 6 (deferral emission)**: no new deferrals. The parent CLFG locale's
three originally-named tail-clusters are now exhausted (super: super-
reference-lowering child + 4f3bd525 partial + 5 super-direct-eval runtime
cells DEFERRED per super-deferred-behind-eval-environment-arc; for-in
destructure: FIDH-EXT 1 closed; update-target: CLFG-EXT 4 closed;
complex-assignment-target: this rung closed). Parent locale telos
substantially achieved; remaining residual is the deferred-by-policy
super-direct-eval runtime cluster.

**Status**: CLFG-EXT 5 CLOSED. Parent locale ready for chapter-close
declaration pending the deferred super-direct-eval runtime cluster's
disposition under the eval-environment arc.
