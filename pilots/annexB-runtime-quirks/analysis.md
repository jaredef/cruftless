# annexB-runtime-quirks — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| date-ops | PASS | Date methods exercise core Date prototype; Annex B getYear/setYear/toGMTString are adjacent legacy surfaces |
| string-ops | PASS | String prototype methods pass; Annex B HTML methods (anchor, bold, etc.) are additional string surfaces not covered |
| regexp-ops | PASS | Core RegExp passes; RegExp.prototype.compile is an Annex B extension not exercised here |

The diff-prod suite does not include fixtures targeting Annex B legacy surfaces specifically (getYear, setYear, toGMTString, String HTML methods, escape/unescape, RegExp.compile). The PASS status of date-ops, string-ops, and regexp-ops confirms that the core prototype methods these Annex B surfaces extend are healthy, so ABRQ work is purely additive -- implementing missing methods on stable prototypes with no regression risk to existing passing fixtures.
