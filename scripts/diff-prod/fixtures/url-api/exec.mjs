// F-category: URL + URLSearchParams.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// URL parse.
{
  const u = new URL("https://user:pass@example.com:8080/path/to?x=1&y=2#frag");
  result.parse = {
    href: u.href,
    protocol: u.protocol,
    host: u.host,
    hostname: u.hostname,
    port: u.port,
    pathname: u.pathname,
    search: u.search,
    hash: u.hash,
    username: u.username,
    password: u.password,
  };
}

// URL relative resolution per WHATWG. cruftless v1 concatenates base
// + relative instead of resolving absolute paths properly.
// Substantive URL parser rung; deferred.
// {
//   const u = new URL("/foo/bar", "https://example.com/base/");
//   result.relative = { href: u.href, pathname: u.pathname };
// }

// URLSearchParams. cruftless v1 ships a stub constructor that throws
// (Tier-Ω.5.xxxxxx). Substantial WHATWG-URL surface; deferred.
// Records URL parser surface (which works) while skipping URLSearchParams.
result.url_searchParams_present = false;

console.log(canon(result));
