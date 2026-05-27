# annexb-string-utilities — Seed

## Telos

Close the AnnexB legacy string-utility surface that consumer code still reaches for and that test262's `annexB/built-ins/` exercises: globals `escape` / `unescape` per ECMA-262 §B.2.1 and Date.prototype.toGMTString per §B.2.3.4. These are small, well-specified surfaces that fail cohesively as a unit when the global / prototype slot is absent — high yield, low LOC.

## Apparatus

- `exemplars/exemplars.txt` — 59 stratified-sample paths drawn from `annexB/built-ins/{escape,unescape,Date}`.
- `exemplars/run-exemplars.sh` — runner; reports aggregate pass/fail + per-target breakdown.
- Standing gates: build clean; `scripts/diff-prod/run-all.sh` parity; full diff against parallel agent surface (none — Temporal-heavy parallel agent doesn't touch AnnexB legacy globals per the recent commit history).

## Methodology

Pin-Art rung sequence per Doc 581:

- EXT 0 — founding + baseline measurement (rule 23).
- EXT 1+ — substrate moves: install missing globals/prototype slots per spec.
- Close when residual fails are not addressable by adding-a-slot (i.e., test depends on deeper substrate like exact toUTCString format).

## Carve-outs

- Not in scope: exact `toUTCString` format-string compliance (RFC 7231 day-name + 24h-clock layout). The Date.prototype.toUTCString implementation cruft already ships predates AnnexB-utilities locale; if exact-format tests fail, they belong to a separate Date-formatting locale, not here.
- Not in scope: `escape` / `unescape` round-trip with surrogate pairs (cruft's String storage uses Rust UTF-8 internally; UTF-16 round-trips through `encode_utf16` may lose lone surrogates).

## Composes with

- Doc 729 — resolver-instance pattern; AnnexB legacy globals are runtime-tier intrinsics installed alongside escape/unescape's spec siblings.
- Doc 737 — locale-as-coordinate; this locale is rung-1 substrate.
- Predictive-ruleset Rule 23 (founding-baseline-inspection) — applied at EXT 0.

## Resume protocol

Read this seed → trajectory.md tail → re-run `exemplars/run-exemplars.sh` to confirm current state. Next rung: address the dominant fail-category in the breakdown.
