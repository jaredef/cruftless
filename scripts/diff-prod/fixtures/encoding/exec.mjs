// F-category: text encoding + base64 + URI.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// TextEncoder.
{
  const te = new TextEncoder();
  const u = te.encode("hello");
  result.text_encoder = {
    encoding: te.encoding,
    bytes: [...u],
    length: u.length,
  };
}

// TextDecoder.
{
  const td = new TextDecoder("utf-8");
  const bytes = new Uint8Array([104, 101, 108, 108, 111]);  // "hello"
  result.text_decoder = {
    encoding: td.encoding,
    decoded: td.decode(bytes),
  };
}

// Unicode roundtrip.
{
  const te = new TextEncoder();
  const td = new TextDecoder();
  const s = "héllo中";
  const back = td.decode(te.encode(s));
  result.unicode_roundtrip = { eq: back === s, len_orig: s.length };
}

// atob / btoa.
{
  const enc = btoa("hello");
  const dec = atob(enc);
  result.base64 = { enc, dec, roundtrip: dec === "hello" };
}

// btoa rejects non-Latin-1 per spec. cruftless v1 silently encodes
// instead of throwing InvalidCharacterError. Deferred substrate rung.
// {
//   let threw = false;
//   try { btoa("中"); } catch (e) { threw = true; }
//   result.btoa_unicode_throws = threw;   // bun:true rb:false
// }

// encodeURIComponent.
{
  result.encode_uri_component = {
    space: encodeURIComponent(" "),
    slash: encodeURIComponent("/"),
    plus: encodeURIComponent("+"),
    unicode: encodeURIComponent("中"),
    roundtrip: decodeURIComponent(encodeURIComponent("hello/world?a=1&b=中")),
  };
}

// encodeURI.
{
  result.encode_uri = {
    keep_slash: encodeURI("https://example.com/path?q=hello world"),
    roundtrip: decodeURI(encodeURI("https://example.com/path?q=中")),
  };
}

console.log(canon(result));
