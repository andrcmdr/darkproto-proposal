#!/bin/bash
##!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

# Docs:
# https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
# https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html
# https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html

curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip -u awscliv2.zip
sudo ./aws/install --bin-dir /usr/local/bin --install-dir /usr/local/aws-cli --update
which aws
ls -l /usr/local/bin/aws
aws --version
aws configure
aws configure set region eu-central-1
aws configure set region eu-central-1 --profile nitro
aws configure set output json
aws configure set output json --profile nitro
aws configure list
# AWS CLI files are stored in:
ls -l $HOME/.aws/config
cat $HOME/.aws/config
ls -l $HOME/.aws/credentials
cat $HOME/.aws/credentials
