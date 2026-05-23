// CRB-EXT 9 fixture: arith_tight_loop
//
// Maximally JIT-eligible workload for testing Pred-crb.5 (cruft's
// relative position improves under JIT-eligible workloads vs JIT-
// ineligible realistic-mixed workloads).
//
// Pure-integer-arithmetic hot loop with NO property access, NO
// callback dispatch, NO allocation in the inner loop. The inner
// function `sum(n)` uses only ops in cruft's JIT translator's
// supported set: PushI32, LoadLocal, StoreLocal, Add, Lt,
// JumpIfFalse. After the first call (threshold=1) the JIT compiles
// `sum` and all subsequent calls run JIT-emitted code; the per-call
// dispatcher cost amortizes across the deep inner loop.
//
// Output: final aggregate + total iteration count (for cross-runtime
// stdout-bytes-equality).

function sum(n) {
  let s = 0;
  let i = 0;
  while (i < n) {
    s = s + i;
    i = i + 1;
  }
  return s;
}

const N_INNER = 1000;     // 1000 iters per call → triangular number 499500
const M_OUTER = 100000;   // 100000 outer calls → 100M total inner iters

// Warm-up: JIT-compile + first-call interp pass.
for (let w = 0; w < 10; w++) sum(N_INNER);

let acc = 0;
for (let k = 0; k < M_OUTER; k++) {
  acc = acc + sum(N_INNER);
}

console.log("arith_tight_loop inner=" + N_INNER + " outer=" + M_OUTER + " acc=" + acc);
