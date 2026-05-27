# intl402-availability — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| intl-collator-segmenter | FAIL | Directly exercises Intl.Collator and Intl.Segmenter availability and behavior |
| intl-numberformat | FAIL | Directly exercises Intl.NumberFormat functionality |

Both Intl-related fixtures FAIL, which is entirely consistent with this locale's scope: cruft exposes constructor-shaped stubs for Intl services but lacks the 3,045 ECMA-402 behaviors behind them (2,008 missing-global-or-binding, 382 wrong-result, 259 missing-method-or-intrinsic). The fixture failures reflect the stub-only state of the Intl implementation surface.
