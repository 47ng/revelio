# revelio

[![Travis CI Build](https://img.shields.io/travis/com/47ng/revelio.svg?label=Travis%20CI)](https://travis-ci.com/47ng/revelio)
[![CircleCI Build](https://img.shields.io/circleci/project/github/47ng/revelio/master.svg?label=CircleCI)](https://circleci.com/gh/47ng/revelio)
[![MIT License](https://img.shields.io/github/license/47ng/revelio.svg?color=blue)](./LICENSE)

**DISCLAIMER**: PoC / WIP - [Feedback is welcome](https://github.com/47ng/revelio/issues) !

Revelio brings auditability and integrity checks for open-source projects
that live on the web.

It tries to answer the question:

> What is the original source code of what's running on my machine, and how was it built ?

Revelio automates transparency checks by linking:

- auditability, with URLs to the public build process and the original sources
- integrity, by checking that built artifacts have not been tampered with at any point in storage or transport

## How it works

In your public CI (see the [list of supported services](#usage)), Revelio will
create a manifest file containing:

- Build context information (URL to the build, the sources, SHA1 of the commit...)
- Artifact map with SHA256 checksum hashes

```json
{
  "version": 1,
  "datetime": "2019-03-20T13:37:32.248626882+00:00",
  "context": {
    "buildUrl": "https://travis-ci.org/acme/example.com/jobs/123456789",
    "sourcesUrl": "https://github.com/acme/example.com",
    "commitSha1": "4cc994bd49fd2bf827bfbf476488e963e9d565e8",
    "commitUrl": "https://github.com/acme/example.com/commit/4cc994bd49fd2bf827bfbf476488e963e9d565e8",
    "compareUrl": "https://github.com/acme/example.com/compare/e68714654da3...4cc994bd49fd"
  },
  "artifacts": {
    "https://example.com/404.html": "sha256:3mAC+oR66YjsNoV33fDK6AbN72N2okGtk9PjdIuRtsE=",
    "https://example.com/app.css": "sha256:R1+IjX4f2u7GRE/XQQfgVx+YTeGqReI4qUGqKDkHA9w=",
    "https://example.com/app.js": "sha256:0vGVpfjqMywtnj/hLODFi5ek6hK7aBQ+n2JZ9F/3Hx0=",
    "https://example.com/index.html": "sha256:03cfGvf3z4VUqCzuaFabxbA396AT63mSjwJwOlKsaHQ=",
    "https://example.com/robots.txt": "sha256:wBFPdm0K88T1Cdgftal2BpreVKzmqjb0FiGbZi9/cOg=",
    "https://example.com/sitemap.xml": "sha256:RMQZ2sZN8ef/vghv58Ccoz4N3s/Oq4Hb2mrw81WK/U0="
  }
}
```

You then deploy that `revelio.json` file along with your artifacts.
The recommended path is `/.well-known/revelio.json`.

Later on, you can use the Revelio CLI to verify the integrity and retrieve the
build context information:

```shell
$ revelio verify https://example.com
🔎  Found https://example.com/.well-known/revelio.json
🔨  Build context:

     Build         https://travis-ci.org/acme/example.com/jobs/123456789
     Sources       https://github.com/acme/example.com
     Commit URL    https://github.com/acme/example.com/commit/4cc994bd49fd2bf827bfbf476488e963e9d565e8
     Compare URL   https://github.com/acme/example.com/compare/e68714654da3...4cc994bd49fd
     Commit SHA-1  4cc994bd49fd2bf827bfbf476488e963e9d565e8

🔬  Integrity:

  ✅  https://example.com/404.html
  ✅  https://example.com/app.css
  ✅  https://example.com/app.js
  ✅  https://example.com/index.html
  ✅  https://example.com/robots.txt
  ✅  https://example.com/sitemap.xml

✅  Verified https://example.com/
```

## Usage

- [x] [Setup for Travis CI](./docs/usage/travis-ci.md)
- [x] [Setup for CircleCI](./docs/usage/circle-ci.md)
- [ ] _Todo: GitLab CI_
- [ ] _Todo: Azure Pipelines_
- [ ] _Todo: Bitbucket Pipelines_
- [ ] _Todo: Jenkins_

## The `revelio` CLI tool

`revelio` is a command-line tool that does the following things:

- When running in a public CI, generate a `revelio.json` file.
- Verify a URL that contains a public `/.well-known/revelio.json` file.

For more details, see the [`revelio` CLI documentation](./src/bin/cli/readme.md).

## FAQ

### How do I use it for private repositories / private CI ?

The core idea behind this project is to bring trust through transparency.
Therefore, it will only ever work with public repositories and public CI services.

## License

[The MIT License (MIT)](./LICENSE)

Copyright (c) 2019 - present, [François Best](https://francoisbest.com)
