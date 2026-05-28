#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
PARENT="$(cd "$HERE/../.." && pwd)"

CLFG_EXEMPLARS_LIST="$HERE/exemplars.txt" \
  "$PARENT/exemplars/run-exemplars.sh"
