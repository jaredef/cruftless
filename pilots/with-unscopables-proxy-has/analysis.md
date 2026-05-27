# with-unscopables-proxy-has — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| with-scoping | FAIL | with-environment HasBinding is the direct surface; unscopables/Proxy compose here |
| proxy-basics | PASS | Proxy has-trap dispatch is a prerequisite; existing Op::In already materializes it |
| proxy-invariants | FAIL | Proxy trap invariant enforcement (mechanism gap #7) applies to the has-trap in with lookup |
| proxy-prototype-chain | FAIL | Proxy prototype chain traversal composes with with-environment HasProperty |

This locale implements Object Environment Record HasBinding for `with`: Proxy `has` trap dispatch plus Symbol.unscopables exclusion. The with-scoping FAIL is the direct surface. The proxy-invariants FAIL connects via mechanism gap #7 (Proxy trap invariant enforcement): the has-trap must enforce invariants when called from with-environment lookup. The proxy-prototype-chain FAIL is relevant because HasProperty on the with-object must traverse the prototype chain including Proxy layers.
