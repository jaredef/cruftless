#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$HERE/../../.." && pwd)"
# shellcheck disable=SC1091
. "$REPO_ROOT/scripts/env.sh"

FIXTURE="$REPO_ROOT/pilots/apparatus/cross-runtime-bench/fixtures/json_parse_transform/main.mjs"
NODE_BIN="${NODE_BIN:-node}"
BUN_BIN="${BUN_BIN:-bun}"
CRUFT_BIN="${CRUFT_BIN:-$REPO_ROOT/target/release/cruft}"
RUNS="${RUNS:-5}"

median() {
  sort -n | awk '
    { vals[NR]=$1 }
    END {
      n=NR
      if (n==0) { print "NaN"; exit }
      if (n%2) print vals[(n+1)/2]
      else printf "%.3f\n", (vals[n/2]+vals[n/2+1])/2
    }
  '
}

time_run_ms() {
  local bin="$1"
  local start_ns end_ns
  start_ns="$(python3 - <<'PY'
import time
print(time.time_ns())
PY
)"
  "$bin" "$FIXTURE" >/dev/null 2>&1
  end_ns="$(python3 - <<'PY'
import time
print(time.time_ns())
PY
)"
  echo "$(( (end_ns - start_ns) / 1000000 ))"
}

node_out="$("$NODE_BIN" "$FIXTURE")"
bun_out="$("$BUN_BIN" "$FIXTURE")"
cruft_out="$("$CRUFT_BIN" "$FIXTURE")"
if [ "$node_out" = "$bun_out" ] && [ "$bun_out" = "$cruft_out" ]; then
  echo "equality: EQUAL"
else
  echo "equality: DIFFER"
  printf 'node=%s\nbun=%s\ncruft=%s\n' "$node_out" "$bun_out" "$cruft_out"
fi

for rt in node bun cruft; do
  case "$rt" in
    node) bin="$NODE_BIN" ;;
    bun) bin="$BUN_BIN" ;;
    cruft) bin="$CRUFT_BIN" ;;
  esac
  vals=""
  for i in $(seq 1 "$RUNS"); do
    ms="$(time_run_ms "$bin")"
    vals="$vals$ms\n"
    printf '%s run %s: %s ms\n' "$rt" "$i" "$ms"
  done
  med="$(printf '%b' "$vals" | median)"
  echo "$rt median: $med ms"
done
