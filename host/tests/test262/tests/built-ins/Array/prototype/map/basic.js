// Copyright (C) 2025 Cruftless project.
/*---
esid: sec-array.prototype.map
description: Array.prototype.map applies callback to each element
---*/
assert.compareArray([1, 2, 3].map(function (x) { return x * 2; }), [2, 4, 6]);
assert.compareArray([].map(function (x) { return x; }), []);
assert.sameValue([1, 2, 3].map(function (x, i) { return i; }).join(","), "0,1,2");
