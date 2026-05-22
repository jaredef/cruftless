// F-category: String.prototype.* coverage probe.

const corpus = [
  "abcdefghij",
  "the quick brown fox",
  "&amp;&lt;&gt;&quot;",       // multi-occurrence needle
  "a,b,,c,",                     // empty splits
  "  trim me  ",
  "héllo",
  "0123456789012345",
];

const result = {};
for (const s of corpus) {
  const ops = {};
  ops.length = s.length;
  ops.indexOf_a = s.indexOf("a");
  ops.indexOf_z = s.indexOf("z");
  ops.indexOf_e_from0 = s.indexOf("e", 0);
  ops.indexOf_e_from5 = s.indexOf("e", 5);
  ops.lastIndexOf_o = s.lastIndexOf("o");
  ops.slice_2_5 = s.slice(2, 5);
  ops.slice_neg3 = s.slice(-3);
  ops.split_comma = s.split(",");
  ops.toUpperCase = s.toUpperCase();
  ops.repeat3 = s.length <= 8 ? s.repeat(3) : null;
  ops.padStart = s.padStart(20, "*");
  ops.padEnd = s.padEnd(20, "*");
  ops.replace_first = s.replace("o", "0");
  ops.replace_all = s.replaceAll("o", "0");
  ops.startsWith_the = s.startsWith("the");
  ops.endsWith_5 = s.endsWith("5");
  ops.trim = s.trim();
  ops.charCodeAt_0 = s.charCodeAt(0);
  result[s] = ops;
}

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {};
      for (const k of Object.keys(v).sort()) out[k] = v[k];
      return out;
    }
    return v;
  });
}

console.log(canon(result));
