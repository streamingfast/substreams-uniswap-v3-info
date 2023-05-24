// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolDayDatasRequest {
    #[prost(string, repeated, tag="3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolDayDatasResponse {
    #[prost(message, repeated, tag="1")]
    pub pool_days_data: ::prost::alloc::vec::Vec<PoolsDayData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsDayData {
    #[prost(uint32, tag="1")]
    pub date: u32,
    #[prost(double, tag="2")]
    pub volume_usd: f64,
    #[prost(double, tag="3")]
    pub tvl_usd: f64,
}
// @@protoc_insertion_point(module)
