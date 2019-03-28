#!/usr/bin/env bash

cd $(dirname "${BASH_SOURCE[0]}")/..

# Build
docker build -t 47ng/revelio:latest -t 47ng/revelio:${TRAVIS_TAG} .

# Login & Push
echo $DOCKER_PASSWORD | docker login -u $DOCKER_USERNAME --password-stdin
docker push 47ng/revelio:latest
docker push 47ng/revelio:${TRAVIS_TAG}
