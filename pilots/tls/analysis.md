# tls — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. This locale targets host/infrastructure APIs outside the ECMA-262 behavioral surface. TLS 1.3 handshake, cipher suites, ALPN, and certificate verification are transport-tier concerns measured by the CDN-passable probe set (registry.npmjs.org, api.github.com, etc.), not by diff-prod.
