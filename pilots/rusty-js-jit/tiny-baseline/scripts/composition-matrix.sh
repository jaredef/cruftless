#!/usr/bin/env bash
# TB-EXT 4: composition matrix across bench_call_overhead + bench_ic
# under all 8 combinations of CRUFTLESS_LEJIT_{TB,STUB,VTI} flags.
#
# Tests Pred-tb.2 (composition target on bench_ic ≤ 90 ns under
# shape + STUB + TB) and quantifies per-flag contribution + any
# composition synergy or interference effects.
#
# Output: markdown table written to docs/composition-matrix.md and
# echoed to stdout. N=5 runs per (bench × config); median reported.
set -euo pipefail

PILOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
REPO_ROOT="$(cd "$PILOT_DIR/../../.." && pwd)"
OUT="$PILOT_DIR/docs/composition-matrix.md"

RUNS=${RUNS:-5}

BENCHES=(bench_call_overhead bench_ic)

# 8 configurations.
declare -A CONFIGS=(
  [none]=""
  [TB]="CRUFTLESS_LEJIT_TB=1"
  [STUB]="CRUFTLESS_LEJIT_STUB=1"
  [VTI]="CRUFTLESS_LEJIT_VTI=1"
  [TB+STUB]="CRUFTLESS_LEJIT_TB=1 CRUFTLESS_LEJIT_STUB=1"
  [TB+VTI]="CRUFTLESS_LEJIT_TB=1 CRUFTLESS_LEJIT_VTI=1"
  [STUB+VTI]="CRUFTLESS_LEJIT_STUB=1 CRUFTLESS_LEJIT_VTI=1"
  [TB+STUB+VTI]="CRUFTLESS_LEJIT_TB=1 CRUFTLESS_LEJIT_STUB=1 CRUFTLESS_LEJIT_VTI=1"
)
# Order to print.
ORDER=(none TB STUB VTI TB+STUB TB+VTI STUB+VTI TB+STUB+VTI)

median() {
  sort -n | awk '
    { vals[NR]=$1 } END {
      n=NR; if (n==0) { print "NaN"; exit }
      if (n%2) print vals[(n+1)/2]
      else printf "%.1f\n", (vals[n/2]+vals[n/2+1])/2
    }'
}

cd "$REPO_ROOT"
{
  echo "# TB-EXT 4 — composition matrix"
  echo
  echo "*N=$RUNS per (bench × config); median ns/iter. Generated $(date -Iseconds).*"
  echo
  echo "| config | bench_call_overhead | bench_ic |"
  echo "|---|---:|---:|"
} > "$OUT"

for cfg in "${ORDER[@]}"; do
  env="${CONFIGS[$cfg]}"
  row="| $cfg |"
  for bench in "${BENCHES[@]}"; do
    runs=()
    for ((i=1; i<=RUNS; i++)); do
      # Each bench prints "per-iter: X ns" — extract X.
      ns=$(env $env "$REPO_ROOT/target/release/examples/$bench" 2>/dev/null \
        | awk '/per-iter:/ { print $2 }')
      runs+=("$ns")
    done
    med=$(printf '%s\n' "${runs[@]}" | median)
    echo "  $cfg $bench: ${runs[*]} -> $med ns" >&2
    row="$row $med |"
  done
  echo "$row" >> "$OUT"
done

echo >> "$OUT"
echo "## Per-flag contribution (delta from \`none\`)" >> "$OUT"
echo >> "$OUT"
cat "$OUT"
echo
echo "wrote: $OUT"
