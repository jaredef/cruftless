// JSF-EXT 8 (per Finding VII.1): component A/B probe on
// json_parse_transform. Isolates per-component cost by running the
// pipeline in additive variants. Each variant runs 500 iters; per-variant
// wall-clock reports the cumulative cost up to that stage. Δ(stage N) -
// Δ(stage N-1) ≈ stage N's per-iter cost contribution.
//
// Variants (additive, each subsuming the prior):
//   V0 — parse only: parsed = JSON.parse(text); no transform/stringify
//   V1 — V0 + filter: filtered = parsed.records.filter(...)
//   V2 — V1 + map: mapped = filtered.map(...)
//   V3 — V2 + stringify: out = JSON.stringify(mapped)
//   V4 — V3 + checksum: cs += out.charCodeAt(i)  (= full json_parse_transform)
//
// Read: V3 - V2 = JSON.stringify cost; V4 - V3 = checksum cost; etc.

function makePayload(n) {
  const recs = [];
  for (let i = 0; i < n; i++) {
    recs.push({
      id: i,
      name: "record_" + i,
      score: (i * 13) % 1000,
      active: (i & 1) === 0,
      tags: ["alpha", "beta", "gamma", (i % 7 === 0) ? "special" : "normal"],
      meta: {
        created: 1700000000 + i * 60,
        updated: 1700000000 + i * 60 + (i % 17) * 3,
        version: (i % 5) + 1,
      },
    });
  }
  return { config: { version: 1, generated: 1700000000 }, records: recs };
}

const ITER = 500;
const N_RECS = 100;

const payload = makePayload(N_RECS);
const text = JSON.stringify(payload);

function variant(name, fn) {
  // Warm: 50 iters not measured.
  for (let i = 0; i < 50; i++) fn(i);
  const t0 = Date.now();
  let sink = 0;
  for (let i = 0; i < ITER; i++) sink = (sink + fn(i)) | 0;
  const dt = Date.now() - t0;
  console.log("VARIANT " + name + " ms=" + dt + " sink=" + sink);
  return dt;
}

const v0 = variant("V0_parse", (i) => {
  const parsed = JSON.parse(text);
  return parsed.records.length;
});

const v1 = variant("V1_parse_filter", (i) => {
  const parsed = JSON.parse(text);
  const filtered = parsed.records.filter((r) => r.active && r.score > 100);
  return filtered.length;
});

const v2 = variant("V2_parse_filter_map", (i) => {
  const parsed = JSON.parse(text);
  const filtered = parsed.records.filter((r) => r.active && r.score > 100);
  const mapped = filtered.map((r) => ({
    id: r.id,
    label: r.name + "_" + r.meta.version,
    bucket: Math.floor(r.score / 100),
  }));
  return mapped.length;
});

const v3 = variant("V3_parse_filter_map_stringify", (i) => {
  const parsed = JSON.parse(text);
  const filtered = parsed.records.filter((r) => r.active && r.score > 100);
  const mapped = filtered.map((r) => ({
    id: r.id,
    label: r.name + "_" + r.meta.version,
    bucket: Math.floor(r.score / 100),
  }));
  const out = JSON.stringify(mapped);
  return out.length;
});

const v4 = variant("V4_full_with_checksum", (i) => {
  const parsed = JSON.parse(text);
  const filtered = parsed.records.filter((r) => r.active && r.score > 100);
  const mapped = filtered.map((r) => ({
    id: r.id,
    label: r.name + "_" + r.meta.version,
    bucket: Math.floor(r.score / 100),
  }));
  const out = JSON.stringify(mapped);
  let cs = 0;
  for (let i = 0; i < out.length; i++) cs = (cs + out.charCodeAt(i)) | 0;
  return cs;
});

console.log("---");
console.log("delta_parse              = " + v0);
console.log("delta_filter             = " + (v1 - v0));
console.log("delta_map                = " + (v2 - v1));
console.log("delta_stringify          = " + (v3 - v2));
console.log("delta_checksum_charcode  = " + (v4 - v3));
console.log("total_v4                 = " + v4);
