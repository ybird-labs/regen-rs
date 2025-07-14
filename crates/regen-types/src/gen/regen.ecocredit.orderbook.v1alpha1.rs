// @generated
/// BuyOrderSellOrderMatch defines the data the FIFO/price-time-priority matching
/// algorithm used to actually match buy and sell orders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyOrderSellOrderMatch {
    /// market_id defines the market within which this match exists.
    #[prost(uint64, tag="1")]
    pub market_id: u64,
    /// buy_order_id is the buy order ID.
    #[prost(uint64, tag="2")]
    pub buy_order_id: u64,
    /// sell_order_id is the sell order ID.
    #[prost(uint64, tag="3")]
    pub sell_order_id: u64,
    /// bid_price_complement is the the complement (^ operator) of the bid price
    /// encoded as a uint32 (which should have sufficient precision) - effectively
    /// ~price * 10^exponent (usually 10^6). The complement is used so that bids
    /// can be sorted high to low.
    #[prost(fixed32, tag="4")]
    pub bid_price_complement: u32,
    /// ask_price is the ask price encoded to a uint32. Ask prices are sorted low
    /// to high.
    #[prost(fixed32, tag="5")]
    pub ask_price: u32,
}
impl ::prost::Name for BuyOrderSellOrderMatch {
const NAME: &'static str = "BuyOrderSellOrderMatch";
const PACKAGE: &'static str = "regen.ecocredit.orderbook.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.orderbook.v1alpha1.{}", Self::NAME)
            }}
/// BuyOrderClassSelector indexes a buy order with class selector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyOrderClassSelector {
    /// buy_order_id is the buy order ID.
    #[prost(uint64, tag="1")]
    pub buy_order_id: u64,
    /// class_id is the class ID.
    #[prost(uint64, tag="2")]
    pub class_id: u64,
    /// project_location is the project location in the selector's criteria.
    #[prost(string, tag="3")]
    pub project_location: ::prost::alloc::string::String,
    /// min_start_date is the minimum start date in the selector's criteria.
    #[prost(message, optional, tag="4")]
    pub min_start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// max_end_date is the maximum end date in the selector's criteria.
    #[prost(message, optional, tag="5")]
    pub max_end_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for BuyOrderClassSelector {
const NAME: &'static str = "BuyOrderClassSelector";
const PACKAGE: &'static str = "regen.ecocredit.orderbook.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.orderbook.v1alpha1.{}", Self::NAME)
            }}
/// BuyOrderProjectSelector indexes a buy order with project selector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyOrderProjectSelector {
    /// buy_order_id is the buy order ID.
    #[prost(uint64, tag="1")]
    pub buy_order_id: u64,
    /// project_id is the project ID.
    #[prost(uint64, tag="2")]
    pub project_id: u64,
    /// min_start_date is the minimum start date in the selector's criteria.
    #[prost(message, optional, tag="3")]
    pub min_start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// max_end_date is the maximum end date in the selector's criteria.
    #[prost(message, optional, tag="4")]
    pub max_end_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for BuyOrderProjectSelector {
const NAME: &'static str = "BuyOrderProjectSelector";
const PACKAGE: &'static str = "regen.ecocredit.orderbook.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.orderbook.v1alpha1.{}", Self::NAME)
            }}
/// BuyOrderBatchSelector indexes a buy order with batch selector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyOrderBatchSelector {
    /// buy_order_id is the buy order ID.
    #[prost(uint64, tag="1")]
    pub buy_order_id: u64,
    /// batch_id is the batch ID.
    #[prost(uint64, tag="2")]
    pub batch_id: u64,
}
impl ::prost::Name for BuyOrderBatchSelector {
const NAME: &'static str = "BuyOrderBatchSelector";
const PACKAGE: &'static str = "regen.ecocredit.orderbook.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.orderbook.v1alpha1.{}", Self::NAME)
            }}
include!("regen.ecocredit.orderbook.v1alpha1.serde.rs");
// @@protoc_insertion_point(module)