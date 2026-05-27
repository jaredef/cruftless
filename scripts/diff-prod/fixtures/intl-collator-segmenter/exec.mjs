// F-category: Intl.Collator, Intl.Segmenter, Intl.PluralRules surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Intl.Collator presence.
{
  result.collator_typeof = typeof Intl.Collator;
  result.collator_is_function = typeof Intl.Collator === "function";
}

// Collator compare: basic string ordering.
{
  try {
    const coll = new Intl.Collator("en");
    result.compare_ab = Math.sign(coll.compare("a", "b"));
    result.compare_ba = Math.sign(coll.compare("b", "a"));
    result.compare_same = coll.compare("abc", "abc");
    result.compare_case = Math.sign(coll.compare("a", "A"));
    const sorted = ["banana", "apple", "cherry"].sort(coll.compare);
    result.collator_sorted = sorted;
  } catch (e) {
    result.collator_compare = e.constructor.name;
  }
}

// Collator resolvedOptions shape.
{
  try {
    const coll = new Intl.Collator("en");
    const opts = coll.resolvedOptions();
    result.coll_opts_has_locale = typeof opts.locale === "string";
    result.coll_opts_has_usage = typeof opts.usage === "string";
    result.coll_opts_has_sensitivity = typeof opts.sensitivity === "string";
    result.coll_opts_has_collation = typeof opts.collation === "string";
    result.coll_opts_keys = Object.keys(opts).sort();
  } catch (e) {
    result.collator_resolvedOptions = e.constructor.name;
  }
}

// Collator sensitivity options.
{
  try {
    const base = new Intl.Collator("en", { sensitivity: "base" });
    result.sensitivity_base_aA = base.compare("a", "A") === 0;
    result.sensitivity_base_ae = base.compare("a", "ä") === 0;
    const variant = new Intl.Collator("en", { sensitivity: "variant" });
    result.sensitivity_variant_aA = variant.compare("a", "A") !== 0;
  } catch (e) {
    result.collator_sensitivity = e.constructor.name;
  }
}

// Intl.Segmenter presence.
{
  result.segmenter_typeof = typeof Intl.Segmenter;
  result.segmenter_available = typeof Intl.Segmenter === "function";
}

// Segmenter with "en", granularity "word".
{
  if (typeof Intl.Segmenter === "function") {
    try {
      const seg = new Intl.Segmenter("en", { granularity: "word" });
      const segments = Array.from(seg.segment("Hello, world!"));
      result.seg_count = segments.length;
      result.seg_first_has_segment = typeof segments[0].segment === "string";
      result.seg_first_has_index = typeof segments[0].index === "number";
      result.seg_first_has_isWordLike = typeof segments[0].isWordLike === "boolean";
      result.seg_word_segments = segments.filter(s => s.isWordLike).map(s => s.segment);
    } catch (e) {
      result.segmenter_word = e.constructor.name;
    }
  } else {
    result.segmenter_word = "not_available";
  }
}

// Segmenter with grapheme granularity.
{
  if (typeof Intl.Segmenter === "function") {
    try {
      const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
      const segments = Array.from(seg.segment("ab"));
      result.seg_grapheme_count = segments.length;
      result.seg_grapheme_vals = segments.map(s => s.segment);
    } catch (e) {
      result.segmenter_grapheme = e.constructor.name;
    }
  } else {
    result.segmenter_grapheme = "not_available";
  }
}

// Segmenter resolvedOptions.
{
  if (typeof Intl.Segmenter === "function") {
    try {
      const seg = new Intl.Segmenter("en", { granularity: "word" });
      const opts = seg.resolvedOptions();
      result.seg_opts_has_locale = typeof opts.locale === "string";
      result.seg_opts_has_granularity = typeof opts.granularity === "string";
      result.seg_opts_granularity = opts.granularity;
    } catch (e) {
      result.segmenter_opts = e.constructor.name;
    }
  } else {
    result.segmenter_opts = "not_available";
  }
}

// Intl.PluralRules presence.
{
  result.plural_typeof = typeof Intl.PluralRules;
  result.plural_is_function = typeof Intl.PluralRules === "function";
}

// PluralRules select with simple numbers.
{
  try {
    const pr = new Intl.PluralRules("en");
    result.plural_0 = pr.select(0);
    result.plural_1 = pr.select(1);
    result.plural_2 = pr.select(2);
    result.plural_100 = pr.select(100);
    result.plural_has_resolvedOptions = typeof pr.resolvedOptions === "function";
    const opts = pr.resolvedOptions();
    result.plural_opts_has_locale = typeof opts.locale === "string";
    result.plural_opts_has_type = typeof opts.type === "string";
  } catch (e) {
    result.plural_select = e.constructor.name;
  }
}

console.log(canon(result));
