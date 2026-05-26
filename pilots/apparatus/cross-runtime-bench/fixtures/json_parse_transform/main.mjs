// CRB-EXT 2 fixture: json_parse_transform
//
// Generates a realistic JSON payload in-memory (config-file shape:
// nested object with array of records), parses it via JSON.parse,
// applies filter+map transform, JSON.stringify the result.
//
// Tests: JSON.parse, JSON.stringify, Array.filter, Array.map,
// Object iteration, string concatenation. No spread (avoids any
// latent shape-bypass surprises). Self-contained; no external deps.
//
// Output: a single line with the final transformed JSON length + a
// checksum. Stdout-bytes-equality across runtimes is the Pred-crb.1
// gate.

// Generate ~50KB JSON payload deterministically (seed=42).
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

const ITER = 500;  // 500 parse+transform+stringify cycles
const N_RECS = 100;

const payload = makePayload(N_RECS);
const text = JSON.stringify(payload);

let checksum = 0;
let lastLen = 0;
for (let iter = 0; iter < ITER; iter++) {
  const parsed = JSON.parse(text);
  const filtered = parsed.records.filter((r) => r.active && r.score > 100);
  const mapped = filtered.map((r) => ({
    id: r.id,
    label: r.name + "_" + r.meta.version,
    bucket: Math.floor(r.score / 100),
  }));
  const out = JSON.stringify(mapped);
  lastLen = out.length;
  // Cheap byte-checksum so the iteration is observable.
  for (let i = 0; i < out.length; i++) checksum = (checksum + out.charCodeAt(i)) | 0;
}

console.log("json_parse_transform iter=" + ITER + " n_recs=" + N_RECS + " out_len=" + lastLen + " checksum=" + checksum);
