#!/bin/bash
##!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

if [[ "$1" == "app" ]]; then
    nitro-cli build-enclave --docker-uri darkproto_app:latest --output-file darkproto_app.eif 2>&1 | tee darkproto_app.eif.pcr; \
    nitro-cli describe-eif --eif-path ./darkproto_app.eif 2>&1 | tee darkproto_app.eif.desc;

elif [[ "$1" == "light" || "$1" == "" ]]; then
    nitro-cli build-enclave --docker-uri darkproto_app_light:latest --output-file darkproto_app_light.eif 2>&1 | tee darkproto_app_light.eif.pcr; \
    nitro-cli describe-eif --eif-path ./darkproto_app_light.eif 2>&1 | tee darkproto_app_light.eif.desc;

elif [[ "$1" == "bench" ]]; then
    nitro-cli build-enclave --docker-uri cipher_benchmarks:latest --output-file cipher_benchmarks.eif 2>&1 | tee cipher_benchmarks.eif.pcr; \
    nitro-cli describe-eif --eif-path ./cipher_benchmarks.eif 2>&1 | tee cipher_benchmarks.eif.desc;

fi
