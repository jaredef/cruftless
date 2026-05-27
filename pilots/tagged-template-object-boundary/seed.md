# tagged-template-object-boundary seed

Status: spawned 2026-05-27

## Telos

Close the tagged-template object boundary where AST-to-bytecode lowering must preserve template literal site identity, raw/cooked segment data, substitution ordering, and the runtime allocation/cache/freeze semantics required by TemplateStringsArray.

This locale treats the visible `.raw` failures as a boundary symptom: the parser has accepted a tagged-template form, but the directive has not yet survived lowering into the call artifact as the first tag argument with the correct object shape.

## Coordinate

- Resolver: `ast-to-bytecode/language-lowering`
- Rung: `E2/internal-method:execution-semantics`
- Axis: `R/ast-to-bytecode`
- Availability: `available-surface`
- Primary cut kinds: `widening/value-semantics`, `successor/semantic-refinement`, `widening/abrupt-completion`
- Primary projection: template-object allocation, `.raw` descriptor shape, frozen object behavior, and call argument ordering

## Apparatus Basis

This spawn follows the high-level cluster reading and the LPA partition:

- `apparatus/locales/CANDIDATES.md` marks `tagged-template-object-boundary` as spawn-ready.
- `pilots/apparatus/locale-positioning-audit/findings/language-lowering-partition.md` identifies it as a fresh Tier M boundary candidate.
- `pilots/apparatus/locale-positioning-audit/findings/spec-boundary-integrity-audit.md` records the LPA-EXT 9 baseline over `language/expressions/tagged-template/`.
- The full-suite matrix shows language lowering as the largest available-surface cluster, but class/private surfaces have recent ownership; this locale chooses the clean boundary candidate to avoid collision.

## Baseline Read

LPA-EXT 9 reported a 27-fixture tagged-template set with 12 pass, 13 fail, and 2 no-json/abort rows.

Failure families:

- Template cache identity by source site, function, top-level, and realm.
- Direct eval / realm context around tag binding.
- Template object allocation, template-map behavior, and constructor shape.
- Frozen template object and strict write behavior.
- Illegal escape cooked value should materialize as `undefined`.
- TCO tagged-template call/member aborts.

Initial carve-out: tail-call optimization rows (`tco-call.js`, `tco-member.js`) are tracked but not used to steer the first substrate rung.

## Invariants

- Parser/lowering must carry both cooked and raw segments to the call boundary.
- The tag call receives a template object as argument 0 and substitutions in source order after it.
- The template object is array-like, frozen, and has a `.raw` property with the required descriptor and frozen raw array.
- The same template site reuses the same template object for a given realm/site identity, while distinct sites remain distinct.
- Illegal escape sequences in tagged templates produce `undefined` cooked entries without rejecting the tagged-template form.
- Eval and realm cases must preserve the correct tag binding context rather than hiding failures behind `$262` or unbound tag errors.

## Falsifiers

- If the first baseline is dominated by parser rejection or raw-source lexing, redirect to `string-literal-and-escape-conformance` or a template-literal lexical locale before touching runtime object construction.
- If `.raw` exists but freeze/cache identity still fails, the locale remains runtime object semantics rather than parser/lowering.
- If only TCO rows remain after object construction/cache/freeze, close this locale and hand TCO to a call-lowering locale.
- If direct-eval lexical capture dominates independently of template object construction, split it toward `direct-eval-lexical-capture` rather than fusing eval semantics into this locale.

## Trajectory

1. Reconstruct and run the 27-fixture tagged-template baseline locally.
2. Close object construction and call argument ordering.
3. Close `.raw` descriptor and freeze semantics for both cooked and raw arrays.
4. Close template-site cache identity.
5. Re-evaluate eval/realm and illegal escape residuals.
6. Leave TCO tagged-template aborts as a terminal carve-out unless they become the only remaining rows.

## Resume Rule

On resume, read this seed, `trajectory.md`, and the current `exemplars/` outputs before modifying engine code.
