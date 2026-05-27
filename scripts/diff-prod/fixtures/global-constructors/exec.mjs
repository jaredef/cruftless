// F-category: global constructor and namespace presence.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// globalThis presence and identity.
{
  result.globalThis_exists = typeof globalThis !== "undefined";
  result.globalThis_is_object = typeof globalThis === "object";
  result.globalThis_self_ref = globalThis === globalThis.globalThis;
}

// Standard constructors: should be "function".
{
  const constructors = [
    "Object", "Array", "Function", "Boolean", "Number", "String",
    "Symbol", "BigInt", "Error", "TypeError", "RangeError",
    "ReferenceError", "SyntaxError", "URIError", "EvalError",
    "Map", "Set", "WeakMap", "WeakSet", "Promise", "Proxy",
    "RegExp", "Date", "ArrayBuffer", "DataView",
    "Int8Array", "Uint8Array", "Uint8ClampedArray",
    "Int16Array", "Uint16Array", "Int32Array", "Uint32Array",
    "Float32Array", "Float64Array", "BigInt64Array", "BigUint64Array",
    "WeakRef", "FinalizationRegistry",
  ];
  const types = {};
  for (const name of constructors) {
    try {
      types[name] = typeof globalThis[name];
    } catch (e) {
      types[name] = "error:" + e.constructor.name;
    }
  }
  result.constructor_types = types;
}

// Namespace objects: should be "object".
{
  const namespaces = ["Math", "JSON", "Reflect", "Atomics"];
  const types = {};
  for (const name of namespaces) {
    try {
      types[name] = typeof globalThis[name];
    } catch (e) {
      types[name] = "error:" + e.constructor.name;
    }
  }
  result.namespace_types = types;
}

// SharedArrayBuffer: may be "function" or "undefined" depending on context.
{
  result.SharedArrayBuffer_typeof = typeof globalThis.SharedArrayBuffer;
}

// Verify constructor prototype chains.
{
  try {
    result.array_proto = Array.prototype === Object.getPrototypeOf([]);
    result.error_proto = Error.prototype === Object.getPrototypeOf(new Error());
    result.map_proto = Map.prototype === Object.getPrototypeOf(new Map());
    result.set_proto = Set.prototype === Object.getPrototypeOf(new Set());
    result.promise_proto = Promise.prototype === Object.getPrototypeOf(Promise.resolve());
    result.regexp_proto = RegExp.prototype === Object.getPrototypeOf(/x/);
    result.date_proto = Date.prototype === Object.getPrototypeOf(new Date(0));
  } catch (e) {
    result.proto_chains = e.constructor.name;
  }
}

// Error subclass chains.
{
  try {
    const te = new TypeError("t");
    result.typeerror_is_error = te instanceof Error;
    result.typeerror_is_typeerror = te instanceof TypeError;
    const re = new RangeError("r");
    result.rangeerror_is_error = re instanceof Error;
    const rfe = new ReferenceError("rf");
    result.referenceerror_is_error = rfe instanceof Error;
  } catch (e) {
    result.error_chains = e.constructor.name;
  }
}

// Typed array constructors have BYTES_PER_ELEMENT.
{
  const typed = [
    "Int8Array", "Uint8Array", "Uint8ClampedArray",
    "Int16Array", "Uint16Array", "Int32Array", "Uint32Array",
    "Float32Array", "Float64Array", "BigInt64Array", "BigUint64Array",
  ];
  const bpe = {};
  for (const name of typed) {
    try {
      bpe[name] = globalThis[name].BYTES_PER_ELEMENT;
    } catch (e) {
      bpe[name] = "error";
    }
  }
  result.bytes_per_element = bpe;
}

// Math and JSON are not constructors.
{
  try { new Math(); result.math_new = "no_throw"; } catch (e) { result.math_new = e.constructor.name; }
  try { new JSON(); result.json_new = "no_throw"; } catch (e) { result.json_new = e.constructor.name; }
}

// Proxy is a constructor but has no prototype property.
{
  result.proxy_is_function = typeof Proxy === "function";
  result.proxy_has_prototype = "prototype" in Proxy;
}

console.log(canon(result));
