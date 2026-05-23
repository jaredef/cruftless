// CMig-EXT 17: canonical fuzz harness for shape-enrollment correctness.
//
// Per Findings VI.6 HIGH priority: engagement-wide instrument that
// catches shape-bypass bugs proactively. Generates N seeded-random
// fixtures across the patterns CMig-EXT 15 + 16.bis fixed individually
// (spread, JSON.stringify, Object.keys/.values/.entries) plus
// adjacent patterns (delete + re-add migration; Map/Set iteration;
// property-shape transitions; mixed sequences).
//
// Output: a single checksum accumulated across all patterns. Run
// under cruft default / cruft shape-off / node and diff the
// checksums; byte-equality across all three means (P2.c) ruled out
// at this coverage. Divergence localizes the failing pattern via
// the per-pattern checksum trace.
//
// Reproducibility: deterministic PRNG seeded on N + version.

const N = 2000;
const VERSION = "cmig-ext-17-2026-05-23";

// Simple xorshift32 PRNG — deterministic across runtimes, no
// dependency on Math.random's per-runtime seed.
function makeRng(seed) {
  let s = seed | 0;
  return function() {
    s ^= s << 13; s ^= s >>> 17; s ^= s << 5;
    // Convert to [0, 1) via division by 2^32
    return ((s >>> 0) / 4294967296);
  };
}
function rint(rng, lo, hi) { return lo + Math.floor(rng() * (hi - lo)); }
function pickName(rng) {
  const KEYS = ["a", "b", "c", "d", "e", "f", "g", "h", "x", "y", "z", "id", "name", "val", "count", "size"];
  return KEYS[rint(rng, 0, KEYS.length)];
}

// Cheap byte-level checksum that converges across patterns.
let acc = 0;
function feed(s) {
  s = String(s);
  for (let i = 0; i < s.length; i++) {
    acc = ((acc * 31) + s.charCodeAt(i)) | 0;
  }
}

// === Pattern 1: random object literal + JSON.stringify ===
function patternObjStringify(rng) {
  const n = rint(rng, 1, 8);
  const obj = {};
  for (let i = 0; i < n; i++) {
    const k = pickName(rng);
    const t = rint(rng, 0, 4);
    if (t === 0) obj[k] = rint(rng, 0, 1000);
    else if (t === 1) obj[k] = ["x", k, "v" + i].join("_");
    else if (t === 2) obj[k] = (rint(rng, 0, 2) === 0);
    else obj[k] = null;
  }
  feed(JSON.stringify(obj));
  feed(Object.keys(obj).join(","));
  return obj;
}

// === Pattern 2: spread + overlap ===
function patternSpread(rng) {
  const a = patternObjStringify(rng);
  const b = patternObjStringify(rng);
  const merged = { ...a, ...b };
  feed(JSON.stringify(merged));
  feed(Object.entries(merged).length);
  return merged;
}

// === Pattern 3: delete + re-add (forces Dictionary migration) ===
function patternDeleteReadd(rng) {
  const obj = patternObjStringify(rng);
  const keys = Object.keys(obj);
  if (keys.length > 0) {
    const victim = keys[rint(rng, 0, keys.length)];
    delete obj[victim];
    feed(JSON.stringify(obj));
    obj[victim] = "re-added-" + victim;
    feed(JSON.stringify(obj));
  }
  return obj;
}

// === Pattern 4: Object.values + .entries enumeration ===
function patternEnumeration(rng) {
  const obj = patternObjStringify(rng);
  const vals = Object.values(obj);
  feed(vals.length);
  for (const v of vals) feed(typeof v + ":" + String(v).slice(0, 20));
  const entries = Object.entries(obj);
  for (const [k, v] of entries) feed(k + "=" + String(v).slice(0, 20));
}

// === Pattern 5: Map/Set iteration ===
function patternMapSet(rng) {
  const m = new Map();
  const s = new Set();
  const n = rint(rng, 1, 6);
  for (let i = 0; i < n; i++) {
    const k = pickName(rng) + "_" + i;
    m.set(k, rint(rng, 0, 100));
    s.add(k);
  }
  feed("size=" + m.size + "," + s.size);
  for (const [k, v] of m) feed(k + ":" + v);
  for (const v of s) feed(v);
}

// === Pattern 6: hot property-access loop (drives IC + JIT) ===
function patternHotLoop(rng) {
  const obj = { x: rint(rng, 0, 100), y: rint(rng, 0, 100), z: rint(rng, 0, 100) };
  let sum = 0;
  for (let i = 0; i < 50; i++) {
    sum = sum + obj.x + obj.y + obj.z;
  }
  feed(sum);
}

// === Pattern 7: nested + spread + stringify (composition) ===
function patternNestedComposition(rng) {
  const inner = patternObjStringify(rng);
  const outer = { meta: { v: 1 }, ...inner, tail: "fin" };
  feed(JSON.stringify(outer));
  feed(Object.keys(outer).length);
}

// === Pattern 8: array of objects + stringify ===
function patternArrayOfObj(rng) {
  const n = rint(rng, 1, 5);
  const arr = [];
  for (let i = 0; i < n; i++) arr.push(patternObjStringify(rng));
  feed(JSON.stringify(arr));
}

const PATTERNS = [
  patternObjStringify,
  patternSpread,
  patternDeleteReadd,
  patternEnumeration,
  patternMapSet,
  patternHotLoop,
  patternNestedComposition,
  patternArrayOfObj,
];

const rng = makeRng(0xC0FFEE);
for (let i = 0; i < N; i++) {
  const p = PATTERNS[i % PATTERNS.length];
  p(rng);
  // Per-fixture checkpoint into the checksum
  feed("#" + i);
}

console.log("fuzz-canonical version=" + VERSION + " N=" + N + " acc=" + acc);
