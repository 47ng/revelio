# revelio

![TravisCI Build](https://img.shields.io/travis/com/47ng/revelio.svg?label=Travis)
![CircleCI Build](https://img.shields.io/circleci/project/github/47ng/revelio/master.svg?label=CircleCI)

**DISCLAIMER**: PoC / WIP - [Feedback is welcome](https://github.com/47ng/revelio/issues) !

Revelio is a service that brings auditability and integrity checks for
open-source projects that live on the web.

It tries to answer the question:

> It's open-source, but how can I know what runs is really what's been built ?

By linking integrity (checking that built artifacts have not been tampered
with at any point in storage or transport) with auditability (linking to the
public build process that created the artifacts and the original sources),
Revelio automates transparency checks.

## Project Structure

- `revelio-server`: The service's REST API. Handles the following tasks:
  - creating tokens for publishers (v1)
  - publishing deployment manifests (PoC)
  - checking deployment manifests against a public URL (PoC)
- `revelio-reporter`: A CLI tool to use in your public CI to generate and
  send reports to the Revelio service. Has the following parts:

  - `hashdir`: Calculate cryptographic hashes for the artifacts
  - `niffler`: Sniffs the environment for auditability data
  - `snitch`: Reports back to the Revelio service

- `revelio-checker`: A CLI tool to check a domain and obtain information
  (build process URL, path to the sources with commit information).
  Has the following parts:
  - `accio`: Retrieve artifacts from a source
  - `hashdir`: Computes the same hashes as `revelio-reporter` to check

## CI Servers

- TravisCI
- CircleCI
- GitLab CI
- Jenkins
- Bitbucket Pipelines
- Azure Pipelines
