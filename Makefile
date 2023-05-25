ENDPOINT ?= mainnet.eth.streamingfast.io:443
STOP_BLOCK ?= +500
ROOT_DIR ?= $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
KV_DSN ?= badger3://$(ROOT_DIR)/kv.db

.PHONY: build
build: build_substreams build_query

.PHONY: build_substreams
build:
	cargo build --target wasm32-unknown-unknown -p substreams-uniswap-v3-info --release

.PHONY: build_query
build_query:
	cargo build --target wasm32-wasi -p query --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: stream_kv
stream_kv: build
	substreams run -e $(ENDPOINT) substreams.yaml kv_out -t $(STOP_BLOCK)

.PHONY: sync_kv
sync_kv: build_query
	DYLD_LIBRARY_PATH=$(LIBRARY_PATH) substreams-sink-kv inject $(ENDPOINT) $(KV_DSN) substreams.yaml

.PHONY: serve_kv
serve_kv: build_query
	DYLD_LIBRARY_PATH=$(LIBRARY_PATH) substreams-sink-kv serve $(KV_DSN) substreams.yaml