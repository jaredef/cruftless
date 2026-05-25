# scripts/locales — Locale discovery + manifest

Per **[Doc 737](../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md)**: a locale is a directory containing `seed.md` + `trajectory.md` (Pin-Art per Doc 581). Its coordinate is its filesystem path relative to the locale root (`pilots/` by default). Its tag is `L.<coordinate-dot-segments>`.

## Files

- `discover.sh` — walks the locale root, finds every `seed.md`, emits `manifest.json` with `{coord, tag, parent, scope, depth, rung_count, status}` per entry. Warns on directories that have `trajectory.md` but no `seed.md` (naming drift).
- `manifest.json` — generated; the structural read of the locale tree.
- `README.md` — this file.

## Usage

```sh
# Generate the manifest
./apparatus/locales/discover.sh

# Or to a custom path
./apparatus/locales/discover.sh /tmp/out.json

# Different locale root
LOCALE_ROOT=$PWD/some/other/root ./apparatus/locales/discover.sh
```

## Tag convention

Every `seed.md` carries a `**Locale tag**: \`L.<coord>\`` annotation on the second line (after the H1 title). The tag is the canonical identifier used in commits, the manifest, and any tool that needs to address a locale without using the full path.

Examples (from this engagement):

```
L.diff-prod
L.rusty-js-caps
L.rusty-js-esm
L.rusty-js-esm.deviations.arktype     ← nested
L.rusty-js-ir
L.rusty-js-jit
L.rusty-js-pm
L.tls
L.web-crypto
```

## Adding a new locale

1. Pick a coordinate. Top-level: `<scope>`. Nested: `<parent-coord>/<scope>`.
2. `mkdir pilots/<coord>` and write `seed.md` + `trajectory.md`. The seed.md's first lines:
   ```markdown
   # <scope> — Resume Vector / Seed

   **Locale tag**: `L.<coord-dot-form>` (per [Doc 737](...))

   **Status as of <date>**: ...
   ```
3. For a nested locale, the seed.md should also declare:
   ```markdown
   **Parent locale**: [<parent-scope>](<relative-path-to-parent-seed>). Spawned <date> when ...
   ```
4. Re-run `discover.sh`. The new locale appears in the manifest with depth = number-of-segments.

## Rung-tier pre-filing

A trajectory.md rung that's queued but warrants its own future nested locale can pre-file the coordinate. Convention: end the rung's description with `**Pre-filed coordinate**: \`L.<future-coord>\``. When the rung escalates and the work spawns, materialize the named coordinate as a real `seed.md + trajectory.md` pair.

Diff-prod's Rungs 6/7/8 are the first explicit pre-filings (async-promise / eval-const / map-set-ops); Rungs 6 and 8 already materialized as inline substrate fixes (no separate locale needed), while Rung 7 remains queued.

## Composition

Locales compose both across substrate tiers (per Doc 733's fractal seed-and-trajectory recurrence) and within tiers (per Doc 737's locale-as-coordinate recognition). The manifest is total over the locale tree — every locale has exactly one coordinate, and every coordinate is at most one locale.

The manifest is the locale-tier analog of `host/tools/dag-coordinates.json` (Doc 728 §IX): a structural read of the apparatus's tag namespace, available for cross-tool consumption (CI, dashboards, corpus-tier rollups).
