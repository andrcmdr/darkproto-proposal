#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

curl -fsSL https://sh.rustup.rs | \
env CARGO_HOME="$HOME/rust" RUSTUP_HOME="$HOME/rustup" PATH="$PATH:$HOME/rust/bin" \
bash -is -- -y --verbose --no-modify-path --default-toolchain stable --profile minimal;

env CARGO_HOME="$HOME/rust" RUSTUP_HOME="$HOME/rustup" PATH="$PATH:$HOME/rust/bin" \
rustup -v toolchain install nightly --profile minimal;
# source "$HOME/rust/env" && rustup -v toolchain install nightly --profile minimal;

tee -a "$HOME/.bashrc" << RUSTTOOLCHAIN

source "$HOME/rust/env"
export CARGO_HOME="$HOME/rust"
export RUSTUP_HOME="$HOME/rustup"
export PATH="$PATH:$HOME/rust/bin"

RUSTTOOLCHAIN

