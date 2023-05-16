extern crate core;

#[path = "kv_out.rs"]
mod kv;
mod pb;

use substreams::errors::Error;
use substreams::store::{DeltaBigDecimal, DeltaBigInt, Deltas};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use crate::pb::uniswap::types::v1::Pools;

#[substreams::handlers::map]
pub fn kv_out(
    pools_created: Pools,                                /* map_pools_created */
    tx_count_deltas: Deltas<DeltaBigInt>,                /* store_total_tx_counts deltas */
    swaps_volume_deltas: Deltas<DeltaBigDecimal>,        /* store_swaps_volume */
    derived_tvl_deltas: Deltas<DeltaBigDecimal>,         /* store_derived_tvl */
) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    kv::pool_feetier_create(&mut kv_ops, &pools_created);
    kv::pool_day_data_create(&mut kv_ops, &tx_count_deltas);
    kv::pool_day_data_update(&mut kv_ops, &swaps_volume_deltas, &derived_tvl_deltas);

    Ok(kv_ops)
}