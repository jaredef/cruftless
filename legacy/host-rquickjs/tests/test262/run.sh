#!/usr/bin/env bash
# Test262 fixture driver for cruftless.
#
# Iterates every .js file under host/tests/test262/tests/ (or a path
# passed as the first argument), invokes cruftless on each via the
# runner.mjs harness, and tallies PASS / FAIL / SKIP / TIMEOUT.
#
# Output: JSON-lines per test on stderr (so the per-test results can
# be redirected and inspected), summary block on stdout.
#
# Usage:
#   ./run.sh                          # all tests under host/tests/test262/tests/
#   ./run.sh path/under/tests/        # subset
#   ./run.sh path/to/single-test.js   # one test
#
# Per-test wall-clock cap: 10s (cruftless's parse+compile+eval is
# fast on the curated fixtures; 10s is well above what any single
# test should take). Exit code: 0 if zero FAILs, 1 otherwise.

set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
# shellcheck disable=SC1091
. "$ROOT/scripts/env.sh"
CRUFT="${CRUFT_BIN:-${RB_BIN:-$ROOT/target/release/cruft}}"
RUNNER="$HERE/runner.mjs"
# Ω.5.P61.E2: support running against the upstream test262 tree by
# setting T262_ROOT to the cloned repo. Harness + tests both come from
# upstream in that mode. Without T262_ROOT, use the cruftless-vendored
# curated tests + harness stand-ins.
if [ -n "${T262_ROOT:-}" ]; then
  HARNESS_DIR="$T262_ROOT/harness"
  DEFAULT_TESTS="$T262_ROOT/test"
else
  HARNESS_DIR="$HERE/harness"
  DEFAULT_TESTS="$HERE/tests"
fi

if [ ! -x "$CRUFT" ]; then
  echo "cruftless binary not found at $CRUFT" >&2
  echo "Build first: cargo build --release --bin cruftless" >&2
  exit 2
fi
if [ ! -f "$RUNNER" ]; then
  echo "Runner missing: $RUNNER" >&2
  exit 2
fi

ROOT_ARG="${1:-$DEFAULT_TESTS}"
if [ -f "$ROOT_ARG" ]; then
  TESTS="$ROOT_ARG"
elif [ -d "$ROOT_ARG" ]; then
  TESTS=$(find "$ROOT_ARG" -name '*.js' -type f | sort)
else
  echo "Path not found: $ROOT_ARG" >&2
  exit 2
fi

n_pass=0
n_fail=0
n_skip=0
n_timeout=0
total=0
fails=()

run_one() {
  if command -v timeout >/dev/null 2>&1; then
    T262_TEST_PATH="$1" T262_HARNESS_DIR="$HARNESS_DIR" \
      timeout 10s "$CRUFT" "$RUNNER" 2>/dev/null
    return $?
  fi

  perl -e '
    my ($limit, $test_path, $harness_dir, @cmd) = @ARGV;
    my $pid = fork;
    die "fork failed: $!" unless defined $pid;
    if ($pid == 0) {
      $ENV{T262_TEST_PATH} = $test_path;
      $ENV{T262_HARNESS_DIR} = $harness_dir;
      exec @cmd;
      exit 127;
    }
    local $SIG{ALRM} = sub {
      kill "TERM", $pid;
      sleep 1;
      kill "KILL", $pid;
      waitpid($pid, 0);
      exit 124;
    };
    alarm $limit;
    waitpid($pid, 0);
    my $status = $?;
    alarm 0;
    exit($status & 127 ? 128 + ($status & 127) : $status >> 8);
  ' 10 "$1" "$HARNESS_DIR" "$CRUFT" "$RUNNER" 2>/dev/null
}

for t in $TESTS; do
  total=$((total + 1))
  rel="${t#$DEFAULT_TESTS/}"
  rel="${rel#$HERE/tests/}"
  out=$(run_one "$t")
  rc=$?
  if [ $rc -eq 124 ]; then
    n_timeout=$((n_timeout + 1))
    fails+=("$rel: TIMEOUT")
    echo "{\"path\":\"$rel\",\"status\":\"TIMEOUT\"}" >&2
    continue
  fi
  echo "$out" >&2
  status=$(printf '%s' "$out" | python3 -c '
import sys, json
try:
    d = json.loads(sys.stdin.read().strip().split("\n")[-1])
    print(d.get("status","FAIL"))
except: print("FAIL")
' 2>/dev/null)
  case "$status" in
    PASS) n_pass=$((n_pass + 1)) ;;
    SKIP) n_skip=$((n_skip + 1)) ;;
    FAIL)
      n_fail=$((n_fail + 1))
      fails+=("$rel: $(printf '%s' "$out" | tail -1 | python3 -c '
import sys, json
try:
    d = json.loads(sys.stdin.read().strip())
    print(d.get("reason","")[:120])
except: print("(bad json)")
' 2>/dev/null)")
      ;;
    *)
      n_fail=$((n_fail + 1))
      fails+=("$rel: unknown status '$status'")
      ;;
  esac
done

echo
echo "═══════════════════════════════════════════════════════════════"
echo "Test262 fixture summary"
echo "═══════════════════════════════════════════════════════════════"
echo "Total:    $total"
echo "Pass:     $n_pass"
echo "Fail:     $n_fail"
echo "Skip:     $n_skip"
echo "Timeout:  $n_timeout"
echo
if [ $total -gt 0 ]; then
  pct=$(echo "scale=1; $n_pass * 100 / ($total - $n_skip)" | bc 2>/dev/null || echo "n/a")
  echo "Pass rate (excluding skips): $pct% ($n_pass / $((total - n_skip)))"
fi

if [ ${#fails[@]} -gt 0 ]; then
  echo
  echo "Failures + timeouts (first 20):"
  for ((i=0; i<${#fails[@]} && i<20; i++)); do
    echo "  ${fails[$i]}"
  done
fi

if [ $n_fail -gt 0 ] || [ $n_timeout -gt 0 ]; then exit 1; fi
exit 0
