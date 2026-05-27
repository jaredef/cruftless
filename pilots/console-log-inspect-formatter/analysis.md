# console-log-inspect-formatter — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| console-assert | FAIL | console.assert uses the same formatting path as console.log; formatting gaps affect its output |
| json-roundtrip | PASS | JSON.stringify produces correct output; the gap is console.log's inspect path, not serialization |
| map-set-ops | PASS | Map/Set operations work; console.log rendering of Map/Set objects (showing `[object Object]`) is the formatter gap |
| structured-clone | PASS | Structured clone works; the formatter is a display concern orthogonal to data fidelity |

The FAIL on console-assert is directly connected: console.assert's failure message formatting uses the same `abstract_ops::to_string` path that produces `[object Object]` for all Objects. Every diff-prod fixture that relies on console.log output of non-string values is potentially affected by this formatter gap, though most fixtures that pass likely use primitive console.log arguments or do not diff console output of complex objects. The locale's fix (adding `inspect_value` with recursive type-aware formatting) would also close the console-assert rendering gap.
