#!/usr/bin/env bash
if [ -z "${NICED:-}" ]; then exec env NICED=1 nice -n19 ionice -c2 -n7 bash "$0" "$@"; fi
# tagged-template-tail-call-boundary exemplar runner.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
. "$ROOT/scripts/env.sh"

T262="${T262_ROOT:-/Users/jaredfoy/Developer/cruftless-sidecar/test262}"
LIST="${TTTC_EXEMPLARS_LIST:-$HERE/exemplars.txt}"
RUNNER="$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
HARNESS="$T262/harness"
CRUFT="${CRUFT_BIN:-$ROOT/target/release/cruft}"

TOTAL=0
PASS=0
FAIL=0
NOJSON=0
FAILS=()
while read -r p; do
  [ -n "$p" ] || continue
  case "$p" in
    */test262/test/*) p="$T262/test/${p#*/test262/test/}" ;;
    /*) ;;
    *) p="$T262/test/$p" ;;
  esac
  TOTAL=$((TOTAL + 1))
  out=$(T262_TEST_PATH="$p" T262_HARNESS_DIR="$HARNESS" "$CRUFT" "$RUNNER" 2>/dev/null | head -1)
  s=$(printf '%s' "$out" | python3 -c "import sys,json
try: print(json.loads(sys.stdin.read() or '{}').get('status','?'))
except Exception: print('?')")
  if [ "$s" = "PASS" ]; then
    PASS=$((PASS + 1))
  elif [ "$s" = "?" ] || [ -z "$out" ]; then
    NOJSON=$((NOJSON + 1))
    FAILS+=("$p :: NOJSON")
  else
    FAIL=$((FAIL + 1))
    FAILS+=("$p")
  fi
done < "$LIST"

PCT=$(awk -v pass="$PASS" -v total="$TOTAL" 'BEGIN{if (total > 0) printf "%.1f", pass*100/total; else printf "0.0"}')
echo "TTTC exemplars: PASS=$PASS FAIL=$FAIL NOJSON=$NOJSON / $TOTAL  ($PCT%)"
echo "--- residuals ---"
if [ "${#FAILS[@]}" -gt 0 ]; then
  printf '%s\n' "${FAILS[@]}" | sed "s#^$T262/test/##" | head -20
fi
