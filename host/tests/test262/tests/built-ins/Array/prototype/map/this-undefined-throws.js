// Copyright (C) 2025 Cruftless project.
/*---
esid: sec-array.prototype.map
description: Array.prototype.map called with null/undefined this throws TypeError
negative:
  phase: runtime
  type: TypeError
---*/
Array.prototype.map.call(undefined, function () {});
