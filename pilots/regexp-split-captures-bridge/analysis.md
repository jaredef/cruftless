# regexp-split-captures-bridge — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regexp-ops | PASS | RegExp.prototype[@@split] is the method this locale fixes |
| regexp-symbol-protocols | FAIL | @@split capture interleaving is a well-known Symbol protocol gap |
| string-ops | PASS | String.prototype.split with regex separator delegates to @@split |

This locale fixes RegExp.prototype[@@split] to interleave captured group strings between split chunks per spec section 22.2.5.13, replacing the prior split_str call that discarded captures. The regexp-symbol-protocols FAIL is directly relevant since it exercises the @@split protocol where capture groups must appear in results. The fix replaces bulk split_str delegation with a custom loop using captures_at that pushes each capture (or undefined for non-participating groups) between chunks.
