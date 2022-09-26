#!/bin/bash
##!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

wget --verbose --report-speed=bits --append-output=wget.log --trust-server-names --content-disposition --tries=100 --continue --progress=bar --show-progress --timestamping --server-response --dns-timeout=60 --connect-timeout=60 --read-timeout=60 --waitretry=60 --prefer-family=IPv4 --retry-connrefused --user-agent='Mozilla/5.0 (X11; Linux x86_64; rv:105.0) Gecko/20100101 Firefox/105.0' --referer= --recursive --level=30 --no-parent --no-directories --no-host-directories --directory-prefix=./ --input-file='./download.list' "${@}" ;
# wget --verbose --spider --report-speed=bits --append-output=wget.log --trust-server-names --content-disposition --tries=100 --continue --progress=bar --show-progress --timestamping --server-response --dns-timeout=60 --connect-timeout=60 --read-timeout=60 --waitretry=60 --prefer-family=IPv4 --retry-connrefused --user-agent='Mozilla/5.0 (X11; Linux x86_64; rv:105.0) Gecko/20100101 Firefox/105.0' --referer= --recursive --level=30 --no-parent --no-directories --no-host-directories --directory-prefix=./ --input-file='./download.list' "${@}" ;
