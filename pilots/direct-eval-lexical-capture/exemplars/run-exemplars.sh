#!/usr/bin/env bash
if [ -z "${NICED:-}" ]; then exec env NICED=1 nice -n19 ionice -c2 -n7 bash "$0" "$@"; fi
set -uo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
. "$ROOT/scripts/env.sh"

T262="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
LIST="$HERE/exemplars.txt"
TOTAL=0
PASS=0
FAIL=0
FAILS=()

while read -r p; do
  [ -z "$p" ] && continue
  rel="${p#test/}"
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
  if [ "$status" = "PASS" ]; then
    PASS=$((PASS + 1))
  else
    FAIL=$((FAIL + 1))
    FAILS+=("$rel :: $status")
  fi
done < "$LIST"

PCT=$(awk -v pass="$PASS" -v total="$TOTAL" 'BEGIN{if (total > 0) printf "%.1f", pass*100/total; else printf "0.0"}')
echo "Direct-eval lexical exemplars: PASS=$PASS FAIL=$FAIL / $TOTAL ($PCT%)"
if [ "${#FAILS[@]}" -gt 0 ]; then
  echo "--- fails ---"
  printf '%s\n' "${FAILS[@]}"
fi
