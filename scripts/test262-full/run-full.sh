#!/usr/bin/env bash
if [ -z "${NICED:-}" ]; then exec env NICED=1 nice -n19 ionice -c2 -n7 bash "$0" "$@"; fi
# Run the FULL test262 suite under cruft with parallelism.
#
# Unlike run-sample.sh (which expands the curated sample-paths.txt), this
# enumerates every *.js test under $T262_ROOT/test, excluding _FIXTURE files
# and the harness/ tree. Output JSONL is the input the full-suite Pin-Art
# categorizer consumes:
#   cargo run --release -p test262-categorize --bin t262-full-pinart -- <results.jsonl>
#
# Per-test cap: 10s. Parallelism: 2 (override with PARALLEL=N).
#
# Output: $CRUFTLESS_TEST262_RESULTS_ROOT/test262-full-<DATE-HHMMSS>/{results.jsonl,summary.txt,paths.txt}
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
# shellcheck disable=SC1091
. "$ROOT/scripts/env.sh"

# Run the binary from a local copy (SD/local disk), keeping the build target
# on whatever backing store it lives on. Auto-refresh when the build is newer.
BUILT_CRUFT="$ROOT/target/release/cruft"
if [ -x "$BUILT_CRUFT" ] && { [ ! -x "$LOCAL_CRUFT" ] || [ "$BUILT_CRUFT" -nt "$LOCAL_CRUFT" ]; }; then
  mkdir -p "$(dirname "$LOCAL_CRUFT")"
  cp "$BUILT_CRUFT" "$LOCAL_CRUFT"
fi
CRUFT="${CRUFT_BIN:-${RB_BIN:-$LOCAL_CRUFT}}"
T262="${T262_ROOT:?set T262_ROOT in env.local or export it before running}"
PARALLEL="${PARALLEL:-2}"
RUNNER="$ROOT/legacy/host-rquickjs/tests/test262/runner.mjs"
HARNESS="$T262/harness"

STAMP="$(date +%Y-%m-%d-%H%M%S)"
OUT="$CRUFTLESS_TEST262_RESULTS_ROOT/test262-full-$STAMP-p$PARALLEL"
mkdir -p "$OUT"
RESULTS="$OUT/results.jsonl"
SUMMARY="$OUT/summary.txt"
: > "$RESULTS"

# Enumerate the full test tree (exclude fixtures + the harness dir).
find "$T262/test" -name '*.js' \
  | grep -v '_FIXTURE' \
  | sort -u > "$OUT/paths.txt"
COUNT=$(wc -l < "$OUT/paths.txt")
echo "Full suite size: $COUNT tests; parallelism: $PARALLEL"
echo "Output: $OUT"

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
  echo "test262 full suite — $STAMP"
  echo "Suite size:      $COUNT"
  echo "Results emitted: $TOTAL"
  echo "PASS:            $PASS"
  echo "FAIL:            $FAIL"
  echo "SKIP:            $SKIP  (frontmatter feature flags, not runnable)"
  echo "Runnable pass rate: $PCT%  ($PASS / $RUNNABLE)"
} | tee "$SUMMARY"
