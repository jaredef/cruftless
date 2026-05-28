# Deprecated docs

Apparatus-tier articulations and engagement docs that have been superseded by reformulations but are preserved for historical reference and audit-trail purposes.

Per keeper directive Telegram 10200, when a load-bearing apparatus articulation is reformulated and promoted to `apparatus/docs/`, the prior version moves here rather than being deleted. The deprecation preserves the read history; an arbiter or future-keeper inspecting why a discipline shifted shape can read the prior articulation as it stood when it was load-bearing.

## Deprecation discipline

A doc moved into `docs/deprecated/` carries a deprecation header at the top recording:

- The date the doc was deprecated.
- The keeper directive (Telegram message id) authorizing the deprecation.
- The successor doc (path + brief description of what changed).
- The reason for deprecation (in one or two sentences).

The body of a deprecated doc is otherwise preserved unmodified. Future readers see the historical articulation as it stood; the header tells them not to use it as current discipline.

## Why preserve rather than delete

Deletion loses the trajectory binding the deletions-ledger discipline was created to preserve. For apparatus-tier articulations, the binding is even more load-bearing — a future apparatus revision may want to recover a frame the deprecation discarded. The deprecated dir is the archive that lets the apparatus reformulate without losing access to prior shape.

This dir is read-only from the apparatus's operational perspective. No resolver consults `docs/deprecated/` as input to a substrate move; the keeper consults it when authoring the next reformulation.
