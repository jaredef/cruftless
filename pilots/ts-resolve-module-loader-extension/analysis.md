# ts-resolve-module-loader-extension — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. TS source lowering is measured by TCC/TXC apparatus. This locale extends the runtime module loader to try .ts/.mts/.cts extensions and apply ts_resolve::strip_ts at import time. Largest single-fix yield observed: execute-parity 5.1% to 52.7% (+47.6 pp).
