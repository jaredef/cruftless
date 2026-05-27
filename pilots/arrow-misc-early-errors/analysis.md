# arrow-misc-early-errors — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| arrow-functions | PASS | Arrow function syntax and semantics work broadly; the locale targets two narrow early-error gaps |
| arrow-edge-cases | FAIL | Edge cases in arrow functions -- may include the ASI-restriction and rest-default early errors this locale targets |
| asi-rules | PASS | ASI rules pass generally; the no-LineTerminator-before-=> restriction is a specific ASI sub-case |

The PASS on arrow-functions confirms that core arrow syntax and semantics are healthy. The FAIL on arrow-edge-cases may partially overlap with this locale's scope: the no-LineTerminator-before-=> restriction (`() \n => {}` should be SyntaxError) and the rest-with-default early error (`(...x = []) => {}` should be SyntaxError) are both edge-case arrow grammar rules. The PASS on asi-rules shows general ASI works but does not probe the arrow-specific no-LineTerminator restriction.
