#!/usr/bin/env bash

set -eo pipefail

echo "Initializing the environment..."
AWS_REGION=$2
DOCKER_IMAGE_BASENAME=$3
DOCKER_IMAGE_TAG=$4
DOCKER_REGISTRY_REPO=$5

DOCKERFILES=$(ls Dockerfile*)

echo "Running container.sh..."

function run_docker_build() {
  echo "Building the images..."
  for dockerfile in $DOCKERFILES; do
    if [ $dockerfile == "Dockerfile" ]; then
      docker build --progress=plain --rm --file $dockerfile --tag $DOCKER_REGISTRY_REPO/$DOCKER_IMAGE_BASENAME:$DOCKER_IMAGE_TAG .
    else
      DOCKER_IMAGE_NAME=$DOCKER_IMAGE_BASENAME"-"$(echo $dockerfile | awk -F"." '{ print $2 }')
      docker build --progress=plain --rm --file $dockerfile --tag $DOCKER_REGISTRY_REPO/$DOCKER_IMAGE_NAME:$DOCKER_IMAGE_TAG .
    fi
  done
}

function run_docker_publish() {
  echo "Logging in to ECR..."
  aws ecr get-login-password --region $AWS_REGION | docker login --username AWS --password-stdin $DOCKER_REGISTRY_REPO

  echo "Pushing the images..."
  for dockerfile in $DOCKERFILES; do
    if [ $dockerfile == "Dockerfile" ]; then
      docker push $DOCKER_REGISTRY_REPO/$DOCKER_IMAGE_BASENAME:$DOCKER_IMAGE_TAG
    else
      DOCKER_IMAGE_NAME=$DOCKER_IMAGE_BASENAME"-"$(echo $dockerfile | awk -F"." '{ print $2 }')
      docker push $DOCKER_REGISTRY_REPO/$DOCKER_IMAGE_NAME:$DOCKER_IMAGE_TAG
    fi
  done
}

case $1 in
  --build)
    run_docker_build
    ;;

  --publish)
    run_docker_publish
    ;;

  *)
    echo "Usage: $0 {--build|--publish}"
    exit 0
esac
