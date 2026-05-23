// CRB-EXT 5 fixture: crypto_sha256_batch
//
// Hashes a batch of small payloads via WebCrypto's subtle.digest.
// Tests the crypto subsystem: SHA-256 over N byte arrays, with the
// runtime's TextEncoder converting source strings to bytes.
//
// Tests: TextEncoder, crypto.subtle.digest, ArrayBuffer/Uint8Array,
// Promise.all (parallel digest). Self-contained; no deps.
//
// Output: count + hex-digest of the first hash. Stdout-bytes-equality
// across runtimes is the Pred-crb.1 gate.

const N = 1000;          // 1000 payloads
const PAYLOAD_LEN = 200; // each ~200 chars

function makePayload(i) {
  // Deterministic payload generation; ~200 chars.
  const base = "the_quick_brown_fox_jumps_over_the_lazy_dog_record_" + i + "_";
  let out = base;
  while (out.length < PAYLOAD_LEN) out += base;
  return out.slice(0, PAYLOAD_LEN);
}

function bytesToHex(bytes) {
  let s = "";
  for (let i = 0; i < bytes.length; i++) {
    const h = bytes[i].toString(16);
    s += h.length === 1 ? "0" + h : h;
  }
  return s;
}

async function main() {
  const encoder = new TextEncoder();
  const digests = [];

  // Batch-hash N payloads. Sequential (await each) for portability;
  // Promise.all would also work but timing variance increases.
  for (let i = 0; i < N; i++) {
    const data = encoder.encode(makePayload(i));
    const hash = await crypto.subtle.digest("SHA-256", data);
    digests.push(new Uint8Array(hash));
  }

  // Aggregate: first-digest hex + total byte count + simple checksum.
  const firstHex = bytesToHex(digests[0]);
  let total = 0;
  let checksum = 0;
  for (const d of digests) {
    total += d.length;
    for (let j = 0; j < d.length; j++) checksum = (checksum + d[j]) | 0;
  }

  console.log("crypto_sha256_batch n=" + N + " total=" + total + " first=" + firstHex + " checksum=" + checksum);
}

main().catch((e) => { console.error(e); process.exit(1); });
