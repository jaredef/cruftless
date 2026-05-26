// CRB-EXT-followup (2026-05-24): component A/B probe for the
// string_url_sweep fixture per standing rule 11 axis 1 (Doc 740 §II.4
// + Finding VII.1). Identifies the cost dominator empirically before
// substrate-pilot spawn.
//
// Variants (additive):
//   V0 — iterate corpus only (counter + checksum from i)
//   V1 — V0 + URL constructor + pathname/search property reads
//   V2 — V1 + per-header indexOf + slice + toLowerCase + trim
//   V3 — V2 + versionRe.exec(u.pathname)
//   V4 — V3 + idRe.exec(u.pathname)
//   V5 — V4 + parseInt(vmatch[1], 10) + parseInt(imatch[1], 10)  (= full)

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

const ITER = 5000;
const corpus = makeCorpus(ITER);
const versionRe = /v(\d+)(?:\.(\d+))?(?:\.(\d+))?/;
const idRe = /\/(\d+)(?:[/?]|$)/;

function variant(name, fn) {
  for (let i = 0; i < 50; i++) fn(i);  // warmup
  const t0 = Date.now();
  let sink = 0;
  for (let i = 0; i < ITER; i++) sink = (sink + fn(i)) | 0;
  const dt = Date.now() - t0;
  console.log("VARIANT " + name + " ms=" + dt + " sink=" + sink);
  return dt;
}

const v0 = variant("V0_iter_only", (i) => {
  const entry = corpus[i % corpus.length];
  return entry.url.length;
});

const v1 = variant("V1_url_constructor", (i) => {
  const entry = corpus[i % corpus.length];
  const u = new URL(entry.url);
  return u.pathname.length + u.search.length;
});

const v2 = variant("V2_header_loop", (i) => {
  const entry = corpus[i % corpus.length];
  const u = new URL(entry.url);
  let s = u.pathname.length + u.search.length;
  for (const h of entry.headers) {
    const idx = h.indexOf(":");
    if (idx > 0) {
      const name = h.slice(0, idx).toLowerCase().trim();
      const val = h.slice(idx + 1).trim();
      s = (s + name.length + val.length) | 0;
    }
  }
  return s;
});

const v3 = variant("V3_version_regex", (i) => {
  const entry = corpus[i % corpus.length];
  const u = new URL(entry.url);
  let s = u.pathname.length + u.search.length;
  for (const h of entry.headers) {
    const idx = h.indexOf(":");
    if (idx > 0) {
      const name = h.slice(0, idx).toLowerCase().trim();
      const val = h.slice(idx + 1).trim();
      s = (s + name.length + val.length) | 0;
    }
  }
  const vmatch = versionRe.exec(u.pathname);
  if (vmatch) s = (s + 1) | 0;
  return s;
});

const v4 = variant("V4_id_regex", (i) => {
  const entry = corpus[i % corpus.length];
  const u = new URL(entry.url);
  let s = u.pathname.length + u.search.length;
  for (const h of entry.headers) {
    const idx = h.indexOf(":");
    if (idx > 0) {
      const name = h.slice(0, idx).toLowerCase().trim();
      const val = h.slice(idx + 1).trim();
      s = (s + name.length + val.length) | 0;
    }
  }
  const vmatch = versionRe.exec(u.pathname);
  if (vmatch) s = (s + 1) | 0;
  const imatch = idRe.exec(u.pathname);
  if (imatch) s = (s + 1) | 0;
  return s;
});

const v5 = variant("V5_full_with_parseInt", (i) => {
  const entry = corpus[i % corpus.length];
  const u = new URL(entry.url);
  let s = u.pathname.length + u.search.length;
  for (const h of entry.headers) {
    const idx = h.indexOf(":");
    if (idx > 0) {
      const name = h.slice(0, idx).toLowerCase().trim();
      const val = h.slice(idx + 1).trim();
      s = (s + name.length + val.length) | 0;
    }
  }
  const vmatch = versionRe.exec(u.pathname);
  if (vmatch) { s = (s + (parseInt(vmatch[1], 10) || 0)) | 0; }
  const imatch = idRe.exec(u.pathname);
  if (imatch) { s = (s + (parseInt(imatch[1], 10) || 0)) | 0; }
  return s;
});

console.log("---");
console.log("delta_iter_only             = " + v0);
console.log("delta_url_constructor       = " + (v1 - v0));
console.log("delta_header_loop           = " + (v2 - v1));
console.log("delta_version_regex         = " + (v3 - v2));
console.log("delta_id_regex              = " + (v4 - v3));
console.log("delta_parseInt              = " + (v5 - v4));
console.log("total_v5                    = " + v5);
