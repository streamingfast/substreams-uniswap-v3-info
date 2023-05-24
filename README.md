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



### Run in inject mode
./inject.sh

*Run in serve mode*
./inject.sh


### Expore the keys in the store

```
kvdb read prefix "kPoolDayData"  --dsn "badger3://$(pwd)/badger_data.db" --decoder="ascii"
```

### Query your server

```bash
# A single pool
curl --header 'Content-Type: application/json' --data '{"addresses": ["0x11b815efb8f581194ae79006d24e0d814b7697f6"]}' localhost:7878/uniswap.info.v1.UniswapInfo/PoolDayDatas

# Top pools
curl --header 'Content-Type: application/json' --data '{"addresses": [                                                                                                                      [⎈ glob:eth-mainnet]
  "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640", "0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8", "0xcbcdf9626bc03e24f779434178a73a0b4bad62ed", "0x4e68ccd3e89f51c3074ca5072bbac773960dfa36", "0x5777d92f208679db4b9778590fa3cab3ac9e2168", "0x4585fe77225b41b697c938b018e2ac67ac5a20c0", "0xc63b0708e2f7e69cb8a1df0e1389a98c35a76d52", "0x3416cf6c708da44db2624d63ea0aaef7113527c6", "0x11b815efb8f581194ae79006d24e0d814b7697f6", "0x99ac8ca7087fa4a2a1fb6357269965a2014abc35", "0x6c6bc977e13df9b0de53b251522280bb72383700", "0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8", "0x7bea39867e4169dbe237d55c8242a8f2fcdcc387", "0xa6cc3c2531fdaa6ae1a3ca84c2855806728693e8", "0x7379e81228514a1d2a6cf7559203998e20598346", "0x5c128d25a21f681e678cb050e551a895c9309945", "0x840deeef2f115cf50da625f7368c24af6fe74410", "0x290a6a7460b308ee3f19023d2d00de604bcf5b42", "0xc5af84701f98fa483ece78af83f11b6c38aca71d", "0xe931b03260b2854e77e8da8378a1bc017b13cb97", "0x97e7d56a0408570ba1a7852de36350f7713906ec", "0xac4b3dacb91461209ae9d41ec517c2b9cb1b7daf", "0x7858e59e0c01ea06df3af3d20ac7b0003275d4bf", "0x60594a405d53811d3bc4766596efd80fd545a270", "0x9db9e0e53058c89e5b94e29621a205198648425b", "0x69d91b94f0aaf8e8a2586909fa77a5c2c89818d5", "0x1d42064fc4beb5f8aaf85f4617ae8b3b5b8bd801", "0x4b5ab61593a2401b1075b90c04cbcdd3f87ce011", "0x40e629a26d96baa6d81fae5f97205c2ab2c1ff29", "0xa3f558aebaecaf0e11ca4b2199cc5ed341edfd74", "0x6c4c7f46d9d4ef6bc5c9e155f011ad19fc4ef321", "0xf56d08221b5942c428acc5de8f78489a97fc5599", "0x42d403ab9b0603442ac991c0cfe124105dde0811", "0xa4e0faa58465a2d369aa21b3e42d43374c6f9613", "0xd340b57aacdd10f96fc1cf10e15921936f41e29c", "0xe8c6c9227491c0a8156a0106a0204d881bb7e531", "0x9febc984504356225405e26833608b17719c82ae", "0x5764a6f2212d502bc5970f9f129ffcd61e5d7563", "0x11950d141ecb863f01007add7d1a342041227b58", "0x3b685307c8611afb2a9e83ebc8743dc20480716e", "0xf79fc43494ce8a4613cb0b2a67a1b1207fd05d27", "0x84383fb05f610222430f69727aa638f8fdbf5cc1", "0x3afdc5e6dfc0b0a507a8e023c9dce2cafc310316", "0xf4ad61db72f114be877e87d62dc5e7bd52df4d9b", "0x5d752f322befb038991579972e912b02f61a3dda", "0x151ccb92bc1ed5c6d0f9adb5cec4763ceb66ac7f", "0x9e0905249ceefffb9605e034b534544684a58be6", "0xe15e6583425700993bd08f51bf6e7b73cd5da91b", "0x34ff465ee92516e9855eb60f520df0384f410b45", "0xe42318ea3b998e8355a3da364eb9d48ec725eb45"
]}' localhost:7878/uniswap.info.v1.UniswapInfo/PoolDayDatas
```

