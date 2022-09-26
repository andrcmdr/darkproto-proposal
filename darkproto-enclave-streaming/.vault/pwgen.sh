#!/bin/bash
##!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

echo
pwgen -1cnys 256 | tr -d "\n" | tee ./pwd
echo -e "\n"
pwgen -1cnys 256 | tr -d "\n" | tee ./seed
echo -e "\n"
pwgen -1cnys 256 | tr -d "\n" | tee ./re_enc_pwd
echo -e "\n"
pwgen -1cnys 256 | tr -d "\n" | tee ./re_enc_seed
echo -e "\n"
