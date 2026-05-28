# Watcher Working Directory

Working surface for the watcher service-tier resolver per `apparatus/docs/engagement-doc-watcher.md` + `apparatus/docs/service-tier-and-statefulness-protocol.md` §III. Active when the keeper appoints a watcher session via `/watcher-load`.

## Layout

```
apparatus/watcher/
├── notifications/         # pending freshness notifications
│   └── closed/            # remediated notifications (archived with refresh commit + timestamp)
```

## Notification authorship

When the watcher detects a stateful surface's drift exceeding the per-surface threshold in `apparatus/docs/service-tier-and-statefulness-protocol.md` §V.2, it authors a notification at `apparatus/watcher/notifications/YYYY-MM-DDTHHMMSS-<surface>.md` per the format in §VI.4:

```yaml
---
surface: <surface-name>
last_known_fresh_value: <value>
currently_observed_value: <value>
staleness_threshold: <threshold>
staleness_observed: <numeric drift>
detected_at: <ISO timestamp>
responsible_role: helmsman | watcher
---

## Observation
<one paragraph: what surface, what changed, when>

## Remediation
<one paragraph: what refresh action closes the staleness>

## Cited at
<paths in apparatus docs that cite the stale value>
```

## Notification archival

When the responsible role (typically the helmsman) completes the refresh, the notification is moved to `apparatus/watcher/notifications/closed/<same-slug>.md` with a footer recording the refresh-commit SHA and timestamp:

```
---
**Closed**: <ISO timestamp>; refresh-commit: <sha>; remediated-by: helmsman | watcher | keeper
```

The closed archive is the apparatus's audit trail of detected freshness drift + the substrate moves that closed it.

## Activation status

This directory is created at Stage 4 of the operational protocol deployment per keeper directive Telegram 10219. The watcher session is not yet appointed; the directory waits.
