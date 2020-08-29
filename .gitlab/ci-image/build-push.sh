#!/usr/bin/env bash

IMAGE_NAME="registry.gitlab.com/remipassmoilesel/notes:0.01"

docker build . -t ${IMAGE_NAME}
docker push ${IMAGE_NAME}