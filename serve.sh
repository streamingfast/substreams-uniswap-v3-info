#!/usr/bin/env bash

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

export DYLD_LIBRARY_PATH=$LIBRARY_PATH
substreams-sink-kv serve "bigkv://dfuseio-global.dfuse-saas/thegraph-uniswap-v3-info-kv-v2" "$ROOT/substreams.yaml"
