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
# Output: $CRUFTLESS_TEST262_RESULTS_ROOT/test262-sample-<DATE>/{results.jsonl,summary.txt}
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
# shellcheck disable=SC1091
. "$ROOT/scripts/env.sh"
# Default cruft to a local SD-card copy at ~/bin/cruft. The build target/
# is a symlink onto the USB-attached T7 (exfat); execing the binary ~7600
# times from there is what most plausibly hung the Pi on 2026-05-22 (USB
# bridge D-state under parallel exec/mmap fan-out). Keep the build on T7,
# run the binary from SD. Auto-refresh the local copy when the built one
# is newer.
BUILT_CRUFT="$ROOT/target/release/cruft"
if [ -x "$BUILT_CRUFT" ] && { [ ! -x "$LOCAL_CRUFT" ] || [ "$BUILT_CRUFT" -nt "$LOCAL_CRUFT" ]; }; then
  mkdir -p "$(dirname "$LOCAL_CRUFT")"
  cp "$BUILT_CRUFT" "$LOCAL_CRUFT"
fi
CRUFT="${CRUFT_BIN:-${RB_BIN:-$LOCAL_CRUFT}}"
T262="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
# PARALLEL default lowered from 4 → 2 on the Pi. Four concurrent cruft
# children pushed the box into a hang on 2026-05-22; 2 leaves headroom on
# the 8 GB / 2 GB-zram-swap setup. Override with PARALLEL=N for big hosts.
PARALLEL="${PARALLEL:-2}"
RUNNER="$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
HARNESS="$T262/harness"

DATE="$(date +%Y-%m-%d)"
OUT="$CRUFTLESS_TEST262_RESULTS_ROOT/test262-sample-$DATE"
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
