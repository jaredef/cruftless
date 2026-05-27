# array-species-create-discipline — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| array-methods | PASS | Core array methods pass for plain Array inputs; species path is bypassed entirely |
| symbol-species | FAIL | Directly exercises Symbol.species lookup on Array constructor -- the mechanism ASCD must implement |
| prototype-chain | PASS | Prototype chain traversal works; species lookup adds constructor-then-Symbol.species chain walk |
| class-inheritance | PASS | Subclass construction works; ArraySpeciesCreate needs IsConstructor + Construct on the species result |

The FAIL on symbol-species is a direct confirmation of this locale's gap: Array.prototype methods bypass ArraySpeciesCreate entirely, allocating plain Arrays instead of consulting `this.constructor[Symbol.species]`. The PASS on array-methods, prototype-chain, and class-inheritance shows the surrounding substrate (array operations, prototype walks, subclass construction) is healthy. ASCD's work is to wire the existing healthy pieces together through the ArraySpeciesCreate algorithm at each method's output-allocation site.
