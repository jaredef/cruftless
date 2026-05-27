# weakmap-upsert-methods — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| map-set-ops | PASS | Map/WeakMap share storage shape; getOrInsert was installed on Map.prototype only |
| weakmap-weakset | PASS | WeakMap is the direct target surface for the missing upsert method installation |

This locale removes the `if !is_weak_proto` gate so that getOrInsert and getOrInsertComputed are installed on WeakMap.prototype alongside Map.prototype. Both relevant fixtures PASS, suggesting the fix has landed or the diff-prod fixtures do not specifically probe WeakMap.prototype.getOrInsert. The 32 test262 failures that triggered this locale are outside the diff-prod fixture surface. No numbered mechanism gap applies; the bug was a conditional-installation gate error.
