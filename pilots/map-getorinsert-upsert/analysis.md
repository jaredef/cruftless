# map-getorinsert-upsert — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| map-set-ops | PASS | Map storage and prototype methods are the foundation for getOrInsert |
| es-recent-methods | PASS | TC39 Stage 3/4 proposal methods including upsert patterns |

This locale implements Map.prototype.getOrInsert and getOrInsertComputed, a TC39 Stage 3 upsert proposal. The map-set-ops fixture validates the underlying Map.prototype.{get,set,has,delete} that getOrInsert builds upon. Both pass, consistent with the locale's CLOSED status. No diff-prod fixture directly exercises the getOrInsert/getOrInsertComputed API surface.
