mod pb;

use crate::pb::uniswap::info::v1::{PoolDayDatasRequest, PoolDayDatasResponse, PoolsDayData};
use bigdecimal::{BigDecimal, ToPrimitive};
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
    volume_usd: f64,
    tvl_usd: f64,
}

#[wasmedge_bindgen]
pub fn uniswap_info_v1_uniswapinfo_pooldaydatas(v: Vec<u8>) -> Result<Vec<u8>, String> {
    // We use a pure Rust handler otherwise editor(s) has problem helping you within the macro block.
    handler(v)
}

pub fn handler(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = PoolDayDatasRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();

    let mut accum = HashMap::<String, PoolDayData>::with_capacity(20 * 1000);

    for pool_addr in req.addresses {
        let start = format!("PoolDayData:{}:0000000000", pool_addr);
        let end = format!("PoolDayData:{}:9999999999", pool_addr);

        let key_values = store.scan(start, end, None);
        for kv_pair in key_values.pairs {
            let Key { data_type, date_id } = split_key(&kv_pair.key);
            let value = f64::from_str(unsafe { str::from_utf8_unchecked(kv_pair.value.as_slice()) }).unwrap();
            let elem: &mut PoolDayData = accum.entry(date_id.to_string()).or_default();

            match data_type.as_str() {
                "volumeUSD" => elem.volume_usd += &value,
                "tvlUSD" => elem.tvl_usd += &value,
                _ => (),
            }
        }
    }

    let mut out: PoolDayDatasResponse = PoolDayDatasResponse {
        pool_days_data: accum
            .into_iter()
            .map(|(key, val)| PoolsDayData {
                date: key,
                volume_usd: val.volume_usd.to_string(),
                tvl_usd: val.tvl_usd.to_string(),
            })
            .collect(),
    };

    out.pool_days_data.sort_by(|a, b| a.date.cmp(&b.date));

    return Ok(out.encode_to_vec());
}

struct Key {
    date_id: String,
    data_type: String,
}

fn split_key(key: &String) -> Key {
    let mut date_id = None;
    let mut last = None;

    for (i, part) in key.split(":").enumerate() {
        if i == 2 {
            date_id = Some(part.to_string())
        }
        last = Some(part)
    }

    if last.is_none() {}

    match last {
        Some(last) => Key {
            date_id: date_id.unwrap(),
            data_type: last.to_string(),
        },
        None => panic!("invalid key {:}", key),
    }
}
