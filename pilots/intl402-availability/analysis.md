# intl402-availability — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| intl-collator-segmenter | FAIL | Intl.Collator and Intl.Segmenter are core ECMA-402 surfaces this locale targets |
| intl-numberformat | FAIL | Intl.NumberFormat is the #2 core Intl surface (204 test262 fails) in this coordinate |

Both relevant diff-prod fixtures FAIL, directly confirming the locale's scope: Intl constructor stubs exist but lack ECMA-402-conformant semantics. The intl-collator-segmenter FAIL witnesses gaps in Collator (34 test262 fails) and Segmenter (60 test262 fails). The intl-numberformat FAIL witnesses the NumberFormat surface (204 test262 fails), identified as the likely first substrate rung. No numbered mechanism gap applies; the gaps are missing intrinsic-object semantics rather than engine protocol issues.
