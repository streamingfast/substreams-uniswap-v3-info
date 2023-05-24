mod helper;
mod pb;

use crate::pb::blockmeta::BlockMeta;

use prost::Message;
use substreams_sink_kv::prelude::*;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn uniswap_info_v1_uniswapinfo(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = Poo::decode(&v[..]).expect("Failed to decode");

    return Ok(out.encode_to_vec());
}
