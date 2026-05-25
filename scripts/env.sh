#!/usr/bin/env bash
# Shared local environment loader for cruftless scripts.
#
# Scripts source this file after they have computed a nearby script root.
# env.local supplies machine-local defaults; repo-relative fallbacks keep
# scripts portable when no local file is present.

cruftless_env_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cruftless_env_root="${CRUFTLESS_ROOT:-$(cd "$cruftless_env_dir/.." && pwd)}"

if [ -f "$cruftless_env_root/env.local" ]; then
  set -a
  # shellcheck disable=SC1091
  . "$cruftless_env_root/env.local"
  set +a
fi

CRUFTLESS_ROOT="${CRUFTLESS_ROOT:-$cruftless_env_root}"
CRUFT_BIN="${CRUFT_BIN:-${RB_BIN:-$CRUFTLESS_ROOT/target/release/cruft}}"
BUN_BIN="${BUN_BIN:-bun}"
NODE_BIN="${NODE_BIN:-node}"
T262_ROOT="${T262_ROOT:-}"
PROD_SANDBOX="${PROD_SANDBOX:-/tmp/cruftless-diff-prod-sandbox}"
RESULTS_DIR="${RESULTS_DIR:-$CRUFTLESS_ROOT/results/diff-prod}"
PROBE_ROOT="${PROBE_ROOT:-/tmp/cruftless-ak-probe}"
CRUFTLESS_CROSS_RUNTIME_RESULTS_ROOT="${CRUFTLESS_CROSS_RUNTIME_RESULTS_ROOT:-}"
LOCAL_CRUFT="${LOCAL_CRUFT:-$HOME/bin/cruft}"

export CRUFTLESS_ROOT CRUFT_BIN BUN_BIN NODE_BIN PROD_SANDBOX RESULTS_DIR PROBE_ROOT LOCAL_CRUFT
if [ -n "$T262_ROOT" ]; then
  export T262_ROOT
fi
if [ -n "$CRUFTLESS_CROSS_RUNTIME_RESULTS_ROOT" ]; then
  export CRUFTLESS_CROSS_RUNTIME_RESULTS_ROOT
fi

unset cruftless_env_dir cruftless_env_root
