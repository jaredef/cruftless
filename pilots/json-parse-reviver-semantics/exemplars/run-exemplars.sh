#!/usr/bin/env bash
if [ -z "${NICED:-}" ]; then exec env NICED=1 nice -n19 ionice -c2 -n7 bash "$0" "$@"; fi
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$HERE/../../.." && pwd)"
# shellcheck disable=SC1091
. "$REPO_ROOT/scripts/env.sh"

T262_ROOT="${T262_ROOT:-/Users/jaredfoy/Developer/cruftless-sidecar/test262}"
CRUFT_BIN="${CRUFT_BIN:-$REPO_ROOT/target/debug/cruft}"
RUNNER="$REPO_ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
HARNESS="$T262_ROOT/harness"

total=0
pass=0
fail=0
skip=0
nojson=0

while IFS= read -r rel; do
  [ -n "$rel" ] || continue
  total=$((total + 1))
  path="$T262_ROOT/test/$rel"
  out="$(T262_TEST_PATH="$path" T262_HARNESS_DIR="$HARNESS" "$CRUFT_BIN" "$RUNNER" 2>/dev/null | head -1 || true)"
  status="$(printf '%s' "$out" | python3 -c 'import json,sys
try:
    print(json.loads(sys.stdin.read() or "{}").get("status", "NOJSON"))
except Exception:
    print("NOJSON")')"
  case "$status" in
    PASS) pass=$((pass + 1)) ;;
    FAIL) fail=$((fail + 1)) ;;
    SKIP) skip=$((skip + 1)) ;;
    *) nojson=$((nojson + 1)) ;;
  esac
  printf '%s\t%s\t%s\n' "$rel" "$status" "$out"
done < "$HERE/exemplars.txt"

printf 'JPRS exemplars: PASS=%s FAIL=%s SKIP=%s NOJSON=%s / %s\n' \
  "$pass" "$fail" "$skip" "$nojson" "$total"
