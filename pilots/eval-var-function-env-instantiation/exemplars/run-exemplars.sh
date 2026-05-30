#!/usr/bin/env bash
if [ -z "${NICED:-}" ]; then exec env NICED=1 nice -n19 ionice -c2 -n7 bash "$0" "$@"; fi
# Focused runner for eval var/function environment-instantiation exemplars.
set -uo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
. "$ROOT/scripts/env.sh"

T262="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
LIST="$HERE/exemplars.txt"
TOTAL=0
PASS=0
FAIL=0
SKIP=0
NOJSON=0
FAILS=()

while read -r p; do
  case "$p" in
    */test262/test/*) p="$T262/test/${p#*/test262/test/}" ;;
  esac
  TOTAL=$((TOTAL + 1))
  if command -v timeout >/dev/null 2>&1; then
    out=$(T262_TEST_PATH="$p" T262_HARNESS_DIR="$T262/harness" timeout 10s \
      "$CRUFT_BIN" "$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs" 2>/dev/null | head -1)
  else
    out=$(T262_TEST_PATH="$p" T262_HARNESS_DIR="$T262/harness" \
      "$CRUFT_BIN" "$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs" 2>/dev/null | head -1)
  fi
  s=$(echo "$out" | python3 -c "import sys,json
try: print(json.loads(sys.stdin.read() or '{}').get('status','NOJSON'))
except Exception: print('NOJSON')")
  case "$s" in
    PASS) PASS=$((PASS + 1)) ;;
    FAIL) FAIL=$((FAIL + 1)); FAILS+=("$p") ;;
    SKIP) SKIP=$((SKIP + 1)) ;;
    *) NOJSON=$((NOJSON + 1)); FAILS+=("$p") ;;
  esac
done < "$LIST"

PCT=$(awk -v pass="$PASS" -v total="$TOTAL" 'BEGIN{if (total > 0) printf "%.1f", pass*100/total; else printf "0.0"}')
echo "EVFEI exemplars: PASS=$PASS FAIL=$FAIL SKIP=$SKIP NOJSON=$NOJSON / $TOTAL  ($PCT%)"

if [ "${#FAILS[@]}" -gt 0 ]; then
  echo "--- fails ---"
  for f in "${FAILS[@]}"; do
    echo "$f" | sed 's#^.*/test/##'
  done
fi
