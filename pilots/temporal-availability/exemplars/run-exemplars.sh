#!/usr/bin/env bash
# Temporal-availability exemplar runner.
# Runs the 100 stratified-sample tests via the test262 harness wrapper
# against cruft; prints pass/fail aggregate and per-Temporal-class
# breakdown of remaining fails.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
. "$ROOT/scripts/env.sh"
LIST="$HERE/exemplars.txt"
TOTAL=0; PASS=0; FAIL=0
FAILS=()
while read p; do
  TOTAL=$((TOTAL+1))
  out=$(T262_TEST_PATH="$p" T262_HARNESS_DIR="$T262_ROOT/harness" timeout 10s \
    $CRUFT_BIN "$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs" 2>/dev/null | head -1)
  s=$(echo "$out" | python3 -c "import sys,json
try: print(json.loads(sys.stdin.read() or '{}').get('status','?'))
except: print('?')")
  if [ "$s" = "PASS" ]; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); FAILS+=("$p"); fi
done < "$LIST"

echo "Temporal exemplars: PASS=$PASS FAIL=$FAIL / $TOTAL  ($(awk "BEGIN{printf \"%.1f\", $PASS*100/$TOTAL}")%)"

# Per-class breakdown of fails
echo "--- fails by Temporal class ---"
for f in "${FAILS[@]:-}"; do
  echo "$f" | awk -F'Temporal/' '{print $2}' | awk -F/ '{print $1}'
done | sort | uniq -c | sort -rn
