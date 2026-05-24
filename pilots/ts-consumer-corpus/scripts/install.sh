#!/usr/bin/env bash
# TCC-EXT 1: corpus installer.
#
# For each package listed in manifest/packages.json:
#   1. Fetch the tarball from registry.npmjs.org (resolves <name>/<version>
#      to the published tgz URL via the registry metadata).
#   2. Extract .ts files (excluding *.d.ts) into fixtures/<name>/.
#   3. Record per-file sha256 + the package version into
#      manifest/file-hashes.json for reproducibility verification.
#
# Idempotent: re-running with the same manifest produces the same
# fixtures + the same file-hashes record. A subsequent measurement run
# can verify reproducibility by re-computing hashes.
#
# Output: fixtures/<package>/<rel-path>.ts + manifest/file-hashes.json
set -euo pipefail

PILOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$PILOT_DIR/manifest/packages.json"
FIXTURES="$PILOT_DIR/fixtures"
HASHES="$PILOT_DIR/manifest/file-hashes.json"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if [ ! -f "$MANIFEST" ]; then
    echo "missing manifest: $MANIFEST" >&2; exit 1
fi

mkdir -p "$FIXTURES"
echo "[]" > "$HASHES.tmp"

# Iterate packages via jq.
pkgs=$(jq -r '.packages[] | "\(.name)|\(.version)"' "$MANIFEST")
total_files=0
ok_pkgs=0
fail_pkgs=0

while IFS='|' read -r name version; do
    if [ -z "$name" ] || [ -z "$version" ]; then continue; fi
    safe_name="${name//\//__}"
    pkg_dest="$FIXTURES/$safe_name"

    # Fetch registry metadata to get tarball URL.
    meta_url="https://registry.npmjs.org/${name}/${version}"
    tgz_url=$(curl -fsS "$meta_url" 2>/dev/null | jq -r '.dist.tarball' || echo "")
    if [ -z "$tgz_url" ] || [ "$tgz_url" = "null" ]; then
        echo "  fail: cannot resolve $name@$version" >&2
        fail_pkgs=$((fail_pkgs + 1))
        continue
    fi

    # Fetch + extract into a temp dir, then move only the .ts files.
    pkg_tmp="$TMP/$safe_name"
    mkdir -p "$pkg_tmp"
    if ! curl -fsS "$tgz_url" -o "$pkg_tmp/pkg.tgz" 2>/dev/null; then
        echo "  fail: tarball fetch $name@$version" >&2
        fail_pkgs=$((fail_pkgs + 1))
        continue
    fi
    tar -xzf "$pkg_tmp/pkg.tgz" -C "$pkg_tmp" 2>/dev/null || {
        echo "  fail: extract $name@$version" >&2
        fail_pkgs=$((fail_pkgs + 1))
        continue
    }

    rm -rf "$pkg_dest"
    mkdir -p "$pkg_dest"

    # Walk extracted "package/" dir; copy each .ts (excluding .d.ts) to
    # fixtures preserving rel path.
    src_root="$pkg_tmp/package"
    [ -d "$src_root" ] || src_root="$pkg_tmp"
    pkg_file_count=0
    while IFS= read -r -d '' f; do
        rel="${f#$src_root/}"
        # Skip .d.ts declarations.
        case "$rel" in *.d.ts) continue;; esac
        # Skip tests/fixtures dirs? Keep them for now — they're real .ts
        # too and TSR should handle them. If they bloat the corpus, add
        # an exclusion list later.
        dst="$pkg_dest/$rel"
        mkdir -p "$(dirname "$dst")"
        cp "$f" "$dst"
        sha=$(sha256sum "$dst" | awk '{print $1}')
        # Append to hashes JSON via a streaming jq update.
        jq --arg pkg "$name" --arg ver "$version" --arg path "$rel" \
            --arg sha "$sha" \
            '. + [{"package": $pkg, "version": $ver, "path": $path, "sha256": $sha}]' \
            "$HASHES.tmp" > "$HASHES.tmp2"
        mv "$HASHES.tmp2" "$HASHES.tmp"
        pkg_file_count=$((pkg_file_count + 1))
    done < <(find "$src_root" -name '*.ts' -type f -print0 2>/dev/null)

    echo "  ok: $name@$version ($pkg_file_count .ts files)"
    total_files=$((total_files + pkg_file_count))
    ok_pkgs=$((ok_pkgs + 1))
done <<< "$pkgs"

mv "$HASHES.tmp" "$HASHES"

echo ""
echo "═══ install summary ═══"
echo "packages ok:    $ok_pkgs"
echo "packages fail:  $fail_pkgs"
echo "total .ts files: $total_files"
echo "fixtures:       $FIXTURES"
echo "hashes record:  $HASHES"
