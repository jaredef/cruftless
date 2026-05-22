// F-category: RegExp coverage.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Construction equivalence.
const r1 = /abc/i;
const r2 = new RegExp("abc", "i");
result.literal_eq_ctor = {
  source_eq: r1.source === r2.source,
  flags_eq: r1.flags === r2.flags,
  ignoreCase: r1.ignoreCase,
  global: r1.global,
  multiline: r1.multiline,
};

// test.
result.test = {
  abc_in_aabcc: /abc/.test("aabcc"),
  abc_in_zzz:   /abc/.test("zzz"),
  case_i:       /abc/i.test("ABC"),
  case_no_i:    /abc/.test("ABC"),
};

// exec.
{
  const m = /(a)(b+)(c)/.exec("zaabbbcz");
  result.exec = m ? { match: m[0], g1: m[1], g2: m[2], g3: m[3], index: m.index } : null;
}

// String.prototype.match.
result.match_first = "the quick brown fox".match(/(\w+)/);  // first match details
result.match_all_g = "a1 b2 c3".match(/[a-z]\d/g);

// String.prototype.matchAll.
result.match_all = [..."a1 b2 c3".matchAll(/([a-z])(\d)/g)].map(m => ({ match: m[0], g1: m[1], g2: m[2], index: m.index }));

// replace / replaceAll.
result.replace = "foo bar foo".replace("foo", "X");
result.replace_g = "foo bar foo".replace(/foo/g, "X");
result.replaceAll = "foo bar foo".replaceAll("foo", "X");
result.replace_fn = "abc".replace(/(.)/g, (m, g1) => g1.toUpperCase());
result.replace_backref = "John Doe".replace(/(\w+) (\w+)/, "$2 $1");

// split.
result.split_str = "a,b,c".split(",");
result.split_re = "a1b22c333d".split(/\d+/);
result.split_limit = "a,b,c,d".split(",", 2);

// search.
result.search = "hello world".search(/world/);
result.search_no_match = "hello world".search(/xyz/);

// flags + state.
const gr = /x/g;
gr.lastIndex = 2;
result.lastIndex_before_exec = gr.lastIndex;
gr.exec("axxx");
result.lastIndex_after_exec = gr.lastIndex;

// special chars.
result.escape = "a.b*c".replace(/[.*]/g, "\\$&");

console.log(canon(result));
