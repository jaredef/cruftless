#!/usr/bin/env bash
# apparatus/scripts/caacp-tmux-bridge.sh
#
# Cybernetic bridge for agent runtimes (OpenAI Codex CLI, etc.) that
# lack native file-watch + task-notification primitives. Polls the
# local CAACP sidecar's /local/inbox for a given role and, when new
# messages arrive, injects a short directive into a tmux pane via
# send-keys so the running agent session is interrupted with a
# wake-and-check-inbox prompt.
#
# Design per the watcher's refinements (Telegram 10272):
#   - Polls the LIVE endpoint /local/inbox?role=<role>, not the
#     inbound-<role>.json file (the file can retain stale payloads
#     after the message has been acked).
#   - Maintains a seen-cache at apparatus/caacp-server/data/bridge-
#     <role>-seen.json so each message_id triggers exactly one
#     injection.
#   - Injects a SHORT directive ("**CAACP NEW** role=X count=N
#     latest=<sender>/<intent>/<slug>. Check sidecar inbox before
#     continuing.") — NOT the full body. The agent reads the inbox
#     itself.
#   - Verifies the tmux target exists before entering the loop; logs
#     failures to apparatus/caacp-server/data/bridge-<role>.log.
#   - Operator-started ONLY. Not auto-invoked from any other repo
#     script. Injecting text into an interactive pane is powerful +
#     context-sensitive; the operator decides which Codex session
#     gets bridged.
#
# Usage:
#   caacp-tmux-bridge.sh <role> <tmux-target> [poll-interval-seconds]
# Example:
#   caacp-tmux-bridge.sh watcher codex-watcher:0.0 5

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DATA_DIR="$REPO_ROOT/apparatus/caacp-server/data"
SIDECAR_HOST="${CAACP_SIDECAR_HOST:-127.0.0.1}"
SIDECAR_PORT="${CAACP_SIDECAR_PORT:-7777}"

usage() {
    cat >&2 <<EOF
Usage: caacp-tmux-bridge.sh <role> <tmux-target> [poll-interval-seconds]

Arguments:
  role                 substrate-resolver | helmsman | arbiter | watcher | deputy
  tmux-target          tmux pane target (e.g. "codex-watcher:0.0", "main:1")
  poll-interval        seconds between polls; default 5

Environment:
  CAACP_SIDECAR_HOST   default 127.0.0.1
  CAACP_SIDECAR_PORT   default 7777
EOF
    exit 1
}

require_jq() {
    command -v jq >/dev/null 2>&1 || { echo "caacp-tmux-bridge.sh: jq required" >&2; exit 2; }
}

[[ $# -ge 2 ]] || usage
ROLE="$1"
TARGET="$2"
INTERVAL="${3:-5}"
require_jq

mkdir -p "$DATA_DIR"
SEEN_FILE="$DATA_DIR/bridge-${ROLE}-seen.json"
LOG_FILE="$DATA_DIR/bridge-${ROLE}.log"
[[ -f "$SEEN_FILE" ]] || echo '[]' > "$SEEN_FILE"

log() {
    local ts
    ts="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
    echo "[caacp-tmux-bridge] $ts role=$ROLE $*" | tee -a "$LOG_FILE" >&2
}

# Pre-flight: verify tmux target exists. tmux's target syntax is
# session:window.pane; has-session validates the session prefix.
SESSION="${TARGET%%:*}"
if ! command -v tmux >/dev/null 2>&1; then
    log "FATAL: tmux not installed; bridge cannot inject"
    exit 3
fi
if ! tmux has-session -t "$SESSION" 2>/dev/null; then
    log "FATAL: tmux session '$SESSION' not found; bridge cannot inject"
    exit 3
fi
# Verify the full target (window.pane) exists by listing panes.
if ! tmux list-panes -t "$TARGET" >/dev/null 2>&1; then
    log "FATAL: tmux target '$TARGET' (full pane) not found"
    exit 3
fi

log "starting; target=$TARGET interval=${INTERVAL}s seen-cache=$SEEN_FILE"

while true; do
    # Source-of-truth: live endpoint via sidecar (not the stale-prone
    # inbound-<role>.json file).
    RESP="$(curl -sf "http://${SIDECAR_HOST}:${SIDECAR_PORT}/local/inbox?role=${ROLE}" 2>/dev/null || true)"
    if [[ -z "$RESP" ]]; then
        log "WARN: sidecar /local/inbox?role=${ROLE} unreachable (or empty); will retry"
        sleep "$INTERVAL"
        continue
    fi

    # Extract PENDING messages.
    MSG_IDS="$(echo "$RESP" | jq -r '.messages[]? | select(.state=="PENDING") | .message_id')"
    if [[ -z "$MSG_IDS" ]]; then
        sleep "$INTERVAL"
        continue
    fi

    # Diff against the seen-cache.
    SEEN="$(cat "$SEEN_FILE")"
    NEW_IDS=()
    while IFS= read -r mid; do
        [[ -z "$mid" ]] && continue
        if ! echo "$SEEN" | jq -e --arg id "$mid" 'index($id)' >/dev/null 2>&1; then
            NEW_IDS+=("$mid")
        fi
    done <<< "$MSG_IDS"

    if [[ ${#NEW_IDS[@]} -eq 0 ]]; then
        sleep "$INTERVAL"
        continue
    fi

    # Build the directive. Use the LATEST new message for the
    # latest=<sender>/<intent>/<slug> field; report total PENDING count.
    LATEST_MID="${NEW_IDS[-1]}"
    LATEST="$(echo "$RESP" | jq -r --arg id "$LATEST_MID" '.messages[] | select(.message_id==$id) | "\(.sender)/\(.intent)/\(.slug)"')"
    COUNT="$(echo "$MSG_IDS" | wc -l | tr -d ' ')"
    DIRECTIVE="**CAACP NEW** role=${ROLE} count=${COUNT} latest=${LATEST}. Check sidecar inbox before continuing."

    # Inject via tmux send-keys. Append Enter so the prompt is submitted.
    tmux send-keys -t "$TARGET" -- "$DIRECTIVE" Enter
    log "injected: $DIRECTIVE  (new_ids=${NEW_IDS[*]})"

    # Update seen-cache.
    NEW_SEEN="$(echo "$SEEN" | jq --argjson new "$(printf '%s\n' "${NEW_IDS[@]}" | jq -R . | jq -s .)" '. + $new | unique | .[-1000:]')"
    echo "$NEW_SEEN" > "$SEEN_FILE"

    sleep "$INTERVAL"
done
