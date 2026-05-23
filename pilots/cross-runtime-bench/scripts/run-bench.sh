#!/usr/bin/env bash
# CRB-EXT 1: cross-runtime bench runner.
#
# Discovers fixtures under fixtures/*/main.mjs, runs each fixture
# × {node, bun, cruft} × N (default N=5), captures wall-clock per
# run, computes median, writes JSONL + markdown summary.
#
# Usage:
#   scripts/run-bench.sh                          # all fixtures, N=5
#   scripts/run-bench.sh --fixtures json_parse    # one fixture
#   scripts/run-bench.sh --runs 10                # 10 runs each
#   scripts/run-bench.sh --skip cruft             # skip a runtime
#
# Output: results/<YYYY-MM-DD>/{summary.md, results.jsonl}
set -euo pipefail

PILOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
FIXTURES_DIR="$PILOT_DIR/fixtures"
RESULTS_ROOT="$PILOT_DIR/results"

# Binary locations.
CRUFT_BIN="${CRUFT_BIN:-$HOME/bin/cruft}"
BUN_BIN="${BUN_BIN:-$HOME/.bun/bin/bun}"
NODE_BIN="${NODE_BIN:-/usr/bin/node}"

RUNS=5
PATTERN="*"
SKIP=""
while [ $# -gt 0 ]; do
  case "$1" in
    --fixtures) PATTERN="$2"; shift 2 ;;
    --runs)     RUNS="$2"; shift 2 ;;
    --skip)     SKIP="$SKIP $2"; shift 2 ;;
    --help|-h)
      head -16 "$0" | tail -14; exit 0 ;;
    *) echo "unknown arg: $1" >&2; exit 1 ;;
  esac
done

# Result dir.
DATE="$(date +%Y-%m-%d)"
OUT_DIR="$RESULTS_ROOT/$DATE"
mkdir -p "$OUT_DIR"
JSONL="$OUT_DIR/results.jsonl"
SUMMARY="$OUT_DIR/summary.md"

# Truncate output files (re-run replaces previous same-day result).
: > "$JSONL"
: > "$SUMMARY"

declare -A RT_BINS=(
  [node]="$NODE_BIN"
  [bun]="$BUN_BIN"
  [cruft]="$CRUFT_BIN"
)

# Compute median of newline-separated numbers via sort + middle.
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

skipped() {
  for s in $SKIP; do [ "$s" = "$1" ] && return 0; done
  return 1
}

# Wall-clock in ms via date +%s%N diff.
time_run_ms() {
  local bin="$1"; shift
  local file="$1"; shift
  local start_ns end_ns
  start_ns=$(date +%s%N)
  if ! "$bin" "$file" > /dev/null 2>&1; then
    echo "FAIL"; return
  fi
  end_ns=$(date +%s%N)
  echo "$(( (end_ns - start_ns) / 1000000 ))"
}

# Verify three-runtime stdout equality on a single run.
verify_equality() {
  local file="$1"
  local node_out bun_out cruft_out
  if skipped node;  then return 0; fi
  if skipped bun;   then return 0; fi
  if skipped cruft; then return 0; fi
  node_out="$("$NODE_BIN"  "$file" 2>/dev/null || echo "NODE_FAIL")"
  bun_out="$("$BUN_BIN"    "$file" 2>/dev/null || echo "BUN_FAIL")"
  cruft_out="$("$CRUFT_BIN" "$file" 2>/dev/null || echo "CRUFT_FAIL")"
  if [ "$node_out" = "$bun_out" ] && [ "$bun_out" = "$cruft_out" ]; then
    echo "EQUAL"
  else
    echo "DIFFER"
  fi
}

# Markdown header.
{
  echo "# cross-runtime-bench summary — $DATE"
  echo
  echo "runs per (fixture × runtime): **$RUNS**; wall-clock in ms (median of $RUNS); runtimes: node $($NODE_BIN --version 2>/dev/null), bun $($BUN_BIN --version 2>/dev/null), cruft $($CRUFT_BIN --version 2>/dev/null | head -1)"
  echo
  echo "| fixture | equality | node (ms) | bun (ms) | cruft (ms) | cruft / node | cruft / bun |"
  echo "|---|---|---:|---:|---:|---:|---:|"
} > "$SUMMARY"

# Discover fixtures.
FIXTURES=()
for d in "$FIXTURES_DIR"/$PATTERN/main.mjs; do
  [ -f "$d" ] || continue
  name="$(basename "$(dirname "$d")")"
  FIXTURES+=("$name")
done
[ ${#FIXTURES[@]} -eq 0 ] && { echo "no fixtures matched: $PATTERN" >&2; exit 1; }

echo "running ${#FIXTURES[@]} fixture(s) × ${#RT_BINS[@]} runtime(s) × $RUNS runs each"
echo "output: $OUT_DIR"
echo

for name in "${FIXTURES[@]}"; do
  file="$FIXTURES_DIR/$name/main.mjs"
  echo "=== $name ==="

  equality="$(verify_equality "$file")"
  echo "  equality: $equality"

  declare -A MEDIANS=()
  for rt in node bun cruft; do
    if skipped "$rt"; then
      MEDIANS[$rt]="SKIP"
      echo "  $rt: SKIPPED"
      continue
    fi
    bin="${RT_BINS[$rt]}"
    runs=()
    for ((i=1; i<=RUNS; i++)); do
      ms="$(time_run_ms "$bin" "$file")"
      runs+=("$ms")
      printf "%s " "$ms" >&2
      echo "{\"fixture\":\"$name\",\"runtime\":\"$rt\",\"run\":$i,\"ms\":\"$ms\"}" >> "$JSONL"
    done
    echo >&2
    # Median (skip FAIL runs).
    valid="$(printf '%s\n' "${runs[@]}" | grep -v FAIL || true)"
    if [ -z "$valid" ]; then
      MEDIANS[$rt]="FAIL"
      echo "  $rt: FAIL (all runs)"
    else
      med="$(printf '%s\n' "$valid" | median)"
      MEDIANS[$rt]="$med"
      echo "  $rt: median ${med} ms (runs: ${runs[*]})"
    fi
  done

  # Ratios.
  ratio_n="-"
  ratio_b="-"
  if [[ "${MEDIANS[cruft]}" != "SKIP" && "${MEDIANS[cruft]}" != "FAIL" ]]; then
    if [[ "${MEDIANS[node]}" != "SKIP" && "${MEDIANS[node]}" != "FAIL" ]]; then
      ratio_n="$(awk -v c="${MEDIANS[cruft]}" -v n="${MEDIANS[node]}" 'BEGIN { printf "%.2fx", c/n }')"
    fi
    if [[ "${MEDIANS[bun]}" != "SKIP" && "${MEDIANS[bun]}" != "FAIL" ]]; then
      ratio_b="$(awk -v c="${MEDIANS[cruft]}" -v b="${MEDIANS[bun]}" 'BEGIN { printf "%.2fx", c/b }')"
    fi
  fi

  echo "| $name | $equality | ${MEDIANS[node]} | ${MEDIANS[bun]} | ${MEDIANS[cruft]} | $ratio_n | $ratio_b |" >> "$SUMMARY"
done

echo
echo "wrote: $SUMMARY"
echo "       $JSONL"
echo
cat "$SUMMARY"
