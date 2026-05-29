---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 84e9dec4d69a8956d532f70ff392cd0a8620a420
target_branch: main
summary: Array.prototype.sort Phase 0 locale spawn and Phase 2 cluster probe
risk_class: apparatus
gates_pre:
  test262_sample: post-EPSUA matrix Array.prototype.sort top cell 25 no-feature rows / 26 total pipeline rows
  build: null
  per_locale:
    array-prototype-sort: not yet founded
gates_post:
  test262_sample: no runtime change; Phase 2 segmentation only
  build: null
  per_locale:
    array-prototype-sort: founded with C4 PASS, 19/26 precise accessor/prototype bucket
---

## Substrate Moves

Commit `84e9dec4d69a8956d532f70ff392cd0a8620a420` lands APS-EXT 0.

- **M** = `array-prototype-sort` locale spawn plus post-EPSUA matrix inspection.
- **T** = top-ranked `Array.prototype.sort` sample cluster: 25 no-feature rows and 26 total pipeline rows.
- **I** = `pilots/array-prototype-sort/{seed.md,trajectory.md}` and refreshed `apparatus/locales/manifest.json`.
- **R** = No runtime substrate edits. Phase 2 C4 passes with 19/26 rows in a coherent precise accessor/prototype side-effect bucket.

## Risk Assessment

This is an apparatus-only probe/spawn commit. Runtime behavior is unchanged. The main risk is coordinate overlap with the older `array-sort-tostring-dispatch` locale; the seed and trajectory explicitly separate the prior object-ToString dispatch closure from the broader current precise sort semantics cluster.

No build or runtime gate is required for this documentation-only spawn. The measurement source is the existing post-EPSUA test262 categorizer output at `pilots/apparatus/test262-categorize/results/2026-05-29/`.

## Composes-With

The proposed Phase 3 move should introduce a sort element record layer in `Runtime::array_proto_sort_via`: accessor-aware `HasProperty`/`Get` collection, distinct present/undefined/absent records, spec-shaped Set/Delete writeback, and no unconditional non-Array array-like length write.
