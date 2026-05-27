// F-category: array length exotic behavior.

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
  const a = [1, 2, 3, 4, 5];
  a.length = 3;
  result.truncate = { arr: Array.from(a), len: a.length };
}

{
  const a = [1, 2];
  a.length = 5;
  result.extend = { len: a.length, has2: 2 in a, has3: 3 in a, has4: 4 in a, val3: a[3] };
}

{
  const a = [10, 20, 30];
  a.length = 0;
  result.truncate_to_zero = { arr: Array.from(a), len: a.length };
}

{
  const a = Array(5);
  result.holey_constructor = {
    len: a.length,
    has0: 0 in a,
    has4: 4 in a,
    val0: a[0],
    keys: Object.keys(a),
  };
}

{
  const a = [1, 2, 3, 4, 5];
  delete a[2];
  result.delete_creates_hole = {
    len: a.length,
    has1: 1 in a,
    has2: 2 in a,
    has3: 3 in a,
    val2: a[2],
    keys: Object.keys(a),
  };
}

{
  const a = [10, , 20, , 30];
  result.literal_holes = {
    len: a.length,
    has0: 0 in a,
    has1: 1 in a,
    has2: 2 in a,
    has3: 3 in a,
    has4: 4 in a,
  };
}

{
  const a = [1, , 3, , 5];
  const forEachLog = [];
  a.forEach((v, i) => forEachLog.push({ i, v }));
  result.forEach_skips_holes = forEachLog;
}

{
  const a = [1, , 3, , 5];
  const mapped = a.map((v, i) => v * 10 + i);
  result.map_preserves_holes = {
    len: mapped.length,
    has0: 0 in mapped,
    has1: 1 in mapped,
    has2: 2 in mapped,
    has3: 3 in mapped,
    has4: 4 in mapped,
    val0: mapped[0],
    val2: mapped[2],
    val4: mapped[4],
  };
}

{
  const a = [1, , 3, , 5];
  const filtered = a.filter(v => v > 2);
  result.filter_skips_holes = { arr: filtered, len: filtered.length };
}

{
  const a = [1, , 3, , 5];
  const found = a.find((v, i) => i === 1);
  result.find_visits_holes = { found };
}

{
  const a = [1, , 3, , 5];
  const idx = a.findIndex((v, i) => i === 1);
  result.findIndex_visits_holes = { idx };
}

{
  const a = [1, , 3, , 5];
  result.indexOf_on_sparse = {
    indexOf_undefined: a.indexOf(undefined),
    includes_undefined: a.includes(undefined),
  };
}

{
  const a = [1, , 3, , 5];
  const joined = a.join("-");
  result.join_on_sparse = joined;
}

{
  const a = [, , ,];
  result.trailing_elision = { len: a.length, has0: 0 in a };
}

{
  const a = [10, 20, 30];
  a[10] = 99;
  result.sparse_assign = { len: a.length, has5: 5 in a, has10: 10 in a, keys: Object.keys(a) };
}

{
  const a = [1, , 3];
  const spread = [...a];
  result.spread_holes = {
    len: spread.length,
    has0: 0 in spread,
    has1: 1 in spread,
    has2: 2 in spread,
    val1: spread[1],
  };
}

console.log(canon(result));
