// F-category: node:crypto basic surface. Avoid timing-dependent or
// random-value-dependent assertions; check only stable shapes.

import * as crypto from "node:crypto";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// createHash + update + digest (deterministic).
{
  const h = crypto.createHash("sha256");
  h.update("hello world");
  const hex = h.digest("hex");
  result.sha256_hex = hex;   // deterministic: b94d27...
}

// MD5 (deterministic).
{
  const h = crypto.createHash("md5");
  h.update("hello");
  result.md5_hex = h.digest("hex");  // 5d41402abc4b2a76b9719d911017c592
}

// SHA-1 (deterministic).
{
  const h = crypto.createHash("sha1");
  h.update("");
  result.sha1_empty = h.digest("hex");  // da39a3ee5e6b4b0d3255bfef95601890afd80709
}

// Multiple updates concat.
{
  const h = crypto.createHash("sha256");
  h.update("hello");
  h.update(" ");
  h.update("world");
  result.sha256_multi = h.digest("hex");
}

// randomBytes shape (don't assert value — random).
{
  const b = crypto.randomBytes(16);
  result.random_bytes_shape = {
    length: b.length,
    is_array_like: typeof b[0] === "number" && typeof b[15] === "number",
    bytes_in_range: b[0] >= 0 && b[0] < 256,
  };
}

// randomUUID format (deterministic shape, random value).
{
  const u = crypto.randomUUID();
  result.uuid_shape = {
    length: u.length,                                   // 36
    dashes: u[8] === "-" && u[13] === "-" && u[18] === "-" && u[23] === "-",
    version_4: u[14] === "4",                            // v4 UUID
    variant_8_9_a_b: ["8","9","a","b"].includes(u[19]),
  };
}

console.log(canon(result));
