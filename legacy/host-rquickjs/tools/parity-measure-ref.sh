#!/usr/bin/env bash
# Reference-parameterized parity measurement.
#
# Identical to parity-measure.sh, except the reference engine is selectable
# via REF_BIN (default: bun). Set REF_BIN=node to measure cruft's package
# parity against Node.js — the actual compatibility target — rather than Bun.
#
# Packages are still installed with `bun add` (fast, idempotent); the populated
# node_modules is read by both the reference engine and cruft. The output JSON
# records the reference engine's output under the "ref" key (with "ref_name"),
# cruft's under "rb".
#
# For each package:
#   1. bun add into an isolated tempdir (cached/idempotent)
#   2. Run parity-probe.mjs under $REF_BIN  → reference output
#   3. Run parity-probe.mjs under cruft     → candidate output
#   4. Compare byte-for-byte (mutual-ERR counts as parity, per parity-measure.sh)
#
# Usage: REF_BIN=node ./parity-measure-ref.sh [list.txt] [out.json]
set -uo pipefail
TOOLS="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$TOOLS/../.." && pwd)"
LIST="${1:-$TOOLS/parity-top500.txt}"
OUT="${2:-$TOOLS/parity-results-ref.json}"

REF_BIN="${REF_BIN:-bun}"
REF_NAME="${REF_NAME:-$(basename "$REF_BIN")}"
INSTALLER="${INSTALLER:-bun}"
SANDBOX="${PARITY_SANDBOX:-/tmp/parity-sandbox}"
mkdir -p "$SANDBOX"

CRUFT="${CRUFT_BIN:-${RB_BIN:-$ROOT/target/release/cruft}}"
if [ ! -x "$CRUFT" ]; then
  echo "Binary not found: $CRUFT"; echo "Build first: cargo build --release --bin cruft -p cruftless"; exit 1
fi
if ! command -v "$REF_BIN" >/dev/null 2>&1 && [ ! -x "$REF_BIN" ]; then
  echo "Reference engine not found: $REF_BIN"; exit 1
fi
echo "Reference engine: $REF_NAME ($REF_BIN); candidate: $CRUFT"

PKGS=$(grep -vE '^\s*(#|$)' "$LIST" | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')

echo "[" > "$OUT"
first=1
n_pass=0; n_fail=0; n_skip=0; total=0
mismatch_log=()

for pkg in $PKGS; do
  total=$((total + 1))
  safe="${pkg//\//--}"
  d="$SANDBOX/$safe"
  if [ ! -d "$d/node_modules/$pkg" ]; then
    mkdir -p "$d"
    (
      cd "$d"
      [ -f package.json ] || echo '{"name":"sb","version":"0.0.0"}' > package.json
      timeout 120s nice -n 19 ionice -c3 "$INSTALLER" add "$pkg" > /dev/null 2>&1
    )
    sleep 0.3
  fi

  if [ ! -d "$d/node_modules/$pkg" ]; then
    status="SKIP_INSTALL_FAILED"
    n_skip=$((n_skip + 1))
    if [ $first -eq 0 ]; then echo "," >> "$OUT"; fi
    pkg_json=$(printf '%s' "$pkg" | python3 -c 'import sys,json; print(json.dumps(sys.stdin.read()))')
    echo "  {\"pkg\":$pkg_json,\"status\":\"SKIP_INSTALL_FAILED\"}" >> "$OUT"
    first=0
    continue
  fi

  cp "$TOOLS/parity-probe.mjs" "$d/parity-probe.mjs"
  ref_out=$(cd "$d" && PARITY_PROBE_PKG="$pkg" timeout 30s nice -n 19 ionice -c3 "$REF_BIN" parity-probe.mjs 2>/dev/null)
  ref_rc=$?
  rb_out=$(cd "$d" && PARITY_PROBE_PKG="$pkg" timeout 30s nice -n 19 ionice -c3 "$CRUFT" parity-probe.mjs 2>/dev/null)
  rb_rc=$?

  ref_is_err=$(printf '%s' "$ref_out" | grep -q '"status":"ERR"' && echo 1 || echo 0)
  rb_is_err=$(printf '%s' "$rb_out" | grep -q '"status":"ERR"' && echo 1 || echo 0)
  if [ $ref_rc -eq 124 ] || [ $rb_rc -eq 124 ]; then
    status="TIMEOUT"; n_fail=$((n_fail + 1))
    mismatch_log+=("$pkg: TIMEOUT ref_rc=$ref_rc rb_rc=$rb_rc")
  elif [ "$ref_out" = "$rb_out" ] && [ -n "$ref_out" ]; then
    status="PASS"; n_pass=$((n_pass + 1))
  elif [ "$ref_is_err" = "1" ] && [ "$rb_is_err" = "1" ]; then
    status="MATCH_OK_ERR_BOTH"; n_pass=$((n_pass + 1))
  else
    status="FAIL"; n_fail=$((n_fail + 1))
    mismatch_log+=("$pkg: ref=${ref_out:0:80} rb=${rb_out:0:80}")
  fi

  if [ $first -eq 0 ]; then echo "," >> "$OUT"; fi
  ref_json=$(printf '%s' "$ref_out" | python3 -c 'import sys,json; print(json.dumps(sys.stdin.read().strip()))')
  rb_json=$(printf '%s' "$rb_out" | python3 -c 'import sys,json; print(json.dumps(sys.stdin.read().strip()))')
  pkg_json=$(printf '%s' "$pkg" | python3 -c 'import sys,json; print(json.dumps(sys.stdin.read().strip()))')
  status_json=$(printf '%s' "$status" | python3 -c 'import sys,json; print(json.dumps(sys.stdin.read().strip()))')
  refname_json=$(printf '%s' "$REF_NAME" | python3 -c 'import sys,json; print(json.dumps(sys.stdin.read().strip()))')
  echo "  {\"pkg\":$pkg_json,\"status\":$status_json,\"ref_name\":$refname_json,\"ref\":$ref_json,\"rb\":$rb_json}" >> "$OUT"
  first=0
done

echo "]" >> "$OUT"

echo
echo "═══════════════════════════════════════════════════════════════"
echo "Parity measurement summary (reference: $REF_NAME)"
echo "═══════════════════════════════════════════════════════════════"
echo "Total:    $total"
echo "Pass:     $n_pass"
echo "Fail:     $n_fail"
echo "Skip:     $n_skip"
echo
if [ $total -gt 0 ]; then
  pct=$(echo "scale=1; $n_pass * 100 / $total" | bc)
  echo "Parity:   $pct% ($n_pass / $total)"
fi
echo "Results JSON: $OUT"
if [ ${#mismatch_log[@]} -gt 0 ]; then
  echo; echo "Mismatches (first 10):"
  for ((i=0; i<${#mismatch_log[@]} && i<10; i++)); do echo "  ${mismatch_log[$i]}"; done
fi
