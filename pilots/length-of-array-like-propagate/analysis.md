# length-of-array-like-propagate — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| array-methods | PASS | Array.prototype methods are the primary callers of length_of_array_like |
| array-from-of | PASS | Array.from uses length_of_array_like for array-like sources |
| error-throws | PASS | Error propagation from getters works in general |

All relevant fixtures PASS, which is consistent with a locale that fixes error propagation from throwing length getters on Array-like objects. Diff-prod fixtures exercise normal-path behavior (non-throwing length access), so they would not expose the `unwrap_or(0)` silent-drop bug. The fix (routing through `try_array_length` instead of `array_length`) benefits 8 generated callers transparently but only manifests when the length getter actually throws, a pattern test262 probes but diff-prod fixtures typically do not.
