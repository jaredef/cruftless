#!/usr/bin/env bash
set -uo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
. "$ROOT/scripts/env.sh"

T262="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
RUNNER="$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
LIST="${PNL_EXEMPLARS_LIST:-$HERE/exemplars.txt}"

TOTAL=0
PASS=0
FAIL=0
FAILS=()

run_one() {
  if command -v timeout >/dev/null 2>&1; then
    T262_TEST_PATH="$1" T262_HARNESS_DIR="$T262/harness" timeout 10s \
      "$CRUFT_BIN" "$RUNNER" 2>/dev/null | head -1
  else
    T262_TEST_PATH="$1" T262_HARNESS_DIR="$T262/harness" \
      "$CRUFT_BIN" "$RUNNER" 2>/dev/null | head -1
  fi
}

while read -r p; do
  [ -n "$p" ] || continue
  case "$p" in
    */test262/test/*) p="$T262/test/${p#*/test262/test/}" ;;
    /*) ;;
    *) p="$T262/test/$p" ;;
  esac
  TOTAL=$((TOTAL + 1))
  out=$(run_one "$p")
  s=$(printf '%s' "$out" | python3 -c "import sys,json
try: print(json.loads(sys.stdin.read() or '{}').get('status','?'))
except: print('?')")
  if [ "$s" = "PASS" ]; then
    PASS=$((PASS + 1))
  else
    FAIL=$((FAIL + 1))
    FAILS+=("$p")
  fi
done < "$LIST"

PCT=$(awk -v pass="$PASS" -v total="$TOTAL" 'BEGIN{if (total > 0) printf "%.1f", pass*100/total; else printf "0.0"}')
echo "Private-name exemplars: PASS=$PASS FAIL=$FAIL / $TOTAL  ($PCT%)"
echo "--- top fails by surface family ---"
if [ "${#FAILS[@]}" -gt 0 ]; then
  for f in "${FAILS[@]}"; do
    echo "$f" | awk -F/test/ '{print $2}' | awk -F/ '{print $1"/"$2"/"$3"/"$4"/"$5}'
  done | sort | uniq -c | sort -nr | head -20
fi
