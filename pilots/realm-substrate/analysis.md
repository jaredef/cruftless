# realm-substrate — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| prototype-chain | PASS | Realm isolation requires per-realm prototype chains for intrinsics |
| proxy-basics | PASS | Proxy-mediated realm boundaries exercise prototype identity separation |
| array-methods | PASS | Array.prototype.map override attack is the locale's first-cut probe |

This locale answers whether realm-scoping the capability-passing runtime closes shared-intrinsic attacks (prototype pollution via Array.prototype.map override). The diff-prod fixtures test valid program behavior, not adversarial cross-realm prototype pollution, so all are PASS. The locale's empirical answer is measured by a dedicated probe (prototype_pollution.mjs), not by diff-prod. The structural work (per-module intrinsic rebinding) is gated on the probe outcome.
