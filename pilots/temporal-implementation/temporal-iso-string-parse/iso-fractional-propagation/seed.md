---
name: iso-fractional-propagation
description: Third sub-rung of temporal-iso-string-parse. Per §13.27 spec, propagate fractional H/M/S downward into smaller units rather than rejecting.
type: project
---

# iso-fractional-propagation — Seed

## Leaf shared-substrate sub-locale under `pilots/temporal-implementation/temporal-iso-string-parse/`.

Per IDP.1 standing residual: my parser stored fractional in the unit's slot then caller's integer-validate rejected. Spec mandates downward cascade: PT1.5H → 1H 30M, PT0.5M → 30S, PT1.5S → 1.5S = 1S 500ms.

## Telos

After `parse_iso_duration` completes, if any slot carries fractional, propagate:
- Slot 4 (hours): `frac*60 → minutes; frac*60 → seconds; sub_sec*1e9 → ms/μs/ns`.
- Slot 5 (minutes): `frac*60 → seconds; sub_sec → ms/μs/ns`.
- Slot 6 (seconds): `frac*1e9 → ms/μs/ns`.
- Slot 0-3 (years/months/weeks/days fractional): REJECT (spec forbids without relativeTo).
- Slot 7-9 (ms/μs/ns fractional): naturally smallest, no propagation needed but accepted.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal::parse_iso_duration` — extended post-loop with cascading propagation logic (~40 LOC).
- **Exemplar suite**: none (measured via Duration sibling rung deltas).

## Status

IFP-EXT 1 LANDED 2026-05-26. duration-static +4 yield (31→35); other Duration rungs unchanged. Probes confirm PT1.5H = 1H 30M, PT0.5M = 30S, PT1.5S = 1S 500ms.
