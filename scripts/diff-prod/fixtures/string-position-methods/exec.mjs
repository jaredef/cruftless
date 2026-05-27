// F-category: string position-argument methods.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};
const s = "hello world";

// startsWith with position argument.
{
  result.starts_with = {
    basic: s.startsWith("hello"),
    from_6: s.startsWith("world", 6),
    from_0: s.startsWith("world", 0),
    negative: s.startsWith("hello", -1),
    beyond: s.startsWith("hello", 100),
    empty: s.startsWith("", 5),
  };
}

// endsWith with endPosition argument.
{
  result.ends_with = {
    basic: s.endsWith("world"),
    at_5: s.endsWith("hello", 5),
    at_11: s.endsWith("world", 11),
    at_3: s.endsWith("hel", 3),
    at_0: s.endsWith("", 0),
  };
}

// includes with position argument.
{
  result.includes = {
    basic: s.includes("world"),
    from_0: s.includes("hello", 0),
    from_6: s.includes("hello", 6),
    from_6_world: s.includes("world", 6),
    from_7: s.includes("world", 7),
    negative: s.includes("hello", -100),
  };
}

// indexOf with fromIndex.
{
  result.index_of = {
    basic: s.indexOf("o"),
    from_5: s.indexOf("o", 5),
    from_8: s.indexOf("o", 8),
    not_found: s.indexOf("z"),
    from_negative: s.indexOf("h", -5),
  };
}

// lastIndexOf with fromIndex.
{
  result.last_index_of = {
    basic: s.lastIndexOf("o"),
    from_5: s.lastIndexOf("o", 5),
    from_3: s.lastIndexOf("o", 3),
    not_found: s.lastIndexOf("z"),
  };
}

// padStart / padEnd.
{
  result.pad_start = {
    basic: "42".padStart(5, "0"),
    space: "hi".padStart(5),
    no_pad: "hello".padStart(3, "x"),
    multi_char: "1".padStart(6, "ab"),
  };
  result.pad_end = {
    basic: "42".padEnd(5, "0"),
    space: "hi".padEnd(5),
    no_pad: "hello".padEnd(3, "x"),
    multi_char: "1".padEnd(6, "ab"),
  };
}

// repeat.
{
  result.repeat = {
    three: "abc".repeat(3),
    zero: "abc".repeat(0),
    one: "abc".repeat(1),
    empty: "".repeat(100),
  };
}

// at (with negative).
{
  result.at = {
    first: s.at(0),
    last: s.at(-1),
    neg2: s.at(-2),
    mid: s.at(6),
    oor: s.at(100),
  };
}

// slice with various arguments.
{
  result.slice = {
    basic: s.slice(0, 5),
    from_6: s.slice(6),
    negative: s.slice(-5),
    neg_neg: s.slice(-5, -1),
    reversed_empty: s.slice(5, 2),
  };
}

// substring edge cases.
{
  result.substring = {
    basic: s.substring(0, 5),
    swapped: s.substring(5, 0),
    negative_clamped: s.substring(-3, 5),
    nan: s.substring(NaN, 5),
  };
}

console.log(canon(result));
