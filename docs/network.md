# Revelio Verification Network

The `revelio` CLI tool can send a network request to a home server, for the
purpose of asynchronous verification.

Such a network request shall only contain:

- The URL to the publicly accessible `revelio.json` manifest
- The hash of the file

Example:

```
HTTP/1.1 POST https://revelio.47ng.com/verify
Content-Type: application/json

{
  "url": "https://rclement.github.io/password-police/.well-known/revelio.json",
  "hash": "sha256:DGqT+E1YGtRB3s41tS3FtFydd7XNiTbLxDIPtEhYVhw="
}
```

The server shall respond with a `202 Accepted` response with an empty body.
Verification will be asynchronous, as deploying the generated files can take
an arbitrary amount of time, unbeknownst to the service.

## Verification Process

Once the server receives a verification request, it:

1. Stores the URL in a queue, using the following algorithm:
   - If the URL is not present in the queue, add it to the end of the queue,
     with the following properties:
     ```
     {
       "retry": 0,
       "added_at": 1552718527
     }
     ```
   - Otherwise, don't do anything (no _bumping_).
2. Returns either:
   - `202 Accepted` with an empty body
   - `503 Service Unavailable` if an error occurred while adding to the queue,
     with the error message (eg: `{ "error": "Verification queue is full" }`).<br/>
     The client will then show a warning, but keep on going, as this is
     not a crititcal part of the system.

Every hour, the queue shall attempt to be emptied using the following algorithm:

```
for each item in the queue:
  if the added_at timestamp is not older than one hour:
    // submission is too recent to be verified yet.
    continue to the next item
  if the item is present in the retried list:
    continue to the next item

  pop the item
  if the item's retry count is greater than 5:
    // too many retries
    log event
    continue to the next item

  send a GET request to the URL in the item

  if the request does not succeed:
    increment the retry count
    push the item back at the end of the queue (without modifying added_at)
    add the item to the retried list
    continue to the next item

  // Validation process starts
  log the verification event starting, with the hash of the contents of the file as an ID

  for each url in the context section:
    send a GET request to the URL in the item
      .then   mark as success
      .catch  mark as failure
  for each artifact in the artifacts section:
    send a GET request to the key URL
      .then   verify hash against manifest
      .then   mark as success
      .catch  mark as failure

  if no failure occurred:
    log verification success event (including the contents of `revelio.json`)
    continue to the next item

  // Failed to verify
  log verification failure event (detailed report)
  attempt to find the commiter's email from the context
  if found:
    send an email with the failure report
```

- Pop the URL at the head of the queue, test if it resolves.
  - If it does, keep going with verification.
  - If it doesn't, push it back to the end of the queue with an incremented
    `retry` count. If the count reaches over a given threshold, drop the URL.

## Privacy

For the sakes of transparency, verification logs shall be public and accessible
from a url such as https://revelio.47ng.com/verify/logs

Telemetry can be disabled with the `--no-telemetry` flag:

```shell
$ revelio generate --no-telemetry --path ./dist --base-url https://example.com
```

When using this flag, `revelio generate` will not send anything to the outside.
