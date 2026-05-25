#!/usr/bin/env bash
# Doc 737 §IV operational artifact — walk the locale root, find every
# seed.md, emit a structured manifest.
#
# A "locale" is a directory containing seed.md + trajectory.md per the
# Pin-Art discipline (Doc 581). Its coordinate is its path relative to
# the locale root. Its tag is `L.<coordinate-dot-segments>`.
#
# Usage:
#   ./discover.sh                   # writes manifest.json to script dir
#   ./discover.sh /tmp/out.json     # write to specified path

set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
LOCALE_ROOT="${LOCALE_ROOT:-$ROOT/pilots}"
OUT="${1:-$HERE/manifest.json}"

python3 - "$LOCALE_ROOT" "$OUT" <<'PY'
import json, os, re, sys

locale_root, out_path = sys.argv[1], sys.argv[2]
locale_root = os.path.realpath(locale_root)

def find_locales(root):
    """Walk root, yield each directory holding a seed.md."""
    for dirpath, dirnames, filenames in os.walk(root):
        if "seed.md" in filenames:
            yield dirpath
        # Don't descend into node_modules etc.
        dirnames[:] = [d for d in dirnames if d not in (
            "node_modules", "target", ".git", "derived"
        )]

def coord_of(path):
    rel = os.path.relpath(path, locale_root)
    return rel.replace(os.sep, "/")

def tag_of(coord):
    return "L." + coord.replace("/", ".")

def parent_of(coord):
    parts = coord.split("/")
    if len(parts) == 1: return None
    return "/".join(parts[:-1])

def first_status_line(seed_path):
    """Pull the first **Status...** line from seed.md, if present."""
    try:
        with open(seed_path) as f:
            for line in f:
                m = re.match(r"\*\*Status.*?\*\*:\s*(.+)$", line.strip())
                if m:
                    return m.group(1).strip()
    except Exception:
        pass
    return None

def rung_count(trajectory_path):
    """Count rung headers in trajectory.md. Matches:
       - `## Rung-N` / `## Move-N` / `## Round-N` / `## Stage-N`
       - `## <PILOT>-EXT N` (the engagement's convention: HI-EXT, OSR-EXT,
         VD-EXT, TL-EXT, Shape-EXT, JSF-EXT, CMig-EXT, CharCode-EXT, Φ-EXT,
         etc.). Optional lowercase suffix on the number (e.g., 5b, 5c).
         `\\w+` matches Unicode word chars (Φ/Ψ/Σ/Τ etc. for Greek pilots).
       - `## §<roman>` (deviation-pipeline section headers per Doc 730 §XII-§XVII)."""
    if not os.path.isfile(trajectory_path):
        return 0
    try:
        with open(trajectory_path) as f:
            text = f.read()
        return len(re.findall(
            r"^## (?:(?:Rung|Move|Round|Stage)-\d+|\w+-EXT[\s-]\d+[a-z]?|§[IVX]+\b)",
            text, re.MULTILINE | re.UNICODE,
        ))
    except Exception:
        return 0

def has_trajectory(dirpath):
    return os.path.isfile(os.path.join(dirpath, "trajectory.md"))

def has_canonical_pair(dirpath):
    return os.path.isfile(os.path.join(dirpath, "seed.md")) and has_trajectory(dirpath)

# Locales considered valid for the manifest: must have a seed.md.
# We also note locales that have a trajectory.md but no seed.md (naming
# drift — pipeline.md instead, etc.). Those are surfaced as warnings.
locales = []
warnings = []

for path in sorted(find_locales(locale_root)):
    coord = coord_of(path)
    tag = tag_of(coord)
    parent = parent_of(coord)
    status = first_status_line(os.path.join(path, "seed.md"))
    rungs = rung_count(os.path.join(path, "trajectory.md"))
    if not has_trajectory(path):
        warnings.append({
            "coord": coord, "tag": tag,
            "warning": "seed.md present but trajectory.md missing",
        })
    locales.append({
        "coord": coord,
        "tag": tag,
        "parent": tag_of(parent) if parent else None,
        "scope": coord.split("/")[-1],
        "depth": coord.count("/") + 1,
        "rung_count": rungs,
        "status": status,
    })

# Surface drift: directories under locale_root with trajectory.md but no
# seed.md (suggests pipeline.md naming or similar).
for dirpath, dirnames, filenames in os.walk(locale_root):
    dirnames[:] = [d for d in dirnames if d not in ("node_modules","target",".git","derived")]
    if "trajectory.md" in filenames and "seed.md" not in filenames:
        coord = coord_of(dirpath)
        # Identify alternative naming (pipeline.md is the known drift).
        alt = next((n for n in ("pipeline.md", "design.md", "README.md") if n in filenames), None)
        warnings.append({
            "coord": coord, "tag": tag_of(coord),
            "warning": f"trajectory.md present without seed.md (found: {alt or 'unknown'})",
        })

manifest = {
    "generated_by": "apparatus/locales/discover.sh",
    "doc_reference": "Doc 737 (The Locale as Coordinate)",
    "locale_root": locale_root,
    "totals": {
        "locales": len(locales),
        "top_level": sum(1 for l in locales if l["depth"] == 1),
        "nested": sum(1 for l in locales if l["depth"] > 1),
        "warnings": len(warnings),
    },
    "locales": locales,
    "warnings": warnings,
}

with open(out_path, "w") as f:
    json.dump(manifest, f, indent=2)

print(f"wrote {out_path}")
print(f"  {manifest['totals']['locales']} locales "
      f"({manifest['totals']['top_level']} top-level, "
      f"{manifest['totals']['nested']} nested), "
      f"{manifest['totals']['warnings']} warnings")
PY
