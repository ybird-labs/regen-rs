// @generated
/// EventCreateClass is an event emitted when a credit class is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateClass {
    /// class_id is the unique ID of credit class.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// admin is the admin of the credit class.
    #[prost(string, tag="2")]
    pub admin: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateClass {
const NAME: &'static str = "EventCreateClass";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// EventCreateBatch is an event emitted when a credit batch is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateBatch {
    /// class_id is the unique ID of credit class.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// batch_denom is the unique ID of credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// issuer is the account address of the issuer of the credit batch.
    #[prost(string, tag="3")]
    pub issuer: ::prost::alloc::string::String,
    /// total_amount is the total number of credits in the credit batch.
    #[prost(string, tag="4")]
    pub total_amount: ::prost::alloc::string::String,
    /// start_date is the beginning of the period during which this credit batch
    /// was quantified and verified.
    #[prost(string, tag="5")]
    pub start_date: ::prost::alloc::string::String,
    /// end_date is the end of the period during which this credit batch was
    /// quantified and verified.
    #[prost(string, tag="6")]
    pub end_date: ::prost::alloc::string::String,
    /// project_location is the location of the project backing the credits in this
    /// batch. Full documentation can be found in MsgCreateBatch.project_location.
    #[prost(string, tag="7")]
    pub project_location: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateBatch {
const NAME: &'static str = "EventCreateBatch";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// EventReceive is an event emitted when credits are received either via
/// creation of a new batch, transfer of credits, or taking credits from a
/// basket. Each batch_denom created, transferred or taken from a baset will
/// result in a separate EventReceive for easy indexing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventReceive {
    /// sender is the sender of the credits in the case that this event is the
    /// result of a transfer. It will not be set when credits are received at
    /// initial issuance or taken from a basket.
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// recipient is the recipient of the credits.
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// batch_denom is the unique ID of credit batch.
    #[prost(string, tag="3")]
    pub batch_denom: ::prost::alloc::string::String,
    /// tradable_amount is the decimal number of tradable credits received.
    #[prost(string, tag="4")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the decimal number of retired credits received.
    #[prost(string, tag="5")]
    pub retired_amount: ::prost::alloc::string::String,
    /// basket_denom is the denom of the basket. when the basket_denom field is
    /// set, it indicates that this event was triggered by the transfer of credits
    /// from a basket. It will not be set if the credits were sent by a user, or by
    /// initial issuance.
    #[prost(string, tag="6")]
    pub basket_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventReceive {
const NAME: &'static str = "EventReceive";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// EventRetire is an event emitted when credits are retired. When credits are
/// retired from multiple batches in the same transaction, a separate event is
/// emitted for each batch_denom. This allows for easier indexing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRetire {
    /// retirer is the account which has done the "retiring". This will be the
    /// account receiving credits in the case that credits were retired upon
    /// issuance using Msg/CreateBatch or retired upon transfer using Msg/Send.
    #[prost(string, tag="1")]
    pub retirer: ::prost::alloc::string::String,
    /// batch_denom is the unique ID of credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// amount is the decimal number of credits that have been retired.
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// location is the location of the beneficiary or buyer of the retired
    /// credits. It is a string of the form
    /// <country-code>\[-<sub-national-code>[ <postal-code>]\], with the first two
    /// fields conforming to ISO 3166-2, and postal-code being up to 64
    /// alphanumeric characters.
    #[prost(string, tag="4")]
    pub location: ::prost::alloc::string::String,
}
impl ::prost::Name for EventRetire {
const NAME: &'static str = "EventRetire";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// EventCancel is an event emitted when credits are cancelled. When credits are
/// cancelled from multiple batches in the same transaction, a separate event is
/// emitted for each batch_denom. This allows for easier indexing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancel {
    /// canceller is the account which has cancelled the credits, which should be
    /// the holder of the credits.
    #[prost(string, tag="1")]
    pub canceller: ::prost::alloc::string::String,
    /// batch_denom is the unique ID of credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// amount is the decimal number of credits that have been cancelled.
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCancel {
const NAME: &'static str = "EventCancel";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// ClassInfo represents the high-level on-chain information for a credit class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassInfo {
    /// class_id is the unique ID of credit class.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// admin is the admin of the credit class.
    #[prost(string, tag="2")]
    pub admin: ::prost::alloc::string::String,
    /// issuers are the approved issuers of the credit class.
    #[prost(string, repeated, tag="3")]
    pub issuers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// metadata is any arbitrary metadata to attached to the credit class.
    #[prost(bytes="vec", tag="4")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
    /// credit_type describes the type of credit (e.g. carbon, biodiversity), as
    /// well as unit and precision.
    #[prost(message, optional, tag="5")]
    pub credit_type: ::core::option::Option<CreditType>,
    /// The number of batches issued in this credit class.
    #[prost(uint64, tag="6")]
    pub num_batches: u64,
}
impl ::prost::Name for ClassInfo {
const NAME: &'static str = "ClassInfo";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// BatchInfo represents the high-level on-chain information for a credit batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchInfo {
    /// class_id is the unique ID of credit class.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// batch_denom is the unique ID of credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// issuer is the issuer of the credit batch.
    #[prost(string, tag="3")]
    pub issuer: ::prost::alloc::string::String,
    /// total_amount is the total number of active credits in the credit batch.
    /// Some of the issued credits may be cancelled and will be removed from
    /// total_amount and tracked in amount_cancelled. total_amount and
    /// amount_cancelled will always sum to the original amount of credits that
    /// were issued.
    #[prost(string, tag="4")]
    pub total_amount: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the credit batch.
    #[prost(bytes="vec", tag="5")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
    /// amount_cancelled is the number of credits in the batch that have been
    /// cancelled, effectively undoing there issuance. The sum of total_amount and
    /// amount_cancelled will always sum to the original amount of credits that
    /// were issued.
    #[prost(string, tag="6")]
    pub amount_cancelled: ::prost::alloc::string::String,
    /// start_date is the beginning of the period during which this credit batch
    /// was quantified and verified.
    #[prost(message, optional, tag="7")]
    pub start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// end_date is the end of the period during which this credit batch was
    /// quantified and verified.
    #[prost(message, optional, tag="8")]
    pub end_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// project_location is the location of the project backing the credits in this
    /// batch. Full documentation can be found in MsgCreateBatch.project_location.
    #[prost(string, tag="9")]
    pub project_location: ::prost::alloc::string::String,
}
impl ::prost::Name for BatchInfo {
const NAME: &'static str = "BatchInfo";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// Params defines the updatable global parameters of the ecocredit module for
/// use with the x/params module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// credit_class_fee is the fixed fee charged on creation of a new credit class
    #[prost(message, repeated, tag="1")]
    pub credit_class_fee: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// allowed_class_creators is an allowlist defining the addresses with
    /// the required permissions to create credit classes
    #[prost(string, repeated, tag="2")]
    pub allowed_class_creators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// allowlist_enabled is a param that enables/disables the allowlist for credit
    /// creation
    #[prost(bool, tag="3")]
    pub allowlist_enabled: bool,
    /// credit_types is a list of definitions for credit types
    #[prost(message, repeated, tag="4")]
    pub credit_types: ::prost::alloc::vec::Vec<CreditType>,
    /// basket_creation_fee is the fee to create a new basket denom.
    #[prost(message, repeated, tag="5")]
    pub basket_creation_fee: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for Params {
const NAME: &'static str = "Params";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// CreditType defines the measurement unit/precision of a certain credit type
/// (e.g. carbon, biodiversity...)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditType {
    /// the type of credit (e.g. carbon, biodiversity, etc)
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// abbreviation is a 1-3 character uppercase abbreviation of the CreditType
    /// name, used in batch denominations within the CreditType. It must be unique.
    #[prost(string, tag="2")]
    pub abbreviation: ::prost::alloc::string::String,
    /// the measurement unit (e.g. kg, ton, etc)
    #[prost(string, tag="3")]
    pub unit: ::prost::alloc::string::String,
    /// the decimal precision
    #[prost(uint32, tag="4")]
    pub precision: u32,
}
impl ::prost::Name for CreditType {
const NAME: &'static str = "CreditType";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// CreditTypeSeq associates a sequence number with a credit type abbreviation.
/// This represents the number of credit classes created with that credit type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditTypeSeq {
    /// The credit type abbreviation
    #[prost(string, tag="1")]
    pub abbreviation: ::prost::alloc::string::String,
    /// The sequence number of classes of the credit type
    #[prost(uint64, tag="2")]
    pub seq_number: u64,
}
impl ::prost::Name for CreditTypeSeq {
const NAME: &'static str = "CreditTypeSeq";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// GenesisState defines ecocredit module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// Params contains the updateable global parameters for use with the x/params
    /// module
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// class_info is the list of credit class info.
    #[prost(message, repeated, tag="2")]
    pub class_info: ::prost::alloc::vec::Vec<ClassInfo>,
    /// batch_info is the list of credit batch info.
    #[prost(message, repeated, tag="3")]
    pub batch_info: ::prost::alloc::vec::Vec<BatchInfo>,
    /// sequences is the list of credit type sequence.
    #[prost(message, repeated, tag="4")]
    pub sequences: ::prost::alloc::vec::Vec<CreditTypeSeq>,
    /// balances is the list of credit batch tradable/retired units.
    #[prost(message, repeated, tag="5")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
    /// supplies is the list of credit batch tradable/retired supply.
    #[prost(message, repeated, tag="6")]
    pub supplies: ::prost::alloc::vec::Vec<Supply>,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// Balance represents tradable or retired units of a credit batch with an
/// account address, batch_denom, and balance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    /// address is the account address of the account holding credits.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// batch_denom is the unique ID of the credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// tradable_balance is the tradable balance of the credit batch.
    #[prost(string, tag="3")]
    pub tradable_balance: ::prost::alloc::string::String,
    /// retired_balance is the retired balance of the credit batch.
    #[prost(string, tag="4")]
    pub retired_balance: ::prost::alloc::string::String,
}
impl ::prost::Name for Balance {
const NAME: &'static str = "Balance";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// Supply represents a tradable or retired supply of a credit batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Supply {
    /// batch_denom is the unique ID of the credit batch.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// tradable_supply is the tradable supply of the credit batch.
    #[prost(string, tag="2")]
    pub tradable_supply: ::prost::alloc::string::String,
    /// retired_supply is the retired supply of the credit batch.
    #[prost(string, tag="3")]
    pub retired_supply: ::prost::alloc::string::String,
}
impl ::prost::Name for Supply {
const NAME: &'static str = "Supply";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryParamsRequest is the Query/Params request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
impl ::prost::Name for QueryParamsRequest {
const NAME: &'static str = "QueryParamsRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryParamsResponse is the Query/Params response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the ecocredit module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
const NAME: &'static str = "QueryParamsResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryClassesRequest is the Query/Classes request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryClassesRequest {
const NAME: &'static str = "QueryClassesRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryClassesResponse is the Query/Classes response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassesResponse {
    /// classes are the fetched credit classes.
    #[prost(message, repeated, tag="1")]
    pub classes: ::prost::alloc::vec::Vec<ClassInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryClassesResponse {
const NAME: &'static str = "QueryClassesResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryClassInfoRequest is the Query/ClassInfo request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassInfoRequest {
    /// class_id is the unique ID of credit class to query.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryClassInfoRequest {
const NAME: &'static str = "QueryClassInfoRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryClassInfoResponse is the Query/ClassInfo request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassInfoResponse {
    /// info is the ClassInfo for the credit class.
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<ClassInfo>,
}
impl ::prost::Name for QueryClassInfoResponse {
const NAME: &'static str = "QueryClassInfoResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryBatchesRequest is the Query/Batches request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesRequest {
    /// class_id is the unique ID of the credit class to query.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBatchesRequest {
const NAME: &'static str = "QueryBatchesRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryBatchesResponse is the Query/Batches response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesResponse {
    /// batches are the fetched credit batches within the class.
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<BatchInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBatchesResponse {
const NAME: &'static str = "QueryBatchesResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryBatchInfoRequest is the Query/BatchInfo request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchInfoRequest {
    /// batch_denom is the unique ID of credit batch to query.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBatchInfoRequest {
const NAME: &'static str = "QueryBatchInfoRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryBatchInfoResponse is the Query/BatchInfo response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchInfoResponse {
    /// info is the BatchInfo for the credit batch.
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<BatchInfo>,
}
impl ::prost::Name for QueryBatchInfoResponse {
const NAME: &'static str = "QueryBatchInfoResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryBalanceRequest is the Query/Balance request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// account is the address of the account whose balance is being queried.
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    /// batch_denom is the unique ID of credit batch balance to query.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBalanceRequest {
const NAME: &'static str = "QueryBalanceRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryBalanceResponse is the Query/Balance response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// tradable_amount is the decimal number of tradable credits.
    #[prost(string, tag="1")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the decimal number of retired credits.
    #[prost(string, tag="2")]
    pub retired_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBalanceResponse {
const NAME: &'static str = "QueryBalanceResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QuerySupplyRequest is the Query/Supply request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyRequest {
    /// batch_denom is the unique ID of credit batch to query.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySupplyRequest {
const NAME: &'static str = "QuerySupplyRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QuerySupplyResponse is the Query/Supply response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyResponse {
    /// tradable_supply is the decimal number of tradable credits in the batch
    /// supply.
    #[prost(string, tag="1")]
    pub tradable_supply: ::prost::alloc::string::String,
    /// retired_supply is the decimal number of retired credits in the batch
    /// supply.
    #[prost(string, tag="2")]
    pub retired_supply: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySupplyResponse {
const NAME: &'static str = "QuerySupplyResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryCreditTypesRequest is the Query/Credit_Types request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCreditTypesRequest {
}
impl ::prost::Name for QueryCreditTypesRequest {
const NAME: &'static str = "QueryCreditTypesRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// QueryCreditTypesRequest is the Query/Credit_Types response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCreditTypesResponse {
    /// list of credit types
    #[prost(message, repeated, tag="1")]
    pub credit_types: ::prost::alloc::vec::Vec<CreditType>,
}
impl ::prost::Name for QueryCreditTypesResponse {
const NAME: &'static str = "QueryCreditTypesResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgCreateClass is the Msg/CreateClass request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClass {
    /// admin is the address of the account that created the credit class.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// issuers are the account addresses of the approved issuers.
    #[prost(string, repeated, tag="2")]
    pub issuers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// metadata is any arbitrary metadata to attached to the credit class.
    #[prost(bytes="vec", tag="3")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
    /// credit_type_name describes the type of credit (e.g. "carbon",
    /// "biodiversity").
    #[prost(string, tag="4")]
    pub credit_type_name: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateClass {
const NAME: &'static str = "MsgCreateClass";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgCreateClassResponse is the Msg/CreateClass response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClassResponse {
    /// class_id is the unique ID of the newly created credit class.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateClassResponse {
const NAME: &'static str = "MsgCreateClassResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgCreateBatch is the Msg/CreateBatch request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBatch {
    /// issuer is the address of the batch issuer.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// class_id is the unique ID of the class.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// issuance are the credits issued in the batch.
    #[prost(message, repeated, tag="3")]
    pub issuance: ::prost::alloc::vec::Vec<msg_create_batch::BatchIssuance>,
    /// metadata is any arbitrary metadata attached to the credit batch.
    #[prost(bytes="vec", tag="4")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
    /// start_date is the beginning of the period during which this credit batch
    /// was quantified and verified.
    #[prost(message, optional, tag="5")]
    pub start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// end_date is the end of the period during which this credit batch was
    /// quantified and verified.
    #[prost(message, optional, tag="6")]
    pub end_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// project_location is the location of the project backing the credits in this
    /// batch. It is a string of the form
    /// <country-code>\[-<sub-national-code>[ <postal-code>]\], with the first two
    /// fields conforming to ISO 3166-2, and postal-code being up to 64
    /// alphanumeric characters. country-code is required, while sub-national-code
    /// and postal-code can be added for increasing precision.
    #[prost(string, tag="7")]
    pub project_location: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MsgCreateBatch`.
pub mod msg_create_batch {
    /// BatchIssuance represents the issuance of some credits in a batch to a
    /// single recipient.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BatchIssuance {
        /// recipient is the account of the recipient.
        #[prost(string, tag="1")]
        pub recipient: ::prost::alloc::string::String,
        /// tradable_amount is the number of credits in this issuance that can be
        /// traded by this recipient. Decimal values are acceptable.
        #[prost(string, tag="2")]
        pub tradable_amount: ::prost::alloc::string::String,
        /// retired_amount is the number of credits in this issuance that are
        /// effectively retired by the issuer on receipt. Decimal values are
        /// acceptable.
        #[prost(string, tag="3")]
        pub retired_amount: ::prost::alloc::string::String,
        /// retirement_location is the location of the beneficiary or buyer of the
        /// retired credits. This must be provided if retired_amount is positive. It
        /// is a string of the form
        /// <country-code>\[-<sub-national-code>[ <postal-code>]\], with the first two
        /// fields conforming to ISO 3166-2, and postal-code being up to 64
        /// alphanumeric characters.
        #[prost(string, tag="4")]
        pub retirement_location: ::prost::alloc::string::String,
    }
impl ::prost::Name for BatchIssuance {
const NAME: &'static str = "BatchIssuance";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.MsgCreateBatch.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgCreateBatch {
const NAME: &'static str = "MsgCreateBatch";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgCreateBatchResponse is the Msg/CreateBatch response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBatchResponse {
    /// batch_denom is the unique denomination ID of the newly created batch.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateBatchResponse {
const NAME: &'static str = "MsgCreateBatchResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgSend is the Msg/Send request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    /// sender is the address of the account sending credits.
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// sender is the address of the account receiving credits.
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// credits are the credits being sent.
    #[prost(message, repeated, tag="3")]
    pub credits: ::prost::alloc::vec::Vec<msg_send::SendCredits>,
}
/// Nested message and enum types in `MsgSend`.
pub mod msg_send {
    /// SendCredits specifies a batch and the number of credits being transferred.
    /// This is split into tradable credits, which will remain tradable on receipt,
    /// and retired credits, which will be retired on receipt.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendCredits {
        /// batch_denom is the unique ID of the credit batch.
        #[prost(string, tag="1")]
        pub batch_denom: ::prost::alloc::string::String,
        /// tradable_amount is the number of credits in this transfer that can be
        /// traded by the recipient. Decimal values are acceptable within the
        /// precision returned by Query/Precision.
        #[prost(string, tag="2")]
        pub tradable_amount: ::prost::alloc::string::String,
        /// retired_amount is the number of credits in this transfer that are
        /// effectively retired by the issuer on receipt. Decimal values are
        /// acceptable within the precision returned by Query/Precision.
        #[prost(string, tag="3")]
        pub retired_amount: ::prost::alloc::string::String,
        /// retirement_location is the location of the beneficiary or buyer of the
        /// retired credits. This must be provided if retired_amount is positive. It
        /// is a string of the form
        /// <country-code>\[-<sub-national-code>[ <postal-code>]\], with the first two
        /// fields conforming to ISO 3166-2, and postal-code being up to 64
        /// alphanumeric characters.
        #[prost(string, tag="4")]
        pub retirement_location: ::prost::alloc::string::String,
    }
impl ::prost::Name for SendCredits {
const NAME: &'static str = "SendCredits";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.MsgSend.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgSend {
const NAME: &'static str = "MsgSend";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgSendResponse is the Msg/Send response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {
}
impl ::prost::Name for MsgSendResponse {
const NAME: &'static str = "MsgSendResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgRetire is the Msg/Retire request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRetire {
    /// holder is the credit holder address.
    #[prost(string, tag="1")]
    pub holder: ::prost::alloc::string::String,
    /// credits are the credits being retired.
    #[prost(message, repeated, tag="2")]
    pub credits: ::prost::alloc::vec::Vec<msg_retire::RetireCredits>,
    /// location is the location of the beneficiary or buyer of the retired
    /// credits. It is a string of the form
    /// <country-code>\[-<sub-national-code>[ <postal-code>]\], with the first two
    /// fields conforming to ISO 3166-2, and postal-code being up to 64
    /// alphanumeric characters.
    #[prost(string, tag="3")]
    pub location: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MsgRetire`.
pub mod msg_retire {
    /// RetireCredits specifies a batch and the number of credits being retired.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetireCredits {
        /// batch_denom is the unique ID of the credit batch.
        #[prost(string, tag="1")]
        pub batch_denom: ::prost::alloc::string::String,
        /// amount is the number of credits being retired.
        /// Decimal values are acceptable within the precision returned by
        /// Query/Precision.
        #[prost(string, tag="2")]
        pub amount: ::prost::alloc::string::String,
    }
impl ::prost::Name for RetireCredits {
const NAME: &'static str = "RetireCredits";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.MsgRetire.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgRetire {
const NAME: &'static str = "MsgRetire";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgRetire is the Msg/Retire response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRetireResponse {
}
impl ::prost::Name for MsgRetireResponse {
const NAME: &'static str = "MsgRetireResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgCancel is the Msg/Cancel request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancel {
    /// holder is the credit holder address.
    #[prost(string, tag="1")]
    pub holder: ::prost::alloc::string::String,
    /// credits are the credits being cancelled.
    #[prost(message, repeated, tag="2")]
    pub credits: ::prost::alloc::vec::Vec<msg_cancel::CancelCredits>,
}
/// Nested message and enum types in `MsgCancel`.
pub mod msg_cancel {
    /// CancelCredits specifies a batch and the number of credits being cancelled.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CancelCredits {
        /// batch_denom is the unique ID of the credit batch.
        #[prost(string, tag="1")]
        pub batch_denom: ::prost::alloc::string::String,
        /// amount is the number of credits being cancelled.
        /// Decimal values are acceptable within the precision returned by
        /// Query/Precision.
        #[prost(string, tag="2")]
        pub amount: ::prost::alloc::string::String,
    }
impl ::prost::Name for CancelCredits {
const NAME: &'static str = "CancelCredits";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.MsgCancel.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgCancel {
const NAME: &'static str = "MsgCancel";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgCancelResponse is the Msg/Cancel response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelResponse {
}
impl ::prost::Name for MsgCancelResponse {
const NAME: &'static str = "MsgCancelResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgUpdateClassAdmin is the Msg/UpdateClassAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassAdmin {
    /// admin is the address of the account that is the admin of the credit class.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// class_id is the unique ID of the credit class.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// new_admin is the address of the new admin of the credit class.
    #[prost(string, tag="3")]
    pub new_admin: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateClassAdmin {
const NAME: &'static str = "MsgUpdateClassAdmin";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgUpdateClassAdminResponse is the MsgUpdateClassAdmin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassAdminResponse {
}
impl ::prost::Name for MsgUpdateClassAdminResponse {
const NAME: &'static str = "MsgUpdateClassAdminResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgUpdateClassIssuers is the Msg/UpdateClassIssuers request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassIssuers {
    /// admin is the address of the account that is the admin of the credit class.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// class_id is the unique ID of the credit class.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// issuers are the updated account addresses of the approved issuers.
    #[prost(string, repeated, tag="3")]
    pub issuers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgUpdateClassIssuers {
const NAME: &'static str = "MsgUpdateClassIssuers";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgUpdateClassIssuersResponse is the MsgUpdateClassIssuers response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassIssuersResponse {
}
impl ::prost::Name for MsgUpdateClassIssuersResponse {
const NAME: &'static str = "MsgUpdateClassIssuersResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgUpdateClassMetadata is the Msg/UpdateClassMetadata request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassMetadata {
    /// admin is the address of the account that is the admin of the credit class.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// class_id is the unique ID of the credit class.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// metadata is the updated arbitrary metadata to be attached to the credit
    /// class.
    #[prost(bytes="vec", tag="3")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MsgUpdateClassMetadata {
const NAME: &'static str = "MsgUpdateClassMetadata";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
/// MsgUpdateClassMetadataResponse is the MsgUpdateClassMetadata response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassMetadataResponse {
}
impl ::prost::Name for MsgUpdateClassMetadataResponse {
const NAME: &'static str = "MsgUpdateClassMetadataResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1alpha1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1alpha1.{}", Self::NAME)
            }}
include!("regen.ecocredit.v1alpha1.serde.rs");
include!("regen.ecocredit.v1alpha1.tonic.rs");
// @@protoc_insertion_point(module)