#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

echo -e "\n"

# AES-128-CBC with AES-NI disabled
OPENSSL_ia32cap="~0x200000200000000" openssl speed -elapsed -evp aes-128-cbc

echo -e "\n"

# AES-128-CBC with AES-NI enabled
openssl speed -elapsed -evp aes-128-cbc

echo -e "\n"

# AES-256-CBC with AES-NI disabled
OPENSSL_ia32cap="~0x200000200000000" openssl speed -elapsed -evp aes-256-cbc

echo -e "\n"

# AES-256-CBC with AES-NI enabled
openssl speed -elapsed -evp aes-256-cbc

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/aes-primitives/
echo -e "$(pwd)\n"
cargo +nightly bench -v --no-fail-fast --workspace --benches --all-features --frozen --locked --offline -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/aes-benchmarks/
echo -e "$(pwd)\n"
cargo +nightly bench -v --no-fail-fast --workspace --benches --all-features --frozen --locked --offline -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/block-ciphers/
echo -e "$(pwd)\n"
cargo +nightly bench -v --no-fail-fast --workspace --benches --all-features --frozen --locked --offline -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/AEADs/
echo -e "$(pwd)\n"
cargo +nightly bench -v --no-fail-fast --workspace --benches --all-features --frozen --locked --offline -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/stream-ciphers/
echo -e "$(pwd)\n"
cargo +nightly bench -v --no-fail-fast --workspace --benches --all-features --frozen --locked --offline -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/RSA/
echo -e "$(pwd)\n"
patch --verbose --backup ./benches/key.rs ../.patches/RSA.key.rs.patch
cargo +nightly bench -v --no-fail-fast --workspace --benches --all-features --frozen --locked --offline -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/elliptic-curves/
echo -e "$(pwd)\n"
patch --verbose --backup ./p256/benches/field.rs ../.patches/EC.p256.field.rs.patch
cargo +nightly bench -v --no-fail-fast --workspace --benches --all-features --frozen --locked --offline -Z unstable-options --keep-going

echo -e "\n"

cd /cipher-benchmarks/darkproto-proposal/threshold-crypto/
echo -e "$(pwd)\n"
cargo +nightly bench -v --no-fail-fast --workspace --benches --all-features --frozen --locked --offline -Z unstable-options --keep-going

echo -e "Successfully finished!!!\n"
