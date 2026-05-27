# error-instance-property-descriptors — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| error-types | PASS | Error type construction works; the gap is descriptor attributes (enumerable), not construction |
| error-throws | PASS | Error throwing works; message/cause properties are present but with wrong descriptors |
| error-aggregate-cause | FAIL | AggregateError's `errors` property should also be non-enumerable; same descriptor discipline gap |

The PASS on error-types and error-throws confirms that Error instances are constructed and thrown correctly with the right message/cause values. The gap is purely in property descriptor attributes: `message`, `cause`, and `stack` are installed as enumerable (default `{w:t, e:t, c:t}`) instead of non-enumerable (`{w:t, e:f, c:t}`). Diff-prod fixtures that only check error message content pass because the values are correct; the descriptor shape is not probed. The FAIL on error-aggregate-cause overlaps with the AggregateError `errors` property descriptor fix this locale includes in its scope.
