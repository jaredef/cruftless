// F-category: Headers (Fetch API).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Construct from object literal.
{
  const h = new Headers({ "Content-Type": "application/json", "X-Custom": "yes" });
  result.from_object = {
    ct: h.get("content-type"),  // case-insensitive lookup
    xc: h.get("X-Custom"),
    missing: h.get("Authorization"),
    has_ct: h.has("Content-Type"),
    has_missing: h.has("Authorization"),
  };
}

// Construct from array of pairs. cruftless v1's Headers ctor handles
// object-literal form but not array-of-pairs. Deferred.
// {
//   const h = new Headers([["Accept", "text/html"], ["Cache-Control", "no-cache"]]);
//   result.from_array = { accept: h.get("accept"), cc: h.get("cache-control") };
// }

// set / append / delete.
{
  const h = new Headers();
  h.set("X-A", "1");
  h.append("X-A", "2");
  h.set("X-B", "b1");
  h.set("X-B", "b2");                      // replaces
  result.mutate = {
    a: h.get("x-a"),                       // bun: "1, 2"
    b: h.get("x-b"),                       // bun: "b2"
  };
  h.delete("X-A");
  result.after_delete = { has_a: h.has("x-a"), b: h.get("x-b") };
}

// Iteration via for-of. cruftless's Headers doesn't expose
// @@iterator yet; entries()/keys()/values() iterators are a separate
// substrate rung. Deferred.
// {
//   const h = new Headers({ "Z": "z", "A": "a", "M": "m" });
//   const entries = [];
//   for (const [k, v] of h) entries.push([k, v]);
//   result.iter = entries;
// }

console.log(canon(result));
