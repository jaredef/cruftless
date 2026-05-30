#!/usr/bin/env bash
# caacp-resolver-complete.sh — send the mandatory terminal Helmsman report
# for a bridged resolver directive and stamp the bridge active ledger.

set -euo pipefail

HOST="${CAACP_SIDECAR_HOST:-127.0.0.1}"
PORT="${CAACP_SIDECAR_PORT:-7777}"
BASE="http://${HOST}:${PORT}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DATA_DIR="${REPO_ROOT}/apparatus/caacp-server/data"

usage() {
    cat >&2 <<'EOF'
Usage:
  caacp-resolver-complete.sh <sender_token> <role> <instance_id|-> <inbound_message_id> <slug> [intent]

Reads terminal report body from stdin, sends it to Helmsman with
related_to=<inbound_message_id>, and records the returned message_id in the
Codex bridge active ledger for <role>/<instance_id>.

Use "-" for instance_id when the bridged role is singleton/no-instance.
Default intent is "notification".
EOF
    exit 1
}

require_jq() {
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; exit 2; }
}

[[ $# -ge 5 ]] || usage
require_jq

TOKEN="$1"
ROLE="$2"
INSTANCE_ID="$3"
INBOUND_ID="$4"
SLUG="$5"
INTENT="${6:-notification}"
BODY="$(cat)"

if [[ "$INSTANCE_ID" == "-" ]]; then
    BRIDGE_ID="$ROLE"
else
    BRIDGE_ID="${ROLE}-${INSTANCE_ID}"
fi
ACTIVE_FILE="${DATA_DIR}/bridge-${BRIDGE_ID}-codex-app-active.json"

payload="$(jq -n \
    --arg sender_token "$TOKEN" \
    --arg recipient "helmsman" \
    --arg intent "$INTENT" \
    --arg slug "$SLUG" \
    --arg related_to "$INBOUND_ID" \
    --arg body "$BODY" \
    '{sender_token:$sender_token, recipient:$recipient, intent:$intent, slug:$slug, related_to:$related_to, body:$body}')"

resp="$(curl -sf -X POST -H "Content-Type: application/json" -d "$payload" "${BASE}/local/send")"
message_id="$(printf '%s' "$resp" | jq -r '.message_id')"
timestamp="$(printf '%s' "$resp" | jq -r '.server_timestamp // empty')"

if [[ -z "$message_id" || "$message_id" == "null" ]]; then
    echo "caacp-resolver-complete.sh: sidecar send did not return message_id" >&2
    printf '%s\n' "$resp" >&2
    exit 3
fi

mkdir -p "$DATA_DIR"
if [[ ! -f "$ACTIVE_FILE" ]]; then
    printf '{}\n' > "$ACTIVE_FILE"
fi

tmp="$(mktemp "${ACTIVE_FILE}.XXXXXX")"
jq \
    --arg id "$INBOUND_ID" \
    --arg mid "$message_id" \
    --arg ts "${timestamp:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}" \
    --arg slug "$SLUG" \
    '
    .[$id] = ((.[$id] // {}) + {
      requires_helmsman_final: true,
      helmsman_final_message_id: $mid,
      terminal_report_sent_at: $ts,
      terminal_report_related_to: $id,
      terminal_report_slug: $slug,
      state: "COMPLETED"
    })
    ' "$ACTIVE_FILE" > "$tmp"
mv "$tmp" "$ACTIVE_FILE"

printf '%s\n' "$resp"
