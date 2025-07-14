// @generated
/// QueryInterchainAccountRequest is the request type for the
/// Query/InterchainAccountAddress RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInterchainAccountRequest {
    /// owner is the address of the account that owns the ICA.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// connection_id is the connection the ICA claimed.
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryInterchainAccountRequest {
const NAME: &'static str = "QueryInterchainAccountRequest";
const PACKAGE: &'static str = "regen.intertx.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.intertx.v1.{}", Self::NAME)
            }}
/// QueryInterchainAccountResponse the response type for the
/// Query/InterchainAccountAddress RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInterchainAccountResponse {
    /// interchain_account_address is the address of the ICA.
    #[prost(string, tag="1")]
    pub interchain_account_address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryInterchainAccountResponse {
const NAME: &'static str = "QueryInterchainAccountResponse";
const PACKAGE: &'static str = "regen.intertx.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.intertx.v1.{}", Self::NAME)
            }}
/// MsgRegisterAccount defines the payload for Msg/RegisterAccount
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccount {
    /// owner is the address of the interchain account owner.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// connection_id is the connection id string (i.e. channel-5).
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
    /// version is the application version string. For example, this could be an
    /// ICS27 encoded metadata type or an ICS29 encoded metadata type with a nested
    /// application version.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRegisterAccount {
const NAME: &'static str = "MsgRegisterAccount";
const PACKAGE: &'static str = "regen.intertx.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.intertx.v1.{}", Self::NAME)
            }}
/// MsgRegisterAccountResponse defines the response for Msg/RegisterAccount
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccountResponse {
}
impl ::prost::Name for MsgRegisterAccountResponse {
const NAME: &'static str = "MsgRegisterAccountResponse";
const PACKAGE: &'static str = "regen.intertx.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.intertx.v1.{}", Self::NAME)
            }}
/// MsgSubmitTx defines the payload for Msg/SubmitTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTx {
    /// owner is the owner address of the interchain account.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// connection_id is the id of the connection.
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
    /// msg is the bytes of the transaction msg to send.
    #[prost(message, optional, tag="3")]
    pub msg: ::core::option::Option<::tendermint_proto::google::protobuf::Any>,
}
impl ::prost::Name for MsgSubmitTx {
const NAME: &'static str = "MsgSubmitTx";
const PACKAGE: &'static str = "regen.intertx.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.intertx.v1.{}", Self::NAME)
            }}
/// MsgSubmitTxResponse defines the response for Msg/SubmitTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTxResponse {
}
impl ::prost::Name for MsgSubmitTxResponse {
const NAME: &'static str = "MsgSubmitTxResponse";
const PACKAGE: &'static str = "regen.intertx.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.intertx.v1.{}", Self::NAME)
            }}
include!("regen.intertx.v1.serde.rs");
include!("regen.intertx.v1.tonic.rs");
// @@protoc_insertion_point(module)