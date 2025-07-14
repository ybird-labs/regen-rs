// @generated
/// EventSell is an event emitted when a sell order is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSell {
    /// sell_order_id is the unique identifier of the sell order that was created.
    #[prost(uint64, tag="1")]
    pub sell_order_id: u64,
}
impl ::prost::Name for EventSell {
const NAME: &'static str = "EventSell";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// EventBuyDirect is an event emitted when a direct buy order is processed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBuyDirect {
    /// sell_order_id is the unique identifier of the sell order that credits were
    /// purchased from.
    #[prost(uint64, tag="1")]
    pub sell_order_id: u64,
    /// seller is the address of the account that sold the credits.
    #[prost(string, tag="2")]
    pub seller: ::prost::alloc::string::String,
    /// seller_fee_paid is the amount of coins paid by the seller
    /// to the marketplace as a fee for facilitating the sale.
    #[prost(message, optional, tag="3")]
    pub seller_fee_paid: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// buyer is the address of the account that purchased the credits.
    #[prost(string, tag="4")]
    pub buyer: ::prost::alloc::string::String,
    /// buyer_fee_paid is the amount of coins paid by the buyer
    /// to the marketplace as a fee for facilitating the sale.
    #[prost(message, optional, tag="5")]
    pub buyer_fee_paid: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventBuyDirect {
const NAME: &'static str = "EventBuyDirect";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// EventUpdateSellOrder is an event emitted when a sell order is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateSellOrder {
    ///   sell_order_id is the unique identifier of the sell order that was updated.
    #[prost(uint64, tag="1")]
    pub sell_order_id: u64,
}
impl ::prost::Name for EventUpdateSellOrder {
const NAME: &'static str = "EventUpdateSellOrder";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// EventCancelSellOrder is an event emitted when a sell order is cancelled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelSellOrder {
    /// sell_order_id is the unique identifier of the sell order that was
    /// cancelled.
    #[prost(uint64, tag="1")]
    pub sell_order_id: u64,
}
impl ::prost::Name for EventCancelSellOrder {
const NAME: &'static str = "EventCancelSellOrder";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// EventAllowDenom is an event emitted when a new denom is added for use in the
/// marketplace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAllowDenom {
    /// denom is the bank denom (e.g. ibc/GLKHDSG423SGS) added to the list of
    /// allowed denoms for use in the marketplace.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventAllowDenom {
const NAME: &'static str = "EventAllowDenom";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// EventRemoveAllowedDenom is an event emitted when a denom is removed from use
/// in the marketplace.
///
/// Since Revision 1
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRemoveAllowedDenom {
    /// denom is the bank denom (e.g. ibc/GLKHDSG423SGS) removed from the list of
    /// allowed denoms for use in the marketplace.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventRemoveAllowedDenom {
const NAME: &'static str = "EventRemoveAllowedDenom";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// SellOrder represents the information for a sell order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellOrder {
    /// id is the unique ID of sell order.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// seller is the address of the account that is selling credits.
    #[prost(bytes="vec", tag="2")]
    pub seller: ::prost::alloc::vec::Vec<u8>,
    /// batch_key is the table row identifier of the credit batch used internally
    /// for efficient lookups. This links a sell order to a credit batch.
    #[prost(uint64, tag="3")]
    pub batch_key: u64,
    /// quantity is the decimal quantity of credits being sold.
    #[prost(string, tag="4")]
    pub quantity: ::prost::alloc::string::String,
    /// market_id is the market in which this sell order exists and specifies
    /// the bank_denom that ask_amount corresponds to forming the ask_price.
    #[prost(uint64, tag="5")]
    pub market_id: u64,
    /// ask_amount is the integer amount (encoded as a string) that the seller is
    /// asking for each credit unit of the batch. Each credit unit of the batch
    /// will be sold for at least the ask_amount. The ask_amount corresponds to the
    /// Market.denom to form the ask price.
    #[prost(string, tag="6")]
    pub ask_amount: ::prost::alloc::string::String,
    /// disable_auto_retire disables auto-retirement of credits which allows a
    /// buyer to disable auto-retirement in their buy order enabling them to
    /// resell the credits to another buyer.
    #[prost(bool, tag="7")]
    pub disable_auto_retire: bool,
    /// expiration is an optional timestamp when the sell order expires. When the
    /// expiration time is reached, the sell order is removed from state.
    #[prost(message, optional, tag="9")]
    pub expiration: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// maker indicates that this is a maker order, meaning that when it hit
    /// the order book, there were no matching buy orders.
    #[prost(bool, tag="10")]
    pub maker: bool,
}
impl ::prost::Name for SellOrder {
const NAME: &'static str = "SellOrder";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// AllowedDenom represents the information for an allowed ask/bid denom.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedDenom {
    /// denom is the bank denom to allow (ex. ibc/GLKHDSG423SGS)
    #[prost(string, tag="1")]
    pub bank_denom: ::prost::alloc::string::String,
    /// display_denom is the denom to display to the user and is informational.
    /// Because the denom is likely an IBC denom, this should be chosen by
    /// governance to represent the consensus trusted name of the denom.
    #[prost(string, tag="2")]
    pub display_denom: ::prost::alloc::string::String,
    /// exponent is the exponent that relates the denom to the display_denom and is
    /// informational
    #[prost(uint32, tag="3")]
    pub exponent: u32,
}
impl ::prost::Name for AllowedDenom {
const NAME: &'static str = "AllowedDenom";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// Market describes a distinctly processed market between a credit type and
/// allowed bank denom. Each market has its own precision in the order book
/// and is processed independently of other markets. Governance must enable
/// markets one by one. Every additional enabled market potentially adds more
/// processing overhead to the blockchain and potentially weakens liquidity in
/// competing markets. For instance, enabling side by side USD/Carbon and
/// EUR/Carbon markets may have the end result that each market individually has
/// less liquidity and longer settlement times. Such decisions should be taken
/// with care.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Market {
    /// id is the unique ID of the market.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// credit_type_abbrev is the abbreviation of the credit type.
    #[prost(string, tag="2")]
    pub credit_type_abbrev: ::prost::alloc::string::String,
    /// bank_denom is an allowed bank denom.
    #[prost(string, tag="3")]
    pub bank_denom: ::prost::alloc::string::String,
    /// precision_modifier is an optional modifier used to convert arbitrary
    /// precision integer bank amounts to uint32 values used for sorting in the
    /// order book. Given an arbitrary precision integer x, its uint32 conversion
    /// will be x / 10^precision_modifier using round half away from zero
    /// rounding.
    ///
    /// uint32 values range from 0 to 4,294,967,295.
    /// This allows for a full 9 digits of precision. In most real world markets
    /// this amount of precision is sufficient and most common downside -
    /// that some orders with very miniscule price differences may be ordered
    /// equivalently (because of rounding) - is acceptable.
    /// Note that this rounding will not affect settlement price which will
    /// always be done exactly.
    ///
    /// Given a USD stable coin with 6 decimal digits, a precision_modifier
    /// of 0 is probably acceptable as long as credits are always less than
    /// $4,294/unit. With precision down to $0.001 (a precision_modifier of 3
    /// in this case), prices can rise up to $4,294,000/unit. Either scenario
    /// is probably quite acceptable given that carbon prices are unlikely to
    /// rise above $1000/ton any time in the near future.
    ///
    /// If credit prices, exceed the maximum range of uint32 with this
    /// precision_modifier, orders with high prices will fail and governance
    /// will need to adjust precision_modifier to allow for higher prices in
    /// exchange for less precision at the lower end.
    #[prost(uint32, tag="4")]
    pub precision_modifier: u32,
}
impl ::prost::Name for Market {
const NAME: &'static str = "Market";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// FeeParams represents the marketplace fee parameters. Fees will be charged in the
/// same denom that the order is denominated in and deposited into the marketplace
/// fee pool, except when the denom is regen, in which case the fees will be
/// burned. Fees in the fee pool are expected to burned by governance in a manual
/// process unless governance agrees to a different approach.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeParams {
    /// buyer_percentage_fee is the decimal percentage fee charged to the buyer.
    /// The string 0.03 means a 3.0% fee.
    /// This fee will be added to the total price of a buy order and is denominated
    /// in the same denom as the buy order's bid denom.
    #[prost(string, tag="1")]
    pub buyer_percentage_fee: ::prost::alloc::string::String,
    /// seller_percentage_fee is the decimal percentage fee charged to the seller.
    /// The string 0.03 means a 3.0% fee.
    /// This fee will be subtracted from the total proceeds of a sell order distributed to the seller
    /// and is denominated in the same denom as the sell order's ask denom.
    #[prost(string, tag="2")]
    pub seller_percentage_fee: ::prost::alloc::string::String,
}
impl ::prost::Name for FeeParams {
const NAME: &'static str = "FeeParams";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QuerySellOrderRequest is the Query/SellOrder request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySellOrderRequest {
    /// sell_order_id is the id of the requested sell order.
    #[prost(uint64, tag="1")]
    pub sell_order_id: u64,
}
impl ::prost::Name for QuerySellOrderRequest {
const NAME: &'static str = "QuerySellOrderRequest";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QuerySellOrderResponse is the Query/SellOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySellOrderResponse {
    /// sell_order contains all information related to a sell order.
    #[prost(message, optional, tag="1")]
    pub sell_order: ::core::option::Option<SellOrderInfo>,
}
impl ::prost::Name for QuerySellOrderResponse {
const NAME: &'static str = "QuerySellOrderResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QuerySellOrdersRequest is the Query/SellOrders request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySellOrdersRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QuerySellOrdersRequest {
const NAME: &'static str = "QuerySellOrdersRequest";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QuerySellOrdersResponse is the Query/SellOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySellOrdersResponse {
    /// sell_orders is a list of sell orders.
    #[prost(message, repeated, tag="1")]
    pub sell_orders: ::prost::alloc::vec::Vec<SellOrderInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QuerySellOrdersResponse {
const NAME: &'static str = "QuerySellOrdersResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QuerySellOrdersByBatchRequest is the Query/SellOrdersByBatch
/// request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySellOrdersByBatchRequest {
    /// batch_denom is an ecocredit denom
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QuerySellOrdersByBatchRequest {
const NAME: &'static str = "QuerySellOrdersByBatchRequest";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QuerySellOrdersByBatchResponse is the Query/SellOrdersByBatch
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySellOrdersByBatchResponse {
    /// sell_orders is a list of sell orders.
    #[prost(message, repeated, tag="1")]
    pub sell_orders: ::prost::alloc::vec::Vec<SellOrderInfo>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QuerySellOrdersByBatchResponse {
const NAME: &'static str = "QuerySellOrdersByBatchResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QuerySellOrdersBySellerRequest is the Query/SellOrdersBySeller request
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySellOrdersBySellerRequest {
    /// seller is the address of the account that is selling credits.
    #[prost(string, tag="1")]
    pub seller: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QuerySellOrdersBySellerRequest {
const NAME: &'static str = "QuerySellOrdersBySellerRequest";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QuerySellOrdersBySellerResponse is the Query/SellOrdersBySellerResponse
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySellOrdersBySellerResponse {
    /// sell_orders is a list of sell orders.
    #[prost(message, repeated, tag="1")]
    pub sell_orders: ::prost::alloc::vec::Vec<SellOrderInfo>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QuerySellOrdersBySellerResponse {
const NAME: &'static str = "QuerySellOrdersBySellerResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QueryAllowedDenomsRequest is the Query/AllowedDenoms request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowedDenomsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAllowedDenomsRequest {
const NAME: &'static str = "QueryAllowedDenomsRequest";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// QueryAllowedDenomsResponse is the Query/AllowedDenoms response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowedDenomsResponse {
    /// allowed_denoms is a list of coin denoms allowed to use in the ask price of
    /// sell orders.
    #[prost(message, repeated, tag="1")]
    pub allowed_denoms: ::prost::alloc::vec::Vec<AllowedDenom>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAllowedDenomsResponse {
const NAME: &'static str = "QueryAllowedDenomsResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// SellOrderInfo is the human-readable sell order information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellOrderInfo {
    /// id is the unique ID of sell order.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// seller is the address of the account that is selling credits.
    #[prost(string, tag="2")]
    pub seller: ::prost::alloc::string::String,
    /// batch_denom is denom of the credit batch being sold.
    #[prost(string, tag="3")]
    pub batch_denom: ::prost::alloc::string::String,
    /// quantity is the decimal quantity of credits being sold.
    #[prost(string, tag="4")]
    pub quantity: ::prost::alloc::string::String,
    /// ask_denom is the denom used in the ask price of the sell order.
    #[prost(string, tag="5")]
    pub ask_denom: ::prost::alloc::string::String,
    /// ask_amount is the amount that the seller is asking for each credit unit of
    /// the batch. Each credit unit of the batch will be sold for at least the
    /// ask_amount.
    #[prost(string, tag="6")]
    pub ask_amount: ::prost::alloc::string::String,
    /// disable_auto_retire disables auto-retirement of credits which allows a
    /// buyer to disable auto-retirement in their buy order enabling them to
    /// resell the credits to another buyer.
    #[prost(bool, tag="7")]
    pub disable_auto_retire: bool,
    /// expiration is an optional timestamp when the sell order expires. When the
    /// expiration time is reached, the sell order is removed from state.
    #[prost(message, optional, tag="9")]
    pub expiration: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for SellOrderInfo {
const NAME: &'static str = "SellOrderInfo";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgSell is the Msg/Sell request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSell {
    /// seller is the address of the account that is selling credits.
    #[prost(string, tag="1")]
    pub seller: ::prost::alloc::string::String,
    /// orders are the sell orders being created.
    #[prost(message, repeated, tag="2")]
    pub orders: ::prost::alloc::vec::Vec<msg_sell::Order>,
}
/// Nested message and enum types in `MsgSell`.
pub mod msg_sell {
    /// Order is the content of a new sell order.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Order {
        /// batch_denom is the credit batch being sold.
        #[prost(string, tag="1")]
        pub batch_denom: ::prost::alloc::string::String,
        /// quantity is the quantity of credits being sold from this batch. If it is
        /// less then the balance of credits the owner has available at the time this
        /// sell order is matched, the quantity will be adjusted downwards to the
        /// owner's balance. However, if the balance of credits is less than this
        /// quantity at the time the sell order is created, the operation will fail.
        #[prost(string, tag="2")]
        pub quantity: ::prost::alloc::string::String,
        /// ask_price is the price the seller is asking for each unit of the
        /// batch_denom. Each credit unit of the batch will be sold for at least the
        /// ask_price or more.
        #[prost(message, optional, tag="3")]
        pub ask_price: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
        /// disable_auto_retire disables auto-retirement of credits which allows a
        /// buyer to disable auto-retirement in their buy order enabling them to
        /// resell the credits to another buyer.
        #[prost(bool, tag="4")]
        pub disable_auto_retire: bool,
        /// expiration is an optional timestamp when the sell order expires. When the
        /// expiration time is reached, the sell order is removed from state.
        #[prost(message, optional, tag="5")]
        pub expiration: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    }
impl ::prost::Name for Order {
const NAME: &'static str = "Order";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.MsgSell.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgSell {
const NAME: &'static str = "MsgSell";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgSellResponse is the Msg/Sell response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSellResponse {
    /// sell_order_ids are the sell order IDs of the newly created sell orders.
    #[prost(uint64, repeated, tag="1")]
    pub sell_order_ids: ::prost::alloc::vec::Vec<u64>,
}
impl ::prost::Name for MsgSellResponse {
const NAME: &'static str = "MsgSellResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgUpdateSellOrders is the Msg/UpdateSellOrders request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateSellOrders {
    /// seller is the address of the account that is selling credits.
    #[prost(string, tag="1")]
    pub seller: ::prost::alloc::string::String,
    /// updates are updates to existing sell orders.
    #[prost(message, repeated, tag="2")]
    pub updates: ::prost::alloc::vec::Vec<msg_update_sell_orders::Update>,
}
/// Nested message and enum types in `MsgUpdateSellOrders`.
pub mod msg_update_sell_orders {
    /// Update is an update to an existing sell order.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Update {
        /// sell_order_id is the ID of an existing sell order.
        #[prost(uint64, tag="1")]
        pub sell_order_id: u64,
        /// new_quantity is the updated quantity of credits available to sell.
        #[prost(string, tag="2")]
        pub new_quantity: ::prost::alloc::string::String,
        /// new_ask_price is the new ask price for this sell order
        #[prost(message, optional, tag="3")]
        pub new_ask_price: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
        /// disable_auto_retire updates the disable_auto_retire field in the sell
        /// order.
        #[prost(bool, tag="4")]
        pub disable_auto_retire: bool,
        /// new_expiration is an optional timestamp when the sell order expires. When
        /// the expiration time is reached, the sell order is removed from state.
        #[prost(message, optional, tag="5")]
        pub new_expiration: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    }
impl ::prost::Name for Update {
const NAME: &'static str = "Update";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.MsgUpdateSellOrders.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgUpdateSellOrders {
const NAME: &'static str = "MsgUpdateSellOrders";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgUpdateSellOrdersResponse is the Msg/UpdateSellOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateSellOrdersResponse {
}
impl ::prost::Name for MsgUpdateSellOrdersResponse {
const NAME: &'static str = "MsgUpdateSellOrdersResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgCancelSellOrder is the Msg/CancelSellOrder request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSellOrder {
    /// seller is the address of the account that created the sell order and is
    /// therefore authorized to cancel the sell order.
    #[prost(string, tag="1")]
    pub seller: ::prost::alloc::string::String,
    /// sell_order_id is the id of the seller order to cancel.
    #[prost(uint64, tag="2")]
    pub sell_order_id: u64,
}
impl ::prost::Name for MsgCancelSellOrder {
const NAME: &'static str = "MsgCancelSellOrder";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgCancelSellOrder is the Msg/CancelSellOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSellOrderResponse {
}
impl ::prost::Name for MsgCancelSellOrderResponse {
const NAME: &'static str = "MsgCancelSellOrderResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgBuyDirect is the Msg/BuyDirect request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBuyDirect {
    /// buyer is the address of the account that is buying credits.
    #[prost(string, tag="1")]
    pub buyer: ::prost::alloc::string::String,
    /// orders is a list of orders for ecocredits.
    #[prost(message, repeated, tag="2")]
    pub orders: ::prost::alloc::vec::Vec<msg_buy_direct::Order>,
}
/// Nested message and enum types in `MsgBuyDirect`.
pub mod msg_buy_direct {
    /// Order contains the information needed to purchase an ecocredit.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Order {
        /// sell_order_id is the sell order ID against which the buyer is trying
        /// to buy.
        #[prost(uint64, tag="2")]
        pub sell_order_id: u64,
        /// quantity is the quantity of credits to buy.
        #[prost(string, tag="3")]
        pub quantity: ::prost::alloc::string::String,
        /// bid_price is the price the buyer is willing to pay per credit.
        #[prost(message, optional, tag="4")]
        pub bid_price: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
        /// disable_auto_retire allows auto-retirement to be disabled. If it is set
        /// to true the credits will not auto-retire and can be resold assuming that
        /// the corresponding sell order has auto-retirement disabled. If the sell
        /// order hasn't disabled auto-retirement and the buy order tries to disable
        /// it, that buy order will fail.
        #[prost(bool, tag="5")]
        pub disable_auto_retire: bool,
        /// retirement_jurisdiction is the optional retirement jurisdiction for the
        /// credits which will be used only if disable_auto_retire is false.
        #[prost(string, tag="6")]
        pub retirement_jurisdiction: ::prost::alloc::string::String,
        /// retirement_reason is any arbitrary string that specifies the reason for
        /// retiring credits. The reason will be included in EventRetire and is not
        /// stored in state.
        ///
        /// Since Revision 1
        #[prost(string, tag="7")]
        pub retirement_reason: ::prost::alloc::string::String,
        /// max_fee_amount is the maximum amount of buyer side fees being paid to the marketplace.
        /// If the marketplace fees end up being greater than this amount, the transaction will fail.
        /// Fees are always paid in the same denomination as the bid price.
        ///
        /// Since Revision 3
        #[prost(message, optional, tag="8")]
        pub max_fee_amount: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    }
impl ::prost::Name for Order {
const NAME: &'static str = "Order";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.MsgBuyDirect.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgBuyDirect {
const NAME: &'static str = "MsgBuyDirect";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgBuyDirectResponse is the Msg/BuyDirect response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBuyDirectResponse {
}
impl ::prost::Name for MsgBuyDirectResponse {
const NAME: &'static str = "MsgBuyDirectResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgAddAllowedDenom is the Msg/AddAllowedDenom request type.
///
/// Since Revision 1
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddAllowedDenom {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// denom is the bank denom to allow (ex. ibc/GLKHDSG423SGS)
    #[prost(string, tag="2")]
    pub bank_denom: ::prost::alloc::string::String,
    /// display_denom is the denom to display to the user and is informational.
    /// Because the denom is likely an IBC denom, this should be chosen by
    /// governance to represent the consensus trusted name of the denom.
    #[prost(string, tag="3")]
    pub display_denom: ::prost::alloc::string::String,
    /// exponent is the exponent that relates the denom to the display_denom and is
    /// informational
    #[prost(uint32, tag="4")]
    pub exponent: u32,
}
impl ::prost::Name for MsgAddAllowedDenom {
const NAME: &'static str = "MsgAddAllowedDenom";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgAddAllowedDenomResponse is the Msg/AddAllowedDenom response type.
///
/// Since Revision 1
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddAllowedDenomResponse {
}
impl ::prost::Name for MsgAddAllowedDenomResponse {
const NAME: &'static str = "MsgAddAllowedDenomResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgRemoveAllowedDenom is the Msg/RemoveAllowedDenom request type.
///
/// Since Revision 1
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveAllowedDenom {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// denom is the denom to remove (ex. ibc/GLKHDSG423SGS)
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRemoveAllowedDenom {
const NAME: &'static str = "MsgRemoveAllowedDenom";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgRemoveAllowedDenomResponse is the Msg/RemoveAllowedDenom response type.
///
/// Since Revision 1
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveAllowedDenomResponse {
}
impl ::prost::Name for MsgRemoveAllowedDenomResponse {
const NAME: &'static str = "MsgRemoveAllowedDenomResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgSetFeeParams is the Msg/SetFeeParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovSetFeeParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// fees are the marketplace fees being set.
    #[prost(message, optional, tag="2")]
    pub fees: ::core::option::Option<FeeParams>,
}
impl ::prost::Name for MsgGovSetFeeParams {
const NAME: &'static str = "MsgGovSetFeeParams";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgSetFeeParamsResponse is the Msg/SetFeeParams response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovSetFeeParamsResponse {
}
impl ::prost::Name for MsgGovSetFeeParamsResponse {
const NAME: &'static str = "MsgGovSetFeeParamsResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgSendFromFeePool is the Msg/SendFromFeePool request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovSendFromFeePool {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// recipient is the address of the account that will receive the funds.
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// coins is the amount of coins to send from the fee pool.
    #[prost(message, repeated, tag="3")]
    pub coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgGovSendFromFeePool {
const NAME: &'static str = "MsgGovSendFromFeePool";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// MsgSendFromFeePoolResponse is the Msg/SendFromFeePool response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovSendFromFeePoolResponse {
}
impl ::prost::Name for MsgGovSendFromFeePoolResponse {
const NAME: &'static str = "MsgGovSendFromFeePoolResponse";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
/// AllowDenomProposal is a gov Content type for approving a denom for use in the
/// marketplace.
///
/// Deprecated (Since Revision 1): This message is no longer used and will be
/// removed in the next version. See MsgAddAllowedDenom.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowDenomProposal {
    /// title is the title of the proposal.
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// description is the description of the proposal.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// denom contains coin denom information that will be added to the
    /// list of allowed denoms for use in the marketplace.
    #[prost(message, optional, tag="3")]
    pub denom: ::core::option::Option<AllowedDenom>,
}
impl ::prost::Name for AllowDenomProposal {
const NAME: &'static str = "AllowDenomProposal";
const PACKAGE: &'static str = "regen.ecocredit.marketplace.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.marketplace.v1.{}", Self::NAME)
            }}
include!("regen.ecocredit.marketplace.v1.serde.rs");
include!("regen.ecocredit.marketplace.v1.tonic.rs");
// @@protoc_insertion_point(module)