// F-category: switch statement lowering.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic switch with break.
{
  function fn(x) {
    switch (x) {
      case 1: return "one";
      case 2: return "two";
      case 3: return "three";
      default: return "other";
    }
  }
  result.basic = { one: fn(1), two: fn(2), three: fn(3), other: fn(99) };
}

// Fallthrough without break.
{
  function fn(x) {
    const log = [];
    switch (x) {
      case 1: log.push("one");
      case 2: log.push("two");
      case 3: log.push("three"); break;
      case 4: log.push("four");
    }
    return log;
  }
  result.fallthrough = { from_1: fn(1), from_2: fn(2), from_3: fn(3), from_4: fn(4) };
}

// Default in middle.
{
  function fn(x) {
    switch (x) {
      case 1: return "one";
      default: return "default";
      case 2: return "two";
    }
  }
  result.default_middle = { one: fn(1), two: fn(2), other: fn(99) };
}

// Default at beginning with fallthrough.
{
  function fn(x) {
    const log = [];
    switch (x) {
      default: log.push("default");
      case 1: log.push("one"); break;
      case 2: log.push("two"); break;
    }
    return log;
  }
  result.default_first = { one: fn(1), two: fn(2), other: fn(99) };
}

// Switch uses strict comparison (===).
{
  function fn(x) {
    switch (x) {
      case 0: return "zero";
      case "0": return "string-zero";
      case false: return "false";
      case null: return "null";
      case undefined: return "undefined";
      default: return "default";
    }
  }
  result.strict_compare = {
    zero: fn(0),
    str_zero: fn("0"),
    false: fn(false),
    null: fn(null),
    undef: fn(undefined),
    empty: fn(""),
  };
}

// Expression in case labels.
{
  const A = 1, B = 2;
  function fn(x) {
    switch (x) {
      case A + B: return "three";
      case A * 10: return "ten";
      default: return "other";
    }
  }
  result.expr_cases = { three: fn(3), ten: fn(10), other: fn(1) };
}

// Empty cases (fallthrough to next).
{
  function fn(x) {
    switch (x) {
      case 1:
      case 2:
      case 3:
        return "one-two-three";
      case 4:
        return "four";
    }
    return "none";
  }
  result.empty_cases = { one: fn(1), two: fn(2), three: fn(3), four: fn(4), five: fn(5) };
}

// Block scoping in cases.
{
  function fn(x) {
    switch (x) {
      case 1: {
        const val = "one";
        return val;
      }
      case 2: {
        const val = "two";
        return val;
      }
    }
    return "none";
  }
  result.block_scope = { one: fn(1), two: fn(2) };
}

// Switch with no matching case and no default.
{
  function fn(x) {
    let reached = false;
    switch (x) {
      case 1: reached = true; break;
    }
    return reached;
  }
  result.no_match = fn(99);
}

// Switch over typeof.
{
  function fn(x) {
    switch (typeof x) {
      case "number": return "num";
      case "string": return "str";
      case "boolean": return "bool";
      case "object": return x === null ? "null" : "obj";
      case "undefined": return "undef";
      default: return "other";
    }
  }
  result.typeof_switch = { num: fn(1), str: fn("a"), bool: fn(true), null: fn(null), obj: fn({}), undef: fn(undefined) };
}

console.log(canon(result));
