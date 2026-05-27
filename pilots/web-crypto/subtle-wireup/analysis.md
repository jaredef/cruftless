# web-crypto/subtle-wireup — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. This locale targets host/infrastructure APIs outside the ECMA-262 behavioral surface. The subtle-wireup locale connects existing SubtleCrypto substrate to the global crypto.subtle JS surface. Its yield is measured by CRB's crypto_sha256_batch fixture and real Node package compatibility, not by diff-prod.
