#!/bin/bash
##!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

if [[ "$1" == "app" ]]; then
    nitro-cli run-enclave --cpu-count 2 --memory 1024 --eif-path ./darkproto_app.eif --debug-mode --enclave-cid 16; \
    ENCLAVE_ID=$(nitro-cli describe-enclaves | jq -r ".[0].EnclaveID"); \
    nitro-cli console --enclave-id "${ENCLAVE_ID}";

elif [[ "$1" == "light" || "$1" == "" ]]; then
    nitro-cli run-enclave --cpu-count 2 --memory 1024 --eif-path ./darkproto_app_light.eif --debug-mode --enclave-cid 16; \
    ENCLAVE_ID=$(nitro-cli describe-enclaves | jq -r ".[0].EnclaveID"); \
    nitro-cli console --enclave-id "${ENCLAVE_ID}";

elif [[ "$1" == "bench" ]]; then
    nitro-cli run-enclave --cpu-count 4 --memory 20000 --eif-path ./cipher_benchmarks.eif --debug-mode --enclave-cid 16; \
    ENCLAVE_ID=$(nitro-cli describe-enclaves | jq -r ".[0].EnclaveID"); \
    nitro-cli console --enclave-id "${ENCLAVE_ID}";

elif [[ "$1" == "term" ]]; then
    ENCLAVE_ID=$(nitro-cli describe-enclaves | jq -r ".[0].EnclaveID"); \
    nitro-cli terminate-enclave --enclave-id "${ENCLAVE_ID}";

fi
