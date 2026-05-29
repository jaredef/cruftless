#!/usr/bin/env bash
# apparatus/scripts/caacp.sh — thin wrapper for the Cybernetic Agentic
# Communication Protocol (CAACP) per apparatus/docs/cybernetic-agentic-
# communication-protocol.md.
#
# Provides four subcommands a resolver session uses to participate in
# CAACP: send (author + register message), inbox (list pending),
# outbox (list outgoing with unread acks), ack (write acknowledgment).
#
# Authentication: CAACP_TOKEN env var (loaded from env.local via
# scripts/env.sh). Without the token, the wrapper operates in
# degraded mode: writes on-disk artifacts but skips endpoint sync;
# logs sync intent to apparatus/caacp/sync-failures/ for later replay
# when Stage B endpoint is live and the token is provisioned.
#
# Endpoint base: https://jaredfoy.com/api/caacp/v1 (per CAACP §VI.1).

set -euo pipefail

ENDPOINT_BASE="${CAACP_ENDPOINT:-https://jaredfoy.com/api/caacp/v1}"
REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CAACP_ROOT="$REPO_ROOT/apparatus/caacp"

usage() {
    cat >&2 <<EOF
Usage: caacp.sh <subcommand> [args...]

Subcommands:
  send <sender> <recipient> <intent> <slug> [related_to]
      Author a CAACP message. Reads body from stdin. Writes canonical
      artifact at apparatus/caacp/inbox/<recipient>/<timestamp>-<slug>.md
      with a symlink at apparatus/caacp/outbox/<sender>/<timestamp>-<slug>.md.
      POSTs to endpoint if CAACP_TOKEN set; logs to sync-failures/
      otherwise.
      Intent: request | notification | response | broadcast | veto-pending.

  inbox <role> [--state PENDING|ACKNOWLEDGED|IN-FLIGHT]
      List pending messages for <role> from on-disk inbox + endpoint
      reconciliation (if token set). Output is one line per message:
      <message-id> <sender> <intent> <slug>.

  outbox <role> [--unread-acks]
      List outgoing messages for <role> with at least one unread
      acknowledgment from the recipient (default behavior). Output is
      one line per message: <message-id> <recipient> <intent> <slug>
      <last-ack-state>.

  ack <original-message-path> <ack-author> <ack-state>
      Write an acknowledgment artifact at
      apparatus/caacp/acknowledgments/<timestamp>-<message-id>-<state>.md.
      ack-state: ACKNOWLEDGED | IN-FLIGHT | RESOLVED. POSTs to endpoint
      if CAACP_TOKEN set.

Environment:
  CAACP_TOKEN     authentication token; absent = degraded mode
  CAACP_ENDPOINT  override base URL (default: https://jaredfoy.com/api/caacp/v1)
EOF
    exit 1
}

require_jq() {
    if ! command -v jq >/dev/null 2>&1; then
        echo "caacp.sh: jq required for CAACP message handling" >&2
        exit 2
    fi
}

iso_timestamp() {
    date -u +"%Y-%m-%dT%H:%M:%SZ"
}

slug_timestamp() {
    date -u +"%Y-%m-%dT%H%M%SZ"
}

content_sha() {
    sha256sum "$1" | awk '{print $1}'
}

endpoint_available() {
    [[ -n "${CAACP_TOKEN:-}" ]]
}

log_sync_failure() {
    local kind="$1" detail="$2"
    local ts
    ts="$(slug_timestamp)"
    mkdir -p "$CAACP_ROOT/sync-failures"
    cat > "$CAACP_ROOT/sync-failures/${ts}-${kind}.md" <<EOF
---
kind: $kind
attempted_at: $(iso_timestamp)
endpoint: $ENDPOINT_BASE
token_set: $(endpoint_available && echo true || echo false)
---

$detail
EOF
}

curl_post() {
    local path="$1" body="$2"
    curl --silent --show-error --fail \
        --header "X-CAACP-Token: $CAACP_TOKEN" \
        --header "Content-Type: application/json" \
        --request POST \
        --data "$body" \
        "${ENDPOINT_BASE}${path}"
}

curl_get() {
    local path="$1"
    curl --silent --show-error --fail \
        --header "X-CAACP-Token: $CAACP_TOKEN" \
        --request GET \
        "${ENDPOINT_BASE}${path}"
}

cmd_send() {
    require_jq
    local sender="${1:-}" recipient="${2:-}" intent="${3:-}" slug="${4:-}" related_to="${5:-}"
    [[ -n "$sender" && -n "$recipient" && -n "$intent" && -n "$slug" ]] || usage

    local valid_intents="request notification response broadcast veto-pending acknowledgment"
    if ! echo "$valid_intents" | tr ' ' '\n' | grep -qx "$intent"; then
        echo "caacp.sh: invalid intent '$intent'; valid: $valid_intents" >&2
        exit 2
    fi

    local ts
    ts="$(slug_timestamp)"
    local artifact_slug="${ts}-${slug}"
    local recipient_dir="$CAACP_ROOT/inbox/$recipient"
    local sender_dir="$CAACP_ROOT/outbox/$sender"
    mkdir -p "$recipient_dir" "$sender_dir"

    local artifact="$recipient_dir/${artifact_slug}.md"

    # Capture stdin body to temporary file for sha computation
    local body_tmp
    body_tmp="$(mktemp)"
    cat > "$body_tmp"
    local body_sha
    body_sha="$(content_sha "$body_tmp")"

    # Build frontmatter
    local session_id="${CAACP_SESSION_ID:-unknown-session}"
    {
        echo "---"
        echo "caacp_version: 1"
        echo "message_id: pending-endpoint-assignment"
        echo "sender: $sender"
        echo "recipient: $recipient"
        echo "intent: $intent"
        [[ -n "$related_to" ]] && echo "related_to: $related_to"
        echo "state: PENDING"
        echo "slug: $slug"
        echo "created_at: $(iso_timestamp)"
        echo "session_id: $session_id"
        echo "content_sha: $body_sha"
        echo "---"
        echo ""
        cat "$body_tmp"
    } > "$artifact"
    rm -f "$body_tmp"

    # Symlink outbox → inbox
    ln -sf "../../inbox/$recipient/${artifact_slug}.md" "$sender_dir/${artifact_slug}.md"

    # Endpoint sync
    local message_id="local-only-$artifact_slug"
    if endpoint_available; then
        local payload
        payload=$(jq -n \
            --arg sender "$sender" \
            --arg recipient "$recipient" \
            --arg intent "$intent" \
            --arg slug "$slug" \
            --arg related_to "$related_to" \
            --arg content_sha "$body_sha" \
            '{sender:$sender, recipient:$recipient, intent:$intent, slug:$slug, related_to:($related_to | select(. != "")), content_sha:$content_sha}')
        local response
        if response=$(curl_post "/messages" "$payload" 2>&1); then
            message_id=$(echo "$response" | jq -r '.message_id')
            # Patch the artifact's message_id in place
            sed -i "s|^message_id: pending-endpoint-assignment$|message_id: $message_id|" "$artifact"
        else
            log_sync_failure "send-failed" "send sender=$sender recipient=$recipient slug=$slug failed: $response"
        fi
    else
        log_sync_failure "send-degraded" "send sender=$sender recipient=$recipient slug=$slug; CAACP_TOKEN unset; artifact written at $artifact (replay on Stage B activation)"
    fi

    echo "$artifact"
    echo "message_id=$message_id" >&2
}

cmd_inbox() {
    local role="${1:-}"
    local state_filter="${2:-PENDING}"
    [[ -n "$role" ]] || usage
    local dir="$CAACP_ROOT/inbox/$role"
    [[ -d "$dir" ]] || { echo "caacp.sh: no inbox for role '$role'" >&2; exit 0; }

    for artifact in "$dir"/*.md; do
        [[ -f "$artifact" ]] || continue
        local frontmatter
        frontmatter=$(awk '/^---$/{n++; if(n==2) exit; next} n==1 {print}' "$artifact")
        local artifact_state
        artifact_state=$(echo "$frontmatter" | awk '/^state:/{print $2; exit}')
        if [[ "$state_filter" == "ALL" || "$artifact_state" == "$state_filter" ]]; then
            local msg_id sender intent slug
            msg_id=$(echo "$frontmatter" | awk '/^message_id:/{print $2; exit}')
            sender=$(echo "$frontmatter" | awk '/^sender:/{print $2; exit}')
            intent=$(echo "$frontmatter" | awk '/^intent:/{print $2; exit}')
            slug=$(echo "$frontmatter" | awk '/^slug:/{print $2; exit}')
            printf "%s\t%s\t%s\t%s\n" "$msg_id" "$sender" "$intent" "$slug"
        fi
    done
}

cmd_outbox() {
    local role="${1:-}"
    [[ -n "$role" ]] || usage
    local dir="$CAACP_ROOT/outbox/$role"
    [[ -d "$dir" ]] || { echo "caacp.sh: no outbox for role '$role'" >&2; exit 0; }

    for artifact in "$dir"/*.md; do
        [[ -e "$artifact" ]] || continue
        local frontmatter
        frontmatter=$(awk '/^---$/{n++; if(n==2) exit; next} n==1 {print}' "$artifact")
        local msg_id recipient intent slug
        msg_id=$(echo "$frontmatter" | awk '/^message_id:/{print $2; exit}')
        recipient=$(echo "$frontmatter" | awk '/^recipient:/{print $2; exit}')
        intent=$(echo "$frontmatter" | awk '/^intent:/{print $2; exit}')
        slug=$(echo "$frontmatter" | awk '/^slug:/{print $2; exit}')
        # Find latest ack for this message_id
        local latest_ack
        latest_ack=$(find "$CAACP_ROOT/acknowledgments" -name "*-${msg_id}-*.md" 2>/dev/null | sort | tail -1)
        local ack_state="(no-ack)"
        if [[ -n "$latest_ack" ]]; then
            ack_state=$(awk '/^state:/{print $2; exit}' "$latest_ack")
        fi
        printf "%s\t%s\t%s\t%s\t%s\n" "$msg_id" "$recipient" "$intent" "$slug" "$ack_state"
    done
}

cmd_ack() {
    require_jq
    local original_path="${1:-}" ack_author="${2:-}" ack_state="${3:-}"
    [[ -n "$original_path" && -n "$ack_author" && -n "$ack_state" ]] || usage
    [[ -f "$original_path" ]] || { echo "caacp.sh: original message not found at $original_path" >&2; exit 2; }

    local valid_states="ACKNOWLEDGED IN-FLIGHT RESOLVED"
    if ! echo "$valid_states" | tr ' ' '\n' | grep -qx "$ack_state"; then
        echo "caacp.sh: invalid ack-state '$ack_state'; valid: $valid_states" >&2
        exit 2
    fi

    local frontmatter
    frontmatter=$(awk '/^---$/{n++; if(n==2) exit; next} n==1 {print}' "$original_path")
    local original_msg_id original_sender original_slug original_intent
    original_msg_id=$(echo "$frontmatter" | awk '/^message_id:/{print $2; exit}')
    original_sender=$(echo "$frontmatter" | awk '/^sender:/{print $2; exit}')
    original_slug=$(echo "$frontmatter" | awk '/^slug:/{print $2; exit}')
    original_intent=$(echo "$frontmatter" | awk '/^intent:/{print $2; exit}')

    local ts
    ts="$(slug_timestamp)"
    mkdir -p "$CAACP_ROOT/acknowledgments"
    local ack_file="$CAACP_ROOT/acknowledgments/${ts}-${original_msg_id}-${ack_state}.md"

    # Read action body from stdin
    local body_tmp
    body_tmp="$(mktemp)"
    cat > "$body_tmp"
    local body_sha
    body_sha="$(content_sha "$body_tmp")"

    local session_id="${CAACP_SESSION_ID:-unknown-session}"
    {
        echo "---"
        echo "caacp_version: 1"
        echo "message_id: pending-endpoint-assignment"
        echo "sender: $ack_author"
        echo "recipient: $original_sender"
        echo "intent: acknowledgment"
        echo "related_to: $original_msg_id"
        echo "state: $ack_state"
        echo "slug: ack-$original_slug"
        echo "created_at: $(iso_timestamp)"
        echo "session_id: $session_id"
        echo "content_sha: $body_sha"
        echo "---"
        echo ""
        echo "## Acknowledged"
        echo ""
        echo "$original_msg_id from $original_sender, intent $original_intent, slug $original_slug."
        echo ""
        echo "## Action taken"
        echo ""
        cat "$body_tmp"
    } > "$ack_file"
    rm -f "$body_tmp"

    # Endpoint sync
    if endpoint_available; then
        local payload
        payload=$(jq -n \
            --arg ack_author "$ack_author" \
            --arg ack_state "$ack_state" \
            --arg ack_slug "ack-$original_slug" \
            --arg content_sha "$body_sha" \
            '{ack_author:$ack_author, ack_intent:$ack_state, ack_slug:$ack_slug, content_sha:$content_sha}')
        local response
        if response=$(curl_post "/messages/${original_msg_id}/acknowledge" "$payload" 2>&1); then
            local new_msg_id
            new_msg_id=$(echo "$response" | jq -r '.ack_id')
            sed -i "s|^message_id: pending-endpoint-assignment$|message_id: $new_msg_id|" "$ack_file"
        else
            log_sync_failure "ack-failed" "ack original=$original_msg_id author=$ack_author state=$ack_state failed: $response"
        fi
    else
        log_sync_failure "ack-degraded" "ack original=$original_msg_id author=$ack_author state=$ack_state; CAACP_TOKEN unset; artifact at $ack_file"
    fi

    echo "$ack_file"
}

[[ $# -ge 1 ]] || usage
subcommand="$1"; shift

case "$subcommand" in
    send)   cmd_send   "$@" ;;
    inbox)  cmd_inbox  "$@" ;;
    outbox) cmd_outbox "$@" ;;
    ack)    cmd_ack    "$@" ;;
    *)      usage ;;
esac
