---
proposal_slug: 2026-05-29T003300-caacp-bash-wrapper
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-29T00:33:00Z
covers_commits:
  - 6c0409b9299c7737134c5c81ba5f1cc7a2b0dc3c
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out.

Keeper Rung-2 authorization: Telegram 10243 ("Yes continue") affirming the bash-wrapper question the helmsman flagged at the close of CAACP Stage A landing. The substrate commit at 6c0409b9 directly executes the wrapper draft.

**Apparatus-tier verification**:

1. **No substrate impact**: gates_pre and gates_post identical.

2. **Smoke-test verified end-to-end** in degraded mode: `send` → `inbox` listing → `ack` → `outbox` listing showing ACKNOWLEDGED state. Artifacts written to expected paths; symlinks resolved; sync-failure logs created.

3. **Conformant with CAACP §VI**: POST payload schemas match the §VI.1 endpoint surface; apparatus convention per §VI.2 (content_sha + canonical artifact + symlink + POST + patch message_id) implemented as specified.

4. **Degraded-mode coherent**: with `CAACP_TOKEN` unset (Stage A state), wrapper operates entirely on on-disk artifacts; endpoint sync skipped + intent logged. `message_id` reconciliation deferred until Stage B activation. Wrapper's behavior matches the CAACP §VI.3 fallback discipline.

5. **No replacement of legacy channels**: wrapper writes only to `apparatus/caacp/*`. Legacy channels untouched per the keeper's "Keep legacy channels" directive from Telegram 10241.

6. **Rule discipline honored**: Rule 4 single coordinated rung; Rule 15 chapter-close-inspect satisfied (smoke test covers all four subcommands).

**Apparatus-meta concerns considered**:

- Wrapper is the optional Stage A tooling component the helmsman flagged at the close of Stage A landing. Stage B activation requires no further wrapper changes; the endpoint sync paths activate immediately on token provision.
- Stage 2 mechanical-veto coverage: this proposal+decision pair covers the substrate commit's SHA.

**APPROVED for push.**

Archive to `apparatus/proposals/archived/2026-05-29T003300-caacp-bash-wrapper/` after push lands.
