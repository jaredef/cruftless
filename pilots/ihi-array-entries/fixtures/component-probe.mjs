// IAE-EXT 1 component probe.
//
// Runs small additive variants over the json_parse_transform body shape so
// the first Array-IHI entry set is chosen from measured current cost.

const ITER = Number(globalThis.process?.env?.IAE_ITER || 500);
const N_RECS = Number(globalThis.process?.env?.IAE_N_RECS || 100);
const RUNS = Number(globalThis.process?.env?.IAE_RUNS || 5);

function nowMs() {
  return Number(process.hrtime.bigint()) / 1_000_000;
}

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

const payload = makePayload(N_RECS);
const text = JSON.stringify(payload);

function checksumString(s) {
  let checksum = 0;
  for (let i = 0; i < s.length; i++) checksum = (checksum + s.charCodeAt(i)) | 0;
  return checksum;
}

function baseline() {
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
    checksum = (checksum + checksumString(out)) | 0;
  }
  return checksum ^ lastLen;
}

function noFilterMethod() {
  let checksum = 0;
  let lastLen = 0;
  for (let iter = 0; iter < ITER; iter++) {
    const parsed = JSON.parse(text);
    const filtered = [];
    for (let i = 0; i < parsed.records.length; i++) {
      const r = parsed.records[i];
      if (r.active && r.score > 100) filtered.push(r);
    }
    const mapped = filtered.map((r) => ({
      id: r.id,
      label: r.name + "_" + r.meta.version,
      bucket: Math.floor(r.score / 100),
    }));
    const out = JSON.stringify(mapped);
    lastLen = out.length;
    checksum = (checksum + checksumString(out)) | 0;
  }
  return checksum ^ lastLen;
}

function noMapMethod() {
  let checksum = 0;
  let lastLen = 0;
  for (let iter = 0; iter < ITER; iter++) {
    const parsed = JSON.parse(text);
    const filtered = parsed.records.filter((r) => r.active && r.score > 100);
    const mapped = [];
    for (let i = 0; i < filtered.length; i++) {
      const r = filtered[i];
      mapped.push({
        id: r.id,
        label: r.name + "_" + r.meta.version,
        bucket: Math.floor(r.score / 100),
      });
    }
    const out = JSON.stringify(mapped);
    lastLen = out.length;
    checksum = (checksum + checksumString(out)) | 0;
  }
  return checksum ^ lastLen;
}

function noFilterNoMap() {
  let checksum = 0;
  let lastLen = 0;
  for (let iter = 0; iter < ITER; iter++) {
    const parsed = JSON.parse(text);
    const mapped = [];
    for (let i = 0; i < parsed.records.length; i++) {
      const r = parsed.records[i];
      if (r.active && r.score > 100) {
        mapped.push({
          id: r.id,
          label: r.name + "_" + r.meta.version,
          bucket: Math.floor(r.score / 100),
        });
      }
    }
    const out = JSON.stringify(mapped);
    lastLen = out.length;
    checksum = (checksum + checksumString(out)) | 0;
  }
  return checksum ^ lastLen;
}

function timeVariant(name, fn) {
  const samples = [];
  let acc = 0;
  for (let run = 0; run < RUNS; run++) {
    const start = nowMs();
    acc ^= fn();
    const end = nowMs();
    samples.push(end - start);
  }
  samples.sort((a, b) => a - b);
  const mid = Math.floor(samples.length / 2);
  const median = samples.length % 2
    ? samples[mid]
    : (samples[mid - 1] + samples[mid]) / 2;
  console.log(`${name}\tmedian_ms=${median.toFixed(3)}\tacc=${acc}`);
}

timeVariant("baseline_filter_map", baseline);
timeVariant("manual_filter_map_method", noFilterMethod);
timeVariant("filter_method_manual_map", noMapMethod);
timeVariant("manual_filter_manual_map", noFilterNoMap);
