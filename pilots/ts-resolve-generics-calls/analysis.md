# ts-resolve-generics-calls — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. TS source lowering is measured by TCC/TXC apparatus. This locale extends strip.rs to handle generic arrow functions and generic instantiation/method calls, resolving the classic a<b operator vs f<T> generic-call ambiguity. Yield measured by TCC parse-success rate (59.9% to 69.3%, +9.4 pp).
