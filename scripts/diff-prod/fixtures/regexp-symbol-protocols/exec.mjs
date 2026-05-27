// F-category: RegExp symbol protocol surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Symbol.match on RegExp.
{
  const re = /o/g;
  result.symbol_match_exists = typeof re[Symbol.match] === "function";
  try {
    const m = "foo bar boo".match(re);
    result.match_result = m;
  } catch (e) {
    result.match_result = e.constructor.name;
  }
}

// Symbol.replace on RegExp.
{
  const re = /o/g;
  result.symbol_replace_exists = typeof re[Symbol.replace] === "function";
  try {
    const r = "foo bar boo".replace(re, "X");
    result.replace_result = r;
  } catch (e) {
    result.replace_result = e.constructor.name;
  }
}

// Symbol.split on RegExp.
{
  const re = /\s+/;
  result.symbol_split_exists = typeof re[Symbol.split] === "function";
  try {
    const s = "foo  bar   baz".split(re);
    result.split_result = s;
  } catch (e) {
    result.split_result = e.constructor.name;
  }
}

// Symbol.search on RegExp.
{
  const re = /bar/;
  result.symbol_search_exists = typeof re[Symbol.search] === "function";
  try {
    const idx = "foo bar baz".search(re);
    result.search_result = idx;
  } catch (e) {
    result.search_result = e.constructor.name;
  }
}

// Custom object with Symbol.match.
{
  const matcher = {
    [Symbol.match](str) {
      return str.includes("hello") ? ["hello"] : null;
    }
  };
  try {
    result.custom_match_hit = "say hello world".match(matcher);
    result.custom_match_miss = "say goodbye".match(matcher);
  } catch (e) {
    result.custom_match = e.constructor.name;
  }
}

// Custom object with Symbol.replace.
{
  const replacer = {
    [Symbol.replace](str, replacement) {
      return str.split("x").join(replacement);
    }
  };
  try {
    result.custom_replace = "axbxc".replace(replacer, "Y");
  } catch (e) {
    result.custom_replace = e.constructor.name;
  }
}

// Custom object with Symbol.split.
{
  const splitter = {
    [Symbol.split](str) {
      return str.split("").filter(c => c !== " ");
    }
  };
  try {
    result.custom_split = "a b c".split(splitter);
  } catch (e) {
    result.custom_split = e.constructor.name;
  }
}

// Custom object with Symbol.search.
{
  const searcher = {
    [Symbol.search](str) {
      return str.indexOf("Z");
    }
  };
  try {
    result.custom_search_found = "abcZdef".search(searcher);
    result.custom_search_miss = "abcdef".search(searcher);
  } catch (e) {
    result.custom_search = e.constructor.name;
  }
}

// Symbol.matchAll.
{
  result.matchAll_symbol_exists = typeof Symbol.matchAll === "symbol";
  const re = /o/g;
  result.matchAll_on_regexp = typeof re[Symbol.matchAll] === "function";
  try {
    const iter = "foo boo".matchAll(/o/g);
    const matches = [];
    for (const m of iter) {
      matches.push({ val: m[0], idx: m.index });
    }
    result.matchAll_result = matches;
  } catch (e) {
    result.matchAll_result = e.constructor.name;
  }
}

// String.match delegates to Symbol.match (non-regexp).
{
  result.match_uses_protocol = typeof Symbol.match === "symbol";
  const obj = { [Symbol.match]: () => "custom" };
  try {
    result.match_delegate = "anything".match(obj);
  } catch (e) {
    result.match_delegate = e.constructor.name;
  }
}

console.log(canon(result));
