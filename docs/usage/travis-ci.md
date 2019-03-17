# Using Revelio on Travis CI

1. Install Revelio and run in in the `before_deploy` phase
2. Add `skip-cleanup: true` to the `deploy` phase, to avoid dropping the generated file

```yml
# .travis.yml

before_deploy:
  - curl -sSL https://raw.githubusercontent.com/47ng/revelio/master/scripts/get.sh | bash
  - revelio generate -p YOUR_DEPLOYMENT_DIR -u https://example.com
  - cat YOUR_DEPLOYMENT_DIR/.well-known/revelio.json

deploy:
  skip-cleanup: true
```
