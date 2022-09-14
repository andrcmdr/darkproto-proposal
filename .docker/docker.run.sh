#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

if [[ "$1" == "bench" ]]; then
    docker run cipher_benchmarks && docker logs --follow cipher_benchmarks
    # docker run cipher_benchmarks && docker logs --follow cipher_benchmarks 2>&1 | tee -a ./cipher_benchmarks.log & disown
    # docker run cipher_benchmarks && docker logs --follow cipher_benchmarks >> ./cipher_benchmarks.log 2>&1 & disown
    # docker run -ti cipher_benchmarks bash
    # docker exec -ti cipher_benchmarks bash
fi

if [[ "$1" == "app" ]]; then
    docker run darkproto_app && docker logs --follow darkproto_app
    # docker run darkproto_app && docker logs --follow darkproto_app 2>&1 | tee -a ./darkproto_app.log & disown
    # docker run darkproto_app && docker logs --follow darkproto_app >> ./darkproto_app.log 2>&1 & disown
    # docker run -ti darkproto_app bash
    # docker exec -ti darkproto_app bash
fi
