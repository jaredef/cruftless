#!/usr/bin/env bash
# Speed-parity sweep. For each exemplar package, time how long it takes
# Bun vs cruftless to bun-install + cold-import + first-key-shape-probe.
# Three latencies captured per package per engine:
#   T1 — process startup + module-resolve overhead (probe with no import)
#   T2 — cold import-to-first-key wall time (one-shot)
#   T3 — warm-cache repeat-import time (3× run mean)
# Output: per-package CSV row + aggregate summary (median + p95 +
# geomean of rb/bun ratio).
#
# Targets host-v2 (override via CRUFT_BIN). Reuses the parity-sandbox
# install layout so the install step doesn't dominate measurement.
#
# Usage: ./speed-measure.sh [list.txt] [out.csv]
#   Defaults: list=parity-exemplars.txt, out=speed-results.csv

set -uo pipefail
TOOLS="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$TOOLS/../.." && pwd)"
CRUFT="${CRUFT_BIN:-${RB_BIN:-$ROOT/target/release/cruft}}"
# Sandbox layout matches parity-measure.sh exactly. The PARITY_SANDBOX
# env var is the same one parity-measure honors; setting it once in
# the shell (e.g. export PARITY_SANDBOX=/media/jaredef/T7/rusty-bun/parity-sandbox)
# points both scripts at the same install tree on the T7 mount so
# package installs are shared and don't double-cost.
SANDBOX_ROOT="${PARITY_SANDBOX:-/media/jaredef/T7/rusty-bun/parity-sandbox}"
LIST="${1:-$TOOLS/parity-exemplars.txt}"
OUT="${2:-$TOOLS/speed-results.csv}"
WARM_RUNS=3

if [ ! -x "$CRUFT" ]; then
  echo "Binary not found: $CRUFT" >&2
  echo "Build first: cargo build --release --bin $(basename "$CRUFT")" >&2
  exit 1
fi

# T1 probe — process up, no import. Measures cold-start overhead only.
T1_PROBE="/tmp/t1-probe.mjs"
cat > "$T1_PROBE" << 'PROBE'
// noop — process up + parser warmed.
PROBE

# T2/T3 probe — import the env-named package + a tiny shape touch.
T_PROBE="/tmp/t-probe.mjs"
cat > "$T_PROBE" << 'PROBE'
const pkg = process.env.PARITY_PROBE_PKG;
try {
  const m = await import(pkg);
  // Touch one key so dead-code-elimination can't skip the import.
  const k = Object.keys(m)[0] || "";
  process.stdout.write(k.length.toString() + "\n");
} catch (_) {
  process.stdout.write("ERR\n");
}
PROBE

# Nanosecond-resolution wall-clock via date +%s%N. /usr/bin/time's %e
# is second-rounded which collapses sub-second measurements to 0;
# `date +%s%N` (GNU date) provides ns granularity for the same shape.
#
# Wraps in `timeout 30s` because cruftless can hang on packages with
# top-level IPC/network side effects; without a cap, one bad package
# stalls the sweep (and on a Pi can wedge the box). Non-zero exit
# (including 124 = timeout) returns sentinel -1 so the aggregate skips
# the row instead of folding a meaningless number into the ratio.
TIME_BUDGET="${TIME_BUDGET:-30s}"
time_ms() {
  local t0 t1 rc
  t0=$(date +%s%N)
  timeout "$TIME_BUDGET" "$@" >/dev/null 2>/dev/null
  rc=$?
  t1=$(date +%s%N)
  if [ $rc -ne 0 ]; then
    echo -1
  else
    echo $(( (t1 - t0) / 1000000 ))
  fi
}

echo "pkg,bun_t1_ms,rb_t1_ms,bun_t2_ms,rb_t2_ms,bun_t3_ms,rb_t3_ms,ratio_t2,ratio_t3" > "$OUT"

