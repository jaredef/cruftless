# reference-error-on-unresolvable — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| reference-semantics | FAIL | Unresolvable identifier reads must throw ReferenceError per GetValue spec |
| error-types | PASS | ReferenceError is the expected error class for unresolvable reads |
| error-throws | PASS | Error throwing machinery must correctly surface ReferenceError |

This locale split Op::LoadGlobal into a throwing variant and Op::LoadGlobalOrUndef (for typeof/delete special cases) so unresolvable identifier reads throw ReferenceError per ECMA-262 section 6.2.4.5. The reference-semantics FAIL is directly relevant since it exercises identifier resolution paths where the prior silent-undefined behavior violated the spec. The locale is CLOSED at REOU-EXT 1 via Doc 740 multi-tier closure across opcode, runtime, and compiler sites.
