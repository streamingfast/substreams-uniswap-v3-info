mod pb;

use crate::pb::uniswap::info::v1::{PoolDayDatasRequest, PoolDayDatasResponse, PoolsDayData};
use prost::Message;
use std::collections::HashMap;
use std::str;
use std::str::FromStr;
use std::time::SystemTime;
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
    let now = SystemTime::now();

    let req = PoolDayDatasRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();

    let mut accum = HashMap::<u32, PoolDayData>::with_capacity(20 * 1000);

    for pool_addr in req.addresses {
        let now_pool = SystemTime::now();

        let start = format!("PoolDayData:{}:0000000000", pool_addr);
        let end = format!("PoolDayData:{}:9999999999", pool_addr);

        let now_scan = SystemTime::now();
        let key_values = store.scan(start, end, None);

        println!(
            "key scanned in {:?} (keys {:?})",
            now_scan.elapsed().unwrap(),
            key_values.pairs.len()
        );

        let now_transform = SystemTime::now();

        for kv_pair in key_values.pairs {
            // if pool_addr == "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640" {
            //     println!("key {:?}", kv_pair.key);
            // }

            let Key { data_type, date_id } = split_key(&kv_pair.key);
            let value = f64::from_str(unsafe { str::from_utf8_unchecked(kv_pair.value.as_slice()) }).unwrap();
            let elem: &mut PoolDayData = accum.entry(date_id).or_default();

            match data_type.as_str() {
                "volumeUSD" => elem.volume_usd += &value,
                "tvlUSD" => elem.tvl_usd += &value,
                _ => (),
            }
        }

        println!("key transformed in {:?}", now_transform.elapsed().unwrap());
        println!("pool took {:?}", now_pool.elapsed().unwrap());
    }

    let now_out = SystemTime::now();
    let mut out = PoolDayDatasResponse {
        pool_days_data: accum
            .into_iter()
            .map(|(key, val)| PoolsDayData {
                date: key,
                volume_usd: val.volume_usd,
                tvl_usd: val.tvl_usd,
            })
            .collect(),
    };
    println!("out done in {:?}", now_out.elapsed().unwrap());

    let now_sort = SystemTime::now();
    out.pool_days_data.sort_by(|a, b| a.date.cmp(&b.date));
    println!("sort done in {:?}", now_sort.elapsed().unwrap());

    println!("done in {:?}", now.elapsed().unwrap());

    return Ok(out.encode_to_vec());
}

struct Key {
    date_id: u32,
    data_type: String,
}

fn split_key(key: &String) -> Key {
    let mut date_id = None;
    let mut last = None;

    for (i, part) in key.split(":").enumerate() {
        if i == 2 {
            date_id = Some(u32::from_str(part).unwrap())
        }
        last = Some(part)
    }

    match last {
        Some(last) => Key {
            date_id: date_id.unwrap(),
            data_type: last.to_string(),
        },
        None => panic!("invalid key {:}", key),
    }
}
