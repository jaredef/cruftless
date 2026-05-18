// Copyright (C) 2025 Cruftless project.
/*---
esid: sec-string.prototype.includes
description: String.prototype.includes basic positive + negative cases
---*/
assert.sameValue("hello world".includes("world"), true);
assert.sameValue("hello world".includes("World"), false);
assert.sameValue("abc".includes(""), true);
assert.sameValue("abc".includes("abc"), true);
assert.sameValue("abc".includes("d"), false);
