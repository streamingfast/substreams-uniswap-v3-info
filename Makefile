ENDPOINT ?= mainnet.eth.streamingfast.io:443
STOP_BLOCK ?= +500


.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: build-query
build:
	./query/build.sh

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: stream_kv
stream_kv: build
	substreams run -e $(ENDPOINT) substreams.yaml kv_out -t $(STOP_BLOCK)
