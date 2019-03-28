# Continuous Integration

All continuous integration operations are done on [Travis CI](https://travis-ci.com/47ng/revelio).

## Common Operations

The following operations will run for every build:

- Run unit tests
- Build a Docker image for the CLI

## Rust Versions

By default, we'd like to support `stable` and `nightly` versions of Rust.

Deployments will only be made from the stable branch.

## Deployments

### Docker Image

The Docker image built will be tagged differently depending on the context:

- `master` for builds on the `master` branch
- `{tag}` for tagged releases (eg: `47ng/revelio:1.2.3`)
- `latest` for the latest tagged release (latest stable version)

No Docker image will be built for Pull Requests.

### GitHub Releases

When building a tagged release, the resulting binary is:

1. Built in Release mode
2. Stripped from its debug symbols (for minimal size)
3. Dogfed to generate its own `revelio.json`
4. Uploaded to GitHub releases

### Cargo Publish

When building a tagged release, the crate is published to Crates.io.

We use `cargo-release` to automate the release process, which usually includes
publishing the crate. In this case, we want Travis to be the sole publisher of
the crate, so we have to pass it the `skip-publish` option, we can do so through
the `disable-publish` configuration flag in `Cargo.toml`.

## Stages

Travis organises jobs into stages.

| Stage / Channel  | `stable`                                        | `nightly` |
| ---------------- | ----------------------------------------------- | --------- |
| install          | yes                                             | yes       |
| test             | yes                                             | yes       |
| deploy to Docker | `{tag}` + `latest` on tag<br>`master` on master | no        |
| deploy to GitHub | on tag only                                     | no        |
| deploy to Cargo  | on tag only                                     | no        |

## Failure

- Should a failed Docker build fail the build ?
- Should a failed upload to GitHub releases fail the build ?
- Should a failed crate publication fail the build ?

At the moment, Travis runs deployment after the build has been marked successful,
therefore we have no way to know if deployments have been successful.

Moreover, deployments should be consistent: if Docker succeeds, but the next
step (GitHub releases) fails, then we get an inconsistent outcome. How can we
ensure that all deployments will be effective only if all of them pass ?