n_done=0
while IFS= read -r pkg; do
  [ -z "$pkg" ] && continue
  [[ "$pkg" == \#* ]] && continue
  d="$SANDBOX_ROOT/${pkg//\//--}"
  [ -d "$d" ] || continue

  cp "$T1_PROBE" "$d/t1.mjs"
  cp "$T_PROBE" "$d/tt.mjs"

  # T1 — engine cold-start with empty script.
  bun_t1=$(cd "$d" && time_ms bun t1.mjs)
  rb_t1=$(cd "$d" && time_ms "$CRUFT" t1.mjs)

  # T2 — cold import (first run, OS cache warm; engine cold).
  bun_t2=$(cd "$d" && PARITY_PROBE_PKG="$pkg" time_ms bun tt.mjs)
  rb_t2=$(cd "$d" && PARITY_PROBE_PKG="$pkg" time_ms "$CRUFT" tt.mjs)

  # T3 — warm repeat-mean over $WARM_RUNS runs. Any sentinel (-1) from
  # time_ms taints the whole T3 row → propagate as -1.
  sum_bun=0; sum_rb=0; bad_t3=0
  for _ in $(seq 1 $WARM_RUNS); do
    bun_n=$(cd "$d" && PARITY_PROBE_PKG="$pkg" time_ms bun tt.mjs)
    rb_n=$(cd "$d" && PARITY_PROBE_PKG="$pkg" time_ms "$CRUFT" tt.mjs)
    if [ "$bun_n" -lt 0 ] || [ "$rb_n" -lt 0 ]; then bad_t3=1; break; fi
    sum_bun=$((sum_bun + bun_n))
    sum_rb=$((sum_rb + rb_n))
  done
  if [ $bad_t3 -eq 1 ]; then
    bun_t3=-1; rb_t3=-1
  else
    bun_t3=$((sum_bun / WARM_RUNS))
    rb_t3=$((sum_rb / WARM_RUNS))
  fi

  # Ratios: 0 if either side is a sentinel or zero. The aggregate awk
  # filters on `$9+0 > 0` so sentinel rows are excluded from stats.
  ratio_t2=$(awk -v r="$rb_t2" -v b="$bun_t2" 'BEGIN { printf "%.2f", (b>0 && r>=0)?r/b:0 }')
  ratio_t3=$(awk -v r="$rb_t3" -v b="$bun_t3" 'BEGIN { printf "%.2f", (b>0 && r>=0)?r/b:0 }')

  # Brief cool-down between packages — the Pi crashed mid-sweep before
  # this was added. 250ms is enough to let the scheduler/thermal settle
  # without materially slowing the run.
  sleep 0.25

  printf "%s,%s,%s,%s,%s,%s,%s,%s,%s\n" \
    "$pkg" "$bun_t1" "$rb_t1" "$bun_t2" "$rb_t2" "$bun_t3" "$rb_t3" "$ratio_t2" "$ratio_t3" \
    >> "$OUT"
  n_done=$((n_done + 1))
done < "$LIST"

echo ""
echo "Wrote $n_done rows to $OUT"
echo ""
echo "Aggregate (rb/bun ratio, T3 warm-mean):"
awk -F, 'NR>1 && $9+0 > 0 {
  n++; sum += $9; log_sum += log($9);
  if (n == 1 || $9 < min) min = $9;
  if (n == 1 || $9 > max) max = $9;
  ratios[n] = $9;
} END {
  if (n == 0) { print "  (no valid rows)"; exit }
  # mawk-portable sort (no asort): simple in-place insertion sort.
  for (i = 2; i <= n; i++) {
    key = ratios[i]; j = i - 1;
    while (j >= 1 && ratios[j] > key) { ratios[j+1] = ratios[j]; j-- }
    ratios[j+1] = key
  }
  med_idx = int((n+1)/2); if (med_idx < 1) med_idx = 1;
  p95_idx = int(n*0.95); if (p95_idx < 1) p95_idx = 1;
  # Geomean is the right aggregate for ratio data — arith-mean is
  # pulled by outliers (one 4x package skews the whole sweep).
  geomean = exp(log_sum / n);
  printf "  n=%d  geomean=%.2fx  median=%.2fx  arith-mean=%.2fx  p95=%.2fx  min=%.2fx  max=%.2fx\n", n, geomean, ratios[med_idx], sum/n, ratios[p95_idx], min, max;
}' "$OUT"
