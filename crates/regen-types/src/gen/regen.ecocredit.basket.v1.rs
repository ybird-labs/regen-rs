// @generated
/// BasketCredit represents the information for a credit batch inside a basket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasketCredit {
    /// batch_denom is the unique ID of the credit batch.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// amount is the number of credits being put into or taken out of the basket.
    /// Decimal values are acceptable within the precision of the corresponding
    ///   credit type for this batch.
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for BasketCredit {
const NAME: &'static str = "BasketCredit";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// DateCriteria represents the information for credit acceptance in a basket.
/// At most, only one of the values should be set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateCriteria {
    /// min_start_date (optional) is the earliest start date for batches of credits
    /// allowed into the basket. At most only one of `start_date_window`,
    /// `min_start_date`, and `years_in_the_past` can be set for a basket.
    #[prost(message, optional, tag="1")]
    pub min_start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// start_date_window (optional) is a duration of time measured into the past
    /// which sets a cutoff for batch start dates when adding new credits to the
    /// basket. Based on the current block timestamp, credits whose start date is
    /// before `block_timestamp - start_date_window` will not be allowed into the
    /// basket. At most only one of `start_date_window`, `min_start_date`, and
    /// `years_in_the_past` can be set for a basket.
    #[prost(message, optional, tag="2")]
    pub start_date_window: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    /// years_in_the_past (optional) is the number of years into the past which
    /// sets a cutoff for the batch start dates when adding new credits to the
    /// basket. Based on the current block timestamp, credits whose start date year
    /// is less than `block_timestamp_year - years_in_the_past` will not be allowed
    /// into the basket. At most only one of `start_date_window`, `min_start_date`,
    /// and `years_in_the_past` can be set for a basket.
    ///
    /// Since Revision 1
    #[prost(uint32, tag="3")]
    pub years_in_the_past: u32,
}
impl ::prost::Name for DateCriteria {
const NAME: &'static str = "DateCriteria";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// EventCreate is an event emitted when a basket is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreate {
    /// basket_denom is the basket bank denom.
    #[prost(string, tag="1")]
    pub basket_denom: ::prost::alloc::string::String,
    /// curator is the address of the basket curator who is able to change certain
    /// basket settings.
    ///
    /// Deprecated (Since Revision 1): This field is still populated and will be
    /// removed in the next version.
    #[deprecated]
    #[prost(string, tag="2")]
    pub curator: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreate {
const NAME: &'static str = "EventCreate";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// EventPut is an event emitted when credits are put into a basket in return for
/// basket tokens.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventPut {
    /// owner is the owner of the credits put into the basket.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// basket_denom is the basket bank denom that the credits were added to.
    #[prost(string, tag="2")]
    pub basket_denom: ::prost::alloc::string::String,
    /// credits are the credits that were added to the basket.
    ///
    /// Deprecated (Since Revision 1): This field is still populated and will be
    /// removed in the next version.
    #[deprecated]
    #[prost(message, repeated, tag="3")]
    pub credits: ::prost::alloc::vec::Vec<BasketCredit>,
    /// amount is the integer number of basket tokens converted from credits.
    ///
    /// Deprecated (Since Revision 1): This field is still populated and will be
    /// removed in the next version.
    #[deprecated]
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventPut {
const NAME: &'static str = "EventPut";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// EventTake is an event emitted when credits are taken from a basket starting
/// from the oldest credits first.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTake {
    /// owner is the owner of the credits taken from the basket.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// basket_denom is the basket bank denom that credits were taken from.
    #[prost(string, tag="2")]
    pub basket_denom: ::prost::alloc::string::String,
    /// credits are the credits that were taken from the basket.
    ///
    /// Deprecated (Since Revision 1): This field is still populated and will be
    /// removed in the next version.
    #[deprecated]
    #[prost(message, repeated, tag="3")]
    pub credits: ::prost::alloc::vec::Vec<BasketCredit>,
    /// amount is the integer number of basket tokens converted to credits.
    ///
    /// Deprecated (Since Revision 1): This field is still populated and will be
    /// removed in the next version.
    #[deprecated]
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventTake {
const NAME: &'static str = "EventTake";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// EventUpdateCurator is an event emitted when the basket curator is updated.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateCurator {
    /// denom is the basket denom.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateCurator {
const NAME: &'static str = "EventUpdateCurator";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// EventUpdateDateCriteria is an event emitted when the basket date criteria is
/// updated.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateDateCriteria {
    /// denom is the basket denom.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateDateCriteria {
const NAME: &'static str = "EventUpdateDateCriteria";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// Basket represents a basket in state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Basket {
    /// id is the uint64 ID of the basket. It is used internally for reducing
    /// storage space.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// basket_denom is the basket bank denom formed from name and credit type with
    /// the format `eco.<prefix><credit_type_abbrev>.<name>` where prefix is the
    /// prefix of the bank denom exponent, a standard SI unit derived from credit
    /// type precision, and mapped as follows:
    ///    0 - no prefix
    ///    1 - d (deci)
    ///    2 - c (centi)
    ///    3 - m (milli)
    ///    6 - u (micro)
    ///    9 - n (nano)
    ///    12 - p (pico)
    ///    15 - f (femto)
    ///    18 - a (atto)
    ///    21 - z (zepto)
    ///    24 - y (yocto)
    #[prost(string, tag="2")]
    pub basket_denom: ::prost::alloc::string::String,
    /// name is the unique name of the basket specified in MsgCreate. Basket
    /// names must be unique across all credit types and choices of exponent
    /// above and beyond the uniqueness constraint on basket_denom.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// disable_auto_retire indicates whether or not the credits will be retired
    /// upon withdraw from the basket.
    #[prost(bool, tag="4")]
    pub disable_auto_retire: bool,
    /// credit_type_abbrev is the abbreviation of the credit type this basket is
    /// able to hold.
    #[prost(string, tag="5")]
    pub credit_type_abbrev: ::prost::alloc::string::String,
    /// date_criteria is the date criteria for batches admitted to the basket.
    #[prost(message, optional, tag="6")]
    pub date_criteria: ::core::option::Option<DateCriteria>,
    /// Deprecated (Since Revision 1): This field is no longer used and will be
    /// removed in the next version. The value of credit type precision is always
    /// used as the exponent when converting credits to/from basket tokens. This
    /// field will be set to the value of credit type precision until removed.
    #[deprecated]
    #[prost(uint32, tag="7")]
    pub exponent: u32,
    /// curator is the address of the basket curator who is able to change certain
    /// basket settings.
    ///
    /// Since Revision 1
    #[prost(bytes="vec", tag="8")]
    pub curator: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Basket {
const NAME: &'static str = "Basket";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// BasketClass describes a credit class that can be deposited in a basket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasketClass {
    /// basket_id is the ID of the basket
    #[prost(uint64, tag="1")]
    pub basket_id: u64,
    /// class_id is the id of the credit class that is allowed to be deposited in
    /// the basket
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for BasketClass {
const NAME: &'static str = "BasketClass";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// BasketBalance stores the amount of credits from a batch in a basket
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasketBalance {
    /// basket_id is the ID of the basket
    #[prost(uint64, tag="1")]
    pub basket_id: u64,
    /// batch_denom is the denom of the credit batch
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// balance is the amount of ecocredits held in the basket
    #[prost(string, tag="3")]
    pub balance: ::prost::alloc::string::String,
    /// batch_start_date is the start date of the batch. This field is used
    /// to create an index which is used to remove the oldest credits first.
    #[prost(message, optional, tag="4")]
    pub batch_start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for BasketBalance {
const NAME: &'static str = "BasketBalance";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// BasketFee is the basket creation fee. If not set, a basket creation fee is
/// not required. This table is controlled via governance.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasketFee {
    /// fee is the basket creation fee. If not set, a basket creation fee is not
    /// required.
    #[prost(message, optional, tag="1")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for BasketFee {
const NAME: &'static str = "BasketFee";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketRequest is the Query/Basket request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketRequest {
    /// basket_denom represents the denom of the basket to query.
    #[prost(string, tag="1")]
    pub basket_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBasketRequest {
const NAME: &'static str = "QueryBasketRequest";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketResponse is the Query/Basket response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketResponse {
    /// basket is the queried basket.
    ///
    /// Deprecated (Since Revision 1): This field is still populated using Basket
    /// but will be updated to use BasketInfo in the next version. In the meantime
    /// basket_info is available using BasketInfo.
    #[deprecated]
    #[prost(message, optional, tag="1")]
    pub basket: ::core::option::Option<Basket>,
    /// classes are the credit classes that can be deposited in the basket.
    #[prost(string, repeated, tag="2")]
    pub classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// basket_info is the queried basket.
    ///
    /// Since Revision 1
    #[prost(message, optional, tag="3")]
    pub basket_info: ::core::option::Option<BasketInfo>,
}
impl ::prost::Name for QueryBasketResponse {
const NAME: &'static str = "QueryBasketResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketsRequest is the Query/Baskets request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBasketsRequest {
const NAME: &'static str = "QueryBasketsRequest";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketsResponse is the Query/Baskets response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketsResponse {
    /// baskets are the fetched baskets.
    ///
    /// Deprecated (Since Revision 1): This field is still populated using Basket
    /// but will be updated to use BasketInfo in the next version. In the meantime
    /// baskets_info is available using BasketInfo.
    #[deprecated]
    #[prost(message, repeated, tag="1")]
    pub baskets: ::prost::alloc::vec::Vec<Basket>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
    /// baskets_info are the fetched baskets.
    ///
    /// Since Revision 1
    #[prost(message, repeated, tag="3")]
    pub baskets_info: ::prost::alloc::vec::Vec<BasketInfo>,
}
impl ::prost::Name for QueryBasketsResponse {
const NAME: &'static str = "QueryBasketsResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketBalancesRequest is the Query/BasketBalances request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketBalancesRequest {
    /// basket_denom is the denom of the basket.
    #[prost(string, tag="1")]
    pub basket_denom: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBasketBalancesRequest {
const NAME: &'static str = "QueryBasketBalancesRequest";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketBalancesResponse is the Query/BasketBalances response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketBalancesResponse {
    /// balances is a list of credit balances in the basket.
    ///
    /// Deprecated (Since Revision 1): This field is still populated using
    /// BasketBalance but will be updated to use BasketBalanceInfo in the next
    /// version. In the meantime baskets_info is available using BasketBalanceInfo.
    #[deprecated]
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<BasketBalance>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
    /// balances_info is a list of credit balances in the basket.
    ///
    /// Since Revision 1
    #[prost(message, repeated, tag="3")]
    pub balances_info: ::prost::alloc::vec::Vec<BasketBalanceInfo>,
}
impl ::prost::Name for QueryBasketBalancesResponse {
const NAME: &'static str = "QueryBasketBalancesResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketBalanceRequest is the Query/BasketBalance request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketBalanceRequest {
    /// basket_denom is the denom of the basket.
    #[prost(string, tag="1")]
    pub basket_denom: ::prost::alloc::string::String,
    /// batch_denom is the denom of the credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBasketBalanceRequest {
const NAME: &'static str = "QueryBasketBalanceRequest";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketBalanceResponse is the Query/BasketBalance response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketBalanceResponse {
    /// balance is the amount of the queried credit batch in the basket.
    #[prost(string, tag="1")]
    pub balance: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBasketBalanceResponse {
const NAME: &'static str = "QueryBasketBalanceResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// BasketInfo is the human-readable basket information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasketInfo {
    /// basket_denom is the basket bank denom.
    #[prost(string, tag="1")]
    pub basket_denom: ::prost::alloc::string::String,
    /// name is the unique name of the basket specified in MsgCreate. Basket
    /// names must be unique across all credit types and choices of exponent
    /// above and beyond the uniqueness constraint on basket_denom.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// disable_auto_retire indicates whether or not the credits will be retired
    /// upon withdraw from the basket.
    #[prost(bool, tag="3")]
    pub disable_auto_retire: bool,
    /// credit_type_abbrev is the abbreviation of the credit type this basket is
    /// able to hold.
    #[prost(string, tag="4")]
    pub credit_type_abbrev: ::prost::alloc::string::String,
    /// date_criteria is the date criteria for batches admitted to the basket.
    #[prost(message, optional, tag="5")]
    pub date_criteria: ::core::option::Option<DateCriteria>,
    /// exponent is the exponent for converting credits to/from basket tokens.
    #[prost(uint32, tag="6")]
    pub exponent: u32,
    /// curator is the address of the basket curator who is able to change certain
    /// basket settings.
    #[prost(string, tag="7")]
    pub curator: ::prost::alloc::string::String,
}
impl ::prost::Name for BasketInfo {
const NAME: &'static str = "BasketInfo";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// BasketBalanceInfo is the human-readable basket balance information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasketBalanceInfo {
    /// batch_denom is the denom of the credit batch
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// balance is the amount of ecocredits held in the basket
    #[prost(string, tag="2")]
    pub balance: ::prost::alloc::string::String,
}
impl ::prost::Name for BasketBalanceInfo {
const NAME: &'static str = "BasketBalanceInfo";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketFeeRequest is the Query/BasketFee request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketFeeRequest {
}
impl ::prost::Name for QueryBasketFeeRequest {
const NAME: &'static str = "QueryBasketFeeRequest";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// QueryBasketFeeResponse is the Query/BasketFee response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBasketFeeResponse {
    /// fee is the basket creation fee. If not set, a basket creation fee is not
    /// required.
    #[prost(message, optional, tag="1")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryBasketFeeResponse {
const NAME: &'static str = "QueryBasketFeeResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgCreateBasket is the Msg/CreateBasket request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreate {
    /// curator is the address of the basket curator who is able to change certain
    /// basket settings.
    #[prost(string, tag="1")]
    pub curator: ::prost::alloc::string::String,
    /// name will be used to together with prefix to create a bank denom for this
    /// basket token. It can be between 3-8 alphanumeric characters, with the
    /// first character being alphabetic.
    ///
    /// The bank denom will be formed from name and credit type with the format
    /// `eco.<prefix><credit_type_abbrev>.<name>` where prefix is the prefix of
    /// a standard SI unit derived from credit type precision.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// description is a human-readable description of the basket denom that should
    /// be at most 256 characters.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Deprecated (Since Revision 1): This field is no longer used and will be
    /// removed in the next version. The value of credit type precision is always
    /// used as the exponent when determining the prefix for basket denom, defining
    /// bank denom metadata, and converting credits to/from basket tokens.
    #[deprecated]
    #[prost(uint32, tag="4")]
    pub exponent: u32,
    /// disable_auto_retire allows auto-retirement to be disabled.
    /// The credits will be auto-retired if disable_auto_retire is
    /// false unless the credits were previously put into the basket by the
    /// address picking them from the basket, in which case they will remain
    /// tradable.
    #[prost(bool, tag="5")]
    pub disable_auto_retire: bool,
    /// credit_type_abbrev is the abbreviation of the credit type this basket is
    /// able to hold.
    #[prost(string, tag="6")]
    pub credit_type_abbrev: ::prost::alloc::string::String,
    /// allowed_classes are the credit classes allowed to be put in the basket
    #[prost(string, repeated, tag="7")]
    pub allowed_classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// date_criteria is the date criteria for batches admitted to the basket.
    /// At most, only one of the date criteria fields can be set.
    #[prost(message, optional, tag="8")]
    pub date_criteria: ::core::option::Option<DateCriteria>,
    /// fee is the basket creation fee. A fee is not required if no fee exists
    /// in the basket fee parameter. The fee must be greater than or equal to the
    /// fee param. The curator will be charged the amount specified in the fee
    /// parameter, even if a greater amount is provided.
    ///
    /// Note (Since Revision 1): Although this field supports a list of fees, the
    /// basket creator must provide no more than one fee (i.e. one Coin in a list
    /// of Coins). Providing more than one fee will fail basic message validation.
    /// This field will be updated to a single fee rather than a list of fees in
    /// the next version to reflect these requirements.
    #[prost(message, repeated, tag="9")]
    pub fee: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgCreate {
const NAME: &'static str = "MsgCreate";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgCreateBasketResponse is the Msg/CreateBasket response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateResponse {
    /// basket_denom is the unique denomination ID of the newly created basket.
    #[prost(string, tag="1")]
    pub basket_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateResponse {
const NAME: &'static str = "MsgCreateResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgAddToBasket is the Msg/AddToBasket request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPut {
    /// owner is the owner of credits being put into the basket.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// basket_denom is the basket denom to add credits to.
    #[prost(string, tag="2")]
    pub basket_denom: ::prost::alloc::string::String,
    /// credits are credits to add to the basket. If they do not match the basket's
    /// admission criteria, the operation will fail.
    #[prost(message, repeated, tag="3")]
    pub credits: ::prost::alloc::vec::Vec<BasketCredit>,
}
impl ::prost::Name for MsgPut {
const NAME: &'static str = "MsgPut";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgAddToBasketResponse is the Msg/AddToBasket response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPutResponse {
    /// amount_received is the integer amount of basket tokens received.
    #[prost(string, tag="1")]
    pub amount_received: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgPutResponse {
const NAME: &'static str = "MsgPutResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgTakeFromBasket is the Msg/TakeFromBasket request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTake {
    /// owner is the owner of the basket tokens.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// basket_denom is the basket bank denom to take credits from.
    #[prost(string, tag="2")]
    pub basket_denom: ::prost::alloc::string::String,
    /// amount is the integer number of basket tokens to convert into credits.
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// retirement_location is the optional retirement jurisdiction for the
    /// credits which will be used only if retire_on_take is true.
    ///
    /// Deprecated (Since Revision 1): This field will be removed in the next
    /// version in favor of retirement_jurisdiction. Only one of these need to be
    /// set and retirement_jurisdiction will be used if both are set.
    #[deprecated]
    #[prost(string, tag="4")]
    pub retirement_location: ::prost::alloc::string::String,
    /// retire_on_take is a boolean that dictates whether the ecocredits
    /// received in exchange for the basket tokens will be received as
    /// retired or tradable credits. If the basket has disable_auto_retire set to
    /// false, retire_on_take MUST be set to true, and a retirement jurisdiction
    /// must be provided.
    #[prost(bool, tag="5")]
    pub retire_on_take: bool,
    /// retirement_jurisdiction is the optional retirement jurisdiction for the
    /// credits which will be used only if retire_on_take is true.
    ///
    /// Since Revision 1
    #[prost(string, tag="6")]
    pub retirement_jurisdiction: ::prost::alloc::string::String,
    /// retirement_reason is any arbitrary string that specifies the reason for
    /// retiring credits. The reason will be included in EventRetire and is not
    /// stored in state.
    ///
    /// Since Revision 2
    #[prost(string, tag="7")]
    pub retirement_reason: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgTake {
const NAME: &'static str = "MsgTake";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgTakeFromBasketResponse is the Msg/TakeFromBasket response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTakeResponse {
    /// credits are the credits taken out of the basket.
    #[prost(message, repeated, tag="1")]
    pub credits: ::prost::alloc::vec::Vec<BasketCredit>,
}
impl ::prost::Name for MsgTakeResponse {
const NAME: &'static str = "MsgTakeResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgUpdateBasketFee is the Msg/UpdateBasketFee request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateBasketFee {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// fee is the basket creation fee. If not set, the basket creation fee will be
    /// removed and no fee will be required to create a basket.
    #[prost(message, optional, tag="2")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgUpdateBasketFee {
const NAME: &'static str = "MsgUpdateBasketFee";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgUpdateBasketFeeResponse is the Msg/UpdateBasketFee response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateBasketFeeResponse {
}
impl ::prost::Name for MsgUpdateBasketFeeResponse {
const NAME: &'static str = "MsgUpdateBasketFeeResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgUpdateCurator is the Msg/UpdateCurator request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateCurator {
    /// curator is the address of the basket curator.
    #[prost(string, tag="1")]
    pub curator: ::prost::alloc::string::String,
    /// denom is the unique identifier of the basket.
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// new_curator is the address of the account that will become the
    /// new curator of the basket.
    #[prost(string, tag="3")]
    pub new_curator: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateCurator {
const NAME: &'static str = "MsgUpdateCurator";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgUpdateCuratorResponse is the Msg/UpdateCurator response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateCuratorResponse {
}
impl ::prost::Name for MsgUpdateCuratorResponse {
const NAME: &'static str = "MsgUpdateCuratorResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgUpdateDateCriteria is the Msg/UpdateDateCriteria request type.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateDateCriteria {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// denom is the unique identifier of the basket.
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// new_date_criteria is the new date criteria for batches admitted to the
    /// basket. At most, only one of the date criteria fields can be set.
    #[prost(message, optional, tag="8")]
    pub new_date_criteria: ::core::option::Option<DateCriteria>,
}
impl ::prost::Name for MsgUpdateDateCriteria {
const NAME: &'static str = "MsgUpdateDateCriteria";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
/// MsgUpdateDateCriteriaResponse is the Msg/UpdateDateCriteria response type.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateDateCriteriaResponse {
}
impl ::prost::Name for MsgUpdateDateCriteriaResponse {
const NAME: &'static str = "MsgUpdateDateCriteriaResponse";
const PACKAGE: &'static str = "regen.ecocredit.basket.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.basket.v1.{}", Self::NAME)
            }}
include!("regen.ecocredit.basket.v1.serde.rs");
include!("regen.ecocredit.basket.v1.tonic.rs");
// @@protoc_insertion_point(module)