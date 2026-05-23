// F-category: ES2024-2026 widely-adopted method surface.
// Object.groupBy + Map.groupBy (ES2024), Set methods (ES2025: intersection / union / difference / symmetricDifference / isSubsetOf / isSupersetOf / isDisjointFrom),
// Promise.try (ES2026 stage 4), Error.isError (ES2025), Array.prototype.{findLast,findLastIndex} (ES2023),
// Array.prototype.toSorted/toReversed/toSpliced/with (ES2023).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v instanceof Map) {
      const out = {}; for (const k of [...v.keys()].sort()) out[String(k)] = v.get(k); return { __map: out };
    }
    if (v instanceof Set) {
      return { __set: [...v].sort() };
    }
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// ES2024: Object.groupBy / Map.groupBy
{
  const xs = [1, 2, 3, 4, 5, 6];
  result.object_group_by = Object.groupBy(xs, (n) => (n % 2 === 0 ? "even" : "odd"));
  const m = Map.groupBy(xs, (n) => n % 3);
  result.map_group_by = m;
}

// ES2025: Set methods
{
  const a = new Set([1, 2, 3, 4]);
  const b = new Set([3, 4, 5, 6]);
  result.set_methods = {
    intersection: a.intersection(b),
    union: a.union(b),
    difference: a.difference(b),
    symmetric_difference: a.symmetricDifference(b),
    is_subset_self: a.isSubsetOf(a),
    is_subset_no: a.isSubsetOf(b),
    is_superset_self: a.isSupersetOf(a),
    is_disjoint_no: a.isDisjointFrom(b),
    is_disjoint_yes: new Set([7, 8]).isDisjointFrom(a),
  };
}

// ES2026 stage 4: Promise.try
{
  const sync_ok = await Promise.try(() => 42);
  let sync_err = null;
  try { await Promise.try(() => { throw new Error("sync-throw"); }); } catch (e) { sync_err = e.message; }
  const async_ok = await Promise.try(async () => 99);
  result.promise_try = { sync_ok, sync_err, async_ok };
}

// ES2025: Error.isError
{
  result.error_is_error = {
    plain: Error.isError(new Error("x")),
    typed: Error.isError(new TypeError("y")),
    not_error: Error.isError({ message: "fake" }),
    null_safe: Error.isError(null),
  };
}

// ES2023: findLast / findLastIndex
{
  const xs = [1, 2, 3, 4, 5];
  result.find_last = {
    val: xs.findLast((x) => x < 4),
    idx: xs.findLastIndex((x) => x < 4),
    no_match: xs.findLast((x) => x > 99),
    no_match_idx: xs.findLastIndex((x) => x > 99),
  };
}

// ES2023: change-array-by-copy
{
  const xs = [3, 1, 4, 1, 5, 9];
  const sorted = xs.toSorted((a, b) => a - b);
  const reversed = xs.toReversed();
  const spliced = xs.toSpliced(1, 2, 99);
  const withed = xs.with(0, 0);
  result.change_by_copy = {
    sorted, reversed, spliced, withed,
    original_unchanged: canon(xs) === canon([3, 1, 4, 1, 5, 9]),
  };
}

console.log(canon(result));
