# npm-coverage reconnaissance (PM-EXT 13)

**Date**: 2026-05-21
**Registry**: registry.npmmirror.com (npm-protocol-compatible mirror;
TLS 1.3 reachable through cruftless's substrate per Doc 730 §XVI
Case-4 endpoint substitution)
**Method**: `pilots/rusty-js-pm/derived/examples/npm_coverage.rs` —
fetch `dist-tags.latest` for each candidate, then run
`resolve_closure(name, latest)`. No tarballs downloaded.
**Sample size**: 24 candidate package names (top-of-mind from npm
popularity rankings).
**Raw output**: `/media/jaredef/T7/cruftless-pm-recon/recon-2026-05-21.md`

## Headline result

- **16/24 (67%)** Reachable — closure resolves without hitting a range
- **8/24 (33%)** RangeAt — a transitive uses a semver range

**Critical refinement**: every single Reachable package has **closure
size 1**. That is — the 16 "exact-pin-reachable" packages are all
zero-dependency leaves. Every package in the sample that declares any
transitive dependency declares it with a range.

This sharpens the open question from PM-EXT 10. The hypothesis "the
exact-pin carve-out reaches into composed graphs" needs revision:

- **For leaf packages** (lodash, ms, chalk, uuid, etc.) — exact-pin
  installs work trivially.
- **For composed packages** — the current ecosystem uses ranges
  universally. PM-EXT 10's debug@4.3.4 → ms@2.1.2 case was a
  **historical exception** (an older version of debug that pinned
  exact); debug@latest declares `ms: "^2.1.3"`.

## Per-package outcomes

| Outcome | Packages |
|---|---|
| Reachable (size=1, leaf) | lodash, ms, chalk, uuid, commander, minimist, semver, mkdirp, dotenv, classnames, ansi-styles, is-number, color-name, tslib, yallist, lru-cache |
| RangeAt | debug `^2.1.3`, axios `^1.16.0`, express `^2.0.0`, yargs `^9.0.1`, glob `^10.2.2`, rimraf `^13.0.3`, fs-extra `^4.2.0`, prop-types `^1.4.0` |

## Latency

Per-package classification: 1.5–2.9 s wall time, dominated by two TLS
1.3 handshakes (root manifest + per-version manifest). Total recon run
on 24 packages: ~50 s under `nice -n 19`.

## What this means for the §VI carve-out

The first-cut exact-pin discipline is workable for **leaf libraries**
(which is a substantial slice of the npm corpus — utilities,
formatters, primitive data structures). It is **insufficient for
composed libraries** as the ecosystem currently publishes them.

The path forward has two branches, neither blocking PM-EXT 11 closure
gate:

**Branch A (substrate move)**: implement a minimal semver-range
resolver. The closure walker already has the right shape; the resolver
gains a `resolve_range(name, range)` that walks the per-package root
manifest's `versions` map and picks the highest matching version. This
opens the full ecosystem to cruftless's PM. ~150 LOC for caret/tilde/
basic-range support per the
[semver spec](https://semver.org/) v2.0 §11.

**Branch B (scope decision)**: ship as-is with a documented "leaves
only" constraint, and treat any range as a Case-2 ecosystem deviation
the user must resolve manually (via a `cruftless-pin-overrides.json`
that pins the transitive themselves). This preserves Doc 732 §VI as
written but caps the ecosystem reach at leaves.

Branch A is the natural next substrate move; Branch B is the
ergonomically simpler hold-pattern. The decision is the keeper's, not
the PM workstream's.

## Doc 730 §XVI classification

Each Reachable outcome is **Case-3 (both-diverge → compositional
success)**: cruftless's PM successfully composes against an ecosystem
package whose transitive surface matches the carve-out.

Each RangeAt outcome is **Case-2 (ecosystem deviation)**: the package
declares a range, which the carve-out explicitly defers. This is
not a substrate bug; the resolver correctly refuses to silently pick
a version. The lift would be a substrate move (Branch A above).

## Open scope at PM-EXT 13 close

1. Decide between Branch A (semver-range resolver) and Branch B
   (leaves-only constraint, documented).
2. Optionally probe a larger sample (top-100 or top-500) to confirm
   the leaf-vs-composed dichotomy holds at scale. The harness scales
   linearly; ~200 s for 100 packages at the observed latency.
3. Conflict detection in the closure walker (still queued; needed
   regardless of which branch).

---

*The PM-EXT 11 closure gate is unaffected: a user with package.json
declaring exact-pinned leaf deps gets a working install + runtime
loop. PM-EXT 13 surfaces the next scope-decision boundary.*
