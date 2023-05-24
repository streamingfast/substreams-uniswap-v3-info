mod helper;
mod pb;

use std::collections::HashMap;
use crate::pb::uniswap::info::v1::{PoolDayDatasRequest, PoolDayDatasResponse, PoolsDayData};
use prost::Message;
use substreams_sink_kv::prelude::*;
use std::str;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use bigdecimal::Zero;

#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

struct PoolDayData {
    volume_usd: BigDecimal,
    tvl_usd: BigDecimal,
}

#[wasmedge_bindgen]
pub fn uniswap_info_v1_uniswapinfo_pooldaydatas(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = PoolDayDatasRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();

    let mut foo = HashMap::new();


    for address in req.addresses {
        let start = format!("PoolDayData:{}:{}", address,req.start_time);
        let end = format!("PoolDayData:{}:1684952073", address);
        println!("start: {}",start);
        println!("end: {}", end);
        let key_values = store.scan(start, end, None);

        for kv_pair in key_values.pairs {
            let date_id = segment(&kv_pair.key.clone(),1);
            let data_type = last_segment(&kv_pair.key.clone());
            let value = BigDecimal::from_str(str::from_utf8(kv_pair.value.as_slice()).unwrap()).unwrap();

            println!("date_id: {}", date_id);
            println!("data_type: {}", data_type);
            println!("value: {}", value);

            let elem = foo.entry(date_id).or_insert(PoolDayData{volume_usd: BigDecimal::zero(), tvl_usd: BigDecimal::zero()});

            if  data_type == "tvlUSD" {
                elem.tvl_usd += value;
            }else if  data_type == "volumeUSD" {
                elem.volume_usd += value;
            }
        }
    }


    let mut out = PoolDayDatasResponse{
        pool_days_data: vec![],
    };

    for (key, val) in foo.iter_mut() {
        out.pool_days_data.push(PoolsDayData{
            date: key.to_string(),
            volume_usd: val.volume_usd.to_string(),
            tvl_usd: val.tvl_usd.to_string(),
        })
    }



    return Ok(out.encode_to_vec());
}

pub fn segment(key: &String, index: usize) -> &str {
    return try_segment(key, index).unwrap();
}


pub fn last_segment(key: &String) -> &str {
    return try_last_segment(key).unwrap();
}

pub fn try_last_segment(key: &String) -> Option<&str> {
    let val = key.split(":").last();
    match val {
        Some(val) => Some(val),
        None => None,
    }
}

pub fn try_segment(key: &String, index: usize) -> Option<&str> {
    let val = key.split(":").nth(index);
    match val {
        Some(val) => Some(val),
        None => None,
    }
}
