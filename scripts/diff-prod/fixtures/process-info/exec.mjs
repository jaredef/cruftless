// F-category: process object surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// ppid: must be a positive integer (parent process id).
{
  result.ppid = {
    type: typeof process.ppid,
    is_number: typeof process.ppid === "number",
    positive: typeof process.ppid === "number" ? process.ppid > 0 : false,
    integer: typeof process.ppid === "number" ? Number.isInteger(process.ppid) : false,
  };
}

// pid.
{
  result.pid = {
    type: typeof process.pid,
    is_number: typeof process.pid === "number",
    positive: typeof process.pid === "number" ? process.pid > 0 : false,
    integer: typeof process.pid === "number" ? Number.isInteger(process.pid) : false,
    differs_from_ppid: process.pid !== process.ppid,
  };
}

// arch + platform: typeof and non-empty.
{
  result.arch = {
    type: typeof process.arch,
    non_empty: typeof process.arch === "string" && process.arch.length > 0,
  };
  result.platform = {
    type: typeof process.platform,
    non_empty: typeof process.platform === "string" && process.platform.length > 0,
  };
}

// version.
{
  result.version = {
    type: typeof process.version,
    starts_with_v: typeof process.version === "string" && process.version.startsWith("v"),
  };
}

// env: accessible object.
{
  result.env = {
    type: typeof process.env,
    is_object: typeof process.env === "object" && process.env !== null,
    has_PATH: typeof process.env.PATH === "string" || typeof process.env.Path === "string",
  };
}

// argv: array shape.
{
  result.argv = {
    is_array: Array.isArray(process.argv),
    length_gte_1: Array.isArray(process.argv) && process.argv.length >= 1,
    first_is_string: Array.isArray(process.argv) && typeof process.argv[0] === "string",
  };
}

// cwd.
{
  result.cwd = {
    is_function: typeof process.cwd === "function",
    returns_string: typeof process.cwd === "function" ? typeof process.cwd() === "string" : false,
    non_empty: typeof process.cwd === "function" ? process.cwd().length > 0 : false,
  };
}

// hrtime (legacy form).
{
  const has = typeof process.hrtime === "function";
  result.hrtime = { present: has };
  if (has) {
    const t = process.hrtime();
    result.hrtime.is_array = Array.isArray(t);
    result.hrtime.length = Array.isArray(t) ? t.length : null;
    result.hrtime.first_number = Array.isArray(t) && t.length >= 1 ? typeof t[0] === "number" : false;
  }
}

// hrtime.bigint.
{
  const has = typeof process.hrtime === "function" && typeof process.hrtime.bigint === "function";
  result.hrtime_bigint = { present: has };
  if (has) {
    const t = process.hrtime.bigint();
    result.hrtime_bigint.type = typeof t;
    result.hrtime_bigint.positive = typeof t === "bigint" ? t > 0n : false;
  }
}

// uptime.
{
  const has = typeof process.uptime === "function";
  result.uptime = { present: has };
  if (has) {
    const u = process.uptime();
    result.uptime.type = typeof u;
    result.uptime.non_negative = typeof u === "number" ? u >= 0 : false;
  }
}

console.log(canon(result));
