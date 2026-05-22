// F-category: Buffer.from + Buffer.toString roundtrips.

import { Buffer } from "node:buffer";

const inputs = [
  "hello world",
  "",
  "x",
  "héllo",
  "0123456789",
  "the quick brown fox jumps over the lazy dog",
];

const encodings = ["utf8", "hex", "base64"];

const result = {};
for (const enc of encodings) {
  result[enc] = {};
  for (const s of inputs) {
    const buf = Buffer.from(s, "utf8");
    const encoded = buf.toString(enc);
    const decoded = Buffer.from(encoded, enc).toString("utf8");
    result[enc][s] = { length: buf.length, encoded, roundtrip_eq: decoded === s };
  }
}

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {};
      for (const k of Object.keys(v).sort()) out[k] = v[k];
      return out;
    }
    return v;
  });
}

console.log(canon(result));
