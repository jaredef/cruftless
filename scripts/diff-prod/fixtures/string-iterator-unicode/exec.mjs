// F-category: String iterator and unicode code-point surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// String[Symbol.iterator] existence.
{
  result.string_iterator_exists = typeof String.prototype[Symbol.iterator] === "function";
}

// Iterating over a string with emoji: yields code points.
{
  try {
    const str = "\u{1F600}A";
    const iter = str[Symbol.iterator]();
    const items = [];
    let next = iter.next();
    while (!next.done) {
      items.push(next.value);
      next = iter.next();
    }
    result.emoji_iter_count = items.length;
    result.emoji_iter_first = items[0];
    result.emoji_iter_second = items[1];
  } catch (e) {
    result.emoji_iter = e.constructor.name;
  }
}

// for-of over string with surrogate pairs.
{
  try {
    const str = "A\u{1F4A9}B\u{1F60D}C";
    const chars = [];
    for (const ch of str) {
      chars.push(ch);
    }
    result.forof_count = chars.length;
    result.forof_chars = chars;
  } catch (e) {
    result.forof = e.constructor.name;
  }
}

// codePointAt vs charCodeAt for astral plane.
{
  try {
    const str = "\u{1F600}";
    result.codePointAt_0 = str.codePointAt(0);
    result.charCodeAt_0 = str.charCodeAt(0);
    result.charCodeAt_1 = str.charCodeAt(1);
    result.codepoint_gt_charcode = str.codePointAt(0) > str.charCodeAt(0);
    result.string_length = str.length;
  } catch (e) {
    result.codePointAt = e.constructor.name;
  }
}

// codePointAt for BMP character.
{
  try {
    result.bmp_codePointAt = "A".codePointAt(0);
    result.bmp_charCodeAt = "A".charCodeAt(0);
    result.bmp_equal = "A".codePointAt(0) === "A".charCodeAt(0);
  } catch (e) {
    result.bmp_code = e.constructor.name;
  }
}

// String.fromCodePoint / String.fromCharCode.
{
  try {
    result.fromCodePoint_emoji = String.fromCodePoint(0x1F600);
    result.fromCodePoint_A = String.fromCodePoint(65);
    result.fromCodePoint_multi = String.fromCodePoint(72, 101, 108, 108, 111);
  } catch (e) {
    result.fromCodePoint = e.constructor.name;
  }
  try {
    result.fromCharCode_A = String.fromCharCode(65);
    result.fromCharCode_multi = String.fromCharCode(72, 101, 108, 108, 111);
  } catch (e) {
    result.fromCharCode = e.constructor.name;
  }
}

// Array.from(string) vs string.split('').
{
  try {
    const str = "\u{1F600}AB";
    const fromArr = Array.from(str);
    const splitArr = str.split("");
    result.array_from_count = fromArr.length;
    result.split_empty_count = splitArr.length;
    result.array_from_uses_codepoints = fromArr.length < splitArr.length;
    result.array_from_values = fromArr;
    result.split_empty_values = splitArr;
  } catch (e) {
    result.array_from_vs_split = e.constructor.name;
  }
}

// Spread also uses iterator (code points).
{
  try {
    const str = "\u{1F4A9}X";
    const spread = [...str];
    result.spread_count = spread.length;
    result.spread_values = spread;
  } catch (e) {
    result.spread = e.constructor.name;
  }
}

// String.prototype.normalize interaction with iteration.
{
  try {
    const combined = "é";
    const decomposed = "é";
    result.norm_combined_len = combined.length;
    result.norm_decomposed_len = decomposed.length;
    result.norm_equal_before = combined === decomposed;
    result.norm_equal_nfc = combined.normalize("NFC") === decomposed.normalize("NFC");
    result.norm_equal_nfd = combined.normalize("NFD") === decomposed.normalize("NFD");
    const nfc_iter = [...decomposed.normalize("NFC")];
    const nfd_iter = [...combined.normalize("NFD")];
    result.nfc_iter_count = nfc_iter.length;
    result.nfd_iter_count = nfd_iter.length;
  } catch (e) {
    result.normalize = e.constructor.name;
  }
}

// Iterator protocol compliance.
{
  try {
    const iter = "abc"[Symbol.iterator]();
    result.iter_has_next = typeof iter.next === "function";
    const first = iter.next();
    result.iter_first = { value: first.value, done: first.done };
    iter.next();
    iter.next();
    const fourth = iter.next();
    result.iter_exhausted = { value: fourth.value, done: fourth.done };
  } catch (e) {
    result.iter_protocol = e.constructor.name;
  }
}

console.log(canon(result));
