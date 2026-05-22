#!/usr/bin/env bash
# Differential prod-test runner — single fixture.
# Reads fixtures/<name>/manifest.json + exec.mjs and runs the workload
# under both engines, then diffs per category. See specs/diff-prod-testing.md.
#
# Usage:
#   ./run.sh <fixture-name>
# Env:
#   PROD_SANDBOX   — install root (default /media/jaredef/T7/rusty-bun/diff-prod-sandbox)
#   RB_BIN         — cruftless binary (default $HOME/rusty-bun/target/release/cruftless)
#   RESULTS_DIR    — per-run results (default /media/jaredef/T7/rusty-bun/diff-prod-results)
#
# Runs all heavy work behind `nice -n 19 ionice -c3` so the harness can
# run alongside a workstation session without disrupting it. Sandbox +
# results default to the T7 mounted drive to keep system disk lean.

set -uo pipefail

NAME="${1:?usage: $0 <fixture-name>}"
HERE="$(cd "$(dirname "$0")" && pwd)"
FIX="$HERE/fixtures/$NAME"
[ -d "$FIX" ] || { echo "no such fixture: $FIX" >&2; exit 2; }

PROD_SANDBOX="${PROD_SANDBOX:-/media/jaredef/T7/rusty-bun/diff-prod-sandbox}"
RB_BIN="${RB_BIN:-$HOME/rusty-bun/target/release/cruftless}"
RESULTS_DIR="${RESULTS_DIR:-/media/jaredef/T7/rusty-bun/diff-prod-results}"

# Nice/ionice wrapper. If ionice isn't installed, fall back to nice-only.
if command -v ionice >/dev/null 2>&1; then
  NICE_WRAP=(nice -n 19 ionice -c3)
else
  NICE_WRAP=(nice -n 19)
fi

SBOX="$PROD_SANDBOX/$NAME"
RESULTS="$RESULTS_DIR/$NAME"
TMP="$RESULTS/_tmp"
mkdir -p "$SBOX" "$RESULTS" "$TMP"

# Read manifest fields (jq-free for portability).
MANIFEST="$FIX/manifest.json"
[ -f "$MANIFEST" ] || { echo "no manifest in $FIX" >&2; exit 2; }
DEPS=$(python3 -c "import json;d=json.load(open('$MANIFEST'));print(' '.join(d.get('deps',[])))")
CATS=$(python3 -c "import json;d=json.load(open('$MANIFEST'));print(' '.join(d.get('categories',['L'])))")
TIMEOUT_MS=$(python3 -c "import json;d=json.load(open('$MANIFEST'));print(d.get('timeout-ms',30000))")
TIMEOUT_S=$(( TIMEOUT_MS / 1000 + 1 ))

# Install deps (idempotent — bun add is no-op if already there).
if [ -n "$DEPS" ]; then
  ( cd "$SBOX" && [ -f package.json ] || echo '{"name":"diff-prod-sbox"}' > package.json )
  for d in $DEPS; do
    if [ ! -d "$SBOX/node_modules/$d" ]; then
      ( cd "$SBOX" && "${NICE_WRAP[@]}" bun add "$d" --silent 2>/dev/null >/dev/null )
    fi
  done
fi

# Copy fixture sources into sandbox.
for f in setup.mjs exec.mjs cassette.json; do
  [ -f "$FIX/$f" ] && cp "$FIX/$f" "$SBOX/$f"
done

# Optional setup (run once under bun; cruftless is the engine under test).
if [ -f "$SBOX/setup.mjs" ]; then
  ( cd "$SBOX" && timeout "$TIMEOUT_S" "${NICE_WRAP[@]}" bun setup.mjs >/dev/null 2>&1 || true )
fi

# Run exec under bun.
bun_out=$(cd "$SBOX" && timeout "$TIMEOUT_S" "${NICE_WRAP[@]}" bun exec.mjs 2>"$TMP/bun.stderr")
bun_rc=$?

# Run exec under cruftless.
rb_out=$(cd "$SBOX" && timeout "$TIMEOUT_S" "${NICE_WRAP[@]}" "$RB_BIN" exec.mjs 2>"$TMP/rb.stderr")
rb_rc=$?

# Write per-engine snapshots.
python3 -c "
import json, sys
json.dump({
  'fixture': '$NAME',
  'categories': '$CATS'.split(),
  'bun':       {'stdout': sys.stdin.read(),
                'stderr': open('$TMP/bun.stderr').read(),
                'rc': $bun_rc},
}, open('$RESULTS/bun.json','w'), indent=2)
" <<< "$bun_out"

python3 -c "
import json, sys
json.dump({
  'fixture': '$NAME',
  'categories': '$CATS'.split(),
  'cruftless': {'stdout': sys.stdin.read(),
                'stderr': open('$TMP/rb.stderr').read(),
                'rc': $rb_rc},
}, open('$RESULTS/cruftless.json','w'), indent=2)
" <<< "$rb_out"

# Clean tmp.
rm -rf "$TMP"

# Comparator dispatch (also nicely; the diff is cheap but consistent).
node_or_bun=$(command -v bun || command -v node)
"${NICE_WRAP[@]}" "$node_or_bun" "$HERE/runners/comparator.mjs" "$NAME" "$RESULTS" "$CATS"
exit $?
