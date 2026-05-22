#!/usr/bin/env bash
# Differential prod-test runner — single fixture.
# Reads fixtures/<name>/manifest.json + exec.mjs and runs the workload
# under both engines, then diffs per category. See specs/diff-prod-testing.md.
#
# Usage:
#   ./run.sh <fixture-name>
# Env:
#   PROD_SANDBOX   — install root (default /tmp/diff-prod-sandbox)
#   RB_BIN         — cruftless binary (default $HOME/rusty-bun/target/release/cruftless)
#   RESULTS_DIR    — per-run results (default $HOME/rusty-bun/scripts/diff-prod/results)

set -uo pipefail

NAME="${1:?usage: $0 <fixture-name>}"
HERE="$(cd "$(dirname "$0")" && pwd)"
FIX="$HERE/fixtures/$NAME"
[ -d "$FIX" ] || { echo "no such fixture: $FIX" >&2; exit 2; }

PROD_SANDBOX="${PROD_SANDBOX:-/tmp/diff-prod-sandbox}"
RB_BIN="${RB_BIN:-$HOME/rusty-bun/target/release/cruftless}"
RESULTS_DIR="${RESULTS_DIR:-$HERE/results}"

SBOX="$PROD_SANDBOX/$NAME"
mkdir -p "$SBOX" "$RESULTS_DIR/$NAME"

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
      ( cd "$SBOX" && nice -n 19 bun add "$d" --silent 2>/dev/null >/dev/null )
    fi
  done
fi

# Copy fixture sources into sandbox.
for f in setup.mjs exec.mjs cassette.json; do
  [ -f "$FIX/$f" ] && cp "$FIX/$f" "$SBOX/$f"
done

# Optional setup (run once under bun; cruftless is the engine under test).
if [ -f "$SBOX/setup.mjs" ]; then
  ( cd "$SBOX" && timeout "$TIMEOUT_S" bun setup.mjs >/dev/null 2>&1 || true )
fi

# Run exec under bun.
bun_out=$(cd "$SBOX" && timeout "$TIMEOUT_S" bun exec.mjs 2>/tmp/diff-prod-bun.stderr)
bun_rc=$?
bun_err=$(cat /tmp/diff-prod-bun.stderr)

# Run exec under cruftless.
rb_out=$(cd "$SBOX" && timeout "$TIMEOUT_S" "$RB_BIN" exec.mjs 2>/tmp/diff-prod-rb.stderr)
rb_rc=$?
rb_err=$(cat /tmp/diff-prod-rb.stderr)

# Write per-engine snapshots.
python3 -c "
import json
json.dump({
  'fixture': '$NAME',
  'categories': '$CATS'.split(),
  'bun':       {'stdout': open('/dev/stdin').read(),
                'stderr': open('/tmp/diff-prod-bun.stderr').read(),
                'rc': $bun_rc},
}, open('$RESULTS_DIR/$NAME/bun.json','w'), indent=2)
" <<< "$bun_out"

python3 -c "
import json
json.dump({
  'fixture': '$NAME',
  'categories': '$CATS'.split(),
  'cruftless': {'stdout': open('/dev/stdin').read(),
                'stderr': open('/tmp/diff-prod-rb.stderr').read(),
                'rc': $rb_rc},
}, open('$RESULTS_DIR/$NAME/cruftless.json','w'), indent=2)
" <<< "$rb_out"

# Comparator dispatch.
node_or_bun=$(command -v bun || command -v node)
"$node_or_bun" "$HERE/runners/comparator.mjs" "$NAME" "$RESULTS_DIR/$NAME" "$CATS"
exit $?
