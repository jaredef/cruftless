# array-literal-elision-length — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| array-methods | PASS | Array methods operate on arrays post-construction; elision-length is a construction-time concern |
| destructuring | PASS | Destructuring of arrays works; elision arrays as sources would expose length bugs but typical fixtures use populated arrays |

No diff-prod fixture directly exercises array literal elision (`[,,,].length`). The locale targets the bytecode/runtime seam where Op::NewArray ignores the parser's length hint for elision-only arrays. Since diff-prod fixtures use populated array literals (not elision-only), the PASS on array-methods and destructuring does not cover this gap. This is a narrow construction-time bug with no footprint in the current diff-prod surface.
