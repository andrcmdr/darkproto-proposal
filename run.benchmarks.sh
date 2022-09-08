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

cd ./aes-primitives/
echo -e "$(pwd)\n"
cargo +nightly bench -v --workspace --all-targets --all-features

echo -e "\n"

cd ../aes-benchmarks/
echo -e "$(pwd)\n"
cargo +nightly bench -v --workspace --all-targets --all-features

echo -e "\n"

cd ../block-ciphers/
echo -e "$(pwd)\n"
cargo +nightly bench -v --workspace --all-targets --all-features

echo -e "\n"

cd ../AEADs/
echo -e "$(pwd)\n"
cargo +nightly bench -v --workspace --all-targets --all-features

echo -e "\n"

cd ../stream-ciphers/
echo -e "$(pwd)\n"
cargo +nightly bench -v --workspace --all-targets --all-features

echo -e "\n"

cd ../RSA/
echo -e "$(pwd)\n"
patch --verbose --backup ./benches/key.rs ../.patches/RSA.key.rs.patch
cargo +nightly bench -v --workspace --all-targets --all-features

echo -e "\n"

cd ../elliptic-curves/
echo -e "$(pwd)\n"
patch --verbose --backup ./p256/benches/field.rs ../.patches/EC.p256.field.rs.patch
cargo +nightly bench -v --workspace --all-targets --all-features

echo -e "\n"

cd ../threshold-crypto/
echo -e "$(pwd)\n"
cargo +nightly bench -v --workspace --all-targets --all-features
