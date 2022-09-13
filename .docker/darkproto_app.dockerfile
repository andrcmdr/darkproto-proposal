# syntax=docker/dockerfile:1.4
FROM public.ecr.aws/amazonlinux/amazonlinux:2 as builder

RUN yum upgrade -y
RUN amazon-linux-extras enable epel
RUN yum clean -y metadata && yum install -y epel-release
RUN yum install -y gcc git

ENV CARGO_HOME="$HOME/rust" RUSTUP_HOME="$HOME/rustup" PATH="$PATH:$HOME/rust/bin"
RUN curl -fsSL https://sh.rustup.rs | bash -is -- -y --verbose --no-modify-path --default-toolchain stable --profile minimal
RUN rustup -v toolchain install nightly --profile minimal

WORKDIR /darkproto-app

RUN <<EOT
#!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

git clone -b main https://github.com/andrcmdr/darkproto-proposal.git
cd ./darkproto-proposal
# bash ./make.sh submodules update && bash ./make.sh submodules
cargo build --release
mv -T ./target/x86_64-unknown-linux-gnu/release/darkproto-proposal /darkproto-app/darkproto-app
EOT

FROM public.ecr.aws/amazonlinux/amazonlinux:2 as enclave_app
WORKDIR /app
COPY --from=builder /darkproto-app/darkproto-app /app

ENV RUST_LOG="darkproto=debug"
CMD ["/app/darkproto-app"]
