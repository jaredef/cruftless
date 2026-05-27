// F-category: Buffer.concat and Buffer factory surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Buffer.concat returns a Buffer, not [object Object].
{
  const a = Buffer.from("hello ");
  const b = Buffer.from("world");
  const c = Buffer.concat([a, b]);
  result.concat = {
    toString: c.toString(),
    is_buffer: Buffer.isBuffer(c),
    length: c.length,
    type: typeof c,
    string_rep: String(c),
    not_object_object: String(c) !== "[object Object]",
  };
}

// Buffer.concat with empty list.
{
  const c = Buffer.concat([]);
  result.concat_empty = {
    toString: c.toString(),
    length: c.length,
    is_buffer: Buffer.isBuffer(c),
  };
}

// Buffer.concat with totalLength.
{
  const a = Buffer.from("abc");
  const b = Buffer.from("def");
  const c = Buffer.concat([a, b], 4);
  result.concat_total_length = {
    toString: c.toString(),
    length: c.length,
  };
}

// Buffer.alloc.
{
  const b = Buffer.alloc(8);
  result.alloc = {
    length: b.length,
    is_buffer: Buffer.isBuffer(b),
    all_zero: b.every(x => x === 0),
  };
}

// Buffer.alloc with fill.
{
  const b = Buffer.alloc(4, 0x41);
  result.alloc_fill = {
    toString: b.toString(),
    length: b.length,
  };
}

// Buffer.from string roundtrip.
{
  const s = "hello, buffer!";
  const b = Buffer.from(s, "utf8");
  result.from_string = {
    roundtrip: b.toString("utf8"),
    match: b.toString("utf8") === s,
    length: b.length,
  };
}

// Buffer.from array.
{
  const b = Buffer.from([72, 101, 108, 108, 111]);
  result.from_array = {
    toString: b.toString(),
    is_buffer: Buffer.isBuffer(b),
  };
}

// Buffer.isBuffer.
{
  result.is_buffer = {
    buffer: Buffer.isBuffer(Buffer.from("x")),
    uint8: Buffer.isBuffer(new Uint8Array(1)),
    string: Buffer.isBuffer("x"),
    null: Buffer.isBuffer(null),
  };
}

// Buffer.byteLength.
{
  result.byte_length = {
    ascii: Buffer.byteLength("hello"),
    emoji: Buffer.byteLength("\u{1F600}"),
    empty: Buffer.byteLength(""),
  };
}

// Buffer.compare.
{
  const a = Buffer.from("abc");
  const b = Buffer.from("abc");
  const c = Buffer.from("abd");
  result.compare = {
    equal: Buffer.compare(a, b),
    less: Buffer.compare(a, c) < 0,
    greater: Buffer.compare(c, a) > 0,
  };
}

console.log(canon(result));
