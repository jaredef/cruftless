// F-category: ArrayBuffer operations.

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
  const buf = new ArrayBuffer(16);
  result.basic = {
    byteLength: buf.byteLength,
    is_object: typeof buf === "object",
    ctor_name: buf.constructor.name,
  };
}

{
  const buf = new ArrayBuffer(8);
  const view = new Uint8Array(buf);
  view.set([10, 20, 30, 40, 50, 60, 70, 80]);
  const sliced = buf.slice(2, 6);
  const slicedView = new Uint8Array(sliced);
  result.slice = {
    original_len: buf.byteLength,
    sliced_len: sliced.byteLength,
    sliced_vals: Array.from(slicedView),
  };
  slicedView[0] = 255;
  result.slice_is_copy = { original_2: view[2], sliced_0: slicedView[0] };
}

{
  const buf = new ArrayBuffer(8);
  const sliceFull = buf.slice(0);
  result.slice_full = { len: sliceFull.byteLength };
}

{
  const buf = new ArrayBuffer(8);
  const sliceNeg = buf.slice(-4);
  result.slice_negative = { len: sliceNeg.byteLength };
}

{
  result.isView = {
    uint8: ArrayBuffer.isView(new Uint8Array(1)),
    int32: ArrayBuffer.isView(new Int32Array(1)),
    dataview: ArrayBuffer.isView(new DataView(new ArrayBuffer(1))),
    buffer: ArrayBuffer.isView(new ArrayBuffer(1)),
    array: ArrayBuffer.isView([1, 2, 3]),
    null_val: ArrayBuffer.isView(null),
    number: ArrayBuffer.isView(42),
    string: ArrayBuffer.isView("hello"),
    object: ArrayBuffer.isView({}),
  };
}

{
  const buf = new ArrayBuffer(0);
  result.zero_length = { byteLength: buf.byteLength, sliced: buf.slice(0).byteLength };
}

try {
  if (typeof ArrayBuffer.prototype.transfer === "function") {
    const buf = new ArrayBuffer(8);
    const view = new Uint8Array(buf);
    view.set([1, 2, 3, 4, 5, 6, 7, 8]);
    const transferred = buf.transfer();
    const newView = new Uint8Array(transferred);
    result.transfer = {
      available: true,
      new_len: transferred.byteLength,
      new_vals: Array.from(newView),
      original_detached: buf.byteLength === 0,
    };
  } else {
    result.transfer = { available: false };
  }
} catch (e) {
  result.transfer = { available: "error", error: e.message };
}

try {
  if (typeof ArrayBuffer.prototype.transfer === "function") {
    const buf = new ArrayBuffer(8);
    const view = new Uint8Array(buf);
    view.set([10, 20, 30, 40, 50, 60, 70, 80]);
    const grown = buf.transfer(16);
    const grownView = new Uint8Array(grown);
    result.transfer_grow = {
      new_len: grown.byteLength,
      first_8: Array.from(grownView.slice(0, 8)),
      last_8: Array.from(grownView.slice(8)),
    };
  } else {
    result.transfer_grow = { available: false };
  }
} catch (e) {
  result.transfer_grow = { available: "error", error: e.message };
}

try {
  if (typeof ArrayBuffer.prototype.transfer === "function") {
    const buf = new ArrayBuffer(8);
    const view = new Uint8Array(buf);
    view.set([1, 2, 3, 4, 5, 6, 7, 8]);
    const shrunk = buf.transfer(4);
    const shrunkView = new Uint8Array(shrunk);
    result.transfer_shrink = {
      new_len: shrunk.byteLength,
      vals: Array.from(shrunkView),
    };
  } else {
    result.transfer_shrink = { available: false };
  }
} catch (e) {
  result.transfer_shrink = { available: "error", error: e.message };
}

try {
  if (typeof ArrayBuffer.prototype.resize === "function") {
    const buf = new ArrayBuffer(4, { maxByteLength: 16 });
    const view = new Uint8Array(buf);
    view.set([1, 2, 3, 4]);
    buf.resize(8);
    const view2 = new Uint8Array(buf);
    result.resize = {
      available: true,
      new_len: buf.byteLength,
      first_4: Array.from(view2.slice(0, 4)),
      last_4: Array.from(view2.slice(4)),
    };
  } else {
    result.resize = { available: false };
  }
} catch (e) {
  result.resize = { available: "error", error: e.message };
}

try {
  if (typeof ArrayBuffer.prototype.transferToFixedLength === "function") {
    const buf = new ArrayBuffer(4, { maxByteLength: 16 });
    const view = new Uint8Array(buf);
    view.set([10, 20, 30, 40]);
    const fixed = buf.transferToFixedLength();
    result.transferToFixedLength = {
      available: true,
      len: fixed.byteLength,
      vals: Array.from(new Uint8Array(fixed)),
      resizable: typeof fixed.resizable === "boolean" ? fixed.resizable : "no-prop",
    };
  } else {
    result.transferToFixedLength = { available: false };
  }
} catch (e) {
  result.transferToFixedLength = { available: "error", error: e.message };
}

try {
  if (typeof SharedArrayBuffer === "function") {
    const sab = new SharedArrayBuffer(8);
    const view = new Uint8Array(sab);
    view.set([1, 2, 3, 4, 5, 6, 7, 8]);
    const sliced = sab.slice(2, 6);
    const slicedView = new Uint8Array(sliced);
    result.shared_slice = {
      available: true,
      len: sliced.byteLength,
      vals: Array.from(slicedView),
      ctor: sliced.constructor.name,
    };
  } else {
    result.shared_slice = { available: false };
  }
} catch (e) {
  result.shared_slice = { available: "error", error: e.message };
}

try {
  if (typeof ArrayBuffer.prototype.transfer === "function") {
    const buf = new ArrayBuffer(4);
    const view = new Uint8Array(buf);
    view.set([1, 2, 3, 4]);
    buf.transfer();
    let threw = false;
    try {
      new Uint8Array(buf);
    } catch (e2) {
      threw = true;
    }
    result.detach_access = {
      byteLength_after: buf.byteLength,
      threw_on_new_view: threw,
    };
  } else {
    result.detach_access = { available: false };
  }
} catch (e) {
  result.detach_access = { available: "error", error: e.message };
}

{
  const buf = new ArrayBuffer(8);
  result.instanceof_check = {
    is_arraybuffer: buf instanceof ArrayBuffer,
    slice_returns_arraybuffer: buf.slice(0) instanceof ArrayBuffer,
  };
}

console.log(canon(result));
