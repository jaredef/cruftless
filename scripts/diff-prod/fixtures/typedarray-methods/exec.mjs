// F-category: TypedArray methods.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

{
  const a = Int32Array.from([10, 20, 30]);
  result.from_array = { vals: Array.from(a), len: a.length, name: a.constructor.name };
}

{
  const a = Float64Array.from([1.5, 2.5, 3.5], x => x * 2);
  result.from_mapFn = Array.from(a);
}

{
  const a = Uint8Array.of(1, 2, 3, 4, 5);
  result.of_basic = { vals: Array.from(a), len: a.length };
}

{
  const a = Int16Array.of(100, 200, 300, 400, 500);
  const sub = a.subarray(1, 4);
  result.subarray = {
    vals: Array.from(sub),
    len: sub.length,
    same_buffer: sub.buffer === a.buffer,
    byte_offset: sub.byteOffset,
  };
  sub[0] = 999;
  result.subarray_shared = { original_1: a[1], sub_0: sub[0] };
}

{
  const a = Uint8Array.of(10, 20, 30, 40, 50);
  const sliced = a.slice(1, 4);
  result.slice = {
    vals: Array.from(sliced),
    len: sliced.length,
    same_buffer: sliced.buffer === a.buffer,
  };
  sliced[0] = 255;
  result.slice_copy = { original_1: a[1], sliced_0: sliced[0] };
}

{
  const a = Int32Array.of(30, 10, 50, 20, 40);
  a.sort();
  result.sort_default = Array.from(a);
}

{
  const a = Int32Array.of(30, 10, 50, 20, 40);
  a.sort((x, y) => y - x);
  result.sort_descending = Array.from(a);
}

{
  const a = Uint8Array.of(1, 2, 3, 4, 5);
  const filtered = a.filter(v => v % 2 !== 0);
  result.filter = { vals: Array.from(filtered), len: filtered.length, name: filtered.constructor.name };
}

{
  const a = Float32Array.of(1, 2, 3, 4);
  const mapped = a.map(v => v * v);
  result.map = { vals: Array.from(mapped), name: mapped.constructor.name };
}

{
  const a = Int32Array.of(10, 20, 30, 40, 50);
  result.find = a.find(v => v > 25);
  result.find_none = a.find(v => v > 100);
}

{
  const a = Int32Array.of(10, 20, 30, 40, 50);
  result.findIndex = a.findIndex(v => v > 25);
  result.findIndex_none = a.findIndex(v => v > 100);
}

{
  const a = Uint8Array.of(5, 10, 15, 20, 25);
  result.indexOf = a.indexOf(15);
  result.indexOf_missing = a.indexOf(99);
  result.indexOf_from = a.indexOf(15, 3);
}

{
  const a = Uint8Array.of(5, 10, 15, 20, 25);
  result.includes_yes = a.includes(15);
  result.includes_no = a.includes(99);
}

{
  const dst = new Uint8Array(5);
  const src = Uint8Array.of(10, 20, 30);
  dst.set(src, 1);
  result.set_from_typed = Array.from(dst);
}

{
  const dst = new Uint8Array(4);
  dst.set([100, 200], 2);
  result.set_from_array = Array.from(dst);
}

{
  const a = Int32Array.of(2, 4, 6, 8);
  result.every_true = a.every(v => v % 2 === 0);
  result.every_false = a.every(v => v > 3);
}

{
  const a = Int32Array.of(1, 3, 5, 8);
  result.some_true = a.some(v => v % 2 === 0);
  result.some_false = a.some(v => v > 10);
}

{
  const a = Int32Array.of(1, 2, 3, 4, 5);
  result.reduce_sum = a.reduce((acc, v) => acc + v, 0);
  result.reduce_no_init = a.reduce((acc, v) => acc + v);
}

{
  const a = new Int32Array([10, 20, 30]);
  result.ctor_from_array = { vals: Array.from(a), len: a.length };
}

{
  const buf = new ArrayBuffer(12);
  const view = new DataView(buf);
  view.setInt32(0, 100, true);
  view.setInt32(4, 200, true);
  view.setInt32(8, 300, true);
  const a = new Int32Array(buf, 4, 2);
  result.ctor_offset = { vals: Array.from(a), byteOffset: a.byteOffset, len: a.length };
}

{
  const a = new Uint8Array(3);
  result.ctor_length = { vals: Array.from(a), len: a.length, allZero: a.every(v => v === 0) };
}

{
  const a = Uint8Array.of(1, 2, 3, 4, 5);
  result.reverse = Array.from(a.reverse());
}

{
  const a = Uint8Array.of(1, 2, 3, 4, 5);
  result.fill = Array.from(a.fill(99, 1, 4));
}

{
  const a = Uint8Array.of(1, 2, 3, 4, 5);
  result.copyWithin = Array.from(a.copyWithin(1, 3));
}

{
  const a = Uint8Array.of(10, 20, 30);
  result.entries = Array.from(a.entries());
  result.keys = Array.from(a.keys());
  result.values = Array.from(a.values());
}

console.log(canon(result));
