# rusty-js-ir — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| math-statics | PASS | IR-encoded Math sections (e.g. Math.max, Math.min) pass parity |
| number-math | PASS | IR-encoded Number methods pass parity |
| string-ops | PASS | IR-encoded String.prototype methods pass parity |
| array-methods | PASS | IR-encoded Array.prototype methods pass parity |

The IR locale translates ECMA-262 algorithm sections into a spec-as-source-of-truth intermediate representation feeding the runtime. Diff-prod fixtures that exercise IR-covered built-in methods validate the translation fidelity. The four PASS fixtures above confirm that IR-encoded sections produce byte-identical output to bun. No FAIL fixtures trace specifically to IR translation error; failures in related areas trace to compiler-tier gaps upstream of IR.
