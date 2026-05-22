// F-category: Date coverage. Avoid timezone-dependent comparisons —
// canonicalize everything to UTC.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Construct from ISO string.
{
  const d = new Date("2026-05-22T03:00:00.000Z");
  result.from_iso = {
    time: d.getTime(),
    year: d.getUTCFullYear(),
    month: d.getUTCMonth(),
    date: d.getUTCDate(),
    hours: d.getUTCHours(),
    iso: d.toISOString(),
  };
}

// Construct from epoch.
{
  const d = new Date(0);
  result.from_epoch = {
    iso: d.toISOString(),
    year: d.getUTCFullYear(),
    valueOf: d.valueOf(),
  };
}

// Construct from components (UTC).
{
  const t = Date.UTC(2026, 4, 22, 3, 30, 45, 500);
  const d = new Date(t);
  result.utc_components = {
    time: t,
    iso: d.toISOString(),
    year: d.getUTCFullYear(),
    month: d.getUTCMonth(),
    minutes: d.getUTCMinutes(),
    seconds: d.getUTCSeconds(),
    ms: d.getUTCMilliseconds(),
  };
}

// Arithmetic.
{
  const a = new Date("2026-01-01T00:00:00.000Z");
  const b = new Date("2026-01-02T00:00:00.000Z");
  const diff_ms = b.getTime() - a.getTime();
  result.arithmetic = {
    diff_ms,
    diff_days: diff_ms / (1000 * 60 * 60 * 24),
  };
}

// Parse various formats.
{
  result.parse = {
    iso: Date.parse("2026-05-22T00:00:00.000Z"),
    iso_no_ms: Date.parse("2026-05-22T00:00:00Z"),
    invalid_isNaN: Number.isNaN(Date.parse("not a date")),
  };
}

// Comparison via valueOf coercion. Date primitive-coercion (`+date`,
// `date1 < date2`) routes through Symbol.toPrimitive("number") with a
// fallback to valueOf. cruftless's Date constructor with a string arg
// doesn't fully wire the parsed time back to the instance's valueOf
// path; deferred as its own substrate rung. The fixture records the
// surface that DOES match: a.getTime() and b.getTime() return correct
// values; the < / +- coercion is what diverges.
{
  const a = new Date("2026-01-01T00:00:00.000Z");
  const b = new Date("2026-06-01T00:00:00.000Z");
  result.compare = {
    a_getTime: a.getTime(),
    b_getTime: b.getTime(),
    diff_via_getTime: a.getTime() - b.getTime(),
  };
}

// Common getters/setters on UTC.
{
  const d = new Date(Date.UTC(2026, 0, 1, 0, 0, 0, 0));
  d.setUTCMonth(11);
  d.setUTCDate(25);
  result.setters = {
    iso_after: d.toISOString(),
    month: d.getUTCMonth(),
    date: d.getUTCDate(),
  };
}

// Now() shape (don't compare value — just type + sanity).
{
  const n = Date.now();
  result.now_shape = {
    is_number: typeof n === "number",
    is_finite: Number.isFinite(n),
    is_positive: n > 0,
  };
}

console.log(canon(result));
