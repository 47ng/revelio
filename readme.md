# revelio

[![TravisCI Build](https://img.shields.io/travis/com/47ng/revelio.svg?label=Travis)](https://travis-ci.com/47ng/revelio)
[![CircleCI Build](https://img.shields.io/circleci/project/github/47ng/revelio/master.svg?label=CircleCI)](https://circleci.com/gh/47ng/revelio)

**DISCLAIMER**: PoC / WIP - [Feedback is welcome](https://github.com/47ng/revelio/issues) !

Revelio brings auditability and integrity checks for open-source projects
that live on the web.

It tries to answer the question:

> How do I link what's running to the public source code ?

By linking auditability (linking to the public build process that created the
artifacts and the original sources) with integrity (checking that built
artifacts have not been tampered with at any point in storage or transport),
Revelio automates transparency checks.

## The `revelio` CLI tool

`revelio` is a command-line tool that does the following things:

- When running in a public CI, generate a `revelio.json` file.
- Validate a URL that contains a valid `/.well-known/revelio.json` file.

For more details, see [the documentation for `revelio`](./revelio/readme.md).

## Supported CI Servers

- [x] TravisCI _(only supported on travis-ci.com)_
- [x] CircleCI _(GitHub and BitBucket support)_
- [ ] _GitLab CI_
- [ ] _Azure Pipelines_
- [ ] _Bitbucket Pipelines_
- [ ] _Jenkins_
