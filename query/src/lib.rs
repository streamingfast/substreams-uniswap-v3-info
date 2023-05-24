mod pb;

use crate::pb::uniswap::info::v1::{PoolDayDatasRequest, PoolDayDatasResponse, PoolsDayData};
use bigdecimal::BigDecimal;
use prost::Message;
use std::collections::HashMap;
use std::str;
use std::str::FromStr;
use substreams_sink_kv::prelude::*;

#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[derive(Default)]
struct PoolDayData {
    volume_usd: BigDecimal,
    tvl_usd: BigDecimal,
}

#[wasmedge_bindgen]
pub fn uniswap_info_v1_uniswapinfo_pooldaydatas(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = PoolDayDatasRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();

    let mut accum = HashMap::<String, PoolDayData>::new();

    for pool_addr in req.addresses {
        let start = format!("PoolDayData:{}:0000000000", pool_addr);
        let end = format!("PoolDayData:{}:9999999999", pool_addr);

        let key_values = store.scan(start, end, None);

        for kv_pair in key_values.pairs {
            let date_id = kv_pair.key.split(":").nth(2).unwrap();
            let data_type = kv_pair.key.split(":").last().unwrap();
            let value = BigDecimal::from_str(str::from_utf8(kv_pair.value.as_slice()).unwrap()).unwrap();

            let elem = accum.entry(date_id.to_string()).or_default();

            match data_type {
                "volumeUSD" => elem.volume_usd += value,
                "tvlUSD" => elem.tvl_usd += value,
                _ => (),
            }
        }
    }

    let mut out = PoolDayDatasResponse { pool_days_data: vec![] };

    for (key, val) in accum.iter_mut() {
        out.pool_days_data.push(PoolsDayData {
            date: key.to_string(),
            volume_usd: val.volume_usd.to_string(),
            tvl_usd: val.tvl_usd.to_string(),
        })
    }

    out.pool_days_data.sort_by(|a, b| a.date.cmp(&b.date));

    return Ok(out.encode_to_vec());
}
