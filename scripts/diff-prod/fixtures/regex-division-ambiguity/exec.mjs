// F-category: regex vs division disambiguation (lexer state machine).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Regex after return keyword.
{
  function fn() { return /abc/.test("abc"); }
  result.after_return = fn();
}

// Regex after assignment.
{
  const re = /hello/;
  result.after_assign = re.test("hello world");
}

// Regex after opening paren.
{
  const arr = [1, 2, 3].filter(x => /^[12]$/.test(String(x)));
  result.after_paren = arr;
}

// Regex after comma.
{
  const a = 1, b = /test/;
  result.after_comma = b instanceof RegExp;
}

// Regex after semicolon (statement boundary).
{
  let x = 1;
  const re = /foo/;
  result.after_semicolon = re instanceof RegExp;
}

// Division after number.
{
  const x = 10 / 2;
  result.division_after_number = x;
}

// Division after identifier.
{
  const a = 10;
  const b = a / 2;
  result.division_after_ident = b;
}

// Division after closing paren.
{
  const x = (4 + 6) / 2;
  result.division_after_paren = x;
}

// Division after closing bracket.
{
  const arr = [10];
  const x = arr[0] / 2;
  result.division_after_bracket = x;
}

// Regex in conditional.
{
  const x = true ? /yes/ : /no/;
  result.in_ternary = x.source;
}

// Regex after typeof.
{
  result.after_typeof = typeof /abc/ === "object";
}

// Regex after void.
{
  result.after_void = void /abc/ === undefined;
}

// Regex after logical operators.
{
  const a = false || /fallback/;
  const b = true && /truthy/;
  result.after_logical = {
    or: a instanceof RegExp,
    and: b instanceof RegExp,
  };
}

// Regex with flags.
{
  const re = /hello/gi;
  result.with_flags = {
    source: re.source,
    global: re.global,
    ignoreCase: re.ignoreCase,
    flags: re.flags,
  };
}

// Regex in array literal.
{
  const arr = [/a/, /b/, /c/];
  result.in_array = arr.map(r => r.source);
}

// Regex in object literal.
{
  const obj = { pattern: /test/i };
  result.in_object = {
    source: obj.pattern.source,
    flags: obj.pattern.flags,
  };
}

// Division compound assignment.
{
  let x = 100;
  x /= 4;
  result.div_assign = x;
}

console.log(canon(result));
