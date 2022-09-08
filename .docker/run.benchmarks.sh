#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

# AES-128-CBC with AES-NI disabled
OPENSSL_ia32cap="~0x200000200000000" openssl speed -elapsed -evp aes-128-cbc

# AES-128-CBC with AES-NI enabled
openssl speed -elapsed -evp aes-128-cbc

# AES-256-CBC with AES-NI disabled
OPENSSL_ia32cap="~0x200000200000000" openssl speed -elapsed -evp aes-256-cbc

# AES-256-CBC with AES-NI enabled
openssl speed -elapsed -evp aes-256-cbc

cd aes-primitives
cargo bench

cd aes-benchmarks
cargo bench

cd block-ciphers
cargo bench

cd AEADs
cargo bench

cd stream-ciphers
cargo bench

cd RSA
cargo bench

cd elliptic-curves
cargo bench

cd threshold-crypto
cargo bench
