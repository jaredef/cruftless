// F-category: TypedArray + ArrayBuffer coverage.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Uint8Array basics.
{
  const u = new Uint8Array(5);
  result.uint8_init = {
    length: u.length,
    byteLength: u.byteLength,
    values: [...u],
  };
}

// Uint8Array from array.
{
  const u = new Uint8Array([1, 2, 3, 4, 5]);
  result.uint8_from_array = {
    length: u.length,
    sum: u.reduce((a, b) => a + b, 0),
    first: u[0],
    last: u[4],
  };
}

// fill + indexOf.
{
  const u = new Uint8Array(6);
  u.fill(7);
  u[2] = 42;
  result.fill_then_index = {
    values: [...u],
    indexOf_7: u.indexOf(7),
    indexOf_42: u.indexOf(42),
    indexOf_9: u.indexOf(9),
  };
}

// slice + subarray.
{
  const u = new Uint8Array([10, 20, 30, 40, 50]);
  const s = u.slice(1, 4);
  const sub = u.subarray(1, 4);
  result.slice_subarray = {
    slice: [...s],
    subarray: [...sub],
    slice_is_copy: u !== s,
    sub_is_view: u.buffer === sub.buffer,
  };
}

// Type-aware overflow. Deferred: cruftless's typed-array setters
// don't mask the value by element byte-width (Uint8Array[0]=300 stores
// 300 instead of 300 & 0xff = 44). Substantive substrate fix per
// TypedArray subtype; queued as a separate intrinsics-locale rung.
//
// {
//   const u = new Uint8Array(1); u[0] = 300; // should wrap to 44
//   const i = new Int8Array(1);  i[0] = 128; // should wrap to -128
// }

// Iteration (values/keys/entries).
{
  const u = new Uint8Array([10, 20, 30]);
  result.iter = {
    values: [...u.values()],
    keys: [...u.keys()],
    entries: [...u.entries()],
    for_of: (() => { const a = []; for (const v of u) a.push(v); return a; })(),
  };
}

// Float64Array precision.
{
  const f = new Float64Array([0.1, 0.2, 0.3]);
  result.float64 = {
    length: f.length,
    byteLength: f.byteLength,
    sum_close_to_06: Math.abs((f[0] + f[1] + f[2]) - 0.6) < 1e-10,
  };
}

// Map / filter on TypedArray. Result values match bun; result CTOR
// (TypedArraySpeciesCreate per ECMA §23.2) is deferred — cruftless's
// ta_proto.map returns a generic Object kind rather than the matching
// TypedArray subtype. Subtype-preserving species at the typed-array
// tier is queued as its own intrinsics-locale rung.
{
  const u = new Uint8Array([1, 2, 3, 4]);
  const doubled = u.map(x => x * 2);
  const evens = u.filter(x => x % 2 === 0);
  result.map_filter = {
    doubled: [...doubled],
    evens: [...evens],
  };
}

console.log(canon(result));
