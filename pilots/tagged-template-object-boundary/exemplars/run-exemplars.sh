#!/usr/bin/env bash
set -uo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
. "$ROOT/scripts/env.sh"

T262="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
LIST="$HERE/exemplars.txt"
TOTAL=0
PASS=0
FAIL=0
ABORT=0
FAILS=()

while read -r p; do
  [ -z "$p" ] && continue
  case "$p" in
    */test262/test/*) rel="${p#*/test262/test/}" ;;
    test/*) rel="${p#test/}" ;;
    *) rel="$p" ;;
  esac
  full="$T262/test/$rel"
  TOTAL=$((TOTAL + 1))

  if command -v timeout >/dev/null 2>&1; then
    out=$(T262_TEST_PATH="$full" T262_HARNESS_DIR="$T262/harness" timeout 10s \
      "$CRUFT_BIN" "$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs" 2>/dev/null | head -1)
  else
    out=$(T262_TEST_PATH="$full" T262_HARNESS_DIR="$T262/harness" \
      "$CRUFT_BIN" "$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs" 2>/dev/null | head -1)
  fi

  status=$(printf '%s' "$out" | python3 -c "import sys,json
try:
    print(json.loads(sys.stdin.read() or '{}').get('status','?'))
except Exception:
    print('?')")

  case "$status" in
    PASS) PASS=$((PASS + 1)) ;;
    ?) ABORT=$((ABORT + 1)); FAIL=$((FAIL + 1)); FAILS+=("$rel :: no-json") ;;
    *) FAIL=$((FAIL + 1)); FAILS+=("$rel :: $status") ;;
  esac
done < "$LIST"

PCT=$(awk -v pass="$PASS" -v total="$TOTAL" 'BEGIN{if (total > 0) printf "%.1f", pass*100/total; else printf "0.0"}')
echo "Tagged-template exemplars: PASS=$PASS FAIL=$FAIL ABORT=$ABORT / $TOTAL ($PCT%)"
if [ "${#FAILS[@]}" -gt 0 ]; then
  echo "--- fails ---"
  printf '%s\n' "${FAILS[@]}"
fi
