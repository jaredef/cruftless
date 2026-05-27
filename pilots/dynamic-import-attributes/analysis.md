# dynamic-import-attributes — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. The locale targets the `import(specifier, { with: { type: 'json' } })` parser grammar extension (stage-4, ratified in ECMA-262). Cruft currently throws `parse: expected ')' in dynamic import()` at the comma. Since `__dynamic_import` is already a throwing stub in the runtime, no diff-prod fixture uses dynamic import syntax -- the feature would fail at parse time before any runtime behavior could be diffed. This is a parser-only locale for DIA-EXT 1; runtime semantics are deferred.
