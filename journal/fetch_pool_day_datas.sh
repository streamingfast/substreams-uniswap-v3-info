#!/usr/bin/env bash

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

option=""

main() {
  pushd "$ROOT" &> /dev/null

  while getopts "ho:" opt; do
    case $opt in
      h) usage && exit 0;;
      o) option="$OPTARG";;
      \?) usage_error "Invalid option: -$OPTARG";;
    esac
  done
  shift $((OPTIND-1))

  addresses=$@
  if [[ $# -le 0 ]]; then
    addresses=(
        0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640
        0xcbcdf9626bc03e24f779434178a73a0b4bad62ed
        0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8
        0x4e68ccd3e89f51c3074ca5072bbac773960dfa36
        0x4585fe77225b41b697c938b018e2ac67ac5a20c0
        0x5777d92f208679db4b9778590fa3cab3ac9e2168
        0xc63b0708e2f7e69cb8a1df0e1389a98c35a76d52
        0x99ac8ca7087fa4a2a1fb6357269965a2014abc35
        0x7379e81228514a1d2a6cf7559203998e20598346
        0x3416cf6c708da44db2624d63ea0aaef7113527c6
        0x6c6bc977e13df9b0de53b251522280bb72383700
        0x11b815efb8f581194ae79006d24e0d814b7697f6
        0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8
        0x1d42064fc4beb5f8aaf85f4617ae8b3b5b8bd801
        0x7bea39867e4169dbe237d55c8242a8f2fcdcc387
        0xa6cc3c2531fdaa6ae1a3ca84c2855806728693e8
        0x5c128d25a21f681e678cb050e551a895c9309945
        0x290a6a7460b308ee3f19023d2d00de604bcf5b42
        0x840deeef2f115cf50da625f7368c24af6fe74410
        0x97e7d56a0408570ba1a7852de36350f7713906ec
    )
  fi

  for address in ${addresses[@]}; do
    fetch_pool_data "$address"
  done
}

fetch_pool_data() {
    address="$1"

    echo -n "Fetching pool day datas for '$address'"
    curl -s 'https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3' \
    -H 'content-type: application/json' \
    --data-raw $'{"operationName":"poolDayDatas","variables":{"address":"'"$address"$'","startTime":1619170975,"skip":0},"query":"query poolDayDatas($startTime: Int\u0021, $skip: Int\u0021, $address: Bytes\u0021) {\\n  poolDayDatas(\\n    first: 1000\\n    skip: $skip\\n    where: {pool: $address, date_gt: $startTime}\\n    orderBy: date\\n    orderDirection: asc\\n    subgraphError: allow\\n  ) {\\n    date\\n    volumeUSD\\n    tvlUSD\\n    feesUSD\\n    pool {\\n      feeTier\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n"}' \
    --compressed > "pool_day_datas_$address".json
    echo " OK"
}

usage_error() {
  message="$1"
  exit_code="$2"

  echo "ERROR: $message"
  echo ""
  usage
  exit ${exit_code:-1}
}

usage() {
  echo "usage: fetch_pool_day_datas [<address>]"
  echo ""
  echo "Fetch the pool day datas from The Graph."
  echo ""
  echo "Options"
  echo "    -h          Display help about this script"
}

main "$@"