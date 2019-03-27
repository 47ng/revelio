# Logging Strategy

Logging events for the verification function depend on the execution context.

When using the CLI for verification, we want the output to be:

- Reactive (lines are output as the events happen)
- Formatted for human reading

On the other hand, the CLI can be used for machine interaction, where the
desired output will be a single JSON object representing the end result of the
operation. The verification library will also be used in a headless context,
where logs can be useful for post-execution analysis, statistics and other
metrics.

Out of this observation, there are two types of logging:

- Reactive, where events are logged as they occur, in a given format.
- Buffered, where events are stored and output in one go at the end of execution,
  in a given format.

For the sake of out-of-band analysis, Buffered logging must preserve the details
of every logging event that occured, especially in the case of error events.
In the case of successful execution, the full trace might be irrelevant if the
output summarises well enough the execution so that events that lead to it may
be inferred from it.

For example, a successful execution printing a single JSON object could look
like this:

```json
{
  "url": "https://example.com",
  "revelioFileUrl": "https://example.com/.well-known/revelio.json",
  "verified": true,
  "verifiedAt": "2019-04-12T05:23:53+00:00",
  "generatedAt": "2019-03-20T13:37:32.248626882+00:00",
  "buildContext": {
    "buildUrl": "https://travis-ci.com/franky47/penelopebuckley.com/jobs/186351150",
    "sourcesUrl": "https://github.com/franky47/penelopebuckley.com",
    "commitSha1": "bc2f507790f0f715aed06ab4f7a3b695c3600e38",
    "commitUrl": "https://github.com/franky47/penelopebuckley.com/commit/bc2f507790f0f715aed06ab4f7a3b695c3600e38",
    "compareUrl": "https://github.com/franky47/penelopebuckley.com/compare/afb92d8cf222...bc2f507790f0"
  },
  "artifacts": [
    {
      "url": "https://example.com/index.html",
      "verified": true,
      "declared_hash": "sha256:k/fOket/4sfRRKmhs5vt6PYa08npYpoCQR3OeImgo+8=",
      "verified_hash": "sha256:k/fOket/4sfRRKmhs5vt6PYa08npYpoCQR3OeImgo+8="
    }
  ]
}
```
