# ts-resolve-string-literal-safety — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. TS source lowering is measured by TCC/TXC apparatus. This locale fixes TSR's strip.rs scanner so it does not corrupt template-literal substitution boundaries (LexerGoal::TemplateTail bug). Yield measured by TCC parse-success rate (37.7% to 47.1%, +9.4 pp).
