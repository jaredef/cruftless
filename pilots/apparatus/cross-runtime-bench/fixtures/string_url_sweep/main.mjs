// CRB-EXT 4 fixture: string_url_sweep
//
// Generates a corpus of fake HTTP request lines + URLs; runs URL
// parsing + header normalization + regex sweep over each entry.
// Representative of per-request work in Node HTTP servers.
//
// Tests: URL constructor, string.split/.replace/.toLowerCase,
// regex matching (literal and dynamic). Self-contained; no deps.
//
// Output: per-request count + checksum derived from extracted
// fields. Stdout-bytes-equality across runtimes is the Pred-crb.1
// gate.

const PATHS = [
  "/api/v1/users/42",
  "/api/v2/orders/9876?fields=id,total,items",
  "/static/assets/main.css?v=2.1.0",
  "/blog/posts/2024-01-15-hello-world",
  "/search?q=lightweight+runtime&limit=20&offset=0",
  "/users/profile/jared?include=preferences",
  "/api/v1/products/categories/electronics/laptops",
  "/healthz",
];
const QUERY_TERMS = ["debug", "format", "lang", "tz", "version", "trace"];
const HEADER_NAMES = ["Content-Type", "Accept", "User-Agent", "Authorization",
                     "X-Request-ID", "Cache-Control", "Accept-Encoding"];

function makeCorpus(n) {
  const out = [];
  for (let i = 0; i < n; i++) {
    const path = PATHS[i % PATHS.length];
    const qsep = path.indexOf("?") >= 0 ? "&" : "?";
    const extra = QUERY_TERMS[i % QUERY_TERMS.length] + "=" + (i * 31 % 1000);
    const url = "https://api.example.com:443" + path + qsep + extra;
    const headers = HEADER_NAMES.map((h, j) =>
      h + ": " + (j === 0 ? "application/json" : "value_" + ((i * 17 + j) % 100))
    );
    out.push({ url, headers });
  }
  return out;
}

const ITER = 5000;  // 5000 simulated request lines
const corpus = makeCorpus(ITER);
const versionRe = /v(\d+)(?:\.(\d+))?(?:\.(\d+))?/;
const idRe = /\/(\d+)(?:[/?]|$)/;

let urlsParsed = 0;
let headerCount = 0;
let versionsFound = 0;
let idsFound = 0;
let checksum = 0;

for (const entry of corpus) {
  // URL parse + property access.
  const u = new URL(entry.url);
  urlsParsed++;
  // Note: cruft does not implement URLSearchParams.size; use u.search
  // length as a portable proxy (counts the query-string bytes incl. "?").
  checksum = (checksum + u.pathname.length + u.search.length) | 0;

  // Header normalization: lower-case + split.
  for (const h of entry.headers) {
    const i = h.indexOf(":");
    if (i > 0) {
      const name = h.slice(0, i).toLowerCase().trim();
      const val = h.slice(i + 1).trim();
      checksum = (checksum + name.length + val.length) | 0;
      headerCount++;
    }
  }

  // Regex sweep on URL pathname.
  const vmatch = versionRe.exec(u.pathname);
  if (vmatch) {
    versionsFound++;
    checksum = (checksum + (parseInt(vmatch[1], 10) || 0)) | 0;
  }
  const imatch = idRe.exec(u.pathname);
  if (imatch) {
    idsFound++;
    checksum = (checksum + (parseInt(imatch[1], 10) || 0)) | 0;
  }
}

console.log("string_url_sweep urls=" + urlsParsed + " headers=" + headerCount + " versions=" + versionsFound + " ids=" + idsFound + " checksum=" + checksum);
