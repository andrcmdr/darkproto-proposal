# syntax=docker/dockerfile:1.4
FROM public.ecr.aws/amazonlinux/amazonlinux:2

RUN yum upgrade -y
RUN amazon-linux-extras enable epel
RUN yum clean -y metadata && yum install -y epel-release
RUN yum install -y gcc git openssl gnuplot
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain 1.63.0

WORKDIR /cipher-benchmarks

RUN <<EOT
#!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

git clone -b main https://github.com/andrcmdr/darkproto-proposal.git
cd ./darkproto-proposal
bash ./make.sh submodules update && bash ./make.sh submodules
source $HOME/.cargo/env && bash ./run.benchmarks.sh
EOT

COPY run.benchmarks.sh /cipher-benchmarks/darkproto-proposal/run.benchmarks.sh
CMD cd ./darkproto-proposal && source $HOME/.cargo/env && bash ./run.benchmarks.sh
