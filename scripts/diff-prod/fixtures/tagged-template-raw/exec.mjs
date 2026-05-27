// F-category: tagged template strings.raw at the AST-to-bytecode boundary.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// strings.raw must be populated on the template object.
function capture(strings, ...values) {
  return {
    cooked: [...strings],
    raw: strings.raw ? [...strings.raw] : null,
    has_raw: strings.raw != null,
    raw_is_array: Array.isArray(strings.raw),
    values,
  };
}

// Basic: raw should match cooked when no escape sequences.
{
  const c = capture`hello world`;
  result.basic = {
    cooked: c.cooked,
    raw: c.raw,
    has_raw: c.has_raw,
    raw_is_array: c.raw_is_array,
  };
}

// Escape sequences: raw preserves the literal source text.
{
  const c = capture`line1\nline2\ttab`;
  result.escapes = {
    cooked: c.cooked,
    raw: c.raw,
    cooked_has_newline: c.cooked[0].includes("\n"),
    raw_has_backslash_n: c.raw ? c.raw[0].includes("\\n") : null,
  };
}

// With interpolations.
{
  const x = 42;
  const c = capture`before\n${x}\tafter`;
  result.interp = {
    cooked: c.cooked,
    raw: c.raw,
    values: c.values,
  };
}

// String.raw (built-in tag that uses strings.raw).
{
  result.string_raw = {
    present: typeof String.raw === "function",
  };
  if (typeof String.raw === "function") {
    result.string_raw.basic = String.raw`hello\nworld`;
    result.string_raw.tab = String.raw`col1\tcol2`;
    result.string_raw.interp = String.raw`a\n${"b"}\nc`;
    result.string_raw.backslash = String.raw`C:\Users\test`;
  }
}

// Template object is frozen.
{
  let templateObj = null;
  function grab(strings) { templateObj = strings; }
  grab`test`;
  result.frozen = {
    is_frozen: Object.isFrozen(templateObj),
    raw_is_frozen: templateObj.raw ? Object.isFrozen(templateObj.raw) : null,
  };
}

// Template object identity: same call site yields same object.
{
  let first = null, second = null;
  function id(strings) { return strings; }
  first = id`same`;
  second = id`same`;
  result.identity = {
    same_object: first === second,
  };
}

// Multiple segments with raw.
{
  const a = 1, b = 2, c = 3;
  const r = capture`\x41${a}B${b}\n${c}\t`;
  result.multi_segment = {
    cooked: r.cooked,
    raw: r.raw,
    values: r.values,
  };
}

console.log(canon(result));
