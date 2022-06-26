#!/bin/bash -x
set -e

source_path=$1
repository_url=$2
tag=${3:-latest} # Defaults to latest
accountid=$4
image_name=$5
region="eu-west-2"

# Build
(cd "$source_path" && DOCKER_BUILDKIT=1 docker build -t "$image_name" .)

# Login to ecr
aws --region "$region" ecr get-login-password | docker login --username AWS --password-stdin ${accountid}.dkr.ecr.$region.amazonaws.com

# Tag image
docker tag "$image_name" "$repository_url":"$tag"

# Push image
docker push "$repository_url":"$tag"
