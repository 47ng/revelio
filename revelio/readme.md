# revelio

Command-line tool to generate and verify auditability on the web.

## Usage

```shell
$ revelio --help
Generate and verify auditability on the web

SUB-COMMANDS:
  generate        Generate a JSON manifest
  verify          Verify a URL and print information

OPTIONS:
      --version   Prints version and dogfood information
      --help      Prints this help message
  -q, --quiet     Don't print anything while running
```

## Running in a CI environment

```yml
# .travis.yml

before_deploy:
  - revelio generate ./dist # Generates ./dist/.well-known/revelio.json

deploy:
  # Example deployment of static files using Surge.sh
  - surge -p ./dist -d https://example.com
```

## Validation

```shell
$ revelio validate https://example.com
🔎 Found https://example.com/.well-known/revelio.json
🔨 Build context:
Build         https://travis-ci.com/47ng/revelio/jobs/185315856
Sources       https://github.com/47ng/revelio
Commit URL    https://github.com/47ng/revelio/commit/670c8e2175cab7d5270d014fc65a36016c6353ed
Compare URL   https://github.com/47ng/revelio/compare
Commit SHA-1  670c8e2175cab7d5270d014fc65a36016c6353ed

🔬 Validation:

  ✅ https://example.com/index.html
  ✅ https://example.com/app.css
  ✅ https://example.com/app.js

✅ Validated https://example.com
```
