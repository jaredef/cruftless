# via-method-audit — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| coercion-pipeline | FAIL | Mechanism gap #1 (ToPrimitive hint dispatch) is exactly bug pattern #2 (static-coerce on user-arg) |
| array-methods | PASS | Array.prototype _via methods were the original LOAL trigger for this audit |
| prototype-chain | PASS | Prototype chain traversal composes with spec-order validation in _via methods |
| symbol-toprimitive | FAIL | @@toPrimitive dispatch is the mechanism static abstract_ops::to_string bypasses |

This locale audits all 228 _via methods in interp.rs for spec-order divergence and static-coerce-on-user-arg bugs. The coercion-pipeline FAIL and symbol-toprimitive FAIL are directly relevant: mechanism gap #1 (ToPrimitive hint dispatch) is the exact pattern this audit's bug pattern #2 detects (abstract_ops::to_string/to_number at user-arg boundaries instead of rt.coerce_to_*). Fixing instances found by this audit should contribute to flipping both failing fixtures.
