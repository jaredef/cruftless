# compartment-primitive — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. The Compartment primitive is a TC39 Stage 1 proposal (`new Compartment({globals, modules}).evaluate(src)`) that does not exist in any current engine's standard surface. Diff-prod measures byte-for-byte parity with Bun, and Bun does not implement Compartments. This locale is a forward-looking architectural feature (Doc 736 capability-passing property at the JS-API level), not a conformance-parity workstream, so diff-prod is not an applicable measurement instrument.
