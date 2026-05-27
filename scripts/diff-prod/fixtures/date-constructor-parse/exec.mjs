// F-category: Date constructor and parse.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

{
  const d = new Date(0);
  result.epoch = {
    getTime: d.getTime(),
    toISOString: d.toISOString(),
  };
}

{
  const d = new Date(2026, 0, 15, 12, 30, 45, 500);
  result.multi_arg = {
    fullYear: d.getFullYear(),
    month: d.getMonth(),
    date: d.getDate(),
    hours: d.getHours(),
    minutes: d.getMinutes(),
    seconds: d.getSeconds(),
    milliseconds: d.getMilliseconds(),
  };
}

{
  const d = new Date(2026, 0);
  result.two_arg = {
    fullYear: d.getFullYear(),
    month: d.getMonth(),
    date: d.getDate(),
  };
}

{
  const d = new Date("2026-01-15T00:00:00.000Z");
  result.iso_string = {
    getTime: d.getTime(),
    toISOString: d.toISOString(),
    fullYear: d.getUTCFullYear(),
    month: d.getUTCMonth(),
    date: d.getUTCDate(),
  };
}

{
  const d = new Date("2026-06-15");
  result.date_only_string = {
    fullYear: d.getUTCFullYear(),
    month: d.getUTCMonth(),
    date: d.getUTCDate(),
  };
}

{
  const t1 = Date.parse("2026-01-15T00:00:00.000Z");
  const t2 = Date.parse("1970-01-01T00:00:00.000Z");
  result.parse = {
    iso_2026: t1,
    epoch: t2,
    type: typeof t1,
  };
}

{
  const p = Date.parse("not-a-date");
  result.parse_invalid = {
    value: p,
    is_nan: Number.isNaN(p),
  };
}

{
  const n = Date.now();
  result.now = {
    type: typeof n,
    is_number: typeof n === "number",
    is_positive: n > 0,
  };
}

{
  const u = Date.UTC(2026, 0, 15, 12, 30, 45, 500);
  result.utc = {
    value: u,
    type: typeof u,
    matches_iso: new Date(u).toISOString(),
  };
}

{
  const u0 = Date.UTC(2026, 0);
  result.utc_minimal = {
    value: u0,
    iso: new Date(u0).toISOString(),
  };
}

{
  const d = new Date(Date.UTC(2026, 5, 15, 8, 30, 0, 0));
  result.getters_utc = {
    getUTCFullYear: d.getUTCFullYear(),
    getUTCMonth: d.getUTCMonth(),
    getUTCDate: d.getUTCDate(),
    getUTCHours: d.getUTCHours(),
    getUTCMinutes: d.getUTCMinutes(),
    getUTCSeconds: d.getUTCSeconds(),
    getUTCMilliseconds: d.getUTCMilliseconds(),
    getUTCDay: d.getUTCDay(),
  };
}

{
  const d = new Date(Date.UTC(2026, 0, 15));
  result.toISOString = d.toISOString();
}

{
  const d = new Date("invalid");
  result.invalid_date = {
    toString: d.toString(),
    is_nan_getTime: Number.isNaN(d.getTime()),
    is_invalid_date: d.toString() === "Invalid Date",
  };
}

{
  const d = new Date(NaN);
  result.nan_date = {
    toString: d.toString(),
    is_nan: Number.isNaN(d.getTime()),
  };
}

{
  try {
    const d = new Date("invalid");
    const iso = d.toISOString();
    result.invalid_toISO = { threw: false, value: iso };
  } catch (e) {
    result.invalid_toISO = { threw: true, name: e.constructor.name };
  }
}

{
  const d = new Date(Date.UTC(2026, 0, 15));
  result.valueOf = {
    value: d.valueOf(),
    equals_getTime: d.valueOf() === d.getTime(),
    type: typeof d.valueOf(),
  };
}

{
  const d1 = new Date(1000);
  const d2 = new Date(2000);
  result.comparison = {
    less: d1 < d2,
    greater: d1 > d2,
    subtract: d2 - d1,
  };
}

{
  result.constructor_shapes = {
    no_args_type: typeof new Date().getTime(),
    single_number: new Date(86400000).toISOString(),
    negative_ms: new Date(-86400000).toISOString(),
  };
}

{
  const d = new Date(Date.UTC(2026, 11, 31, 23, 59, 59, 999));
  result.end_of_year = {
    iso: d.toISOString(),
    year: d.getUTCFullYear(),
    month: d.getUTCMonth(),
    date: d.getUTCDate(),
  };
}

console.log(canon(result));
