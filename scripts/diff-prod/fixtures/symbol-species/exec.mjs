// F-category: Symbol.species factory pattern.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Symbol.species exists.
{
  result.species_exists = typeof Symbol.species === "symbol";
}

// Array[@@species] defaults to Array.
{
  result.array_default_species = {
    has_species: Symbol.species in Array,
    is_self: Array[Symbol.species] === Array,
  };
}

// Array subclass: map/filter/slice use species constructor.
{
  class MyArray extends Array {
    static get [Symbol.species]() { return Array; }
  }
  const ma = MyArray.from([1, 2, 3]);
  const mapped = ma.map(x => x * 2);
  const filtered = ma.filter(x => x > 1);
  const sliced = ma.slice(0, 2);

  result.array_species = {
    source_is_myarray: ma instanceof MyArray,
    mapped_is_array: mapped instanceof Array,
    mapped_not_myarray: !(mapped instanceof MyArray),
    filtered_is_array: filtered instanceof Array,
    sliced_is_array: sliced instanceof Array,
    mapped_values: [...mapped],
    filtered_values: [...filtered],
    sliced_values: [...sliced],
  };
}

// Array subclass without species override: derived methods return subclass instances.
{
  class SubArray extends Array {}
  const sa = SubArray.from([1, 2, 3]);
  const mapped = sa.map(x => x * 10);
  const filtered = sa.filter(x => x > 1);
  const sliced = sa.slice(1);

  result.array_no_override = {
    source_is_subarray: sa instanceof SubArray,
    mapped_is_subarray: mapped instanceof SubArray,
    filtered_is_subarray: filtered instanceof SubArray,
    sliced_is_subarray: sliced instanceof SubArray,
  };
}

// Array subclass with species = null: falls back to Array.
{
  class NullSpecies extends Array {
    static get [Symbol.species]() { return null; }
  }
  const ns = NullSpecies.from([1, 2, 3]);
  let threw = false;
  let mapped_type = null;
  try {
    const m = ns.map(x => x);
    mapped_type = m.constructor.name;
  } catch { threw = true; }
  result.array_null_species = { threw, mapped_type };
}

// Promise[@@species].
{
  result.promise_species = {
    has_species: Symbol.species in Promise,
    is_self: Promise[Symbol.species] === Promise,
  };
}

// Promise subclass with species override: then() uses species.
{
  let speciesCalled = false;
  class MyPromise extends Promise {
    static get [Symbol.species]() {
      speciesCalled = true;
      return Promise;
    }
  }
  const mp = MyPromise.resolve(42);
  const chained = mp.then(x => x);
  result.promise_species_then = {
    source_is_mypromise: mp instanceof MyPromise,
    chained_is_promise: chained instanceof Promise,
  };
}

// Map[@@species].
{
  result.map_species = {
    has_species: Symbol.species in Map,
    is_self: Map[Symbol.species] === Map,
  };
}

// Set[@@species].
{
  result.set_species = {
    has_species: Symbol.species in Set,
    is_self: Set[Symbol.species] === Set,
  };
}

// RegExp[@@species].
{
  result.regexp_species = {
    has_species: Symbol.species in RegExp,
    is_self: RegExp[Symbol.species] === RegExp,
  };
}

// TypedArray species: Int32Array.map should use species.
{
  class MyInt32 extends Int32Array {
    static get [Symbol.species]() { return Int32Array; }
  }
  const mi = new MyInt32([1, 2, 3]);
  const mapped = mi.map(x => x * 2);
  result.typedarray_species = {
    source_is_myint32: mi instanceof MyInt32,
    mapped_is_int32: mapped instanceof Int32Array,
    mapped_not_myint32: !(mapped instanceof MyInt32),
    mapped_values: [...mapped],
  };
}

// ArrayBuffer[@@species].
{
  result.arraybuffer_species = {
    has_species: Symbol.species in ArrayBuffer,
    is_self: ArrayBuffer[Symbol.species] === ArrayBuffer,
  };
}

console.log(canon(result));
