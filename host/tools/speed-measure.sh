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
# Targets host-v2 (override via RB_BIN). Reuses the parity-sandbox
# install layout so the install step doesn't dominate measurement.
#
# Usage: ./speed-measure.sh [list.txt] [out.csv]
#   Defaults: list=parity-exemplars.txt, out=speed-results.csv

set -uo pipefail
TOOLS="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$TOOLS/../.." && pwd)"
RB="${RB_BIN:-$ROOT/target/release/cruftless}"
# Sandbox layout matches parity-measure.sh exactly. The PARITY_SANDBOX
# env var is the same one parity-measure honors; setting it once in
# the shell (e.g. export PARITY_SANDBOX=/media/jaredef/T7/rusty-bun/parity-sandbox)
# points both scripts at the same install tree on the T7 mount so
# package installs are shared and don't double-cost.
SANDBOX_ROOT="${PARITY_SANDBOX:-/media/jaredef/T7/rusty-bun/parity-sandbox}"
LIST="${1:-$TOOLS/parity-exemplars.txt}"
OUT="${2:-$TOOLS/speed-results.csv}"
WARM_RUNS=3

if [ ! -x "$RB" ]; then
  echo "Binary not found: $RB" >&2
  echo "Build first: cargo build --release --bin $(basename "$RB")" >&2
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

# nanosecond-resolution wall-clock via /usr/bin/time -f's %e (sec).
time_ms() {
  # Returns wall-clock ms via /usr/bin/time -f format string.
  # $1 = command path, rest = args. Output to stderr from `time -v` is
  # captured + parsed; the command's stdout/stderr are silenced.
  local out
  out=$( { /usr/bin/time -f '%e' "$@" >/dev/null 2>/dev/null; } 2>&1 )
  # Convert seconds to ms; %e is seconds with 2 decimals.
  awk -v s="$out" 'BEGIN { printf "%.0f", s*1000 }'
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
  rb_t1=$(cd "$d" && time_ms "$RB" t1.mjs)

  # T2 — cold import (first run, OS cache warm; engine cold).
  bun_t2=$(cd "$d" && PARITY_PROBE_PKG="$pkg" time_ms bun tt.mjs)
  rb_t2=$(cd "$d" && PARITY_PROBE_PKG="$pkg" time_ms "$RB" tt.mjs)

  # T3 — warm repeat-mean over $WARM_RUNS runs.
  sum_bun=0; sum_rb=0
  for _ in $(seq 1 $WARM_RUNS); do
    bun_n=$(cd "$d" && PARITY_PROBE_PKG="$pkg" time_ms bun tt.mjs)
    rb_n=$(cd "$d" && PARITY_PROBE_PKG="$pkg" time_ms "$RB" tt.mjs)
    sum_bun=$((sum_bun + bun_n))
    sum_rb=$((sum_rb + rb_n))
  done
  bun_t3=$((sum_bun / WARM_RUNS))
  rb_t3=$((sum_rb / WARM_RUNS))

  ratio_t2=$(awk -v r="$rb_t2" -v b="$bun_t2" 'BEGIN { printf "%.2f", b>0?r/b:0 }')
  ratio_t3=$(awk -v r="$rb_t3" -v b="$bun_t3" 'BEGIN { printf "%.2f", b>0?r/b:0 }')

  printf "%s,%s,%s,%s,%s,%s,%s,%s,%s\n" \
    "$pkg" "$bun_t1" "$rb_t1" "$bun_t2" "$rb_t2" "$bun_t3" "$rb_t3" "$ratio_t2" "$ratio_t3" \
    >> "$OUT"
  n_done=$((n_done + 1))
done < "$LIST"

echo ""
echo "Wrote $n_done rows to $OUT"
echo ""
echo "Aggregate (rb/bun ratio, T3 warm-mean):"
awk -F, 'NR>1 && $9 != "0.00" {
  n++; sum += $9;
  if (n == 1 || $9 < min) min = $9;
  if (n == 1 || $9 > max) max = $9;
  ratios[n] = $9;
} END {
  asort(ratios);
  med = ratios[int(n/2)];
  p95 = ratios[int(n*0.95)];
  printf "  n=%d  geomean(approx-arith)=%.2fx  median=%.2fx  p95=%.2fx  min=%.2fx  max=%.2fx\n", n, sum/n, med, p95, min, max;
}' "$OUT"
