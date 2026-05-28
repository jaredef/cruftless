#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$HERE/../../.." && pwd)"
# shellcheck disable=SC1091
. "$REPO_ROOT/scripts/env.sh"

T262_ROOT="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
CRUFT_BIN="${CRUFT_BIN:-$REPO_ROOT/target/debug/cruft}"
RUNNER="$REPO_ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
HARNESS="$T262_ROOT/harness"
LIST="${CLFG_EXEMPLARS_LIST:-$HERE/exemplars.txt}"
TIMEOUT_SECONDS="${CLFG_TIMEOUT_SECONDS:-10}"

total=0
pass=0
fail=0
skip=0
nojson=0

while IFS= read -r rel; do
  [ -n "$rel" ] || continue
  total=$((total + 1))
  path="$T262_ROOT/test/$rel"
  if command -v timeout >/dev/null 2>&1; then
    out="$(T262_TEST_PATH="$path" T262_HARNESS_DIR="$HARNESS" timeout "${TIMEOUT_SECONDS}s" "$CRUFT_BIN" "$RUNNER" 2>/dev/null | head -1 || true)"
  else
    out="$(T262_TEST_PATH="$path" T262_HARNESS_DIR="$HARNESS" "$CRUFT_BIN" "$RUNNER" 2>/dev/null | head -1 || true)"
  fi
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
done < "$LIST"

printf 'CLFG exemplars: PASS=%s FAIL=%s SKIP=%s NOJSON=%s / %s\n' \
  "$pass" "$fail" "$skip" "$nojson" "$total"
