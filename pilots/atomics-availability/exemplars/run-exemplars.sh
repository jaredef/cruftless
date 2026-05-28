#!/usr/bin/env bash
# atomics-availability exemplar runner. Reports aggregate + per-method breakdown.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
. "$ROOT/scripts/env.sh"
T262="${T262_ROOT:?set T262_ROOT}"
LIST="$HERE/exemplars.txt"
TOTAL=0; PASS=0; FAIL=0; FAILS=()
while read p; do
  TOTAL=$((TOTAL+1))
  out=$(T262_TEST_PATH="$p" T262_HARNESS_DIR="$T262/harness" timeout 8s \
    "$CRUFT_BIN" "$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs" 2>/dev/null | head -1)
  s=$(echo "$out" | python3 -c "import sys,json
try: print(json.loads(sys.stdin.read() or '{}').get('status','?'))
except: print('?')")
  if [ "$s" = "PASS" ]; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); FAILS+=("$p"); fi
done < "$LIST"
PCT=$(awk -v p="$PASS" -v t="$TOTAL" 'BEGIN{if(t>0) printf "%.1f", p*100/t; else printf "0.0"}')
echo "atomics-availability: PASS=$PASS FAIL=$FAIL / $TOTAL  ($PCT%)"
echo "--- fail breakdown per Atomics surface ---"
for f in "${FAILS[@]:-}"; do
  echo "$f" | awk -F/Atomics/ '{print $2}' | awk -F/ '{print $1}'
done | sort | uniq -c | sort -rn | head
