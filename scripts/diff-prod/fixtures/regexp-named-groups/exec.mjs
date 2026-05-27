// F-category: RegExp named capture groups (AST-to-bytecode boundary).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic named groups: match.groups must be populated.
{
  const re = /(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})/;
  const m = "2026-05-22".match(re);
  result.basic = {
    has_groups: m.groups != null,
    groups_type: typeof m.groups,
    year: m.groups ? m.groups.year : null,
    month: m.groups ? m.groups.month : null,
    day: m.groups ? m.groups.day : null,
    positional_still_works: { y: m[1], m: m[2], d: m[3] },
  };
}

// exec() also populates groups.
{
  const re = /(?<first>\w+)\s+(?<last>\w+)/;
  const m = re.exec("John Doe");
  result.exec_groups = {
    has_groups: m.groups != null,
    first: m.groups ? m.groups.first : null,
    last: m.groups ? m.groups.last : null,
  };
}

// No match returns null (groups not applicable).
{
  const re = /(?<x>\d+)/;
  const m = "no digits".match(re);
  result.no_match = { result: m };
}

// Groups object has null prototype (no inherited keys).
{
  const re = /(?<a>x)/;
  const m = "x".match(re);
  result.groups_proto = {
    has_groups: m.groups != null,
    proto_null: m.groups ? Object.getPrototypeOf(m.groups) === null : null,
    keys: m.groups ? Object.keys(m.groups) : null,
  };
}

// Unmatched named group: value is undefined.
{
  const re = /(?<a>\d+)|(?<b>\w+)/;
  const m = "hello".match(re);
  result.unmatched_group = {
    has_groups: m.groups != null,
    a: m.groups ? m.groups.a : null,
    b: m.groups ? m.groups.b : null,
    a_is_undefined: m.groups ? m.groups.a === undefined : null,
  };
}

// Named backreference \k<name>.
{
  const re = /(?<word>\w+)\s+\k<word>/;
  result.backreference = {
    match: re.test("hello hello"),
    no_match: re.test("hello world"),
  };
}

// String.prototype.replace with named groups $<name>.
{
  const re = /(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})/;
  result.replace_named = "2026-05-22".replace(re, "$<day>/$<month>/$<year>");
}

// Replace with function receiving groups.
{
  const re = /(?<first>\w+)-(?<second>\w+)/;
  const out = "hello-world".replace(re, (...args) => {
    const groups = args[args.length - 1];
    return groups && typeof groups === "object"
      ? `${groups.second}_${groups.first}`
      : "no-groups";
  });
  result.replace_fn_groups = out;
}

// matchAll with named groups.
{
  const re = /(?<letter>[a-z])(?<digit>\d)/g;
  const matches = [...("a1 b2 c3".matchAll(re))];
  result.match_all = matches.map(m => ({
    full: m[0],
    has_groups: m.groups != null,
    letter: m.groups ? m.groups.letter : null,
    digit: m.groups ? m.groups.digit : null,
  }));
}

// Multiple named groups in a single pattern.
{
  const re = /(?<proto>\w+):\/\/(?<host>[^/:]+)(?::(?<port>\d+))?(?<path>\/[^\s]*)?/;
  const m = "https://example.com:8080/api/data".match(re);
  result.complex = {
    has_groups: m.groups != null,
    proto: m.groups ? m.groups.proto : null,
    host: m.groups ? m.groups.host : null,
    port: m.groups ? m.groups.port : null,
    path: m.groups ? m.groups.path : null,
  };
}

console.log(canon(result));
