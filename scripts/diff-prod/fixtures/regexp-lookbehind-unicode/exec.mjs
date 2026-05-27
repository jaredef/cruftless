// F-category: RegExp lookbehind, unicode, dotAll, hasIndices.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Positive lookbehind.
{
  try {
    const re = /(?<=\$)\d+/;
    const m = "$100 and 200".match(re);
    result.pos_lookbehind = m ? { val: m[0], idx: m.index } : null;
  } catch (e) {
    result.pos_lookbehind = e.constructor.name;
  }
}

// Negative lookbehind.
{
  try {
    const re = /(?<!\$)\d+/;
    const m = "$100 and 200".match(re);
    result.neg_lookbehind = m ? { val: m[0], idx: m.index } : null;
  } catch (e) {
    result.neg_lookbehind = e.constructor.name;
  }
}

// Positive lookbehind with global flag.
{
  try {
    const re = /(?<=@)\w+/g;
    const matches = "hello @foo and @bar".match(re);
    result.pos_lookbehind_global = matches;
  } catch (e) {
    result.pos_lookbehind_global = e.constructor.name;
  }
}

// Unicode flag basic.
{
  try {
    const re = /\u{1F600}/u;
    result.unicode_emoji = re.test("\u{1F600}");
    result.unicode_flag = re.unicode;
  } catch (e) {
    result.unicode_emoji = e.constructor.name;
  }
}

// Unicode flag with surrogate pair string.
{
  try {
    const str = "😀";
    const re = /./gu;
    const matches = str.match(re);
    result.unicode_surrogate_count = matches ? matches.length : -1;
    const reNoU = /./g;
    const matchesNoU = str.match(reNoU);
    result.no_unicode_surrogate_count = matchesNoU ? matchesNoU.length : -1;
  } catch (e) {
    result.unicode_surrogates = e.constructor.name;
  }
}

// Unicode property escapes.
{
  try {
    const reLetter = /\p{Letter}+/u;
    result.prop_letter = reLetter.test("abc");
    result.prop_letter_digit = reLetter.test("123");
  } catch (e) {
    result.prop_letter = e.constructor.name;
  }
  try {
    const reNumber = /\p{Number}+/u;
    result.prop_number = reNumber.test("456");
    result.prop_number_alpha = reNumber.test("abc");
  } catch (e) {
    result.prop_number = e.constructor.name;
  }
}

// Unicode property escape matching specific chars.
{
  try {
    const re = /\p{Emoji_Presentation}/u;
    result.prop_emoji = re.test("\u{1F600}");
  } catch (e) {
    result.prop_emoji = e.constructor.name;
  }
}

// dotAll flag (s).
{
  try {
    const reNoDot = /foo.bar/;
    const reDotAll = /foo.bar/s;
    result.dotall_no_s = reNoDot.test("foo\nbar");
    result.dotall_with_s = reDotAll.test("foo\nbar");
    result.dotall_flag = reDotAll.dotAll;
  } catch (e) {
    result.dotall = e.constructor.name;
  }
}

// dotAll + unicode combined.
{
  try {
    const re = /./su;
    const m = "\u{1F600}".match(re);
    result.dotall_unicode = m ? m[0] === "\u{1F600}" : false;
  } catch (e) {
    result.dotall_unicode = e.constructor.name;
  }
}

// hasIndices flag (d).
{
  try {
    const re = /(?<word>\w+)/d;
    const m = re.exec("hello world");
    result.hasIndices_flag = re.hasIndices;
    if (m && m.indices) {
      result.indices_0 = m.indices[0];
      result.indices_group = m.indices.groups ? { word: m.indices.groups.word } : null;
    } else {
      result.indices_0 = null;
      result.indices_group = null;
    }
  } catch (e) {
    result.hasIndices = e.constructor.name;
  }
}

// hasIndices with multiple groups.
{
  try {
    const re = /(?<first>\w+)\s+(?<second>\w+)/d;
    const m = re.exec("foo bar");
    if (m && m.indices) {
      result.indices_multi = {
        full: m.indices[0],
        first: m.indices[1],
        second: m.indices[2],
        groups_first: m.indices.groups ? m.indices.groups.first : null,
        groups_second: m.indices.groups ? m.indices.groups.second : null,
      };
    } else {
      result.indices_multi = null;
    }
  } catch (e) {
    result.indices_multi = e.constructor.name;
  }
}

// Lookbehind with unicode flag.
{
  try {
    const re = /(?<=\p{Letter})\d+/u;
    const m = "a42".match(re);
    result.lookbehind_unicode = m ? m[0] : null;
  } catch (e) {
    result.lookbehind_unicode = e.constructor.name;
  }
}

console.log(canon(result));
