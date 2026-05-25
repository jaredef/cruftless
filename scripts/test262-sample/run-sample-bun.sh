#!/usr/bin/env bash
# Run the curated test262 representative sample under bun for parity comparison.
# Mirrors run-sample.sh but invokes bun instead of cruftless. Same runner.mjs,
# same harness, same sample paths — so the comparison is apples-to-apples.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
# shellcheck disable=SC1091
. "$ROOT/scripts/env.sh"
T262="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
PARALLEL="${PARALLEL:-2}"
RUNNER="$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
HARNESS="$T262/harness"

DATE="$(date +%Y-%m-%d)"
OUT="$ROOT/results/test262-sample-$DATE-bun"
mkdir -p "$OUT"
RESULTS="$OUT/results.jsonl"
SUMMARY="$OUT/summary.txt"
: > "$RESULTS"

xargs -I {} find "$T262/test/{}" -name '*.js' 2>/dev/null \
  < "$HERE/sample-paths.txt" \
  | grep -v '_FIXTURE' \
  | sort -u > "$OUT/paths.txt"
COUNT=$(wc -l < "$OUT/paths.txt")
echo "Sample size: $COUNT tests; parallelism: $PARALLEL (bun)"

run_one() {
  local p="$1"
  T262_TEST_PATH="$p" T262_HARNESS_DIR="$HARNESS" \
    timeout 10s "$BUN_BIN" "$RUNNER" 2>/dev/null \
    | head -1
}
export -f run_one
export RUNNER HARNESS

nice -n 19 ionice -c3 xargs -a "$OUT/paths.txt" -P "$PARALLEL" -I {} \
  bash -c 'run_one "$@"' _ {} \
  >> "$RESULTS"

PASS=$(grep -c '"status":"PASS"' "$RESULTS" || true)
FAIL=$(grep -c '"status":"FAIL"' "$RESULTS" || true)
SKIP=$(grep -c '"status":"SKIP"' "$RESULTS" || true)
TOTAL=$((PASS + FAIL + SKIP))
RUNNABLE=$((PASS + FAIL))
PCT=$(awk -v p="$PASS" -v r="$RUNNABLE" 'BEGIN{printf "%.1f", r>0?100*p/r:0}')
{
  echo "test262 representative sample (bun) — $DATE"
  echo "bun version: $($BUN_BIN --version)"
  echo "Sample size:    $COUNT"
  echo "Results emitted: $TOTAL"
  echo "PASS:           $PASS"
  echo "FAIL:           $FAIL"
  echo "SKIP:           $SKIP"
  echo "Runnable pass rate: $PCT%  ($PASS / $RUNNABLE)"
} | tee "$SUMMARY"
