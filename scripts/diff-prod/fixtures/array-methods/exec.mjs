// L-category: Array.prototype surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// flat / flatMap.
{
  result.flat = {
    one: [1, [2, [3, [4]]]].flat(),
    two: [1, [2, [3, [4]]]].flat(2),
    inf: [1, [2, [3, [4]]]].flat(Infinity),
  };
  result.flat_map = [1, 2, 3].flatMap(x => [x, x * 10]);
}

// at — including negative.
{
  const a = [10, 20, 30, 40];
  result.at = { first: a.at(0), last: a.at(-1), neg2: a.at(-2), oor: a.at(100) };
}

// includes / indexOf.
{
  const a = [1, 2, 3, NaN];
  result.includes = {
    has_2: a.includes(2),
    no_5: a.includes(5),
    has_nan: a.includes(NaN),
    idx_nan: a.indexOf(NaN),  // -1 (strict-eq misses NaN)
  };
}

// find / findIndex / findLast.
{
  const a = [1, 2, 3, 4, 5];
  result.find = {
    first_even: a.find(x => x % 2 === 0),
    idx_first_even: a.findIndex(x => x % 2 === 0),
    last_even: typeof a.findLast === "function" ? a.findLast(x => x % 2 === 0) : "absent",
  };
}

// Sort stability.
{
  const items = [
    { k: "a", v: 1 }, { k: "b", v: 2 }, { k: "a", v: 3 },
    { k: "b", v: 4 }, { k: "a", v: 5 },
  ];
  const sorted = [...items].sort((x, y) => x.k.localeCompare(y.k));
  result.sort_stable = sorted.map(x => x.k + x.v);
}

// Non-mutating: toSorted, toReversed, with.
{
  if (typeof [].toSorted === "function") {
    const a = [3, 1, 2];
    const s = a.toSorted();
    result.to_sorted = { sorted: s, src_unchanged: [...a] };
  } else {
    result.to_sorted = "absent";
  }
  if (typeof [].toReversed === "function") {
    const a = [1, 2, 3];
    const r = a.toReversed();
    result.to_reversed = { reversed: r, src_unchanged: [...a] };
  }
  if (typeof [].with === "function") {
    const a = [1, 2, 3];
    const w = a.with(1, 99);
    result.with_idx = { mutated: w, src_unchanged: [...a] };
  }
}

console.log(canon(result));
