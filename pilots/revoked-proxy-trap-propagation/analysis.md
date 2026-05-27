# revoked-proxy-trap-propagation — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| proxy-basics | PASS | Proxy revocation mechanics are foundational to this locale |
| proxy-invariants | FAIL | Revoked proxy trap enforcement is a proxy invariant compliance gap (mechanism gap #7) |
| proxy-prototype-chain | FAIL | Revoked proxy as receiver through prototype chain exercises species-routed methods |
| array-methods | PASS | concat/filter/map/slice/splice are the five array_species_create-routed methods fixed |

This locale adds revoked-proxy early-throw checks at array_species_create, the first-contact site for concat/filter/map/slice/splice when the receiver is a revoked proxy. The proxy-invariants FAIL (mechanism gap #7: proxy trap invariant enforcement) and proxy-prototype-chain FAIL are directly relevant since they exercise revoked-proxy TypeError propagation. The locale is CLOSED at RPTP-EXT 1 with +5 exemplar gains and zero regressions across 570 previously-passing array method tests.
