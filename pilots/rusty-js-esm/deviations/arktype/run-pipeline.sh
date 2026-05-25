#!/usr/bin/env bash
# Deviation-resolution pipeline runner for arktype.
# Runs each level (L0, L1, …) on both engines and captures structured artifacts.
#
# Usage:
#   ./run-pipeline.sh capture <level>    — run level <level>, write capture JSON
#   ./run-pipeline.sh trace <level>      — run level <level> with instrumentation
#   ./run-pipeline.sh diff <level>       — diff bun vs cruftless trace
#
# Env:
#   PROBE_ROOT — directory containing node_modules/arktype (default from env.local or /tmp)
#   CRUFT_BIN  — cruftless binary (default from env.local or target/release/cruft)

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$HERE/../../../.." && pwd)"
# shellcheck disable=SC1091
. "$REPO_ROOT/scripts/env.sh"

case "${1:-}" in
  capture)
    level="${2:-L0}"
    probe="$HERE/captures/${level}-probe.mjs"
    [ -f "$probe" ] || { echo "missing $probe"; exit 1; }
    out="$HERE/captures/${level}-result.json"
    cp "$probe" "$PROBE_ROOT/_pipeline_probe.mjs"
    cd "$PROBE_ROOT"
    bun_out=$("$BUN_BIN" "$PROBE_ROOT/_pipeline_probe.mjs" 2>&1 || true)
    rb_out=$("$CRUFT_BIN" "$PROBE_ROOT/_pipeline_probe.mjs" 2>&1 || true)
    printf '{"level":"%s","bun":%s,"cruftless":%s}\n' \
      "$level" \
      "$(printf %s "$bun_out" | python3 -c 'import sys,json;print(json.dumps(sys.stdin.read()))')" \
      "$(printf %s "$rb_out"  | python3 -c 'import sys,json;print(json.dumps(sys.stdin.read()))')" \
      > "$out"
    echo "wrote $out"
    ;;
  trace)
    level="${2:-L4}"
    probe="$HERE/captures/${level}-probe.mjs"
    [ -f "$probe" ] || { echo "missing $probe"; exit 1; }
    cp "$probe" "$PROBE_ROOT/_pipeline_probe.mjs"
    cd "$PROBE_ROOT"
    "$BUN_BIN" "$PROBE_ROOT/_pipeline_probe.mjs" > "$HERE/traces/${level}-bun.jsonl" 2>&1 || true
    "$CRUFT_BIN" "$PROBE_ROOT/_pipeline_probe.mjs" > "$HERE/traces/${level}-cruftless.jsonl" 2>&1 || true
    echo "wrote traces/${level}-{bun,cruftless}.jsonl"
    ;;
  diff)
    level="${2:-L4}"
    diff -u "$HERE/traces/${level}-bun.jsonl" "$HERE/traces/${level}-cruftless.jsonl" \
      > "$HERE/traces/${level}-diff.txt" 2>&1 || true
    head -60 "$HERE/traces/${level}-diff.txt"
    ;;
  *)
    echo "usage: $0 {capture|trace|diff} <level>" >&2
    exit 1
    ;;
esac
