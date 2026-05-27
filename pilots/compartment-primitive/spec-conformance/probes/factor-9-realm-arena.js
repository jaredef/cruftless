// Factor 9: many short-lived compartments should not leak unboundedly.
// Probe creates N compartments and checks process is alive at end.
// Substrate-arena leak isn't directly observable from JS; this probe
// is a coarse smoke test (no crash, no obvious hang).
const N = 100;
let last;
for (let i = 0; i < N; i++) {
  last = new Compartment({globals: {i: i}});
}
console.log("survived_N:", N);
console.log("last_i:", last.globalThis.i);
console.log("REFUTED_IF_CRASH_OR_HANG:", false);
