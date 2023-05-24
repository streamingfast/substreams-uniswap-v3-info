// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolDayDatasRequest {
    #[prost(int64, tag="1")]
    pub start_time: i64,
    #[prost(int64, tag="2")]
    pub skip: i64,
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
    #[prost(string, tag="1")]
    pub date: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tvl_usd: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
