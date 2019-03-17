# Security Aspects

In its inital version, Revelio does not use any security, and only provides a
way to generate a digest of the built artifacts, as well as public information
about the build context.

The artifact hashes can then be verified by the client on their end.
If something does not match here, it can mean a few things:

1. The manifest is not in sync with the rest of the artifacts
2. Some files have been tampered with, by any party

The classic case of mismatch is a developer deploying from an environment
different than the normal CI, like their development machine, for a "quick fix",
or acting directly on the production server (eg: using a FTP client), as
[Hanlon's razor](https://en.wikipedia.org/wiki/Hanlon's_razor) states:

> _"Never attribute to malice what can be explained by incompetence."_

Revelio will detect both tampering that was caused by malice (3rd party
tampering with the artifacts) and incompetence (developer failing to follow
the convention of having the CI deploy to production).

> How do we know if something has been tampered with after the fact ?

There needs to be regular checks, hence the need for a
[Revelio Verification Network](./network.md).

## It's A Question Of Trust

Trust no one. Or rather, [trust, but verify](https://en.wikipedia.org/wiki/Trust,_but_verify).

As developers, there are a few places that we implicit trust. For example:

1. We trust Git not to modify our sources without our explicit action
2. We trust GitHub (or the repository host) not to tamper with our sources
3. We trust the CI service not to modify the sources before building them
4. We trust the deployment system to not tamper with our artifacts when deploying
5. We trust the hosting service not to tamper with our artifacts at rest
6. We trust the hosting service not to tamper with our artifacts on request
7. We trust the network to transport those artifacts untampered with
8. We trust our browser to receive and store those artifacts as received
9. We trust our browser to run the artifacts it has received

That's a lot of trust to verify. Revelio places itself between points 3 and 8.
Before and after that, you're on your own (but if you don't trust these systems,
you probably have bigger problems).

## Threat Models

**Case 1**: An attacker that can modify the sources on the static server can also
modify the `revelio.json` file and recompute the hashes to cover their tracks.
Therefore, we need a way to authenticate that the file has not changed since
its generation on the server.

Fortunately, because everything is public, it makes verifying this information
relatively simple. Comparing the hash of the file with the one calculated by the
CI server could be enough to detect tampering.

The attacker would have to compromise the CI server to trigger a malicious build
through the official channel to generate a "valid" `revelio.json`, which falls
outside the scope of this project.

**Case 2**: A Man-In-The-Middle can modify the traffic between the static server
and the client. The attacker would have to distinguish requests from Revelio's
verification system from normal resource request, to serve pristine files to
Revelio and compromised files to the legit client.

Let's assume for this exercise that the attacker has successfully put such a
system in place. It means Revelio would have to run the verification checks
against the actual client resources rather than requesting its own. It is
possible to do so for desktop apps that keep their resources on the filesystem.
For browser-based apps, there is no such way at the moment.

**Case 3**: An attacker does not have a way to compromise anything, but wishes
to poison the verification process from the Revelio verification service.

They could forge a `revelio.json` file, containing invalid hashes for their
target domain `https://example.com`, upload it to a server they control such as
`https://evil.com`, and point Revelio to `evil.com`.

To protect against that, Revelio will only verify artifact URLs that are
"siblings" of the `revelio.json` file.
