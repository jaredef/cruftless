# ast-bytecode-missing-throw-typeerror — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| error-throws | PASS | General error throwing works; the gap is operations that should throw TypeError but silently succeed |
| error-types | PASS | Error type construction is correct; the locale targets missing throws, not wrong error types |
| symbol-toprimitive | FAIL | Symbol-to-primitive coercion should throw TypeError; same shared-upstream pattern (mechanism gap #1) |
| coercion-pipeline | FAIL | Coercion operations that should throw on illegal receivers/Symbols silently succeed |

The PASS on error-throws and error-types confirms cruft can throw and construct TypeErrors correctly; the gap is that certain operations silently succeed instead of throwing. The FAIL on symbol-toprimitive and coercion-pipeline directly overlaps with this locale's scope: ToPrimitive/ToObject/ResolveThisBinding operations that should throw TypeError on Symbol args or illegal receivers instead produce wrong results silently. Mechanism gap #1 (ToPrimitive hint dispatch) is the shared upstream cause for many of the 622 fixtures in this cluster.
