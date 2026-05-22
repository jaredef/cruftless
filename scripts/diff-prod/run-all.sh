#!/usr/bin/env bash
# Run every fixture under scripts/diff-prod/fixtures/*. Emits per-fixture
# PASS/FAIL plus an aggregate summary. Runs everything `nicely` and uses
# the T7 mounted drive for sandbox + results by default.

set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"

PROD_SANDBOX="${PROD_SANDBOX:-/media/jaredef/T7/rusty-bun/diff-prod-sandbox}"
RESULTS_DIR="${RESULTS_DIR:-/media/jaredef/T7/rusty-bun/diff-prod-results}"
export PROD_SANDBOX RESULTS_DIR

mkdir -p "$PROD_SANDBOX" "$RESULTS_DIR"

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

# Aggregate summary file.
python3 - "$RESULTS_DIR" "$HERE" <<'PY'
import json, os, sys
results_dir, here = sys.argv[1], sys.argv[2]
fixtures_dir = os.path.join(here, "fixtures")
summary = {"fixtures": {}, "totals": {"pass": 0, "fail": 0, "total": 0}}
for name in sorted(os.listdir(fixtures_dir)):
    res = os.path.join(results_dir, name, "result.json")
    if not os.path.isfile(res):
        continue
    r = json.load(open(res))
    summary["fixtures"][name] = {
        "overall": r["overall"],
        "categories": {c: v["status"] for c, v in r["categories"].items()},
    }
    summary["totals"]["total"] += 1
    if r["overall"] == "PASS":
        summary["totals"]["pass"] += 1
    else:
        summary["totals"]["fail"] += 1
json.dump(summary, open(os.path.join(results_dir, "summary.json"), "w"), indent=2)
PY

echo
echo "═══════════════════════════════════════════"
echo "diff-prod summary"
echo "═══════════════════════════════════════════"
echo "total:  $n_total"
echo "PASS:   $n_pass"
echo "FAIL:   $n_fail"
echo
echo "Per-fixture results: $RESULTS_DIR/<name>/result.json"
echo "Aggregate summary:   $RESULTS_DIR/summary.json"
[ "$n_fail" -eq 0 ]
