#!/usr/bin/env bash
# apparatus/scripts/caacp-sidecar.sh — thin wrapper for the LOCAL CAACP
# sidecar at http://127.0.0.1:7777 (companion to caacp.sh which talks
# directly to https://jaredfoy.com/api/caacp/v1).
#
# Preferred over caacp.sh going forward: the sidecar routes per-agent
# tokens, runs the polling loop, and writes notification files. Use
# caacp.sh only for admin/diagnostic direct-to-endpoint calls.
#
# Subcommands:
#   register <role> [instance_id] [callback_url]
#     POST /local/register; returns {token, role, instance_id,
#     notification_file}. Store token in your session memory.
#
#   send <sender_token> <recipient> <intent> <slug> [related_to]
#     Reads body from stdin. POST /local/send via the sidecar.
#
#   ack <ack_author_token> <original_message_id> <ack_state> <ack_slug>
#     Reads body from stdin. POST /local/ack via the sidecar.
#
#   inbox <role> [instance_id]
#     GET /local/inbox?role=...&instance_id=...
#
#   health
#     GET /local/health — sidecar status + registered agents.
#
# Configuration via env (with sensible defaults):
#   CAACP_SIDECAR_HOST     default 127.0.0.1
#   CAACP_SIDECAR_PORT     default 7777

set -euo pipefail

HOST="${CAACP_SIDECAR_HOST:-127.0.0.1}"
PORT="${CAACP_SIDECAR_PORT:-7777}"
BASE="http://${HOST}:${PORT}"

usage() {
    cat >&2 <<EOF
Usage: caacp-sidecar.sh <subcommand> [args...]

  register <role> [instance_id] [callback_url]
  send     <sender_token> <recipient> <intent> <slug> [related_to]    (body on stdin)
  ack      <ack_author_token> <original_message_id> <ack_state> <ack_slug>  (body on stdin)
  inbox    <role> [instance_id]
  health
EOF
    exit 1
}

require_jq() {
    command -v jq >/dev/null 2>&1 || { echo "jq required" >&2; exit 2; }
}

cmd_register() {
    require_jq
    local role="${1:-}" instance_id="${2:-}" callback_url="${3:-}"
    [[ -n "$role" ]] || usage
    local body
    body=$(jq -n \
        --arg role "$role" \
        --arg instance_id "$instance_id" \
        --arg callback_url "$callback_url" \
        '{role:$role, instance_id:(if $instance_id == "" then null else $instance_id end), callback_url:(if $callback_url == "" then null else $callback_url end)}')
    curl -sf -X POST -H "Content-Type: application/json" -d "$body" "${BASE}/local/register"
    echo
}

cmd_send() {
    require_jq
    local sender_token="${1:-}" recipient="${2:-}" intent="${3:-}" slug="${4:-}" related_to="${5:-}"
    [[ -n "$sender_token" && -n "$recipient" && -n "$intent" && -n "$slug" ]] || usage
    local body
    body=$(cat)
    local payload
    payload=$(jq -n \
        --arg sender_token "$sender_token" \
        --arg recipient "$recipient" \
        --arg intent "$intent" \
        --arg slug "$slug" \
        --arg related_to "$related_to" \
        --arg body "$body" \
        '{sender_token:$sender_token, recipient:$recipient, intent:$intent, slug:$slug, related_to:(if $related_to == "" then null else $related_to end), body:$body}')
    curl -sf -X POST -H "Content-Type: application/json" -d "$payload" "${BASE}/local/send"
    echo
}

cmd_ack() {
    require_jq
    local ack_author_token="${1:-}" original_message_id="${2:-}" ack_state="${3:-}" ack_slug="${4:-}"
    [[ -n "$ack_author_token" && -n "$original_message_id" && -n "$ack_state" && -n "$ack_slug" ]] || usage
    local body
    body=$(cat)
    local payload
    payload=$(jq -n \
        --arg ack_author_token "$ack_author_token" \
        --arg original_message_id "$original_message_id" \
        --arg ack_state "$ack_state" \
        --arg ack_slug "$ack_slug" \
        --arg body "$body" \
        '{ack_author_token:$ack_author_token, original_message_id:$original_message_id, ack_state:$ack_state, ack_slug:$ack_slug, body:$body}')
    curl -sf -X POST -H "Content-Type: application/json" -d "$payload" "${BASE}/local/ack"
    echo
}

cmd_inbox() {
    local role="${1:-}" instance_id="${2:-}"
    [[ -n "$role" ]] || usage
    local url="${BASE}/local/inbox?role=${role}"
    [[ -n "$instance_id" ]] && url="${url}&instance_id=${instance_id}"
    curl -sf "$url"
    echo
}

cmd_health() {
    curl -sf "${BASE}/local/health"
    echo
}

[[ $# -ge 1 ]] || usage
sub="$1"; shift
case "$sub" in
    register) cmd_register "$@" ;;
    send)     cmd_send     "$@" ;;
    ack)      cmd_ack      "$@" ;;
    inbox)    cmd_inbox    "$@" ;;
    health)   cmd_health        ;;
    *)        usage ;;
esac
