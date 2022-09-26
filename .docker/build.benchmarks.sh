#!/bin/bash
##!/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/aes-primitives/
echo -e "$(pwd)\n"
cargo +nightly build -v --workspace --benches --all-features --release -Z unstable-options --keep-going
cargo +nightly bench -v --no-run --no-fail-fast --workspace --benches --all-features -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/aes-benchmarks/
echo -e "$(pwd)\n"
cargo +nightly build -v --workspace --benches --all-features --release -Z unstable-options --keep-going
cargo +nightly bench -v --no-run --no-fail-fast --workspace --benches --all-features -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/block-ciphers/
echo -e "$(pwd)\n"
cargo +nightly build -v --workspace --benches --all-features --release -Z unstable-options --keep-going
cargo +nightly bench -v --no-run --no-fail-fast --workspace --benches --all-features -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/AEADs/
echo -e "$(pwd)\n"
cargo +nightly build -v --workspace --benches --all-features --release -Z unstable-options --keep-going
cargo +nightly bench -v --no-run --no-fail-fast --workspace --benches --all-features -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/stream-ciphers/
echo -e "$(pwd)\n"
cargo +nightly build -v --workspace --benches --all-features --release -Z unstable-options --keep-going
cargo +nightly bench -v --no-run --no-fail-fast --workspace --benches --all-features -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/RSA/
echo -e "$(pwd)\n"
patch --verbose --backup ./benches/key.rs ../.patches/RSA.key.rs.patch
cargo +nightly build -v --workspace --benches --all-features --release -Z unstable-options --keep-going
cargo +nightly bench -v --no-run --no-fail-fast --workspace --benches --all-features -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/elliptic-curves/
echo -e "$(pwd)\n"
patch --verbose --backup ./p256/benches/field.rs ../.patches/EC.p256.field.rs.patch
cargo +nightly build -v --workspace --benches --all-features --release -Z unstable-options --keep-going
cargo +nightly bench -v --no-run --no-fail-fast --workspace --benches --all-features -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/threshold-crypto/
echo -e "$(pwd)\n"
cargo +nightly build -v --workspace --benches --all-features --release -Z unstable-options --keep-going
cargo +nightly bench -v --no-run --no-fail-fast --workspace --benches --all-features -Z unstable-options --keep-going

echo -e "Successfully finished!!!\n"
