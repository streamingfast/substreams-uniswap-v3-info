#!/usr/bin/env bash

export DYLD_LIBRARY_PATH=$LIBRARY_PATH  # for MacOS, to link to wasmedge
substreams-sink-kv inject mainnet.eth.streamingfast.io:443 "badger3://$(pwd)/badger_data.db" substreams.yaml
