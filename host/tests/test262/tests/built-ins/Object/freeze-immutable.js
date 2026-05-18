// Copyright (C) 2025 Cruftless project.
/*---
esid: sec-object.freeze
description: Object.freeze makes properties non-writable
---*/
var o = { a: 1 };
Object.freeze(o);
o.a = 2;  // Sloppy-mode: silent no-op. Strict-mode would throw.
assert.sameValue(o.a, 1, "frozen property unchanged");
assert.sameValue(Object.isFrozen(o), true);
