## Uniswap V3 Investigation Journal

### Investigation Uniswap V3 Slowness on TVL/Volume charts

On start, info site retrieves `poolDayDatas` for the top 20 pools addresses currently active. Once those 20 addresses, have been retrieved, the code perform a query for each of those pool on:

```gql
  query poolDayDatas($startTime: Int!, $skip: Int!, $address: Bytes!) {
    poolDayDatas(
      first: 1000
      skip: $skip
      where: { pool: $address, date_gt: $startTime }
      orderBy: date
      orderDirection: asc
      subgraphError: allow
    ) {
      date
      volumeUSD
      tvlUSD
      feesUSD
      pool {
        feeTier
      }
    }
  }
```

Those are actually perform sequentially, one after each other. To speed up the site loading, multiple queries should be performed in parallel.

### Key/Value Replacement

We are going to offer a replacement to this API where data is retrieved through Protobuf backed by a custom WASM resolver that will transform high-level requests (`poolDayDatas` where, between, etc.) to the underlying key/value space output by `kv_out` on Uniswap V3.

#### Data

Here the data received for a request perform on May 9th, 2023:

- [0x11b815efb8f581194ae79006d24e0d814b7697f6](./pool_day_datas_0x11b815efb8f581194ae79006d24e0d814b7697f6.json)
- [0x1d42064fc4beb5f8aaf85f4617ae8b3b5b8bd801](./pool_day_datas_0x1d42064fc4beb5f8aaf85f4617ae8b3b5b8bd801.json)
- [0x290a6a7460b308ee3f19023d2d00de604bcf5b42](./pool_day_datas_0x290a6a7460b308ee3f19023d2d00de604bcf5b42.json)
- [0x3416cf6c708da44db2624d63ea0aaef7113527c6](./pool_day_datas_0x3416cf6c708da44db2624d63ea0aaef7113527c6.json)
- [0x4585fe77225b41b697c938b018e2ac67ac5a20c0](./pool_day_datas_0x4585fe77225b41b697c938b018e2ac67ac5a20c0.json)
- [0x4e68ccd3e89f51c3074ca5072bbac773960dfa36](./pool_day_datas_0x4e68ccd3e89f51c3074ca5072bbac773960dfa36.json)
- [0x5777d92f208679db4b9778590fa3cab3ac9e2168](./pool_day_datas_0x5777d92f208679db4b9778590fa3cab3ac9e2168.json)
- [0x5c128d25a21f681e678cb050e551a895c9309945](./pool_day_datas_0x5c128d25a21f681e678cb050e551a895c9309945.json)
- [0x6c6bc977e13df9b0de53b251522280bb72383700](./pool_day_datas_0x6c6bc977e13df9b0de53b251522280bb72383700.json)
- [0x7379e81228514a1d2a6cf7559203998e20598346](./pool_day_datas_0x7379e81228514a1d2a6cf7559203998e20598346.json)
- [0x7bea39867e4169dbe237d55c8242a8f2fcdcc387](./pool_day_datas_0x7bea39867e4169dbe237d55c8242a8f2fcdcc387.json)
- [0x840deeef2f115cf50da625f7368c24af6fe74410](./pool_day_datas_0x840deeef2f115cf50da625f7368c24af6fe74410.json)
- [0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640](./pool_day_datas_0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640.json)
- [0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8](./pool_day_datas_0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8.json)
- [0x97e7d56a0408570ba1a7852de36350f7713906ec](./pool_day_datas_0x97e7d56a0408570ba1a7852de36350f7713906ec.json)
- [0x99ac8ca7087fa4a2a1fb6357269965a2014abc35](./pool_day_datas_0x99ac8ca7087fa4a2a1fb6357269965a2014abc35.json)
- [0xa6cc3c2531fdaa6ae1a3ca84c2855806728693e8](./pool_day_datas_0xa6cc3c2531fdaa6ae1a3ca84c2855806728693e8.json)
- [0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8](./pool_day_datas_0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8.json)
- [0xc63b0708e2f7e69cb8a1df0e1389a98c35a76d52](./pool_day_datas_0xc63b0708e2f7e69cb8a1df0e1389a98c35a76d52.json)
- [0xcbcdf9626bc03e24f779434178a73a0b4bad62ed](./pool_day_datas_0xcbcdf9626bc03e24f779434178a73a0b4bad62ed.json)


