# rusty-js-pm — Diff-Prod Analysis

No diff-prod fixtures directly exercise this locale's scope. This locale targets host/infrastructure APIs outside the ECMA-262 behavioral surface. The package manager (resolver-instance #0) handles specifier resolution, registry fetch, lockfile, and node_modules layout. Its correctness is measured by end-to-end install probes (require('lodash') after cruftless install), not by diff-prod.
