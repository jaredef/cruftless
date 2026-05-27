# ts-resolve-type-only-imports — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. TS source lowering is measured by TCC/TXC apparatus. This locale detects imports whose bindings are used only in stripped-out type positions and elides them, eliminating purely-typological ESM cycles (e.g. rxjs types.ts). Yield measured by TXC execute-parity (60.7% to 69.0%, +8.3 pp).
