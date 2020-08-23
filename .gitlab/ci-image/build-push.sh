#!/usr/bin/env bash

IMAGE_NAME="registry.gitlab.com/remipassmoilesel/notes:0.1"

docker build . -t ${IMAGE_NAME}
docker push ${IMAGE_NAME}