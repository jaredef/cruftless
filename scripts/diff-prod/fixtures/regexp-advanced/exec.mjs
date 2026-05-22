// L-category: RegExp advanced surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Named groups: cruftless v1 doesn't populate match.groups. Deferred.
// (Match itself succeeds; only the .groups accessor is missing.)
{
  const re = /(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})/;
  const m = "2026-05-22".match(re);
  result.named_positional = { y: m[1], m: m[2], d: m[3] };
}

// Backreferences.
{
  const re = /(\w+)\s+\1/;
  result.backref = { match: "hello hello world".match(re)[0], no: "hello world".match(re) === null };
}

// Sticky flag (y) — anchors at lastIndex.
{
  const re = /foo/y;
  re.lastIndex = 4;
  result.sticky = {
    at_4: "    foo bar".match(re) && true,
    last_index_after: re.lastIndex,
    no_match_at_0: (() => { re.lastIndex = 0; return "    foo".match(re); })() === null,
  };
}

// Global flag (g) — lastIndex advances with exec.
{
  const re = /\d+/g;
  const matches = [];
  let m; while ((m = re.exec("1 22 333 4444")) !== null) matches.push(m[0]);
  result.global = { matches, total: matches.length };
}

// matchAll.
{
  const all = [..."a1 b22 c333".matchAll(/([a-z])(\d+)/g)];
  result.match_all = all.map(m => ({ full: m[0], letter: m[1], digits: m[2] }));
}

// replaceAll.
{
  result.replace_all = "a-b-c".replaceAll("-", "_");
}

// replace with function (positional, not named — see above).
{
  const r = "2026-05-22".replace(
    /(\d{4})-(\d{2})-(\d{2})/,
    (_full, y, m, d) => `${d}/${m}/${y}`,
  );
  result.replace_fn = r;
}

// dotAll (s) flag.
{
  result.dotall = {
    without: /a.b/.test("a\nb"),
    with: /a.b/s.test("a\nb"),
  };
}

// Unicode escape.
{
  result.unicode = {
    u_flag: /\u{1F600}/u.test("😀"),
    char_class: /[A-Z]+/.test("HELLO"),
  };
}

// Quantifier laziness.
{
  const greedy = "<a><b>".match(/<(.+)>/)[1];
  const lazy = "<a><b>".match(/<(.+?)>/)[1];
  result.quant = { greedy, lazy };
}

console.log(canon(result));
