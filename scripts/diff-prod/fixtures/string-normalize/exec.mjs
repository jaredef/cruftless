// F-category: String.prototype.normalize.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Presence.
{
  result.present = typeof String.prototype.normalize === "function";
}

// NFC: composed form (e-acute as single code point).
{
  const decomposed = "é";
  const composed = "é";
  result.nfc = {
    before_equal: decomposed === composed,
    after_equal: decomposed.normalize("NFC") === composed.normalize("NFC"),
    decomposed_len: decomposed.length,
    composed_len: composed.length,
    nfc_len: decomposed.normalize("NFC").length,
  };
}

// NFD: decomposed form.
{
  const composed = "é";
  const nfd = composed.normalize("NFD");
  result.nfd = {
    nfd_len: nfd.length,
    first_cp: nfd.codePointAt(0),
    second_cp: nfd.codePointAt(1),
    roundtrip: nfd.normalize("NFC") === composed,
  };
}

// NFKC: compatibility composition (e.g., fi ligature).
{
  const fi = "ﬁ";
  const nfkc = fi.normalize("NFKC");
  result.nfkc = {
    original: fi,
    normalized: nfkc,
    original_len: fi.length,
    nfkc_len: nfkc.length,
  };
}

// NFKD: compatibility decomposition.
{
  const fi = "ﬁ";
  const nfkd = fi.normalize("NFKD");
  result.nfkd = {
    normalized: nfkd,
    nfkd_len: nfkd.length,
  };
}

// Default form is NFC.
{
  const s = "é";
  result.default_is_nfc = {
    equal: s.normalize() === s.normalize("NFC"),
  };
}

// ASCII is already normalized in all forms.
{
  const s = "hello";
  result.ascii = {
    nfc: s.normalize("NFC") === s,
    nfd: s.normalize("NFD") === s,
    nfkc: s.normalize("NFKC") === s,
    nfkd: s.normalize("NFKD") === s,
  };
}

// Empty string.
{
  result.empty = {
    nfc: "".normalize("NFC"),
    nfd: "".normalize("NFD"),
  };
}

// Multiple combining marks.
{
  const s = "à́";
  const nfc = s.normalize("NFC");
  result.multi_combining = {
    original_len: s.length,
    nfc_len: nfc.length,
  };
}

// Invalid form argument.
{
  let threw = false;
  let err_name = null;
  try { "x".normalize("INVALID"); } catch (e) { threw = true; err_name = e.constructor.name; }
  result.invalid_form = { threw, err_name };
}

console.log(canon(result));
