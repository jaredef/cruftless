#!/usr/bin/env bash
# Parity sweep targeting cruftless-rquickjs (host/, the rquickjs ceiling
# reference). Thin shim: sets RB_BIN to the rquickjs binary, then defers
# the actual sweep logic to parity-measure.sh.
#
# Usage: ./parity-measure-rquickjs.sh [list.txt] [out.json]
# Defaults: list=parity-top500.txt, out=parity-results-rquickjs.json

set -uo pipefail
TOOLS="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$TOOLS/../.." && pwd)"
LIST="${1:-$TOOLS/parity-top500.txt}"
OUT="${2:-$TOOLS/parity-results-rquickjs.json}"
exec env RB_BIN="$ROOT/target/release/cruftless-rquickjs" \
  "$TOOLS/parity-measure.sh" "$LIST" "$OUT"
