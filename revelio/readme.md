# revelio

Command-line tool to generate and verify auditability on the web.

## Usage

```shell
$ revelio help
revelio 0.1.2
Francois Best <contact@francoisbest.com>
Generate and verify auditability on the web

USAGE:
    revelio <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate    Generate a revelio.json manifest file to be deployed with your artifacts
    help        Prints this message or the help of the given subcommand(s)
    verify      Verify the integrity of artifacts on the given URL and print build context
```

## Using Docker

A [Docker image](../Dockerfile) is built and available at [`47ng/revelio`](https://hub.docker.com/47ng/revelio), with the following tags:

- `master` follows the `master` branch on GitHub
- `{tag}` points to tagged releases (eg: `47ng/revelio:0.1.2`)
- `latest` points to the latest tagged release (latest stable version)

Example:

```shell
$ docker run --rm -it 47ng/revelio verify https://example.com
```

## Subcommands

### `generate`

The `generate` subcommand is to be run in a CI server environment.

It will sniff out build context information (URLs to the sources, the build
instance and Git information), and calculate checksum hashes for the built
artifacts to be deployed.

For this reason, `revelio generate` should run in the pre-deployment phase,
after the final artifacts have been built and just before they are deployed.

The command requires a path to the artifacts directory, and a public URL that
will be the base URL for all the artifacts.

### `verify`

The only argument is a URL that contains a public `revelio.json` file.

Revelio will print the build context, revealing links to the sources and
public CI service, then attempt to verify the integrity of the listed
artifacts:

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
