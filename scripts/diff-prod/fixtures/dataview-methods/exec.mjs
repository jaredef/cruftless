// F-category: DataView.prototype method surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Int8.
{
  const buf = new ArrayBuffer(4);
  const dv = new DataView(buf);
  dv.setInt8(0, 127);
  dv.setInt8(1, -128);
  dv.setInt8(2, 0);
  dv.setInt8(3, -1);
  result.int8 = {
    v0: dv.getInt8(0),
    v1: dv.getInt8(1),
    v2: dv.getInt8(2),
    v3: dv.getInt8(3),
  };
}

// Uint8.
{
  const buf = new ArrayBuffer(4);
  const dv = new DataView(buf);
  dv.setUint8(0, 0);
  dv.setUint8(1, 128);
  dv.setUint8(2, 255);
  result.uint8 = {
    v0: dv.getUint8(0),
    v1: dv.getUint8(1),
    v2: dv.getUint8(2),
  };
}

// Int16 + endianness.
{
  const buf = new ArrayBuffer(4);
  const dv = new DataView(buf);
  dv.setInt16(0, 0x1234, false);
  dv.setInt16(2, 0x1234, true);
  result.int16 = {
    be: dv.getInt16(0, false),
    le: dv.getInt16(2, true),
    cross_endian_be_as_le: dv.getInt16(0, true),
  };
}

// Uint16.
{
  const buf = new ArrayBuffer(4);
  const dv = new DataView(buf);
  dv.setUint16(0, 65535, false);
  dv.setUint16(2, 0, true);
  result.uint16 = {
    max_be: dv.getUint16(0, false),
    zero_le: dv.getUint16(2, true),
  };
}

// Int32.
{
  const buf = new ArrayBuffer(8);
  const dv = new DataView(buf);
  dv.setInt32(0, 2147483647, false);
  dv.setInt32(4, -2147483648, true);
  result.int32 = {
    max_be: dv.getInt32(0, false),
    min_le: dv.getInt32(4, true),
  };
}

// Float32.
{
  const buf = new ArrayBuffer(8);
  const dv = new DataView(buf);
  dv.setFloat32(0, 3.140000104904175, false);
  dv.setFloat32(4, -0, true);
  result.float32 = {
    pi_approx: dv.getFloat32(0, false),
    neg_zero: Object.is(dv.getFloat32(4, true), -0),
  };
}

// Float64.
{
  const buf = new ArrayBuffer(16);
  const dv = new DataView(buf);
  dv.setFloat64(0, Math.PI, false);
  dv.setFloat64(8, Number.MAX_SAFE_INTEGER, true);
  result.float64 = {
    pi: dv.getFloat64(0, false),
    max_safe: dv.getFloat64(8, true),
  };
}

// BigInt64 / BigUint64.
{
  const buf = new ArrayBuffer(16);
  const dv = new DataView(buf);
  try {
    dv.setBigInt64(0, 9007199254740993n, false);
    dv.setBigUint64(8, 18446744073709551615n, true);
    result.bigint64 = {
      signed: String(dv.getBigInt64(0, false)),
      unsigned: String(dv.getBigUint64(8, true)),
    };
  } catch (e) {
    result.bigint64 = { error: e.constructor.name, message: e.message };
  }
}

// byteLength / byteOffset with sub-view.
{
  const buf = new ArrayBuffer(16);
  const dv = new DataView(buf, 4, 8);
  result.offsets = {
    byteLength: dv.byteLength,
    byteOffset: dv.byteOffset,
    bufferByteLength: dv.buffer.byteLength,
  };
}

console.log(canon(result));
