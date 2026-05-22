// F-category: template literals + tagged templates.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic interpolation.
const name = "world";
const n = 42;
result.basic = `hello, ${name}! n=${n}`;

// Multi-line.
result.multiline = `line1
line2
line3`;

// Expression in interpolation.
result.expr = `sum is ${1 + 2 + 3}`;

// Method call in interpolation.
result.method = `up: ${"abc".toUpperCase()}`;

// Ternary in interpolation.
const x = 5;
result.ternary = `x is ${x > 0 ? "positive" : "non-positive"}`;

// Nested template.
result.nested = `outer ${`inner ${name}`} done`;

// Escapes.
result.escapes = {
  quote: `with \`backtick\``,
  dollar: `not \${interpolated}`,
  newline: `a\nb`,
  tab: `a\tb`,
};

// Tagged template — collects strings and expressions. cruftless v1
// doesn't populate strings.raw on tagged templates; the field is
// expected by String.raw and pricier tag implementations. Substantive
// substrate rung; recorded as v2 boundary.
function tag(strings, ...values) {
  return {
    cooked_strings: strings.slice(),
    values: values,
    // raw_field omitted — strings.raw is undefined in cruftless v1
  };
}
const a = 1, b = 2;
result.tagged = tag`prefix ${a} middle ${b} suffix`;

// Tagged template with no expressions.
result.tag_no_expr = tag`just a string`;

// Tagged template with adjacent expressions.
result.tag_adjacent = tag`${a}${b}`;

// HTML-escape-shaped tag.
function htmlEscape(strings, ...values) {
  const escMap = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" };
  const esc = s => String(s).replace(/[&<>"]/g, c => escMap[c]);
  let out = strings[0];
  for (let i = 0; i < values.length; i++) {
    out += esc(values[i]) + strings[i + 1];
  }
  return out;
}
result.html_escape = htmlEscape`<p>${"<script>alert(1)</script>"}</p>`;

console.log(canon(result));
