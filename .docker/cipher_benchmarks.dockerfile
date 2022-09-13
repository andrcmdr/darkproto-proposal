# syntax=docker/dockerfile:1.4
FROM public.ecr.aws/amazonlinux/amazonlinux:2

RUN yum upgrade -y
RUN amazon-linux-extras enable epel
RUN yum clean -y metadata && yum install -y epel-release
RUN yum install -y gcc git openssl gnuplot patch

ENV CARGO_HOME="$HOME/rust" RUSTUP_HOME="$HOME/rustup" PATH="$PATH:$HOME/rust/bin"
RUN curl -fsSL https://sh.rustup.rs | bash -is -- -y --verbose --no-modify-path --default-toolchain stable --profile minimal
RUN rustup -v toolchain install nightly --profile minimal

WORKDIR /cipher-benchmarks

RUN <<EOT
#!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

git clone -b main https://github.com/andrcmdr/darkproto-proposal.git
cd /cipher-benchmarks/darkproto-proposal
bash ./make.sh submodules update && bash ./make.sh submodules
# bash ./run.benchmarks.sh
EOT

COPY --link build.benchmarks.sh /cipher-benchmarks/darkproto-proposal/build.benchmarks.sh
RUN bash /cipher-benchmarks/darkproto-proposal/build.benchmarks.sh
COPY --link run.benchmarks.sh /cipher-benchmarks/darkproto-proposal/run.benchmarks.sh
# RUN bash /cipher-benchmarks/darkproto-proposal/run.benchmarks.sh
CMD bash /cipher-benchmarks/darkproto-proposal/run.benchmarks.sh
