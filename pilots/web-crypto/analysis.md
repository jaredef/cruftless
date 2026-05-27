# web-crypto — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. This locale targets host/infrastructure APIs outside the ECMA-262 behavioral surface. Cryptographic primitives (SHA-2, AES-GCM, RSA, ECDSA, ECDH, HKDF, HMAC) serve TLS, JOSE, node:crypto, and WebCrypto consumers. Correctness is measured by the TLS handshake probe set and per-algorithm verification fixtures, not by diff-prod.
