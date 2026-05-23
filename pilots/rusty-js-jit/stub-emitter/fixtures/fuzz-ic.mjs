// StubE-EXT 7: IC fast-path fuzz fixture.
//
// Exercises the IC fast-path across monomorphic, polymorphic, and
// megamorphic property-access patterns + Dictionary fallback. Runs
// under cruft both with and without CRUFTLESS_LEJIT_STUB=1; the
// outputs must match byte-for-byte (Pred-stub.5 — no illegal-speed
// bug per Doc 735 §X.h.b).
//
// Reproducibility: deterministic generation seeded on N_OBJECTS so
// stdout-bytes-equality holds.
//
// Workload shapes:
//   1. Mono   — same shape, same property, 500 accesses → IC stays WarmMono
//   2. Bi     — alternating two shapes → IC oscillates ColdAfterMiss/WarmMono/Degraded
//   3. Mega   — 10 distinct shapes round-robin → IC degrades fast
//   4. DictIn — properties added after object creation triggering migrate_to_dictionary
//   5. Mixed  — interleaved patterns

function makeMono(n) {
  const arr = [];
  for (let i = 0; i < n; i++) arr.push({ x: i, y: i * 2 });
  return arr;
}

function makeBi(n) {
  const arr = [];
  for (let i = 0; i < n; i++) {
    arr.push((i & 1) === 0 ? { x: i, y: i * 2 } : { y: i * 2, x: i });
  }
  return arr;
}

function makeMega(n) {
  // 10 distinct shapes by adding a varying number of pre-properties.
  const arr = [];
  for (let i = 0; i < n; i++) {
    const o = {};
    const k = i % 10;
    if (k > 0) o.a = k;
    if (k > 1) o.b = k * 2;
    if (k > 2) o.c = k * 3;
    if (k > 3) o.d = k * 4;
    if (k > 4) o.e = k * 5;
    if (k > 5) o.f = k * 6;
    if (k > 6) o.g = k * 7;
    if (k > 7) o.h = k * 8;
    if (k > 8) o.i = k * 9;
    o.x = i;
    arr.push(o);
  }
  return arr;
}

function makeDictIn(n) {
  const arr = [];
  for (let i = 0; i < n; i++) {
    const o = { x: i };
    // Force migration to Dictionary via delete + add pattern (CMig-EXT
    // 5 / 8 ensures these sites migrate).
    if (i % 3 === 0) {
      o.tmp = i;
      delete o.tmp;
    }
    arr.push(o);
  }
  return arr;
}

function sumX(arr) {
  let total = 0;
  for (let i = 0; i < arr.length; i++) total = (total + arr[i].x) | 0;
  return total;
}

// Warm-up so JIT compiles + cache populates.
const warmup = makeMono(100);
for (let w = 0; w < 50; w++) sumX(warmup);

// Five-shape fuzz.
const N = 500;
const M = 100;  // 100 outer iterations to repeat-access (drives IC state)
let acc = 0;

for (let m = 0; m < M; m++) {
  const monoArr = makeMono(N);
  const biArr = makeBi(N);
  const megaArr = makeMega(N);
  const dictArr = makeDictIn(N);

  acc = (acc + sumX(monoArr)) | 0;
  acc = (acc + sumX(biArr)) | 0;
  acc = (acc + sumX(megaArr)) | 0;
  acc = (acc + sumX(dictArr)) | 0;
}

console.log("fuzz-ic N=" + N + " M=" + M + " acc=" + acc);
