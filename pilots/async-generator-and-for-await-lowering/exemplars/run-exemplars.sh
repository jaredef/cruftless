#!/usr/bin/env bash
if [ -z "${NICED:-}" ]; then exec env NICED=1 nice -n19 ionice -c2 -n7 bash "$0" "$@"; fi
# Async-generator / for-await lowering exemplar runner.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
. "$ROOT/scripts/env.sh"

T262="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
LIST="${AGFA_EXEMPLARS_LIST:-$HERE/exemplars.txt}"
RUNNER="$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
HARNESS="$T262/harness"

TOTAL=0
PASS=0
FAIL=0
FAILS=()
while read -r p; do
  [ -n "$p" ] || continue
  case "$p" in
    */test262/test/*) p="$T262/test/${p#*/test262/test/}" ;;
    /*) ;;
    *) p="$T262/test/$p" ;;
  esac
  TOTAL=$((TOTAL + 1))
  if command -v timeout >/dev/null 2>&1; then
    out=$(T262_TEST_PATH="$p" T262_HARNESS_DIR="$HARNESS" timeout 10s \
      "$CRUFT_BIN" "$RUNNER" 2>/dev/null | head -1)
  else
    out=$(T262_TEST_PATH="$p" T262_HARNESS_DIR="$HARNESS" \
      "$CRUFT_BIN" "$RUNNER" 2>/dev/null | head -1)
  fi
  s=$(printf '%s' "$out" | python3 -c "import sys,json
try: print(json.loads(sys.stdin.read() or '{}').get('status','?'))
except Exception: print('?')")
  if [ "$s" = "PASS" ]; then
    PASS=$((PASS + 1))
  else
    FAIL=$((FAIL + 1))
    FAILS+=("$p")
  fi
done < "$LIST"

PCT=$(awk -v pass="$PASS" -v total="$TOTAL" 'BEGIN{if (total > 0) printf "%.1f", pass*100/total; else printf "0.0"}')
echo "AGFA exemplars: PASS=$PASS FAIL=$FAIL / $TOTAL  ($PCT%)"
echo "--- top fails by surface ---"
if [ "${#FAILS[@]}" -gt 0 ]; then
  for f in "${FAILS[@]}"; do
    echo "$f" | awk -F'/test/' '{print $2}' | awk -F/ '{print $1"/"$2"/"$3}'
  done | sort | uniq -c | sort -rn | head -20
fi

