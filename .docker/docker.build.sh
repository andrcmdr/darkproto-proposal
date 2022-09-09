#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

### For cleaning up the Overlay FS (/var/lib/docker/overlay2)
if [[ "$1" == "clean" ]]; then
    docker image prune --all;
    docker system prune --all --volumes;
    docker volume prune;
    docker builder prune --all;
    DOCKER_BUILDKIT=1 docker buildx prune --all;
fi

docker build --no-cache -f ./enclave_base.dockerfile -t "enclave_base" ./

DOCKER_BUILDKIT=1 docker build --no-cache -f ./cipher_benchmarks.dockerfile -t "cipher_benchmarks" ./

DOCKER_BUILDKIT=1 docker build --no-cache -f ./darkproto_app.dockerfile -t "darkproto_app" ./
