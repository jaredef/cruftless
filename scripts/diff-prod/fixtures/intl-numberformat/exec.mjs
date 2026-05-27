// F-category: Intl.NumberFormat surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Intl.NumberFormat presence.
{
  result.nf_typeof = typeof Intl.NumberFormat;
  result.nf_is_function = typeof Intl.NumberFormat === "function";
}

// Constructor shape.
{
  try {
    const nf = new Intl.NumberFormat();
    result.nf_instance = typeof nf === "object";
    result.nf_has_format = typeof nf.format === "function";
    result.nf_has_resolvedOptions = typeof nf.resolvedOptions === "function";
    result.nf_has_formatToParts = typeof nf.formatToParts === "function";
  } catch (e) {
    result.nf_constructor = e.constructor.name;
  }
}

// resolvedOptions shape.
{
  try {
    const nf = new Intl.NumberFormat("en-US");
    const opts = nf.resolvedOptions();
    result.resolved_has_locale = typeof opts.locale === "string";
    result.resolved_has_numberingSystem = typeof opts.numberingSystem === "string";
    result.resolved_has_style = typeof opts.style === "string";
    result.resolved_has_minimumIntegerDigits = typeof opts.minimumIntegerDigits === "number";
    result.resolved_keys = Object.keys(opts).sort();
  } catch (e) {
    result.resolvedOptions = e.constructor.name;
  }
}

// format with en-US: integers.
{
  try {
    const nf = new Intl.NumberFormat("en-US");
    result.fmt_zero = nf.format(0);
    result.fmt_int = nf.format(1234);
    result.fmt_large = nf.format(1000000);
    result.fmt_neg = nf.format(-42);
  } catch (e) {
    result.fmt_integers = e.constructor.name;
  }
}

// format with en-US: decimals.
{
  try {
    const nf = new Intl.NumberFormat("en-US", { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    result.fmt_dec1 = nf.format(3.14159);
    result.fmt_dec2 = nf.format(0.5);
    result.fmt_dec3 = nf.format(1000.1);
  } catch (e) {
    result.fmt_decimals = e.constructor.name;
  }
}

// format with en-US: currency.
{
  try {
    const nf = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
    result.fmt_currency = nf.format(1234.56);
    result.fmt_currency_zero = nf.format(0);
    result.fmt_currency_neg = nf.format(-99.99);
  } catch (e) {
    result.fmt_currency = e.constructor.name;
  }
}

// supportedLocalesOf.
{
  try {
    result.supported_typeof = typeof Intl.NumberFormat.supportedLocalesOf === "function";
    const supported = Intl.NumberFormat.supportedLocalesOf(["en-US", "de-DE"]);
    result.supported_is_array = Array.isArray(supported);
    result.supported_len = supported.length;
  } catch (e) {
    result.supportedLocalesOf = e.constructor.name;
  }
}

// formatToParts presence and result shape.
{
  try {
    const nf = new Intl.NumberFormat("en-US");
    const parts = nf.formatToParts(1234.56);
    result.parts_is_array = Array.isArray(parts);
    result.parts_length_gt0 = parts.length > 0;
    result.parts_first_has_type = typeof parts[0].type === "string";
    result.parts_first_has_value = typeof parts[0].value === "string";
    result.parts_types = parts.map(p => p.type);
  } catch (e) {
    result.formatToParts = e.constructor.name;
  }
}

// format with percent style.
{
  try {
    const nf = new Intl.NumberFormat("en-US", { style: "percent" });
    result.fmt_percent = nf.format(0.75);
    result.fmt_percent_one = nf.format(1);
  } catch (e) {
    result.fmt_percent = e.constructor.name;
  }
}

console.log(canon(result));
