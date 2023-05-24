# Uniswap v3 Substreams to Key/Value Sink

Build:

```bash
make build
```

Run:

```bash
substreams gui -e api.streamingfast.io:443 substreams.yaml kv_out -t +1000
```

Use in conjunction with https://github.com/streamingfast/substreams-sink-kv



*Run in inject mode*
./inject.sh

*Run in serve mode*
./inject.sh


*Querying kv store*
kvdb read prefix "kPoolDayData"  --dsn "badger3://$(pwd)/badger_data.db" --decoder="ascii"

*WASM Query curl*

```bash
curl --header 'Content-Type: application/json' --data '{"addresses": ["0x11b815efb8f581194ae79006d24e0d814b7697f6"]}' localhost:7878/uniswap.info.v1.UniswapInfo/PoolDayDatas
```

