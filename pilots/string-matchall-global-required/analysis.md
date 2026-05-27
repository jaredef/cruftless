# string-matchall-global-required — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regexp-ops | PASS | String.prototype.matchAll takes a RegExp argument; regexp-ops exercises this surface |
| string-ops | PASS | matchAll is a String.prototype method; string-ops is the direct surface family |
| error-types | PASS | TypeError is the expected throw when the /g flag is missing |

This locale adds the spec-mandated TypeError throw when String.prototype.matchAll receives a non-global RegExp. All relevant fixtures PASS, which is consistent with either the fix having landed or the diff-prod fixtures not specifically probing the non-global-RegExp edge case. No numbered mechanism gap applies; the bug was a missing validation check, not a protocol-level gap.
