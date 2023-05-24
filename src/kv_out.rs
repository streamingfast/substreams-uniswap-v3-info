use substreams::pb::substreams::store_delta::Operation;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{Delta, DeltaBigDecimal, key_first_segments_in, operations_ne};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams::store::{ DeltaBigInt, Deltas };
use crate::pb::uniswap::types::v1::Pools;

pub fn pool_feetier_create(ops: &mut KvOperations, pools: &Pools) {
    for pool in &pools.pools {
        let pool_addr = format!("0x{}", &pool.address);
        ops.push_new(format!("{pool_addr}:feeTier"), &pool.fee_tier.to_string(), 0);
    }
}

pub fn pool_day_data_create(ops: &mut KvOperations, tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas
        .deltas
        .iter()
        .filter(key_first_segments_in(vec!["PoolDayData"]))
        .filter(operations_ne(Operation::Delete))
        .filter(|d| d.new_value.eq(&BigInt::one()))
    {
        let time_id = key_segment(&delta.key, 1).parse::<i64>().unwrap();
        let pool_address = key_segment(&delta.key, 2);
        create_pool_day_data(ops, time_id, pool_address, delta.ordinal);
    }
}

pub fn pool_day_data_update(
    ops: &mut KvOperations,
    swaps_volume_deltas: &Deltas<DeltaBigDecimal>,
    derived_tvl_deltas: &Deltas<DeltaBigDecimal>,
) {
    swap_volume(ops, &swaps_volume_deltas);
    total_value_locked_usd(ops, &derived_tvl_deltas);
}

fn create_pool_day_data(
    ops: &mut KvOperations,
    date_id: i64,
    pool_address: &str,
    ordinal: u64,
) {
    let date = BigInt::from(date_id * 86400);
    let pool_addr = format!("0x{pool_address}");
    let padded_date_id = left_pad(date_id);
    ops.push_new(format!("pool:{pool_addr}:{padded_date_id}:date"), date.to_string(), ordinal);
    ops.push_new(format!("pool:{pool_addr}:{padded_date_id}:volumeUSD"), BigDecimal::zero().to_string(), ordinal);
    ops.push_new(format!("pool:{pool_addr}:{padded_date_id}:tvlUSD"), BigDecimal::zero().to_string(), ordinal);
    ops.push_new(format!("pool:{pool_addr}:{padded_date_id}:feesUSD"), BigDecimal::zero().to_string(), ordinal);
}


pub fn swap_volume(ops: &mut KvOperations, swaps_volume_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in swaps_volume_deltas
        .deltas
        .iter()
        .filter(key_first_segments_in(vec!["PoolDayData"]))
        .filter(operations_ne(Operation::Delete))
        .filter(key_last_segments_in(vec![
            "volumeUSD",
            "feesUSD",
        ]))
    {
        let (_, time_id, pool_address) = pool_windows_id_fields(&delta.key);
        let date_id = time_id.parse::<i64>().unwrap();
        let attribute = last_segment(&delta.key);
        let pool_addr = format!("0x{pool_address}");
        let padded_date_id = left_pad(date_id);
        ops.push_new(format!("{pool_addr}:{padded_date_id}:{attribute}"),&delta.new_value.to_string(), delta.ordinal);
    }
}

pub fn total_value_locked_usd(ops: &mut KvOperations, derived_tvl_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in derived_tvl_deltas
        .deltas
        .iter()
        .filter(key_first_segments_in(vec!["PoolDayData"]))
        .filter(operations_ne(Operation::Delete))
        .filter(key_last_segment_in("totalValueLockedUSD"))
    {
        let (_, time_id, pool_address) = pool_windows_id_fields(&delta.key);
        let date_id = time_id.parse::<i64>().unwrap();
        let pool_addr = format!("0x{pool_address}");
        let padded_date_id = left_pad(date_id);
        ops.push_new(format!("{pool_addr}:{padded_date_id}:tvlUSD"), &delta.new_value.to_string(), delta.ordinal);
    }
}

fn key_segment(key: &String, index: usize) -> &str {
    return try_segment(key, index).unwrap();
}

fn key_last_segments_in<T: Delta>(idx: Vec<&str>) -> impl FnMut(&&T) -> bool + '_ {
    move |delta| idx.contains(&last_segment(delta.get_key()))
}

fn key_last_segment_in<T: Delta>(key_segment: &str) -> impl FnMut(&&T) -> bool + '_ {
    move |delta| last_segment(delta.get_key()) == key_segment
}

fn try_segment(key: &String, index: usize) -> Option<&str> {
    let val = key.split(":").nth(index);
    match val {
        Some(val) => Some(val),
        None => None,
    }
}

fn last_segment(key: &String) -> &str {
    return try_last_segment(key).unwrap();
}

fn try_last_segment(key: &String) -> Option<&str> {
    let val = key.split(":").last();
    match val {
        Some(val) => Some(val),
        None => None,
    }
}

fn first_segment(key: &String) -> &str {
    key.split(":").next().unwrap()
}

fn segment(key: &String, index: usize) -> &str {
    return try_segment(key, index).unwrap();
}

fn pool_windows_id_fields(key: &String) -> (&str, &str, &str) {
    let table_name = first_segment(key);
    let time_id = segment(key, 1);
    let pool_address = segment(key, 2);

    return (table_name, time_id, pool_address);
}

fn left_pad(key: i64) -> String {
    return format!("{:0>10}", key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_pad() {

        assert_eq!("0000000123", left_pad(123));
    }
}
