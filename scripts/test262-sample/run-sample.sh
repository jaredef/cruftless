#!/usr/bin/env bash
# Run the curated test262 representative sample with parallelism.
#
# The sample (sample-paths.txt) targets the surface that production
# Node.js packages actually exercise: core builtins (JSON, Map, Set,
# Number, Math, Symbol, Error, Promise), the most-used Array/String
# prototype methods, key Object statics, RegExp exec/test, and the
# language constructs that real code uses (arrow-function, for-of,
# for-in).
#
# Per-test cap: 10s. Parallelism: 4 (override with PARALLEL=N).
#
# Output: results/test262-sample-<DATE>/{results.jsonl,summary.txt}
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
CRUFT="${CRUFT_BIN:-${RB_BIN:-$ROOT/target/release/cruft}}"
T262="${T262_ROOT:-/home/jaredef/test262}"
PARALLEL="${PARALLEL:-4}"
RUNNER="$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
HARNESS="$T262/harness"

DATE="$(date +%Y-%m-%d)"
OUT="$ROOT/results/test262-sample-$DATE"
mkdir -p "$OUT"
RESULTS="$OUT/results.jsonl"
SUMMARY="$OUT/summary.txt"
: > "$RESULTS"

# Expand sample paths to absolute test file paths.
xargs -I {} find "$T262/test/{}" -name '*.js' 2>/dev/null \
  < "$HERE/sample-paths.txt" \
  | grep -v '_FIXTURE' \
  | sort -u > "$OUT/paths.txt"
COUNT=$(wc -l < "$OUT/paths.txt")
echo "Sample size: $COUNT tests; parallelism: $PARALLEL"

run_one() {
  local p="$1"
  T262_TEST_PATH="$p" T262_HARNESS_DIR="$HARNESS" \
    timeout 10s "$CRUFT" "$RUNNER" 2>/dev/null \
    | head -1
}
export -f run_one
export CRUFT RUNNER HARNESS

nice -n 19 ionice -c3 xargs -a "$OUT/paths.txt" -P "$PARALLEL" -I {} \
  bash -c 'run_one "$@"' _ {} \
  >> "$RESULTS"

# Tally.
PASS=$(grep -c '"status":"PASS"' "$RESULTS" || true)
FAIL=$(grep -c '"status":"FAIL"' "$RESULTS" || true)
SKIP=$(grep -c '"status":"SKIP"' "$RESULTS" || true)
TOTAL=$((PASS + FAIL + SKIP))
RUNNABLE=$((PASS + FAIL))
if [ "$RUNNABLE" -gt 0 ]; then
  PCT=$(awk -v p="$PASS" -v r="$RUNNABLE" 'BEGIN{printf "%.1f", 100*p/r}')
else
  PCT="0.0"
fi
{
  echo "test262 representative sample — $DATE"
  echo "Sample size:    $COUNT"
  echo "Results emitted: $TOTAL"
  echo "PASS:           $PASS"
  echo "FAIL:           $FAIL"
  echo "SKIP:           $SKIP  (frontmatter feature flags, not runnable)"
  echo "Runnable pass rate: $PCT%  ($PASS / $RUNNABLE)"
} | tee "$SUMMARY"
