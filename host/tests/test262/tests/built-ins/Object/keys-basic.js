// Copyright (C) 2025 Cruftless project.
/*---
esid: sec-object.keys
description: Object.keys returns own enumerable string keys
---*/
assert.compareArray(Object.keys({ a: 1, b: 2 }), ["a", "b"]);
assert.compareArray(Object.keys({}), []);
assert.sameValue(Object.keys.length, 1);
