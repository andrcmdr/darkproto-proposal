#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

docker build -f ./enclave_base.dockerfile -t "enclave_base" ./

DOCKER_BUILDKIT=1 docker build -f ./cipher_benchmarks.dockerfile -t "cipher_benchmarks" ./

DOCKER_BUILDKIT=1 docker build -f ./darkproto_app.dockerfile -t "darkproto_app" ./

