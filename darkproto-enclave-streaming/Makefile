ARCH := $(shell uname -m)
RUST_DIR := $(shell readlink -m $(shell dirname $(firstword $(MAKEFILE_LIST))))

build:
	curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
	rustup target install ${ARCH}-unknown-linux-musl
	cargo build --manifest-path=${RUST_DIR}/Cargo.toml --target=${ARCH}-unknown-linux-musl --release
	cp ${RUST_DIR}/target/${ARCH}-unknown-linux-musl/release/darkproto-enclave-streaming ${RUST_DIR}

server: build
	docker build -t darkproto-enclave-streaming-server -f Dockerfile.server .
	nitro-cli build-enclave --docker-uri darkproto-enclave-streaming-server --output-file darkproto-enclave-streaming-server.eif

client: build
	docker build -t darkproto-enclave-streaming-client -f Dockerfile.client .
	nitro-cli build-enclave --docker-uri darkproto-enclave-streaming-client --output-file darkproto-enclave-streaming-client.eif

.PHONY: clean
clean:
	rm -rf ${RUST_DIR}/target ${RUST_DIR}/darkproto-enclave-streaming-*.eif ${RUST_DIR}/darkproto-enclave-streaming
