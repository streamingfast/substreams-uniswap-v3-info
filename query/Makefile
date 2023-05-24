ENDPOINT ?= api-unstable.streamingfast.io:443
GRAPH_CONFIG ?= ../graph-node-dev/config/graphman.toml

.PHONY: build
build:
	cargo build --target wasm32-wasi --release

.PHONY: protogen
protogen:
	buf generate https://github.com/streamingfast/substreams-uniswap-v3-info/releases/download/v0.1.3/uniswap-v3-kv-v0.1.3.spkg#format=bin --exclude-path=sf/substreams/rpc,sf/substreams/entity,sf/substreams/v1,google

.PHONY: package
package: build
	substreams pack -o substreams.spkg substreams.yaml
