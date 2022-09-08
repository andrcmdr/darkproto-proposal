# syntax=docker/dockerfile:1.4
FROM public.ecr.aws/amazonlinux/amazonlinux:2 as builder

RUN yum upgrade -y
RUN amazon-linux-extras enable epel
RUN yum clean -y metadata && yum install -y epel-release
RUN yum install -y gcc git
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain 1.63.0

WORKDIR /darkproto-app

RUN <<EOT
#!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

git clone -b main https://github.com/andrcmdr/darkproto-proposal.git
cd ./darkproto-proposal
bash ./make.sh submodules update && bash ./make.sh submodules
source $HOME/.cargo/env && cargo build --release
mv -T ./target/x86_64-unknown-linux-gnu/release/darkproto-proposal /darkproto-app/darkproto-app
EOT

FROM public.ecr.aws/amazonlinux/amazonlinux:2 as enclave_app
WORKDIR /app
COPY --from=builder /darkproto-app/darkproto-app /app

ENV RUST_LOG="darkproto=debug"

CMD ["/app/darkproto-app"]
