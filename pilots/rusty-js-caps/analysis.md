# rusty-js-caps — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. This locale targets host/infrastructure APIs outside the ECMA-262 behavioral surface. The capability-passing runtime (Doc 736) controls authority boundaries for module load, execution, and runtime surfaces. Its correctness is measured by the 9/9 synthetic-adversary probe suite under --sealed mode, not by diff-prod behavioral parity.
