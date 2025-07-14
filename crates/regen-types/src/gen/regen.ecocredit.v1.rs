// @generated
/// CreditType defines the measurement unit/precision of a certain credit type
/// (e.g. carbon, biodiversity...)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditType {
    /// abbreviation is a 1-3 character uppercase abbreviation of the CreditType
    /// name, used in batch denominations within the CreditType. It must be unique.
    #[prost(string, tag="1")]
    pub abbreviation: ::prost::alloc::string::String,
    /// name is the name of the credit type (e.g. carbon, biodiversity).
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// unit is the measurement unit of the credit type (e.g. kg, ton).
    #[prost(string, tag="3")]
    pub unit: ::prost::alloc::string::String,
    /// precision is the decimal precision of the credit type.
    #[prost(uint32, tag="4")]
    pub precision: u32,
}
impl ::prost::Name for CreditType {
const NAME: &'static str = "CreditType";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// Class represents the high-level on-chain information for a credit class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Class {
    /// key is the table row identifier of the credit class used internally for
    /// efficient lookups. This identifier is auto-incrementing.
    #[prost(uint64, tag="1")]
    pub key: u64,
    /// id is the unique identifier of the credit class auto-generated from the
    /// credit type abbreviation and the credit class sequence number.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// admin is the admin of the credit class.
    #[prost(bytes="vec", tag="3")]
    pub admin: ::prost::alloc::vec::Vec<u8>,
    /// metadata is any arbitrary metadata to attached to the credit class.
    #[prost(string, tag="4")]
    pub metadata: ::prost::alloc::string::String,
    /// credit_type_abbrev is the abbreviation of the credit type.
    #[prost(string, tag="5")]
    pub credit_type_abbrev: ::prost::alloc::string::String,
}
impl ::prost::Name for Class {
const NAME: &'static str = "Class";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ClassIssuers is a JOIN table for Class Info that stores the credit class
/// issuers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassIssuer {
    /// class_key is the table row identifier of the credit class used internally
    /// for efficient lookups. This links a class issuer to a credit class.
    #[prost(uint64, tag="1")]
    pub class_key: u64,
    /// issuer is the approved issuer of the credit class.
    #[prost(bytes="vec", tag="2")]
    pub issuer: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ClassIssuer {
const NAME: &'static str = "ClassIssuer";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// Project represents the high-level on-chain information for a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Project {
    /// key is the table row identifier of the project used internally for
    /// efficient lookups. This identifier is auto-incrementing.
    #[prost(uint64, tag="1")]
    pub key: u64,
    /// id is the unique identifier of the project either auto-generated from the
    /// credit class id and project sequence number or provided upon creation.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// admin is the admin of the project.
    #[prost(bytes="vec", tag="3")]
    pub admin: ::prost::alloc::vec::Vec<u8>,
    /// class_key is the table row identifier of the credit class used internally
    /// for efficient lookups. This links a project to a credit class.
    #[prost(uint64, tag="4")]
    pub class_key: u64,
    /// jurisdiction is the jurisdiction of the project.
    /// Full documentation can be found in MsgCreateProject.jurisdiction.
    #[prost(string, tag="5")]
    pub jurisdiction: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the project.
    #[prost(string, tag="6")]
    pub metadata: ::prost::alloc::string::String,
    /// reference_id is any arbitrary string used to reference the project.
    #[prost(string, tag="7")]
    pub reference_id: ::prost::alloc::string::String,
}
impl ::prost::Name for Project {
const NAME: &'static str = "Project";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// Batch represents the high-level on-chain information for a credit batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Batch {
    /// key is the table row identifier of the credit batch used internally for
    /// efficient lookups. This identifier is auto-incrementing.
    #[prost(uint64, tag="1")]
    pub key: u64,
    /// issuer is the address that created the batch and which is
    /// authorized to mint more credits if open=true.
    #[prost(bytes="vec", tag="2")]
    pub issuer: ::prost::alloc::vec::Vec<u8>,
    /// project_key is the table row identifier of the credit class used internally
    /// for efficient lookups. This links a credit batch to a project.
    #[prost(uint64, tag="3")]
    pub project_key: u64,
    /// denom is the unique identifier of the credit batch formed from the
    /// credit class ID (or just project ID for old project IDs which included the credit class),
    /// project id, the batch sequence number, and the start and
    /// end date of the credit batch.
    #[prost(string, tag="4")]
    pub denom: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the credit batch.
    #[prost(string, tag="5")]
    pub metadata: ::prost::alloc::string::String,
    /// start_date is the beginning of the period during which this credit batch
    /// was quantified and verified.
    #[prost(message, optional, tag="6")]
    pub start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// end_date is the end of the period during which this credit batch was
    /// quantified and verified.
    #[prost(message, optional, tag="7")]
    pub end_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// issuance_date is the timestamp when the credit batch was issued.
    #[prost(message, optional, tag="8")]
    pub issuance_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// open tells if it's possible to mint new credits in the future.
    /// Once `open` is set to false, it can't be toggled any more.
    #[prost(bool, tag="9")]
    pub open: bool,
    /// class_key is the table row identifier of the credit class used internally
    /// for efficient lookups. This links a batch to a credit class.
    #[prost(uint64, tag="10")]
    pub class_key: u64,
}
impl ::prost::Name for Batch {
const NAME: &'static str = "Batch";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ClassSequence stores and increments the sequence number for credit classes
/// within a credit type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassSequence {
    /// credit_type_abbrev is the credit type abbreviation. This links a class
    /// sequence to a credit type.
    #[prost(string, tag="1")]
    pub credit_type_abbrev: ::prost::alloc::string::String,
    /// next_sequence is the next sequence number for a credit class within the
    /// credit type. The sequence number is used to generate a class id.
    #[prost(uint64, tag="2")]
    pub next_sequence: u64,
}
impl ::prost::Name for ClassSequence {
const NAME: &'static str = "ClassSequence";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ProjectSequence stores and increments the sequence number for projects within
/// a credit class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectSequence {
    /// class_key is the table row identifier of the credit class used internally
    /// for efficient lookups. This links a project sequence to a credit class.
    #[prost(uint64, tag="1")]
    pub class_key: u64,
    /// next_sequence is the next sequence number for a project within the credit
    /// class. The sequence number is used to generate a project id.
    #[prost(uint64, tag="2")]
    pub next_sequence: u64,
}
impl ::prost::Name for ProjectSequence {
const NAME: &'static str = "ProjectSequence";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// BatchSequence stores and increments the sequence number for credit batches
/// within a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSequence {
    /// project_key is the table row identifier of the project used internally for
    /// efficient lookups. This links a batch sequence to a project.
    #[prost(uint64, tag="1")]
    pub project_key: u64,
    /// next_sequence is the next sequence number for a credit batch within the
    /// project. The sequence number is used to generate a batch denom.
    #[prost(uint64, tag="2")]
    pub next_sequence: u64,
}
impl ::prost::Name for BatchSequence {
const NAME: &'static str = "BatchSequence";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// BatchBalance stores each accounts credit balance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchBalance {
    /// batch_key is the table row identifier of the credit batch used internally
    /// for efficient lookups. This links a batch balance to a credit batch.
    #[prost(uint64, tag="1")]
    pub batch_key: u64,
    /// address is the address of the account that owns the credits.
    #[prost(bytes="vec", tag="2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// tradable_amount is the total number of tradable credits owned by address.
    #[prost(string, tag="3")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the total number of retired credits owned by address.
    #[prost(string, tag="4")]
    pub retired_amount: ::prost::alloc::string::String,
    /// escrowed_amount is the total number of escrowed credits owned by address
    /// and held in escrow by the marketplace. Credits are held in escrow when a
    /// sell order is created and taken out of escrow when the sell order is either
    /// cancelled, updated with a reduced quantity, or processed.
    #[prost(string, tag="5")]
    pub escrowed_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for BatchBalance {
const NAME: &'static str = "BatchBalance";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// BatchSupply stores the supply of credits for a credit batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSupply {
    /// batch_key is the table row identifier of the credit batch used internally
    /// for efficient lookups. This links a batch supply to a credit batch.
    #[prost(uint64, tag="1")]
    pub batch_key: u64,
    /// tradable_amount is the total number of tradable credits in the credit
    /// batch. Tradable credits may be retired in which case they will be removed
    /// from tradable_amount and tracked in retired_amount. Tradable credits may
    /// also be cancelled in which case they will be removed from tradable_amount
    /// and tracked in cancelled_amount. The sum of the tradable, retired, and
    /// cancelled amounts will always equal the original credit issuance amount.
    #[prost(string, tag="2")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the total amount of credits that have been retired in the
    /// credit batch. The sum of the tradable, retired, and cancelled amounts will
    /// always equal the original credit issuance amount.
    #[prost(string, tag="3")]
    pub retired_amount: ::prost::alloc::string::String,
    /// cancelled_amount is the number of credits in the batch that have been
    /// cancelled, effectively undoing the issuance. The sum of the tradable,
    /// retired, and cancelled amounts will always equal the original credit
    /// issuance amount.
    #[prost(string, tag="4")]
    pub cancelled_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for BatchSupply {
const NAME: &'static str = "BatchSupply";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// OriginTxIndex indexes the transaction ID and source from the OriginTx
/// included in Msg/CreateBatch and Msg/MintBatchCredits to prevent double
/// minting errors. The index is scoped to a credit class (it includes the
/// class_key) to prevent malicious credit class issuers from blocking any
/// bridge operations taking place within another credit class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginTxIndex {
    /// class_key is the table row identifier of the credit class within which the
    /// credits were issued or minted. The class_key is included within the index
    /// to prevent malicious credit class issuers from blocking bridge operations
    /// taking place within another credit class.
    #[prost(uint64, tag="1")]
    pub class_key: u64,
    /// id is the transaction ID of an originating transaction or operation
    /// based on a type (i.e. transaction ID, serial number).
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// source is the source chain or registry of the transaction originating
    /// the mint process (e.g. polygon, ethereum, verra).
    #[prost(string, tag="3")]
    pub source: ::prost::alloc::string::String,
}
impl ::prost::Name for OriginTxIndex {
const NAME: &'static str = "OriginTxIndex";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// BatchContract stores the contract address from which credits were bridged
/// when credits are bridged from a contract-based chain, therefore ensuring
/// that each credit batch corresponds to a single contract and credits that
/// have been bridged will always be bridged back to the original contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchContract {
    /// batch_key is the table row identifier of the credit batch used internally
    /// for efficient lookups. This links an external contract to a credit batch.
    #[prost(uint64, tag="1")]
    pub batch_key: u64,
    /// class_key is the table row identifier of the credit class within which the
    /// credit batch exists. A contract is unique within the scope of a credit
    /// class to prevent malicious credit class issuers from blocking bridge
    /// operations taking place within another credit class.
    #[prost(uint64, tag="2")]
    pub class_key: u64,
    /// contract is the address of the contract on the source chain that was
    /// executed when creating the transaction. This address will be used when
    /// sending credits back to the source chain.
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
}
impl ::prost::Name for BatchContract {
const NAME: &'static str = "BatchContract";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ClassCreatorAllowlist determines if the credit class creator allowlist is
/// enabled. When set to true, only the addresses in the AllowedClassCreator
/// table may create credit classes. When set to false, any address may create
/// credit classes. This table is controlled via governance.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassCreatorAllowlist {
    /// enabled is whether or not the allow list is enabled.
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
impl ::prost::Name for ClassCreatorAllowlist {
const NAME: &'static str = "ClassCreatorAllowlist";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// AllowedClassCreator is an allowed credit class creator. This table is
/// controlled via governance.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedClassCreator {
    /// address is the address that is allowed to create credit classes
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for AllowedClassCreator {
const NAME: &'static str = "AllowedClassCreator";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ClassFee is the credit class creation fee. If not set, a credit class
/// creation fee is not required. This table is controlled via governance.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassFee {
    /// fee is the credit class creation fee. If not set, a credit class creation
    /// fee is not required.
    #[prost(message, optional, tag="1")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for ClassFee {
const NAME: &'static str = "ClassFee";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// AllowedBridgeChain is a list of chains that are allowed to be used in
/// bridging operations. NOTE: chain_names MUST be converted to lowercase before
/// writing to and reading from this table in order to keep entries consistent.
/// This table is controlled via governance.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedBridgeChain {
    /// chain_name is the name of the chain allowed to bridge ecocredits to.
    #[prost(string, tag="1")]
    pub chain_name: ::prost::alloc::string::String,
}
impl ::prost::Name for AllowedBridgeChain {
const NAME: &'static str = "AllowedBridgeChain";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ProjectEnrollment stores the data a project's enrollment in a credit class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectEnrollment {
    /// project_key is the table row identifier of the project used internally for
    /// efficient lookups.
    #[prost(uint64, tag="1")]
    pub project_key: u64,
    /// class_key is the table row identifier of the credit class used internally
    /// for efficient lookups.
    #[prost(uint64, tag="3")]
    pub class_key: u64,
    /// status is the status of the enrollment.
    #[prost(enumeration="ProjectEnrollmentStatus", tag="4")]
    pub status: i32,
    /// application_metadata is any arbitrary metadata set by the project
    /// admin related to its application to the credit class.
    #[prost(string, tag="5")]
    pub application_metadata: ::prost::alloc::string::String,
    /// enrollment_metadata is any arbitrary metadata set by the credit class
    /// admin evaluating the project's application to the credit class.
    #[prost(string, tag="6")]
    pub enrollment_metadata: ::prost::alloc::string::String,
}
impl ::prost::Name for ProjectEnrollment {
const NAME: &'static str = "ProjectEnrollment";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ProjectFee is the project creation fee. If not set, a project creation fee is
/// not required. This table is controlled via governance.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectFee {
    /// fee is the project creation fee. If not set, a project creation fee is not
    /// required.
    #[prost(message, optional, tag="1")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for ProjectFee {
const NAME: &'static str = "ProjectFee";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// Application represents the evaluation status that a credit class issuer
/// assigns to a credit class application.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProjectEnrollmentStatus {
    /// PROJECT_ENROLLMENT_STATUS_UNSPECIFIED indicates that a credit class application
    /// has been submitted but not evaluated.
    Unspecified = 0,
    /// PROJECT_ENROLLMENT_STATUS_ACCEPTED indicates that the project has been
    /// accepted into the credit class.
    Accepted = 1,
    /// PROJECT_ENROLLMENT_STATUS_CHANGES_REQUESTED indicates that an application to
    /// a credit class has been reviewed and that changes to the application have
    /// been requested. It can also be used to indicate that a project within a credit
    /// class has had its status reassessed and that changes to the project are
    /// requested in order to continue in the credit class.
    ChangesRequested = 2,
    /// PROJECT_ENROLLMENT_STATUS_REJECTED indicates that the application has been
    /// rejected and that the project will not be accepted into the credit class.
    Rejected = 3,
    /// PROJECT_ENROLLMENT_STATUS_TERMINATED indicates that the project has been
    /// terminated from the credit class. This status is used when a project the
    /// was previously accepted into the credit class but has been removed or
    /// completed.
    Terminated = 4,
}
impl ProjectEnrollmentStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProjectEnrollmentStatus::Unspecified => "PROJECT_ENROLLMENT_STATUS_UNSPECIFIED",
            ProjectEnrollmentStatus::Accepted => "PROJECT_ENROLLMENT_STATUS_ACCEPTED",
            ProjectEnrollmentStatus::ChangesRequested => "PROJECT_ENROLLMENT_STATUS_CHANGES_REQUESTED",
            ProjectEnrollmentStatus::Rejected => "PROJECT_ENROLLMENT_STATUS_REJECTED",
            ProjectEnrollmentStatus::Terminated => "PROJECT_ENROLLMENT_STATUS_TERMINATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROJECT_ENROLLMENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PROJECT_ENROLLMENT_STATUS_ACCEPTED" => Some(Self::Accepted),
            "PROJECT_ENROLLMENT_STATUS_CHANGES_REQUESTED" => Some(Self::ChangesRequested),
            "PROJECT_ENROLLMENT_STATUS_REJECTED" => Some(Self::Rejected),
            "PROJECT_ENROLLMENT_STATUS_TERMINATED" => Some(Self::Terminated),
            _ => None,
        }
    }
}
/// Params defines the updatable global parameters of the ecocredit module for
/// use with the x/params module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// credit_class_fee is a list of credit class creation fees accepted when
    /// creating a credit class. Any fee listed is accepted and charged to the
    /// credit class creator when creating a credit class.
    #[prost(message, repeated, tag="1")]
    pub credit_class_fee: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// basket_fee is a list of basket creation fees accepted when creating a
    /// basket. Any fee listed is accepted and charged to the basket creator when
    /// creating a basket.
    #[prost(message, repeated, tag="2")]
    pub basket_fee: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// allowed_class_creators is an allowlist defining the addresses with the
    /// required permissions to create credit classes when allowlist_enabled is set
    /// to true. If allowlist_enabled is set to false, this list has no effect.
    #[prost(string, repeated, tag="3")]
    pub allowed_class_creators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// allowlist_enabled determines whether or not the allowlist for creating
    /// credit classes is enabled. When set to true, only the addresses listed in
    /// allowed_class_creators can create credit classes. When set to false, any
    /// address can create credit classes.
    #[prost(bool, tag="4")]
    pub allowlist_enabled: bool,
    /// allowed_denoms is a list of bank denoms allowed to be used in the ask price
    /// of sell orders.
    ///
    /// Since Revision 2
    #[prost(message, repeated, tag="5")]
    pub allowed_denoms: ::prost::alloc::vec::Vec<AllowedDenom>,
    /// AllowedBridgeChains is a list of chain names that are allowed to be used in
    /// bridge operations.
    ///
    /// Since Revision 2
    #[prost(string, repeated, tag="6")]
    pub allowed_bridge_chains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for Params {
const NAME: &'static str = "Params";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// Credits represents a simple structure for credits.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Credits {
    /// batch_denom is the denom of the credit batch.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// amount is the amount of credits.
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for Credits {
const NAME: &'static str = "Credits";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// BatchIssuance represents a simple structure for a credit batch issuance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchIssuance {
    /// recipient is the address of the account receiving the issued credits.
    #[prost(string, tag="1")]
    pub recipient: ::prost::alloc::string::String,
    /// tradable_amount is the amount of credits that the recipient will receive in
    /// a tradable state. The number of decimal places must be less than or equal
    /// to the credit type precision.
    #[prost(string, tag="2")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the amount of credits that the recipient will receive in
    /// a retired state. The number of decimal places must be less than or equal to
    /// the credit type precision.
    #[prost(string, tag="3")]
    pub retired_amount: ::prost::alloc::string::String,
    /// retirement_jurisdiction is the jurisdiction of the recipient and is only
    /// required if retired_amount is positive. A jurisdiction has the following
    /// format: <country-code>\[-<sub-national-code>[ <postal-code>]\]
    /// The country-code must be 2 alphabetic characters, the sub-national-code
    /// can be 1-3 alphanumeric characters, and the postal-code can be up to 64
    /// alphanumeric characters. Only the country-code is required, while the
    /// sub-national-code and postal-code are optional and can be added for
    /// increased precision.
    #[prost(string, tag="4")]
    pub retirement_jurisdiction: ::prost::alloc::string::String,
    /// retirement_reason is any arbitrary string that specifies the reason for
    /// retiring credits. The reason will be included in EventRetire and is not
    /// stored in state.
    ///
    /// Since Revision 2
    #[prost(string, tag="5")]
    pub retirement_reason: ::prost::alloc::string::String,
}
impl ::prost::Name for BatchIssuance {
const NAME: &'static str = "BatchIssuance";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// OriginTx is the transaction from another chain or registry that triggered
/// the minting of credits.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginTx {
    /// id is the transaction ID of an originating transaction or operation based
    /// on a type (i.e. transaction ID, serial number).
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// source is the source chain or registry of the transaction originating the
    /// mint process (e.g. polygon, ethereum, verra).
    #[prost(string, tag="2")]
    pub source: ::prost::alloc::string::String,
    /// contract is the address of the contract on the source chain that was
    /// executed when creating the transaction. This address will be stored in
    /// state separately from the origin tx and on a per credit batch basis to be
    /// used when sending credits back to the source chain. This field can be left
    /// blank if credits are bridged from a non-contract-based source.
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    /// note is a reference note for accounting that will be included in an event
    /// emitted from either Msg/CreateBatch or Msg/MintBatchCredits.
    #[prost(string, tag="4")]
    pub note: ::prost::alloc::string::String,
}
impl ::prost::Name for OriginTx {
const NAME: &'static str = "OriginTx";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// CreditTypeProposal is a gov Content type for adding a credit type.
/// Deprecated (Since Revision 2): This message is no longer used and will be
/// removed in the next version. See MsgAddCreditType.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditTypeProposal {
    /// title is the title of the proposal.
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// description is the description of the proposal.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// credit_type is the credit type to be added to the network if the proposal
    /// passes.
    #[prost(message, optional, tag="3")]
    pub credit_type: ::core::option::Option<CreditType>,
}
impl ::prost::Name for CreditTypeProposal {
const NAME: &'static str = "CreditTypeProposal";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// AllowedDenom represents the information for an allowed ask denom.
///
/// Since Revision 2
///
/// Deprecated(Since Revision 2): This type was added to support historical
/// queries for params but will also be removed in the next version.
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
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventCreateClass is an event emitted when a credit class is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateClass {
    /// class_id is the unique identifier of the credit class.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateClass {
const NAME: &'static str = "EventCreateClass";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventCreateProject is an event emitted when a project is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateProject {
    /// project_id is the unique identifier of the project.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateProject {
const NAME: &'static str = "EventCreateProject";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventCreateBatch is an event emitted when a credit batch is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateBatch {
    /// batch_denom is the unique identifier of the credit batch.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// origin_tx is the transaction from another chain or registry that triggered
    /// the creation of the credit batch.
    #[prost(message, optional, tag="2")]
    pub origin_tx: ::core::option::Option<OriginTx>,
}
impl ::prost::Name for EventCreateBatch {
const NAME: &'static str = "EventCreateBatch";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventMint is an event emitted when credits are minted either when creating a
/// credit batch or when bridging assets from another chain or registry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMint {
    /// batch_denom is the unique identifier of the credit batch within which the
    /// credits were minted.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// tradable_amount is the amount of tradable credits minted.
    #[prost(string, tag="2")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the amount of retired credits minted.
    #[prost(string, tag="3")]
    pub retired_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventMint {
const NAME: &'static str = "EventMint";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventMintBatchCredits is an event emitted when credits are minted to an
/// existing open credit batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMintBatchCredits {
    /// batch_denom is the unique identifier of the credit batch within which the
    /// credits were minted.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// origin_tx is the transaction from another chain or registry that triggered
    /// the minting of credits within the credit batch.
    #[prost(message, optional, tag="2")]
    pub origin_tx: ::core::option::Option<OriginTx>,
}
impl ::prost::Name for EventMintBatchCredits {
const NAME: &'static str = "EventMintBatchCredits";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventTransfer is an event emitted when credits are transferred from one
/// account to another including transfers to or from a module account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTransfer {
    /// sender is the sender of the credits. In the case that the credits were
    /// transferred from a base account, this will be the account address. In the
    /// case that the credits were transferred from a module, this will be the
    /// module address (i.e. either the ecocredit module or basket submodule).
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// recipient is the recipient of the credits. In the case that the credits
    /// were transferred to a base account, this will be the account address. In
    /// the case that the credits were transferred to a module, this will be the
    /// module address (i.e. either the ecocredit module or basket submodule).
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the credit batch.
    #[prost(string, tag="3")]
    pub batch_denom: ::prost::alloc::string::String,
    /// tradable_amount is the decimal number of tradable credits received.
    #[prost(string, tag="4")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the decimal number of retired credits received.
    #[prost(string, tag="5")]
    pub retired_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventTransfer {
const NAME: &'static str = "EventTransfer";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventRetire is an event emitted when credits are retired. When credits are
/// retired from multiple batches in the same transaction, a separate event is
/// emitted for each batch_denom. This allows for easier indexing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRetire {
    /// owner is the address of the account that owns the retired credits. This
    /// will be the account receiving credits in the case that credits were retired
    /// upon issuance using Msg/CreateBatch, retired upon transfer using Msg/Send,
    /// retired upon taking from a basket using basket.Msg/Take, or retired upon
    /// purchase using marketplace.Msg/BuyDirect.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the credit batch within which the
    /// credits were retired.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// amount is the decimal number of credits that have been retired.
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// jurisdiction is the jurisdiction of the beneficiary or buyer of the retired
    /// credits. It is a string of the form
    /// <country-code>\[-<sub-national-code>[ <postal-code>]\], with the first two
    /// fields conforming to ISO 3166-2, and postal-code being up to 64
    /// alphanumeric characters.
    #[prost(string, tag="4")]
    pub jurisdiction: ::prost::alloc::string::String,
    /// reason is any arbitrary string that specifies the reason for retiring
    /// credits.
    ///
    /// Since Revision 2
    #[prost(string, tag="5")]
    pub reason: ::prost::alloc::string::String,
}
impl ::prost::Name for EventRetire {
const NAME: &'static str = "EventRetire";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventCancel is an event emitted when credits are cancelled. When credits are
/// cancelled from multiple batches in the same transaction, a separate event is
/// emitted for each batch_denom. This allows for easier indexing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancel {
    /// owner is the address of the account that cancelled the credits.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the credit batch within which the
    /// credits were cancelled.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// amount is the decimal number of credits that have been cancelled.
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// reason is the reason the credits were cancelled.
    #[prost(string, tag="4")]
    pub reason: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCancel {
const NAME: &'static str = "EventCancel";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventUpdateClassAdmin is emitted when the admin address of a credit class is
/// changed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateClassAdmin {
    /// class_id is the unique identifier of the class that was updated.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateClassAdmin {
const NAME: &'static str = "EventUpdateClassAdmin";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventUpdateClassIssuers is emitted when the issuer list for a credit class
/// is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateClassIssuers {
    /// class_id is the unique identifier of the class that was updated.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateClassIssuers {
const NAME: &'static str = "EventUpdateClassIssuers";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventUpdateClassMetadata is emitted when the credit class metadata is
/// changed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateClassMetadata {
    /// class_id is the unique identifier of the class that was updated.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateClassMetadata {
const NAME: &'static str = "EventUpdateClassMetadata";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventUpdateProjectAdmin is emitted when the project admin is changed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateProjectAdmin {
    /// project_id is the unique identifier of the project that was updated.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateProjectAdmin {
const NAME: &'static str = "EventUpdateProjectAdmin";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventUpdateProjectMetadata is emitted when the project metadata is changed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateProjectMetadata {
    /// project_id is the unique identifier of the project that was updated.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateProjectMetadata {
const NAME: &'static str = "EventUpdateProjectMetadata";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventUpdateBatchMetadata is emitted when the credit batch metadata is
/// changed.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateBatchMetadata {
    /// batch_denom is the unique identifier of the batch that was updated.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateBatchMetadata {
const NAME: &'static str = "EventUpdateBatchMetadata";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventSealBatch is emitted when a batch is sealed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSealBatch {
    /// batch_denom is the denom of the batch that was sealed.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventSealBatch {
const NAME: &'static str = "EventSealBatch";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventAddCreditType is emitted when governance approves a new credit type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAddCreditType {
    /// abbreviation is the abbreviation of the credit type.
    #[prost(string, tag="1")]
    pub abbreviation: ::prost::alloc::string::String,
}
impl ::prost::Name for EventAddCreditType {
const NAME: &'static str = "EventAddCreditType";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventBridge is emitted when credits are bridged to another chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBridge {
    /// target is the target chain.
    #[prost(string, tag="1")]
    pub target: ::prost::alloc::string::String,
    /// recipient is the recipient address.
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// contract is the contract address.
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    /// amount is the amount of credits.
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    /// owner is the owner address.
    ///
    /// Since Revision 1
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
    /// batch_denom is the credit batch denom.
    ///
    /// Since Revision 3
    #[prost(string, tag="6")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventBridge {
const NAME: &'static str = "EventBridge";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventBridgeReceive is emitted when credits are bridged from another chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBridgeReceive {
    /// project_id is the unique identifier of the project that was either created
    /// or the existing project within which the credit batch exists.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the credit batch either created
    /// or within which the credits were dynamically minted.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// amount is the amount of credits.
    ///
    /// Since Revision 3
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// origin_tx is the transaction from another chain or registry that triggered
    /// the minting of credits within the credit batch.
    ///
    /// Since Revision 3
    #[prost(message, optional, tag="4")]
    pub origin_tx: ::core::option::Option<OriginTx>,
}
impl ::prost::Name for EventBridgeReceive {
const NAME: &'static str = "EventBridgeReceive";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventBurnRegen is an event emitted when REGEN is burned to account for credit origination, transfer, etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBurnRegen {
    /// burner is the address that burned REGEN.
    #[prost(string, tag="1")]
    pub burner: ::prost::alloc::string::String,
    /// amount is the integer amount of uregen burned.
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// reason is the reason for the burn.
    #[prost(string, tag="3")]
    pub reason: ::prost::alloc::string::String,
}
impl ::prost::Name for EventBurnRegen {
const NAME: &'static str = "EventBurnRegen";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventUpdateApplication is emitted when a project admin creates, updates
/// or withdraws a project's application to a credit class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateApplication {
    /// project_id is the unique identifier of the project that was updated.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the class that was updated.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// action is the action that was taken on the application.
    #[prost(enumeration="event_update_application::Action", tag="3")]
    pub action: i32,
    /// new_application_metadata is any new application metadata.
    #[prost(string, tag="4")]
    pub new_application_metadata: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EventUpdateApplication`.
pub mod event_update_application {
    /// Action describes an action taken on an application.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        /// ACTION_UNSPECIFIED is the default value for the action and is invalid.
        Unspecified = 0,
        /// ACTION_CREATE is the action taken when a project admin creates an
        /// application to a credit class.
        Create = 1,
        /// ACTION_UPDATE is the action taken when a project admin updates an
        /// application to a credit class.
        Update = 2,
        /// ACTION_WITHDRAW is the action taken when a project admin withdraws an
        /// application to a credit class.
        Withdraw = 3,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unspecified => "ACTION_UNSPECIFIED",
                Action::Create => "ACTION_CREATE",
                Action::Update => "ACTION_UPDATE",
                Action::Withdraw => "ACTION_WITHDRAW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTION_CREATE" => Some(Self::Create),
                "ACTION_UPDATE" => Some(Self::Update),
                "ACTION_WITHDRAW" => Some(Self::Withdraw),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for EventUpdateApplication {
const NAME: &'static str = "EventUpdateApplication";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// EventUpdateProjectEnrollment is emitted when a credit class issuer updates
/// the enrollment status of a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateProjectEnrollment {
    /// issuer is the address of the credit class issuer which evaluated the
    /// project.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// project_id is the unique identifier of the project that was evaluated.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the class in which the project was
    /// evaluated.
    #[prost(string, tag="3")]
    pub class_id: ::prost::alloc::string::String,
    /// old_status is the old status of the project class relationship.
    #[prost(enumeration="ProjectEnrollmentStatus", tag="4")]
    pub old_status: i32,
    /// new_status is the new status of the project class relationship.
    #[prost(enumeration="ProjectEnrollmentStatus", tag="5")]
    pub new_status: i32,
    /// new_enrollment_metadata is any new enrollment metadata.
    #[prost(string, tag="6")]
    pub new_enrollment_metadata: ::prost::alloc::string::String,
}
impl ::prost::Name for EventUpdateProjectEnrollment {
const NAME: &'static str = "EventUpdateProjectEnrollment";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
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
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
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
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassesByAdminRequest is the Query/ClassesByAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassesByAdminRequest {
    /// admin is the address of the admin of the class.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryClassesByAdminRequest {
const NAME: &'static str = "QueryClassesByAdminRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassesByAdminResponse is the Query/ClassesByAdmin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassesByAdminResponse {
    /// classes are the fetched credit classes.
    #[prost(message, repeated, tag="1")]
    pub classes: ::prost::alloc::vec::Vec<ClassInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryClassesByAdminResponse {
const NAME: &'static str = "QueryClassesByAdminResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassRequest is the Query/Class request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassRequest {
    /// class_id is the unique identifier of the credit class to query.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryClassRequest {
const NAME: &'static str = "QueryClassRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassResponse is the Query/Class request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassResponse {
    /// class is the fetched credit class.
    #[prost(message, optional, tag="1")]
    pub class: ::core::option::Option<ClassInfo>,
}
impl ::prost::Name for QueryClassResponse {
const NAME: &'static str = "QueryClassResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassIssuersRequest is the Query/ClassIssuers request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassIssuersRequest {
    /// class_id is the unique identifier of the credit class to query.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryClassIssuersRequest {
const NAME: &'static str = "QueryClassIssuersRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassIssuersRequest is the Query/ClassIssuers response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassIssuersResponse {
    /// issuers is a list of issuers for the credit class
    #[prost(string, repeated, tag="1")]
    pub issuers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryClassIssuersResponse {
const NAME: &'static str = "QueryClassIssuersResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectsRequest is the Query/Projects request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryProjectsRequest {
const NAME: &'static str = "QueryProjectsRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectsResponse is the Query/Projects response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectsResponse {
    /// projects are the fetched projects.
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<ProjectInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryProjectsResponse {
const NAME: &'static str = "QueryProjectsResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectsByClassRequest is the Query/ProjectsByClass request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectsByClassRequest {
    /// class_id is the unique identifier of the credit class to query.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryProjectsByClassRequest {
const NAME: &'static str = "QueryProjectsByClassRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectsByClassResponse is the Query/ProjectsByClass response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectsByClassResponse {
    /// projects are the fetched projects.
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<ProjectInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryProjectsByClassResponse {
const NAME: &'static str = "QueryProjectsByClassResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectsByReferenceIdRequest is the Query/ProjectsByReferenceId request
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectsByReferenceIdRequest {
    /// reference_id is the project reference id.
    #[prost(string, tag="1")]
    pub reference_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryProjectsByReferenceIdRequest {
const NAME: &'static str = "QueryProjectsByReferenceIdRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectsByReferenceIdResponse is the Query/ProjectsByReferenceId
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectsByReferenceIdResponse {
    /// projects are the fetched projects.
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<ProjectInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryProjectsByReferenceIdResponse {
const NAME: &'static str = "QueryProjectsByReferenceIdResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectsByAdminRequest is the Query/ProjectByAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectsByAdminRequest {
    /// admin is the account address of project admin.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryProjectsByAdminRequest {
const NAME: &'static str = "QueryProjectsByAdminRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectsByAdminResponse is the Query/ProjectByAdmin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectsByAdminResponse {
    /// projects are the fetched projects.
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<ProjectInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryProjectsByAdminResponse {
const NAME: &'static str = "QueryProjectsByAdminResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectRequest is the Query/Project request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectRequest {
    /// project_id is the unique identifier of the project to query.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryProjectRequest {
const NAME: &'static str = "QueryProjectRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectResponse is the Query/Project response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectResponse {
    /// project is the fetched project.
    #[prost(message, optional, tag="1")]
    pub project: ::core::option::Option<ProjectInfo>,
}
impl ::prost::Name for QueryProjectResponse {
const NAME: &'static str = "QueryProjectResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchesRequest is the Query/Batches request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBatchesRequest {
const NAME: &'static str = "QueryBatchesRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchesResponse is the Query/Batches response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesResponse {
    /// batches are the fetched credit batches.
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<BatchInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBatchesResponse {
const NAME: &'static str = "QueryBatchesResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchesByIssuerRequest is the Query/BatchesByIssuer request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesByIssuerRequest {
    /// issuer is the address that issued the batch
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBatchesByIssuerRequest {
const NAME: &'static str = "QueryBatchesByIssuerRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchesByIssuerResponse is the Query/BatchesByIssuer response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesByIssuerResponse {
    /// batches are the fetched credit batches.
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<BatchInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBatchesByIssuerResponse {
const NAME: &'static str = "QueryBatchesByIssuerResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchesByClassRequest is the Query/BatchesByClass request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesByClassRequest {
    /// class_id is the unique identifier of the credit class to query.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBatchesByClassRequest {
const NAME: &'static str = "QueryBatchesByClassRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchesByProjectRequest is the Query/BatchesByProject request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesByProjectRequest {
    /// project_id is the unique identifier of the project to query.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBatchesByProjectRequest {
const NAME: &'static str = "QueryBatchesByProjectRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchesByProjectResponse is the Query/BatchesByProject response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesByProjectResponse {
    /// batches are the fetched credit batches.
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<BatchInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBatchesByProjectResponse {
const NAME: &'static str = "QueryBatchesByProjectResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchesByClassResponse is the Query/BatchesByClass response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesByClassResponse {
    /// batches are the fetched credit batches.
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<BatchInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBatchesByClassResponse {
const NAME: &'static str = "QueryBatchesByClassResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchRequest is the Query/Batch request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchRequest {
    /// batch_denom is the unique identifier of the credit batch to query.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBatchRequest {
const NAME: &'static str = "QueryBatchRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBatchResponse is the Query/Batch response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchResponse {
    /// batch is the fetched credit batch.
    #[prost(message, optional, tag="1")]
    pub batch: ::core::option::Option<BatchInfo>,
}
impl ::prost::Name for QueryBatchResponse {
const NAME: &'static str = "QueryBatchResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBalanceRequest is the Query/Balance request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// address is the address of the account whose balance is being queried.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the credit batch to query.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBalanceRequest {
const NAME: &'static str = "QueryBalanceRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBalanceResponse is the Query/Balance response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// balance is the balance of the given account.
    #[prost(message, optional, tag="1")]
    pub balance: ::core::option::Option<BatchBalanceInfo>,
}
impl ::prost::Name for QueryBalanceResponse {
const NAME: &'static str = "QueryBalanceResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBalancesRequest is the Query/Balances request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalancesRequest {
    /// address is the address of the account whose balance is being queried.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBalancesRequest {
const NAME: &'static str = "QueryBalancesRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBalancesResponse is the Query/Balances response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalancesResponse {
    /// balances are a list of balances from different credit batches that the
    /// account holds.
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<BatchBalanceInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBalancesResponse {
const NAME: &'static str = "QueryBalancesResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBalancesByBatchRequest is the Query/BalancesByBatch request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalancesByBatchRequest {
    /// batch_denom is the denom of the batch to query by.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBalancesByBatchRequest {
const NAME: &'static str = "QueryBalancesByBatchRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryBalancesByBatchResponse is the Query/BalancesByBatch response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalancesByBatchResponse {
    /// balances are a list of balances from different credit batches that the
    /// account holds.
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<BatchBalanceInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBalancesByBatchResponse {
const NAME: &'static str = "QueryBalancesByBatchResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryAllBalancesRequest is the Query/AllBalances request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBalancesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAllBalancesRequest {
const NAME: &'static str = "QueryAllBalancesRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryAllBalancesResponse is the Query/AllBalances response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBalancesResponse {
    /// balances are a list of balances from different credit batches that the
    /// account holds.
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<BatchBalanceInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAllBalancesResponse {
const NAME: &'static str = "QueryAllBalancesResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QuerySupplyRequest is the Query/Supply request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyRequest {
    /// batch_denom is the unique identifier of the credit batch to query.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySupplyRequest {
const NAME: &'static str = "QuerySupplyRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QuerySupplyResponse is the Query/Supply response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyResponse {
    /// tradable_amount is the decimal number of tradable credits in the batch
    /// supply.
    #[prost(string, tag="1")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the decimal number of retired credits in the batch
    /// supply.
    #[prost(string, tag="2")]
    pub retired_amount: ::prost::alloc::string::String,
    /// cancelled_amount is the decimal number of cancelled credits in the batch
    /// supply.
    #[prost(string, tag="3")]
    pub cancelled_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySupplyResponse {
const NAME: &'static str = "QuerySupplyResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryCreditTypesRequest is the Query/Credit_Types request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCreditTypesRequest {
}
impl ::prost::Name for QueryCreditTypesRequest {
const NAME: &'static str = "QueryCreditTypesRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryCreditTypesRequest is the Query/Credit_Types response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCreditTypesResponse {
    /// credit_types are the fetched credit types.
    #[prost(message, repeated, tag="1")]
    pub credit_types: ::prost::alloc::vec::Vec<CreditType>,
}
impl ::prost::Name for QueryCreditTypesResponse {
const NAME: &'static str = "QueryCreditTypesResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryParamsRequest is the Query/Params request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
impl ::prost::Name for QueryParamsRequest {
const NAME: &'static str = "QueryParamsRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
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
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryCreditTypeRequest is the Query/CreditType request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCreditTypeRequest {
    /// abbreviation is the abbreviation of the credit type.
    #[prost(string, tag="1")]
    pub abbreviation: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryCreditTypeRequest {
const NAME: &'static str = "QueryCreditTypeRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryCreditTypeResponse is the Query/CreditType response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCreditTypeResponse {
    /// credit_type is the fetched credit type.
    #[prost(message, optional, tag="1")]
    pub credit_type: ::core::option::Option<CreditType>,
}
impl ::prost::Name for QueryCreditTypeResponse {
const NAME: &'static str = "QueryCreditTypeResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ClassInfo is the human-readable credit class information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassInfo {
    /// id is the unique identifier of the credit class.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// admin is the admin of the credit class.
    #[prost(string, tag="2")]
    pub admin: ::prost::alloc::string::String,
    /// metadata is the arbitrary metadata attached to the credit class.
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    /// credit_type_abbrev is the abbreviation of the credit type within which this
    /// credit class was created.
    #[prost(string, tag="4")]
    pub credit_type_abbrev: ::prost::alloc::string::String,
}
impl ::prost::Name for ClassInfo {
const NAME: &'static str = "ClassInfo";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// ProjectInfo is the human-readable project information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectInfo {
    /// id is the unique identifier of the project.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// admin is the admin of the project.
    #[prost(string, tag="2")]
    pub admin: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the credit class within which the
    /// project was created.
    #[prost(string, tag="3")]
    pub class_id: ::prost::alloc::string::String,
    /// jurisdiction is the jurisdiction of the project. Full documentation can be
    /// found in MsgCreateProject.jurisdiction.
    #[prost(string, tag="4")]
    pub jurisdiction: ::prost::alloc::string::String,
    /// metadata is the arbitrary metadata attached to the project.
    #[prost(string, tag="5")]
    pub metadata: ::prost::alloc::string::String,
    /// reference_id is any arbitrary string that can be use to reference project.
    #[prost(string, tag="6")]
    pub reference_id: ::prost::alloc::string::String,
}
impl ::prost::Name for ProjectInfo {
const NAME: &'static str = "ProjectInfo";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// BatchInfo is the human-readable credit batch information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchInfo {
    /// issuer is the address that created the batch and the address authorized to
    /// mint new credits to the credit batch if the credit batch is open.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// project_id is the unique identifier of the project within which this credit
    /// batch was created.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// denom is the unique identifier of the credit batch formed from the project
    /// name, batch sequence number and dates.
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the credit batch.
    #[prost(string, tag="4")]
    pub metadata: ::prost::alloc::string::String,
    /// start_date is the beginning of the period during which this credit batch
    /// was quantified and verified.
    #[prost(message, optional, tag="5")]
    pub start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// end_date is the end of the period during which this credit batch was
    /// quantified and verified.
    #[prost(message, optional, tag="6")]
    pub end_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// issuance_date is the timestamp when the credit batch was issued.
    #[prost(message, optional, tag="7")]
    pub issuance_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// open determines whether or not the credit batch is open, i.e. whether or
    /// not new credits can be minted to the credit batch.
    #[prost(bool, tag="8")]
    pub open: bool,
}
impl ::prost::Name for BatchInfo {
const NAME: &'static str = "BatchInfo";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// BatchBalanceInfo is the human-readable batch balance information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchBalanceInfo {
    /// address is the address of the account that owns the credits.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// tradable_amount is the total number of tradable credits owned by address.
    #[prost(string, tag="3")]
    pub tradable_amount: ::prost::alloc::string::String,
    /// retired_amount is the total number of retired credits owned by address.
    #[prost(string, tag="4")]
    pub retired_amount: ::prost::alloc::string::String,
    /// escrowed_amount is the total number of escrowed credits owned by address
    /// and held in escrow by the marketplace. Credits are held in escrow when a
    /// sell order is created and taken out of escrow when the sell order is either
    /// cancelled, updated with a reduced quantity, or processed.
    #[prost(string, tag="5")]
    pub escrowed_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for BatchBalanceInfo {
const NAME: &'static str = "BatchBalanceInfo";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassCreatorAllowlistRequest is the Query/ClassCreatorAllowlist request
/// type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassCreatorAllowlistRequest {
}
impl ::prost::Name for QueryClassCreatorAllowlistRequest {
const NAME: &'static str = "QueryClassCreatorAllowlistRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassCreatorAllowlistResponse is the Query/ClassCreatorAllowlist
/// response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassCreatorAllowlistResponse {
    /// enabled determines whether or not the allowlist for creating credit classes
    /// is enabled.
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
impl ::prost::Name for QueryClassCreatorAllowlistResponse {
const NAME: &'static str = "QueryClassCreatorAllowlistResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryAllowedClassCreatorsRequest is the Query/AllowedClassCreators request
/// type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowedClassCreatorsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAllowedClassCreatorsRequest {
const NAME: &'static str = "QueryAllowedClassCreatorsRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryAllowedClassCreatorsResponse is the Query/AllowedClassCreators response
/// type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowedClassCreatorsResponse {
    /// class_creators is the list of allowed credit class creators.
    #[prost(string, repeated, tag="1")]
    pub class_creators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAllowedClassCreatorsResponse {
const NAME: &'static str = "QueryAllowedClassCreatorsResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassFeeRequest is the Query/ClassFee request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassFeeRequest {
}
impl ::prost::Name for QueryClassFeeRequest {
const NAME: &'static str = "QueryClassFeeRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryClassFeeResponse is the Query/ClassFee response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassFeeResponse {
    /// fee is the credit class creation fee. If not set, a credit class creation
    /// fee is not required.
    #[prost(message, optional, tag="1")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryClassFeeResponse {
const NAME: &'static str = "QueryClassFeeResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryAllowedBridgeChainsRequest is the Query/AllowedBridgeChains request
/// type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowedBridgeChainsRequest {
}
impl ::prost::Name for QueryAllowedBridgeChainsRequest {
const NAME: &'static str = "QueryAllowedBridgeChainsRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryAllowedBridgeChainsResponse is the Query/AllowedBridgeChains response
/// type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowedBridgeChainsResponse {
    /// allowed_bridge_chains is a list of chains that are allowed to be used in
    /// bridge operations.
    #[prost(string, repeated, tag="1")]
    pub allowed_bridge_chains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryAllowedBridgeChainsResponse {
const NAME: &'static str = "QueryAllowedBridgeChainsResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectEnrollmentRequest is the Query/ProjectEnrollment request type.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectEnrollmentRequest {
    /// project_id is the unique identifier of the project to query.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the credit class to query.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryProjectEnrollmentRequest {
const NAME: &'static str = "QueryProjectEnrollmentRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectEnrollmentResponse is the Query/ProjectEnrollment response type.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectEnrollmentResponse {
    /// project_class is the fetched project class relationship.
    #[prost(message, optional, tag="1")]
    pub project_class: ::core::option::Option<ProjectEnrollment>,
}
impl ::prost::Name for QueryProjectEnrollmentResponse {
const NAME: &'static str = "QueryProjectEnrollmentResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectEnrollmentsRequest is the Query/ProjectEnrollments request type.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectEnrollmentsRequest {
    /// project_id is the unique identifier of the project to query.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryProjectEnrollmentsRequest {
const NAME: &'static str = "QueryProjectEnrollmentsRequest";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// QueryProjectEnrollmentsResponse is the Query/ProjectEnrollments response type.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProjectEnrollmentsResponse {
    /// enrollments are the fetched project credit class enrollments.
    #[prost(message, repeated, tag="1")]
    pub enrollments: ::prost::alloc::vec::Vec<ProjectEnrollment>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryProjectEnrollmentsResponse {
const NAME: &'static str = "QueryProjectEnrollmentsResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgAddCreditType is the Msg/AddCreditType request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddCreditType {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// credit_type defines a credit type to add to the credit types parameter.
    #[prost(message, optional, tag="2")]
    pub credit_type: ::core::option::Option<CreditType>,
}
impl ::prost::Name for MsgAddCreditType {
const NAME: &'static str = "MsgAddCreditType";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgAddCreditTypeResponse is the Msg/AddCreditType response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddCreditTypeResponse {
}
impl ::prost::Name for MsgAddCreditTypeResponse {
const NAME: &'static str = "MsgAddCreditTypeResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateClass is the Msg/CreateClass request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClass {
    /// admin is the address of the account creating the credit class that will
    /// become the admin of the credit class upon creation. The admin will have
    /// permissions within the credit class to update the credit class including
    /// the list of approved issuers. If Params.allowlist_enabled is set to true,
    /// this address must be included in Params.allowed_class_creators.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// issuers are the addresses of the accounts that will have permissions within
    /// the credit class to create projects and issue credits.
    #[prost(string, repeated, tag="2")]
    pub issuers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// metadata is any arbitrary string with a maximum length of 256 characters
    /// that includes or references metadata to attach to the credit class.
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    /// credit_type_abbrev is the abbreviation of the credit type under which the
    /// credit class will be created (e.g. "C", "BIO").
    #[prost(string, tag="4")]
    pub credit_type_abbrev: ::prost::alloc::string::String,
    /// fee is the credit class creation fee. An equal fee is required if the class
    /// creation fee parameter is set. The provided fee can be greater than the
    /// parameter, but only the amount in the parameter will be charged.
    #[prost(message, optional, tag="5")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgCreateClass {
const NAME: &'static str = "MsgCreateClass";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateClassResponse is the Msg/CreateClass response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClassResponse {
    /// class_id is the unique identifier of the credit class.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateClassResponse {
const NAME: &'static str = "MsgCreateClassResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateProjectResponse is the Msg/CreateProject request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateProject {
    /// admin is the address of the account creating the project that will become
    /// the admin of the project upon creation. The creator of the project must be
    /// an approved issuer within the credit class under which the project is being
    /// created. The admin will have permissions to update the project including
    /// the ability to reassign the admin role to another account.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the credit class under which the
    /// project will be created.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// metadata is any arbitrary string with a maximum length of 256 characters
    /// that includes or references metadata to attach to the project.
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    /// jurisdiction is the jurisdiction of the project. A jurisdiction has with
    /// the format: <country-code>\[-<sub-national-code>[ <postal-code>]\]
    /// The country-code must be 2 alphabetic characters, the sub-national-code
    /// can be 1-3 alphanumeric characters, and the postal-code can be up to 64
    /// alphanumeric characters. Only the country-code is required, while the
    /// sub-national-code and postal-code are optional and can be added for
    /// increased precision.
    #[prost(string, tag="4")]
    pub jurisdiction: ::prost::alloc::string::String,
    /// reference_id is any arbitrary string used to reference the project with a
    /// maximum length of 32 characters.
    #[prost(string, tag="5")]
    pub reference_id: ::prost::alloc::string::String,
    /// fee is the project creation fee. An equal fee is required if the project
    /// creation fee parameter is set. The provided fee can be greater than the
    /// parameter, but only the amount in the parameter will be charged.
    ///
    /// Since Revision 3
    #[prost(message, optional, tag="6")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgCreateProject {
const NAME: &'static str = "MsgCreateProject";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateProjectResponse is the Msg/CreateProject response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateProjectResponse {
    /// project_id is the unique identifier of the project.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateProjectResponse {
const NAME: &'static str = "MsgCreateProjectResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateUnregisteredProject is the Msg/CreateUnregisteredProject request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateUnregisteredProject {
    /// admin is the address of the account creating the project that will become
    /// the admin of the project upon creation.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// metadata is any arbitrary string with a maximum length of 256 characters
    /// that includes or references metadata to attach to the project.
    #[prost(string, tag="2")]
    pub metadata: ::prost::alloc::string::String,
    /// jurisdiction is the jurisdiction of the project. A jurisdiction has with
    /// the format: <country-code>\[-<sub-national-code>[ <postal-code>]\]
    /// The country-code must be 2 alphabetic characters, the sub-national-code
    /// can be 1-3 alphanumeric characters, and the postal-code can be up to 64
    /// alphanumeric characters. Only the country-code is required, while the
    /// sub-national-code and postal-code are optional and can be added for
    /// increased precision.
    #[prost(string, tag="3")]
    pub jurisdiction: ::prost::alloc::string::String,
    /// reference_id is any arbitrary string used to reference the project with a
    /// maximum length of 32 characters.
    #[prost(string, tag="4")]
    pub reference_id: ::prost::alloc::string::String,
    /// fee is the project creation fee. An equal fee is required if the project
    /// creation fee parameter is set. The provided fee can be greater than the
    /// parameter, but only the amount in the parameter will be charged.
    ///
    /// Since Revision 3
    #[prost(message, optional, tag="5")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgCreateUnregisteredProject {
const NAME: &'static str = "MsgCreateUnregisteredProject";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateUnregisteredProjectResponse is the Msg/CreateUnregisteredProject response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateUnregisteredProjectResponse {
    /// project_id is the unique identifier of the project.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateUnregisteredProjectResponse {
const NAME: &'static str = "MsgCreateUnregisteredProjectResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateOrUpdateApplication is the Msg/CreateOrUpdateApplication request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateOrUpdateApplication {
    /// project_admin is the address of the account that is the admin of the
    /// project which is applying to the credit class.
    #[prost(string, tag="1")]
    pub project_admin: ::prost::alloc::string::String,
    /// project_id is the identifier of the project which is applying to
    /// the credit class.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// class_id is the identifier of the credit class which the project is
    /// applying to.
    #[prost(string, tag="3")]
    pub class_id: ::prost::alloc::string::String,
    /// metadata is any optional arbitrary string with a maximum length of 256 characters
    /// that includes or references any metadata relevant to the application.
    /// This could be used as a digital reference to the actual contents of the application.
    #[prost(string, tag="4")]
    pub metadata: ::prost::alloc::string::String,
    /// withdraw is a boolean that indicates whether the application is being
    /// withdrawn rather than updated.
    #[prost(bool, tag="5")]
    pub withdraw: bool,
}
impl ::prost::Name for MsgCreateOrUpdateApplication {
const NAME: &'static str = "MsgCreateOrUpdateApplication";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateOrUpdateApplicationResponse is the Msg/CreateOrUpdateApplication response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateOrUpdateApplicationResponse {
}
impl ::prost::Name for MsgCreateOrUpdateApplicationResponse {
const NAME: &'static str = "MsgCreateOrUpdateApplicationResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateProjectEnrollment is the Msg/UpdateProjectEnrollment request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateProjectEnrollment {
    /// issuer is the address of the account that is the issuer of the credit class
    /// which is updating the project enrollment status.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// project_id is the identifier of the project.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// class_id is the identifier of the credit class.
    #[prost(string, tag="3")]
    pub class_id: ::prost::alloc::string::String,
    /// new_status is the new status of the project enrollment.
    #[prost(enumeration="ProjectEnrollmentStatus", tag="4")]
    pub new_status: i32,
    /// metadata is any optiopnal arbitrary string with a maximum length of 256 characters
    /// that includes or references the reason for the approving, requesting changes
    /// to, or rejecting the application, or terminating the project.
    #[prost(string, tag="5")]
    pub metadata: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateProjectEnrollment {
const NAME: &'static str = "MsgUpdateProjectEnrollment";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgEvaluateProjectEnrollmentResponse is the Msg/EvaluateProjectEnrollment response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateProjectEnrollmentResponse {
}
impl ::prost::Name for MsgUpdateProjectEnrollmentResponse {
const NAME: &'static str = "MsgUpdateProjectEnrollmentResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateBatch is the Msg/CreateBatch request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBatch {
    /// issuer is the address of the account issuing the credits and must be an
    /// approved issuer within a credit class of the project.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// project_id is the unique identifier of the project under which the credit
    /// batch will be created.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// issuance specifies the amount of tradable and retired credits that will be
    /// issued to each recipient and the jurisdiction in which the credits will be
    /// retired if credits are to be retired upon receipt.
    #[prost(message, repeated, tag="3")]
    pub issuance: ::prost::alloc::vec::Vec<BatchIssuance>,
    /// metadata is any arbitrary string with a maximum length of 256 characters
    /// that includes or references metadata to attach to the credit batch.
    #[prost(string, tag="4")]
    pub metadata: ::prost::alloc::string::String,
    /// start_date is the beginning of the period during which this credit batch
    /// was quantified and verified.
    #[prost(message, optional, tag="5")]
    pub start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// end_date is the end of the period during which this credit batch was
    /// quantified and verified.
    #[prost(message, optional, tag="6")]
    pub end_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// open determines whether or not the credits can be dynamically minted to the
    /// credit batch following the creation of the credit batch. This field should
    /// only be set to true when bridging credits from another chain or registry as
    /// a result of a bridge operation and is not intended for native issuance.
    #[prost(bool, tag="7")]
    pub open: bool,
    /// origin_tx is the transaction from another chain or registry that triggered
    /// the creation of the credit batch. This field can be ignored when natively
    /// issuing credits and should only be set when bridging assets from another
    /// chain or registry as a result of a bridge operation.
    #[prost(message, optional, tag="8")]
    pub origin_tx: ::core::option::Option<OriginTx>,
    /// class_id is the unique identifier of the credit class under which the
    /// credit batch will be created.
    ///
    /// Since Revision 3
    #[prost(string, tag="9")]
    pub class_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateBatch {
const NAME: &'static str = "MsgCreateBatch";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCreateBatchResponse is the Msg/CreateBatch response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBatchResponse {
    /// batch_denom is the unique identifier of the credit batch.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateBatchResponse {
const NAME: &'static str = "MsgCreateBatchResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgMintBatchCredits is the Msg/MintBatchCredits request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintBatchCredits {
    /// issuer is the address of the account minting the credits and must be the
    /// same issuer who created the credit batch.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// issuance specifies the amount of tradable and retired credits that will be
    /// issued to each recipient and the jurisdiction in which the credits will be
    /// retired if credits are to be retired upon receipt.
    #[prost(message, repeated, tag="3")]
    pub issuance: ::prost::alloc::vec::Vec<BatchIssuance>,
    /// origin_tx is the transaction from another chain or registry that triggered
    /// the minting of credits.
    #[prost(message, optional, tag="4")]
    pub origin_tx: ::core::option::Option<OriginTx>,
}
impl ::prost::Name for MsgMintBatchCredits {
const NAME: &'static str = "MsgMintBatchCredits";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgMintBatchCreditsResponse is the Msg/MintBatchCredits response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintBatchCreditsResponse {
}
impl ::prost::Name for MsgMintBatchCreditsResponse {
const NAME: &'static str = "MsgMintBatchCreditsResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgSealBatch is the Msg/MintBatchCredits request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSealBatch {
    /// issuer is the address of the account that created the credit batch and the
    /// only account with permissions to seal the credit batch.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the credit batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSealBatch {
const NAME: &'static str = "MsgSealBatch";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgSealBatchResponse is the Msg/SealBatch response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSealBatchResponse {
}
impl ::prost::Name for MsgSealBatchResponse {
const NAME: &'static str = "MsgSealBatchResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgSend is the Msg/Send request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    /// sender is the address of the account sending credits.
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// recipient is the address of the account receiving credits.
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// credits are the credits being sent to the recipient.
    #[prost(message, repeated, tag="3")]
    pub credits: ::prost::alloc::vec::Vec<msg_send::SendCredits>,
}
/// Nested message and enum types in `MsgSend`.
pub mod msg_send {
    /// SendCredits specifies the amount of tradable and retired credits of a
    /// credit batch that will be sent to the recipient and the jurisdiction in
    /// which the credits will be retired upon receipt.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendCredits {
        /// batch_denom is the unique identifier of the credit batch.
        #[prost(string, tag="1")]
        pub batch_denom: ::prost::alloc::string::String,
        /// tradable_amount is the amount of credits in this transfer that can be
        /// traded by the recipient. The number of decimal places must be less than
        /// or equal to the credit type precision.
        #[prost(string, tag="2")]
        pub tradable_amount: ::prost::alloc::string::String,
        /// retired_amount is the amount of credits in this transfer that are retired
        /// upon receipt. The number of decimal places must be less than or equal to
        /// the credit type precision.
        #[prost(string, tag="3")]
        pub retired_amount: ::prost::alloc::string::String,
        /// retirement_jurisdiction is the jurisdiction of the recipient and is only
        /// required if retired_amount is positive. A jurisdiction has the format:
        /// <country-code>\[-<sub-national-code>[ <postal-code>]\]
        /// The country-code and sub-national-code must conform to ISO 3166-2 and the
        /// postal-code can be up to 64 alphanumeric characters. Only the
        /// country-code is required, while the sub-national-code and postal-code are
        /// optional and can be added for increased precision.
        #[prost(string, tag="4")]
        pub retirement_jurisdiction: ::prost::alloc::string::String,
        /// retirement_reason is any arbitrary string that specifies the reason for
        /// retiring credits. This field is only required if retired_amount is
        /// positive.
        ///
        /// Since Revision 2
        #[prost(string, tag="5")]
        pub retirement_reason: ::prost::alloc::string::String,
    }
impl ::prost::Name for SendCredits {
const NAME: &'static str = "SendCredits";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.MsgSend.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgSend {
const NAME: &'static str = "MsgSend";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgSendResponse is the Msg/Send response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {
}
impl ::prost::Name for MsgSendResponse {
const NAME: &'static str = "MsgSendResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgRetire is the Msg/Retire request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRetire {
    /// owner is the address of the account that owns the credits being retired.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// credits specifies a credit batch and the number of credits being retired.
    #[prost(message, repeated, tag="2")]
    pub credits: ::prost::alloc::vec::Vec<Credits>,
    /// jurisdiction is the jurisdiction of the credit owner. A jurisdiction has
    /// the format: <country-code>\[-<sub-national-code>[ <postal-code>]\]
    /// The country-code must be 2 alphabetic characters, the sub-national-code
    /// can be 1-3 alphanumeric characters, and the postal-code can be up to 64
    /// alphanumeric characters. Only the country-code is required, while the
    /// sub-national-code and postal-code are optional and can be added for
    /// increased precision.
    #[prost(string, tag="3")]
    pub jurisdiction: ::prost::alloc::string::String,
    /// reason is any arbitrary string that specifies the reason for retiring
    /// credits.
    ///
    /// Since Revision 2
    #[prost(string, tag="4")]
    pub reason: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRetire {
const NAME: &'static str = "MsgRetire";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgRetire is the Msg/Retire response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRetireResponse {
}
impl ::prost::Name for MsgRetireResponse {
const NAME: &'static str = "MsgRetireResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCancel is the Msg/Cancel request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancel {
    /// owner is the address of the account that owns the credits being cancelled.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// credits specifies a credit batch and the number of credits being cancelled.
    #[prost(message, repeated, tag="2")]
    pub credits: ::prost::alloc::vec::Vec<Credits>,
    /// reason is any arbitrary string that specifies the reason for cancelling
    /// credits.
    #[prost(string, tag="3")]
    pub reason: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCancel {
const NAME: &'static str = "MsgCancel";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgCancelResponse is the Msg/Cancel response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelResponse {
}
impl ::prost::Name for MsgCancelResponse {
const NAME: &'static str = "MsgCancelResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateClassAdmin is the Msg/UpdateClassAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassAdmin {
    /// admin is the address of the account that is currently the admin of the
    /// credit class.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the credit class.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// new_admin is the address of the account that will become the new admin of
    /// the credit class.
    #[prost(string, tag="3")]
    pub new_admin: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateClassAdmin {
const NAME: &'static str = "MsgUpdateClassAdmin";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateClassAdminResponse is the MsgUpdateClassAdmin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassAdminResponse {
}
impl ::prost::Name for MsgUpdateClassAdminResponse {
const NAME: &'static str = "MsgUpdateClassAdminResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateClassIssuers is the Msg/UpdateClassIssuers request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassIssuers {
    /// admin is the address of the account that is the admin of the credit class.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the credit class.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// add_issuers are the addresses of the accounts that will be added to the
    /// list of approved credit class issuers.
    #[prost(string, repeated, tag="3")]
    pub add_issuers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// remove_issuers are the addresses of the accounts that will be removed from
    /// the list of approved credit class issuers.
    #[prost(string, repeated, tag="4")]
    pub remove_issuers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgUpdateClassIssuers {
const NAME: &'static str = "MsgUpdateClassIssuers";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateClassIssuersResponse is the MsgUpdateClassIssuers response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassIssuersResponse {
}
impl ::prost::Name for MsgUpdateClassIssuersResponse {
const NAME: &'static str = "MsgUpdateClassIssuersResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateClassMetadata is the Msg/UpdateClassMetadata request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassMetadata {
    /// admin is the address of the account that is the admin of the credit class.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the credit class.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// new_metadata is new metadata that will replace the existing metadata. It
    /// can be any arbitrary string with a maximum length of 256 characters that
    /// includes or references the metadata to attach to the credit class.
    #[prost(string, tag="3")]
    pub new_metadata: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateClassMetadata {
const NAME: &'static str = "MsgUpdateClassMetadata";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateClassMetadataResponse is the Msg/UpdateClassMetadata response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassMetadataResponse {
}
impl ::prost::Name for MsgUpdateClassMetadataResponse {
const NAME: &'static str = "MsgUpdateClassMetadataResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateProjectAdmin is the Msg/UpdateProjectAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateProjectAdmin {
    /// admin is the address of the account that is the currently the admin of the
    /// project.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// project_id is the unique identifier of the project.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// new_admin is the address of the account that will become the new admin of
    /// the project.
    #[prost(string, tag="3")]
    pub new_admin: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateProjectAdmin {
const NAME: &'static str = "MsgUpdateProjectAdmin";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateProjectAdmin is the Msg/UpdateProjectAdmin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateProjectAdminResponse {
}
impl ::prost::Name for MsgUpdateProjectAdminResponse {
const NAME: &'static str = "MsgUpdateProjectAdminResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateProjectMetadata is the Msg/UpdateProjectMetadata request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateProjectMetadata {
    /// admin is the address of the account that is the admin of the project.
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// project_id is the unique identifier of the project.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// new_metadata is new metadata that will replace the existing metadata. It
    /// can be any arbitrary string with a maximum length of 256 characters that
    /// includes or references the metadata to attach to the project.
    #[prost(string, tag="3")]
    pub new_metadata: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateProjectMetadata {
const NAME: &'static str = "MsgUpdateProjectMetadata";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateProjectMetadataResponse is the Msg/UpdateProjectMetadataResponse
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateProjectMetadataResponse {
}
impl ::prost::Name for MsgUpdateProjectMetadataResponse {
const NAME: &'static str = "MsgUpdateProjectMetadataResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgBridge is the Msg/Bridge request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBridge {
    /// owner is the address of the account that owns the credits being bridged.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// target is the name of the target chain or registry.
    #[prost(string, tag="2")]
    pub target: ::prost::alloc::string::String,
    /// recipient is the address of the account receiving the bridged credits.
    #[prost(string, tag="3")]
    pub recipient: ::prost::alloc::string::String,
    /// credits specifies a credit batch and the number of credits being bridged.
    #[prost(message, repeated, tag="4")]
    pub credits: ::prost::alloc::vec::Vec<Credits>,
}
impl ::prost::Name for MsgBridge {
const NAME: &'static str = "MsgBridge";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateBatchMetadata is the Msg/UpdateBatchMetadata request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateBatchMetadata {
    /// issuer is the address of the account that is the issuer of the batch.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// batch_denom is the unique identifier of the batch.
    #[prost(string, tag="2")]
    pub batch_denom: ::prost::alloc::string::String,
    /// new_metadata is new metadata that will replace the existing metadata. It
    /// can be any arbitrary string with a maximum length of 256 characters that
    /// includes or references the metadata to attach to the batch.
    #[prost(string, tag="3")]
    pub new_metadata: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateBatchMetadata {
const NAME: &'static str = "MsgUpdateBatchMetadata";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateBatchMetadataResponse is the Msg/UpdateBatchMetadataResponse
/// response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateBatchMetadataResponse {
}
impl ::prost::Name for MsgUpdateBatchMetadataResponse {
const NAME: &'static str = "MsgUpdateBatchMetadataResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgBridgeResponse is the Msg/Bridge response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBridgeResponse {
}
impl ::prost::Name for MsgBridgeResponse {
const NAME: &'static str = "MsgBridgeResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgBridgeReceive is the Msg/BridgeReceive request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBridgeReceive {
    /// issuer is the account address of the service bridging the credits.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// class_id is the unique identifier of the credit class within which the
    /// project and credit batch already exist or will be created.
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    /// project defines the project information for the bridged credits.
    #[prost(message, optional, tag="3")]
    pub project: ::core::option::Option<msg_bridge_receive::Project>,
    /// batch defines the credit batch information for the bridged credits.
    #[prost(message, optional, tag="4")]
    pub batch: ::core::option::Option<msg_bridge_receive::Batch>,
    /// origin_tx is a reference to a transaction which caused the transfer from
    /// another chain or registry.
    #[prost(message, optional, tag="5")]
    pub origin_tx: ::core::option::Option<OriginTx>,
}
/// Nested message and enum types in `MsgBridgeReceive`.
pub mod msg_bridge_receive {
    /// Batch defines the credit batch information for the bridged credits. This
    /// information will be used to create a credit batch or to dynamically mint
    /// credits to an existing credit batch.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Batch {
        /// recipient is the recipient of the bridged credits.
        #[prost(string, tag="1")]
        pub recipient: ::prost::alloc::string::String,
        /// amount is the amount of credits being bridged.
        #[prost(string, tag="2")]
        pub amount: ::prost::alloc::string::String,
        /// start_date is the beginning of the period during which this credit batch
        /// was quantified and verified.
        #[prost(message, optional, tag="3")]
        pub start_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
        /// end_date is the end of the period during which this credit batch was
        /// quantified and verified.
        #[prost(message, optional, tag="4")]
        pub end_date: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
        /// metadata is the metadata for the credit batch.
        #[prost(string, tag="5")]
        pub metadata: ::prost::alloc::string::String,
    }
impl ::prost::Name for Batch {
const NAME: &'static str = "Batch";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.MsgBridgeReceive.{}", Self::NAME)
            }}
    /// Project defines the project information for the bridged credits. This
    /// information will be used to find an existing project or to create a new
    /// project if a project with the same reference id does not already exist.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Project {
        /// reference_id is the reference id of the project.
        #[prost(string, tag="1")]
        pub reference_id: ::prost::alloc::string::String,
        /// jurisdiction is the project jurisdiction.
        #[prost(string, tag="2")]
        pub jurisdiction: ::prost::alloc::string::String,
        /// metadata is the metadata for the project.
        #[prost(string, tag="3")]
        pub metadata: ::prost::alloc::string::String,
    }
impl ::prost::Name for Project {
const NAME: &'static str = "Project";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.MsgBridgeReceive.{}", Self::NAME)
            }}
}
impl ::prost::Name for MsgBridgeReceive {
const NAME: &'static str = "MsgBridgeReceive";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgBridgeReceiveResponse is the Msg/BridgeReceive response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBridgeReceiveResponse {
    /// batch_denom is the unique identifier of the credit batch either created
    /// or within which the credits were dynamically minted.
    #[prost(string, tag="1")]
    pub batch_denom: ::prost::alloc::string::String,
    /// project_id is the unique identifier of the project that was either created
    /// or the existing project within which the credit batch exists.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgBridgeReceiveResponse {
const NAME: &'static str = "MsgBridgeReceiveResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgAddClassCreator is the Msg/AddClassCreator request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddClassCreator {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// creator is the address to add to the class creator list.
    #[prost(string, tag="2")]
    pub creator: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgAddClassCreator {
const NAME: &'static str = "MsgAddClassCreator";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgAddClassCreatorResponse is the Msg/AddClassCreator response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddClassCreatorResponse {
}
impl ::prost::Name for MsgAddClassCreatorResponse {
const NAME: &'static str = "MsgAddClassCreatorResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgSetClassCreatorAllowlist is the Msg/SetClassCreatorAllowlist request
/// type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetClassCreatorAllowlist {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// enabled defines the boolean value to set the allowlist on or off.
    #[prost(bool, tag="2")]
    pub enabled: bool,
}
impl ::prost::Name for MsgSetClassCreatorAllowlist {
const NAME: &'static str = "MsgSetClassCreatorAllowlist";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgSetClassCreatorAllowlistResponse is the Msg/SetClassCreatorAllowlist
/// response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetClassCreatorAllowlistResponse {
}
impl ::prost::Name for MsgSetClassCreatorAllowlistResponse {
const NAME: &'static str = "MsgSetClassCreatorAllowlistResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgRemoveClassCreator is the Msg/RemoveClassCreator request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveClassCreator {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// creator is the address to remove from the class creator list.
    #[prost(string, tag="2")]
    pub creator: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRemoveClassCreator {
const NAME: &'static str = "MsgRemoveClassCreator";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgRemoveClassCreatorResponse is the Msg/RemoveClasssCreator response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveClassCreatorResponse {
}
impl ::prost::Name for MsgRemoveClassCreatorResponse {
const NAME: &'static str = "MsgRemoveClassCreatorResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateClassFee is the Msg/UpdateClassFee request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassFee {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// fee is the credit class creation fee. If not set, the credit class creation
    /// fee will be removed and no fee will be required to create a credit class.
    #[prost(message, optional, tag="2")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgUpdateClassFee {
const NAME: &'static str = "MsgUpdateClassFee";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateClassFeeResponse is the Msg/UpdateClassFee response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassFeeResponse {
}
impl ::prost::Name for MsgUpdateClassFeeResponse {
const NAME: &'static str = "MsgUpdateClassFeeResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateProjectFee is the Msg/UpdateProjectFee request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateProjectFee {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// fee is the project creation fee. If not set, the project creation fee will
    /// be removed and no fee will be required to create a project.
    #[prost(message, optional, tag="2")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgUpdateProjectFee {
const NAME: &'static str = "MsgUpdateProjectFee";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgUpdateProjectFeeResponse is the Msg/UpdateProjectFee response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateProjectFeeResponse {
}
impl ::prost::Name for MsgUpdateProjectFeeResponse {
const NAME: &'static str = "MsgUpdateProjectFeeResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgAddAllowedBridgeChain is the Msg/AddAllowedBridgeChain request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddAllowedBridgeChain {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// chain_name is the name of the chain to allow bridging of ecocredits to
    /// (i.e. polygon, ethereum, celo).
    #[prost(string, tag="2")]
    pub chain_name: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgAddAllowedBridgeChain {
const NAME: &'static str = "MsgAddAllowedBridgeChain";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgAddAllowedBridgeChainResponse is the Msg/AddAllowedBridgeChain response
/// type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddAllowedBridgeChainResponse {
}
impl ::prost::Name for MsgAddAllowedBridgeChainResponse {
const NAME: &'static str = "MsgAddAllowedBridgeChainResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgRemoveAllowedBridgeChain is the Msg/RemoveAllowedBridgeChain request type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveAllowedBridgeChain {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// chain_name is the name of the chain to remove from the list of allowed
    /// chains to bridge ecocredits to (i.e. polygon, ethereum, celo).
    #[prost(string, tag="2")]
    pub chain_name: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRemoveAllowedBridgeChain {
const NAME: &'static str = "MsgRemoveAllowedBridgeChain";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgRemoveAllowedBridgeChainResponse is the Msg/RemoveAllowedBridgeChain
/// response type.
///
/// Since Revision 2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveAllowedBridgeChainResponse {
}
impl ::prost::Name for MsgRemoveAllowedBridgeChainResponse {
const NAME: &'static str = "MsgRemoveAllowedBridgeChainResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgBurnRegen is the Msg/BurnRegen request type.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnRegen {
    /// burner is the address of the account burning REGEN tokens.
    #[prost(string, tag="1")]
    pub burner: ::prost::alloc::string::String,
    /// amount is the integer amount of uregen tokens to burn.
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// reason is any arbitrary string that specifies the reason for burning
    /// REGEN tokens. It may be at most 256 characters long.
    #[prost(string, tag="3")]
    pub reason: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgBurnRegen {
const NAME: &'static str = "MsgBurnRegen";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
/// MsgBurnResponse is the Msg/Burn response type.
///
/// Since Revision 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnRegenResponse {
}
impl ::prost::Name for MsgBurnRegenResponse {
const NAME: &'static str = "MsgBurnRegenResponse";
const PACKAGE: &'static str = "regen.ecocredit.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.ecocredit.v1.{}", Self::NAME)
            }}
include!("regen.ecocredit.v1.serde.rs");
include!("regen.ecocredit.v1.tonic.rs");
// @@protoc_insertion_point(module)