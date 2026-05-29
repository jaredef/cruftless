---
proposal_slug: 2026-05-29T020000-caacp-rung-4
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-29T02:00:00Z
covers_commits:
  - 598c5523f2ab2176e3af34a7896051ed092cef57
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out.

Keeper Rung-2 authorization: Telegram 10252–10257. The substrate commit at 598c5523 executes Rung 4, the final rung of the four-rung CAACP shared-sidecar + per-instance-token deployment.

**Apparatus-tier verification**:

1. **No substrate impact**: gates unchanged.

2. **Singleton tokens provisioned**: four roles registered via admin token; tokens persisted to env.local (gitignored); `caacp-<role>-<uuid>` shape verified.

3. **Skill-step integration**: each appointed-role load skill now has Step 1c sidecar-registration; resolvers will execute on session entry.

4. **Sidecar wrapper available**: caacp-sidecar.sh provides bash interface for resolvers without TS/HTTP-client capability.

5. **CAACP doc status updated** with the sidecar+per-instance-token extension summary; cross-references to apparatus surfaces complete.

6. **Rule discipline honored**: Rule 4 single coordinated rung; Rule 15 four-rung deployment chapter-close — apparatus inventory verified end-to-end.

**Note on htx-engine framework usage**: the helmsman interpreted the keeper's "use the engine" directive pragmatically (bun primitives, not the htx-engine framework dependency). Acceptable for v1 sidecar given the single-purpose local-only HTTP API; if the keeper wants a refactor to vendored htx-engine for operational consistency with jaredfoy.com, a follow-up apparatus pass can land that.

**Apparatus-meta concerns considered**:

- Stage 2 mechanical-veto coverage: this proposal+decision pair covers the substrate commit's SHA.
- Stage C (Telegram demotion) becomes the next CAACP milestone after the cybernetic loop has run for some engagement cycles.

**APPROVED for push.**

Archive to `apparatus/proposals/archived/2026-05-29T020000-caacp-rung-4/` after push lands.
