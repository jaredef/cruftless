// TB-EXT 7: tiny-baseline fast-path fuzz fixture.
//
// Exercises the closure-side metadata cache fast-path across patterns
// covering its (P2.c) hazard surface (TB seed §I.2 falsifier Pred-tb.5):
//
//   1. Mono   — same closure called repeatedly (cache populates, fast-path hot)
//   2. Multi  — many distinct closures interleaved (cache populates per-closure)
//   3. Mixed  — JIT-eligible + JIT-ineligible calls interleaved (some fast-path,
//               others fall through to standard dispatcher)
//   4. Arrow  — arrow closures with bound_this (must read bound_this correctly)
//   5. Deopt  — calls that pass arg-shape boundary then arg-shape mismatch
//               (forces fall-through on the mismatch; fast-path must invalidate
//               cell + retry correctly)
//
// Runs under cruft both with and without CRUFTLESS_LEJIT_TB=1; outputs must
// match byte-for-byte (Pred-tb.5 — no illegal-speed bug per Doc 735 §X.h.b).

function add(a, b) { return a + b; }
function mul(a, b) { return a * b; }
function sub(a, b) { return a - b; }
function inc(x) { return x + 1; }
function dbl(x) { return x * 2; }

// 1. Mono — single closure hot loop.
function mono(n) {
  let s = 0;
  for (let i = 0; i < n; i++) s = add(s, i);
  return s;
}

// 2. Multi — multiple closures interleaved.
function multi(n) {
  let s = 0;
  for (let i = 0; i < n; i++) {
    s = add(s, i);
    s = sub(s, 1);
    s = mul(s, 1);  // identity to keep value bounded
  }
  return s;
}

// 3. Mixed — JIT-eligible (Number args) interleaved with JIT-ineligible
//    (string concat). The standard dispatcher will JIT the Number paths
//    and bypass JIT for string paths. Fast-path cell must populate only
//    for JIT-eligible calls + correctly remain unpopulated for others.
function mixed(n) {
  let s = 0;
  let label = "";
  for (let i = 0; i < n; i++) {
    s = add(s, i);
    if ((i & 7) === 0) label = label + "x";  // string op disables JIT this site
  }
  return s + label.length;
}

// 4. Arrow with bound_this.
function makeArrow(base) {
  return (x) => base + x;
}

function arrowSum(n, base) {
  const f = makeArrow(base);
  let s = 0;
  for (let i = 0; i < n; i++) s = s + f(i);
  return s;
}

// 5. Deopt — typed-i64 path then non-integer Number forces deopt.
function deoptDriver(n) {
  let s = 0;
  for (let i = 0; i < n; i++) {
    s = add(s, i);              // JIT path on integer-Number args
    if (i === n / 2) {
      s = add(s, 0.5);          // non-integer → arg-shape boundary mismatch
    }
  }
  return s;
}

// Run all five patterns; accumulate into a single checksum.
const N = 300;
let acc = 0;

for (let r = 0; r < 50; r++) {
  acc = (acc + mono(N))         | 0;
  acc = (acc + multi(N))        | 0;
  acc = (acc + mixed(N))        | 0;
  acc = (acc + arrowSum(N, r))  | 0;
  acc = (acc + deoptDriver(N))  | 0;
}

console.log("fuzz-tb N=" + N + " R=50 acc=" + acc);
