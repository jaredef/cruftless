#!/usr/bin/env bash
# Run every fixture under scripts/diff-prod/fixtures/*. Emits per-fixture
# PASS/FAIL plus an aggregate summary.

set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
RESULTS="${RESULTS_DIR:-$HERE/results}"
mkdir -p "$RESULTS"

n_pass=0
n_fail=0
n_total=0
for d in "$HERE/fixtures"/*/; do
  name="$(basename "$d")"
  n_total=$((n_total + 1))
  if "$HERE/run.sh" "$name"; then
    n_pass=$((n_pass + 1))
  else
    n_fail=$((n_fail + 1))
  fi
done

echo
echo "═══════════════════════════════════════════"
echo "diff-prod summary"
echo "═══════════════════════════════════════════"
echo "total:  $n_total"
echo "PASS:   $n_pass"
echo "FAIL:   $n_fail"
echo
echo "Per-fixture results: $RESULTS/<name>/result.json"
[ "$n_fail" -eq 0 ]
