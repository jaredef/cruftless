// OSR-EXT 5e (2026-05-23): synthetic do-while-shape bench fixture for
// empirical validation of the OSR invoke path end-to-end (5b + 5c + 5d).
//
// Loop body uses ONLY JIT-alphabet ops:
//   LoadLocal / StoreLocal / Op::Add / Op::Lt / PushI32 / JumpIfTrue
// No GetProp, no CallMethod, no LoadGlobal, no MakeClosure inside the
// loop body. The trailing console.log is outside the loop and doesn't
// affect OSR firing on the back-edge (OSR fires per-back-edge-site).
//
// Expected outcome with OSR-EXT 5d wired:
//   - try_osr_compile fires at back-edge iter 1000 (OSR_BACK_EDGE_THRESHOLD)
//   - compile_function_osr SUCCEEDS (alphabet covered; do-while shape so
//     no forward exit outside the slice per Finding OSR.2)
//   - try_osr_invoke fires on every back-edge from iter 1000 onward
//   - JIT body runs remaining iters internally; locals marshaled in + out
//   - Final sum matches expected (correctness via Number-only locals)

let sum = 0;
let i = 0;
const N = 1000000;
do {
  sum = sum + i;
  i = i + 1;
} while (i < N);

// Expected sum = N*(N-1)/2 = 1000000 * 999999 / 2 = 499999500000
console.log("synth-do-while sum=" + sum + " expected=" + (N * (N - 1) / 2));
