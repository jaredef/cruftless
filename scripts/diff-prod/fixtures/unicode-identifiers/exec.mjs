// F-category: Unicode identifiers (ECMA-262 §12.7).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic non-ASCII identifiers.
{
  const café = "coffee";
  const π = Math.PI;
  const über = "above";
  result.non_ascii = { café, π: typeof π === "number", über };
}

// CJK identifiers.
{
  const 変数 = "variable";
  const 名前 = "name";
  result.cjk = { 変数, 名前 };
}

// Greek letters.
{
  const α = 1;
  const β = 2;
  const Σ = α + β;
  result.greek = { α, β, Σ };
}

// Underscore and $ are valid starts.
{
  const _private = 1;
  const $special = 2;
  const __dunder = 3;
  result.underscore_dollar = { _private, $special, __dunder };
}

// Unicode escape in identifier: a = 'a'.
{
  const a = 42;
  result.unicode_escape = { a };
}

// Unicode escape braced form: \u{61} = 'a'.
{
  const \u{62} = 99;
  result.unicode_brace = { b };
}

// Identifier with combining marks (ID_Continue).
{
  const é = "e-acute";
  result.combining = typeof é === "string" ? é : "not-found";
}

// Property names can be any identifier.
{
  const obj = { ñ: 1, Ω: 2, _: 3 };
  result.prop_names = { keys: Object.keys(obj).sort() };
}

// typeof on unresolvable identifier returns "undefined" (no throw).
{
  result.typeof_unresolvable = typeof 不存在的变量 === "undefined";
}

// Reserved words with unicode escapes are still identifiers (not keywords).
// e.g., if is "if" but as an escaped identifier, it's valid in some contexts.
{
  const obj = {};
  obj["if"] = "keyword-as-prop";
  result.keyword_as_prop = obj["if"];
}

// Computed property with unicode.
{
  const key = "日本語";
  const obj = { [key]: "nihongo" };
  result.computed_unicode = obj["日本語"];
}

// Destructuring with unicode identifiers.
{
  const { α: alpha, β: beta } = { α: 10, β: 20 };
  result.destructure_unicode = { alpha, beta };
}

console.log(canon(result));
