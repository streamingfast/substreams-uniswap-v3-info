extern crate core;

use substreams::errors::Error;
use substreams::key::{
    first_segment, key_first_segment_in, key_first_segments_in, key_last_segment_in, operations_ne, segment,
};
use substreams::store::{DeltaBigDecimal, DeltaBigInt, Deltas};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use substreams::pb::substreams::store_delta::Operation;
use substreams::scalar::BigInt;

#[substreams::handlers::map]
pub fn kv_out(
    tx_count_deltas: Deltas<DeltaBigInt>,         /* store_total_tx_counts deltas */
    swaps_volume_deltas: Deltas<DeltaBigDecimal>, /* store_swaps_volume */
    derived_tvl_deltas: Deltas<DeltaBigDecimal>,  /* store_derived_tvl */
) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    pool_day_data_create(&mut kv_ops, &tx_count_deltas);
    pool_day_data_update(&mut kv_ops, &swaps_volume_deltas, &derived_tvl_deltas);

    Ok(kv_ops)
}

pub fn pool_day_data_create(ops: &mut KvOperations, tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas
        .deltas
        .iter()
        .filter(key_first_segment_in("PoolDayData"))
        .filter(operations_ne(Operation::Delete))
        .filter(|d| d.new_value.eq(&BigInt::one()))
    {
        let (_table_name, pool_addr, day_id) = pool_windows_id_fields(&delta.key);
        let date = day_id * 86400;
        ops.push_new(format!("PoolDayData:0x{pool_addr}:{date:0>10}:volumeUSD"), "0.0", 0);
        ops.push_new(format!("PoolDayData:0x{pool_addr}:{date:0>10}:tvlUSD"), "0.0", 0);
    }
}

pub fn pool_day_data_update(
    ops: &mut KvOperations,
    swaps_volume_deltas: &Deltas<DeltaBigDecimal>,
    derived_tvl_deltas: &Deltas<DeltaBigDecimal>,
) {
    for delta in swaps_volume_deltas
        .deltas
        .iter()
        .filter(key_first_segment_in("PoolDayData"))
        .filter(operations_ne(Operation::Delete))
        .filter(key_last_segment_in("volumeUSD"))
    {
        let (_table_name, pool_addr, day_id) = pool_windows_id_fields(&delta.key);
        let date = day_id * 86400;
        ops.push_new(
            format!("PoolDayData:0x{pool_addr}:{date:0>10}:volumeUSD"),
            &delta.new_value.to_string(),
            0,
        );
    }

    for delta in derived_tvl_deltas
        .deltas
        .iter()
        .filter(key_first_segments_in(vec!["PoolDayData"]))
        .filter(operations_ne(Operation::Delete))
        .filter(key_last_segment_in("totalValueLockedUSD"))
    {
        let (_table_name, pool_addr, day_id) = pool_windows_id_fields(&delta.key);
        let date = day_id * 86400;
        ops.push_new(
            format!("PoolDayData:0x{pool_addr}:{date:0>10}:tvlUSD"),
            &delta.new_value.to_string(),
            0,
        );
    }
}

pub fn pool_windows_id_fields(key: &String) -> (&str, &str, u64) {
    let table_name = first_segment(key);
    let time_id = segment(key, 1).parse::<u64>().unwrap();
    let pool_addr = segment(key, 2);

    return (table_name, pool_addr, time_id);
}
