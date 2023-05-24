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