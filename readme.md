# revelio

[![Travis CI Build](https://img.shields.io/travis/com/47ng/revelio.svg?label=Travis%20CI)](https://travis-ci.com/47ng/revelio)
[![CircleCI Build](https://img.shields.io/circleci/project/github/47ng/revelio/master.svg?label=CircleCI)](https://circleci.com/gh/47ng/revelio)
[![MIT License](https://img.shields.io/github/license/47ng/revelio.svg?color=blue)](./LICENSE)

**DISCLAIMER**: PoC / WIP - [Feedback is welcome](https://github.com/47ng/revelio/issues) !

Revelio brings auditability and integrity checks for open-source projects
that live on the web.

It tries to answer the question:

> What is the original source code of what's running on my machine, and how was it built ?

By linking auditability (linking to the public build process that created the
artifacts and the original sources) with integrity (checking that built
artifacts have not been tampered with at any point in storage or transport),
Revelio automates transparency checks.

## Usage

- [x] [Setup for Travis CI](./docs/usage/travis-ci.md)
- [x] [Setup for CircleCI](./docs/usage/circle-ci.md)
- [ ] _GitLab CI_
- [ ] _Azure Pipelines_
- [ ] _Bitbucket Pipelines_
- [ ] _Jenkins_

## The `revelio` CLI tool

`revelio` is a command-line tool that does the following things:

- When running in a public CI, generate a `revelio.json` file.
- Verify a URL that contains a public `/.well-known/revelio.json` file.

For more details, see [the documentation for `revelio`](./revelio/readme.md).

## FAQ

### How do I use it for private repositories ?

The core idea behind this project is to bring trust through transparency.
Therefore, it will only ever work with public repositories and public CI services.

## License

[The MIT License (MIT)](./LICENSE)

Copyright (c) 2019 - present, [François Best](https://francoisbest.com)
