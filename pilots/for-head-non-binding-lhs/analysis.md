# for-head-non-binding-lhs — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| for-in-for-of-lowering | FAIL | Directly exercises for-in/for-of runtime semantics including non-binding LHS |
| prototype-chain | PASS | Proto-chain walk is prerequisite for for-in with MemberExpression LHS |

The FAIL on for-in-for-of-lowering aligns with this locale's scope: for-in/for-of with a MemberExpression LHS (e.g., `for (o.x in {...})`) silently drops the per-iteration PutValue because the compiler treats all ForBinding::Pattern cases as local-slot bindings. This is the runtime-tier gap downstream of the parse-tier fix (PPIF-EXT 1). The fixture likely covers LHS assignment-target forms that expose this missing Reference-based put.
